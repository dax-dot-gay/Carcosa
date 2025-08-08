use std::{ collections::HashMap, ops::Deref, str::FromStr };
use serde::{ Deserialize, Serialize };
use serde_json::{ Number, Value };
use specta::Type;
use uuid::Uuid;

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

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        self.0
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
