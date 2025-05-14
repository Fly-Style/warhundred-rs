# Warhundred Backend Development Guidelines

This document provides essential information for developers working on the Warhundred backend project.

## Build/Configuration Instructions

### Prerequisites
- Rust (version 1.84.1 or higher)
- SQLite
- Redis
- Docker (for integration tests)

### Setup
1. Clone the repository
2. Configure environment variables:
   ```
   DATABASE_URL=path/to/your/sqlite.db
   REDIS_URL=redis://localhost:6379
   ```
   These can be placed in a `.env` file at the project root.

### Development Build
To build the project in development mode:
```bash
cargo build
```

### Production Build
To build the project in release mode:
```bash
cargo build --release
```

### Running the Application
To run the application:
```bash
cargo run
```

Or with release optimizations:
```bash
cargo run --release
```

## Testing Information

### Testing Framework
The project uses:
- Rust's built-in testing framework
- rstest for test fixtures and organization
- axum_test for HTTP API testing
- testcontainers for integration tests with Redis

### Running Tests
To run all tests:
```bash
cargo test
```

To run tests with specific features:
```bash
cargo test --features local  # Use local Redis instead of containers
cargo test --features it_test  # Run integration tests
```

To run a specific test:
```bash
cargo test test_name
```

### Test Structure
- Unit tests are typically placed in the same file as the code they test
- Integration tests are placed in the `tests/` directory
- The `tests/common/mod.rs` file provides shared test utilities

### Writing Tests
Tests follow a standard Rust testing pattern:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        // Arrange
        let input = 42;
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }
}
```

For HTTP API tests, use the axum_test framework:

```rust
#[rstest]
#[tokio::test]
async fn test_api_endpoint(#[future] app: eyre::Result<App>) -> eyre::Result<()> {
    let App { pool_ref, server } = app.await?;

    // Make a request to the API
    let res = server
        .post("/endpoint")
        .add_header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .json(&serde_json::json!({ "key": "value" }))
        .await;

    // Assert on the response
    res.assert_status_ok();
    
    // Clean up
    after_test(pool_ref.clone()).await?;

    Ok(())
}
```

### Integration Tests with Containers
The project uses testcontainers for Redis integration tests. This requires Docker to be installed and running.

Test containers are automatically started and configured in the test fixtures:

```rust
#[fixture]
pub async fn app() -> eyre::Result<App> {
    // Start Redis container
    let redis = start_containers().await?;
    
    // Get connection URI
    let redis_url = redis_conn_uri(&redis).await?;
    
    // Set up application with test containers
    // ...
}
```

## Additional Development Information

### Project Structure
- `src/app/` - Core application code
- `src/app/routes/` - API routes
- `src/app/middleware/` - Middleware components
- `src/app/protos/` - Protocol Buffer definitions
- `tests/` - Integration tests

### Database
- The project uses Diesel ORM with SQLite
- Migrations are handled by diesel_migrations
- The database connection is managed through deadpool-diesel

### Protocol Buffers
The project uses Protocol Buffers for message definitions:
- Proto files are located in `src/app/protos/`
- The build.rs script compiles these files during the build process

### Error Handling
- The project uses thiserror and eyre for error handling
- Custom error types are defined in `src/app/error.rs`

### Linting and Code Style
- The project forbids unsafe code (`unsafe_code = "forbid"`)
- Dead code is allowed (`dead_code = "allow"`)
- Follow standard Rust formatting conventions (use `rustfmt`)
- Use `cargo clippy` for additional linting

### Async Runtime
- The project uses Tokio as the async runtime
- Async functions should be annotated with `#[tokio::main]` or `#[tokio::test]`

### Logging
- The project uses tracing for logging and diagnostics
- Configure the log level using the RUST_LOG environment variable