#[derive(Debug, Clone)]
pub enum ScrapedValue {
    Str(String),
    Float(f32)
}

#[derive(Debug, PartialEq, Eq)]
struct ParseScrapedValueError;

impl TryInto<String> for ScrapedValue {
    
    type Error = ParseScrapedValueError;

    fn try_into(self) -> Result<String, ParseScrapedValueError> {
        match self {
            ScrapedValue::Str(x) => {
                Ok(x)
            },
            _ => {
                println!("Failed to coerce {:?} to string", self);
                Err(ParseScrapedValueError)
            }
        }
    }
}

impl TryInto<f32> for ScrapedValue {
    
    type Error = ParseScrapedValueError;

    fn try_into(self) -> Result<f32, ParseScrapedValueError> {
        match self {
            ScrapedValue::Float(x) => {
                Ok(x)
            },
            _ => {
                println!("Failed to coerce {:?} to float", self);
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
        if let Ok(float) = trimmed_string.parse::<f32>() {
            Ok(ScrapedValue::Float(float))
        } else {
            Ok(ScrapedValue::Str(trimmed_string.to_string()))
        }
    }

}
