#[taurpc::procedures]
pub trait ApplicationApi {}

#[derive(Clone)]
pub struct ApplicationApiImpl;

#[taurpc::resolvers]
impl ApplicationApi for ApplicationApiImpl {}
