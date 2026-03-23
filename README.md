# navitime-mcp

An MCP (Model Context Protocol) server that provides transit route search in Japan using the Navitime Route Transit API via RapidAPI.

## Build

```bash
cargo build --release
```

The binary will be at `target/release/navitime-mcp`.

## Configuration

### Environment Variable

Set your RapidAPI key:

```bash
export TOKEN_RAPIDAPI="your-rapidapi-key-here"
```

You can obtain a key by subscribing to the [Navitime Route Totalnavi API on RapidAPI](https://rapidapi.com/navitimejapan-navitimejapan/api/navitime-route-totalnavi).

### Claude Desktop Configuration

Add this to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "navitime": {
      "command": "/path/to/navitime-mcp",
      "env": {
        "TOKEN_RAPIDAPI": "your-rapidapi-key-here"
      }
    }
  }
}
```

## MCP Tools

### search_transit_route

Finds train/walking transit routes in Japan between two coordinates.

**Arguments:**

| Name         | Type   | Required | Description                                              |
|--------------|--------|----------|----------------------------------------------------------|
| `start_lat`  | number | Yes      | Start latitude                                           |
| `start_lon`  | number | Yes      | Start longitude                                          |
| `goal_lat`   | number | Yes      | Goal latitude                                            |
| `goal_lon`   | number | Yes      | Goal longitude                                           |
| `start_time` | string | No       | Departure time (YYYY-MM-DDTHH:MM:SS). Defaults to now.   |

## Testing

```bash
cargo test
```

## Architecture

This project follows Hexagonal Architecture (Ports and Adapters):

- `src/domain/` - Domain models and error types (no external dependencies)
- `src/ports/` - Trait definitions for inbound (use cases) and outbound (API clients)
- `src/use_cases/` - Application logic implementing inbound ports
- `src/adapters/outbound/` - Reqwest-based Navitime RapidAPI client
- `src/adapters/inbound/` - MCP server (JSON-RPC over stdio)
- `src/main.rs` - Dependency injection and runtime initialization
