use std::{collections::HashMap, iter, mem};

use crate::common::DisjointSet;

pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let accounts = accounts
        .into_iter()
        .map(|account| Account {
            name: account[0].clone(),
            emails: account[1..].to_vec(),
            combined: false,
        })
        .collect::<Vec<_>>();

    let mut emails = HashMap::<String, usize>::new();
    let mut djs = DisjointSet::new(&vec![(); accounts.len()], |a, _| a);

    for (i, a) in accounts.iter().enumerate() {
        for e in &a.emails {
            if let Some(j) = emails.get(e) {
                djs.union(i, *j);
            } else {
                emails.insert(e.clone(), i);
            }
        }
    }
    let mut accounts = accounts;
    for i in 0..accounts.len() {
        let root = djs.find(i);
        if root == i {
            continue;
        }
        if root > i {
            let mut name = String::default();
            mem::swap(&mut name, &mut accounts[root].name);
            mem::swap(&mut name, &mut accounts[i].name);
            mem::swap(&mut name, &mut accounts[root].name);
        }
        let emails = mem::take(&mut accounts[i].emails);
        accounts[root].emails.extend(emails);
        accounts[i].combined = true;
    }

    accounts
        .into_iter()
        .filter(|a| !a.combined)
        .map(|a| {
            let mut a = a;
            a.emails.sort();
            a.emails.dedup();
            iter::once(a.name).chain(a.emails).collect()
        })
        .collect()
}

struct Account {
    name: String,
    emails: Vec<String>,
    combined: bool,
}
