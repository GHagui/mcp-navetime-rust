use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::domain::models::{RouteResponse, Section};
use crate::ports::inbound::RouteSearchUseCase;

pub struct McpServer<U: RouteSearchUseCase> {
    use_case: U,
}

impl<U: RouteSearchUseCase> McpServer<U> {
    pub fn new(use_case: U) -> Self {
        Self { use_case }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                break; // EOF
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let request: Value = match serde_json::from_str(trimmed) {
                Ok(v) => v,
                Err(e) => {
                    let error_response = json!({
                        "jsonrpc": "2.0",
                        "id": null,
                        "error": {
                            "code": -32700,
                            "message": format!("Parse error: {}", e)
                        }
                    });
                    let mut out = serde_json::to_string(&error_response)?;
                    out.push('\n');
                    stdout.write_all(out.as_bytes()).await?;
                    stdout.flush().await?;
                    continue;
                }
            };

            let id = request.get("id").cloned();
            let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");

            // Notifications (no id) don't require a response
            if id.is_none() {
                continue;
            }

            let response = match method {
                "initialize" => self.handle_initialize(&id),
                "tools/list" => self.handle_tools_list(&id),
                "tools/call" => self.handle_tools_call(&id, &request).await,
                _ => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32601,
                        "message": format!("Method not found: {}", method)
                    }
                }),
            };

            let mut out = serde_json::to_string(&response)?;
            out.push('\n');
            stdout.write_all(out.as_bytes()).await?;
            stdout.flush().await?;
        }

        Ok(())
    }

    fn handle_initialize(&self, id: &Option<Value>) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "navitime-mcp",
                    "version": "0.1.0"
                }
            }
        })
    }

    fn handle_tools_list(&self, id: &Option<Value>) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": [
                    {
                        "name": "search_transit_route",
                        "description": "Finds train/walking transit routes in Japan between two coordinates using Navitime API.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "start_lat": {
                                    "type": "number",
                                    "description": "Start latitude"
                                },
                                "start_lon": {
                                    "type": "number",
                                    "description": "Start longitude"
                                },
                                "goal_lat": {
                                    "type": "number",
                                    "description": "Goal latitude"
                                },
                                "goal_lon": {
                                    "type": "number",
                                    "description": "Goal longitude"
                                },
                                "start_time": {
                                    "type": "string",
                                    "description": "Departure time in YYYY-MM-DDTHH:MM:SS format. Defaults to current time if not provided."
                                }
                            },
                            "required": ["start_lat", "start_lon", "goal_lat", "goal_lon"]
                        }
                    }
                ]
            }
        })
    }

    async fn handle_tools_call(&self, id: &Option<Value>, request: &Value) -> Value {
        let params = request.get("params").cloned().unwrap_or(json!({}));
        let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");

        if tool_name != "search_transit_route" {
            return json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [{
                        "type": "text",
                        "text": format!("Unknown tool: {}", tool_name)
                    }],
                    "isError": true
                }
            });
        }

        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        let start_lat = match arguments.get("start_lat").and_then(|v| v.as_f64()) {
            Some(v) => v,
            None => return self.tool_error(id, "Missing required argument: start_lat"),
        };
        let start_lon = match arguments.get("start_lon").and_then(|v| v.as_f64()) {
            Some(v) => v,
            None => return self.tool_error(id, "Missing required argument: start_lon"),
        };
        let goal_lat = match arguments.get("goal_lat").and_then(|v| v.as_f64()) {
            Some(v) => v,
            None => return self.tool_error(id, "Missing required argument: goal_lat"),
        };
        let goal_lon = match arguments.get("goal_lon").and_then(|v| v.as_f64()) {
            Some(v) => v,
            None => return self.tool_error(id, "Missing required argument: goal_lon"),
        };
        let start_time = arguments
            .get("start_time")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        match self
            .use_case
            .search_route(start_lat, start_lon, goal_lat, goal_lon, start_time)
            .await
        {
            Ok(response) => {
                let text = format_route_response(&response);
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": text
                        }],
                        "isError": false
                    }
                })
            }
            Err(e) => self.tool_error(id, &e.to_string()),
        }
    }

    fn tool_error(&self, id: &Option<Value>, message: &str) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "content": [{
                    "type": "text",
                    "text": message
                }],
                "isError": true
            }
        })
    }
}

fn format_route_response(response: &RouteResponse) -> String {
    let mut output = String::new();

    for (i, item) in response.items.iter().enumerate() {
        output.push_str(&format!("=== Route {} ===\n", i + 1));

        let summary = &item.summary.move_info;
        if let Some(time) = summary.time {
            output.push_str(&format!("Total time: {} min\n", time));
        }
        if let Some(ref fare) = summary.fare {
            if let Some(yen) = fare.unit_0 {
                output.push_str(&format!("Fare: {} yen\n", yen));
            }
            if let Some(ic) = fare.unit_48 {
                output.push_str(&format!("IC Fare: {} yen\n", ic));
            }
        }
        if let Some(distance) = summary.distance {
            output.push_str(&format!("Distance: {} m\n", distance));
        }
        if let Some(walk) = summary.walk_distance {
            output.push_str(&format!("Walk distance: {} m\n", walk));
        }
        if let Some(transits) = summary.transit_count {
            output.push_str(&format!("Transfers: {}\n", transits));
        }
        if let Some(ref from_time) = summary.from_time {
            output.push_str(&format!("Departure: {}\n", from_time));
        }
        if let Some(ref to_time) = summary.to_time {
            output.push_str(&format!("Arrival: {}\n", to_time));
        }

        output.push_str("\nSections:\n");
        for section in &item.sections {
            match section {
                Section::Point(p) => {
                    let name = p.name.as_deref().unwrap_or("?");
                    output.push_str(&format!("  [Point] {}\n", name));
                }
                Section::Move(m) => {
                    let move_type = m.move_type.as_deref().unwrap_or("unknown");
                    let line = m.line_name.as_deref().unwrap_or("");
                    output.push_str(&format!("  [{}] {}\n", move_type, line));

                    if let Some(ref t) = m.transport {
                        if let Some(ref name) = t.name {
                            output.push_str(&format!("    Line: {}\n", name));
                        }
                        if let Some(ref company) = t.company {
                            if let Some(ref cn) = company.name {
                                output.push_str(&format!("    Company: {}\n", cn));
                            }
                        }
                    }

                    if let Some(ref ft) = m.from_time {
                        output.push_str(&format!("    Depart: {}\n", ft));
                    }
                    if let Some(ref tt) = m.to_time {
                        output.push_str(&format!("    Arrive: {}\n", tt));
                    }
                    if let Some(t) = m.time {
                        output.push_str(&format!("    Time: {} min\n", t));
                    }
                }
            }
        }
        output.push('\n');
    }

    if response.items.is_empty() {
        output.push_str("No routes found.\n");
    }

    output
}
