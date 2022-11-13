use std::collections::HashMap;

use crate::{db::create_connection};
use diesel::{dsl::count, prelude::*};
use serde::Serialize;

pub struct TaskRunner {
    conn: PgConnection,
}

#[derive(Serialize, Clone)]
pub struct Answers {
    pub first: Vec<i32>,
    pub second: Vec<i32>,
    pub third: Vec<(String, String)>,
    pub forth: Vec<String>,
    pub fifth: Vec<String>,
}

impl TaskRunner {
    pub fn new() -> Self {
        Self {
            conn: create_connection(),
        }
    }

    pub fn get_answers(&mut self) -> Answers {
        Answers {
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
        use crate::schema::olympics::events;
        use crate::schema::olympics::olympics;
        use crate::schema::olympics::players;
        use crate::schema::olympics::results;

        results::table
            .inner_join(players::table.on(players::player_id.eq(results::player_id)))
            .inner_join(events::table.on(events::event_id.eq(results::event_id)))
            .inner_join(olympics::table.on(olympics::olympic_id.eq(events::olympic_id)))
            .select((players::player_id, olympics::olympic_id))
            .load(&mut self.conn)
            .expect("Cannot execute query")
    }

    pub fn run4(&mut self) -> Vec<String> {
        use crate::schema::olympics::countries;
        use crate::schema::olympics::players;

        let mut countries = Vec::<(String, i32)>::new();

        for c in ['A', 'E', 'I', 'O', 'U'] {
            countries.extend(
                countries::table
                        .inner_join(players::table.on(players::country_id.eq(countries::country_id)))
                        .filter(players::name.like(format!("{}%", c)))
                        .select((countries::country_id, countries::population))
                        .load(&mut self.conn)
                        .expect("Cannot execute query")
            );
        }

        let mut counts = HashMap::<String, (i32, i32)>::new();
        for (c, p) in countries {
            if counts.contains_key(&c) {
                counts.get_mut(&c).expect("Cannot find the value").0 += 1;
                continue;
            }
            counts.insert(c, (1, p));
        }

        let mut ans: (&String, f64) = (&String::new(), -1f64);
        for country in counts.keys() {
            let vals = counts[country];
            if (vals.0 as f64) / (vals.1 as f64) > ans.1 {
                ans.0 = country;
            }
        }

        vec![ans.0.clone()]
    }

    pub fn run5(&mut self) -> Vec<String> {
        use crate::schema::olympics::countries;
        use crate::schema::olympics::events;
        use crate::schema::olympics::players;
        use crate::schema::olympics::results;

        countries::table
            .inner_join(players::table.on(players::country_id.eq(countries::country_id)))
            .inner_join(results::table.on(results::player_id.eq(players::player_id)))
            .inner_join(events::table.on(events::event_id.eq(results::event_id)))
            .filter(events::is_team_event.eq(1))
            .group_by(countries::country_id)
            .order_by((
                count(players::player_id).desc(),
                countries::population.desc(),
            ))
            .select(countries::country_id)
            .load(&mut self.conn)
            .expect("Cannot execute query")[..5]
            .to_vec()
    }
}
