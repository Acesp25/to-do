use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Reoccurance {
    Yearly,
    Monthly,
    Fornite,
    Weekly,
    Daily,
    None,
}