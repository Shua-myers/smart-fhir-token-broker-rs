# smart-fhir-token-broker-rs

Rust service implementing SMART-on-FHIR token brokering for internal healthcare integrations.

## Why
This service centralizes OAuth2/SMART token exchange and validation so downstream tools can avoid duplicating auth logic.

## Features
- SMART authorization code exchange
- Refresh token flow
- Token validation/introspection endpoint
- Scope enforcement middleware
- Structured logging and typed errors

## Architecture
- `handlers`: HTTP routes
- `services`: SMART/OAuth business logic
- `middleware`: scope checks, request metadata
- `models`: request/response and error types
- `config`: typed app configuration

## API
### `POST /token/exchange`
Exchange SMART auth code for access token.

### `GET /token/validate`
Validate token and return scope/expiry metadata.

### `GET /health`
Service health status.

## Quick Start
```bash
cp .env.example .env
cargo run
```

## Configuration
- `SMART_CLIENT_ID`
- `SMART_CLIENT_SECRET`
- `SMART_TOKEN_URL`
- `BIND_ADDR`

## Testing
```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## Security Notes
- Never log raw access/refresh tokens
- Store secrets outside source control
- Enforce least-privilege scopes

## Roadmap
- [ ] JWKS rotation support
- [ ] Enhanced audit event export
- [ ] Deployment helm chart

## License
MIT
