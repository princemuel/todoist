//! # Settings Configuration System
//!
//! This crate provides a comprehensive configuration system for the
//! application, supporting multiple environments (local, production, test) with
//! layered configuration loading and environment-specific overrides.
//!
//! ## Architecture
//!
//! The configuration system is organized into focused modules:
//! - **error**: Error types for configuration operations
//! - **env**: Environment detection and parsing
//! - **loader**: Configuration file loading logic
//! - **server**: Server-specific settings
//! - **database**: Database connection and pool settings
//! - **cache**: Redis cache settings
//! - **elasticsearch**: Elasticsearch integration settings
//! - **auth**: Authentication and JWT settings
//! - **settings**: Main Settings struct aggregating all sub-configurations
//! - **prelude**: Commonly used items for convenient importing
//!
//! ## Configuration Loading Order
//!
//! Settings are loaded in this order (later sources override earlier):
//! 1. Built-in defaults from struct implementations
//! 2. `Config.toml` (global defaults for all environments)
//! 3. Environment-specific file:
//!    - `config/local.toml` for development
//!    - `config/prod.toml` for production
//!    - `config/test.toml` for testing
//! 4. Environment variables prefixed with `APP_` (e.g.,
//!    `APP_DATABASE__PASSWORD`)
//!
//! ## Environment Selection
//!
//! Set the environment via the `APP_ENVIRONMENT` variable:
//! ```bash
//! export APP_ENVIRONMENT=local      # Development (default)
//! export APP_ENVIRONMENT=prod       # Production
//! export APP_ENVIRONMENT=test       # Testing
//! ```
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use settings::prelude::*;
//!
//! fn main() {
//!     // Access the globally loaded configuration
//!     let db_url = CONFIG.database.url();
//!     let server_port = CONFIG.server.port;
//! }
//! ```

mod auth;
mod cache;
mod database;
mod elasticsearch;
mod env;
mod error;
mod loader;
pub mod prelude;
mod server;
mod settings;
