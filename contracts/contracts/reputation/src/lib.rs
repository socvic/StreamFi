#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, symbol_short, Address, Env, Map};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    ProfileNotFound = 4,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    pub address: Address,
    pub score: u32,
    pub successful_payments: u32,
    pub total_payments: u32,
    pub tier: u32,
}

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&symbol_short!("admin")) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("admin"), &admin);
        env.events().publish((symbol_short!("init"),), admin);
        Ok(())
    }

    pub fn create_profile(env: Env, address: Address) -> Result<(), Error> {
        address.require_auth();
        
        let profile = Profile {
            address: address.clone(),
            score: 100,
            successful_payments: 0,
            total_payments: 0,
            tier: 1,
        };
        
        env.storage().persistent().set(&address, &profile);
        env.events().publish((symbol_short!("profile"), address), true);
        Ok(())
    }

    pub fn record_payment(env: Env, address: Address, success: bool) -> Result<(), Error> {
        let mut profile: Profile = env.storage().persistent().get(&address).ok_or(Error::ProfileNotFound)?;
        
        profile.total_payments += 1;
        if success {
            profile.successful_payments += 1;
            profile.score = (profile.score as i32 + 5).min(1000) as u32;
        } else {
            profile.score = profile.score.saturating_sub(10);
        }
        
        profile.tier = match profile.score {
            0..=200 => 1,
            201..=400 => 2,
            401..=600 => 3,
            601..=800 => 4,
            _ => 5,
        };
        
        env.storage().persistent().set(&address, &profile);
        env.events().publish((symbol_short!("update"), address), success);
        Ok(())
    }

    pub fn get_profile(env: Env, address: Address) -> Result<Profile, Error> {
        env.storage().persistent().get(&address).ok_or(Error::ProfileNotFound)
    }
}

mod test;
