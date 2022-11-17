use std::collections::HashMap;

use crate::db::create_connection;
use diesel::{dsl::count, prelude::*};
use serde::Serialize;

pub struct TaskRunner {
    conn: PgConnection,
}

#[derive(Serialize, Clone)]
pub struct Answers {
    pub first: Vec<(String, i64, i64)>,
    pub second: Vec<String>,
    pub third: Vec<(String, String)>,
    pub forth: Vec<String>,
    pub fifth: Vec<(String, f64)>,
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

    pub fn run1(&mut self) -> Vec<(String, i64, i64)> {
        use crate::schema::olympics::events;
        use crate::schema::olympics::olympics;
        use crate::schema::olympics::players;
        use crate::schema::olympics::results;

        let all_counts: Vec<(String, i64)> = olympics::table
            .filter(olympics::year.eq(2004))
            .inner_join(events::table.on(events::olympic_id.eq(olympics::olympic_id)))
            .inner_join(results::table.on(results::event_id.eq(events::event_id)))
            .inner_join(players::table.on(players::player_id.eq(results::player_id)))
            .group_by(olympics::olympic_id)
            .select((olympics::olympic_id, count(players::player_id)))
            .load(&mut self.conn)
            .expect("Cannot execute query");

        let golden_counts: Vec<(String, i64)> = olympics::table
            .filter(olympics::year.eq(2004))
            .inner_join(events::table.on(events::olympic_id.eq(olympics::olympic_id)))
            .inner_join(results::table.on(results::event_id.eq(events::event_id)))
            .filter(results::medal.eq("GOLD"))
            .inner_join(players::table.on(players::player_id.eq(results::player_id)))
            .group_by(olympics::olympic_id)
            .select((olympics::olympic_id, count(players::player_id)))
            .load(&mut self.conn)
            .expect("Cannot execute query");

        let mut counts = HashMap::<String, (i64, i64)>::new();

        for (olymp, count) in all_counts {
            counts.insert(olymp, (count, 0i64));
        }

        for (olymp, count) in golden_counts {
            counts.get_mut(&olymp).expect("Cannot get a key").1 = count;
        }

        counts
            .into_iter()
            .map(|v: (String, (i64, i64))| (v.0, v.1 .0, v.1 .1))
            .collect()
    }

    pub fn run2(&mut self) -> Vec<String> {
        use crate::schema::olympics::events;

        events::table
            .filter(events::is_team_event.eq(0))
            .select(events::event_id)
            .load(&mut self.conn)
            .expect("Cannot execute query")
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
                    .expect("Cannot execute query"),
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

    pub fn run5(&mut self) -> Vec<(String, f64)> {
        use crate::schema::olympics::countries;
        use crate::schema::olympics::events;
        use crate::schema::olympics::players;
        use crate::schema::olympics::results;

        let countries: Vec<(String, i64, i32)> = countries::table
            .inner_join(players::table.on(players::country_id.eq(countries::country_id)))
            .inner_join(results::table.on(results::player_id.eq(players::player_id)))
            .inner_join(events::table.on(events::event_id.eq(results::event_id)))
            .filter(events::is_team_event.eq(1))
            .group_by(countries::country_id)
            .select((
                countries::country_id,
                count(players::player_id),
                countries::population,
            ))
            .load(&mut self.conn)
            .expect("Cannot execute query")
            .to_vec();

        let mut densities: Vec<(String, f64)> = countries
            .into_iter()
            .map(|v: (String, i64, i32)| (v.0, v.1 as f64 / v.2 as f64))
            .collect();

        densities
            .sort_by(
                |a, b| {
                    if a.1 > b.1 {
                        return std::cmp::Ordering::Greater;
                    } else if a.1 < b.1 {
                        return std::cmp::Ordering::Less;
                    }
                    return std::cmp::Ordering::Equal;
                }
            );

        densities[..5].to_vec()
    }
}
