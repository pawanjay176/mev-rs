pub(crate) use ethereum_consensus::bellatrix::mainnet::{
    ExecutionPayload, ExecutionPayloadHeader, SignedBlindedBeaconBlock,
};
pub(crate) use ethereum_consensus::builder::SignedValidatorRegistration;
pub(crate) use ethereum_consensus::primitives::BlsPublicKey;
use ethereum_consensus::primitives::BlsSignature;
pub(crate) use ethereum_consensus::primitives::{ExecutionAddress, Hash32, Slot};
pub(crate) use ssz_rs::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct BidRequest {
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
    pub parent_hash: Hash32,
    pub public_key: BlsPublicKey,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BuilderBid {
    pub header: ExecutionPayloadHeader,
    pub value: U256,
    pub public_key: BlsPublicKey,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SignedBuilderBid {
    pub message: BuilderBid,
    pub signature: BlsSignature,
}
