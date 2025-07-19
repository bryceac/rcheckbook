use std::fmt;

pub enum ImportError {
    TransactionTypeParsingError
}

impl ImportError {
    pub fn to_string(&self) -> String {
        match self {
            Self::TransactionTypeParsingError => "Cannot determine transaction type. Please make sure there is a value in only either the credit or withdrawal column".to_owned()
        }
    }
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}