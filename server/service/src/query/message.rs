use crate::Query;
use ::entity::prelude::Message;
use sea_orm::prelude::Uuid;
use sea_orm::*;

impl Query {
    pub async fn find_count_unread_messages_by_conversation_for_account(
        db: &DbConn,
        conversation: ::entity::conversation::Model,
        account_id: Uuid,
    ) -> Result<u64, DbErr> {
        let condition = Condition::all()
            .add(::entity::message::Column::ReadAt.is_not_null())
            .add(::entity::message::Column::RecipientId.eq(account_id));

        conversation
            .find_related(Message)
            .filter(condition)
            .count(db)
            .await
    }

    pub async fn find_messages_by_conversation(
        db: &DbConn,
        conversation: ::entity::conversation::Model,
        params: Option<::entity::api::message_api::MessageQueryParams>,
    ) -> Result<Vec<::entity::message::Model>, DbErr> {
        let mut query = conversation
            .find_related(Message)
            .order_by_asc(::entity::message::Column::CreatedAt)
            .cursor_by(::entity::message::Column::CreatedAt);

        if let Some(params) = params {
            let mut query_ref = &mut query;
            if let Some(limit) = params.limit {
                query_ref = query_ref.last(limit);
            }
            if let Some(before) = params.before {
                query_ref = query_ref.before(before);
            }

            return query_ref.all(db).await;
        }

        query.all(db).await
    }
}
