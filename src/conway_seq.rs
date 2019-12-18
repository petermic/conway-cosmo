use std::num;
use std::io;
use num_bigint::{BigInt};
use num_traits::{Zero,pow};
use num_iter::{range};

pub fn print_bigint_list(list: Vec<BigInt>){
    let mut s: String = String::new();
    for (i,l) in list.iter().enumerate() {
        s += &l.to_str_radix(10);
        if i!=list.iter().count()-1 {
            s += ",";
        }
    }
    println!("[{}]",s);
}
    
pub fn get_number() -> BigInt {
    let mut r: Result<i64,num::ParseIntError>;
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)
            .expect("Error reading from stdin!");
        buf = buf.trim().to_string();
        r = i64::from_str_radix(&buf,10);
        match r {
            Ok(_) => {
                break;
            },               
            Err(e) => {
                println!("error: {}",e);
                continue;
            },
        }
    }
    BigInt::from(r.unwrap())
}
    
pub fn generate_conway_sequence(initial_value: BigInt, length: BigInt) -> Vec<BigInt> {
    let mut v: Vec<BigInt> = Vec::new();
    v.push(initial_value);
    for _ in range(BigInt::from(1),length) {
        let mut new_entry: Vec<BigInt> = Vec::new();
        let mut digit_lengths: Vec<BigInt> = Vec::new();
        let mut digit_ids: Vec<BigInt> = Vec::new();
        let mut current_run_digit: BigInt = BigInt::from(-1);
        let mut current_run_counter: BigInt = BigInt::from(1);
        let digits: Vec<BigInt> = split_to_digits(v.last().cloned().unwrap());
        for j in digits {
            if current_run_digit == BigInt::from(-1) {
                current_run_digit = j;
            }
            else if current_run_digit == j {
                current_run_counter = current_run_counter + 1;
            } 
            else {
                if current_run_digit >= BigInt::from(0) {
                    digit_ids.push(current_run_digit);
                    digit_lengths.push(current_run_counter);
                } else {
                    digit_ids.push(j.clone());
                    digit_lengths.push(BigInt::from(1));
                }
                current_run_digit = j;
                current_run_counter = BigInt::from(1);
            }
        }
        digit_ids.push(current_run_digit);
        digit_lengths.push(current_run_counter);
        for (i,j) in digit_lengths.iter().zip(digit_ids.iter_mut()) {
            new_entry.push(i.clone());
            new_entry.push(j.clone());
        }
        v.push(construct_from_digits(new_entry));
    }
    v
}
    
pub fn split_to_digits(n: BigInt) -> Vec<BigInt> {
    let mut v: Vec<BigInt> = Vec::new();
    let mut m = n.clone();
    if n==Zero::zero(){
        v = vec![Zero::zero()];
    }
    while m!=Zero::zero(){
        v.push(m.clone()%10);
        m = m / 10;
    }
    v.reverse();
    v
}
    
pub fn construct_from_digits(digits: Vec<BigInt>) -> BigInt {
    let mut n = Zero::zero();
    let mut stigid = digits;
    stigid.reverse();
    for (index,d) in stigid.iter().enumerate() {
        n = n + pow(BigInt::from(10),index as usize)*d;
    }
    n
}