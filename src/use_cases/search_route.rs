use async_trait::async_trait;

use crate::domain::error::AppError;
use crate::domain::models::RouteResponse;
use crate::ports::inbound::RouteSearchUseCase;
use crate::ports::outbound::TransitApiClient;

pub struct SearchRouteService<C: TransitApiClient> {
    client: C,
}

impl<C: TransitApiClient> SearchRouteService<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: TransitApiClient> RouteSearchUseCase for SearchRouteService<C> {
    async fn search_route(
        &self,
        start_lat: f64,
        start_lon: f64,
        goal_lat: f64,
        goal_lon: f64,
        start_time: Option<String>,
    ) -> Result<RouteResponse, AppError> {
        if !(-90.0..=90.0).contains(&start_lat) || !(-90.0..=90.0).contains(&goal_lat) {
            return Err(AppError::InvalidArgument("Latitude must be between -90 and 90".to_string()));
        }
        if !(-180.0..=180.0).contains(&start_lon) || !(-180.0..=180.0).contains(&goal_lon) {
            return Err(AppError::InvalidArgument("Longitude must be between -180 and 180".to_string()));
        }

        let time = start_time.unwrap_or_else(|| {
            chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
        });

        self.client
            .fetch_route(start_lat, start_lon, goal_lat, goal_lon, Some(time))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::*;
    use crate::ports::outbound::MockTransitApiClient;

    fn sample_response() -> RouteResponse {
        RouteResponse {
            items: vec![RouteItem {
                summary: RouteSummary {
                    no: Some("1".to_string()),
                    start: Some(SummaryPoint {
                        name: Some("Omotesando".to_string()),
                        coord: Some(Coordinate { lat: 35.665291, lon: 139.712613 }),
                        node_id: Some("00007820".to_string()),
                        node_types: Some(vec!["station".to_string()]),
                    }),
                    goal: Some(SummaryPoint {
                        name: Some("Ginza".to_string()),
                        coord: Some(Coordinate { lat: 35.671335, lon: 139.76513 }),
                        node_id: Some("00001908".to_string()),
                        node_types: Some(vec!["station".to_string()]),
                    }),
                    move_info: SummaryMove {
                        transit_count: Some(0),
                        walk_distance: None,
                        fare: Some(Fare {
                            unit_0: Some(170.0),
                            unit_48: Some(165.0),
                            other_units: std::collections::HashMap::new(),
                        }),
                        from_time: Some("2019-10-01T08:01:00+09:00".to_string()),
                        to_time: Some("2019-10-01T08:14:00+09:00".to_string()),
                        time: Some(13),
                        distance: Some(5900),
                    },
                },
                sections: vec![
                    Section::Point(SectionPoint {
                        name: Some("Omotesando".to_string()),
                        coord: Some(Coordinate { lat: 35.665291, lon: 139.712613 }),
                        node_id: Some("00007820".to_string()),
                        node_types: Some(vec!["station".to_string()]),
                        gateway: None,
                    }),
                    Section::Move(Box::new(SectionMove {
                        move_type: Some("local_train".to_string()),
                        transport: Some(Transport {
                            name: Some("Tokyo Metro Ginza Line".to_string()),
                            color: Some("#FF9500".to_string()),
                            fare: Some(Fare {
                                unit_0: Some(170.0),
                                unit_48: Some(165.0),
                                other_units: std::collections::HashMap::new(),
                            }),
                            company: Some(Company {
                                id: Some("00000113".to_string()),
                                name: Some("Tokyo Metro".to_string()),
                            }),
                            links: Some(vec![Link {
                                id: Some("00000768".to_string()),
                                name: Some("Tokyo Metro Ginza Line".to_string()),
                                direction: Some("up".to_string()),
                                destination: Some(NodeItem {
                                    id: Some("00005270".to_string()),
                                    name: Some("Asakusa".to_string()),
                                }),
                                from: Some(NodeItem {
                                    id: Some("00007820".to_string()),
                                    name: Some("Omotesando".to_string()),
                                }),
                                to: Some(NodeItem {
                                    id: Some("00001908".to_string()),
                                    name: Some("Ginza".to_string()),
                                }),
                            }]),
                            transport_type: Some("Local".to_string()),
                        }),
                        line_name: Some("Tokyo Metro Ginza Line".to_string()),
                        from_time: Some("2019-10-01T08:01:00+09:00".to_string()),
                        to_time: Some("2019-10-01T08:14:00+09:00".to_string()),
                        time: Some(13),
                        distance: Some(5900),
                    })),
                    Section::Point(SectionPoint {
                        name: Some("Ginza".to_string()),
                        coord: Some(Coordinate { lat: 35.671335, lon: 139.76513 }),
                        node_id: Some("00001908".to_string()),
                        node_types: Some(vec!["station".to_string()]),
                        gateway: None,
                    }),
                ],
            }],
        }
    }

    #[tokio::test]
    async fn test_search_route_success() {
        let mut mock = MockTransitApiClient::new();
        mock.expect_fetch_route()
            .returning(|_, _, _, _, _| Ok(sample_response()));

        let service = SearchRouteService::new(mock);
        let result = service
            .search_route(35.6812, 139.7671, 35.6595, 139.7004, Some("2026-01-01T10:00:00".to_string()))
            .await;

        assert!(result.is_ok());
        let response = result.ok().expect("expected Ok");
        assert_eq!(response.items.len(), 1);
    }

    #[tokio::test]
    async fn test_search_route_invalid_latitude() {
        let mock = MockTransitApiClient::new();
        let service = SearchRouteService::new(mock);
        let result = service
            .search_route(91.0, 139.0, 35.0, 139.0, None)
            .await;

        assert!(result.is_err());
        let err = result.err().expect("expected Err");
        assert!(matches!(err, AppError::InvalidArgument(_)));
    }

    #[tokio::test]
    async fn test_search_route_api_error_propagation() {
        let mut mock = MockTransitApiClient::new();
        mock.expect_fetch_route()
            .returning(|_, _, _, _, _| {
                Err(AppError::ApiError {
                    status: 429,
                    body: "Rate limit exceeded".to_string(),
                })
            });

        let service = SearchRouteService::new(mock);
        let result = service
            .search_route(35.6812, 139.7671, 35.6595, 139.7004, None)
            .await;

        assert!(result.is_err());
        let err = result.err().expect("expected Err");
        assert!(matches!(err, AppError::ApiError { status: 429, .. }));
    }

    #[tokio::test]
    async fn test_search_route_defaults_time_when_none() {
        let mut mock = MockTransitApiClient::new();
        mock.expect_fetch_route()
            .withf(|_, _, _, _, time| time.is_some())
            .returning(|_, _, _, _, _| Ok(sample_response()));

        let service = SearchRouteService::new(mock);
        let result = service
            .search_route(35.6812, 139.7671, 35.6595, 139.7004, None)
            .await;

        assert!(result.is_ok());
    }

    #[test]
    fn test_deserialize_api_response_fixture() {
        let json = r##"{
            "items": [{
                "summary": {
                    "no": "1",
                    "start": {
                        "type": "point",
                        "coord": { "lat": 35.665291, "lon": 139.712613 },
                        "name": "Omotesando",
                        "node_id": "00007820",
                        "node_types": ["station"]
                    },
                    "goal": {
                        "type": "point",
                        "coord": { "lat": 35.671335, "lon": 139.76513 },
                        "name": "Ginza",
                        "node_id": "00001908",
                        "node_types": ["station"]
                    },
                    "move": {
                        "transit_count": 0,
                        "fare": { "unit_0": 170.0, "unit_48": 165.0 },
                        "type": "move",
                        "from_time": "2019-10-01T08:01:00+09:00",
                        "to_time": "2019-10-01T08:14:00+09:00",
                        "time": 13,
                        "distance": 5900
                    }
                },
                "sections": [
                    {
                        "type": "point",
                        "coord": { "lat": 35.665291, "lon": 139.712613 },
                        "name": "Omotesando",
                        "node_id": "00007820",
                        "node_types": ["station"]
                    },
                    {
                        "transport": {
                            "fare": {
                                "unit_130": 19900.0,
                                "unit_0": 170.0,
                                "unit_48": 165.0,
                                "unit_128": 6980.0
                            },
                            "color": "#FF9500",
                            "name": "Tokyo Metro Ginza Line",
                            "fare_season": "normal",
                            "company": { "id": "00000113", "name": "Tokyo Metro" },
                            "links": [{
                                "id": "00000768",
                                "name": "Tokyo Metro Ginza Line",
                                "direction": "up",
                                "destination": { "name": "Asakusa", "id": "00005270" },
                                "from": { "name": "Omotesando", "id": "00007820" },
                                "to": { "name": "Ginza", "id": "00001908" }
                            }],
                            "id": "00000559",
                            "type": "Local"
                        },
                        "type": "move",
                        "move": "local_train",
                        "from_time": "2019-10-01T08:01:00+09:00",
                        "to_time": "2019-10-01T08:14:00+09:00",
                        "time": 13,
                        "distance": 5900,
                        "line_name": "Tokyo Metro Ginza Line"
                    },
                    {
                        "type": "point",
                        "coord": { "lat": 35.671335, "lon": 139.76513 },
                        "name": "Ginza",
                        "node_id": "00001908",
                        "node_types": ["station"]
                    }
                ]
            }]
        }"##;

        let response: RouteResponse = serde_json::from_str(json)
            .expect("Failed to deserialize API response fixture");

        assert_eq!(response.items.len(), 1);
        let item = &response.items[0];

        // Verify summary
        let summary = &item.summary;
        assert_eq!(summary.start.as_ref().and_then(|s| s.name.as_deref()), Some("Omotesando"));
        assert_eq!(summary.goal.as_ref().and_then(|g| g.name.as_deref()), Some("Ginza"));
        assert_eq!(summary.move_info.transit_count, Some(0));
        assert_eq!(summary.move_info.time, Some(13));
        assert_eq!(summary.move_info.distance, Some(5900));
        assert_eq!(summary.move_info.fare.as_ref().and_then(|f| f.unit_0), Some(170.0));
        assert_eq!(summary.move_info.fare.as_ref().and_then(|f| f.unit_48), Some(165.0));

        // Verify sections
        assert_eq!(item.sections.len(), 3);
        match &item.sections[0] {
            Section::Point(p) => assert_eq!(p.name.as_deref(), Some("Omotesando")),
            _ => panic!("Expected Point section"),
        }
        match &item.sections[1] {
            Section::Move(m) => {
                assert_eq!(m.move_type.as_deref(), Some("local_train"));
                assert_eq!(m.line_name.as_deref(), Some("Tokyo Metro Ginza Line"));
                assert_eq!(m.time, Some(13));
                let transport = m.transport.as_ref().expect("transport missing");
                assert_eq!(transport.name.as_deref(), Some("Tokyo Metro Ginza Line"));
                assert_eq!(transport.color.as_deref(), Some("#FF9500"));
                let company = transport.company.as_ref().expect("company missing");
                assert_eq!(company.name.as_deref(), Some("Tokyo Metro"));
            }
            _ => panic!("Expected Move section"),
        }
    }
}
