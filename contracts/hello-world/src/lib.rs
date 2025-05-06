#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short, String};

// Structure for a Product
#[contracttype]
#[derive(Clone)]
pub struct Product {
    pub product_id: u64,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub available: bool,
}

// Key for tracking product count
const PRODUCT_COUNT: Symbol = symbol_short!("P_COUNT");

// Mapping Product IDs to Product details
#[contracttype]
pub enum ProductKey {
    Item(u64),
}

#[contract]
pub struct EcommerceContract;

#[contractimpl]
impl EcommerceContract {
    // Add a new product
    pub fn add_product(env: Env, name: String, description: String, price: u64) -> u64 {
        let mut count: u64 = env.storage().instance().get(&PRODUCT_COUNT).unwrap_or(0);
        count += 1;

        let product = Product {
            product_id: count,
            name,
            description,
            price,
            available: true,
        };

        env.storage().instance().set(&ProductKey::Item(count), &product);
        env.storage().instance().set(&PRODUCT_COUNT, &count);
        count
    }

    // Retrieve a product by ID
    pub fn get_product(env: Env, product_id: u64) -> Product {
        env.storage()
            .instance()
            .get(&ProductKey::Item(product_id))
            .unwrap_or(Product {
                product_id: 0,
                name: String::from_str(&env, "Not Found"),
                description: String::from_str(&env, "N/A"),
                price: 0,
                available: false,
            })
    }

    // Mark a product as unavailable (simulate "purchase" or "removal")
    pub fn disable_product(env: Env, product_id: u64) {
        let mut product = Self::get_product(env.clone(), product_id);
        product.available = false;
        env.storage().instance().set(&ProductKey::Item(product_id), &product);
    }
}
