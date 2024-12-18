use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Acct {
    pub username: String,
    pub host: Option<String>,
}

pub fn parse(_acct: &str) -> Acct {
    let mut acct = _acct;
    if acct.starts_with('@') {
        acct = &acct[1..];
    }
    let split: Vec<&str> = acct.splitn(2, '@').collect();
    Acct {
        username: split[0].to_string(),
        host: split.get(1).map(|&s | s.to_string()),
    }
}

pub fn to_string(acct: &Acct) -> String {
    match &acct.host {
        Some(host) => format!("{}@{}", acct.username, host),
        None => acct.username.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let acct = parse("@admin@example.com");
        assert_eq!(acct.username, "admin");
    }
}