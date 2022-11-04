use diesel::prelude::*;
use chrono::{Date, Utc};
use serde::{Serialize};

#[derive(Debug, Queryable, Clone, Serialize)]
#[diesel(table_name = countries)]
pub struct Country {
    pub name: String,
    pub country_id: String,
    pub area_sqkm: i32,
    pub population: i32,
}

#[derive(Debug, Queryable, Clone)]
#[diesel(table_name = olympics)]
pub struct Olympic {
    pub olympic_id: String,
    pub country_id: String,
    pub city: String,
    pub year: i32,
    pub start_date: Date<Utc>,
    pub end_date: Date<Utc>,
}

#[derive(Debug, Queryable, Clone)]
#[diesel(table_name = players)]
pub struct Player {
    pub name: String,
    pub player_id: String,
    pub country_id: String,
    pub birth_date: Date<Utc>,
}

#[derive(Debug, Queryable, Clone, Serialize)]
#[diesel(table_name = events)]
pub struct Event {
    pub event_id: String,
    pub name: String,
    pub event_type: String,
    pub olympic_id: String,
    pub is_team_event: bool,
    pub num_players_in_team: i32,
    pub result_noted_in: String,
}

#[derive(Debug, Queryable, Clone, Serialize)]
#[diesel(table_name = results)]
pub struct Result {
    pub event_id: String,
    pub player_id: String,
    pub medal: String,
    pub result: f64,
}
