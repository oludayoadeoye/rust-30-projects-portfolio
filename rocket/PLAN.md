# Rocket Projects Implementation Plan (10 Projects)

This plan details the implementation of 10 backend projects using the **Rocket** framework.

## 1. Project Selection
| Category | # | Project Name | Description |
| :--- | :--- | :--- | :--- |
| Basic | 10 | `recipe-app` | Recipe management with ingredient scaling. |
| Basic | 12 | `contact-form` | Message ingestion and ticketing system. |
| Basic | 14 | `bmi-calculator` | User health metrics tracking. |
| Basic | 15 | `quote-generator` | Multi-author quote repository. |
| Basic | 17 | `tip-calculator` | Service record and payment calculation history. |
| Intermediate | 34 | `event-management` | Event scheduling and attendee tracking. |
| Intermediate | 35 | `movie-recommendation` | Movie meta-data and user rating CRUD. |
| Intermediate | 39 | `library-management` | Book catalog and member loan management. |
| Advanced | 50 | `home-automation` | IoT device status and command history. |
| Advanced | 53 | `finance-manager` | Budgeting, goals, and transaction ledger. |

## 2. Technical Standard
- **Framework**: Rocket 0.5 (Stable).
- **Database**: PostgreSQL 15 via Docker Compose.
- **Persistence**: `rocket_db_pools` + `sqlx`.
- **Docs**: `utoipa` + `utoipa-swagger-ui`.
- **Best Practices**: Using Request Guards for security, Sentinels for state verification, and Catchers for errors.

## 3. Workflow
1. `cargo new <name>`
2. Configure `Cargo.toml` and `Rocket.toml`.
3. Add `docker-compose.yml` for Postgres.
4. Implement migrations.
5. Develop logic using Rocket's attribute-based routing.
6. `cargo check` and fix any issues.
7. Git commit.
