use std::{collections::BTreeMap, sync::LazyLock};

use regex::Regex;

static DATA: &[u8] = include_bytes!("../assets/emoji.json");
static TABLE: LazyLock<BTreeMap<&str, &str>> =
    LazyLock::new(|| serde_json::from_slice::<BTreeMap<&str, &str>>(DATA).unwrap());
static RE_EMOJI: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(:[a-zA-Z0-9\-_+]+:)").unwrap());

pub trait Emojify {
    fn emojify(&self) -> String;
}

impl<T> Emojify for T
where
    T: AsRef<str>,
{
    fn emojify(&self) -> String {
        let s = self.as_ref();
        let mut new_text = String::with_capacity(s.len());
        let mut last = 0;

        for cap in RE_EMOJI.captures_iter(s) {
            if let Some(m) = cap.get(0) {
                if let Some(emoji) = TABLE.get(m.as_str()) {
                    new_text.push_str(&s[last..m.start()]);
                    new_text.push_str(emoji);
                    last = m.end();
                }
            }
        }

        new_text.push_str(&s[last..]);
        new_text
    }
}
