# Rust Web Server

## Description

A high-performance HTTP server built with Rust, featuring an organized file structure similar to Django, making it easy to manage routes, views, models, and middleware.

## Features

- Asynchronous processing for handling concurrent requests
- Modular file structure resembling Django
- Static file serving capabilities

## Getting Started

## Rules

### Naming Convention

- variables & functions -> snake_case
- constants -> UPPER_SNAKE_CASE
- structs -> UpperCamelCase

Certainly! Here’s a README section you can add for documenting code effectively:

---

### Documentation Guidelines

Effective documentation helps keep code understandable without adding unnecessary clutter. Here are the key principles we follow:

1. **Avoid Unnecessary Documentation**  
   - **Self-Explaining Code**: Avoid documenting language features, obvious code, or well-documented external libraries. Favor clear, descriptive naming over comments.
   - **Concise and Focused**: Keep comments short and relevant. Explain why the code exists, not what each line does.

2. **Comment for Context, Not Code**  
   - **Explain Decisions**: Comment on the *reasoning* behind non-obvious decisions, edge cases, or logic that may seem counterintuitive.
   - **Highlight Edge Cases**: Document any assumptions, edge cases, or specific constraints that could affect functionality.

3. **Summarize Complex Logic**  
   - **High-Level Overviews**: Use comments to summarize complex algorithms or intricate logic rather than line-by-line breakdowns.

4. **External Documentation for Extensive Details**  
   - **Separate High-Level Explanations**: For broader overviews or architectural decisions, consider using separate markdown files or external documents to avoid crowding the code.

5. **Focus on Public Interfaces**  
   - **Document Public APIs**: Emphasize documentation on public methods or properties, especially those essential for understanding and using the class’s purpose.

6. **Use TODO/FIXME with Purpose**  
   - **Be Clear and Actionable**: Use TODO or FIXME tags thoughtfully, specifying the issue and providing context or a suggested solution if possible.

7. **Regularly Review and Update Documentation**  
   - **Stay Up-to-Date**: Treat documentation as part of the codebase. Regularly review it to ensure it remains accurate and relevant as the code evolves. Outdated comments can cause confusion and should be removed or updated.

Following these guidelines will help maintain clean, concise, and useful documentation that adds value without unnecessary complexity.

--- 

These rules should keep your codebase well-documented, focusing on clarity and purpose.

## Prerequisites

- Rust (version 1.56 or later)
- Cargo (Rust package manager)

## Installation

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

Rust Web Server/
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
│   └── utils/                 # Utility functions and helpers
│   │   └── mod.rs             # Entry point for utilities
│   │   └── hash.rs            # Utility function for hashing passwords
│   └── middleware/            # Middleware for handling requests (e.g., logging, authentication)
│   │   └── mod.rs             # Middleware module entry point
│   └── errors/                # Centralized error handling
│       └── mod.rs             # Error types and error handlers
│
|__ tests.py
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
