use bincode::Options;
use pwhash::bcrypt;
use serde::Deserialize;
use serde::Serialize;
use sled::IVec;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub name: String,
    pub userurl: String,
    pub gymemail: String,
    pub groupurl: String,
    pub packageurl: String,
    pub skills: HashMap<String, usize>,
}

impl User {
    pub fn new(name: &str, gymemail: &str, groupurl: &str,  packageurl: &str, skills: HashMap<String, usize>) -> Self {
        let name = name.to_string();
        let gymemail = gymemail.to_string();
        let groupurl = groupurl.to_string();
        let packageurl = packageurl.to_string();
        let mut userurl: String = bcrypt::hash(&name)
            .unwrap()
            .to_lowercase()
            .chars()
            .rev()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        userurl.truncate(7);
        let skills = skills;
        User {
            name,
            userurl,
            gymemail,
            groupurl,
            packageurl,
            skills,
        }
    }
}

impl From<User> for IVec {
    fn from(user: User) -> Self {
        IVec::from(
            bincode::DefaultOptions::new()
                .with_big_endian()
                .serialize(&user)
                .unwrap(),
        )
    }
}

impl From<IVec> for User {
    fn from(ivec: IVec) -> Self {
        bincode::DefaultOptions::new()
            .with_big_endian()
            .deserialize(&ivec[..])
            .unwrap()
    }
}
