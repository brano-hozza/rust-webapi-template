# Rust WebApi template

## Motivation

I was trying to use rust for my backend service,
but I couldn't find any template for it. So I stole some code from multiple
repositories and managed to create something.

### Sources

- [Clean architecture rust](https://github.com/MSC29/clean-architecture-rust/tree/main)
- [Actix example](https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel)

## Setup

- First of all you need to install `rust` and `cargo` on your machine.
- Then you need to install `diesel_cli` to manage your database.
- You need to create `.env` file in the root directory of the project based on example file.

### Start a server

```bash
cargo run
```

## Folder structure

- `migrations`: Database migrations
- `src`: Source code
    - `controllers`: Controllers logic (Path matching, etc.)
    - `dtos`: Data transfer objects
    - `middlewares`: Middlewares
    - `models`: Database models
    - `repositories`: Database repositories
    - `services`: Business logic
    - `utils`: Utilities
        - `db.rs`: Database connection
        - `di.rs`: Dependency injection (DI) container
        - `error.rs`: Error definitions
        - `hasher.rs`: Password hasher
    - `main.rs`: Main file
    - `app_state.rs`: App state object definition (DI registration)
    - `constants.rs`: Global constants (Env keys, etc.)
    - `schema.rs`: Auto-generated database schema definition

### Controllers

All controllers are defined in `src/controllers/mod.rs` file.
You can create a new controller for each entity in your system.

### DTOs

All objects that are coming into the system
should be defined as an DTO in `src/dtos/mod.rs` file.

### Middlewares

All middleware should be defined in `src/middlewares` folder
and registered in the `src/main.rs` file.

### Models

All database models should be defined in `src/models` directory.

### Repositories

Every interaction with the `diesel` database should be defined in `src/repositories` folder.
Every new repository should be registered in `src/utils/di.rs` file.

### Services

All the business logic should be defined in the service file.
Services should be also registered in DI container.

### Utilities

Utilities should be located in `src/utils` folder.

## Database

This template uses `diesel` with Postgres as an ORM.

### Create new migration

Be aware that diesel uses DATABASE_URL environment variable saved in `.env` file to connect to the database.

```bash
diesel migration generate <migration_name>
```

### Apply the migration

```bash
diesel migration run
```
