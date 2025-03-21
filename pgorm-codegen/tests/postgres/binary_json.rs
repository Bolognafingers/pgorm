use pgorm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[pgorm(table_name = "task")]
pub struct Model {
    #[pgorm(primary_key)]
    pub id: i32,
    pub payload: Json,
    #[pgorm(column_type = "JsonBinary")]
    pub payload_binary: Json,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
