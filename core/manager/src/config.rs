use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct ManagerConfig {
    #[allow(unused)]
    pub(crate) identifier: String,
}
