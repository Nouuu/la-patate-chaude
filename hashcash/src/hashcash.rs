use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::thread;

use crate::config::{NTHREADS, THREAD_SEED_ATTRIBUTION};
use crate::dto::MD5HashCashOutput;
use crate::utils::check_hash;

pub struct Hashcash;

impl Hashcash {
    pub fn solve(message: String, complexity: u32) -> MD5HashCashOutput {
        let seed_counter = Arc::new(AtomicU64::new(0));
        let is_solved = Arc::new(AtomicBool::new(false));
        let (worker_tx, worker_rx) = mpsc::channel();
        for _ in 0..NTHREADS {
            let worker_tx = worker_tx.clone();
            let seed_counter = seed_counter.clone();
            let is_solved = is_solved.clone();
            let message = message.clone();
            thread::spawn(move || {
                'outer: loop {
                    let seed = seed_counter.fetch_add(THREAD_SEED_ATTRIBUTION, Ordering::Relaxed);
                    for seed in seed..seed + THREAD_SEED_ATTRIBUTION {
                        if is_solved.load(Ordering::Relaxed) {
                            break 'outer;
                        }
                        let hash = md5::compute(format!("{:016X}", seed) + &message);
                        let md5 = format!("{:032X}", hash);
                        if !check_hash(complexity, md5.clone()) {
                            continue;
                        }
                        worker_tx.send(MD5HashCashOutput { seed, hashcode: md5.to_string() }).expect("Error while sending answer to main thread");
                        is_solved.store(true, Ordering::Relaxed);
                    }
                }
            });
        }
        let workers_result = worker_rx.recv();
        if workers_result.is_err() {
            panic!("error");
        }
        workers_result.unwrap()
    }

    pub fn verify(hash: String, complexity: u32) -> bool {
        check_hash(complexity, hash)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::check_hash;

    use super::*;

    #[test]
    fn test_hashcash() {
        let message = "hello world".to_string();
        let complexity = 5;
        let output = Hashcash::solve(message.clone(), complexity);
        assert!(check_hash(complexity, output.hashcode));
    }

    #[test]
    fn test_hashcash_with_long_string() {
        let message = "lorem ipsum dolor sit atme les fronti7ere des regions ont bien Changéeeeeqf".to_string();
        let complexity = 5;
        let output = Hashcash::solve(message.clone(), complexity);
        assert!(check_hash(complexity, output.hashcode));
    }

    #[test]
    fn test_hashcash_with_high_complexity() {
        let message = "Bonjour monde".to_string();
        let complexity = 14;
        let output = Hashcash::solve(message.clone(), complexity);
        assert!(check_hash(complexity, output.hashcode));
    }
}