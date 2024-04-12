#[derive(Debug)]
pub enum OverseerError { 
    MissingAccountId, 
    MissingData {dataField: String},
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
