/*
 * Apache Iceberg REST Catalog API
 *
 * Defines the specification for the first version of the REST Catalog API. Implementations should ideally support both Iceberg table specs v1 and v2, with priority given to v2.
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetNamespaceResponse {
    /// Reference to one or more levels of a namespace
    #[serde(rename = "namespace")]
    pub namespace: Vec<String>,
    /// Properties stored on the namespace, if supported by the server. If the server does not support namespace properties, it should return null for this field. If namespace properties are supported, but none are set, it should return an empty object.
    #[serde(
        rename = "properties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

impl GetNamespaceResponse {
    pub fn new(namespace: Vec<String>) -> GetNamespaceResponse {
        GetNamespaceResponse {
            namespace,
            properties: None,
        }
    }
}
