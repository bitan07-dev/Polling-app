#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct PollingContract;

#[contractimpl]
impl PollingContract {
    /// Casts a vote for a specific option and returns the updated vote count.
    /// In Soroban, `Symbol` is a highly efficient short string (up to 32 characters).
    pub fn vote(env: Env, option: Symbol) -> u32 {
        // Retrieve the current vote count from the contract's instance storage.
        // If the option hasn't been voted for yet, default to 0.
        let mut count: u32 = env.storage().instance().get(&option).unwrap_or(0);
        
        // Increment the vote count
        count += 1;
        
        // Save the updated count back to the blockchain's storage
        env.storage().instance().set(&option, &count);
        
        // Extend the Time To Live (TTL) of the instance storage so the data isn't archived
        env.storage().instance().extend_ttl(100, 100);
        
        count
    }

    /// Fetches the current number of votes for a given option without modifying state.
    pub fn get_votes(env: Env, option: Symbol) -> u32 {
        env.storage().instance().get(&option).unwrap_or(0)
    }
}
