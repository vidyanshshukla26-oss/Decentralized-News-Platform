#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Vec, String, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct Article {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub author: String,
}

#[contract]
pub struct NewsContract;

#[contractimpl]
impl NewsContract {

    // Publish a new article
    pub fn publish(env: Env, id: u64, title: String, content: String, author: String) {
        let article = Article {
            id,
            title,
            content,
            author,
        };

        // Store article by ID
        env.storage().instance().set(&id, &article);

        // Maintain list of article IDs
        let key: Symbol = symbol_short!("IDS");
        let mut ids: Vec<u64> = env.storage().instance().get(&key).unwrap_or(Vec::new(&env));

        ids.push_back(id);
        env.storage().instance().set(&key, &ids);
    }

    // Get article by ID
    pub fn get(env: Env, id: u64) -> Article {
        env.storage().instance().get(&id).unwrap()
    }

    // Get all article IDs
    pub fn list(env: Env) -> Vec<u64> {
        let key: Symbol = symbol_short!("IDS");
        env.storage().instance().get(&key).unwrap_or(Vec::new(&env))
    }
}