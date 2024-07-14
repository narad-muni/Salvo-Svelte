use super::super::controller::employee_controller;

use salvo::prelude::*;

pub fn protected_routes() -> Router {
    let employee_router =
        Router::with_path("employee").push(Router::new().get(employee_controller::list));

    Router::with_path("api").push(employee_router)
}
