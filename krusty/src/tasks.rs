use diesel::{prelude::*, sql_query};
use crate::model::*;
use crate::db::create_connection;
use serde::{Serialize};

pub struct TaskRunner {
    conn: PgConnection,
}

#[derive(Serialize, Clone)]
pub struct Answers {
    pub first: Vec<i32>,
    pub second: Vec<i32>,
    pub third: Vec<(String, String)>,
    pub forth: Vec<i32>,
    pub fifth: Vec<i32>,
}

impl TaskRunner {
    pub fn new() -> Self {
        Self {
            conn: create_connection(),
        }
    }

    pub fn get_answers(&mut self) -> Answers {
        Answers{
            first: self.run1(),
            second: self.run2(),
            third: self.run3(),
            forth: self.run4(),
            fifth: self.run5(),
        }
    }

    pub fn run1(&mut self) -> Vec<i32> {
        vec![1]
    }

    pub fn run2(&mut self) -> Vec<i32> {        
        vec![2]
    }

    pub fn run3(&mut self) -> Vec<(String, String)> {
        use crate::schema::olympics::results;
        use crate::schema::olympics::players;
        use crate::schema::olympics::olympics;
        use crate::schema::olympics::events;

        let ans: Vec<(String, String)> = results::table
        .inner_join(players::table.on(players::player_id.eq(results::player_id)))
        .inner_join(events::table.on(events::event_id.eq(results::event_id)))
        .inner_join(olympics::table.on(olympics::olympic_id.eq(events::olympic_id)))
        .select((players::player_id, olympics::olympic_id))
        .load(&mut self.conn)
        .expect("Cannot execute query");

        ans
    }

    pub fn run4(&mut self) -> Vec<i32> {
        vec![4]
    }

    pub fn run5(&mut self) -> Vec<i32> {
        vec![5]
    }
}
