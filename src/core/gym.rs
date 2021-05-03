use super::user::User;
use pwhash::bcrypt;
use serde::Deserialize;
use serde::Serialize;

// make sure that when removing users, they are not athletes of
// anybody else in the gym! If so then remove pointer
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Gym {
    pub name: String,
    pub email: String,
    pub pwhash: String,
}

impl Gym {
    pub fn new(name: &str, email: &str, pw: &str) -> Self {
        let name = name.to_string();
        let email = email.to_string();
        let users: Vec<User> = vec![];
        let pwhash = bcrypt::hash(pw).unwrap();
        Gym {
            name,
            email,
            pwhash,
        }
    }
}
