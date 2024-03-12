extern crate regex;
use self::regex::Regex;

use super::datastructs::Identifier;

pub trait Validation {
    fn validate(&self) -> bool;
}


impl Validation for Identifier {
    fn validate(&self) -> bool {
        let s = self.get_value();
        let namespace_regex: Regex = Regex::new(r"[a-z0-9.-_]+").expect("Namespace regex could not be build");
        let value_regex: Regex = Regex::new(r"[a-z0-9.-_/]+").expect("Value regex could not be build");
        if s.contains(":") {
            let mut splited = s.split(":");
            let namespace = splited.next().unwrap();
            let value = splited.next().unwrap();
            let namespace_matches = namespace_regex.is_match(namespace);
            let value_matches = value_regex.is_match(value);
            return namespace_matches && value_matches;
        } else {
            return value_regex.is_match(&s);
        }
    }
}
