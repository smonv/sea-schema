use super::{InformationSchema, SchemaQueryBuilder};
use crate::sqlx_types::{postgres::PgRow, Row};
use sea_query::{Expr, Iden, Query, SelectStatement};
use std::rc::Rc;

#[derive(Debug, sea_query::Iden)]
/// Ref: https://www.postgresql.org/docs/13/infoschema-tables.html
pub enum TablesFields {
    TableCatalog,
    TableSchema,
    TableName,
    TableType,
    UserDefinedTypeCatalog,
    UserDefinedTypeSchema,
    UserDefinedTypeName,
    // IsInsertableInto is always true for BASE TABLEs
    IsInsertableInto,
    IsTyped,
}

#[derive(Debug, sea_query::Iden)]
pub enum TableType {
    #[iden = "BASE TABLE"]
    BaseTable,
    #[iden = "VIEW"]
    View,
    #[iden = "FOREIGN"]
    Foreign,
    #[iden = "LOCAL TEMPORARY"]
    Temporary,
}

#[derive(Debug, Default)]
pub struct TableQueryResult {
    pub table_name: String,
    pub user_defined_type_schema: Option<String>,
    pub user_defined_type_name: Option<String>,
}

impl SchemaQueryBuilder {
    pub fn query_tables(&self, schema: Rc<dyn Iden>) -> SelectStatement {
        Query::select()
            .columns(vec![
                TablesFields::TableName,
                TablesFields::UserDefinedTypeSchema,
                TablesFields::UserDefinedTypeName,
            ])
            .from((InformationSchema::Schema, InformationSchema::Tables))
            .and_where(Expr::col(TablesFields::TableSchema).eq(schema.to_string()))
            .and_where(Expr::col(TablesFields::TableType).eq(TableType::BaseTable.to_string()))
            .take()
    }
}

#[cfg(feature = "sqlx-postres")]
impl From<&PgRow> for TableQueryResult {
    fn from(row: &PgRow) -> Self {
        Self {
            table_name: row.get(0),
            user_defined_type_schema: row.get(1),
            user_defined_type_name: row.get(2),
        }
    }
}

#[cfg(not(feature = "sqlx-postres"))]
impl From<&PgRow> for TableQueryResult {
    fn from(row: &PgRow) -> Self {
        Self::default()
    }
}