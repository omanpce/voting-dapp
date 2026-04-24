#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Map, String, Symbol, Vec,
};

// ─── Storage Keys ────────────────────────────────────────────────────────────

const ADMIN: Symbol        = symbol_short!("ADMIN");
const PROPOSALS: Symbol    = symbol_short!("PROPOSALS");
const VOTES: Symbol        = symbol_short!("VOTES");
const PROP_COUNT: Symbol   = symbol_short!("PROP_CNT");
const VOTING_OPEN: Symbol  = symbol_short!("V_OPEN");

// ─── Data Types ──────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub vote_count: u32,
    pub creator: Address,
}

#[contracttype]
pub enum DataKey {
    Proposal(u32),
    HasVoted(Address, u32), // (voter, proposal_id)
}

// ─── Contract ────────────────────────────────────────────────────────────────

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    // ── Initialise ─────────────────────────────────────────────────────────

    /// Deploy the contract. Call once; panics if already initialised.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialised");
        }
        admin.require_auth();
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&PROP_COUNT, &0u32);
        env.storage().instance().set(&VOTING_OPEN, &true);
    }

    // ── Admin actions ──────────────────────────────────────────────────────

    /// Open or close voting (admin only).
    pub fn set_voting_open(env: Env, open: bool) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();
        env.storage().instance().set(&VOTING_OPEN, &open);
    }

    // ── Proposals ─────────────────────────────────────────────────────────

    /// Add a new proposal. Anyone can propose while voting is open.
    pub fn add_proposal(
        env: Env,
        creator: Address,
        title: String,
        description: String,
    ) -> u32 {
        creator.require_auth();
        Self::assert_voting_open(&env);

        let id: u32 = env.storage().instance().get(&PROP_COUNT).unwrap_or(0);
        let proposal = Proposal {
            id,
            title,
            description,
            vote_count: 0,
            creator,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Proposal(id), &proposal);

        env.storage().instance().set(&PROP_COUNT, &(id + 1));
        id
    }

    /// Fetch a single proposal by id.
    pub fn get_proposal(env: Env, id: u32) -> Proposal {
        env.storage()
            .persistent()
            .get(&DataKey::Proposal(id))
            .unwrap_or_else(|| panic!("proposal not found"))
    }

    /// Return the total number of proposals.
    pub fn proposal_count(env: Env) -> u32 {
        env.storage().instance().get(&PROP_COUNT).unwrap_or(0)
    }

    // ── Voting ────────────────────────────────────────────────────────────

    /// Cast a vote for a proposal. Each address may vote once per proposal.
    pub fn vote(env: Env, voter: Address, proposal_id: u32) {
        voter.require_auth();
        Self::assert_voting_open(&env);

        let voted_key = DataKey::HasVoted(voter.clone(), proposal_id);
        if env.storage().persistent().has(&voted_key) {
            panic!("already voted on this proposal");
        }

        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("proposal not found"));

        proposal.vote_count += 1;

        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &proposal);

        // Mark voter as having voted on this proposal
        env.storage().persistent().set(&voted_key, &true);
    }

    /// Check whether an address has voted on a given proposal.
    pub fn has_voted(env: Env, voter: Address, proposal_id: u32) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::HasVoted(voter, proposal_id))
    }

    /// Return current vote count for a proposal.
    pub fn get_vote_count(env: Env, proposal_id: u32) -> u32 {
        let proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("proposal not found"));
        proposal.vote_count
    }

    // ── Status ────────────────────────────────────────────────────────────

    /// Is voting currently open?
    pub fn voting_open(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&VOTING_OPEN)
            .unwrap_or(false)
    }

    /// Return the admin address.
    pub fn admin(env: Env) -> Address {
        env.storage().instance().get(&ADMIN).unwrap()
    }

    // ── Helpers ───────────────────────────────────────────────────────────

    fn assert_voting_open(env: &Env) {
        let open: bool = env
            .storage()
            .instance()
            .get(&VOTING_OPEN)
            .unwrap_or(false);
        if !open {
            panic!("voting is closed");
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, VotingContractClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        (env, client, admin)
    }

    #[test]
    fn test_initialize() {
        let (_env, client, admin) = setup();
        assert_eq!(client.admin(), admin);
        assert!(client.voting_open());
        assert_eq!(client.proposal_count(), 0);
    }

    #[test]
    fn test_add_proposal() {
        let (env, client, _admin) = setup();
        let creator = Address::generate(&env);
        let id = client.add_proposal(
            &creator,
            &String::from_str(&env, "Proposal Alpha"),
            &String::from_str(&env, "Description of proposal alpha"),
        );
        assert_eq!(id, 0);
        assert_eq!(client.proposal_count(), 1);
        let p = client.get_proposal(&0);
        assert_eq!(p.vote_count, 0);
    }

    #[test]
    fn test_vote() {
        let (env, client, _admin) = setup();
        let creator = Address::generate(&env);
        let voter = Address::generate(&env);
        client.add_proposal(
            &creator,
            &String::from_str(&env, "Proposal Beta"),
            &String::from_str(&env, "Description"),
        );
        assert!(!client.has_voted(&voter, &0));
        client.vote(&voter, &0);
        assert!(client.has_voted(&voter, &0));
        assert_eq!(client.get_vote_count(&0), 1);
    }

    #[test]
    #[should_panic(expected = "already voted on this proposal")]
    fn test_double_vote_fails() {
        let (env, client, _admin) = setup();
        let creator = Address::generate(&env);
        let voter = Address::generate(&env);
        client.add_proposal(
            &creator,
            &String::from_str(&env, "Proposal Gamma"),
            &String::from_str(&env, "Description"),
        );
        client.vote(&voter, &0);
        client.vote(&voter, &0); // should panic
    }

    #[test]
    #[should_panic(expected = "voting is closed")]
    fn test_vote_when_closed_fails() {
        let (env, client, admin) = setup();
        let creator = Address::generate(&env);
        let voter = Address::generate(&env);
        client.add_proposal(
            &creator,
            &String::from_str(&env, "Proposal Delta"),
            &String::from_str(&env, "Description"),
        );
        client.set_voting_open(&admin, &false);
        client.vote(&voter, &0); // should panic
    }

    #[test]
    fn test_multiple_proposals_and_voters() {
        let (env, client, _admin) = setup();
        let creator = Address::generate(&env);
        // Add two proposals
        client.add_proposal(
            &creator,
            &String::from_str(&env, "Proposal One"),
            &String::from_str(&env, "Desc One"),
        );
        client.add_proposal(
            &creator,
            &String::from_str(&env, "Proposal Two"),
            &String::from_str(&env, "Desc Two"),
        );
        // Three voters each vote on proposal 0
        for _ in 0..3 {
            let v = Address::generate(&env);
            client.vote(&v, &0);
        }
        // One voter votes on proposal 1
        let v = Address::generate(&env);
        client.vote(&v, &1);

        assert_eq!(client.get_vote_count(&0), 3);
        assert_eq!(client.get_vote_count(&1), 1);
    }
}