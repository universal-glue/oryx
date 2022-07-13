use crate::schemas::{Currency, CurrencyPair};

pub struct Spot {

}

impl Spot {
    pub fn with_transport() -> Self {
        Spot {
        }
    }

    pub fn list_currencies(&self) -> Vec<Currency> {
        unimplemented!()
    }

    pub fn list_currency_pairs(&self) -> Vec<CurrencyPair> {
        vec![]
    }

    pub fn get_currency_pair(&self, currency_pair: &str) -> CurrencyPair {
        unimplemented!()
    }

}