use crate as pgorm;
use crate::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[pgorm(table_name = "vendor")]
pub struct Model {
    #[pgorm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::filling::Entity> for Entity {
    fn to() -> RelationDef {
        super::filling::Relation::Vendor.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
