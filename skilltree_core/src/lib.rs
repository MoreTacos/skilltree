use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    username: String,
    skills: HashMap<String, usize>,
}

pub struct Database {
    users: Tree<User>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
