use serde::{ Deserialize, Serialize };
use specta::Type;

use crate::templates::{ Identifier, TemplateNode, TemplateContainer };

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
                super::NodeCategory::Container
            }

            fn parent(&self) -> Option<Identifier> {
                self.parent.clone()
            }
        }

        impl TemplateContainer for $node {
            fn children(&self) -> Vec<Identifier> {
                self.children.clone()
            }
        }
    };
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Columns {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,
    pub children: Vec<Identifier>,
    pub columns: Vec<Vec<Identifier>>,
}

impl_node!(Columns, "columns");

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Collapsible {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,
    pub children: Vec<Identifier>,
    pub default_collapsed: bool,
    pub title: String,

    #[serde(default)]
    pub icon: Option<String>,
}

impl_node!(Collapsible, "collapsible");

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct LabelledGroup {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,
    pub children: Vec<Identifier>,
    pub label: String,

    #[serde(default)]
    pub icon: Option<String>,
}

impl_node!(LabelledGroup, "labelled_group");

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Wrapper {
    #[serde(default)]
    pub id: Identifier,

    #[serde(default)]
    pub parent: Option<Identifier>,
    pub children: Vec<Identifier>,
}

impl_node!(Wrapper, "wrapper");

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "node_kind")]
pub enum ContainerNode {
    Columns(Columns),
    Collapsible(Collapsible),
    LabelledGroup(LabelledGroup),
    Wrapper(Wrapper),
}
