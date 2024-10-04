use web_time::Instant;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConversionResult {
    pub value: String,
    pub conversion_time: f32,
    pub size_reduced_by: f32,
}

pub fn decode(zipson: String) -> anyhow::Result<ConversionResult> {
    let start = Instant::now();
    let zipson_value = serde_zipson::de::from_str::<serde_zipson::value::Value>(&zipson)?;
    // counting only zipson parsing
    let conversion_time = start.elapsed().as_micros() as f32 / 1000.;

    let json = serde_json::to_value(zipson_value)?.to_string();

    let size_reduced_by = (1. - zipson.chars().count() as f32 / json.chars().count() as f32) * 100.;
    Ok(ConversionResult {
        value: json,
        conversion_time,
        size_reduced_by: size_reduced_by.round(),
    })
}

pub fn encode(json: String) -> anyhow::Result<ConversionResult> {
    let json_value = serde_json::from_str::<serde_json::Value>(&json)?;

    // can't use input json, because it may be not minified
    let json_len = json_value.to_string().chars().count();

    let zipson_value = serde_json::from_value::<serde_zipson::value::Value>(json_value)?;

    let start = Instant::now();
    let zipson = serde_zipson::ser::to_string(&zipson_value, true, true)?;
    // counting only zipson stringify
    let conversion_time = start.elapsed().as_micros() as f32 / 1000.;

    let size_reduced_by = (1. - zipson.chars().count() as f32 / json_len as f32) * 100.;
    Ok(ConversionResult {
        value: zipson,
        conversion_time,
        size_reduced_by: size_reduced_by.round(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        assert_eq!("\"string\"", decode(encode("\"string\"".to_string()).unwrap().value).unwrap().value);
    }

    #[test]
    fn number() {
        assert_eq!("123", decode(encode("123".to_string()).unwrap().value).unwrap().value);
    }

    #[test]
    fn bool() {
        assert_eq!("true", decode(encode("true".to_string()).unwrap().value).unwrap().value);
    }

    #[test]
    fn null() {
        assert_eq!("null", decode(encode("null".to_string()).unwrap().value).unwrap().value);
    }

    #[test]
    fn array() {
        assert_eq!("[123,\"123\"]", decode(encode("[123,\"123\"]".to_string()).unwrap().value).unwrap().value);
    }

    #[test]
    fn object() {
        assert_eq!("{\"key\":123}", decode(encode("{\"key\":123}".to_string()).unwrap().value).unwrap().value);
    }
}