# navitime-mcp

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![MCP](https://img.shields.io/badge/MCP-compatible-blue.svg)](https://modelcontextprotocol.io/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A lightweight [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server written in Rust that enables AI assistants to search **transit routes across Japan** — trains, subways, bullet trains, buses, walking, and more — powered by the [Navitime API](https://rapidapi.com/navitimejapan-navitimejapan/api/navitime-route-totalnavi) via RapidAPI.

## What it does

Give any AI assistant (Claude, GPT, etc.) the ability to plan real transit trips in Japan:

```
User: "How do I get from Tokyo Station to Shibuya at 8am?"
AI:   [calls search_transit_route] →
      Route 1: JR Yamanote Line (local_train)
        Tokyo → Shibuya | 26 min | 200 yen (IC: 198 yen)
```

The server returns structured route data including fare breakdown, transfer count, walking distance, departure/arrival times, line colors, and company information.

## Features

- **Full Japan coverage** — JR, private railways, Tokyo Metro, subways, Shinkansen, buses, domestic flights
- **Rich route details** — fares (cash + IC card), transfers, distances, travel times, platform info
- **Transport metadata** — line colors, company names, station IDs, node types
- **MCP protocol** — works with any MCP-compatible AI client over stdio (JSON-RPC 2.0)
- **Hexagonal architecture** — clean separation of domain, ports, adapters, and use cases
- **Fully tested** — unit tests with mocks + API response deserialization fixtures
- **Fast** — native Rust binary, async I/O with Tokio

## Quick Start

### 1. Get a RapidAPI key

Subscribe to the [Navitime Route Totalnavi API](https://rapidapi.com/navitimejapan-navitimejapan/api/navitime-route-totalnavi) on RapidAPI (free tier available).

### 2. Build

```bash
git clone https://github.com/GHagui/mcp-navitime-rust.git
cd mcp-navitime-rust
cargo build --release
```

### 3. Configure your AI client

#### Claude Desktop

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "navitime": {
      "command": "/path/to/target/release/navitime-mcp",
      "env": {
        "TOKEN_RAPIDAPI": "your-rapidapi-key"
      }
    }
  }
}
```

#### Claude Code

Add to `.claude/settings.json` or configure via Claude Code settings:

```json
{
  "mcpServers": {
    "navitime": {
      "command": "/path/to/target/release/navitime-mcp",
      "env": {
        "TOKEN_RAPIDAPI": "your-rapidapi-key"
      }
    }
  }
}
```

#### Any MCP-compatible client

The server communicates over **stdio** using JSON-RPC 2.0. Set the `TOKEN_RAPIDAPI` environment variable and run the binary.

## MCP Tools

### `search_transit_route`

Finds train/walking transit routes in Japan between two geographic coordinates.

| Parameter    | Type   | Required | Description                                                    |
|-------------|--------|----------|----------------------------------------------------------------|
| `start_lat` | number | Yes      | Starting point latitude (e.g., `35.6812`)                     |
| `start_lon` | number | Yes      | Starting point longitude (e.g., `139.7671`)                   |
| `goal_lat`  | number | Yes      | Destination latitude                                           |
| `goal_lon`  | number | Yes      | Destination longitude                                          |
| `start_time`| string | No       | Departure time in `YYYY-MM-DDTHH:MM:SS` format. Defaults to now |

#### Example output

```
=== Route 1 ===
Total time: 13 min
Fare: 170 yen
IC Fare: 165 yen
Distance: 5900 m
Transfers: 0
Departure: 2025-10-01T08:01:00+09:00
Arrival: 2025-10-01T08:14:00+09:00

Sections:
  [Point] Omotesando
  [local_train] Tokyo Metro Ginza Line
    Line: Tokyo Metro Ginza Line
    Company: Tokyo Metro
    Depart: 2025-10-01T08:01:00+09:00
    Arrive: 2025-10-01T08:14:00+09:00
    Time: 13 min
  [Point] Ginza
```

#### Response data includes

- **Summary**: total time, distance, fare (normal + IC card), transfer count, walk distance
- **Sections**: alternating point/move segments with:
  - Station names, coordinates, node IDs
  - Transport details: line name, color, company, direction
  - Move type: `walk`, `local_train`, `rapid_train`, `superexpress_train`, `domestic_flight`, etc.
  - Timing: departure and arrival per segment

## Architecture

```
src/
├── main.rs                          # Entry point & dependency injection
├── domain/
│   ├── models.rs                    # Route, Section, Transport, Fare types
│   └── error.rs                     # AppError enum (API, network, parsing, validation)
├── ports/
│   ├── inbound.rs                   # RouteSearchUseCase trait
│   └── outbound.rs                  # TransitApiClient trait (+ mock for tests)
├── use_cases/
│   └── search_route.rs             # Business logic + input validation + tests
└── adapters/
    ├── inbound/
    │   └── mcp_server.rs           # MCP JSON-RPC server (stdio) + response formatting
    └── outbound/
        └── rapidapi.rs             # Navitime RapidAPI HTTP client
```

The project follows **Hexagonal Architecture** (Ports & Adapters):

- **Domain** — Pure data models and error types, no external dependencies
- **Ports** — Trait interfaces defining inbound (use cases) and outbound (API clients) boundaries
- **Use Cases** — Application logic with coordinate validation; auto-defaults departure time to now
- **Adapters** — Concrete implementations: MCP server (inbound) and RapidAPI client (outbound)

## Testing

```bash
cargo test
```

Tests include:
- Route search success flow with mock API client
- Input validation (invalid coordinates)
- API error propagation
- Default time generation when not provided
- JSON deserialization of real Navitime API response fixtures

## Supported transport types

| Type | Description |
|------|-------------|
| `walk` | Walking |
| `local_train` | Local/regular trains |
| `rapid_train` | Rapid/express (fare-free) trains |
| `superexpress_train` | Shinkansen (bullet trains) |
| `ultraexpress_train` | Limited express trains |
| `express_train` | Express trains |
| `local_bus` | Local buses |
| `highway_bus` | Highway buses |
| `domestic_flight` | Domestic flights |
| `ferry` | Ferry |
| `car` | Car |
| `bicycle` | Bicycle / share cycle |

## Requirements

- Rust 1.70+
- A [RapidAPI key](https://rapidapi.com/navitimejapan-navitimejapan/api/navitime-route-totalnavi) for the Navitime Route Totalnavi API

## License

MIT
