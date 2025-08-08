use serde::{ Deserialize, Serialize };
use specta::Type;

use crate::templates::{ Identifier, TemplateNode };

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
                super::NodeCategory::Other
            }

            fn parent(&self) -> Option<Identifier> {
                self.parent.clone()
            }
        }
    };
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Text {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,
    pub content: String,
}

impl_node!(Text, "text");

#[derive(Serialize, Deserialize, Clone, Debug, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum AlertLevel {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Alert {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,
    pub content: String,

    #[serde(default)]
    pub level: AlertLevel,

    #[serde(default)]
    pub title: Option<String>,
}

impl_node!(Alert, "alert");

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "node_kind")]
pub enum OtherNode {
    Text(Text),
    Alert(Alert),
}
