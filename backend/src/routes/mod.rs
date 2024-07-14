mod frontend_route;
mod open_routes;
mod protected_routes;

use frontend_route::frontend_route;
use open_routes::open_routes;
use protected_routes::protected_routes;

use crate::middleware::session_middleware;

use salvo::prelude::*;

pub fn get_router() -> Router {
    let api_router = Router::new()
        .hoop(session_middleware())
        .push(open_routes())
        .push(protected_routes());

    let doc = OpenApi::new("MehtaG API", "Development").merge_router(&api_router);

    api_router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
        .push(frontend_route())
}
