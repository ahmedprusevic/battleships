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
    }
}
