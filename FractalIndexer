use std::collections::HashMap;

struct FractalIndexer {
    indexes: HashMap<String, HashMap<String, Vec<TokenFractal>>>,
}

impl FractalIndexer {
    fn new() -> FractalIndexer {
        FractalIndexer {
            indexes: HashMap::new(),
        }
    }

    fn add_index(&mut self, index_name: String) {
        self.indexes.insert(index_name, HashMap::new());
    }

    fn remove_index(&mut self, index_name: &str) {
        self.indexes.remove(index_name);
    }

    fn index_fractal(&mut self, index_name: &str, token_fractal: TokenFractal) {
        if let Some(index) = self.indexes.get_mut(index_name) {
            let properties = self.extract_properties(&token_fractal);
            for (prop, value) in properties {
                let entry = index.entry(prop).or_insert_with(Vec::new);
                entry.push(token_fractal.clone());
            }
        }
    }

    fn search_index(&self, index_name: &str, property_name: &str, value: &str) -> Vec<TokenFractal> {
        if let Some(index) = self.indexes.get(index_name) {
            if let Some(fractals) = index.get(value) {
                return fractals.clone();
            }
        }
        Vec::new()
    }

    fn extract_properties(&self, token_fractal: &TokenFractal) -> HashMap<String, String> {
        // Extract relevant properties from the token fractal
        let mut properties = HashMap::new();
        // Add code here to extract properties from the token fractal
        properties
    }
}

#[derive(Clone)]
struct TokenFractal {
    // Define the structure of the TokenFractal
}

// Explanation:
The FractalIndexer struct represents the Fractal Indexer with its corresponding methods.
The indexes field is a HashMap that stores the indexes by their names, where each index maps property values to a vector of TokenFractal objects.
The new function is an associated function that creates a new instance of FractalIndexer.
The add_index method adds a new index to the indexes hashmap.
The remove_index method removes an index from the indexes hashmap.
The index_fractal method indexes a TokenFractal in a specific index. It extracts the relevant properties from the fractal and adds the fractal to the appropriate property value entry in the index.
The search_index method searches for TokenFractals in a specific index based on a property value. It returns the vector of matching TokenFractals or an empty vector if no matches are found.
The extract_properties method extracts the relevant properties from a TokenFractal object and returns them as a HashMap.
The TokenFractal struct represents the structure of the TokenFractal object. You'll need to define its fields and methods based on your requirements.
To use the FractalIndexer in your Rust code, create an instance of FractalIndexer using FractalIndexer::new() and call its methods as needed, passing the appropriate parameters.
