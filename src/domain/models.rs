use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponse {
    pub items: Vec<RouteItem>,
}

/// A single route option (Route object)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteItem {
    pub summary: RouteSummary,
    pub sections: Vec<Section>,
}

/// RouteSummary object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSummary {
    #[serde(default)]
    pub no: Option<String>,
    #[serde(default)]
    pub start: Option<SummaryPoint>,
    #[serde(default)]
    pub goal: Option<SummaryPoint>,
    #[serde(rename = "move")]
    pub move_info: SummaryMove,
}

/// RouteSummaryItem (type=point) for start/goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryPoint {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub coord: Option<Coordinate>,
    #[serde(default)]
    pub node_id: Option<String>,
    #[serde(default)]
    pub node_types: Option<Vec<String>>,
}

/// RouteSummaryItem (type=move) - movement overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryMove {
    #[serde(default)]
    pub transit_count: Option<i32>,
    #[serde(default)]
    pub walk_distance: Option<i32>,
    #[serde(default)]
    pub fare: Option<Fare>,
    #[serde(default)]
    pub from_time: Option<String>,
    #[serde(default)]
    pub to_time: Option<String>,
    #[serde(default)]
    pub time: Option<i32>,
    #[serde(default)]
    pub distance: Option<i32>,
}

/// Fare object - dynamic unit_* keys with f64 values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fare {
    /// unit_0 = normal fare
    #[serde(default)]
    pub unit_0: Option<f64>,
    /// unit_48 = IC card fare
    #[serde(default)]
    pub unit_48: Option<f64>,
    /// Capture all other unit_* fare fields
    #[serde(flatten)]
    pub other_units: HashMap<String, serde_json::Value>,
}

/// Section tagged enum (type = "point" | "move")
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Section {
    #[serde(rename = "point")]
    Point(SectionPoint),
    #[serde(rename = "move")]
    Move(Box<SectionMove>),
}

/// RouteSectionItem (type=point)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionPoint {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub coord: Option<Coordinate>,
    #[serde(default)]
    pub node_id: Option<String>,
    #[serde(default)]
    pub node_types: Option<Vec<String>>,
    #[serde(default)]
    pub gateway: Option<String>,
}

/// RouteSectionItem (type=move)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionMove {
    /// Movement type: "walk", "local_train", "superexpress_train", etc.
    #[serde(rename = "move", default)]
    pub move_type: Option<String>,
    /// Transport details (fare, line info, company, etc.)
    #[serde(default)]
    pub transport: Option<Transport>,
    /// Line name (e.g., "東京メトロ銀座線")
    #[serde(default)]
    pub line_name: Option<String>,
    /// Departure time from the preceding point
    #[serde(default)]
    pub from_time: Option<String>,
    /// Arrival time at the following point
    #[serde(default)]
    pub to_time: Option<String>,
    /// Travel time in minutes
    #[serde(default)]
    pub time: Option<i32>,
    /// Distance in meters
    #[serde(default)]
    pub distance: Option<i32>,
}

/// Transport object - public transit details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transport {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub fare: Option<Fare>,
    #[serde(default)]
    pub company: Option<Company>,
    #[serde(default)]
    pub links: Option<Vec<Link>>,
    /// Train type (e.g., "普通", "Local")
    #[serde(rename = "type", default)]
    pub transport_type: Option<String>,
}

/// Company object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

/// Link object - route/line information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub direction: Option<String>,
    #[serde(default)]
    pub destination: Option<NodeItem>,
    #[serde(default)]
    pub from: Option<NodeItem>,
    #[serde(default)]
    pub to: Option<NodeItem>,
}

/// NodeItem object - station/stop reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

/// Coordinate object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}
