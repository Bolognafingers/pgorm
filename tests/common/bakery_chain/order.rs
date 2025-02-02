use pgorm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[pgorm(table_name = "order")]
pub struct Model {
    #[pgorm(primary_key)]
    pub id: i32,
    #[pgorm(column_type = "Decimal(Some((16, 4)))")]
    pub total: Decimal,
    pub bakery_id: i32,
    pub customer_id: i32,
    pub placed_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[pgorm(
        belongs_to = "super::bakery::Entity",
        from = "Column::BakeryId",
        to = "super::bakery::Column::Id"
    )]
    Bakery,
    #[pgorm(
        belongs_to = "super::customer::Entity",
        from = "Column::CustomerId",
        to = "super::customer::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Customer,
    #[pgorm(has_many = "super::lineitem::Entity")]
    Lineitem,
}

impl Related<super::bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bakery.def()
    }
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::lineitem::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lineitem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
