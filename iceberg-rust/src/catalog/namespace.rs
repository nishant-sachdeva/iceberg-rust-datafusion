/*!
Defining the [Namespace] struct for handling namespaces in the catalog.
*/

use core::fmt::{self, Display};
use itertools::Itertools;
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{catalog::identifier::SEPARATOR, error::Error};

/// Namespace struct for iceberg catalogs
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Namespace(Vec<String>);

impl Namespace {
    /// Try to create new namespace with sequence of strings.
    pub fn try_new(levels: &[String]) -> Result<Self, Error> {
        if levels.iter().any(|x| x.is_empty()) {
            Err(Error::InvalidFormat("namespace sequence".to_string()))
        } else {
            Ok(Namespace(levels.to_vec()))
        }
    }
    /// Create empty namespace
    pub fn empty() -> Self {
        Namespace(vec![])
    }
    /// Url encodes the namespace
    pub fn url_encode(&self) -> String {
        url::form_urlencoded::byte_serialize(self.0.join("\u{1F}").as_bytes()).collect()
    }
    /// Create namespace from url encoded string
    pub fn from_url_encoded(namespace: &str) -> Result<Self, Error> {
        Ok(Namespace(
            url::form_urlencoded::parse(namespace.as_bytes())
                .next()
                .ok_or(Error::InvalidFormat(format!(
                    "Namespace {} is empty",
                    namespace
                )))?
                .0
                .split("\u{1F}")
                .map(ToString::to_string)
                .collect(),
        ))
    }
}

impl Deref for Namespace {
    type Target = [String];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Itertools::intersperse(self.0.iter().map(|x| x as &str), SEPARATOR).collect::<String>()
        )
    }
}

#[cfg(test)]

mod tests {
    use super::Namespace;

    #[test]
    fn test_new() {
        let namespace = Namespace::try_new(&[
            "level1".to_string(),
            "level2".to_string(),
            "level3".to_string(),
        ])
        .unwrap();
        assert_eq!(&format!("{}", namespace), "level1.level2.level3");
    }
    #[test]
    #[should_panic]
    fn test_empty() {
        let _ = Namespace::try_new(&["".to_string(), "level2".to_string()]).unwrap();
    }
}
