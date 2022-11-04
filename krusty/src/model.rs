use diesel::prelude::*;
use diesel::sql_types;
use crate::schema::olympics;

#[derive(Debug, Queryable)]
#[diesel(table_name = countries)]
pub struct Country<'a> {
    pub name: &'a str,
    pub country_id: &'a str,
    pub area_sqkm: i32,
    pub population: i32,
}

#[derive(Debug, Queryable)]
#[diesel(table_name = olympics)]
pub struct Olympic<'a> {
    pub olympic_id: &'a str,
    pub country_id: &'a str,
    pub city: &'a str,
    pub year: i32,
    pub start_date: sql_types::Date,
    pub end_date: sql_types::Date,
}

#[derive(Debug, Queryable)]
#[diesel(table_name = players)]
pub struct Player<'a> {
    pub name: &'a str,
    pub player_id: &'a str,
    pub country_id: &'a str,
    pub birth_date: sql_types::Date,
}

#[derive(Debug, Queryable)]
#[diesel(table_name = events)]
pub struct Event<'a> {
    pub event_id: &'a str,
    pub name: &'a str,
    pub event_type: &'a str,
    pub olympic_id: &'a str,
    pub is_team_event: bool,
    pub num_players_in_team: i32,
    pub result_noted_in: &'a str,
}

#[derive(Debug, Queryable)]
#[diesel(table_name = results)]
pub struct Result<'a> {
    pub event_id: &'a str,
    pub player_id: &'a str,
    pub medal: &'a str,
    pub result: f64,
}
