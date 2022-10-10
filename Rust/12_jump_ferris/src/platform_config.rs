use std::str::FromStr;

use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlatformCfg {
    pub platforms: Vec<Platform>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Platform {
    pub id: u32,
    pub length: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub angle: f32,
    pub text: String,
}

impl FromStr for PlatformCfg {
    type Err = AnyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

impl Default for Platform {
    fn default() -> Self {
        Self {
            id: 0,
            length: 2.0,
            pos_x: 0.0,
            pos_y: 0.0,
            angle: 0.0,
            text: String::new(),
        }
    }
}

impl PlatformCfg {
    pub fn last_id(&self) -> u32 {
        self.platforms.last().map(|p| p.id).unwrap_or_default()
    }
    pub fn push(&mut self, plfm: Platform) {
        self.platforms.push(plfm);
    }
    pub fn push_default(&mut self) {
        self.push(Platform::default());
    }
    pub fn pop(&mut self) -> Option<Platform> {
        self.platforms.pop()
    }
    pub fn remove(&mut self, idx: usize) -> Platform {
        self.platforms.remove(idx)
    }
    /// Remove a platform by its id.
    ///
    /// This method is fairly expensive, as it will loop through all the platforms
    /// trying to find and delete the one with given id, then return the deleted object.
    pub fn remove_by_id(&mut self, id: u32) -> Option<Platform> {
        for (i, plfm) in self.platforms.iter().enumerate() {
            if plfm.id == id {
                return Some(self.platforms.remove(i));
            }
        }
        None
    }
    pub fn get(&self, idx: usize) -> Option<&Platform> {
        self.platforms.get(idx)
    }
    /// Find a platform by its id.
    ///
    /// This method is relatively expensive, as it loop through each objects
    /// to find a matching id.
    pub fn get_by_id(&self, id: u32) -> Option<&Platform> {
        self.platforms.iter().find(|plfm| plfm.id == id)
    }
}

#[cfg(test)]
mod platform_tests {
    use super::PlatformCfg;

    #[test]
    fn deserialize_platform_cfg() {
        let cfg_str = r#"
        {
            "platforms": [
                {
                    "id": 0,
                    "length": 10.0,
                    "pos_x": 0.0,
                    "pos_y": 0.0,
                    "angle": 0.0,
                    "text": "some text"
                },
                {
                    "id": 1,
                    "length": 2.0,
                    "pos_x": -100.0,
                    "pos_y": 20.0,
                    "angle": 0.0,
                    "text": "some other text"
                }
            ]
        }"#;

        let cfg = cfg_str.parse::<PlatformCfg>().unwrap();
        assert_eq!(cfg.platforms.len(), 2);

        let plfm = cfg.platforms.first().unwrap();
        assert_eq!(plfm.id, 0);
        assert_eq!(plfm.length, 10.0);
        assert_eq!(plfm.pos_x, 0.0);
        assert_eq!(plfm.text, "some text");
    }
}
