#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, Map};

#[contract]
pub struct StellarVote;

#[contractimpl]
impl StellarVote {

    fn voters_key() -> Symbol {
        symbol_short!("VOTERS")
    }

    fn votes_key() -> Symbol {
        symbol_short!("VOTES")
    }

    fn voted_key() -> Symbol {
        symbol_short!("VOTED")
    }

    pub fn init(env: Env) {
        let votes: Map<u32, u32> = Map::new(&env);
        env.storage().instance().set(&Self::votes_key(), &votes);
    }

    pub fn vote(env: Env, voter: Address, proposal_id: u32) {
        voter.require_auth();

        let mut voted: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&Self::voted_key())
            .unwrap_or(Map::new(&env));

        if voted.get(voter.clone()).unwrap_or(false) {
            panic!("Already voted!");
        }

        let mut votes: Map<u32, u32> = env
            .storage()
            .instance()
            .get(&Self::votes_key())
            .unwrap();

        let count = votes.get(proposal_id).unwrap_or(0);
        votes.set(proposal_id, count + 1);

        voted.set(voter, true);

        env.storage().instance().set(&Self::votes_key(), &votes);
        env.storage().instance().set(&Self::voted_key(), &voted);
    }

    pub fn get_votes(env: Env, proposal_id: u32) -> u32 {
        let votes: Map<u32, u32> = env
            .storage()
            .instance()
            .get(&Self::votes_key())
            .unwrap();

        votes.get(proposal_id).unwrap_or(0)
    }
}
