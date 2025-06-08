use sha2::{Digest, Sha256};
use chrono::prelude::*;

#[derive(Debug, Clone)]
struct Block {
    index: u32,
    timestamp: String,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, data: String, prev_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut block = Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}",
            self.index, self.prev_hash, self.timestamp, self.data
        ));
        format!("{:x}", hasher.finalize())
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".into(), "0".into());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let prev_block = self.chain.last().unwrap();
        let new_block = Block::new(
            prev_block.index + 1,
            data,
            prev_block.hash.clone(),
        );
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.prev_hash != previous.hash {
                return false;
            }
        }
        true
    }

    fn tamper_block(&mut self, index: usize, new_data: &str) {
        if index < self.chain.len() {
            self.chain[index].data = new_data.into();
            self.chain[index].hash = self.chain[index].calculate_hash();
        }
    }

    fn recompute_hashes_from(&mut self, index: usize) {
        for i in index..self.chain.len() {
            if i == 0 {
                self.chain[i].prev_hash = "0".into();
            } else {
                self.chain[i].prev_hash = self.chain[i - 1].hash.clone();
            }
            self.chain[i].hash = self.chain[i].calculate_hash();
        }
    }

    fn print_chain(&self) {
        for block in &self.chain {
            println!(
                "Block {} | Data: {} | Hash: {} | Prev Hash: {}\n",
                block.index, block.data, block.hash, block.prev_hash
            );
        }
    }
}

fn main() {
    let mut my_chain = Blockchain::new();
    my_chain.add_block("hari pays ram 10 coins".into());
    my_chain.add_block("ram pays rahim 5 coins".into());
    my_chain.add_block("rahim pays tina 2 coins".into());

    println!("✅ Initial Blockchain:");
    my_chain.print_chain();
    println!("Is chain valid? {}", my_chain.is_valid());

    // Tamper block 1
    println!("\n⚠️ Tampering with Block 1...");
    my_chain.tamper_block(1, "Hari pays ram 1000 coins");

    println!("\n⛔ Blockchain after tampering:");
    my_chain.print_chain();
    println!("Is chain valid? {}", my_chain.is_valid());

    // Recalculate hashes to fix the chain
    println!("\n🔄 Recomputing hashes from Block 1...");
    my_chain.recompute_hashes_from(1);

    println!("\n✅ Blockchain after fixing:");
    my_chain.print_chain();
    println!("Is chain valid? {}", my_chain.is_valid());
}
