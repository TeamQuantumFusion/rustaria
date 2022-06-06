use rsa_core::ty::RawId;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct PrototypeComp(pub RawId);