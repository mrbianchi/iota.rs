use crate::{Client, Error, Result};

use bee_signing_ext::{binary::BIP32Path, Seed};

/// Builder of get_balance API
pub struct GetBalanceBuilder<'a> {
    client: &'a Client,
    seed: &'a Seed,
    path: Option<&'a BIP32Path>,
    index: Option<usize>,
}

impl<'a> GetBalanceBuilder<'a> {
    /// Create get_balance builder
    pub fn new(client: &'a Client, seed: &'a Seed) -> Self {
        Self {
            client,
            seed,
            path: None,
            index: None,
        }
    }

    /// Set path to the builder
    pub fn path(mut self, path: &'a BIP32Path) -> Self {
        self.path = Some(path);
        self
    }

    /// Set index to the builder
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Consume the builder and get the API result
    pub fn get(self) -> Result<u64> {
        let path = match self.path {
            Some(p) => {
                if p.depth() != 2 {
                    return Err(Error::InvalidParameter(String::from(
                        "Must provide BIP32Path with depth of 2",
                    )));
                }
                p
            }
            None => return Err(Error::MissingParameter),
        };

        let mut index = match self.index {
            Some(r) => r,
            None => 0,
        };

        // get account balance and check with value
        let mut balance = 0;
        loop {
            let addresses = self
                .client
                .get_addresses(self.seed)
                .path(path)
                .range(index..index + 20)
                .get()?;

            let outputs = self.client.get_outputs().addresses(&addresses).get()?;

            let mut end = false;
            for output in outputs {
                match output.spent {
                    true => {
                        if output.amount != 0 {
                            return Err(Error::SpentAddress);
                        }
                    }
                    false => {
                        if output.amount != 0 {
                            balance += output.amount;
                        } else {
                            end = true;
                        }
                    }
                }
            }

            match end {
                true => break,
                false => index += 20,
            }
        }

        Ok(balance)
    }
}
