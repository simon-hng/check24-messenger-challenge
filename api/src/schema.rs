// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "conversation_state"))]
    pub struct ConversationState;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "message_type"))]
    pub struct MessageType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "sender_type"))]
    pub struct SenderType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ConversationState;

    conversation (id) {
        id -> Int4,
        #[max_length = 255]
        customer_name -> Varchar,
        #[max_length = 255]
        service_provider_name -> Varchar,
        state -> ConversationState,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MessageType;
    use super::sql_types::SenderType;

    message (id) {
        id -> Int4,
        conversation_id -> Int4,
        message_type -> MessageType,
        text -> Nullable<Text>,
        sender_type -> SenderType,
        read_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(message -> conversation (conversation_id));

diesel::allow_tables_to_appear_in_same_query!(
    conversation,
    message,
);
