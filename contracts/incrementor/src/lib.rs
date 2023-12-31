#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementorContract;

#[contractimpl]
impl IncrementorContract {
    /// Increment an internal counter; return the new value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count = count + 1;
        log!(&env, "testing increment count: {}", count);
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().bump(100,500);
      //  println!("****************** ending **********{}",count);
        count
    }

     /// decrement an internal counter; return the new value.
    pub fn decrement(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
                     count = count - 1;

        log!(&env, "count: {}", count);
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().bump(100,500);
        count
    }

     /// Reset an internal counter; return the new value.
    pub fn reset(env: Env) -> u32 { 
        
        let count: u32 = 0;
        log!(&env, "count: {}", count);
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().bump(100,500);
        count
    }
}

#[cfg(test)]
mod test;