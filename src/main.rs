extern crate num_bigint;
extern crate num_traits;
extern crate num_iter;
mod conway_seq;
use num_bigint::{BigInt};

fn main() {
    println!("Enter first element:");
    let first_element = conway_seq::get_number();
    println!("Enter sequence length:");
    let seq_length = conway_seq::get_number();
    println!("Generating Conway sequence starting at {first} of length {length}...",
        first=first_element,
        length=seq_length);
    let conway_seq: Vec<BigInt> = conway_seq::generate_conway_sequence(first_element,seq_length);
    println!("Generated Conway sequence:");
    conway_seq::print_bigint_list(conway_seq);
}