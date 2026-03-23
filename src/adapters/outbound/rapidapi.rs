use async_trait::async_trait;
use reqwest::Client;

use crate::domain::error::AppError;
use crate::domain::models::RouteResponse;
use crate::ports::outbound::TransitApiClient;

const BASE_URL: &str = "https://navitime-route-totalnavi.p.rapidapi.com/route_transit";
const RAPIDAPI_HOST: &str = "navitime-route-totalnavi.p.rapidapi.com";

pub struct NavitimeRapidApiClient {
    client: Client,
    api_key: String,
}

impl NavitimeRapidApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl TransitApiClient for NavitimeRapidApiClient {
    async fn fetch_route(
        &self,
        start_lat: f64,
        start_lon: f64,
        goal_lat: f64,
        goal_lon: f64,
        start_time: Option<String>,
    ) -> Result<RouteResponse, AppError> {
        let start = format!("{},{}", start_lat, start_lon);
        let goal = format!("{},{}", goal_lat, goal_lon);

        let mut query = vec![
            ("start", start),
            ("goal", goal),
            ("limit", "5".to_string()),
        ];

        if let Some(time) = start_time {
            query.push(("start_time", time));
        }

        let response = self
            .client
            .get(BASE_URL)
            .header("x-rapidapi-key", &self.api_key)
            .header("x-rapidapi-host", RAPIDAPI_HOST)
            .query(&query)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            return Err(AppError::ApiError { status, body });
        }

        let route_response: RouteResponse = response
            .json()
            .await
            .map_err(|e| AppError::ParsingError(e.to_string()))?;

        Ok(route_response)
    }
}
