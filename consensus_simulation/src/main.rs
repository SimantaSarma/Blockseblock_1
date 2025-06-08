use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
struct Validator {
    id: String,
    power: u32,   // for PoW
    stake: u32,   // for PoS
}

fn pow_select(validators: &[Validator]) -> &Validator {
    println!("🔨 PoW selects the validator with the highest mining power.");
    validators.iter().max_by_key(|v| v.power).unwrap()
}

fn pos_select(validators: &[Validator]) -> &Validator {
    println!("💰 PoS selects the validator with the highest stake.");
    validators.iter().max_by_key(|v| v.stake).unwrap()
}

fn dpos_select(voters: &[String]) -> String {
    println!("🗳️ DPoS selects a delegate randomly but weighted by votes.");

    // Count votes for each delegate
    let mut vote_count: HashMap<String, u32> = HashMap::new();
    for vote in voters {
        *vote_count.entry(vote.clone()).or_insert(0) += 1;
    }

    // Display vote counts
    println!("Vote counts per delegate: {:?}", vote_count);

    // Build a weighted list for random choice
    let mut weighted_list = vec![];
    for (delegate, count) in &vote_count {
        for _ in 0..*count {
            weighted_list.push(delegate);
        }
    }

    let mut rng = rand::thread_rng();
    let winner = weighted_list[rng.gen_range(0..weighted_list.len())];

    winner.to_string()
}

fn main() {
    let mut rng = rand::thread_rng();

    // Create 3 validators with random power and stake
    let validators = vec![
        Validator {
            id: "Validator_A".to_string(),
            power: rng.gen_range(1..101),
            stake: rng.gen_range(1..101),
        },
        Validator {
            id: "Validator_B".to_string(),
            power: rng.gen_range(1..101),
            stake: rng.gen_range(1..101),
        },
        Validator {
            id: "Validator_C".to_string(),
            power: rng.gen_range(1..101),
            stake: rng.gen_range(1..101),
        },
    ];

    println!("Validators:\n{:#?}\n", validators);

    // Simulate PoW
    let pow_winner = pow_select(&validators);
    println!("PoW winner: {:?}\n", pow_winner);

    // Simulate PoS
    let pos_winner = pos_select(&validators);
    println!("PoS winner: {:?}\n", pos_winner);

    // DPoS voters voting for delegates
    let voters = vec![
        "Validator_A".to_string(),
        "Validator_A".to_string(),
        "Validator_B".to_string(),
    ];

    let dpos_winner = dpos_select(&voters);
    println!("DPoS winner: {}\n", dpos_winner);
}
