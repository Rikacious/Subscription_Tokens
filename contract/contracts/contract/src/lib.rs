#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env,
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Price,
    Duration,
    Subscriber(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subscription {
    pub expires_at: u64,
    pub active: bool,
}

#[contract]
pub struct SubscriptionTokensContract;

#[contractimpl]
impl SubscriptionTokensContract {
    pub fn initialize(env: Env, admin: Address, price: i128, duration: u64) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Price, &price);
        env.storage().instance().set(&DataKey::Duration, &duration);
    }

    pub fn subscribe(env: Env, user: Address) {
        user.require_auth();

        let duration: u64 = env
            .storage()
            .instance()
            .get(&DataKey::Duration)
            .unwrap_or(30 * 24 * 60 * 60);

        let now = env.ledger().timestamp();

        let existing = env
            .storage()
            .persistent()
            .get::<DataKey, Subscription>(&DataKey::Subscriber(user.clone()));

        let new_expiry = match existing {
            Some(sub) if sub.active && sub.expires_at > now => sub.expires_at + duration,
            _ => now + duration,
        };

        let subscription = Subscription {
            expires_at: new_expiry,
            active: true,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Subscriber(user.clone()), &subscription);

        env.events()
            .publish((symbol_short!("SUBSCRIBE"), user), new_expiry);
    }

    pub fn is_active(env: Env, user: Address) -> bool {
        let now = env.ledger().timestamp();

        match env
            .storage()
            .persistent()
            .get::<DataKey, Subscription>(&DataKey::Subscriber(user))
        {
            Some(sub) => sub.active && sub.expires_at > now,
            None => false,
        }
    }

    pub fn get_subscription(env: Env, user: Address) -> Subscription {
        env.storage()
            .persistent()
            .get::<DataKey, Subscription>(&DataKey::Subscriber(user))
            .unwrap_or(Subscription {
                expires_at: 0,
                active: false,
            })
    }

    pub fn get_price(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Price).unwrap_or(0)
    }

    pub fn set_price(env: Env, admin: Address, new_price: i128) {
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        stored_admin.require_auth();

        if admin != stored_admin {
            panic!("not authorized");
        }

        env.storage().instance().set(&DataKey::Price, &new_price);
    }

    pub fn get_duration(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::Duration).unwrap_or(0)
    }

    pub fn set_duration(env: Env, admin: Address, new_duration: u64) {
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        stored_admin.require_auth();

        if admin != stored_admin {
            panic!("not authorized");
        }

        env.storage().instance().set(&DataKey::Duration, &new_duration);
    }
}