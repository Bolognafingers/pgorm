use pgorm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[pgorm(table_name = "applog", comment = "app logs")]
pub struct Model {
    #[pgorm(primary_key, comment = "ID")]
    pub id: i32,
    #[pgorm(comment = "action")]
    pub action: String,
    #[pgorm(comment = "action data")]
    pub json: Json,
    #[pgorm(comment = "create time")]
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
