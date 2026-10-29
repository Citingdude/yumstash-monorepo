# Yumstash

A monorepo starter with a Rust/Axum API, PostgreSQL via SQLx, and a Vue 3/TypeScript frontend. The API publishes an OpenAPI contract consumed by a typed `openapi-fetch` client.

## Development

Prerequisites: Rust and Cargo, Node.js 20.19+ or 22.12+, npm, and Docker Compose.

```sh
cp .env.example .env
docker compose up -d db
npm install
cargo run -p yumstash-api
```

In another terminal, run `npm run dev:web` and open the Vite URL. Vite proxies `/api` to the API at `127.0.0.1:3000`; production deployments should provide an equivalent same-origin proxy. The health endpoint is a liveness check, not a database readiness check. The PostgreSQL pool is initialized lazily for future repositories.

## API contract

With the API running, execute `npm run api:generate` to regenerate `web/src/api/schema.d.ts` from `/api/openapi.json`, then run `npm run typecheck`. Commit the generated types alongside API changes. The checked-in initial declaration is a minimal starter contract; regenerate it when the Rust API is available.

## Checks and migrations

Run `cargo test --workspace` and `npm run build`. Add versioned migrations under `api/migrations` and apply them with `sqlx migrate run --source api/migrations` (requires `sqlx-cli` and `DATABASE_URL`). Migrations are not run automatically. No authentication or application-specific data model is included yet.
