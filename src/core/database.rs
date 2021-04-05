use super::gym::Gym;
use sled_extensions::bincode::Tree;

pub struct Database {
    pub gyms: Tree<Gym>,
}
