use custom_error::custom_error;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

custom_error!{ pub SideError
    Parse{s: String} = "String '{s}' cannot be converted to Side enum. Must be 'S', 'B', 'Sell'",
}

impl Side {
    pub fn from(s: &str) -> Result<Side, SideError> {
        match s {
            "B" => Ok(Side::Buy),
            "S" => Ok(Side::Sell),
            "Buy" => Ok(Side::Buy),
            "Sell" => Ok(Side::Sell),
            _ => Err(SideError::Parse { s: s.to_string() }),
        }
    }
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Side::Buy => write!(f, "Buy"),
            Side::Sell => write!(f, "Sell"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_s() {
        assert_eq!(
            Side::from("S").unwrap(),
            Side::Sell,
        )
    }

    #[test]
    fn from_b() {
        assert_eq!(
            Side::from("B").unwrap(),
            Side::Buy,
        )
    }

    #[test]
    fn from_sell() {
        assert_eq!(
            Side::from("Sell").unwrap(),
            Side::Sell,
        )
    }

    #[test]
    fn from_buy() {
        assert_eq!(
            Side::from("Buy").unwrap(),
            Side::Buy,
        )
    }

    #[test]
    fn from_invalid() {
        assert!(
            Side::from("bad").is_err()
        )
    }
}
