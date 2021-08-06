use bincode::Options;
use pwhash::bcrypt;
use serde::Deserialize;
use serde::Serialize;
use sled::IVec;
use crate::core::User;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Group {
    pub name: String,
    pub groupurl: String,
    pub gymemail: String,
}

impl Group {
    pub fn new(name: &str, gymemail: &str) -> Self {
        let name: String = name.to_string();
        assert_ne!(&name, "");
        let mut groupurl: String = bcrypt::hash(&name)
            .unwrap()
            .to_lowercase()
            .chars()
            .rev()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        groupurl.truncate(7);
        let gymemail: String = gymemail.to_string();
        Group {
            name,
            groupurl,
            gymemail,
        }
    }
}

impl From<Group> for IVec {
    fn from(group: Group) -> Self {
        IVec::from(
            bincode::DefaultOptions::new()
                .with_big_endian()
                .serialize(&group)
                .unwrap(),
        )
    }
}

impl From<IVec> for Group {
    fn from(ivec: IVec) -> Self {
        bincode::DefaultOptions::new()
            .with_big_endian()
            .deserialize(&ivec[..])
            .unwrap()
    }
}
