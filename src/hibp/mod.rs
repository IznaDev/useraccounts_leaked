use crate::account::Account;
use crate::error::Error;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use reqwest;
use sha1::{Digest, Sha1};
use std::collections::HashMap;

//this function hash an account
fn sha1(account: &Account) -> (String, String) {
    let password_hash = format!("{:X}", Sha1::digest(account.password.as_str()));

    let prefix_password_hash = &password_hash[..5];
    let suffix_password_hash = &password_hash[5..];
    (
        prefix_password_hash.to_string(),
        suffix_password_hash.to_string(),
    )
}

//Parallel hashing of all acounts
fn allsha1(accounts: &[Account]) -> Vec<(String, String, &Account)> {
    let hash_accounts = accounts
        .par_iter()
        .map(|account| {
            let (prefix_hash, suffix_hash) = sha1(account);
            (prefix_hash, suffix_hash, account)
        })
        .collect();

    hash_accounts
}

pub fn sha1_by_prefix(accounts: &[Account]) -> HashMap<String, Vec<(String, &Account)>> {
    let hash_accounts = allsha1(accounts);
    let mut prefix_hash_accounts = HashMap::new();
    for (prefix_hash, suffix_hash, account) in hash_accounts {
        prefix_hash_accounts
            .entry(prefix_hash)
            .and_modify(|v: &mut Vec<_>| v.push((suffix_hash.clone(), account)))
            .or_insert(vec![(suffix_hash.clone(), account)]);
    }
    prefix_hash_accounts
}

//connect to the pwnedpasswords API and retrieve all hashing suffix and number
fn get_page(prefix: &str) -> Result<Vec<String>, Error> {
    let mut url = "https://api.pwnedpasswords.com/range/".to_string();
    url += prefix;
    let body = reqwest::blocking::get(url)?;
    let response = body.text()?;
    let v = response.lines().map(String::from).collect();
    Ok(v)
}

//store all hashing suffix with the same hashing prefix in a HashMap
fn get_suffixes(prefix: &str) -> Result<HashMap<String, u64>, Error> {
    let mut map_suffix: HashMap<String, u64> = HashMap::new();
    let lines = get_page(prefix)?;
    for line in lines {
        if let Some((suffix, num_str)) = line.split_once(':') {
            match num_str.parse::<u64>() {
                Ok(number) => {
                    map_suffix.insert(suffix.to_string(), number);
                }
                Err(e) => {
                    return Err(Error::ParseIntError(e));
                }
            }
        } else {
            return Err(Error::NoColon);
        }
    }
    Ok(map_suffix)
}

// check if the accounts in my file are leaked or not. th computing take some time so I add a progress bar
pub fn check_accounts(accounts: &[Account]) -> Result<Vec<(&Account, u64)>, Error> {
    let prefix_map = sha1_by_prefix(accounts);

    let progress_bar = ProgressBar::new(prefix_map.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar} {pos}/{len} {msg}")
            .expect("Invalid template")
            .progress_chars("█░"),
    );

    let mut vec_accounts = Vec::new();
    for (p, vec) in prefix_map {
        progress_bar.set_message(format!("Processing prefix: {}", p));
        let map_suffix = match get_suffixes(p.as_str()) {
            Ok(map) => map,
            Err(e) => return Err(e),
        };
        for (suff, account) in vec {
            if let Some(suffixe_number) = map_suffix.get(&suff) {
                vec_accounts.push((account, *suffixe_number));
            } else {
                vec_accounts.push((account, 0));
            }
        }
        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("Processing complete!");
    vec_accounts.sort_unstable_by_key(|(_, occurrences)| std::cmp::Reverse(*occurrences));
    Ok(vec_accounts)
}
