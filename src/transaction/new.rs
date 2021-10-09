use super::*;
use std::error::Error;
use csv::StringRecord;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct New {
    pub user: i32,
    pub symbol: String,
    pub price: usize,
    pub quantity: usize,
    pub side: Side,
    pub user_order_id: i32,
}

impl New {
    pub fn from_string_record(
        record: &StringRecord
    ) -> Result<Transaction, Box<dyn Error>> {
        if record.len() != 7 {
            return Err(Box::new(TransactionParseError::ArgCount))
        }

        if &record[0] != "N" {
            return Err(Box::new(TransactionParseError::InvalidTypeNew))
        }

        Ok(
            Transaction::New(
                New {
                    user:          record[1].trim_start().trim_end().parse::<i32>()?,
                    symbol:        record[2].trim_start().trim_end().to_string(),
                    price:         record[3].trim_start().trim_end().parse::<usize>()?,
                    quantity:      record[4].trim_start().trim_end().parse::<usize>()?,
                    side:          Side::from(&record[5].trim_start().trim_end())?,
                    user_order_id: record[6].trim_start().trim_end().parse::<i32>()?,
                }
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_valid_record() {
        let mut record = StringRecord::new();
        record.push_field("N");
        record.push_field("1");
        record.push_field("IBM");
        record.push_field("10");
        record.push_field("100");
        record.push_field("B");
        record.push_field("1");

        assert_eq!(
            New::from_string_record(&record).unwrap(),
            Transaction::New(New {
                user: 1,
                symbol: "IBM".to_string(),
                price: 10,
                quantity: 100,
                side: Side::Buy,
                user_order_id: 1,
            })
        )
    }
}

