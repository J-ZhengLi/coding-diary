#![allow(dead_code)]

use std::{str::FromStr, fmt::Display};

#[derive(Debug, Clone)]
pub struct HtmlParseError;

impl Display for HtmlParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fail to parse given data into Html type.")
    }
}

pub struct Html {
    head: HtmlHead,
    body: Vec<HtmlItem>,
}

pub struct HtmlHead {
    title: HtmlItem,
    style: Option<HtmlItem>,
    base: Option<HtmlItem>,
    link: Option<HtmlItem>,
    meta: Option<HtmlItem>,
    script: Option<HtmlItem>,
    noscript: Option<HtmlItem>,
}

pub struct HtmlItem {
    tag: String,
    attrs: Vec<Attribute>,
    content: Vec<HtmlItem>,
}

pub struct Attribute {
    name: String,
    value: String,
}

impl Html {
    pub fn new() -> Html {
        Html {
            head: HtmlHead::new(),
            body: Vec::new(),
        }
    }

    pub fn is_blank(&self) -> bool {
        self.body.is_empty()
    }
}

impl FromStr for Html {
    type Err = HtmlParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for ch in s.trim().chars() {

        }
        
        Ok(Html::new())
    }
}

impl HtmlHead {
    pub fn new() -> HtmlHead {
        HtmlHead {
            title: HtmlItem::init_with_tag("title"),
            style: None,
            base: None,
            link: None,
            meta: None,
            script: None,
            noscript: None
        }
    }
}

impl HtmlItem {
    pub fn new() -> HtmlItem {
        HtmlItem {
            tag: String::new(),
            attrs: Vec::new(),
            content: Vec::new()
        }
    }

    pub fn init_with_tag(tag: &str) -> HtmlItem {
        HtmlItem {
            tag: String::from(tag),
            ..Default::default()
        }
    }
}

impl Default for HtmlItem {
    fn default() -> Self {
        HtmlItem::new()
    }
}