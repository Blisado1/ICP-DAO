#[macro_use]
extern crate serde;

mod dao_service;
mod dao_types;

use crate::dao_types::*;
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
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static DAO_STORAGE: RefCell<DaoData> = RefCell::default();

    static PROPOSAL_STORAGE: RefCell<StableBTreeMap<u32, Proposal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
    ));

    static ACCOUNT_STORAGE: RefCell<StableBTreeMap<String, DaoAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
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
    let result = do_transfer_to_canister(&payload)
        .await
        .map_err(|e| format!("Failed to call ledger: {:?}", e))?
        .map_err(|e| format!("Ledger transfer error: {:?}", e))?;

    match result {
        Ok(value) => {
            // Create user account
            let user_account = DaoAccount {
                id: ic_cdk::caller(),
                shares: payload.amount.clone(),
            };

            // Add user
            add_user(&user_account);

            // Update dao data
            DAO_STORAGE.with(|dao| dao.borrow_mut().add_total_shares(&payload.amount));
            DAO_STORAGE.with(|dao| dao.borrow_mut().add_available_shares(&payload.amount));

            // Return ok value
            Ok(value)
        }
        Err(error) => Err(error),
    }
}

#[ic_cdk::query]
fn get_shares(user: Principal) -> Result<DaoAccount, String> {
    let account = get_user(&user);
    match account {
        Some(account) => Ok(account),
        None => Err("You do not have a DAO account".to_string()),
    }
}

    // check if available funds is greater than the amount
    let dao_data = DAO_STORAGE.with(|dao| dao.borrow().clone());
    if dao_data.available_funds < payload.amount {
        return Err(format!("not enough available funds"));
    }

    // check if user has enough shares
    if account.shares < payload.amount {
        return Err(format!("you do not have enough funds"));
    }

    // create transfer args
    let transfer_args = TransferPayload {
        to: account.id,
        amount: payload.amount.clone(),
    };

    // transfer the funds from canister back to user
    let result = _transfer(transfer_args)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e))?
        .map_err(|e| format!("ledger transfer error {:?}", e));

    match result {
        Ok(value) => {
            // update user records
            account.sub_shares(&payload.amount);
            add_user(&account);

            // update dao data
            DAO_STORAGE.with(|dao| dao.borrow_mut().sub_available_shares(&payload.amount));
            DAO_STORAGE.with(|dao| dao.borrow_mut().sub_total_shares(&payload.amount));

            // return ok value
            Ok(value)
        }
        Err(error) => Err(error),
    }
}

#[ic_cdk::update]
fn transfer_shares(payload: TransferSharesPayload) -> Result<String, String> {
    // get transferer acount
    let mut from_account = get_user(&ic_cdk::caller()).expect("you do not have a dao account");

    // check if user has enough shares
    if from_account.shares < payload.shares {
        return Err(format!("you do not have enough funds"));
    }

    // get recipient account
    match get_user(&payload.to) {
        Some(mut to_account) => {
            // subtract shares from from account and update in records
            from_account.sub_shares(&payload.shares);
            add_user(&from_account);

            // add shares to to_account and update in records
            to_account.add_shares(&payload.shares);
            add_user(&to_account);
        }
        None => {
            // create new user account and update in records
            let new_acount = DaoAccount {
                id: payload.to,
                shares: payload.shares,
            };
            add_user(&new_acount);
        }
    }
    Ok(format!("shares successfully transfered to {}", payload.to))
}

#[ic_cdk::update]
fn create_proposal(payload: ProposalPayload) -> Result<Proposal, String> {
    // check if user has an account
    get_user(&ic_cdk::caller()).expect("you do not have a dao account");

    // check if available funds is greater than the amount
    let dao_data = DAO_STORAGE.with(|dao| dao.borrow().clone());
    if dao_data.available_funds < payload.amount {
        return Err(format!("not enough available funds"));
    }

    // create new proposal
    let new_proposal = Proposal {
        id: dao_data.next_proposal_id,
        title: payload.title,
        amount: payload.amount.clone(),
        recipient: payload.recipient,
        votes: 0u128.into(),
        ends: time() + dao_data.vote_time,
        executed: false,
        ended: false,
        voters: Vec::new(),
    };

    // add to records
    add_proposal(&new_proposal);

    // update dao storage
    DAO_STORAGE.with(|dao| {
        dao.borrow_mut()
            .increment_proposal_n_lock_funds(&payload.amount)
    });

    Ok(new_proposal)
}

#[ic_cdk::update]
fn vote_proposal(payload: QueryPayload) -> Result<String, String> {
    // check if user has an account
    let account = get_user(&ic_cdk::caller()).expect("you do not have a dao account");

    match get_proposal(&payload.id) {
        Some(mut proposal) => {
            // check that vote time has not elapsed
            if time() > proposal.ends {
                return Err(format!("proposal has ended"));
            }
            // check that user has not voted before
            if proposal.voters.contains(&ic_cdk::caller()) {
                return Err(format!("you have already voted"));
            }

            // add voters shares to proposal count
            proposal.votes += account.shares;

            // add proposal to proposal count
            proposal.voters.push(ic_cdk::caller());

            // update proposal in records
            add_proposal(&proposal);

            Ok(format!(
                "you have successfully voted for proposal with id {}",
                proposal.id
            ))
        }
        None => Err(format!("proposal with id {} not found", payload.id)),
    }
}

#[ic_cdk::update]
async fn execute_proposal(payload: QueryPayload) -> Result<String, String> {
    // check if user has an account
    get_user(&ic_cdk::caller()).expect("you do not have a dao account");

    // check if available funds is greater than the amount
    let dao_data = DAO_STORAGE.with(|dao| dao.borrow().clone());

    let mut proposal = get_proposal(&payload.id).expect("invalid proposal id");

    //     Some(mut proposal) => {
    // check that vote time has not elapsed
    if time() < proposal.ends {
        return Err(format!("cannot execute proposal before end date"));
    }
    // check that proposal had already ended
    if proposal.ended {
        return Err(format!("cannot execute proposal already ended"));
    }

    let hundred_percent: Nat = 100u128.into();

    if proposal.votes * hundred_percent / dao_data.total_shares >= dao_data.quorum {
        // create transfer arguments
        let transfer_args = TransferPayload {
            to: proposal.recipient,
            amount: proposal.amount.clone(),
        };
        // transfer the funds from canister back to user
        let result = _transfer(transfer_args)
            .await
            .map_err(|e| format!("failed to call ledger: {:?}", e))?
            .map_err(|e| format!("ledger transfer error {:?}", e));

        match result {
            Ok(_) => {
                // update dao data
                DAO_STORAGE.with(|dao| dao.borrow_mut().sub_available_shares(&proposal.amount));
                DAO_STORAGE.with(|dao| dao.borrow_mut().sub_total_shares(&proposal.amount));
                DAO_STORAGE.with(|dao| dao.borrow_mut().sub_locked_shares(&proposal.amount));

                proposal.executed = true;
                proposal.ended = true;

                Ok(format!(
                    "proposal with id {} successfully executed",
                    payload.id
                ))
            }
            Err(error) => Err(error),
        }
    } else {
        proposal.ended = true;
        DAO_STORAGE.with(|dao| dao.borrow_mut().sub_locked_shares(&proposal.amount));
        DAO_STORAGE.with(|dao| dao.borrow_mut().add_available_shares(&proposal.amount));
        Ok(format!("Proposal with id {} was not succesful", payload.id))
    }
}

#[ic_cdk::query]
fn get_canister_principal() -> Principal {
    ic_cdk::id()
}

#[ic_cdk::query]
fn get_caller_principal() -> Principal {
    ic_cdk::caller()
}

// Helper method to transfer tokens from ledger to canister
async fn do_transfer_to_canister(
    payload: &SharesPayload,
) -> CallResult<Result<Nat, TransferFromError>> {
    let ledger_id = Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap();

    let args = TransferFromArgs {
        spender_subaccount: None,
        from: Account {
            owner: ic_cdk::caller(),
            subaccount: None,
        },
        to: Account {
            owner: ic_cdk::id(),
            subaccount: None,
        },
        fee: None,
        created_at_time: None,
        memo: None,
        amount: payload.amount.clone(),
    };

    ic_cdk::call(ledger_id, "icrc2_transfer_from", (args,)).await.map_err(|e| {
        format!(
            "Failed to call ledger 'icrc2_transfer_from': {:?}",
            e
        )
    })
}

// Helper method to add a new user.
fn add_user(account: &DaoAccount) {
    ACCOUNT_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(account.id.to_string(), account.clone())
    });
}

// Helper method to perform an insert.
fn add_proposal(proposal: &Proposal) {
    PROPOSAL_STORAGE.with(|service| service.borrow_mut().insert(proposal.id, proposal.clone()));
}

// Helper method to get users
fn get_user(account: &Principal) -> Option<DaoAccount> {
    ACCOUNT_STORAGE.with(|service| service.borrow().get(&account.to_string()).cloned())
}

// Helper method to get a proposal 

fn get_proposal(id: &u32) -> Option<Proposal> {
    PROPOSAL_STORAGE.with(|service| service.borrow().get(id).cloned())
}

// Helper method to transfer tokens
async fn _transfer(transfer_args: TransferPayload) -> CallResult<Result<Nat, TransferError>> {
    let ledger_id = Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap();

    let args = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: transfer_args.to,
            subaccount: None,
        },
        fee: None,
        created_at_time: None,
        memo: None,
        amount: transfer_args.amount,
    };

    ic_cdk::call(ledger_id, "icrc1_transfer", (args,)).await.map_err(|e| {
        format!(
            "Failed to call ledger 'icrc1_transfer': {:?}",
            e
        )
    })
}

// Need this to generate Candid
ic_cdk::export_candid!();



