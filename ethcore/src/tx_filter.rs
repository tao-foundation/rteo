// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Smart contract based transaction filter.

use ethereum_types::{H256, Address};
use lru_cache::LruCache;

use client::{BlockInfo, CallContract, BlockId};
use parking_lot::Mutex;
use spec::CommonParams;
use transaction::{Action, SignedTransaction};
use hash::KECCAK_EMPTY;

use_contract!(transact_acl, "TransactAcl", "res/contracts/tx_acl.json");

const MAX_CACHE_SIZE: usize = 4096;

mod tx_permissions {
	pub const _ALL: u32 = 0xffffffff;
	pub const NONE: u32 = 0x0;
	pub const BASIC: u32 = 0b00000001;
	pub const CALL: u32 = 0b00000010;
	pub const CREATE: u32 = 0b00000100;
	pub const _PRIVATE: u32 = 0b00001000;
}

/// Connection filter that uses a contract to manage permissions.
pub struct TransactionFilter {
	contract: transact_acl::TransactAcl,
	contract_address: Address,
	permission_cache: Mutex<LruCache<(H256, Address), u32>>,
}

impl TransactionFilter {
	/// Create a new instance if address is specified in params.
	pub fn from_params(params: &CommonParams) -> Option<TransactionFilter> {
		params.transaction_permission_contract.map(|address|
			TransactionFilter {
				contract: transact_acl::TransactAcl::default(),
				contract_address: address,
				permission_cache: Mutex::new(LruCache::new(MAX_CACHE_SIZE)),
			}
		)
	}

	/// Check if transaction is allowed at given block.
	pub fn transaction_allowed<C: BlockInfo + CallContract>(&self, parent_hash: &H256, transaction: &SignedTransaction, client: &C) -> bool {
		let mut cache = self.permission_cache.lock();

		let tx_type = match transaction.action {
			Action::Create => tx_permissions::CREATE,
			Action::Call(address) => if client.code_hash(&address, BlockId::Hash(*parent_hash)).map_or(false, |c| c != KECCAK_EMPTY) {
				tx_permissions::CALL
			} else {
				tx_permissions::BASIC
			}
		};

		let sender = transaction.sender();
		let key = (*parent_hash, sender);

		if let Some(permissions) = cache.get_mut(&key) {
			return *permissions & tx_type != 0;
		}

		let contract_address = self.contract_address;
		let permissions = self.contract.functions()
			.allowed_tx_types()
			.call(sender, &|data| client.call_contract(BlockId::Hash(*parent_hash), contract_address, data))
			.map(|p| p.low_u32())
			.unwrap_or_else(|e| {
				debug!("Error callling tx permissions contract: {:?}", e);
				tx_permissions::NONE
			});

		cache.insert((*parent_hash, sender), permissions);

		trace!("Permissions required: {}, got: {}", tx_type, permissions);
		permissions & tx_type != 0
	}
}

#[cfg(test)]
mod test {
	use std::sync::Arc;
	use spec::Spec;
	use client::{BlockChainClient, Client, ClientConfig, BlockId};
	use miner::Miner;
	use ethereum_types::Address;
	use io::IoChannel;
	use ethkey::{Secret, KeyPair};
	use super::TransactionFilter;
	use transaction::{Transaction, Action};
	use tempdir::TempDir;

	/// Contract code: https://gist.github.com/arkpar/38a87cb50165b7e683585eec71acb05a
	#[test]
	fn transaction_filter() {
		let spec_data = r#"
		{
			"name": "TestNodeFilterContract",
			"engine": {
				"authorityRound": {
					"params": {
						"stepDuration": 1,
						"startStep": 2,
						"validators": {
							"contract": "0x0000000000000000000000000000000000000000"
						}
					}
				}
			},
			"params": {
				"accountStartNonce": "0x0",
				"maximumExtraDataSize": "0x20",
				"minGasLimit": "0x1388",
				"networkID" : "0x69",
				"gasLimitBoundDivisor": "0x0400",
				"transactionPermissionContract": "0x0000000000000000000000000000000000000005"
			},
			"genesis": {
				"seal": {
					"generic": "0xc180"
				},
				"difficulty": "0x20000",
				"author": "0x0000000000000000000000000000000000000000",
				"timestamp": "0x00",
				"parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
				"extraData": "0x",
				"gasLimit": "0x222222"
			},
			"accounts": {
				"0000000000000000000000000000000000000001": { "balance": "1", "builtin": { "name": "ecrecover", "pricing": { "linear": { "base": 3000, "word": 0 } } } },
				"0000000000000000000000000000000000000002": { "balance": "1", "builtin": { "name": "sha256", "pricing": { "linear": { "base": 60, "word": 12 } } } },
				"0000000000000000000000000000000000000003": { "balance": "1", "builtin": { "name": "ripemd160", "pricing": { "linear": { "base": 600, "word": 120 } } } },
				"0000000000000000000000000000000000000004": { "balance": "1", "builtin": { "name": "identity", "pricing": { "linear": { "base": 15, "word": 3 } } } },
				"0000000000000000000000000000000000000005": {
					"balance": "1",
					"constructor": "6060604052341561000f57600080fd5b5b6101868061001f6000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff168063e17512211461003e575b600080fd5b341561004957600080fd5b610075600480803573ffffffffffffffffffffffffffffffffffffffff16906020019091905050610097565b604051808263ffffffff1663ffffffff16815260200191505060405180910390f35b6000737e5f4552091a69125d5dfcb7b8c2659029395bdf8273ffffffffffffffffffffffffffffffffffffffff1614156100d75763ffffffff9050610155565b732b5ad5c4795c026514f8317c7a215e218dccd6cf8273ffffffffffffffffffffffffffffffffffffffff1614156101155760026001179050610155565b736813eb9362372eef6200f3b1dbc3f819671cba698273ffffffffffffffffffffffffffffffffffffffff1614156101505760019050610155565b600090505b9190505600a165627a7a72305820f1f21cb978925a8a92c6e30c8c81adf598adff6d1ef941cf5ed6c0ec7ad1ae3d0029"
				}
			}
		}
		"#;

		let tempdir = TempDir::new("").unwrap();
		let spec = Spec::load(&tempdir.path(), spec_data.as_bytes()).unwrap();
		let client_db = Arc::new(::kvdb_memorydb::create(::db::NUM_COLUMNS.unwrap_or(0)));

		let client = Client::new(
			ClientConfig::default(),
			&spec,
			client_db,
			Arc::new(Miner::new_for_tests(&spec, None)),
			IoChannel::disconnected(),
		).unwrap();
		let key1 = KeyPair::from_secret(Secret::from("0000000000000000000000000000000000000000000000000000000000000001")).unwrap();
		let key2 = KeyPair::from_secret(Secret::from("0000000000000000000000000000000000000000000000000000000000000002")).unwrap();
		let key3 = KeyPair::from_secret(Secret::from("0000000000000000000000000000000000000000000000000000000000000003")).unwrap();
		let key4 = KeyPair::from_secret(Secret::from("0000000000000000000000000000000000000000000000000000000000000004")).unwrap();

		let filter = TransactionFilter::from_params(spec.params()).unwrap();
		let mut basic_tx = Transaction::default();
		basic_tx.action = Action::Call(Address::from("000000000000000000000000000000000000032"));
		let create_tx = Transaction::default();
		let mut call_tx = Transaction::default();
		call_tx.action = Action::Call(Address::from("0000000000000000000000000000000000000005"));

		let genesis = client.block_hash(BlockId::Latest).unwrap();

		assert!(filter.transaction_allowed(&genesis, &basic_tx.clone().sign(key1.secret(), None), &*client));
		assert!(filter.transaction_allowed(&genesis, &create_tx.clone().sign(key1.secret(), None), &*client));
		assert!(filter.transaction_allowed(&genesis, &call_tx.clone().sign(key1.secret(), None), &*client));

		assert!(filter.transaction_allowed(&genesis, &basic_tx.clone().sign(key2.secret(), None), &*client));
		assert!(!filter.transaction_allowed(&genesis, &create_tx.clone().sign(key2.secret(), None), &*client));
		assert!(filter.transaction_allowed(&genesis, &call_tx.clone().sign(key2.secret(), None), &*client));

		assert!(filter.transaction_allowed(&genesis, &basic_tx.clone().sign(key3.secret(), None), &*client));
		assert!(!filter.transaction_allowed(&genesis, &create_tx.clone().sign(key3.secret(), None), &*client));
		assert!(!filter.transaction_allowed(&genesis, &call_tx.clone().sign(key3.secret(), None), &*client));

		assert!(!filter.transaction_allowed(&genesis, &basic_tx.clone().sign(key4.secret(), None), &*client));
		assert!(!filter.transaction_allowed(&genesis, &create_tx.clone().sign(key4.secret(), None), &*client));
		assert!(!filter.transaction_allowed(&genesis, &call_tx.clone().sign(key4.secret(), None), &*client));
	}
}

