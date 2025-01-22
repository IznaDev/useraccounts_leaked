use std::fs::File;
use std::io::prelude::*;

use std::io::BufReader;
use std::path::Path;
use std::{collections::HashMap, str::FromStr};

//structure representing user accounts
#[derive(Debug)]
pub struct Account {
    pub login: String,
    pub password: String,
}

use crate::error::Error;

impl Account {
    //to create a new account with login and password. It is not used in this code
    #[warn(dead_code)]
    pub fn new(log: &str, pass: &str) -> Self {
        let login = log.to_string();
        let password = pass.to_string();
        Self { login, password }
    }

    //bring together in a hashmap all logins with the same password
    pub fn group(accounts: &[Account]) -> HashMap<&String, Vec<&String>> {
        let mut group = HashMap::new();

        for account in accounts {
            group
                .entry(&account.password)
                .and_modify(|v: &mut Vec<_>| v.push(&account.login))
                .or_insert(vec![&account.login]);
        }
        group
    }

    //retrieve data from file and store it into a vector
    pub fn from_file(filename: &Path) -> Result<Vec<Account>, Error> {
        let path = Path::new(filename);
        let file = File::open(path)?;
        let mut accounts = Vec::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let l = line?;
            accounts.push(Account::from_str(&l)?);
        }
        Ok(accounts)
    }
}

//implentation of trait FromStr for Account structure
impl FromStr for Account {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        if let Some((l, p)) = s.split_once(":") {
            let login = l.to_string();
            let password = p.to_string();
            Ok(Self { login, password })
        } else {
            Err(Error::NoColon)
        }
    }
}
