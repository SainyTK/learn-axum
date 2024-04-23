# Rust API with Axum
This repository is a workspace where I learn how to build an API server using Rust (Axum).

## VSCode extensions
1. Rust-analyzer
2. Error Lens
3. Even Better TOML
4. Crates
5. Thunder Client

## Running the project
1. Installation <br/>
`cargo install cargo-watch`
2. Usage <br/>
`cargo watch -x run`

## Connecting to DB through SeaORM CLI
1. Installation
`cargo install sea-orm-cli`
2. Usage <br/>
`sea-orm-cli `

## Package installation examples
1. `cargo add axum`
2. `cargo add tokio -F macros -F rt-multi-thread`
3. `cargo add serde -F derive`

## Lesson learned
1. <a href="./src/lib.rs">Setting up an API server with Axum (Rust)</a>
2. <a href="./src/routes/mod.rs">Route creation</a>
3. <a href="./src/routes/mod.rs">Http request handling</a>
4. <a href="src/routes/mirror_json.rs">JSON data handling with `serde`</a>
5. <a href="src/routes/path_variables.rs">Route params handling</a>
6. <a href="src/routes/query_params.rs">Query params handling</a>
7. <a href="src/routes/mirror_custom_headers.rs">Headers handling</a>
8. <a href="src/routes/middleware_data.rs">Middlewares + Customization</a>
9. <a href="src/routes/return_201.rs">Status Codes and response</a>
10. <a href="src/routes/custom_json_extractor.rs"> Input data validation with `validator`</a>

## In progress
1. Database connection (Doing)
2. Authentication (JWT)
3. Route groping
4. Http client
5. Websocket client
6. Websocker server (optional)
7. Deployment