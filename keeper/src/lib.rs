pub mod bindings;
pub mod escalator;
pub mod keeper;
pub mod liquidator;
pub mod watcher;

use ethers::prelude::*;
// use std::collections::HashMap;

// // merge & deduplicate the 2 data structs
// pub fn merge<K: Clone + Ord, T>(a: Vec<K>, b: &HashMap<K, T>) -> Vec<K> {
//     let keys = b.keys().cloned().collect::<Vec<_>>();
//     let mut all = [a, keys].concat();
//     all.sort_unstable();
//     all.dedup();
//     all
// }

pub type Result<T, M> = std::result::Result<T, ContractError<M>>;
