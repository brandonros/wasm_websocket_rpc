use std::error::Error;

pub fn serialize<T: serde::ser::Serialize>(input: &T) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut output: Vec<u8> = Vec::new();
    let mut serializer = rmp_serde::Serializer::new(&mut output).with_struct_map();
    input.serialize(&mut serializer)?;
    Ok(output)
}

pub fn deserialize<T: serde::de::DeserializeOwned>(input: &[u8]) -> Result<T, Box<dyn Error>> {
    rmp_serde::from_slice::<T>(input).map_err(Into::into)
}
