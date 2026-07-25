#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

const TOTAL_BADGES_KEY: Symbol = symbol_short!("TOTAL_BG");

#[derive(Clone)]
#[contracttype]
pub struct Badge {
    pub id: u32,
    pub holder: Address,
    pub badge_type: soroban_sdk::String,
    pub timestamp: u64,
}

#[contract]
pub struct VotingBadge;

#[contractimpl]
impl VotingBadge {
    pub fn initialize(env: Env) {
        if env.storage().instance().has(&TOTAL_BADGES_KEY) {
            panic!("already initialized");
        }
        env.storage()
            .instance()
            .set(&TOTAL_BADGES_KEY, &0u32);
    }

    pub fn award_badge(
        env: Env,
        holder: Address,
        badge_type: soroban_sdk::String,
    ) -> u32 {
        holder.require_auth();

        let badge_id: u32 = env
            .storage()
            .instance()
            .get(&TOTAL_BADGES_KEY)
            .unwrap_or(0);

        let timestamp = env.ledger().timestamp();

        let badge = Badge {
            id: badge_id,
            holder: holder.clone(),
            badge_type,
            timestamp,
        };

        let key = symbol_short!("BADGE");
        let badge_key = (&key, badge_id);
        env.storage().persistent().set(&badge_key, &badge);

        let holder_key = (symbol_short!("HOLDER"), holder.clone(), badge_id);
        env.storage().persistent().set(&holder_key, &true);

        let count_key = (symbol_short!("COUNT"), holder.clone());
        let current_count: u32 = env
            .storage()
            .persistent()
            .get(&count_key)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&count_key, &(current_count + 1));

        env.storage()
            .instance()
            .set(&TOTAL_BADGES_KEY, &(badge_id + 1));

        env.events().publish(
            (symbol_short!("BADGE"),),
            (badge_id, holder),
        );

        badge_id
    }

    pub fn get_badge(env: Env, badge_id: u32) -> Badge {
        let key = symbol_short!("BADGE");
        let badge_key = (&key, badge_id);
        env.storage()
            .persistent()
            .get(&badge_key)
            .expect("badge not found")
    }

    pub fn get_holder_count(env: Env, holder: Address) -> u32 {
        let count_key = (symbol_short!("COUNT"), holder);
        env.storage().persistent().get(&count_key).unwrap_or(0)
    }

    pub fn has_badge(env: Env, holder: Address, badge_id: u32) -> bool {
        let holder_key = (symbol_short!("HOLDER"), holder, badge_id);
        env.storage().persistent().has(&holder_key)
    }

    pub fn get_total_badges(env: Env) -> u32 {
        env.storage().instance().get(&TOTAL_BADGES_KEY).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, VotingBadgeClient<'static>) {
        let env = Env::default();
        let contract_id = env.register_contract(None, VotingBadge);
        let client = VotingBadgeClient::new(&env, &contract_id);
        client.initialize();
        (env, client)
    }

    #[test]
    fn test_initialize() {
        let (env, client) = setup();
        assert_eq!(client.get_total_badges(), 0);
        let _ = env;
    }

    #[test]
    fn test_award_badge() {
        let (env, client) = setup();
        let user = Address::generate(&env);
        let badge_type = soroban_sdk::String::from_str(&env, "first_vote");

        let badge_id = client.award_badge(&user, &badge_type);
        assert_eq!(badge_id, 0);
        assert_eq!(client.get_total_badges(), 1);

        let badge = client.get_badge(&0);
        assert_eq!(badge.id, 0);
        assert_eq!(badge.badge_type, badge_type);
    }

    #[test]
    fn test_get_holder_count() {
        let (env, client) = setup();
        let user = Address::generate(&env);

        assert_eq!(client.get_holder_count(&user), 0);

        let bt1 = soroban_sdk::String::from_str(&env, "first_vote");
        let bt2 = soroban_sdk::String::from_str(&env, "ten_votes");
        client.award_badge(&user, &bt1);
        client.award_badge(&user, &bt2);

        assert_eq!(client.get_holder_count(&user), 2);
    }

    #[test]
    fn test_has_badge() {
        let (env, client) = setup();
        let user = Address::generate(&env);

        assert!(!client.has_badge(&user, &0));

        let bt = soroban_sdk::String::from_str(&env, "active_voter");
        client.award_badge(&user, &bt);

        assert!(client.has_badge(&user, &0));
        assert!(!client.has_badge(&user, &1));
    }

    #[test]
    fn test_multiple_users() {
        let (env, client) = setup();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        let bt = soroban_sdk::String::from_str(&env, "participant");
        client.award_badge(&user1, &bt);
        client.award_badge(&user2, &bt);

        assert_eq!(client.get_total_badges(), 2);
        assert_eq!(client.get_holder_count(&user1), 1);
        assert_eq!(client.get_holder_count(&user2), 1);
        assert!(client.has_badge(&user1, &0));
        assert!(!client.has_badge(&user1, &1));
    }

    #[test]
    fn test_events_published() {
        let (env, client) = setup();
        let user = Address::generate(&env);
        let bt = soroban_sdk::String::from_str(&env, "event_test");

        client.award_badge(&user, &bt);

        let events = env.events().all();
        assert!(events.len() >= 1);
    }
}
