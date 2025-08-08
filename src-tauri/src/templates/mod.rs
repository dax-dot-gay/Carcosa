use uuid::Uuid;
pub mod types;

pub use types::{ FromValue, ValueType };

pub trait TemplateNode {
    fn id(&self) -> String;
    fn parent(&self) -> Option<String>;
    fn new_id() -> String where Self: Sized {
        Uuid::new_v4().to_string()
    }
}

pub trait TemplateContainer: TemplateNode {
    fn children(&self) -> Vec<String>;
}

pub trait TemplateField: TemplateNode {
    fn key(&self) -> String;
    fn value_type(&self) -> ValueType;
}
