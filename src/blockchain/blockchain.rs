use std::sync::{Arc, Mutex};
use std::collections::{HashSet, HashMap};
use crate::blockchain::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub voters: HashMap<String, HashSet<String>>,
    pub elections: HashMap<String, HashSet<String>>,  
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            voters: HashMap::new(),
            elections: HashMap::new(),  
        };

        // Criar o bloco gênesis
        let genesis_block = Block::new(0, String::from(""), String::from(""), String::from(""), String::from("0"));
        blockchain.chain.push(genesis_block);

        blockchain
    }

    pub fn create_election(&mut self, election_id: String, vote_options: HashSet<String>) -> Result<(), String> {
        if self.elections.contains_key(&election_id) {
            return Err("Election already exists".to_string());
        }

        self.elections.insert(election_id, vote_options);
        Ok(())
    }

    pub fn add_vote_operation(&mut self, voter_id: String, election_id: String, vote_option_id: String) -> Result<(), String> {
        if let Some(vote_options) = self.elections.get(&election_id) {
            if !vote_options.contains(&vote_option_id) {
                return Err("Vote option does not exist in this election".to_string());
            }
        } else {
            return Err("Election does not exist".to_string());
        }

        if let Some(voters) = self.voters.get(&election_id) {
            if voters.contains(&voter_id) {
                return Err("Voter has already voted in this election".to_string());
            }
        } else {
            self.voters.insert(election_id.clone(), HashSet::new());
        }

        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            voter_id.clone(),
            election_id.clone(),
            vote_option_id,
            last_block.hash.clone(),
        );

        self.chain.push(new_block);
        self.voters.get_mut(&election_id).unwrap().insert(voter_id);

        Ok(())
    }
}

pub type SharedBlockchain = Arc<Mutex<Blockchain>>;