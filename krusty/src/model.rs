use diesel::prelude::*;
use chrono::{Date, Utc};
use serde::{Serialize};
use diesel::sql_types;

use crate::schema::olympics::events;

#[derive(Debug, Queryable, QueryableByName, Clone, Serialize)]
#[diesel(table_name = countries)]
#[table_name = "countries"]
pub struct Country {
    #[diesel(sql_type = sql_types::Varchar)]
    pub name: String,
    #[diesel(sql_type = sql_types::Varchar)]
    pub country_id: String,
    #[diesel(sql_type = sql_types::Integer)]
    pub area_sqkm: i32,
    #[diesel(sql_type = sql_types::Integer)]
    pub population: i32,
}

#[derive(Debug, Queryable, QueryableByName, Clone)]
#[diesel(table_name = olympics)]
#[table_name = "olympics"]
pub struct Olympic {
    #[diesel(sql_type = sql_types::Varchar)]
    pub olympic_id: String,
    #[diesel(sql_type = sql_types::Varchar)]
    pub country_id: String,
    #[diesel(sql_type = sql_types::Varchar)]
    pub city: String,
    #[diesel(sql_type = sql_types::Integer)]
    pub year: i32,
    #[diesel(sql_type = sql_types::Date)]
    pub start_date: Date<Utc>,
    #[diesel(sql_type = sql_types::Date)]
    pub end_date: Date<Utc>,
}

#[derive(Debug, Queryable, QueryableByName, Clone)]
#[diesel(table_name = players)]
#[table_name = "players"]
pub struct Player {
    #[diesel(sql_type = sql_types::Varchar)]
    pub name: String,
    #[diesel(sql_type = sql_types::Varchar)]
    pub player_id: String,
    #[diesel(sql_type = sql_types::Varchar)]
    pub country_id: String,
    #[diesel(sql_type = sql_types::Date)]
    pub birth_date: Date<Utc>,
}

#[derive(Debug, Queryable, QueryableByName, Clone, Serialize)]
#[diesel(table_name = events)]
#[table_name = "events"]
pub struct Event {
    #[diesel(sql_type = sql_types::Varchar)]
    pub event_id: String,
    #[diesel(sql_type = sql_types::Varchar)]
    pub name: String,
    #[diesel(sql_type = sql_types::Varchar)]
    pub event_type: String,
    #[diesel(sql_type = sql_types::Varchar)]
    pub olympic_id: String,
    pub is_team_event: bool,
    #[diesel(sql_type = sql_types::Integer)]
    pub num_players_in_team: i32,
    #[diesel(sql_type = sql_types::Varchar)]
    pub result_noted_in: String,
}

#[derive(Debug, Queryable, QueryableByName, Clone, Serialize)]
#[diesel(table_name = results)]
#[table_name = "results"]
pub struct Result {
    #[diesel(sql_type = sql_types::Varchar)]
    pub event_id: String,
    #[diesel(sql_type = sql_types::Varchar)]
    pub player_id: String,
    #[diesel(sql_type = sql_types::Varchar)]
    pub medal: String,
    #[diesel(sql_type = sql_types::Double)]
    pub result: f64,
}
