use crate::game::ship::{Ship, ShipName};

pub const FLEET_SIZE: usize = 5;

pub struct Fleet {
    pub ships: [Ship; FLEET_SIZE],
}

impl Fleet {
    pub fn new() -> Fleet {
        let fleet: [Ship; FLEET_SIZE] = [
            Ship::new(5, ShipName::Hydra),
            Ship::new(4, ShipName::Kraken),
            Ship::new(4, ShipName::Medusa),
            Ship::new(3, ShipName::Phoenix),
            Ship::new(2, ShipName::Poseidon),
        ];

        Fleet { ships: fleet }
    }
}
