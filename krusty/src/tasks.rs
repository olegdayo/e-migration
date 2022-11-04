use diesel::prelude::*;
use crate::model::*;
use crate::db::create_connection;
use serde::{Serialize};

pub struct TaskRunner {
    conn: PgConnection,
}

#[derive(Serialize, Clone)]
pub struct Answers {
    pub first: Vec<Country>,
    pub second: Vec<i32>,
    pub third: Vec<i32>,
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

    pub fn run1(&mut self) -> Vec<Country> {
        use crate::schema::olympics::countries::dsl::*;

        let cntrs: Vec<Country> = countries
            .load(&mut self.conn)
            .expect("Something went wrong with the countries!");
        cntrs
    }

    pub fn run2(&mut self) -> Vec<i32> {
        vec![2]
    }

    pub fn run3(&mut self) -> Vec<i32> {
        vec![3]
    }

    pub fn run4(&mut self) -> Vec<i32> {
        vec![4]
    }

    pub fn run5(&mut self) -> Vec<i32> {
        vec![5]
    }
}
