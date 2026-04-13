#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, Symbol, Address, Map
};

#[contracttype]
#[derive(Clone)]
pub struct Audit {
    pub creator: Address,
    pub description: Symbol,
    pub approved: bool,
    pub votes: i32,
}

#[contract]
pub struct AuditDAO;

#[contractimpl]
impl AuditDAO {

    // Submit audit
    pub fn submit_audit(env: Env, user: Address, description: Symbol) {
        user.require_auth();

        let mut audits: Map<u32, Audit> = env
            .storage()
            .instance()
            .get(&symbol_short!("AUDITS"))
            .unwrap_or(Map::new(&env));

        let mut counter: u32 = env
            .storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0);

        let audit = Audit {
            creator: user,
            description,
            approved: false,
            votes: 0,
        };

        audits.set(counter, audit);
        counter += 1;

        env.storage().instance().set(&symbol_short!("AUDITS"), &audits);
        env.storage().instance().set(&symbol_short!("COUNT"), &counter);
    }

    // Approve audit
    pub fn approve_audit(env: Env, id: u32, approver: Address) {
        approver.require_auth();

        let mut audits: Map<u32, Audit> = env
            .storage()
            .instance()
            .get(&symbol_short!("AUDITS"))
            .unwrap();

        let mut audit = audits.get(id).unwrap();

        if audit.creator != approver {
            panic!("Not authorized");
        }

        audit.approved = true;

        audits.set(id, audit);
        env.storage().instance().set(&symbol_short!("AUDITS"), &audits);
    }

    // Vote
    pub fn vote(env: Env, id: u32, voter: Address, vote: bool) {
        voter.require_auth();

        let mut audits: Map<u32, Audit> = env
            .storage()
            .instance()
            .get(&symbol_short!("AUDITS"))
            .unwrap();

        let mut voted: Map<Symbol, bool> = env
            .storage()
            .instance()
            .get(&symbol_short!("VOTED"))
            .unwrap_or(Map::new(&env));

        // Create unique key like "vote_1_user"
        let key = Symbol::new(&env, "V");

        if voted.contains_key(key.clone()) {
            panic!("Already voted");
        }

        let mut audit = audits.get(id).unwrap();

        if vote {
            audit.votes += 1;
        } else {
            audit.votes -= 1;
        }

        voted.set(key, true);
        audits.set(id, audit);

        env.storage().instance().set(&symbol_short!("AUDITS"), &audits);
        env.storage().instance().set(&symbol_short!("VOTED"), &voted);
    }

    // Get audit
    pub fn get_audit(env: Env, id: u32) -> Audit {
        let audits: Map<u32, Audit> = env
            .storage()
            .instance()
            .get(&symbol_short!("AUDITS"))
            .unwrap();

        audits.get(id).unwrap()
    }
}