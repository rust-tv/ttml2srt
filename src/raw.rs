use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct P {
    pub begin: String,
    pub end: String,
    #[serde(rename = "$value")]
    pub val: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Div {
    #[serde(rename = "p")]
    pub p_vec: Vec<P>,
}

#[derive(Debug, Deserialize)]
pub struct Body {
    pub div: Div,
}

#[derive(Debug, Deserialize)]
pub struct Tt {
    pub body: Body,
}

impl Tt {
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = quick_xml::de::from_reader(&*bytes)?;
        Ok(raw)
    }

    pub fn try_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = std::fs::read(file_path)?;
        Self::try_from_bytes(&*bytes)
    }
}
