use std::fmt;

/// Custom errors for the overseer crate to help distinguish issues stemmung
/// changes in websites where scraping is used or public apis.
#[derive(Debug)]
pub enum OverseerError { 
    /// Flag when account unique id is required but not given
    MissingAccountId, 
    /// Flag when a field is expected in a fetch but cant be found.
    MissingData {dataField: String},
    /// Flag when a fetch fails completely.
    FailedFetch {url: String}
}

impl fmt::Display for OverseerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OverseerError::MissingAccountId => { 
                write!(f, "Account id is invalid!")
            },
            OverseerError::MissingData {dataField} => {
                write!(f, "Corrupt field could not fetch field '{}'", dataField)
            },
            OverseerError::FailedFetch {url} => {
                write!(f, "Failed to reach '{}'", url)
            }
        } 
    }
}
