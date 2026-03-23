use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponse {
    pub items: Vec<RouteItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteItem {
    pub summary: Summary,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    #[serde(rename = "move")]
    pub move_info: SummaryMove,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryMove {
    #[serde(default)]
    pub transit_count: Option<i32>,
    #[serde(default)]
    pub walk_distance: Option<i32>,
    #[serde(default)]
    pub fare: Option<FareDetail>,
    #[serde(default)]
    pub time: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FareDetail {
    #[serde(default)]
    pub unit_0: Option<i32>,
    #[serde(default)]
    pub unit_1: Option<i32>,
    #[serde(default)]
    pub unit_2: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Section {
    #[serde(rename = "point")]
    Point(SectionPoint),
    #[serde(rename = "move")]
    Move(SectionMove),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionPoint {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub coord: Option<Coordinate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionMove {
    #[serde(default)]
    pub transport: Option<String>,
    #[serde(default)]
    pub line_name: Option<String>,
    #[serde(default)]
    pub from: Option<StopInfo>,
    #[serde(default)]
    pub to: Option<StopInfo>,
    #[serde(default)]
    pub distance: Option<i32>,
    #[serde(default)]
    pub time: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}
