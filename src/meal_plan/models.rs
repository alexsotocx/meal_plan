use std::io::Write;

use crate::schema::sql_types::MealTypeSql;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::pg::PgValue;
use diesel::serialize::Output;
use diesel::serialize::ToSql;
use diesel::{expression::AsExpression, pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub password: String,
    pub family_id: Uuid,
}

#[derive(Debug)]
pub struct Family {
    pub id: Uuid,
}

#[derive(Debug, Insertable, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::meal_plans)]
pub struct MealPlan {
    pub id: Uuid,
    pub week: i16,
    pub year: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression)]
#[diesel(sql_type = MealTypeSql)]
pub enum MealTypeEnum {
    Breakfast,
    Lunch,
    Dinner,
}

impl ToSql<MealTypeSql, Pg> for MealTypeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            MealTypeEnum::Breakfast => out.write_all(b"breakfast")?,
            MealTypeEnum::Lunch => out.write_all(b"lunch")?,
            MealTypeEnum::Dinner => out.write_all(b"dinner")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromSql<MealTypeSql, Pg> for MealTypeEnum {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"breakfast" => Ok(MealTypeEnum::Breakfast),
            b"lunch" => Ok(MealTypeEnum::Lunch),
            b"dinner" => Ok(MealTypeEnum::Dinner),
            _ => Err("Unrecognized meal type".into()),
        }
    }
}

#[derive(Insertable, Queryable, Identifiable, Debug)]
#[diesel(table_name = crate::schema::meal_plan_items)]
pub struct MealPlanItem {
    pub id: Uuid,
    pub meal_plan_id: Uuid,
    pub recipe_id: Uuid,
    pub date: NaiveDate,
    pub servings: i16,
    pub meal_type: MealTypeEnum,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub servings: i16,
}

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Associations, Identifiable)]
#[diesel(table_name = crate::schema::ingridients)]
#[diesel(belongs_to(Recipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ingridient {
    pub id: Uuid,
    pub name: String,
    pub unit: String,
    pub quantity: bigdecimal::BigDecimal,
    pub recipe_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
