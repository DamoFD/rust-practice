#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

fn main() {
    pub struct Bank {
        balance: f32
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32){
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Current Balance: {} Withdraw a smaller amount", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdrew {} Current Balance {}", amt, bank_ref.balance);
        }
    }
    fn customer(the_bank: Arc<Mutex<Bank>>){
        withdraw(&the_bank, 5.00);
    }
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank {balance: 20.00}));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref)
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total {}", bank.lock().unwrap().balance);
}
