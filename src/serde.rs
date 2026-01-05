//! A collection of serde helpers.

pub(crate) mod space_separated_strings_as_vec {
    use serde::{de::Deserializer, ser::Serializer};

    pub(crate) fn serialize<S>(
        value: &Option<Vec<String>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(vec) if !vec.is_empty() => {
                let s = vec.join(" ");
                serializer.serialize_some(&s)
            }
            _ => serializer.serialize_none(),
        }
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = serde::Deserialize::deserialize(deserializer)?;
        Ok(s.map(|s| s.split_whitespace().map(String::from).collect()))
    }
}
