// @generated automatically by Diesel CLI.

pub mod olympics {
    diesel::table! {
        olympics.countries (country_id) {
            name -> Varchar,
            country_id -> Varchar,
            area_sqkm -> Int4,
            population -> Int4,
        }
    }

    diesel::table! {
        olympics.events (event_id) {
            event_id -> Varchar,
            name -> Varchar,
            eventtype -> Varchar,
            olympic_id -> Varchar,
            is_team_event -> Int4,
            num_players_in_team -> Int4,
            result_noted_in -> Varchar,
        }
    }

    diesel::table! {
        olympics.olympics (olympic_id) {
            olympic_id -> Varchar,
            country_id -> Varchar,
            city -> Varchar,
            year -> Int4,
            startdate -> Date,
            enddate -> Date,
        }
    }

    diesel::table! {
        olympics.players (player_id) {
            name -> Varchar,
            player_id -> Varchar,
            country_id -> Varchar,
            birthdate -> Date,
        }
    }

    diesel::table! {
        olympics.results (event_id, player_id) {
            event_id -> Varchar,
            player_id -> Varchar,
            medal -> Varchar,
            result -> Float8,
        }
    }

    diesel::joinable!(events -> olympics (olympic_id));
    diesel::joinable!(olympics -> countries (country_id));
    diesel::joinable!(players -> countries (country_id));
    diesel::joinable!(results -> events (event_id));
    diesel::joinable!(results -> players (player_id));

    diesel::allow_tables_to_appear_in_same_query!(
        countries,
        events,
        olympics,
        players,
        results,
    );
}
