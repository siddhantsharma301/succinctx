//! An example of a basic circuit function which wraps an existing circuit and makes it compatible
//! with a standard for serializing and deserializing inputs and outputs.
//!
//! To build the binary:
//!
//!     `cargo build --release --bin circuit_function_field`
//!
//! To build the circuit:
//!
//!     `./target/release/circuit_function_field build`
//!
//! To prove the circuit using evm io:
//!
//!    `./target/release/circuit_function_evm prove --input-json src/bin/circuit_function_evm_input.json`
//!
//! Note that this circuit will not work with evm-based io.

use std::env;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::AlgebraicHasher;
use plonky2x::circuit::Circuit;
use plonky2x::function::CircuitFunction;
use plonky2x::prelude::CircuitBuilder;
use plonky2x::vars::ByteVariable;

struct Function {}

impl CircuitFunction for Function {
    fn build<F, C, const D: usize>() -> Circuit<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: plonky2::plonk::config::GenericConfig<D, F = F> + 'static,
        <C as plonky2::plonk::config::GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let mut builder = CircuitBuilder::<F, D>::new();
        let a = builder.evm_read::<ByteVariable>();
        let b = builder.evm_read::<ByteVariable>();
        let c = builder.xor(a, b);
        builder.evm_write(c);
        builder.build::<C>()
    }
}

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    Function::run();
}
