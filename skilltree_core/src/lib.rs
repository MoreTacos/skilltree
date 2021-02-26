use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;
use sled_extensions::bincode::Tree;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub skills: HashMap<String, usize>,
}

pub struct Database {
    pub users: Tree<User>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
