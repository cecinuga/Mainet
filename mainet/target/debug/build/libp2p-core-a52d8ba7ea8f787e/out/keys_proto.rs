#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    #[prost(enumeration = "KeyType", required, tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", required, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKey {
    #[prost(enumeration = "KeyType", required, tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", required, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyType {
    Rsa = 0,
    Ed25519 = 1,
    Secp256k1 = 2,
    Ecdsa = 3,
}
impl KeyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyType::Rsa => "RSA",
            KeyType::Ed25519 => "Ed25519",
            KeyType::Secp256k1 => "Secp256k1",
            KeyType::Ecdsa => "ECDSA",
        }
    }
}
