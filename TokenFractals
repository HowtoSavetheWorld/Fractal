// TokenFractals.rs
// This module allows for the fractalization of tokens and the creation of token fractals.

use std::collections::HashMap;

// Token Fractal Structure
pub struct TokenFractal<T> {
    pub id: u64, // Unique identifier for the token fractal
    pub parent_token: T, // Original token being fractalized
    pub amount: u64, // Amount of the parent token in this fractal
    pub children: HashMap<T, u64>, // Child tokens and their amounts in this fractal
}

// TokenFractals Module
pub struct TokenFractals<T> {
    pub fractals: Vec<TokenFractal<T>>, // List of token fractals
    pub next_id: u64, // Next available identifier for a token fractal
}

impl<T> TokenFractals<T> {
    pub fn new() -> Self {
        TokenFractals {
            fractals: Vec::new(),
            next_id: 0,
        }
    }

    pub fn fractalize(&mut self, parent_token: T, amount: u64, children: HashMap<T, u64>) -> u64 {
        let fractal = TokenFractal {
            id: self.next_id,
            parent_token,
            amount,
            children,
        };

        self.next_id += 1;
        self.fractals.push(fractal);

        fractal.id
    }

    pub fn get_fractal(&self, fractal_id: u64) -> Option<&TokenFractal<T>> {
        self.fractals.iter().find(|f| f.id == fractal_id)
    }

    pub fn update_fractal(
        &mut self,
        fractal_id: u64,
        new_parent_token: T,
        new_amount: u64,
        new_children: HashMap<T, u64>,
    ) -> bool {
        if let Some(fractal) = self.fractals.iter_mut().find(|f| f.id == fractal_id) {
            fractal.parent_token = new_parent_token;
            fractal.amount = new_amount;
            fractal.children = new_children;
            true
        } else {
            false
        }
    }

    pub fn remove_fractal(&mut self, fractal_id: u64) -> bool {
        if let Some(index) = self.fractals.iter().position(|f| f.id == fractal_id) {
            self.fractals.remove(index);
            true
        } else {
            false
        }
    }
}
