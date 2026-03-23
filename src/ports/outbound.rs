use async_trait::async_trait;

use crate::domain::error::AppError;
use crate::domain::models::RouteResponse;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TransitApiClient: Send + Sync {
    async fn fetch_route(
        &self,
        start_lat: f64,
        start_lon: f64,
        goal_lat: f64,
        goal_lon: f64,
        start_time: Option<String>,
    ) -> Result<RouteResponse, AppError>;
}
