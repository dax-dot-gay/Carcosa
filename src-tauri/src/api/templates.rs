use crate::templates::Template;

#[taurpc::procedures(path = "templates")]
pub trait TemplateApi {
    async fn get_template(id: String) -> Option<Template>;
}

#[derive(Clone)]
pub struct TemplateApiImpl;

#[taurpc::resolvers]
impl TemplateApi for TemplateApiImpl {
    // Stub to check type conversion
    async fn get_template(self, _id: String) -> Option<Template> {
        None
    }
}
