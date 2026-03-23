use async_trait::async_trait;

use crate::domain::error::AppError;
use crate::domain::models::RouteResponse;

#[async_trait]
pub trait RouteSearchUseCase: Send + Sync {
    async fn search_route(
        &self,
        start_lat: f64,
        start_lon: f64,
        goal_lat: f64,
        goal_lon: f64,
        start_time: Option<String>,
    ) -> Result<RouteResponse, AppError>;
}
