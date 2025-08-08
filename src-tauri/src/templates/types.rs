use std::collections::HashMap;
use serde::{ Deserialize, Serialize };
use serde_json::{ Number, Value };
use specta::Type;

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
        }
    }

    pub fn resolve<T: FromValue>(&self, value: Value) -> crate::Result<T> {
        T::from_value(self.clone(), value)
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
