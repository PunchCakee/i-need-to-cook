pub struct Bank {
    pub(crate) balance: u128,
}

impl Bank {
    pub fn withdraw(&mut self, amount: u128) -> Result<(), WithdrawError> {
        if amount > self.balance {
            return Err(WithdrawError::Lowbalance {
                balance: self.balance,
                amount,
            });
        }
        self.balance -= amount;
        return Ok(());
    }

    pub fn deposit(&mut self,amount: u128){
        self.balance += amount;
    }
}

#[derive(Debug)]
pub enum WithdrawError {
    Lowbalance { balance: u128, amount: u128 },
}
