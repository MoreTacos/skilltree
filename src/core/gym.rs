use super::user::User;
use bincode::Options;
use pwhash::bcrypt;
use serde::Deserialize;
use serde::Serialize;
use sled::IVec;

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
        let name: String = name.to_string();
        let email: String = email.to_string();
        let users: Vec<User> = vec![];
        let pwhash: String = bcrypt::hash(pw).unwrap();
        Gym {
            name,
            email,
            pwhash,
        }
    }
}

impl From<Gym> for IVec {
    fn from(gym: Gym) -> Self {
        IVec::from(
            bincode::DefaultOptions::new()
                .with_big_endian()
                .serialize(&gym)
                .unwrap(),
        )
    }
}

impl From<IVec> for Gym {
    fn from(ivec: IVec) -> Self {
        bincode::DefaultOptions::new()
            .with_big_endian()
            .deserialize(&ivec[..])
            .unwrap()
    }
}
