use icrc_ledger_types::icrc1::transfer::NumTokens;

use crate::types::*;

impl DaoData {
    pub fn init_params(&mut self, payload: InitPayload) {
        self.vote_time = payload.vote_duration.clone();
        self.quorum = payload.quorum.clone();
    }

    pub fn add_total_shares(&mut self, amount: &NumTokens) {
        self.total_shares += amount.clone();
    }

    pub fn sub_total_shares(&mut self, amount: &NumTokens) {
        self.total_shares -= amount.clone();
    }

    pub fn add_available_shares(&mut self, amount: &NumTokens) {
        self.available_funds += amount.clone();
    }

    pub fn sub_available_shares(&mut self, amount: &NumTokens) {
        self.available_funds -= amount.clone();
    }

    pub fn sub_locked_shares(&mut self, amount: &NumTokens) {
        self.locked_funds -= amount.clone();
    }

    pub fn increment_proposal_n_lock_funds(&mut self, amount: &NumTokens) {
        self.available_funds += amount.clone();
        self.locked_funds += amount.clone();
        self.next_proposal_id += 1;
    }
}

impl DaoAccount {
    pub fn add_shares(&mut self, amount: &NumTokens) {
        self.shares += amount.clone();
    }

    pub fn sub_shares(&mut self, amount: &NumTokens) {
        self.shares -= amount.clone();
    }
}
