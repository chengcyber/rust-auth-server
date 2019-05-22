use actix::prelude::*;
use actix_web::{http::Method, middleware, App};
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use chrono::Duration;
use crate::models::DbExecutor;
use crate::invitation_routes::{register_email};
use crate::register_routes::{register_user};
use crate::auth_routes::{login, logout, get_me};

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

// helper function to create and returns the app after mounting all routes/resources
pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {

    // secret is a random 32 character logn base 64 string
    let secret: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0".repeat(32));
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    App::with_state(AppState { db })
        // setup buildin logger to get nice logging for each request
        .middleware(middleware::Logger::new("\"%r\" %s %b %Dms"))
        .middleware(IdentityService::new(
            CookieIdentityPolicy::new(secret.as_bytes())
                .name("auth")
                .path("/")
                .domain(domain.as_str())
                .max_age(Duration::days(1)) // just for testing
                .secure(false)
        ))
        // routes for authentication
        .resource("/auth", |r| {
            r.method(Method::POST).with(login);
            r.method(Method::DELETE).with(logout);
            r.method(Method::GET).with(get_me);
        })
        // routes to invitation
        .resource("/invitation", |r| {
            r.method(Method::POST).with(register_email);
        })
        // routes to register as a user
        .resource("/register/{invitation_id}", |r| {
            r.method(Method::POST).with(register_user);
        })
}
