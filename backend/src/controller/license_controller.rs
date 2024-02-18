use salvo::prelude::*;

#[endpoint]
pub fn status() -> &'static str {
    "Hello World"
}

#[endpoint]
pub fn upload() {

}