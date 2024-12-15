use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Video {
    pub title: Cow<'static, str>, // Usamos Cow<str> para title
    pub url: Cow<'static, str>,   // Usamos Cow<str> para url
}

pub struct Page {}
