use std::{ collections::HashMap, fmt::Display, ops::Deref, str::FromStr };
use native_db::{ Key, ToKey };
use serde::{ Deserialize, Serialize };
use serde_json::{ Number, Value };
use specta::Type;
use uuid::Uuid;

use crate::{ models::Template, templates::LayoutKind };

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum ValueType {
    Optional {
        contained: Box<ValueType>,
    },
    Boolean {},
    Number {},
    String {},
    Array {
        elements: Box<ValueType>,
    },
    Object {
        elements: Box<ValueType>,
    },
    Opaque {},
    Identifier {},
}

impl ValueType {
    pub fn validate(&self, value: Value) -> bool {
        match self {
            ValueType::Optional { contained } => {
                match value {
                    Value::Null => true,
                    other => contained.validate(other),
                }
            }
            ValueType::Boolean {} => value.is_boolean(),
            ValueType::Number { .. } => value.is_number(),
            ValueType::String {} => value.is_string(),
            ValueType::Array { elements } =>
                match value {
                    Value::Array(els) => els.iter().all(|v| elements.validate(v.clone())),
                    _ => false,
                }
            ValueType::Object { elements } =>
                match value {
                    Value::Object(items) => items.values().all(|v| elements.validate(v.clone())),
                    _ => false,
                }
            ValueType::Opaque {} => true,
            ValueType::Identifier {} => {
                if let Value::String(data) = value { Uuid::from_str(&data).is_ok() } else { false }
            }
        }
    }

    pub fn resolve<T: FromValue>(&self, value: Value) -> crate::Result<T> {
        T::from_value(self.clone(), value)
    }

    pub fn boolean() -> Self {
        Self::Boolean {}
    }

    pub fn number() -> Self {
        Self::Number {}
    }

    pub fn string() -> Self {
        Self::String {}
    }

    pub fn opaque() -> Self {
        Self::Opaque {}
    }

    pub fn identifier() -> Self {
        Self::Identifier {}
    }

    pub fn optional(contained: ValueType) -> Self {
        Self::Optional { contained: Box::new(contained) }
    }

    pub fn array(element_type: ValueType) -> Self {
        Self::Array { elements: Box::new(element_type) }
    }

    pub fn object(value_type: ValueType) -> Self {
        Self::Object { elements: Box::new(value_type) }
    }
}

pub trait FromValue {
    fn from_value(value_type: ValueType, value: Value) -> crate::Result<Self> where Self: Sized;
}

impl FromValue for bool {
    fn from_value(value_type: ValueType, value: Value) -> crate::Result<Self> {
        if let ValueType::Boolean {} = value_type {
            if let Value::Bool(val) = value {
                Ok(val)
            } else {
                Err(crate::Error::InvalidCastDatatype { value_type, value })
            }
        } else {
            Err(crate::Error::InvalidCast {
                value_type,
                expected_type: String::from("boolean"),
            })
        }
    }
}

impl FromValue for String {
    fn from_value(value_type: ValueType, value: Value) -> crate::Result<Self> {
        if let ValueType::String {} = value_type {
            if let Value::String(val) = value {
                Ok(val)
            } else {
                Err(crate::Error::InvalidCastDatatype { value_type, value })
            }
        } else {
            Err(crate::Error::InvalidCast {
                value_type,
                expected_type: String::from("string"),
            })
        }
    }
}

impl FromValue for Number {
    fn from_value(value_type: ValueType, value: Value) -> crate::Result<Self> {
        if let ValueType::Number {} = value_type {
            if let Value::Number(val) = value {
                Ok(val)
            } else {
                Err(crate::Error::InvalidCastDatatype { value_type, value })
            }
        } else {
            Err(crate::Error::InvalidCast {
                value_type,
                expected_type: String::from("number"),
            })
        }
    }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(value_type: ValueType, value: Value) -> crate::Result<Self> {
        if let ValueType::Array { elements } = value_type.clone() {
            if let Value::Array(vals) = value {
                let mut result = Vec::new();

                for v in vals {
                    result.push(T::from_value(*elements.clone(), v)?);
                }

                Ok(result)
            } else {
                Err(crate::Error::InvalidCastDatatype { value_type, value })
            }
        } else {
            Err(crate::Error::InvalidCast {
                value_type,
                expected_type: String::from("array"),
            })
        }
    }
}

impl<T: FromValue> FromValue for HashMap<String, T> {
    fn from_value(value_type: ValueType, value: Value) -> crate::Result<Self> {
        if let ValueType::Object { elements } = value_type.clone() {
            if let Value::Object(vals) = value {
                let mut result = HashMap::new();

                for (key, val) in vals {
                    result.insert(key, T::from_value(*elements.clone(), val)?);
                }

                Ok(result)
            } else {
                Err(crate::Error::InvalidCastDatatype { value_type, value })
            }
        } else {
            Err(crate::Error::InvalidCast {
                value_type,
                expected_type: String::from("object"),
            })
        }
    }
}

impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value_type: ValueType, value: Value) -> crate::Result<Self> {
        if let ValueType::Optional { contained } = value_type.clone() {
            match value {
                Value::Null => Ok(None),
                other => Ok(Some(T::from_value(*contained.clone(), other)?)),
            }
        } else {
            Err(crate::Error::InvalidCast {
                value_type,
                expected_type: String::from("optional"),
            })
        }
    }
}

impl FromValue for Value {
    fn from_value(value_type: ValueType, value: Value) -> crate::Result<Self> {
        if let ValueType::Opaque {} = value_type {
            Ok(value)
        } else {
            Err(crate::Error::InvalidCast {
                value_type,
                expected_type: String::from("opaque"),
            })
        }
    }
}

impl FromValue for Identifier {
    fn from_value(value_type: ValueType, value: Value) -> crate::Result<Self> where Self: Sized {
        if let ValueType::Identifier {} = value_type.clone() {
            if let Value::String(data) = value.clone() {
                if Uuid::from_str(&data).is_ok() {
                    Ok(Identifier::from(data))
                } else {
                    Err(crate::Error::InvalidCastDatatype { value_type, value })
                }
            } else {
                Err(crate::Error::InvalidCastDatatype { value_type, value })
            }
        } else {
            Err(crate::Error::InvalidCast {
                value_type,
                expected_type: String::from("identifier"),
            })
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case")]
pub enum NodeCategory {
    Field,
    Container,
    Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(String);

impl<T: Into<String>> From<T> for Identifier {
    fn from(value: T) -> Self {
        let s: String = value.into();
        Self(s)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Deref for Identifier {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Identifier {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::new()
    }
}

impl ToKey for Identifier {
    fn to_key(&self) -> native_db::Key {
        Key::new(self.0.as_bytes().to_vec())
    }

    fn key_names() -> Vec<String> {
        vec!["Identifier".to_string()]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub enum InnerPackageId {
    #[serde(rename = "::project")]
    Project,

    #[serde(rename = "::internal")]
    Internal
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(untagged)]
pub enum PackageId {
    #[serde(rename = "::project")]
    Project(InnerPackageId),

    #[serde(rename = "::internal")]
    Internal(InnerPackageId),

    Package(String),
}

impl PackageId {
    pub fn project() -> Self {
        Self::Project(InnerPackageId::Project)
    }

    pub fn internal() -> Self {
        Self::Internal(InnerPackageId::Internal)
    }

    pub fn package(id: impl AsRef<str>) -> Self {
        Self::Package(id.as_ref().to_string())
    }
}

impl Default for PackageId {
    fn default() -> Self {
        Self::Project(InnerPackageId::Project)
    }
}

impl Display for PackageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageId::Project(_) => f.write_str("::project"),
            PackageId::Internal(_) => f.write_str("::internal"),
            PackageId::Package(package_id) => f.write_str(&package_id),
        }
    }
}

impl From<String> for PackageId {
    fn from(value: String) -> Self {
        match value.as_str() {
            "::project" => Self::Project(InnerPackageId::Project),
            "::internal" => Self::Internal(InnerPackageId::Internal),
            other => Self::Package(other.to_string()),
        }
    }
}

impl Into<String> for PackageId {
    fn into(self) -> String {
        self.to_string()
    }
}

impl ToKey for PackageId {
    fn to_key(&self) -> native_db::Key {
        Key::new(self.to_string().as_bytes().to_vec())
    }

    fn key_names() -> Vec<String> {
        vec!["PackageId".to_string()]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "placement")]
pub enum Parent {
    Root,
    Child {parent: Identifier, collection: String}
}

impl Default for Parent {
    fn default() -> Self {
        Self::Root
    }
}

impl ToKey for Parent {
    fn key_names() -> Vec<String> {
        vec!["Parent".to_string()]
    }

    fn to_key(&self) -> Key {
        Key::new(self.to_string().as_bytes().to_vec())
    }
}

impl Display for Parent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self.clone() {
            Parent::Root => format!("root::root/root"),
            Parent::Child { parent, collection } => format!("child::{}/{}", parent.to_string(), collection),
        })
    }
}
