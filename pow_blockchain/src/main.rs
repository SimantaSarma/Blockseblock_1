use sha2::{Digest, Sha256};
use chrono::prelude::*;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Block {
    index: u32,
    timestamp: String,
    data: String,
    prev_hash: String,
    hash: String,
    nonce: u64,
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
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}",
            self.index, self.prev_hash, self.timestamp, self.data, self.nonce
        ));
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        let start = Instant::now();

        while &self.hash[0..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        let duration = start.elapsed();
        println!(
            "✅ Block mined with nonce {} in {:.2?} | Hash: {}",
            self.nonce, duration, self.hash
        );
    }
}

struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let mut genesis = Block::new(0, "Genesis Block".into(), "0".into());
        genesis.mine_block(difficulty);
        Blockchain {
            chain: vec![genesis],
            difficulty,
        }
    }

    fn add_block(&mut self, data: String) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let mut new_block = Block::new(self.chain.len() as u32, data, prev_hash);
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    fn print_chain(&self) {
        for block in &self.chain {
            println!(
                "Block {} | Nonce: {} | Hash: {} | Prev: {}\nData: {}\n",
                block.index, block.nonce, block.hash, block.prev_hash, block.data
            );
        }
    }
}

fn main() {
    let mut my_chain = Blockchain::new(4); // Set difficulty (number of leading 0s)

    my_chain.add_block("hari pays ram 10 coins".into());
    my_chain.add_block("ram pays rahim 5 coins".into());

    println!("\n📜 Final Blockchain:");
    my_chain.print_chain();
}
