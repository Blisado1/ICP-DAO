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
   echo "2.     run_faucet <amount>"
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
    if [ -z "$1" ] 
    then
        echo -e 'Please set vote_duration\nRun dao help for more info'
        exit 1
    elif [ -z "$2" ] 
    then
        echo -e "Please set quorum\nRun dao help for more info"
        exit 1
    elif [ $1 -ge 60 ] 
    then
        echo "ERROR: vote_duration must be less than 60"
        exit 1
    elif [ $2 -ge 100 ]
    then
        echo "ERROR: quorum must be less than 100"
        exit 1
    fi

    dfx deploy icp_dao --argument "(record { vote_duration = $1; quorum = $2 })"
}

run_faucet(){
    if [ -z "$1" ] 
    then
        echo -e 'Please set amount\nRun dao help for more info'
        exit 1
    fi
    account=$(dfx identity get-principal)
    current_id=$(dfx identity whoami)
    echo "Your identity is $current_id"
    dfx identity use minter
    dfx canister call icrc1_ledger icrc1_transfer "(record { to = record { owner = principal \"$account\" };  amount = $1 })"
    dfx identity use $current_id
}      

join_dao(){
    if [ -z "$1" ] 
    then
        echo -e 'Please set amount\nRun dao help for more info'
        exit 1
    fi
    # displaying identity information
    id=$(dfx identity whoami)
    echo "Your identity is $id"
    # get canister principal
    canister=$(dfx canister call icp_dao get_canister_principal)
    # approve the canister to be able to spend amount from user account
    dfx canister call icrc1_ledger icrc2_approve "(record { amount = $1; spender = record{ owner = $canister } })"
    # call the join the canister function
    dfx canister call icp_dao join_dao "(record {amount = $1})"
}

increase_shares(){
    if [ -z "$1" ] 
    then
        echo -e 'Please set amount\nRun dao help for more info'
        exit 1
    fi
    # displaying identity information
    id=$(dfx identity whoami)
    echo "Your identity is $id"
    # get canister principal
    canister=$(dfx canister call icp_dao get_canister_principal)
    # approve the canister to be able to spend amount from user account
    dfx canister call icrc1_ledger icrc2_approve "(record { amount = $1; spender = record{ owner = $canister } })"
    # call the join the canister function
    dfx canister call icp_dao increase_shares "(record { amount = $1 })"
}

redeem_shares(){
    if [ -z "$1" ] 
    then
        echo -e 'Please set amount\nRun dao help for more info'
        exit 1
    fi
    # displaying identity information
    id=$(dfx identity whoami)
    echo "Your identity is $id"
    # call the redeem shares the canister function
    dfx canister call icp_dao redeem_shares "(record { amount = $1 })"
}

transfer_shares(){
    if [ -z "$1" ] 
    then
        echo -e "Please set receiver identity\nRun dao help for more info"
        exit 1
    elif [ -z "$2" ] 
    then
        echo -e 'Please set amount\nRun dao help for more info'
        exit 1
    fi
    # displaying identity information
    id=$(dfx identity whoami)
    echo "Your identity is $id"
    # get receiver principal address
    receiver=$(dfx identity get-principal --identity $1)
    # call the transfer shares the canister function
    dfx canister call icp_dao transfer_shares "(record { to = principal \"$receiver\"; shares = $2 })"
}

create_proposal(){
    if [ -z "$1" ] 
    then
        echo -e "Please set title of proposal\nRun dao help for more info"
        exit 1
    elif [ -z "$2" ] 
    then
        echo -e 'Please set recipient identity\nRun dao help for more info'
        exit 1
    elif [ -z "$3" ] 
    then
        echo -e 'Please set amount\nRun dao help for more info'
        exit 1
    fi
    # displaying identity information
    id=$(dfx identity whoami)
    echo "Your identity is $id"
    # get receiver principal address
    receiver=$(dfx identity get-principal --identity $2)
    # call the create proposal the canister function
    dfx canister call icp_dao create_proposal "(record { title = \"$1\"; recipient = principal \"$receiver\"; amount = $3 })"
}

vote_proposal(){
    if [ -z "$1" ] 
    then
        echo -e "Please set id of proposal\nRun dao help for more info"
        exit 1
    fi
    # displaying identity information
    id=$(dfx identity whoami)
    echo "Your identity is $id"
    # call the vote proposal the canister function
    dfx canister call icp_dao vote_proposal "(record { id = $1 })"
}

execute_proposal(){
    if [ -z "$1" ] 
    then
        echo -e "Please set id of proposal\nRun dao help for more info"
        exit 1
    fi
    # displaying identity information
    id=$(dfx identity whoami)
    echo "Your identity is $id"
    # call the execute proposal the canister function
    dfx canister call icp_dao execute_proposal "(record { id = $1 })"
}

view_shares(){
    # displaying identity information
    id=$(dfx identity whoami)
    echo "Your identity is $id"
    # call the view share canister function
    account=$(dfx identity get-principal)
    dfx canister call icp_dao get_shares "(principal \"$account\")"
}

view_proposal(){
    if [ -z "$1" ] 
    then
        echo -e "Please set id of proposal\nRun dao help for more info"
        exit 1
    fi
    # displaying identity information
    id=$(dfx identity whoami)
    echo "Your identity is $id"
    # call the view proposal the canister function
    dfx canister call icp_dao view_proposal "(record { id = $1 })"
}

get_dao_data(){
    # call the get dao data the canister function
    dfx canister call icp_dao get_dao_data "()"
}

get_account_balance(){
    # displaying identity information
    id=$(dfx identity whoami)
    echo "Your identity is $id"
    # return wallet balance
    account=$(dfx identity get-principal)
    dfx canister call icrc1_ledger icrc1_balance_of "(record { owner = principal \"$account\" })"
}

"$@"