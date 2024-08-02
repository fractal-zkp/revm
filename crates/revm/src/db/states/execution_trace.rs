use revm_interpreter::primitives::{Address, HashMap, HashSet, B256, U256};

/// A struct that is used to trace the data which has been accessed from the database.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExecutionTrace {
    /// Accessed accounts and associated storage slots.
    pub accounts: HashMap<Address, HashSet<U256>>,
    /// Accessed codes.
    pub codes: HashSet<B256>,
    /// Block numbers for which block hashes have been accessed.
    pub block_hashes: HashSet<u64>,
}

impl ExecutionTrace {
    /// Add accessed account.
    pub fn add_account(&mut self, address: Address) {
        self.accounts.entry(address).or_default();
    }

    /// Add accessed storage slot.
    pub fn add_storage(&mut self, address: &Address, slot: U256) {
        if let Some(storage) = self.accounts.get_mut(address) {
            storage.insert(slot);
        } else {
            unreachable!("Account is guaranteed to be loaded before storage is accessed.");
        }
    }

    /// Add accessed code.
    pub fn add_code(&mut self, hash: B256) {
        self.codes.insert(hash);
    }

    /// Add accessed block hash.
    pub fn add_block_num(&mut self, number: u64) {
        self.block_hashes.insert(number);
    }

    /// Return execution trace and leave the remaining trace empty.
    pub fn take(&mut self) -> ExecutionTrace {
        core::mem::take(self)
    }
}
