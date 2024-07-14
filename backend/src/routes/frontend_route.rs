use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use std::env;

pub fn frontend_route() -> Router {
    if env::var("env").unwrap() == "development" {
        Router::with_path("<**path>").get(
            StaticDir::new(["../frontend", "../frontend/public"])
                .defaults("index.html")
                .auto_list(true),
        )
    } else {
        Router::with_path("<**path>").get(
            StaticDir::new(["./public"])
                .defaults("index.html")
                .auto_list(true),
        )
    }
}
