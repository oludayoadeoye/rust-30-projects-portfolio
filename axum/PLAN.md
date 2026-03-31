# Axum Projects Implementation Plan (10 Projects)

This plan details the implementation of 10 backend projects using the **Axum** framework.

## 1. Project Selection
| Category | # | Project Name | Description |
| :--- | :--- | :--- | :--- |
| Basic | 1 | `todo-list` | Task management with Postgres persistence. |
| Basic | 2 | `weather-app` | Weather data CRUD (simulated/cached). |
| Basic | 3 | `calculator` | History-tracking calculator CRUD. |
| Basic | 4 | `currency-converter` | Currency rate management and conversion CRUD. |
| Basic | 5 | `notes-app` | Categorized notes with tag search. |
| Intermediate | 31 | `chat-app` | Message and room management CRUD. |
| Intermediate | 32 | `ecommerce-platform` | Product, Category, and Inventory management. |
| Intermediate | 33 | `task-manager` | Team-based task assignments and status. |
| Advanced | 46 | `realtime-collab` | Document state synchronization CRUD. |
| Advanced | 49 | `blockchain-voting` | Vote ledger with immutable characteristics. |

## 2. Technical Standard
- **Framework**: Axum (with Tokio runtime).
- **Database**: PostgreSQL 15 via Docker Compose.
- **Persistence**: `sqlx` (async).
- **Docs**: `utoipa` + `utoipa-swagger-ui`.
- **Best Practices**: Using `State` extractor, `IntoResponse` for errors, and correct extractor ordering.

## 3. Workflow
1. `cargo new <name>`
2. Setup `Cargo.toml` and `docker-compose.yml`.
3. Create migrations.
4. Implement `models.rs`, `handlers.rs`, and `main.rs`.
5. `cargo check` and fix any issues.
6. Git commit.
