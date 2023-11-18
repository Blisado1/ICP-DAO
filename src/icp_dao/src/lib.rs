#[macro_use]
extern crate serde;

mod service;
mod types;

use crate::types::*;
use candid::{Nat, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{TransferArg, TransferError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use std::cell::RefCell;

thread_local! {
    static MEMORY_MANAGER: RefCell<Option<MemoryManager<DefaultMemoryImpl>>> = RefCell::new(None);
    static DAO_STORAGE: RefCell<DaoData> = RefCell::default();
    static PROPOSAL_STORAGE: RefCell<StableBTreeMap<u32, Proposal, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(StableBTreeMap::init(MemoryManager::init(DefaultMemoryImpl::default()).get(MemoryId::new(0))));
    static ACCOUNT_STORAGE: RefCell<StableBTreeMap<Principal, DaoAccount, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(StableBTreeMap::init(MemoryManager::init(DefaultMemoryImpl::default()).get(MemoryId::new(1))));
}

#[ic_cdk::init]
fn init(payload: InitPayload) {
    DAO_STORAGE.with(|dao| dao.borrow_mut().init_params(payload));
}

#[ic_cdk::query]
fn get_dao_data() -> Result<DaoData, String> {
    let dao_data = DAO_STORAGE.with(|dao| dao.borrow().clone());
    Ok(dao_data)
}

#[ic_cdk::update]
async fn join_dao(payload: SharesPayload) -> Result<Nat, String> {
    if get_user(&ic_cdk::caller()).is_some() {
        return Err("You already have a dao account".to_string());
    }

    let result = do_transfer_to_canister(&payload)
        .await
        .map_err(|e| format!("Failed to call ledger: {:?}", e))
        .and_then(|value| {
            let user_account = DaoAccount {
                id: ic_cdk::caller(),
                shares: payload.amount.clone(),
            };

            add_user(&user_account);
            DAO_STORAGE.with(|dao| {
                dao.borrow_mut().add_total_shares(&payload.amount);
                dao.borrow_mut().add_available_shares(&payload.amount);
            });

            Ok(value)
        });

    result
}

// ... (similar adjustments for other functions)

// Helper method to transfer tokens from ledger to canister
async fn do_transfer_to_canister(
    _payload: &SharesPayload,
) -> CallResult<Result<Nat, TransferFromError>> {
    // Adjusted implementation...
}

// Helper method to add a new user.
fn add_user(account: &DaoAccount) {
    ACCOUNT_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(account.id, account.clone())
    });
}

// Helper method to perform an insert.
fn add_proposal(proposal: &Proposal) {
    PROPOSAL_STORAGE.with(|service| service.borrow_mut().insert(proposal.id, proposal.clone()));
}

// Helper method to get users
fn get_user(account: &Principal) -> Option<DaoAccount> {
    ACCOUNT_STORAGE.with(|service| service.borrow().get(account))
}

// Helper method to get a proposal
fn get_proposal(id: &u32) -> Option<Proposal> {
    PROPOSAL_STORAGE.with(|service| service.borrow().get(id))
}

// A helper method to transfer the coffee amount to the creator
async fn _transfer(transfer_args: TransferPayload) -> CallResult<Result<Nat, TransferError>> {
    // Adjusted implementation...
}

// Need this to generate candid
ic_cdk::export_candid!();
                
