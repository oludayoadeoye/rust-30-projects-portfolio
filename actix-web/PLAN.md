# Actix-web Projects Implementation Plan (10 Projects)

This plan details the implementation of 10 backend projects using the **Actix-web** framework.

## 1. Project Selection
| Category | # | Project Name | Description |
| :--- | :--- | :--- | :--- |
| Basic | 18 | `color-picker` | Palette management and hex code storage. |
| Basic | 20 | `flashcard-app` | Card/Deck management with mastery logs. |
| Basic | 24 | `url-shortener` | Alias generation and redirection analytics. |
| Basic | 27 | `lang-learning-app` | Vocab lists and lesson progress CRUD. |
| Basic | 28 | `simple-chatbot` | Pattern-based response configuration. |
| Intermediate | 41 | `restaurant-res` | Table booking and availability tracking. |
| Intermediate | 43 | `fitness-tracker` | Exercise logs and progress monitoring. |
| Intermediate | 45 | `job-board` | Job listing and applicant management. |
| Advanced | 56 | `healthcare-mgmt` | Patient records and appointment system. |
| Advanced | 61 | `smart-inventory` | Warehouse stock and SKU management. |

## 2. Technical Standard
- **Framework**: Actix-web 4.
- **Database**: PostgreSQL 15 via Docker Compose.
- **Persistence**: `sqlx` (async).
- **Docs**: `utoipa` + `utoipa-swagger-ui`.
- **Best Practices**: Module-based configuration (`cfg.configure`), `web::Data` for state, and custom error types implementing `ResponseError`.

## 3. Workflow
1. `cargo new <name>`
2. Setup `Cargo.toml` and `docker-compose.yml`.
3. Create migrations.
4. Implement `models.rs`, `handlers.rs`, and modular configuration.
5. `cargo check` and fix any issues.
6. Git commit.
