use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum ScrapedValue {
    Value(String)
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseScrapedValueError;

impl TryInto<String> for ScrapedValue {
    
    type Error = ParseScrapedValueError;

    fn try_into(self) -> Result<String, ParseScrapedValueError> {
        let ScrapedValue::Value(value) = self;
        Ok(value)
    }
}

impl TryInto<f32> for ScrapedValue {
    
    type Error = ParseScrapedValueError;

    fn try_into(self) -> Result<f32, ParseScrapedValueError> {
        let ScrapedValue::Value(value) = self;
        let parsed_value = value.parse::<f32>();
        match parsed_value {
            Ok(x) => {
                Ok(x)
            },
            _ => {
                println!("Failed to coerce {:?} to float", value);
                Err(ParseScrapedValueError)
            }
        }
    }
}

impl FromStr for ScrapedValue {

    type Err = ParseScrapedValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed_string = s
            .trim()
            .replace("£", "")
            .replace(",","");
        Ok(ScrapedValue::Value(trimmed_string.to_string()))
    }

}
