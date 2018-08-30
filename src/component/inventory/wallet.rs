use std::ops::{Add, Sub, AddAssign, SubAssign};
use specs_derive::Component;
use game_engine::prelude::*;
use crate::model::money::Money;

#[derive(Component, Clone, Default, Debug)]
pub struct Wallet(Money);

impl Wallet {
    pub fn new(money: Money) -> Self {
        Wallet(money)
    }

    pub fn amount(&self) -> Money {
        self.0
    }

    pub fn transfer(&mut self, rhs: &mut Wallet, amount: Money) {
        *self += amount;
        *rhs -= amount;
    }
}

impl Add<Money> for Wallet {
    type Output = Wallet;
    fn add(self, rhs: Money) -> Self {
        Wallet(self.0 + rhs)
    }
}

impl AddAssign<Money> for Wallet {
    fn add_assign(&mut self, rhs: Money) {
        self.0 += rhs;
    }
}

impl Sub<Money> for Wallet {
    type Output = Wallet;
    fn sub(self, rhs: Money) -> Self {
        Wallet(self.0 + rhs)
    }
}

impl SubAssign<Money> for Wallet {
    fn sub_assign(&mut self, rhs: Money) {
        self.0 -= rhs;
    }
}
