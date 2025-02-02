use pgorm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[pgorm(table_name = "users_votes")]
pub struct Model {
    #[pgorm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[pgorm(primary_key, auto_increment = false)]
    pub bill_id: i32,
    pub vote: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[pgorm(
        belongs_to = "super::bills::Entity",
        from = "Column::BillId",
        to = "super::bills::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Bills,
    #[pgorm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::bills::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bills.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
