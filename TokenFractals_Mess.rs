use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise,
};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct SelfFractalizedSCTS777 {
    // Standard fields
    name: String,
    symbol: String,
    total_supply: Balance,
    balances: LookupMap<AccountId, Balance>,
    allowances: LookupMap<AccountId, LookupMap<AccountId, Balance>>,

    // New fields for self-fractalization
    child_tokens: LookupMap<String, u64>,
    parent_token: Option<String>,
    parent_balance: Option<Balance>,
}

impl SelfFractalizedSCTS777 {
    // Constructor
    #[init]
    pub fn new(name: String, symbol: String, total_supply: Balance) -> Self {
        let mut instance = Self {
            name,
            symbol,
            total_supply,
            balances: LookupMap::new(b"b".to_vec()),
            allowances: LookupMap::new(b"a".to_vec()),
            child_tokens: LookupMap::new(b"c".to_vec()),
            parent_token: None,
            parent_balance: None,
        };
        instance.balances.insert(&env::predecessor_account_id(), &total_supply);
        instance
    }

    // Standard functions
    // ...

    // Functions for self-fractalization
    pub fn create_child_token(&mut self, name: String, symbol: String, total_supply: Balance) -> Promise {
        let child_token_id = self.child_tokens.len() as u64;
        let child_token_contract_name = format!("{}_child_{}", env::current_account_id(), child_token_id);

        let child_token_contract = SelfFractalizedSCTS777::new(name, symbol, total_supply);
        let child_token_contract_bytes = borsh::BorshSerialize::try_to_vec(&child_token_contract).unwrap();
        let child_token_deposit = env::account_balance() / 10;

        let promise = Promise::new(child_token_contract_name.clone())
            .create_account()
            .deploy_contract(child_token_contract_bytes)
            .transfer(child_token_deposit)
            .function_call(
                b"new".to_vec(),
                borsh::BorshSerialize::try_to_vec(&(name, symbol, total_supply)).unwrap(),
                0,
                env::prepaid_gas() - 25_000,
            )
            .then(ext_self_fractalized_scts777::register_child_token(
                child_token_contract_name,
                child_token_id,
            ));

        self.child_tokens.insert(&child_token_contract_name, &child_token_id);
        promise
    }

    pub fn set_parent_token(&mut self, parent_token: String) -> Promise {
        assert!(
            self.parent_token.is_none(),
            "Parent token already set"
        );
        assert!(
            !self.child_tokens.contains_key(&parent_token),
            "Cannot set parent token to a child token"
        );

        let parent_balance = self.balances.get(&parent_token).unwrap_or(&0);
        self.balances.remove(&parent_token);

        self.parent_token = Some(parent_token);
        self.parent_balance = Some(parent_balance);

        Promise::new(env::current_account_id()).as_return()
    }

    pub fn remove_parent_token(&mut self) -> Promise {
        assert!(
            self.parent_token.is_some(),
            "Parent token not set"
        );
    
        let parent_token = self.parent_token.take().unwrap();
        let parent_balance = self.balances.get(&parent_token.owner_id).unwrap();
        let mut new_parent_balance = parent_balance - self.parent_token_quantity;
        assert!(
            new_parent_balance >= 0,
            "Removing parent token would result in negative balance"
        );
    
        if new_parent_balance == 0 {
            self.balances.remove(&parent_token.owner_id);
        } else {
            self.balances.insert(&parent_token.owner_id, &new_parent_balance);
        }
    
        self.total_supply -= self.parent_token_quantity;
    
        Promise::new(env::current_account_id()).as_return()
    }
    
    pub fn get_child_tokens(&self, parent_token_id: u64) -> Option<Vec<u64>> {
        let mut child_tokens = vec![];
    
        for (token_id, token) in self.tokens.iter() {
            if let Some(pt) = token.parent_token {
                if pt == parent_token_id {
                    child_tokens.push(token_id);
                }
            }
        }
    
        if child_tokens.is_empty() {
            None
        } else {
            Some(child_tokens)
        }
    }
    
    pub fn add_child_token(&mut self, parent_token_id: u64, child_token_id: u64) -> Promise {
        assert!(
            self.token_owners.contains_key(&parent_token_id),
            "Parent token does not exist"
        );
    
        let owner = self.token_owners.get(&parent_token_id).unwrap();
        assert_eq!(
            owner,
            &env::predecessor_account_id(),
            "Only the token owner can add child tokens"
        );
    
        let child_owners = self
            .token_child_owners
            .get(&parent_token_id)
            .unwrap_or(&LookupMap::new(format!("Child owners of token {}", parent_token_id).as_bytes().to_vec()));
    
        assert!(
            !child_owners.contains_key(&child_token_id),
            "Child token already exists"
        );
    
        child_owners.insert(&child_token_id, &env::predecessor_account_id());
        self.token_child_owners
            .insert(&parent_token_id, &child_owners);
    
        Promise::new(env::current_account_id()).as_return()
    }
    use near_sdk::{env, Promise, StorageMap, BorshStorageKey};
use crate::token::{Token, TokenId};
use crate::parent_token::ParentToken;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum ChildTokenKey {
    #[key("child_token")]
    ChildToken(TokenId),
    #[key("parent_token")]
    ParentToken(TokenId),
}

impl ParentToken {
    pub fn transfer_child_tokens(&mut self, recipient: TokenId, token_ids: Vec<TokenId>) -> Promise {
        let sender_id = env::predecessor_account_id();
        let mut sender_token = self.get_child_token(&sender_id)
            .expect("Sender does not own any tokens");
        let mut recipient_token = self.get_child_token(&recipient)
            .expect("Recipient does not own any tokens");

        for token_id in token_ids.iter() {
            let balance = sender_token.get_balance(token_id)
                .expect("Token not found in sender's balance");
            sender_token.remove_balance(token_id, balance);
            recipient_token.add_balance(token_id, balance);

            let key = ChildTokenKey::ChildToken(token_id.clone());
            self.child_tokens.remove(&key);
            let key = ChildTokenKey::ParentToken(token_id.clone());
            self.child_tokens.insert(&key, &recipient);
        }

        Promise::new(env::current_account_id()).as_return()
    }
}

    use near_sdk::{
        borsh::{self, BorshDeserialize, BorshSerialize},
        env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise,
    };
    
    #[near_bindgen]
    #[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
    pub struct SCTS777 {
        // Standard fields
        name: String,
        symbol: String,
        total_supply: Balance,
        balances: LookupMap<AccountId, Balance>,
        allowances: LookupMap<AccountId, LookupMap<AccountId, Balance>>,
    
        // New fields for non-fungible tokens and granchild tokens
        next_token_id: u64,
        token_owners: LookupMap<u64, AccountId>,
        token_balances: LookupMap<AccountId, LookupMap<u64, Balance>>,
        token_approvals: LookupMap<u64, AccountId>,
        granchild_tokens: LookupMap<u64, LookupMap<u64, u64>>,
    }
    
    impl SCTS777 {
        // Constructor
        #[init]
        pub fn new(name: String, symbol: String, total_supply: Balance) -> Self {
            let mut instance = Self {
                name,
                symbol,
                total_supply,
                balances: LookupMap::new(b"b".to_vec()),
                allowances: LookupMap::new(b"a".to_vec()),
                next_token_id: 0,
                token_owners: LookupMap::new(b"o".to_vec()),
                token_balances: LookupMap::new(b"t".to_vec()),
                token_approvals: LookupMap::new(b"p".to_vec()),
                grandchild_tokens: LookupMap::new(b"g".to_vec()),
            };
            instance.balances.insert(&env::predecessor_account_id(), &total_supply);
            instance
        }
    
        // Standard functions
        // ...

        // Functions for self-fractalization
        // ...

        // Functions for funible tokens
        // ...
    
        // Functions for non-fungible tokens
        // ...
    
        // Functions for grandchild tokens
        pub fn generate_grandchild_token(&mut self, parent_token_id: u64, to: AccountId) -> Promise {
            assert!(
                self.token_owners.contains_key(&parent_token_id),
                "Parent token does not exist"
            );
    
            let grandchild_token_id = self.next_token_id;
            self.next_token_id += 1;
    
            self.token_owners.insert(&grandchild_token_id, &to);
    
            let grandchild_token_balance = self.token_balances
                .get(&to)
                .unwrap_or(&LookupMap::new(format!("Token balances of {}", to).as_bytes().to_vec()));
            grandchild_token_balance.insert(&grandchild_token_id, &1);
            self.token_balances.insert(&to, &grandchild_token_balance);
    
            let grandchild_tokens = self.grandchild_tokens
                .get(&parent_token_id)
                .unwrap_or(&LookupMap::new(format!("Grandchild tokens of parent token {}", parent_token_id).as_bytes().to_vec()));
            grandchild_tokens.insert(&grandchild_token_id, &1);
            self.grandchild_tokens.insert(&parent_token_id, &grandchild_tokens);
    
            Promise::new(env::current_account_id()).as_return()
        }
    }
    
    pub fn transfer_grandchild_tokens(&mut self, from: AccountId, to: AccountId, grandchild_token_ids: Vec<u64>) -> Promise {
        let mut sender_grandchild_token_balances = self.token_balances.get(&from).unwrap();
        let mut recipient_grandchild_token_balances = self.token_balances.get(&to).unwrap_or(&LookupMap::new(to.as_bytes().to_vec()));
    
        for grandchild_token_id in grandchild_token_ids.iter() {
            let balance = sender_grandchild_token_balances.get(grandchild_token_id).unwrap_or(&0);
            sender_grandchild_token_balances.remove(grandchild_token_id);
    
            recipient_grandchild_token_balances.insert(grandchild_token_id, balance);
            self.token_owners.insert(grandchild_token_id, &to);
        }
    
        self.token_balances.insert(&from, &sender_grandchild_token_balances);
        self.token_balances.insert(&to, &recipient_grandchild_token_balances);
    
        Promise::new(env::current_account_id()).as_return()
    }

    pub fn burn_grandchild_token(&mut self, parent_token_id: u64, grandchild_token_id: u64) -> Promise {
        let owner_id = env::predecessor_account_id();
        assert!(
            self.granchild_tokens.contains_key(&parent_token_id),
            "Parent token does not exist"
        );
        let grandchild_tokens = self.granchild_tokens.get(&parent_token_id).unwrap();
        assert!(
            grandchild_tokens.contains_key(&grandchild_token_id),
            "Grandchild token does not exist"
        );
        let grandchild_owner_id = self.token_owners.get(&grandchild_token_id).unwrap();
        assert_eq!(
            grandchild_owner_id,
            &owner_id,
            "Only the grandchild token owner can burn the token"
        );
    
        // Reduce the balance of the grandchild token owner
        let grandchild_owner_balance = self.token_balances.get(&owner_id).unwrap();
        let mut grandchild_balance = grandchild_owner_balance
            .get(&grandchild_token_id)
            .expect("Grandchild token balance not found");
        grandchild_balance -= 1;
        if grandchild_balance == 0 {
            grandchild_owner_balance.remove(&grandchild_token_id);
        } else {
            grandchild_owner_balance.insert(&grandchild_token_id, &grandchild_balance);
        }
        self.token_balances.insert(&owner_id, &grandchild_owner_balance);
    
        // Remove the grandchild token from the granchild_tokens list
        let mut new_grandchild_tokens = LookupMap::new(format!("Grandchild tokens of parent token {}", parent_token_id).as_bytes().to_vec());
        for (token_id, _) in grandchild_tokens.iter() {
            if token_id != &grandchild_token_id {
                new_grandchild_tokens.insert(&token_id, &1);
            }
        }
        self.granchild_tokens.insert(&parent_token_id, &new_grandchild_tokens);
    
        Promise::new(env::current_account_id()).as_return()
    }
    
pub fn allowance_grandchild_token(&mut self, grandchild_token_id: u64, spender_id: AccountId, allowance: Balance) -> Promise {
    let owner_id = env::predecessor_account_id();
    assert!(
        self.token_owners.contains_key(&grandchild_token_id),
        "Grandchild token does not exist"
    );
    assert_eq!(
        owner_id,
        self.token_owners.get(&grandchild_token_id).unwrap(),
        "Account does not own grandchild token"
    );

    let grandchild_token_allowances = self.token_allowances
        .get(&owner_id)
        .unwrap_or(&LookupMap::new(format!("Token allowances of {}", owner_id).as_bytes().to_vec()));

    grandchild_token_allowances.insert(&spender_id, &allowance);
    self.token_allowances.insert(&owner_id, &grandchild_token_allowances);

    Promise::new(env::current_account_id()).as_return()
}

#[allow(dead_code)]
impl SelfFractalizedSCTS777 {
// Standard functions
// ...

// Functions for self-fractalization
// ...

// Functions for grandchild tokens
// ...

pub fn allowance_grandchild_token(&mut self, grandchild_token_id: u64, spender_id: AccountId, allowance: Balance) -> Promise {
    let owner_id = env::predecessor_account_id();
    assert!(
        self.token_owners.contains_key(&grandchild_token_id),
        "Grandchild token does not exist"
    );
    assert_eq!(
        owner_id,
        self.token_owners.get(&grandchild_token_id).unwrap(),
        "Account does not own grandchild token"
    );

    let grandchild_token_allowances = self.token_allowances
        .get(&owner_id)
        .unwrap_or(&LookupMap::new(format!("Token allowances of {}", owner_id).as_bytes().to_vec()));

    grandchild_token_allowances.insert(&spender_id, &allowance);
    self.token_allowances.insert(&owner_id, &grandchild_token_allowances);

    Promise::new(env::current_account_id()).as_return()
}

    pub fn approve_grandchild_token(&mut self, token_id: u64, spender: AccountId, amount: Balance) -> bool {
        let owner = self.token_owners.get(&token_id).expect("Token does not exist");
        assert_eq!(
            owner,
            env::predecessor_account_id(),
            "Only token owner can approve a spender"
        );
        let mut approved_tokens = self.token_approvals
            .get(&spender)
            .unwrap_or(&LookupMap::new(format!("Approved tokens for spender {}", spender).as_bytes().to_vec()));
        approved_tokens.insert(&token_id, &amount);
        self.token_approvals.insert(&spender, &approved_tokens);
        true
    }
}

pub fn transfer_from_grandchild_token(&mut self, from: AccountId, to: AccountId, grandchild_token_id: u64, amount: u64) -> bool {
    let spender = env::predecessor_account_id();
    let allowance = self.allowances.get(&(from.clone(), spender)).unwrap_or(0);
    let owner = self.granchild_tokens.get(&grandchild_token_id).expect("Grandchild token does not exist");
    let owner_account = owner.get(&1).expect("Owner account does not exist");

    assert!(allowance >= amount, "Allowance not enough");
    assert!(owner_account == &from, "Sender is not the owner of the token");
    
    let grandchild_token_balance = self.token_balances
        .get(&from)
        .expect("Sender has no tokens");
    let balance = grandchild_token_balance.get(&grandchild_token_id).expect("Token does not exist");
    
    assert!(balance >= amount, "Balance not enough");
    
    grandchild_token_balance.insert(&grandchild_token_id, &(balance - amount));
    self.token_balances.insert(&from, &grandchild_token_balance);
    
    let recipient_token_balance = self.token_balances
        .get(&to)
        .unwrap_or(&LookupMap::new(format!("Token balances of {}", to).as_bytes().to_vec()));
    recipient_token_balance.insert(&grandchild_token_id, &(recipient_token_balance.get(&grandchild_token_id).unwrap_or(0) + amount));
    self.token_balances.insert(&to, &recipient_token_balance);
    
    if allowance > 0 {
        self.allowances.insert(&(from.clone(), spender), &(allowance - amount));
    }
    
    true
}

impl SCTS777 {
    // ...

    // Functions for grandchild tokens
    // ...

    pub fn set_token_metadata(&mut self, token_id: u64, metadata: String) {
        assert!(self.token_owners.contains_key(&token_id), "Token does not exist");
        let mut token_metadata = self.token_metadata.get(&token_id).unwrap_or_default();
        token_metadata.push(metadata);
        self.token_metadata.insert(&token_id, &token_metadata);
    }

    pub fn get_token_metadata(&self, token_id: u64) -> Vec<String> {
        assert!(self.token_owners.contains_key(&token_id), "Token does not exist");
        self.token_metadata.get(&token_id).unwrap_or_default()
    }
}

pub fn get_token_history(&self, token_id: TokenId) -> Vec<Transaction> {
    let mut history = Vec::new();
    let mut last_transaction = Transaction {
        from: env::current_account_id(),
        to: env::current_account_id(),
        amount: 0,
        timestamp: env::block_timestamp(),
    };

    let mut current_owner = self.token_owners.get(&token_id).unwrap_or_default();
    let mut current_balance = self.token_balances
        .get(&current_owner)
        .unwrap_or(&LookupMap::new(format!("Token balances of {}", current_owner).as_bytes().to_vec()))
        .get(&token_id)
        .unwrap_or(&0);

    history.push(last_transaction.clone());

    if let Some(transfers) = self.token_transfers.get(&token_id) {
        for transfer in transfers.iter() {
            let transfer_amount = transfer.amount as u128;
            let transfer_timestamp = transfer.timestamp;

            if transfer.from != current_owner {
                current_balance = self.token_balances
                    .get(&transfer.from)
                    .unwrap_or(&LookupMap::new(format!("Token balances of {}", transfer.from).as_bytes().to_vec()))
                    .get(&token_id)
                    .unwrap_or(&0);

                current_owner = transfer.from.clone();
            }

            let transaction = Transaction {
                from: transfer.from.clone(),
                to: transfer.to.clone(),
                amount: transfer_amount,
                timestamp: transfer_timestamp,
            };

            history.push(transaction.clone());
            last_transaction = transaction.clone();
        }
    }

    if *current_balance > 0 {
        let burn_timestamp = env::block_timestamp();
        let transaction = Transaction {
            from: current_owner.clone(),
            to: env::current_account_id(),
            amount: *current_balance,
            timestamp: burn_timestamp,
        };

        history.push(transaction.clone());
        last_transaction = transaction.clone();
    }

    history
}

    // Pauses all token transfers and functions in the event of a security concern or bug.
    pub fn pause(&mut self) {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "This function can only be called by the contract owner"
        );
        
        self.paused = true;
        env::log("Token transfers and functions have been paused".as_bytes());
    }

    // Resumes all token transfers and functions after a pause.
    pub fn unpause(&mut self) {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "This function can only be called by the contract owner"
        );
        
        self.paused = false;
        env::log("Token transfers and functions have been resumed".as_bytes());
    }

    impl SCTS777 {
        // Other functions
        
        pub fn activate(&mut self, feature: String) -> bool {
            assert_eq!(
                env::predecessor_account_id(),
                env::current_account_id(),
                "Activation must be called by contract owner"
            );
            
            if !self.active_features.contains(&feature) {
                self.active_features.push(feature);
                return true;
            }
            
            false
        }
        
        pub fn deactivate(&mut self, feature: String) -> bool {
            assert_eq!(
                env::predecessor_account_id(),
                env::current_account_id(),
                "Deactivation must be called by contract owner"
            );
            
            if let Some(pos) = self.active_features.iter().position(|f| f == &feature) {
                self.active_features.remove(pos);
                return true;
            }
            
            false
        }
    }