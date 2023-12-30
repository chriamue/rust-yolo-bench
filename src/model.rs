use std::str::FromStr;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Model {
    None,
    #[cfg(feature = "tract")]
    Tract,

    #[cfg(feature = "candle")]
    Candle,
}

impl ToString for Model {
    fn to_string(&self) -> String {
        match self {
            Model::None => "None".into(),
            #[cfg(feature = "tract")]
            Model::Tract => "Tract".into(),
            #[cfg(feature = "candle")]
            Model::Candle => "Candle".into(),
        }
    }
}

impl FromStr for Model {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Model::None),
            #[cfg(feature = "tract")]
            "Tract" => Ok(Model::Tract),
            #[cfg(feature = "candle")]
            "Candle" => Ok(Model::Candle),
            _ => Err(format!("Unknown model: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_to_string() {
        assert_eq!(Model::None.to_string(), "None");
        #[cfg(feature = "tract")]
        assert_eq!(Model::Tract.to_string(), "Tract");
        #[cfg(feature = "candle")]
        assert_eq!(Model::Candle.to_string(), "Candle");
    }

    #[test]
    fn model_from_str() {
        assert_eq!(Model::from_str("None").unwrap(), Model::None);
        #[cfg(feature = "tract")]
        assert_eq!(Model::from_str("Tract").unwrap(), Model::Tract);
        #[cfg(feature = "candle")]
        assert_eq!(Model::from_str("Candle").unwrap(), Model::Candle);
    }
}
