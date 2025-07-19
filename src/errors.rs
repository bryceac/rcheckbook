use std::fmt;

pub enum ImportError {
    InvalidDateFormat,
    TransactionTypeParsingError
}

impl ImportError {
    pub fn to_string(&self) -> String {
        match self {
            Self::InvalidDateFormat => "Date must be in the YYYY-MM-DD format.".to_owned(),
            Self::TransactionTypeParsingError => "Cannot determine transaction type. Please make sure there is a value in only either the credit or withdrawal column".to_owned()
        }
    }
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}