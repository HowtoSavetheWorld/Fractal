use std::collections::{HashMap, HashSet};

type TokenId = usize;

pub struct SelfComposableModule {
    tokens: HashMap<TokenId, TokenData>,
}

#[derive(Default)]
pub struct TokenData {
    // ... define fields here ...
    composable_tokens: HashSet<TokenId>,
    self_references: HashSet<TokenId>,
}

impl SelfComposableModule {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }

    pub fn add_token(&mut self, token_id: TokenId, token_data: TokenData) {
        self.tokens.insert(token_id, token_data);
    }

    pub fn add_self_reference(&mut self, token_id: TokenId, reference_id: TokenId) {
        let token_data = self.tokens.get_mut(&token_id).unwrap();
        token_data.self_references.insert(reference_id);
    }

    pub fn remove_self_reference(&mut self, token_id: TokenId, reference_id: TokenId) {
        let token_data = self.tokens.get_mut(&token_id).unwrap();
        token_data.self_references.remove(&reference_id);
    }

    pub fn get_self_references(&self, token_id: TokenId) -> Option<HashSet<TokenId>> {
        self.tokens.get(&token_id).map(|token_data| token_data.self_references.clone())
    }

    pub fn add_composable_token(&mut self, token_id: TokenId, composable_id: TokenId) {
        let token_data = self.tokens.get_mut(&token_id).unwrap();
        token_data.composable_tokens.insert(composable_id);
    }

    pub fn remove_composable_token(&mut self, token_id: TokenId, composable_id: TokenId) {
        let token_data = self.tokens.get_mut(&token_id).unwrap();
        token_data.composable_tokens.remove(&composable_id);
    }

    pub fn get_composable_tokens(&self, token_id: TokenId) -> Option<HashSet<TokenId>> {
        self.tokens.get(&token_id).map(|token_data| token_data.composable_tokens.clone())
    }

    pub fn compose_tokens(&mut self, token_id1: TokenId, token_id2: TokenId) -> Option<TokenId> {
        let mut new_token_data = TokenData::default();
        let token_data1 = self.tokens.get(&token_id1)?;
        let token_data2 = self.tokens.get(&token_id2)?;

        // ... do some logic to compose the tokens ...

        let new_token_id = self.tokens.len() + 1; // or use a unique ID generator
        self.tokens.insert(new_token_id, new_token_data);

        Some(new_token_id)
    }

    pub fn merge_tokens(&mut self, token_id1: TokenId, token_id2: TokenId) -> TokenId {
        let mut new_token_data = TokenData {
            self_references: HashSet::new(),
            // copy all fields from both tokens
            //...
        };
        // merge self references
        let token1_references = self.get_self_references(token_id1).unwrap();
        let token2_references = self.get_self_references(token_id2).unwrap();
        new_token_data.self_references.extend(token1_references);
        new_token_data.self_references.extend(token2_references);

        // remove original tokens
        self.tokens.remove(&token_id1);
        self.tokens.remove(&token_id2);

        // add merged token
        let new_token_id = /* generate new ID */;
        self.tokens.insert(new_token_id, new_token_data);
        new_token_id
    }

    pub fn split_token(&mut self, token_id: TokenId, field_subset: HashSet<&str>) -> TokenId {
        let original_token_data = self.tokens.get(&token_id).unwrap();

        // create new token with fields not in subset
        let mut new_token_data = TokenData {
            self_references: original_token_data.self_references.clone(),
            // copy all fields not in subset
            //...
        };

        // modify original token data to only contain fields in subset
        //...
        
        // remove original token
        self.tokens.remove(&token_id);

        // add new tokens
        let original_token_id = /* generate original token ID */;
        let new_token_id = /* generate new token ID */;
        self.tokens.insert(original_token_id, original_token_data);
        self.tokens.insert(new_token_id, new_token_data);
        new_token_id
    }

    pub fn clone_token(&mut self, token_id: TokenId) -> Option<TokenId> {
        let original_token_data = match self.tokens.get(&token_id) {
            Some(data) => data,
            None => return None, // Token doesn't exist
        };
    
        // Create a new token with identical data
        let mut new_token_data = TokenData {
            // Copy all fields from the original token data
            field1: original_token_data.field1.clone(),
            field2: original_token_data.field2.clone(),
            // ... add additional fields as needed ...
            self_references: HashSet::new(),
        };
    
        // Copy over all self-references
        for reference_id in &original_token_data.self_references {
            new_token_data.self_references.insert(*reference_id);
        }
    
        // Generate a new token ID and insert it into the tokens map
        let new_token_id = generate_token_id();
        self.tokens.insert(new_token_id, new_token_data);
    
        Some(new_token_id)
    }
    
    // Helper function to generate a unique token ID
    fn generate_token_id() -> TokenId {
        // ... implementation omitted ...
    }
    
    // ... add more functionality as needed ...
    // MISSING --- GenerateTokenID API (for all interconnected modules) ,,, et cetera
    // very basic, very simple, very abstract
}
