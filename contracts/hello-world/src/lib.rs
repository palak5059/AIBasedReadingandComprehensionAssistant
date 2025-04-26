#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String};

#[contracttype]
#[derive(Clone)]
pub struct Article {
    pub title: String,
    pub summary: String,
}

#[contract]
pub struct AIReadAssistContract;

#[contractimpl]
impl AIReadAssistContract {
    // Store a new article
    pub fn store_article(env: Env, title: String, summary: String) {
        env.storage().instance().set(&title, &summary);
    }

    // Retrieve a summary by title
    pub fn get_summary(env: Env, title: String) -> String {
        env.storage().instance().get(&title).unwrap_or(String::from_str(&env, "Not Found"))
    }

    // Update a summary
    pub fn update_summary(env: Env, title: String, new_summary: String) {
        if env.storage().instance().has(&title) {
            env.storage().instance().set(&title, &new_summary);
        } else {
            panic!("Article not found");
        }
    }

    // Delete an article
    pub fn delete_article(env: Env, title: String) {
        env.storage().instance().remove(&title);
    }
}