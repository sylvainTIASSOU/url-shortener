What We're Building
A URL shortener API — a REST service that creates short codes for long URLs, tracks click counts, and returns redirect targets. Simple enough to fit in one course, real enough to exercise every concept.

By the end you'll have a production-shaped Rust service with proper structure, async handlers, JSON serialization, database access, middleware, error handling, testing, and deployment.

What You'll Learn
Project structure: how to organize an Axum service without over-engineering
Axum: routing, extractors, state sharing, and the Handler trait
Serde: JSON serialization and deserialization with zero boilerplate
Error handling: thiserror, custom error types, and consistent API responses
Configuration: environment variables and .env files with dotenvy
Middleware: logging, request IDs, and CORS with tower
SQLx: async PostgreSQL with compile-time query checking
Database migrations: sqlx migrate for schema management
Testing: unit tests, integration tests, and testing handlers directly
Graceful shutdown: catching signals and draining in-flight requests
Deployment: Docker, environment configuration, and health checks

curl -X POST http://localhost:3000/links \
-H "Content-Type: application/json" \
-d '{"url":"https://rust-lang.org"}'