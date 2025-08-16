use serde::{ Deserialize, Serialize };
use specta::Type;
use spire_enum::prelude::{delegate_impl, delegated_enum};

use crate::templates::{ types::Parent, Identifier, TemplateContainer, TemplateNode };

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

            fn parent(&self) -> Parent {
                self.parent.clone()
            }

            fn next(&self) -> Option<Identifier> {
                self.next.clone()
            }

            fn previous(&self) -> Option<Identifier> {
                self.previous.clone()
            }

            fn set_parent(&mut self, parent: Parent) -> () {
                self.parent = parent;
            }

            fn set_previous(&mut self, previous: Option<Identifier>) -> () {
                self.previous = previous;
            }

            fn set_next(&mut self, next: Option<Identifier>) -> () {
                self.next = next;
            }

            fn set_id(&mut self, id: Identifier) -> () {
                self.id = id;
            }
        }
    };
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Columns {
    pub id: Identifier,
    pub parent: Parent,
    pub previous: Option<Identifier>,
    pub next: Option<Identifier>,
    pub columns: usize,
}

impl_node!(Columns, "columns");

impl TemplateContainer for Columns {
    fn collections(&self) -> Vec<String> {
        (0..self.columns).map(|i| format!("col-{i}")).collect()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Collapsible {
    pub id: Identifier,

    pub parent: Parent,
    pub previous: Option<Identifier>,
    pub next: Option<Identifier>,
    pub default_collapsed: bool,
    pub title: String,

    #[serde(default)]
    pub icon: Option<String>,
}

impl_node!(Collapsible, "collapsible");

impl TemplateContainer for Collapsible {
    fn collections(&self) -> Vec<String> {
        vec!["children".to_string()]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct LabelledGroup {
    pub id: Identifier,

    pub parent: Parent,
    pub previous: Option<Identifier>,
    pub next: Option<Identifier>,
    pub label: String,

    #[serde(default)]
    pub icon: Option<String>,
}

impl_node!(LabelledGroup, "labelled_group");

impl TemplateContainer for LabelledGroup {
    fn collections(&self) -> Vec<String> {
        vec!["children".to_string()]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Wrapper {
    pub id: Identifier,

    pub parent: Parent,
    pub previous: Option<Identifier>,
    pub next: Option<Identifier>,
}

impl_node!(Wrapper, "wrapper");

impl TemplateContainer for Wrapper {
    fn collections(&self) -> Vec<String> {
        vec!["children".to_string()]
    }
}

#[delegated_enum(
    impl_conversions
)]
#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "node_kind")]
pub enum ContainerNode {
    Columns(Columns),
    Collapsible(Collapsible),
    LabelledGroup(LabelledGroup),
    Wrapper(Wrapper),
}

#[delegate_impl]
impl TemplateNode for ContainerNode {
    fn id(&self) -> Identifier;
    fn node_kind(&self) -> String;
    fn node_category(&self) -> super::NodeCategory;
    fn parent(&self) -> Parent;
    fn previous(&self) -> Option<Identifier>;
    fn next(&self) -> Option<Identifier>;
    fn set_parent(&mut self, parent: Parent) -> ();
    fn set_previous(&mut self, previous: Option<Identifier>) -> ();
    fn set_next(&mut self, next: Option<Identifier>) -> ();
    fn set_id(&mut self, id: Identifier) -> ();
}

#[delegate_impl]
impl TemplateContainer for ContainerNode {
    fn collections(&self) -> Vec<String>;
}
