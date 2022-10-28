use diesel::table;

table! {
    #[sql_name = "Countries"]
    countries (country_id) {
        name -> Text,
        country_id -> Text,
        area_sqkm -> Integer,
        population -> Integer,
    }
}

table! {
    #[sql_name = "Olympics"]
    olympics (olympic_id) {
        olympic_id -> Text,
        country_id -> Text,
        city -> Text,
        year -> Integer,
        #[sql_name = "startdate"]
        start_date -> Date,
        #[sql_name = "enddate"]
        end_date -> Date,
    }
}

table! {
    #[sql_name = "Players"]
    players (player_id) {
        name -> Text,
        player_id -> Text,
        country_id -> Text,
        #[sql_name = "birthdate"]
        birth_date -> Date,
    }
}

table! {
    #[sql_name = "Events"]
    events (event_id) {
        event_id -> Text,
        name -> Text,
        #[sql_name = "eventtype"]
        event_type -> Text,
        olympic_id -> Text,
        is_team_event -> Bool,
        num_players_in_team -> Integer,
        result_noted_in -> Text,
    }
}

table! {
    #[sql_name = "Results"]
    results (event_id, player_id) {
        event_id -> Text,
        player_id -> Text,
        medal -> Text,
        result -> Float,
    }
}
