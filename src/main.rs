use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
enum TokenError {
    InsufficientBalance,
    TransferLocked,
    NotAuthorized,
    InvalidOperation,
}

#[derive(Debug, Clone)]
struct Token {
    total_supply: u64,
    decimals: u8,
    locked_until: Option<u64>, // Unix timestamp
    owner: String,
    balances: HashMap<String, u64>,
    allowed: HashMap<String, u64>,
}

impl Token {
    fn new(total_supply: u64, decimals: u8, owner: String) -> Self {
        let mut balances = HashMap::new();
        balances.insert(owner.clone(), total_supply);

        Token {
            total_supply,
            decimals,
            locked_until: None,
            owner,
            balances,
            allowed: HashMap::new(),
        }
    }

    fn mint(&mut self, amount: u64, caller: String) -> Result<(), TokenError> {
        if self.owner != caller {
            return Err(TokenError::NotAuthorized);
        }

        // Check if the contract is locked
        if let Some(lock_time) = self.locked_until {
            if get_current_timestamp() < lock_time {
                return Err(TokenError::TransferLocked);
            }
        }

        self.total_supply += amount;
        *self.balances.entry(self.owner.clone()).or_insert(0) += amount;
        Ok(())
    }

    fn change_owner(&mut self, new_owner: String, caller: String) -> Result<(), TokenError> {
        if self.owner != caller {
            return Err(TokenError::NotAuthorized);
        }
        self.owner = new_owner;
        Ok(())
    }

    fn change_decimals(&mut self, new_decimals: u8, caller: String) -> Result<(), TokenError> {
        if self.owner != caller {
            return Err(TokenError::NotAuthorized);
        }
        self.decimals = new_decimals;
        Ok(())
    }

    // Implement a transfer function as an example to use other variants
    fn transfer(&mut self, to: String, amount: u64, caller: String) -> Result<(), TokenError> {
        if let Some(balance) = self.balances.get_mut(&caller) {
            if *balance < amount {
                return Err(TokenError::InsufficientBalance);
            }
            *balance -= amount;
            *self.balances.entry(to).or_insert(0) += amount;
            Ok(())
        } else {
            Err(TokenError::InvalidOperation)
        }
    }
}

fn get_current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn main() {
    let mut my_token = Token::new(1000000, 2, "owner_address".to_string());

    // Minting successful
    match my_token.mint(500, "owner_address".to_string()) {
        Ok(_) => println!("Minting successful"),
        Err(e) => println!("Minting failed: {:?}", e),
    }

    // Ownership changed successfully
    match my_token.change_owner("new_owner_address".to_string(), "owner_address".to_string()) {
        Ok(_) => println!("Ownership changed successfully"),
        Err(e) => println!("Failed to change ownership: {:?}", e),
    }

    // Failed to change decimals: NotAuthorized
    match my_token.change_decimals(4, "owner_address".to_string()) {
        Ok(_) => println!("Decimals changed successfully"),
        Err(e) => println!("Failed to change decimals: {:?}", e),
    }

    // Simulate a successful transfer
    match my_token.transfer("recipient_address".to_string(), 100, "owner_address".to_string()) {
        Ok(_) => println!("Transfer successful"),
        Err(e) => println!("Transfer failed: {:?}", e),
    }
}
