use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    pub kty: String,
    pub kid: String,
    #[serde(rename(deserialize = "use"))]
    pub use_claim: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}
