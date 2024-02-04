use rand::prelude::*;
use serde_json;

pub fn generate_grid() {
        let a: Vec<f64> = vec![0.0; 100].iter().map(|_| thread_rng().gen_range(0.0..1.0)).collect();
        println!("{:?}", a);

        // Convert the array to JSON
        let json = serde_json::to_string(&a).unwrap_or_else(|err| panic!("Failed to convert to JSON: {}", err));
        print!("{}", json);

        // Save the JSON to a file
        std::fs::write("data.json", &json).unwrap_or_else(|err| panic!("Failed to write JSON to file: {}", err));
        println!("done!");
}
