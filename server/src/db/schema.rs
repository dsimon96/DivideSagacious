table! {
    balance (id) {
        id -> Int4,
        node_id -> Int4,
        person_id -> Int4,
        squad_id -> Int4,
    }
}

table! {
    node (id) {
        id -> Int4,
        uid -> Uuid,
        node_type -> crate::db::models::NodeTypeMapping,
    }
}

table! {
    person (id) {
        id -> Int4,
        node_id -> Int4,
        display_name -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

table! {
    squad (id) {
        id -> Int4,
        node_id -> Int4,
        display_name -> Varchar,
    }
}

joinable!(balance -> node (node_id));
joinable!(balance -> person (person_id));
joinable!(balance -> squad (squad_id));
joinable!(person -> node (node_id));
joinable!(squad -> node (node_id));

allow_tables_to_appear_in_same_query!(
    balance,
    node,
    person,
    squad,
);
