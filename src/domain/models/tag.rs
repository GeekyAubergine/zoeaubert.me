use serde::{Deserialize, Serialize};

pub struct TagSlug(String);

impl TagSlug {
    pub fn from_string(slug: &str) -> Self {
        Self(slug.to_string())
    }

    pub fn to_tag(self) -> Tag {
        let tag = self.0;

        if tag.ends_with("-gp") {
            let mut chars = tag.chars();
            let tag = match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().chain(chars).collect(),
            };
            return Tag::from_string(&tag.replace("-gp", "GP"));
        }

        if tag == "wip-wednesday" {
            return Tag::from_string("WIPWednesday");
        }

        if tag == "tv" {
            return Tag::from_string("TV");
        }

        // UpperCase followed by LowerCase, separate with hyphen
        let re = regex::Regex::new(r"([a-z])-([a-z])").unwrap();
        let tag = re.replace_all(&tag, |caps: &regex::Captures| {
            let lower = caps.get(1).map_or("", |m| m.as_str());
            let upper = caps.get(2).map_or("", |m| m.as_str());

            format!("{}{}", lower, upper.to_uppercase())
        });

        let tag = tag
            .replace('-', "")
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().chain(chars).collect(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        let tag = tag.replace("40k", "40K");

        Tag::from_string(&tag)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tag {
    tag: String,
}

impl Tag {
    pub fn from_string(tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
        }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn slug(&self) -> String {
        // If the tag is in the known tag replacements, return the replacement
        if self.tag == "F1" {
            return "f1".to_string();
        }

        if self.tag == "F2" {
            return "f2".to_string();
        }

        if self.tag == "F3" {
            return "f3".to_string();
        }

        if self.tag == "WIPWednesday" {
            return "wip-wednesday".to_string();
        }

        if self.tag.ends_with("GP") {
            return self.tag.to_lowercase().replace("gp", "-gp");
        }

        if self.tag.to_lowercase() == "tv" {
            return "tv".to_string();
        }

        // Space is replaced with a hyphen
        let tag = self.tag.replace(' ', "-");

        // UpperCase followed by LowerCase, separate with hyphen
        let re = regex::Regex::new(r"([a-z])([A-Z])").unwrap();
        let tag = re.replace_all(&tag, |caps: &regex::Captures| {
            let lower = caps.get(1).map_or("", |m| m.as_str());
            let upper = caps.get(2).map_or("", |m| m.as_str());

            format!("{}-{}", lower, upper)
        });

        let re = regex::Regex::new(r"([A-Za-z])(\d)").unwrap();
        let tag = re.replace_all(&tag, |caps: &regex::Captures| {
            let letter = caps.get(1).map_or("", |m| m.as_str());
            let number = caps.get(2).map_or("", |m| m.as_str());

            match (letter, number) {
                (letter, number) if !letter.is_empty() && !number.is_empty() => {
                    format!("{}-{}", letter, number)
                }
                _ => "".to_string(),
            }
        });

        tag.into_owned().to_lowercase()
    }

    pub fn title(&self) -> String {
        let tag = self.tag.as_str();

        if tag == "F1" {
            return "F1".to_string();
        }

        if tag == "F2" {
            return "F2".to_string();
        }

        if tag == "F3" {
            return "F3".to_string();
        }

        let re = regex::Regex::new(r"([a-z])([A-Z])").unwrap();
        let tag = re.replace_all(tag, |caps: &regex::Captures| {
            let lower = caps.get(1).map_or("", |m| m.as_str());
            let upper = caps.get(2).map_or("", |m| m.as_str());

            format!("{} {}", lower, upper)
        });

        let re = regex::Regex::new(r"([A-Za-z])(\d)").unwrap();
        let tag = re.replace_all(&tag, |caps: &regex::Captures| {
            let letter = caps.get(1).map_or("", |m| m.as_str());
            let number = caps.get(2).map_or("", |m| m.as_str());

            match (letter, number) {
                (letter, number) if !letter.is_empty() && !number.is_empty() => {
                    format!("{} {}", letter, number)
                }
                _ => "".to_string(),
            }
        });

        tag.into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(Tag::from_string("F1").slug(), "f1");
        assert_eq!(Tag::from_string("F2").slug(), "f2");
        assert_eq!(Tag::from_string("F3").slug(), "f3");
        assert_eq!(Tag::from_string("WIPWednesday").slug(), "wip-wednesday");
        assert_eq!(Tag::from_string("TestGP").slug(), "test-gp");
        assert_eq!(Tag::from_string("TV").slug(), "tv");
        assert_eq!(Tag::from_string("Warhammer40K").slug(), "warhammer-40k");
        assert_eq!(Tag::from_string("SpaceMarines").slug(), "space-marines");
        assert_eq!(Tag::from_string("AgeOfSigmar").slug(), "age-of-sigmar");
    }

    #[test]
    fn test_unslugify() {
        assert_eq!(TagSlug::from_string("f1").to_tag().tag(), "F1");
        assert_eq!(TagSlug::from_string("f2").to_tag().tag(), "F2");
        assert_eq!(TagSlug::from_string("f3").to_tag().tag(), "F3");
        assert_eq!(
            TagSlug::from_string("wip-wednesday").to_tag().tag(),
            "WIPWednesday"
        );
        assert_eq!(TagSlug::from_string("test-gp").to_tag().tag(), "TestGP");
        assert_eq!(TagSlug::from_string("tv").to_tag().tag(), "TV");
        assert_eq!(
            TagSlug::from_string("warhammer-40k").to_tag().tag(),
            "Warhammer40K"
        );
        assert_eq!(
            TagSlug::from_string("space-marines").to_tag().tag(),
            "SpaceMarines"
        );
        assert_eq!(
            TagSlug::from_string("age-of-sigmar").to_tag().tag(),
            "AgeOfSigmar"
        );
    }

    #[test]
    fn test_title() {
        assert_eq!(Tag::from_string("F1").title(), "F1");
        assert_eq!(Tag::from_string("F2").title(), "F2");
        assert_eq!(Tag::from_string("F3").title(), "F3");
        assert_eq!(Tag::from_string("WIPWednesday").title(), "WIPWednesday");
        assert_eq!(Tag::from_string("TestGP").title(), "Test GP");
        assert_eq!(Tag::from_string("TV").title(), "TV");
        assert_eq!(Tag::from_string("Warhammer40K").title(), "Warhammer 40K");
        assert_eq!(Tag::from_string("SpaceMarines").title(), "Space Marines");
        assert_eq!(Tag::from_string("AgeOfSigmar").title(), "Age Of Sigmar");
    }
}
