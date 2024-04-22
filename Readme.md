# Rust API with Axum
This repository is a workspace where I learn how to build an API server using Rust (Axum).

## VSCode extensions
1. Rust-analyzer
2. Error Lens
3. Even Better TOML
4. Crates
5. Thunder Client

## Hot reload
1. Installation
`cargo install cargo-watch`
2. Usage
`cargo watch -x run`

## Package installation examples
1. `cargo add axum`
2. `cargo add tokio -F macros -F rt-multi-thread`
3. `cargo add serde -F derive`

## Lesson learned
1. <a href="./src/lib.rs">Setting up an API server with Axum (Rust)</a>
2. Route creation
3. Http request handling
4. JSON data handling with `serde`
5. Route params handling
6. Query params handling
7. Headers handling
8. Middlewares + Customization
9. Status Codes and response
10. Input data validation with `validator`

## In progress
1. Database connection
2. Authentication (JWT)
3. Route groping
4. Http client
5. Websocket client
6. Websocker server (optional)
7. Deployment