#![cfg_attr(not(feature = "std"), no_std)]
use hash::rpo::RpoDigest;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen_test::console_log;
// extern crate console_error_panic_hook;

#[cfg(not(feature = "std"))]
#[cfg_attr(test, macro_use)]
extern crate alloc;

pub mod hash;
pub mod merkle;
pub mod utils;
use sp_std::{vec, vec::Vec};

// RE-EXPORTS
// ================================================================================================

pub use winter_crypto::{RandomCoin, RandomCoinError};
pub use winter_math::{fields::f64::BaseElement as Felt, FieldElement, StarkField};

// TYPE ALIASES
// ================================================================================================

/// A group of four field elements in the Miden base field.
pub type Word = [Felt; WORD_SIZE];

// CONSTANTS
// ================================================================================================

/// Number of field elements in a word.
pub const WORD_SIZE: usize = 4;

/// Field element representing ZERO in the Miden base filed.
pub const ZERO: Felt = Felt::ZERO;

/// Field element representing ONE in the Miden base filed.
pub const ONE: Felt = Felt::ONE;

// TESTS
// ================================================================================================

// The function rescue prime optimezed function's inputs should be uint64
// #[wasm_bindgen]
pub fn RescuePrimeOptimized(values: Vec<u64>) -> Vec<u64> {
    let mut values_in_u64 = values;
    if values_in_u64.len() < 8 {
        values_in_u64.push(1);
        while values_in_u64.len() < 8 {
            values_in_u64.push(0);
        }
    } else if values_in_u64.len() % 4 == 3 {
        values_in_u64.push(1);
    } else if values_in_u64.len() % 4 == 0 {
    } else {
        values_in_u64.push(1);
        while values_in_u64.len() % 4 != 0 {
            values_in_u64.push(0);
        }
    }

    assert!(
    (values_in_u64.len()== 8) || ( (values_in_u64.len() > 8) && (values_in_u64.len() % 4 == 0)),
    "expected len of values_in_u64 to be [exactly 8] or [over 8 but should be some multiple of 4] but received {}",
    values_in_u64.len()
    );

    let mut elements = from_vec(values_in_u64);
    let hash_times = if elements.len() == 8 { 1 } else { elements.len() / 4 - 1 };

    let result: RpoDigest;
    if hash_times == 1 {
        result = hash::rpo::Rpo256::hash_elements(&elements);
    } else {
        let first: Vec<Felt> = elements.drain(elements.len() - 8..).collect();
        let mut to_be_hash: RpoDigest = hash::rpo::Rpo256::hash_elements(&first);
        for _i in 1..hash_times {
            let mut a = to_be_hash.to_vec();
            let mut drain_4_element: Vec<Felt> = elements.drain(elements.len() - 4..).collect();
            drain_4_element.append(&mut a);
            to_be_hash = hash::rpo::Rpo256::hash_elements(&drain_4_element);
        }
        result = to_be_hash;
    };

    return as_u64(result).to_vec();
}

/// HELPER
pub fn from_vec(origin: Vec<u64>) -> Vec<Felt> {
    let mut res: Vec<Felt> = Vec::new();
    for i in 0..origin.len() {
        res.push(Felt::new(origin[i as usize]))
    }
    let result = res.try_into().unwrap();
    return result;
}

pub fn as_u64(origin: RpoDigest) -> [u64; 4] {
    let mut result = [0; 4];
    result[..1].copy_from_slice(&[origin[0].as_int()]);
    result[1..2].copy_from_slice(&[origin[1].as_int()]);
    result[2..3].copy_from_slice(&[origin[2].as_int()]);
    result[3..4].copy_from_slice(&[origin[3].as_int()]);
    result
}

// #[wasm_bindgen]
// pub fn init_panic_hook() {
//     console_error_panic_hook::set_once();
// }
