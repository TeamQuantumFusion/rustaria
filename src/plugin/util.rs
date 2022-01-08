use std::collections::HashSet;

pub enum Filter<T> {
    All,
    None,
    Whitelist(HashSet<T>),
    Blacklist(HashSet<T>),
}