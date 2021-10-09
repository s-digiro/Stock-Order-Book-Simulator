use csv::StringRecord;
use custom_error::custom_error;
use std::error::Error;

mod side;
mod new;
mod cancel;

pub use side::Side;
pub use new::New;
use cancel::Cancel;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Transaction {
    New(New),
    Cancel(Cancel),
    Flush,
}

custom_error!{ TransactionParseError
    ArgCount = "Invalid number of arguments",
    InvalidType = "Invalid transaction type. First item in CSV must be 'N', 'C', or 'F'.",
    InvalidTypeNew = "'New' transaction must have 'N' as first item in CSV.",
    InvalidTypeCancel = "'Cancel' transaction must have 'C' as first item in CSV.",
    InvalidTypeFlush = "'Flush' transaction must have 'F' as first item in CSV.",
}

impl Transaction {
    pub fn from_string_record(
        record: &StringRecord
    ) -> Result<Transaction, Box<dyn Error>> {
        if record.len() == 0 {
            return Err(Box::new(TransactionParseError::ArgCount))
        }

        match &record[0] {
            "N" => New::from_string_record(record),
            "C" => Cancel::from_string_record(record),
            "F" => {
                if record.len() != 1 {
                    return Err(Box::new(TransactionParseError::ArgCount))
                }

                if &record[0] != "F" {
                    return Err(Box::new(TransactionParseError::InvalidTypeFlush))
                }

                Ok(Transaction::Flush)
            },
            _ => Err(Box::new(TransactionParseError::InvalidType)),
        }
    }
}
