use salvo::prelude::*;
use salvo::session::{Session, SessionDepotExt};
use super::super::globals;

#[endpoint]
pub fn login(depot: &mut Depot) -> String {
    let mut session = Session::new();
    session
        .insert("username", "Logged In")
        .unwrap();

    session
        .insert("user_id", "Hello")
        .unwrap();
    depot.set_session(session);

    "Logged In".to_owned()
}

#[endpoint]
pub async fn status(depot: &mut Depot) -> String {
    let session = globals::memory_session::MEMORY_SESSION.as_ref();
    let session = session.write().await;

    println!("{:?} {:?}",session.get(""), session.keys());
    println!("{:?}", globals::memory_session::USER_MAP.as_ref());
    
    if let Some(session) = depot.session_mut() {
        session.get::<String>("username").unwrap_or("Logged Out".to_owned())
    }else{
        "Logged Out".to_owned()
    }
}

#[endpoint]
pub fn logout(depot: &mut Depot) -> String {
    if let Some(session) = depot.session_mut() {
        session.remove("username");
    }

    "Logged Out".to_owned()
}