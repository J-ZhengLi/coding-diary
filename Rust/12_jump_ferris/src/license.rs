use std::str::FromStr;
use anyhow::Error as AnyError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssetCredit {
    pub title: String,
    #[serde(rename = "type", default = "default_asset_type")]
    pub asset_type: AssetType,
    pub url: String,
    pub authors: Vec<String>,
    #[serde(default)]
    pub licenses: Vec<String>,
    pub license_detail: Option<String>,
}

impl FromStr for AssetCredit {
    type Err = AnyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r: Self = serde_json::from_str(s)?;
        Ok(r)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    Audio,
    Font,
    Model,
    Sprite,
    Texture,
    Other,
}

fn default_asset_type() -> AssetType {
    AssetType::Other
}

#[cfg(test)]
mod credit_tests {
    use super::{AssetCredit, AssetType};

    #[test]
    fn deserialize_credit() {
        let credit_str = r#"
        {
            "title": "asset title",
            "type": "sprite",
            "url": "http://hostname.of/asset",
            "authors": [
              "the author"
            ],
            "licenses": [ "CC" ],
            "license-detail": "optional license detail in case some assets don't have any license"
        }
        "#;
        let credit =  credit_str.parse::<AssetCredit>().unwrap();

        assert_eq!(credit.title, "asset title");
        assert_eq!(credit.url, "http://hostname.of/asset");
        assert_eq!(credit.asset_type, AssetType::Sprite);
        assert_eq!(credit.authors, ["the author"]);
        assert_eq!(credit.licenses, ["CC"]);
        assert!(credit.license_detail.is_some());
    }
}