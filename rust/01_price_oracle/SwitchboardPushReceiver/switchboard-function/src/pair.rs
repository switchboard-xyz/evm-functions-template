use serde::Deserialize;
use serde::Deserializer;

use std::hash::Hash;
pub use switchboard_utils::reqwest;

#[allow(non_snake_case)]
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pair {
    pub base: String,
    pub quote: String,
}
impl Pair {
    pub fn from_string(mut s: String) -> Self {
        s = s.to_uppercase();
        let parts: Vec<String> = s
            .split(|c| c == '-' || c == '/' || c == '_' || c == ':')
            .map(|x| x.to_string())
            .collect();
        if parts.len() == 2 {
            return Pair {
                base: parts[0].to_string(),
                quote: parts[1].to_string(),
            };
        }
        let suffixes = ["USDC", "USDT"];
        if parts.len() == 1 {
            let ends_with_any = suffixes.iter().any(|&suffix| s.ends_with(suffix));
            if ends_with_any {
                let quote = s.split_off(s.len() - 4);
                return Pair {
                    base: s.clone(),
                    quote: quote.clone(),
                };
            } else {
                let quote = s.split_off(s.len() - 3);
                return Pair {
                    base: s.clone(),
                    quote: quote.clone(),
                };
            }
        }

        if parts.len() != 2 {
            return Pair {
                base: s.clone(),
                quote: "".to_string(),
            };
        }

        Pair {
            base: parts[0].to_string(),
            quote: parts[1].to_string(),
        }
    }
}
impl<'de> Deserialize<'de> for Pair {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Pair::from_string(s))
    }
}
impl From<String> for Pair {
    fn from(s: String) -> Self {
        Pair::from_string(s)
    }
}
impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        Pair::from_string(s.to_string())
    }
}
