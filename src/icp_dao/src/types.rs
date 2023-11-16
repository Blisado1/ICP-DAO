use candid::{Decode, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc_ledger_types::icrc1::transfer::NumTokens;
use std::borrow::Cow;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct DaoData {
    pub quorum: u64,
    pub vote_time: u64,
    pub total_shares: NumTokens,
    pub available_funds: NumTokens,
    pub locked_funds: NumTokens,
    pub next_proposal_id: u32,
}

// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for DaoData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub amount: NumTokens,
    pub recipient: Principal,
    pub votes: NumTokens,
    pub ends: u64,
    pub executed: bool,
    pub ended: bool,
    pub voters: Vec<Principal>,
}

// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for Proposal {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct InitPayload {
    pub vote_duration: u64,
    pub quorum: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct SharesPayload {
    pub amount: NumTokens,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct TransferSharesPayload {
    pub to: Principal,
    pub shares: NumTokens,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct QueryPayload {
    pub id: u32,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct DaoAccount {
    pub id: Principal,
    pub shares: NumTokens,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct ProposalPayload {
    pub title: String,
    pub amount: NumTokens,
    pub recipient: Principal,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct TransferPayload {
    pub to: Principal,
    pub amount: NumTokens,
}

impl Storable for DaoAccount {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
