use serde::{ Deserialize, Serialize };
use serde_json::Value;
use specta::Type;

use crate::templates::{ Identifier, TemplateNode, TemplateField, ValueType };

macro_rules! impl_node {
    ($node:ty, $kind:literal) => {
        impl TemplateNode for $node {
            fn id(&self) -> Identifier {
                self.id.clone()
            }

            fn node_kind(&self) -> String {
                String::from($kind)
            }

            fn node_category(&self) -> super::NodeCategory {
                super::NodeCategory::Field
            }

            fn parent(&self) -> Option<Identifier> {
                self.parent.clone()
            }
        }
    };
}

macro_rules! impl_field {
    (node = $node:ty; typ = $vtype:expr; default = $default:item) => {
        impl TemplateField for $node {
            fn key(&self) -> String {
                self.key.clone()
            }
            fn value_type(&self) -> ValueType {
                $vtype
            }
            fn label(&self) -> Option<String> {
                self.label.clone()
            }
            fn icon(&self) -> Option<String> {
                self.icon.clone()
            }
            fn help_text(&self) -> Option<String> {
                self.help_text.clone()
            }

            $default
        }
    };
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct TextField {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,

    pub key: String,

    #[serde(default)]
    pub label: Option<String>,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub help_text: Option<String>,

    #[serde(default)]
    pub placeholder: Option<String>,

    #[serde(default)]
    pub default_value: Option<String>,
}

impl_node!(TextField, "text_field");
impl_field! {
    node = TextField;
    typ = ValueType::string();
    default = fn default_value(&self) -> Value {
        self.default_value.clone().unwrap_or(String::new()).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct NumberField {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,

    pub key: String,

    #[serde(default)]
    pub label: Option<String>,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub help_text: Option<String>,

    #[serde(default)]
    pub placeholder: Option<String>,

    #[serde(default)]
    pub allow_negatives: bool,

    #[serde(default)]
    pub allow_decimals: bool,

    #[serde(default)]
    pub minimum: Option<f64>,

    #[serde(default)]
    pub maximum: Option<f64>,

    #[serde(default)]
    pub default_value: Option<f64>,
}

impl_node!(NumberField, "number_field");
impl_field! {
    node = NumberField;
    typ = ValueType::number();
    default = fn default_value(&self) -> Value {
        self.default_value.clone().unwrap_or(0.0f64).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Switch {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,

    pub key: String,

    #[serde(default)]
    pub label: Option<String>,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub help_text: Option<String>,

    #[serde(default)]
    pub on_icon: Option<String>,

    #[serde(default)]
    pub off_icon: Option<String>,

    #[serde(default)]
    pub default_value: Option<bool>,
}

impl_node!(Switch, "switch");
impl_field! {
    node = Switch;
    typ = ValueType::boolean();
    default = fn default_value(&self) -> Value {
        self.default_value.clone().unwrap_or(false).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct SingleSelect {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,

    pub key: String,

    #[serde(default)]
    pub label: Option<String>,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub help_text: Option<String>,

    #[serde(default)]
    pub placeholder: Option<String>,

    #[serde(default)]
    pub default_value: Option<String>,

    pub options: Vec<String>,
}

impl_node!(SingleSelect, "single_select");
impl_field! {
    node = SingleSelect;
    typ = ValueType::optional(ValueType::string());
    default = fn default_value(&self) -> Value {
        self.default_value.clone().into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct MultiSelect {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,

    pub key: String,

    #[serde(default)]
    pub label: Option<String>,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub help_text: Option<String>,

    #[serde(default)]
    pub placeholder: Option<String>,

    #[serde(default)]
    pub default_value: Option<Vec<String>>,

    pub options: Vec<String>,

    #[serde(default)]
    pub max_selections: Option<usize>,
}

impl_node!(MultiSelect, "single_select");
impl_field! {
    node = MultiSelect;
    typ = ValueType::array(ValueType::string());
    default = fn default_value(&self) -> Value {
        self.default_value.clone().unwrap_or(Vec::new()).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct MultiLine {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,

    pub key: String,

    #[serde(default)]
    pub label: Option<String>,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub help_text: Option<String>,

    #[serde(default)]
    pub placeholder: Option<String>,

    #[serde(default)]
    pub default_value: Option<String>,

    #[serde(default)]
    pub lines: Option<usize>,
}

impl_node!(MultiLine, "multi_line");
impl_field! {
    node = MultiLine;
    typ = ValueType::string();
    default = fn default_value(&self) -> Value {
        self.default_value.clone().unwrap_or(String::new()).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct RichText {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,

    pub key: String,

    #[serde(default)]
    pub label: Option<String>,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub help_text: Option<String>,
}

impl_node!(RichText, "rich_text");
impl_field! {
    node = RichText;
    typ = ValueType::optional(ValueType::opaque());
    default = fn default_value(&self) -> Value {
        Value::Null
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(untagged)]
pub enum MatchCriteria {
    TemplateAndValue {
        template_id: Identifier,
        key: String,
        value: Value,
    },
    Template {
        template_id: Identifier,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct LinkedDocument {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,

    pub key: String,

    #[serde(default)]
    pub label: Option<String>,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub help_text: Option<String>,

    #[serde(default)]
    pub constraints: Option<Vec<MatchCriteria>>,
}

impl_node!(LinkedDocument, "linked_document");
impl_field! {
    node = LinkedDocument;
    typ = ValueType::optional(ValueType::identifier());
    default = fn default_value(&self) -> Value {
        Value::Null
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct MultiLinkedDocuments {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,

    pub key: String,

    #[serde(default)]
    pub label: Option<String>,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub help_text: Option<String>,

    #[serde(default)]
    pub constraints: Option<Vec<MatchCriteria>>,

    #[serde(default)]
    pub max_selections: Option<usize>,
}

impl_node!(MultiLinkedDocuments, "multi_linked_document");
impl_field! {
    node = MultiLinkedDocuments;
    typ = ValueType::array(ValueType::identifier());
    default = fn default_value(&self) -> Value {
        Value::Array(Vec::new())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "node_kind")]
pub enum FieldNode {
    TextField(TextField),
    NumberField(NumberField),
    Switch(Switch),
    SingleSelect(SingleSelect),
    MultiSelect(MultiSelect),
    RichText(RichText),
    LinkedDocument(LinkedDocument),
    MultiLinkedDocuments(MultiLinkedDocuments),
}
