use std::collections::HashMap;

struct RecursiveFractals {
    token_fractals: HashMap<String, Fractal>,
}

struct Fractal {
    fractal_id: String,
    parent_id: Option<String>,
    children: Vec<String>,
}

impl RecursiveFractals {
    fn new() -> RecursiveFractals {
        RecursiveFractals {
            token_fractals: HashMap::new(),
        }
    }

    fn create_fractal(&mut self, fractal_id: String, parent_id: Option<String>) {
        if self.token_fractals.contains_key(&fractal_id) {
            panic!("Fractal with the same ID already exists.");
        }

        let fractal = Fractal {
            fractal_id: fractal_id.clone(),
            parent_id: parent_id.clone(),
            children: Vec::new(),
        };

        if let Some(pid) = parent_id {
            if let Some(parent_fractal) = self.token_fractals.get_mut(&pid) {
                parent_fractal.children.push(fractal_id.clone());
            }
        }

        self.token_fractals.insert(fractal_id, fractal);
    }

    fn remove_fractal(&mut self, fractal_id: &str) {
        if let Some(fractal) = self.token_fractals.get(fractal_id) {
            if let Some(parent_id) = &fractal.parent_id {
                if let Some(parent_fractal) = self.token_fractals.get_mut(parent_id) {
                    parent_fractal.children.retain(|x| x != fractal_id);
                }
            }
        }

        self.token_fractals.remove(fractal_id);
    }

    fn get_fractal(&self, fractal_id: &str) -> Option<&Fractal> {
        self.token_fractals.get(fractal_id)
    }

    fn get_children(&self, fractal_id: &str) -> Vec<String> {
        if let Some(fractal) = self.token_fractals.get(fractal_id) {
            fractal.children.clone()
        } else {
            Vec::new()
        }
    }

    fn get_parent(&self, fractal_id: &str) -> Option<String> {
        if let Some(fractal) = self.token_fractals.get(fractal_id) {
            fractal.parent_id.clone()
        } else {
            None
        }
    }
}

//Explanation:The RecursiveFractals struct represents the Recursive Fractals module with its associated methods.
The token_fractals field is a HashMap that stores fractal IDs mapped to Fractal objects.
The Fractal struct represents a single fractal with its ID, parent ID, and children IDs.
The new function is an associated function that creates a new instance of RecursiveFractals.
The create_fractal method creates a new fractal with the given ID and optional parent ID. It checks for duplicate IDs, adds the fractal to its parent's children list, and inserts the fractal into the token_fractals hashmap.
The remove_fractal method removes a fractal with the given ID. It updates the parent's children list and removes the fractal from the token_fractals hashmap.
The get_fractal method returns a reference to the fractal object with the given ID if it exists.
The get_children method returns a vector of children IDs for the fractal with the given ID.
The get_parent method returns the parent ID for the fractal with the given ID, if it exists.
To use the RecursiveFractals module in your Rust code, create an instance of RecursiveFractals using RecursiveFractals::new() and call its methods as needed, passing the appropriate parameters.
