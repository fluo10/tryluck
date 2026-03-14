use rand::RngExt as _;
use serde_json::json;

pub fn run(count: u32, sides: u32, modifier: i32, sum: bool, output_json: bool) {
    let mut rng = rand::rng();
    let rolls: Vec<u32> = (0..count).map(|_| rng.random_range(1..=sides)).collect();
    let show_total = sum || modifier != 0;
    let total: i64 = rolls.iter().map(|&r| r as i64).sum::<i64>() + modifier as i64;

    if output_json {
        if show_total {
            println!("{}", serde_json::to_string_pretty(&json!({ "rolls": rolls, "total": total })).unwrap());
        } else {
            println!("{}", serde_json::to_string_pretty(&rolls).unwrap());
        }
    } else {
        for roll in &rolls {
            println!("{roll}");
        }
        if show_total {
            println!("Total: {total}");
        }
    }
}

/// Parse a spec string into (count, sides, modifier).
/// Accepts:
/// - Dice notation: "3d10", "d6", "3d10+2", "2d8-1"
/// - Plain number: "3" (treated as count)
/// Returns None for any component not present in the spec.
pub fn parse_spec(spec: &str) -> (Option<u32>, Option<u32>, Option<i32>) {
    let lower = spec.to_ascii_lowercase();
    if let Some(d_pos) = lower.find('d') {
        let count_str = &spec[..d_pos];
        let rest = &spec[d_pos + 1..];

        let count = if count_str.is_empty() {
            None
        } else {
            count_str.parse().ok()
        };

        // Split rest into sides and modifier at first '+' or '-'
        let (sides_str, modifier) = if let Some(plus_pos) = rest.find('+') {
            let m: i32 = rest[plus_pos + 1..].parse().unwrap_or(0);
            (&rest[..plus_pos], Some(m))
        } else if let Some(minus_pos) = rest.find('-') {
            let m: i32 = rest[minus_pos + 1..].parse::<i32>().map(|v| -v).unwrap_or(0);
            (&rest[..minus_pos], Some(m))
        } else {
            (rest, None)
        };

        (count, sides_str.parse().ok(), modifier)
    } else {
        // Plain number = count only
        (spec.parse().ok(), None, None)
    }
}
