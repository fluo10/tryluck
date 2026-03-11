use rand::RngExt as _;

pub fn run(count: u32, boolean: bool) {
    for _ in 0..count {
        let result = rand::rng().random_bool(0.5);
        if boolean {
            println!("{result}");
        } else {
            println!("{}", if result { "heads" } else { "tails" });
        }
    }
}
