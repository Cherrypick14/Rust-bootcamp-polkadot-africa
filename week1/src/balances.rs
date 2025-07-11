use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
   balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who:&String, amount: u128){
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who:&String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        sender: String,
        receiver: String,
        amount: u128,
        fee: u128,
    ) -> Result<(), &'static str> {
            let sender_balance = self.balance(&sender);
            let receiver_balance = self.balance(&receiver);

            // Check if sender has enough for amount + fee
            let total = amount.checked_add(fee).ok_or("Overflow")?;
            let new_sender_balance = sender_balance.checked_sub(total).ok_or("Not enough balance for amount + fee")?;
            let new_receiver_balance = receiver_balance.checked_add(amount).ok_or("Overflow")?;

            self.balances.insert(sender, new_sender_balance);
            self.balances.insert(receiver, new_receiver_balance);
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    #[test]
fn init_balances(){
    let mut balances = super::Pallet::new();

    assert_eq!(balances.balance(&"alice".to_string()), 0);
    balances.set_balance(&"alice".to_string(), 100);
    assert_eq!(balances.balance(&"alice".to_string()), 100);
    assert_eq!(balances.balance(&"bob".to_string()), 0);

}

#[test]
fn transfer_balance(){
    let mut balances = super::Pallet::new();

    // Not enough funds for amount + fee
    assert_eq!(
        balances.transfer("alice".to_string(), "bob".to_string(), 51, 2),
     Err("Not enough balance for amount + fee")
    );

    balances.set_balance(&"alice".to_string(), 100);
    // Transfer 51 with fee 2, should succeed
    assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 51, 2), 
Ok(())
);

    // Alice: 100 - 51 - 2 = 47, Bob: 51
    assert_eq!(balances.balance(&"alice".to_string()), 47);
    assert_eq!(balances.balance(&"bob".to_string()), 51);

    // Not enough for another transfer (alice has 47, needs 51+2=53)
    assert_eq!(
        balances.transfer("alice".to_string(), "bob".to_string(), 51, 2),
        Err("Not enough balance for amount + fee")
    );
   
}
}
