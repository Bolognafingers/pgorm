use pgorm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[pgorm(schema_name = "schema_name", table_name = "_cake_filling_")]
pub struct Model {
    #[pgorm(primary_key, auto_increment = false)]
    pub cake_id: i32,
    #[pgorm(primary_key, auto_increment = false)]
    pub filling_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[pgorm(
        belongs_to = "super::cake::Entity",
        from = "Column::CakeId",
        to = "super::cake::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade",
    )]
    Cake,
    #[pgorm(
        belongs_to = "super::filling::Entity",
        from = "Column::FillingId",
        to = "super::filling::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade",
    )]
    Filling,
}

impl Related<super::cake::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cake.def()
    }
}

impl Related<super::filling::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Filling.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
