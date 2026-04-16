#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, symbol_short, Address, Env, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    StreamNotFound = 5,
    NothingToClaim = 6,
    StreamNotActive = 7,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stream {
    pub id: u32,
    pub sender: Address,
    pub recipient: Address,
    pub amount: i128,
    pub claimed: i128,
    pub start_time: u64,
    pub end_time: u64,
    pub rate_per_second: i128,
}

#[contract]
pub struct StreamingContract;

#[contractimpl]
impl StreamingContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&symbol_short!("admin")) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("admin"), &admin);
        env.storage().instance().set(&symbol_short!("count"), &0u32);
        env.events().publish((symbol_short!("init"),), admin);
        Ok(())
    }

    pub fn create_stream(
        env: Env,
        sender: Address,
        recipient: Address,
        amount: i128,
        start_time: u64,
        end_time: u64,
    ) -> Result<u32, Error> {
        sender.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if end_time <= start_time {
            return Err(Error::InvalidAmount);
        }

        let duration = end_time - start_time;
        let rate_per_second = amount / (duration as i128);

        let mut count: u32 = env.storage().instance().get(&symbol_short!("count")).unwrap_or(0);
        count += 1;

        let stream = Stream {
            id: count,
            sender: sender.clone(),
            recipient,
            amount,
            claimed: 0,
            start_time,
            end_time,
            rate_per_second,
        };

        env.storage().persistent().set(&count, &stream);
        env.storage().instance().set(&symbol_short!("count"), &count);

        env.events().publish((symbol_short!("stream"), sender), count);
        Ok(count)
    }

    pub fn claim(env: Env, recipient: Address, stream_id: u32) -> Result<i128, Error> {
        recipient.require_auth();

        let mut stream: Stream = env.storage().persistent().get(&stream_id).ok_or(Error::StreamNotFound)?;

        if stream.recipient != recipient {
            return Err(Error::Unauthorized);
        }

        let now = env.ledger().timestamp();
        let claimable = Self::calculate_claimable(&stream, now);

        if claimable <= 0 {
            return Err(Error::NothingToClaim);
        }

        stream.claimed += claimable;
        env.storage().persistent().set(&stream_id, &stream);

        env.events().publish((symbol_short!("claim"), recipient), claimable);
        Ok(claimable)
    }

    fn calculate_claimable(stream: &Stream, now: u64) -> i128 {
        if now <= stream.start_time {
            return 0;
        }

        let effective_time = if now >= stream.end_time {
            stream.end_time
        } else {
            now
        };

        let elapsed = effective_time - stream.start_time;
        let total_accrued = stream.rate_per_second * (elapsed as i128);
        total_accrued - stream.claimed
    }

    pub fn get_stream(env: Env, stream_id: u32) -> Result<Stream, Error> {
        env.storage().persistent().get(&stream_id).ok_or(Error::StreamNotFound)
    }
}

mod test;
