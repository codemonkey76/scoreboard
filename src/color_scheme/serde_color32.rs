    use egui_multiwin::egui::Color32;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(color: &Color32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("#{:02X}{:02X}{:02X}{:02X}", color.r(), color.g(), color.b(), color.a()))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with('#') && s.len() == 9 {
            let r = u8::from_str_radix(&s[1..3], 16).map_err(serde::de::Error::custom)?;
            let g = u8::from_str_radix(&s[3..5], 16).map_err(serde::de::Error::custom)?;
            let b = u8::from_str_radix(&s[5..7], 16).map_err(serde::de::Error::custom)?;
            let a = u8::from_str_radix(&s[7..9], 16).map_err(serde::de::Error::custom)?;
            Ok(Color32::from_rgba_unmultiplied(r, g, b, a))
        } else {
            Err(serde::de::Error::custom("expected color to start with '#' and have length 9"))
        }
    }

