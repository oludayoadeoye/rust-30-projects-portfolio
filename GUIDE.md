# 30 Rust Backend Projects - Master Guide

This repository contains 30 complete Rust backend projects across three frameworks: **Axum**, **Rocket**, and **Actix-web**.

## **Prerequisites**
- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install) (Edition 2021).
- **Docker**: Required for PostgreSQL/SQLite containers.
- **SQLx CLI**: (Optional) For manual migration management: `cargo install sqlx-cli`.

---

## **Standard Project Structure**
Each project follows this clean, modular architecture:
- `src/main.rs`: Entry point, server configuration, and route registration.
- `src/models.rs`: Data structures, database schema mapping (SQLx), and OpenAPI schemas (Utoipa).
- `src/handlers.rs`: Business logic and HTTP route handlers.
- `migrations/`: SQL files for automatic database schema setup.
- `docker-compose.yml`: Isolated database environment configuration.

---

## **How to Run Any Project**

1. **Navigate to the project directory**:
   ```bash
   cd RustPractise/<framework>/<project-name>
   ```

2. **Start the Database (PostgreSQL)**:
   Ensure your Docker Desktop/Daemon is running.
   ```bash
   docker-compose up -d
   ```
   *Note: Each project uses a unique port (5432-5461) to allow multiple databases to run simultaneously.*

3. **Run the Application**:
   The app will automatically run migrations on startup.
   ```bash
   cargo run
   ```

4. **Access Swagger UI (Documentation)**:
   Open your browser to:
   ```text
   http://127.0.0.1:<port>/swagger-ui
   ```
   - **Axum Ports**: 3000-3009
   - **Rocket Ports**: 8000 (standard) or specified in `Rocket.toml`
   - **Actix Ports**: 8080-8089

---

## **Project List & Port Mapping**

| # | Framework | Project | DB Port | App Port |
| :--- | :--- | :--- | :--- | :--- |
| 1-10 | Axum | `todo-list` to `blockchain-voting` | 5432-5441 | 3000-3009 |
| 11-20 | Rocket | `recipe-app` to `finance-manager` | 5442-5451 | 8000 |
| 21-30 | Actix | `color-picker` to `smart-inventory` | 5452-5461 | 8080-8089 |

---

## **Features included in every project**
- ✅ **Full CRUD**: Create, Read, Update, Delete logic.
- ✅ **Async DB**: Powered by `sqlx` and `tokio`.
- ✅ **Auto-Docs**: Interactive Swagger UI via `utoipa`.
- ✅ **Type Safety**: Strong Rust typing for requests and responses.
- ✅ **Docker Ready**: One-command database setup.
