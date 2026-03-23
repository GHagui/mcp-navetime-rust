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
                summary: Summary {
                    move_info: SummaryMove {
                        transit_count: Some(2),
                        walk_distance: Some(500),
                        fare: Some(FareDetail {
                            unit_0: Some(450),
                            unit_1: None,
                            unit_2: None,
                        }),
                        time: Some(1800),
                    },
                },
                sections: vec![
                    Section::Point(SectionPoint {
                        name: Some("Tokyo Station".to_string()),
                        coord: Some(Coordinate {
                            lat: 35.6812,
                            lon: 139.7671,
                        }),
                    }),
                    Section::Move(SectionMove {
                        transport: Some("train".to_string()),
                        line_name: Some("JR Yamanote Line".to_string()),
                        from: Some(StopInfo {
                            name: Some("Tokyo".to_string()),
                            time: Some("2026-01-01T10:00:00".to_string()),
                        }),
                        to: Some(StopInfo {
                            name: Some("Shibuya".to_string()),
                            time: Some("2026-01-01T10:30:00".to_string()),
                        }),
                        distance: Some(10000),
                        time: Some(1800),
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
}
