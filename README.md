# Yumstash

A monorepo starter with a Rust/Axum API, PostgreSQL via SQLx, and a Vue 3/TypeScript frontend. The API publishes an OpenAPI contract consumed by a typed `openapi-fetch` client.

## Development

Prerequisites: Rust and Cargo, Node.js 20.19+ or 22.12+, pnpm 12.16.1, and Docker Compose.

```sh
cp .env.example .env
docker compose up -d db
pnpm install
cargo run -p yumstash-api
```

In another terminal, run `pnpm dev:web` and open the Vite URL. Vite proxies `/api` to the API at `127.0.0.1:3000`; production deployments should provide an equivalent same-origin proxy. The API connects to PostgreSQL and applies migrations on startup. The health endpoint is a liveness check.

## API contract

With the API running, open `http://127.0.0.1:3000/docs` for interactive Swagger UI documentation. The UI loads its JavaScript and CSS from unpkg, so the browser needs internet access. The OpenAPI document is also available at `/api/openapi.json`.

With the API running, execute `pnpm api:generate` to regenerate `apps/web/src/api/schema.d.ts` from `/api/openapi.json`, then run `pnpm typecheck`. Commit the generated types alongside API changes. The checked-in declaration is generated from the Rust API.

## Checks and migrations

Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and `pnpm build`. Install the Rust formatting and lint components with `rustup component add rustfmt clippy`, and run `cargo fmt --all` to apply the workspace formatting settings in `rustfmt.toml`. Add versioned migrations under `apps/api/migrations` and apply them with `sqlx migrate run --source apps/api/migrations` (requires `sqlx-cli` and `DATABASE_URL`). Migrations also run automatically when the API starts. Recipes support listing, viewing, creating, updating, and deleting. No authentication is included yet.

The JavaScript packages use pnpm workspaces; the Rust API uses the Cargo workspace. Turbo is not needed for one frontend package and can be added if the repository grows enough to benefit from task orchestration and caching.

## API structure

The API is organized into modules under `apps/api/src/modules`. Each module exposes its router and OpenAPI document through `mod.rs` and keeps a route overview in `routes.rs`. Operations live under `use_cases/<operation>/`, with their HTTP handler and operation-specific tests together. Shared request and response types belong in `models.rs`; modules that access the database keep shared queries in `repository.rs`. Module-wide tests live beside the code they test. Shared HTTP error types live in `error.rs`; startup and middleware live in `main.rs` and `app.rs`. Register each module's router and merge its OpenAPI document in `app.rs`.
