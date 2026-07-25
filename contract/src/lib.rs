#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

const NEXT_ID_KEY: Symbol = symbol_short!("NEXT_ID");

#[derive(Clone)]
#[contracttype]
pub struct Poll {
    pub id: u32,
    pub question: soroban_sdk::String,
    pub options: soroban_sdk::Vec<soroban_sdk::String>,
    pub votes: soroban_sdk::Vec<u32>,
    pub creator: Address,
    pub total_votes: u32,
    pub active: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum PollEntry {
    Poll(Poll),
}

#[contract]
pub struct LivePoll;

#[contractimpl]
impl LivePoll {
    pub fn initialize(env: Env) {
        if env.storage().instance().has(&NEXT_ID_KEY) {
            panic!("already initialized");
        }
        env.storage().instance().set(&NEXT_ID_KEY, &0u32);
    }

    pub fn create_poll(
        env: Env,
        creator: Address,
        question: soroban_sdk::String,
        options: soroban_sdk::Vec<soroban_sdk::String>,
    ) -> u32 {
        creator.require_auth();

        if options.len() < 2 || options.len() > 10 {
            panic!("poll must have 2-10 options");
        }

        let poll_id: u32 = env.storage().instance().get(&NEXT_ID_KEY).unwrap_or(0);
        let mut vote_counts = soroban_sdk::Vec::new(&env);
        for _ in 0..options.len() {
            vote_counts.push_back(0u32);
        }

        let poll = Poll {
            id: poll_id,
            question,
            options,
            votes: vote_counts,
            creator: creator.clone(),
            total_votes: 0,
            active: true,
        };

        let key = symbol_short!("POLL");
        let poll_key = (&key, poll_id);
        env.storage().persistent().set(&poll_key, &poll);
        env.storage()
            .instance()
            .set(&NEXT_ID_KEY, &(poll_id + 1));

        env.events().publish(
            (symbol_short!("CREATE"),),
            (poll_id, creator),
        );

        poll_id
    }

    pub fn vote(env: Env, voter: Address, poll_id: u32, option_index: u32) {
        voter.require_auth();

        let key = symbol_short!("POLL");
        let poll_key = (&key, poll_id);
        let mut poll: Poll = env
            .storage()
            .persistent()
            .get(&poll_key)
            .expect("poll not found");

        if !poll.active {
            panic!("poll is closed");
        }

        if option_index >= poll.options.len() {
            panic!("invalid option index");
        }

        let voted_key = (symbol_short!("VOTED"), poll_id, voter.clone());
        if env.storage().persistent().has(&voted_key) {
            panic!("already voted");
        }

        let current = poll.votes.get_unchecked(option_index);
        poll.votes.set(option_index, current + 1);
        poll.total_votes += 1;

        env.storage().persistent().set(&poll_key, &poll);
        env.storage().persistent().set(&voted_key, &true);

        env.events().publish(
            (symbol_short!("VOTE"),),
            (poll_id, voter, option_index),
        );
    }

    pub fn close_poll(env: Env, caller: Address, poll_id: u32) {
        caller.require_auth();

        let key = symbol_short!("POLL");
        let poll_key = (&key, poll_id);
        let mut poll: Poll = env
            .storage()
            .persistent()
            .get(&poll_key)
            .expect("poll not found");

        if poll.creator != caller {
            panic!("only creator can close poll");
        }

        poll.active = false;
        env.storage().persistent().set(&poll_key, &poll);

        env.events().publish(
            (symbol_short!("CLOSE"),),
            (poll_id,),
        );
    }

    pub fn get_poll(env: Env, poll_id: u32) -> Poll {
        let key = symbol_short!("POLL");
        let poll_key = (&key, poll_id);
        env.storage()
            .persistent()
            .get(&poll_key)
            .expect("poll not found")
    }

    pub fn get_results(env: Env, poll_id: u32) -> soroban_sdk::Vec<u32> {
        let poll = Self::get_poll(env, poll_id);
        poll.votes
    }

    pub fn get_poll_count(env: Env) -> u32 {
        env.storage().instance().get(&NEXT_ID_KEY).unwrap_or(0)
    }

    pub fn has_voted(env: Env, poll_id: u32, voter: Address) -> bool {
        let voted_key = (symbol_short!("VOTED"), poll_id, voter);
        env.storage().persistent().has(&voted_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::{Address as _, Events}, Env};

    fn create_test_env() -> (Env, Address) {
        let env = Env::default();
        let admin = Address::generate(&env);
        env.mock_all_auths();
        (env, admin)
    }

    #[test]
    fn test_initialize() {
        let (env, _) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();
        assert_eq!(client.get_poll_count(), 0);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn test_initialize_twice_panics() {
        let (env, _) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();
        client.initialize();
    }

    #[test]
    fn test_create_poll() {
        let (env, admin) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();

        let question = soroban_sdk::String::from_str(&env, "Favorite color?");
        let mut options = soroban_sdk::Vec::new(&env);
        options.push_back(soroban_sdk::String::from_str(&env, "Red"));
        options.push_back(soroban_sdk::String::from_str(&env, "Blue"));

        let poll_id = client.create_poll(&admin, &question, &options);
        assert_eq!(poll_id, 0);
        assert_eq!(client.get_poll_count(), 1);

        let poll = client.get_poll(&0);
        assert_eq!(poll.id, 0);
        assert_eq!(poll.question, question);
        assert_eq!(poll.total_votes, 0);
        assert!(poll.active);
    }

    #[test]
    #[should_panic(expected = "poll must have 2-10 options")]
    fn test_create_poll_invalid_options_too_few() {
        let (env, admin) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();

        let question = soroban_sdk::String::from_str(&env, "Only one?");
        let mut options = soroban_sdk::Vec::new(&env);
        options.push_back(soroban_sdk::String::from_str(&env, "Yes"));

        client.create_poll(&admin, &question, &options);
    }

    #[test]
    fn test_vote() {
        let (env, admin) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();

        let question = soroban_sdk::String::from_str(&env, "JS or Rust?");
        let mut options = soroban_sdk::Vec::new(&env);
        options.push_back(soroban_sdk::String::from_str(&env, "JS"));
        options.push_back(soroban_sdk::String::from_str(&env, "Rust"));

        client.create_poll(&admin, &question, &options);

        let voter = Address::generate(&env);
        client.vote(&voter, &0, &1);

        let results = client.get_results(&0);
        assert_eq!(results.get(0).unwrap(), 0);
        assert_eq!(results.get(1).unwrap(), 1);

        let poll = client.get_poll(&0);
        assert_eq!(poll.total_votes, 1);
    }

    #[test]
    #[should_panic(expected = "already voted")]
    fn test_already_voted() {
        let (env, admin) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();

        let question = soroban_sdk::String::from_str(&env, "A or B?");
        let mut options = soroban_sdk::Vec::new(&env);
        options.push_back(soroban_sdk::String::from_str(&env, "A"));
        options.push_back(soroban_sdk::String::from_str(&env, "B"));

        client.create_poll(&admin, &question, &options);

        let voter = Address::generate(&env);
        client.vote(&voter, &0, &0);

        assert!(client.has_voted(&0, &voter));
        client.vote(&voter, &0, &1);
    }

    #[test]
    fn test_close_poll() {
        let (env, admin) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();

        let question = soroban_sdk::String::from_str(&env, "Close me");
        let mut options = soroban_sdk::Vec::new(&env);
        options.push_back(soroban_sdk::String::from_str(&env, "Yes"));
        options.push_back(soroban_sdk::String::from_str(&env, "No"));

        client.create_poll(&admin, &question, &options);
        client.close_poll(&admin, &0);

        let poll = client.get_poll(&0);
        assert!(!poll.active);
    }

    #[test]
    #[should_panic(expected = "only creator can close poll")]
    fn test_close_poll_wrong_creator() {
        let (env, admin) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();

        let question = soroban_sdk::String::from_str(&env, "Nope");
        let mut options = soroban_sdk::Vec::new(&env);
        options.push_back(soroban_sdk::String::from_str(&env, "A"));
        options.push_back(soroban_sdk::String::from_str(&env, "B"));

        client.create_poll(&admin, &question, &options);

        let other = Address::generate(&env);
        client.close_poll(&other, &0);
    }

    #[test]
    #[should_panic(expected = "poll is closed")]
    fn test_vote_on_closed_poll() {
        let (env, admin) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();

        let question = soroban_sdk::String::from_str(&env, "Closed poll");
        let mut options = soroban_sdk::Vec::new(&env);
        options.push_back(soroban_sdk::String::from_str(&env, "X"));
        options.push_back(soroban_sdk::String::from_str(&env, "Y"));

        client.create_poll(&admin, &question, &options);
        client.close_poll(&admin, &0);

        let voter = Address::generate(&env);
        client.vote(&voter, &0, &0);
    }

    #[test]
    fn test_multiple_voters() {
        let (env, admin) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();

        let question = soroban_sdk::String::from_str(&env, "Best chain?");
        let mut options = soroban_sdk::Vec::new(&env);
        options.push_back(soroban_sdk::String::from_str(&env, "Stellar"));
        options.push_back(soroban_sdk::String::from_str(&env, "Ethereum"));

        client.create_poll(&admin, &question, &options);

        let v1 = Address::generate(&env);
        let v2 = Address::generate(&env);
        let v3 = Address::generate(&env);

        client.vote(&v1, &0, &0);
        client.vote(&v2, &0, &0);
        client.vote(&v3, &0, &1);

        let results = client.get_results(&0);
        assert_eq!(results.get(0).unwrap(), 2);
        assert_eq!(results.get(1).unwrap(), 1);

        let poll = client.get_poll(&0);
        assert_eq!(poll.total_votes, 3);

        assert!(client.has_voted(&0, &v1));
        assert!(client.has_voted(&0, &v2));
        assert!(!client.has_voted(&0, &Address::generate(&env)));
    }

    #[test]
    fn test_get_poll_count() {
        let (env, admin) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();

        assert_eq!(client.get_poll_count(), 0);

        let q1 = soroban_sdk::String::from_str(&env, "Q1");
        let q2 = soroban_sdk::String::from_str(&env, "Q2");
        let mut opts = soroban_sdk::Vec::new(&env);
        opts.push_back(soroban_sdk::String::from_str(&env, "A"));
        opts.push_back(soroban_sdk::String::from_str(&env, "B"));

        client.create_poll(&admin, &q1, &opts);
        assert_eq!(client.get_poll_count(), 1);

        client.create_poll(&admin, &q2, &opts);
        assert_eq!(client.get_poll_count(), 2);
    }

    #[test]
    fn test_events_published() {
        let (env, admin) = create_test_env();
        let contract_id = env.register(LivePoll, ());
        let client = LivePollClient::new(&env, &contract_id);
        client.initialize();

        let question = soroban_sdk::String::from_str(&env, "Event test");
        let mut options = soroban_sdk::Vec::new(&env);
        options.push_back(soroban_sdk::String::from_str(&env, "A"));
        options.push_back(soroban_sdk::String::from_str(&env, "B"));

        client.create_poll(&admin, &question, &options);

        let events = env.events().all();
        assert!(events.len() >= 1);
    }
}
