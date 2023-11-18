#!/bin/bash
############################################################
# help                                                     #
############################################################
help()
{
   echo "Welcome to the ICP DAO Canister."
   echo
   echo "functions:"
   echo "1.     deploy <vote_duration> <quorum>"
   echo "2.     run_faucet <account> <amount>"
   echo "3.     get_account_balance"
   echo "4.     join_dao <amount>"
   echo "5.     increase_shares <amount>"
   echo "6.     redeem_shares <amount>"
   echo "7.     view_shares"
   echo "8.     transfer_shares <to> <amount>"
   echo "9.     create_proposal <title> <recipient> <amount>"
   echo "10.    vote_proposal <proposalId>"
   echo "11.    execute_proposal <proposalId>"
   echo "12.    view_proposal <proposalId>"
   echo "13.    get_dao_data"
   echo "14.    help"
   echo
}

############################################################
# Main program                                             #
############################################################
deploy(){
    if [ $1 -ge 60 ] || [ $2 -ge 100]; then
        echo "ERROR: vote_duration must be less than 60 and quorum must be less than 100"
        exit 1
    fi
    dfx deploy icp_dao --argument "(record { vote_duration = $1; quorum = $2 })"
}

run_faucet(){
    dfx identity use minter
    account=$(dfx identity get-principal --identity default)
    dfx canister call icrc1_ledger icrc1_transfer "(record { to = record { owner = principal \"$account\" };  amount = $1; })"
}      

join_dao(){
    dfx identity use default
    # get canister principal
    canister=$(dfx canister call icp_dao get_canister_principal)
    # approve the canister to be able to spend amount from user account
    dfx canister call icrc1_ledger icrc2_approve "(record { amount = $1; spender = record{ owner = $canister } })"
    # call the join the canister function
    dfx canister call icp_dao join_dao "(record {amount = $1})"
}

increase_shares(){
    dfx identity use default
    # get canister principal
    canister=$(dfx canister call icp_dao get_canister_principal)
    # approve the canister to be able to spend amount from user account
    dfx canister call icrc1_ledger icrc2_approve "(record { amount = $1; spender = record{ owner = $canister } })"
    # call the join the canister function
    dfx canister call icp_dao increase_shares "(record { amount = $1 })"
}

redeem_shares(){
    dfx identity use default
    # call the redeem shares the canister function
    dfx canister call icp_dao redeem_shares "(record { amount = $1 })"
}

transfer_shares(){
    dfx identity use default
    # call the transfer shares the canister function
    dfx canister call icp_dao transfer_shares "(record { to: principal \"$1\"; amount = $2 })"
}

create_proposal(){
    dfx identity use default
    # call the create proposal the canister function
    dfx canister call icp_dao create_proposal "(record { title: $1; recipient: principal \"$2\"; amount = $3 })"
}

vote_proposal(){
    dfx identity use default
    # call the vote proposal the canister function
    dfx canister call icp_dao vote_proposal "(record { id: $1 })"
}

execute_proposal(){
    dfx identity use default
    # call the execute proposal the canister function
    dfx canister call icp_dao execute_proposal "(record { id: $1 })"
}

view_shares(){
    dfx identity use default
    # call the view share canister function
    account=$(dfx identity get-principal --identity default)
    dfx canister call icp_dao get_shares "(principal \"$account\")"
}

view_proposal(){
    # call the view proposal the canister function
    dfx canister call icp_dao view_proposal "(record { id: $1 })"
}

get_dao_data(){
    # call the get dao data the canister function
    dfx canister call icp_dao get_dao_data "(record { id: $1 })"
}

get_account_balance(){
    # return wallet balance
    account=$(dfx identity get-principal --identity default)
    dfx canister call icrc1_ledger icrc1_balance_of "(record { owner = $account })"
}

"$@"