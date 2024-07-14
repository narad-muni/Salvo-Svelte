use crate::{async_trait, SessionStore};
use async_lock::RwLock;
use async_session::{Result, Session};
use std::{collections::HashMap, sync::Arc};

/// # in-memory session store
/// Because there is no external
/// persistance, this session store is ephemeral and will be cleared
/// on server restart.
///
/// # ***DO NOT USE THIS IN A PRODUCTION DEPLOYMENT.***
#[derive(Debug, Clone)]
pub struct MemoryStore {
    pub inner: Arc<RwLock<HashMap<String, Session>>>,
}

#[async_trait]
impl SessionStore for MemoryStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        let id = Session::id_from_cookie_value(&cookie_value)?;
        Ok(self
            .inner
            .read()
            .await
            .get(&id)
            .cloned()
            .and_then(Session::validate))
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        self.inner
            .write()
            .await
            .insert(session.id().to_string(), session.clone());

        session.reset_data_changed();
        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result {
        self.inner.write().await.remove(session.id());
        Ok(())
    }

    async fn clear_store(&self) -> Result {
        self.inner.write().await.clear();
        Ok(())
    }
}

impl MemoryStore {
    /// Create a new instance of MemoryStore
    pub fn new(memory_session: Arc<RwLock<HashMap<String, Session>>>) -> Self {
        Self {
            inner: memory_session,
        }
    }
}
