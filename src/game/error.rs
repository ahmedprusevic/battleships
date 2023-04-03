use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum ShipInputError {
        InvalidInput {
            display("The input you entered is invalid")
        }
        ShipInPath {
            display("There is ship in specified path")
        }
        NoFreeFields {
            display("There is no free fields eiter ship is on the way or all fields are out of bounds")
        }
        SameFieldError {
            display("The fields are same")
        }
    }
}
