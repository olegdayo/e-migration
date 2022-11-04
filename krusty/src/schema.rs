// @generated automatically by Diesel CLI.

pub mod olympics {
    diesel::table! {
        olympics.countries (country_id) {
            name -> Nullable<Bpchar>,
            country_id -> Bpchar,
            area_sqkm -> Nullable<Int4>,
            population -> Nullable<Int4>,
        }
    }

    diesel::table! {
        olympics.events (event_id) {
            event_id -> Bpchar,
            name -> Nullable<Bpchar>,
            eventtype -> Nullable<Bpchar>,
            olympic_id -> Nullable<Bpchar>,
            is_team_event -> Nullable<Int4>,
            num_players_in_team -> Nullable<Int4>,
            result_noted_in -> Nullable<Bpchar>,
        }
    }

    diesel::table! {
        olympics.olympics (olympic_id) {
            olympic_id -> Bpchar,
            country_id -> Nullable<Bpchar>,
            city -> Nullable<Bpchar>,
            year -> Nullable<Int4>,
            startdate -> Nullable<Date>,
            enddate -> Nullable<Date>,
        }
    }

    diesel::table! {
        olympics.players (player_id) {
            name -> Nullable<Bpchar>,
            player_id -> Bpchar,
            country_id -> Nullable<Bpchar>,
            birthdate -> Nullable<Date>,
        }
    }

    diesel::table! {
        olympics.results (event_id, player_id) {
            event_id -> Bpchar,
            player_id -> Bpchar,
            medal -> Nullable<Bpchar>,
            result -> Nullable<Float8>,
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
