use super::pgorm_active_enums::*;
use pgorm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[pgorm(table_name = "categories")]
pub struct Model {
    #[pgorm(primary_key, auto_increment = false)]
    pub id: i32,
    pub categories: Option<Vec<Category>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
