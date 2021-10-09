use super::*;
use csv::StringRecord;
use std::error::Error;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Cancel {
    pub user: i32,
    pub user_order_id: i32,
}

impl Cancel {
    pub fn from_string_record(
        record: &StringRecord
    ) -> Result<Transaction, Box<dyn Error>> {
        if record.len() != 3 {
            return Err(Box::new(TransactionParseError::ArgCount))
        }

        if &record[0] != "C" {
            return Err(Box::new(TransactionParseError::InvalidTypeCancel))
        }

        Ok(Transaction::Cancel(
            Cancel {
                user: record[1].trim_start().trim_end().parse::<i32>()?,
                user_order_id: record[2].trim_start().trim_end().parse::<i32>()?,
            }
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_valid_record() {
        let mut record = StringRecord::new();
        record.push_field("C");
        record.push_field("1");
        record.push_field("10");

        assert_eq!(
            Cancel::from_string_record(&record).unwrap(),
            Transaction::Cancel(Cancel {
                user: 1,
                user_order_id: 10,
            })
        )
    }
}
