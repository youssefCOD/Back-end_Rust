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
   git clone https://github.com/yourusername/rust-web-server.git
   cd rust-web-server
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
```bash
/rust_web_server
├── rust_web_server
│   ├── __init__.rs
│   ├── settings.rs
│   ├── urls.rs
│   ├── views.rs
│   ├── models.rs
│   ├── middleware.rs
│   ├── static             # Static files
│   └── templates          # HTML templates
├── main.rs                # Entry point
├── Cargo.toml             # Project configuration
└── README.md              # Project documentation
```
# Usage

Routes
Define your application's routes in urls.rs. Map URLs to views in views.rs.

Views
Implement request handling logic in views.rs. Each view corresponds to a specific route.

Models
Define data models in models.rs to manage data within your application.

Middleware
Create middleware in middleware.rs to handle cross-cutting concerns like logging and authentication.


