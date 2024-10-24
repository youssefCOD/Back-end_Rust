# Rust Web Server

## Description

A high-performance HTTP server built with Rust, featuring an organized file structure similar to Django, making it easy to manage routes, views, models, and middleware.

## Features

- Asynchronous processing for handling concurrent requests
- Modular file structure resembling Django
- Static file serving capabilities
- Dynamic HTML template rendering
- Custom middleware support

## Getting Started

### Prerequisites

- Rust (version 1.56 or later)
- Cargo (Rust package manager)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/youssefCOD/Back-end_Rust
   cd Back-end_Rust
   ```

2.Build the project:

```bash
cargo build
```

3.Run the server:

```bash
cargo run
```

File Structure

my_rust_backend/
│
├── src/
│   ├── main.rs                # The main entry point of the application
│   ├── lib.rs                 # (optional) If you are building a library alongside the app
│   ├── config/                # Configuration files and settings
│   │   └── mod.rs             # Config module for handling application settings
│   │   └── database.rs        # Database configuration (e.g., connection pool)
│   ├── routes/                # Route handlers or controllers
│   │   └── mod.rs             # Entry point for routes
│   │   └── auth.rs            # Authentication routes (e.g., login, register)
│   │   └── users.rs           # User-specific routes (e.g., user profile, CRUD)
│   └── services/              # Business logic and service layer
│   │   └── mod.rs             # Entry point for services
│   │   └── auth_service.rs    # Authentication-related logic (JWT, password hashing)
│   └── models/                # Data models and database schema
│   │   └── mod.rs             # Entry point for models
│   │   └── user.rs            # User model, typically using ORM (e.g., Diesel, SQLx)
│   └── utils/                 # Utility functions and helpers
│   │   └── mod.rs             # Entry point for utilities
│   │   └── hash.rs            # Utility function for hashing passwords
│   └── middleware/            # Middleware for handling requests (e.g., logging, authentication)
│   │   └── mod.rs             # Middleware module entry point
│   └── errors/                # Centralized error handling
│       └── mod.rs             # Error types and error handlers
│
├── tests/                     # Integration and unit tests
│   └── integration_test.rs    # Example integration test
│
├── migrations/                # Database migrations (if using an ORM like Diesel)
│
├── Cargo.toml                 # Cargo manifest file (dependencies, metadata)
└── .env                       # Environment variables (database URL, API keys)

## Usage

### Routes

Define your application's routes in urls.rs. Map URLs to views in views.rs.

### Views

Implement request handling logic in views.rs. Each view corresponds to a specific route.

### Models

Define data models in models.rs to manage data within your application.

### Middleware

Create middleware in middleware.rs to handle cross-cutting concerns like logging and authentication
