use super::super::globals::MEMORY_SESSION;
use super::super::middleware::MemoryStore;

use salvo::session::SessionHandler;

pub fn session_middleware() -> SessionHandler<MemoryStore> {
    SessionHandler::builder(
        MemoryStore::new(MEMORY_SESSION.to_owned()),
        b"secrkeybabsecretabsecretabrandometabsecretabsecreaaecretabsecretab",
    )
    .build()
    .unwrap()
}
