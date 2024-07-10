use std::sync::{Arc, Mutex};
use std::collections::{HashSet, HashMap};
use crate::blockchain::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub voters: HashMap<String, HashSet<String>>,
    pub elections: HashMap<String, HashSet<String>>,
    pub creators: HashMap<String, HashSet<String>>  
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            voters: HashMap::new(),
            elections: HashMap::new(),
            creators: HashMap::new(), 
        };

        // Criar o bloco gênesis
        let genesis_block = Block::new(0, String::from(""), String::from(""), String::from(""), String::from("0"));
        blockchain.chain.push(genesis_block);

        blockchain
    }

    pub fn create_election(&mut self, election_id: String, vote_options: HashSet<String>, creator_id: String) -> Result<(), String> {
        if self.elections.contains_key(&election_id) {
            return Err("Election already exists".to_string());
        }

        if self.creators.contains_key(&creator_id) {
            self.creators
                .entry(creator_id.to_string())
                .or_insert_with(HashSet::new)
                .insert(election_id.to_string());
        } else {
            self.creators.insert(creator_id.clone(), [election_id.to_string()].iter().cloned().collect());
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

    pub fn get_votes_by_user(&self, voter_id: &str, election_id: &str) -> Option<(String, String)> {
        self.chain
            .iter()
            .rev()  // Itera reversamente para pegar o voto mais recente primeiro
            .find(|block| block.voter_id == voter_id && block.election_id == election_id)
            .map(|block| (block.election_id.clone(), block.vote_option_id.clone()))
    }

    pub fn get_elections_by_user(&self, voter_id: &str) -> Vec<(String, String)> {
        self.chain
            .iter()
            .filter(|block| block.voter_id == voter_id)
            .map(|block| (block.election_id.clone(), block.vote_option_id.clone()))
            .collect()
    
    
    }

    pub fn get_elections_created_by_user(&self, creator_id: &str) -> Vec<String> {
        self.creators
        .get(creator_id)
        .map_or_else(Vec::new, |elections| elections.iter().cloned().collect())
    
    
    }

    pub fn get_results_election(&self, election_id: &str) -> Vec<(String, String)> {
        self.chain
            .iter()
            .filter(|block| block.election_id == election_id)
            .map(|block| (block.election_id.clone(), block.vote_option_id.clone()))
            .collect()
    
    }

}

pub type SharedBlockchain = Arc<Mutex<Blockchain>>;