use crate::{
    hl_client::HL,
    trading212_api::Trading212,
};

/// Enum to differentiate between vendors when using overseer traits,
pub enum Vendor {
    HL(HL),
    Trading212(Trading212)
}


