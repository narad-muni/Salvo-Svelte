use super::super::controller::license_controller;
use super::super::controller::auth_controller;

use salvo::prelude::*;

pub fn open_routes() -> Router {
    let license_router = Router::with_path("license")
        .push(Router::with_path("status").get(license_controller::status))
        .push(Router::with_path("upload").post(license_controller::upload));

    let auth_router = Router::with_path("auth")
        .push(Router::with_path("login").post(auth_controller::login))
        .push(Router::with_path("status").get(auth_controller::status))
        .push(Router::with_path("logout").get(auth_controller::logout));

    Router::with_path("api")
        .push(license_router)
        .push(auth_router)
}
