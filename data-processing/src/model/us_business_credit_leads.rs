//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(
    schema_name = "us_market_insights",
    table_name = "us_business_credit_leads"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub business_name: String,
    #[sea_orm(column_type = "Text")]
    pub mailing_address: String,
    #[sea_orm(column_type = "Text")]
    pub mailing_city: String,
    #[sea_orm(column_type = "Text")]
    pub mailing_state: String,
    #[sea_orm(column_type = "Text")]
    pub mailing_zip: String,
    #[sea_orm(column_type = "Text")]
    pub phone: String,
    #[sea_orm(column_type = "Text")]
    pub fax: String,
    #[sea_orm(column_type = "Text")]
    pub sales_volume: String,
    #[sea_orm(column_type = "Text")]
    pub number_of_employees: String,
    #[sea_orm(column_type = "Text")]
    pub public_private_company: String,
    #[sea_orm(column_type = "Text")]
    pub location_type: String,
    #[sea_orm(column_type = "Text")]
    pub firm_or_individual: String,
    #[sea_orm(column_type = "Text")]
    pub credit_score: String,
    #[sea_orm(column_type = "Text")]
    pub stock_exchange: String,
    #[sea_orm(column_type = "Text")]
    pub stock_ticker_symbol: String,
    #[sea_orm(column_type = "Text")]
    pub website: String,
    #[sea_orm(column_type = "Text")]
    pub first_name: String,
    #[sea_orm(column_type = "Text")]
    pub last_name: String,
    #[sea_orm(column_type = "Text")]
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub industry: String,
    #[sea_orm(column_type = "Text")]
    pub sic: String,
    #[sea_orm(column_type = "Text")]
    pub naics: String,
    #[sea_orm(column_type = "Text")]
    pub ncci: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
