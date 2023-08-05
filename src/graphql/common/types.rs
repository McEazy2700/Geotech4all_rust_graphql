use async_graphql::*;
use crate::graphql::users::types::Register;


#[derive(SimpleObject)]
#[graphql(concrete(name="RegisterSuccess", params(Register)))]
pub struct Success<T: OutputType> {
    pub data: Option<T>,
    pub success: bool,
}
