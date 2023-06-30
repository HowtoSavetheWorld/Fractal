use std::collections::HashMap;

pub struct TokenId(u64);

pub mod token {
    use super::TokenId;

    pub trait TokenManager {
        fn create_token(&self) -> TokenId;
        fn validate_token(&self, token: TokenId) -> bool;
        // Add additional token-related functions as needed
    }

    pub struct TokenManagerImpl;

    impl TokenManager for TokenManagerImpl {
        fn create_token(&self) -> TokenId {
            // Token creation logic
            TokenId(0) // Placeholder implementation, replace with actual logic
        }

        fn validate_token(&self, token: TokenId) -> bool {
            // Token validation logic
            true // Placeholder implementation, replace with actual logic
        }
    }
}

pub enum TokenHierarchyOption {
    NoHierarchy,
    VerticalHierarchy,
    HorizontalHierarchy,
    Heterarchy,
    Holarchy,
}

pub struct TokenHierarchy {
    pub(crate) parent: Option<TokenId>,
    pub(crate) children: Vec<TokenId>,
}

pub struct TokenHierarchyManager {
    option: TokenHierarchyOption,
    token_hierarchy_data: HashMap<TokenId, TokenHierarchy>,
}

impl TokenHierarchyManager {
    pub fn new(option: TokenHierarchyOption) -> Self {
        TokenHierarchyManager {
            option,
            token_hierarchy_data: HashMap::new(),
        }
    }

    pub fn create_child_token(&mut self, parent_token: TokenId) -> TokenId {
        if let TokenHierarchyOption::NoHierarchy = self.option {
            return token::TokenManagerImpl.create_token();
        }

        let child_token = token::TokenManagerImpl.create_token(); // Create a new child token
        self.set_parent_token(child_token, parent_token); // Set the parent token for the child token
        child_token
    }

    pub fn set_parent_token(&mut self, child_token: TokenId, parent_token: TokenId) {
        if let TokenHierarchyOption::NoHierarchy = self.option {
            return;
        }

        let token_hierarchy_data = &mut self.token_hierarchy_data;
        let child_hierarchy = token_hierarchy_data
            .entry(child_token)
            .or_insert_with(|| TokenHierarchy {
                parent: None,
                children: Vec::new(),
            });
        if let Some(old_parent) = child_hierarchy.parent {
            let old_parent_hierarchy = token_hierarchy_data
                .get_mut(&old_parent)
                .expect("Invalid parent token");
            old_parent_hierarchy.children.retain(|&x| x != child_token);
        }
        child_hierarchy.parent = Some(parent_token);
        let parent_hierarchy = token_hierarchy_data
            .entry(parent_token)
            .or_insert_with(|| TokenHierarchy {
                parent: None,
                children: Vec::new(),
            });
        parent_hierarchy.children.push(child_token);
    }

    pub fn remove_parent_token(&mut self, token: TokenId) {
        if let TokenHierarchyOption::NoHierarchy = self.option {
            return;
        }

        let token_hierarchy_data = &mut self.token_hierarchy_data;
        let child_hierarchy = token_hierarchy_data
            .get_mut(&token)
            .expect("Invalid token");
        if let Some(parent_token) = child_hierarchy.parent {
            let parent_hierarchy = token_hierarchy_data
                .get_mut(&parent_token)
                .expect("Invalid parent token");
            parent_hierarchy.children.retain(|&x| x != token);
        }
        child_hierarchy.parent = None;
    }

    pub fn get_child_tokens(&self, parent_token: TokenId) -> Vec<TokenId> {
        if let TokenHierarchyOption::NoHierarchy = self.option {
            return Vec::new();
        }

        let token_hierarchy_data = &self.token_hierarchy_data;
        if let Some(parent_hierarchy) = token_hierarchy_data.get(&parent_token) {
            parent_hierarchy.children.clone()
        } else {
            Vec::new()
        }
    }
}

fn main() {
    let token_manager = token::TokenManagerImpl;
    let mut token_hierarchy_manager = TokenHierarchyManager::new(TokenHierarchyOption::Holarchy);

    let grandparent_token = token_manager.create_token();
    let parent_token = token_hierarchy_manager.create_child_token(grandparent_token);
    let child_token = token_hierarchy_manager.create_child_token(parent_token);

    token_hierarchy_manager.set_parent_token(child_token, parent_token);

    let child_tokens = token_hierarchy_manager.get_child_tokens(parent_token);

    token_hierarchy_manager.remove_parent_token(child_token);
}