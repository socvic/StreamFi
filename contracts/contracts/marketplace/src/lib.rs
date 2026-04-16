#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, symbol_short, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    OrderNotFound = 4,
    InvalidAmount = 5,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Order {
    pub id: u32,
    pub seller: Address,
    pub stream_id: u32,
    pub amount: i128,
    pub price: i128,
    pub filled: bool,
}

#[contract]
pub struct MarketplaceContract;

#[contractimpl]
impl MarketplaceContract {
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

    pub fn create_order(
        env: Env,
        seller: Address,
        stream_id: u32,
        amount: i128,
        price: i128,
    ) -> Result<u32, Error> {
        seller.require_auth();
        
        if amount <= 0 || price <= 0 {
            return Err(Error::InvalidAmount);
        }

        let mut count: u32 = env.storage().instance().get(&symbol_short!("count")).unwrap_or(0);
        count += 1;

        let order = Order {
            id: count,
            seller: seller.clone(),
            stream_id,
            amount,
            price,
            filled: false,
        };

        env.storage().persistent().set(&count, &order);
        env.storage().instance().set(&symbol_short!("count"), &count);
        
        env.events().publish((symbol_short!("order"), seller), count);
        Ok(count)
    }

    pub fn fill_order(env: Env, buyer: Address, order_id: u32) -> Result<(), Error> {
        buyer.require_auth();

        let mut order: Order = env.storage().persistent().get(&order_id).ok_or(Error::OrderNotFound)?;
        
        if order.filled {
            return Err(Error::InvalidAmount);
        }

        order.filled = true;
        env.storage().persistent().set(&order_id, &order);

        env.events().publish((symbol_short!("fill"), buyer), order_id);
        Ok(())
    }

    pub fn get_order(env: Env, order_id: u32) -> Result<Order, Error> {
        env.storage().persistent().get(&order_id).ok_or(Error::OrderNotFound)
    }
}

mod test;
