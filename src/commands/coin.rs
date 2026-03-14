use rand::RngExt as _;

pub fn run(count: u32, boolean: bool, json: bool) {
    let mut rng = rand::rng();
    if json {
        let flips: Vec<&str> = (0..count)
            .map(|_| {
                let result = rng.random_bool(0.5);
                if boolean {
                    if result { "true" } else { "false" }
                } else {
                    if result { "heads" } else { "tails" }
                }
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&flips).unwrap());
    } else {
        for _ in 0..count {
            let result = rng.random_bool(0.5);
            if boolean {
                println!("{result}");
            } else {
                println!("{}", if result { "heads" } else { "tails" });
            }
        }
    }
}
