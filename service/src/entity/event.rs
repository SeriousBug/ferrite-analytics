//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "event")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub key: String,
    pub date: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::property_boolean::Entity")]
    PropertyBoolean,
    #[sea_orm(has_many = "super::property_integer::Entity")]
    PropertyInteger,
    #[sea_orm(has_many = "super::property_string::Entity")]
    PropertyString,
}

impl Related<super::property_boolean::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PropertyBoolean.def()
    }
}

impl Related<super::property_integer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PropertyInteger.def()
    }
}

impl Related<super::property_string::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PropertyString.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
