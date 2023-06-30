use std::collections::HashMap;

struct FractalValidator {
    token_fractals: HashMap<String, Fractal>,
}

struct FractalOperations {
    token_fractals: HashMap<String, Fractal>,
}

struct Fractal {
    fractal_id: String,
    parent_id: Option<String>,
    amount: u32,
    children: Vec<String>,
}

impl FractalValidator {
    fn new() -> FractalValidator {
        FractalValidator {
            token_fractals: HashMap::new(),
        }
    }

    fn validate_fractal(&self, fractal_id: &str) {
        if let Some(fractal) = self.token_fractals.get(fractal_id) {
            if let Some(parent_id) = &fractal.parent_id {
                if let Some(parent_fractal) = self.token_fractals.get(parent_id) {
                    let parent_amount = parent_fractal.amount;
                    let child_amounts: Vec<u32> = fractal
                        .children
                        .iter()
                        .filter_map(|child_id| self.token_fractals.get(child_id))
                        .map(|child_fractal| child_fractal.amount)
                        .collect();

                    if child_amounts.iter().sum::<u32>() != parent_amount {
                        panic!("Child amounts do not sum up to the parent amount.");
                    }
                }
            }
        }
    }

    fn check_circular_reference(&self, fractal_id: &str) {
        let mut visited = std::collections::HashSet::new();

        fn dfs(current_id: &str, visited: &mut std::collections::HashSet<String>, token_fractals: &HashMap<String, Fractal>) {
            visited.insert(current_id.to_string());
            if let Some(fractal) = token_fractals.get(current_id) {
                for child_id in &fractal.children {
                    if visited.contains(child_id) {
                        panic!("Circular reference detected.");
                    }
                    dfs(child_id, visited, token_fractals);
                }
            }
        }

        dfs(fractal_id, &mut visited, &self.token_fractals);
    }

    fn check_conflicting_updates(&self, fractal_id: &str) {
        let mut visited = std::collections::HashSet::new();

        fn dfs(current_id: &str, visited: &mut std::collections::HashSet<String>, token_fractals: &HashMap<String, Fractal>) {
            visited.insert(current_id.to_string());
            if let Some(fractal) = token_fractals.get(current_id) {
                for child_id in &fractal.children {
                    if visited.contains(child_id) {
                        panic!("Conflicting updates detected.");
                    }
                    dfs(child_id, visited, token_fractals);
                }
            }
        }

        dfs(fractal_id, &mut visited, &self.token_fractals);
    }
}

impl FractalOperations {
    fn new() -> FractalOperations {
        FractalOperations {
            token_fractals: HashMap::new(),
        }
    }

    fn merge_fractals(&mut self, fractal_id1: &str, fractal_id2: &str) {
        if let Some(fractal1) = self.token_fractals.get(fractal_id1) {
            if let Some(fractal2) = self.token_fractals.get(fractal_id2) {
                let merged_fractal = Fractal {
                    fractal_id: fractal_id1.to_string(),
                    parent_id: fractal1.parent_id.clone(),
                    amount: fractal1.amount + fractal2.amount,
                    children: fractal1.children.iter().chain(&fractal2.children).cloned().collect(),
                };

                // Update child-parent relationship
                for child_id in &merged_fractal.children {
                    if let Some(child_fractal) = self.token_fractals.get_mut(child_id) {
                        child_fractal.parent_id = Some(fractal_id1.to_string());
                    }
                }

                self.token_fractals.insert(fractal_id1.to_string(), merged_fractal);
                self.token_fractals.remove(fractal_id2);
            }
        }
    }

    fn split_fractal(&mut self, fractal_id: &str, child_fractals: HashMap<String, u32>) {
        if let Some(fractal) = self.token_fractals.get_mut(fractal_id) {
            let total_amount: u32 = child_fractals.values().sum();

            if total_amount > fractal.amount {
                panic!("Total child fractal amount exceeds the parent fractal amount.");
            }

            fractal.amount -= total_amount;

            for (child_id, child_amount) in child_fractals {
                let child_fractal = Fractal {
                    fractal_id: child_id.clone(),
                    parent_id: Some(fractal_id.to_string()),
                    amount: child_amount,
                    children: Vec::new(),
                };
                self.token_fractals.insert(child_id, child_fractal);
                fractal.children.push(child_id);
            }
        }
    }

    fn calculate_aggregated_value(&self, fractal_id: &str, property_name: &str) -> u32 {
        if let Some(fractal) = self.token_fractals.get(fractal_id) {
            let mut aggregated_value = 0;

            if property_name == "amount" {
                aggregated_value += fractal.amount;
            }

            for child_id in &fractal.children {
                aggregated_value += self.calculate_aggregated_value(child_id, property_name);
            }

            aggregated_value
        } else {
            panic!("Invalid fractal ID.");
        }
    }
}
