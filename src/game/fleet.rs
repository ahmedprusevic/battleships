use crate::game::ship::Ship;

const FLEET_SIZE: u8 = 5;

pub struct Fleet {
    size: u8,
    pub ships: [Ship; FLEET_SIZE as usize],
}

impl Fleet {
    pub fn new(fleet: [Ship; FLEET_SIZE as usize]) -> Fleet {
        Fleet {
            size: FLEET_SIZE,
            ships: fleet,
        }
    }
}
