mod adapters;
mod domain;
mod ports;
mod use_cases;

use adapters::inbound::mcp_server::McpServer;
use adapters::outbound::rapidapi::NavitimeRapidApiClient;
use use_cases::search_route::SearchRouteService;

#[tokio::main]
async fn main() {
    let api_key = match std::env::var("TOKEN_RAPIDAPI") {
        Ok(key) if !key.is_empty() => key,
        _ => {
            eprintln!("Error: TOKEN_RAPIDAPI environment variable is not set.");
            eprintln!("Please set it to your RapidAPI key for the Navitime API.");
            std::process::exit(1);
        }
    };

    let client = NavitimeRapidApiClient::new(api_key);
    let service = SearchRouteService::new(client);
    let server = McpServer::new(service);

    if let Err(e) = server.run().await {
        eprintln!("MCP server error: {}", e);
        std::process::exit(1);
    }
}
