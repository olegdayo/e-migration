use diesel::table;

table! {
    #[sql_name = "Countries"]
    countries (country_id) {
        name -> Varchar,
        country_id -> Varchar,
        area_sqkm -> Integer,
        population -> Integer,
    }
}

table! {
    #[sql_name = "Olympics"]
    olympics (olympic_id) {
        olympic_id -> Varchar,
        country_id -> Varchar,
        city -> Varchar,
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
        name -> Varchar,
        player_id -> Varchar,
        country_id -> Varchar,
        #[sql_name = "birthdate"]
        birth_date -> Date,
    }
}

table! {
    #[sql_name = "Events"]
    events (event_id) {
        event_id -> Varchar,
        name -> Varchar,
        #[sql_name = "eventtype"]
        event_type -> Varchar,
        olympic_id -> Varchar,
        is_team_event -> Bool,
        num_players_in_team -> Integer,
        result_noted_in -> Varchar,
    }
}

table! {
    #[sql_name = "Results"]
    results (event_id, player_id) {
        event_id -> Varchar,
        player_id -> Varchar,
        medal -> Varchar,
        result -> Float,
    }
}
