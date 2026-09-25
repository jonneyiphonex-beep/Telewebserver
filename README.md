# Telewebserver

Telewebserver is a lightweight Rust + Axum telecom control server that exposes a small set of telecom-related endpoints and a browser dashboard for controlling SIM, VoLTE, APN, and messaging behavior.

## What this project does

The project simulates a telecom control surface for testing and demonstration purposes. It includes:

- SIM slot switching
- VoLTE state toggling
- APN configuration
- Messaging dispatch
- A simple web UI served directly by the backend

The main goal is to provide a compact and understandable telecom microservice with a minimal web interface.

## Project structure

- `src/main.rs` — server startup and route registration
- `src/state.rs` — in-memory telecom engine state
- `src/models/` — request/response payload models
- `src/models/handles/` — HTTP handlers for each endpoint
- `src/models/services/` — business logic for SIM, VoLTE, and messaging
- `web/` — HTML, CSS, and JavaScript for the dashboard UI

## Requirements

- Rust toolchain
- Cargo
- A modern browser for the dashboard

## Run the project

From the project root:

```bash
source "$HOME/.cargo/env"
cargo run
```

Then open:

```text
http://localhost:8080/
```

## API endpoints

### SIM switch

POST `/sim/switch/:slot_id`

Example:

```bash
curl -X POST http://127.0.0.1:8080/sim/switch/2
```

### VoLTE switch

POST `/volte/data-switch`

Example:

```bash
curl -X POST http://127.0.0.1:8080/volte/data-switch \
  -H "content-type: application/json" \
  -d "true"
```

### APN config

POST `/apn/config`

Example:

```bash
curl -X POST http://127.0.0.1:8080/apn/config \
  -H "content-type: application/json" \
  -d '{
    "name": "Enterprise",
    "apn": "data.operator",
    "os_target": "Android",
    "pdn_type": "IPv4v6"
  }'
```

For iPhone devices, the dashboard can automatically detect iOS and include an extra Cellular & LTE setup section with APN, username, and password values.

### Messaging send

POST `/messaging/send`

Example:

```bash
curl -X POST http://127.0.0.1:8080/messaging/send \
  -H "content-type: application/json" \
  -d '{
    "from": "+15550100",
    "to": "+15550199",
    "content": "Hello from Telewebserver",
    "is_rcs": true
  }'
```

## Dashboard features

The browser UI includes:

- SIM switching controls
- VoLTE toggle
- APN configuration form
- Messaging form
- Automatic OS detection
- iPhone Cellular & LTE credential fields
- Diagnostic path map for latency tracing
- Live connection telemetry

## Notes

This is a demo telecom control service and uses in-memory state rather than a real carrier backend. The server is intended for local development and learning, not production telecom deployment.

## License

This project is provided as-is for educational and demonstration purposes.
