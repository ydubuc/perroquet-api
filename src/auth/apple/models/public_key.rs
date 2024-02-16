use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    #[serde(rename(deserialize = "kty"))]
    pub kty: String,
    #[serde(rename(deserialize = "kid"))]
    pub kid: String,
    #[serde(rename(deserialize = "use"))]
    pub use_claim: String,
    #[serde(rename(deserialize = "alg"))]
    pub alg: String,
    #[serde(rename(deserialize = "n"))]
    pub n: String,
    #[serde(rename(deserialize = "e"))]
    pub e: String,
}
