//! A message in transit and at rest. Should be serializable to JSON.

use chrono::prelude::{DateTime, Utc};
use diesel::{
    backend::Backend, serialize, serialize::Output, sql_types::Timestamptz, types::ToSql,
};
use serde::{Deserialize, Serialize};

use crate::{
    conversation::ConversationId,
    user::{User, UserId},
};

use crate::schema::messages;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug)]
pub struct Message {
    id: MessageId,
    from: UserId,
    to: ConversationId,
    body: String,
    timestamp: DateTime<Utc>,
}

// impl<DB: Backend> ToSql<Timestamptz, DB> for DateTime<Utc>
//     where
//         i16: ToSql<Timestamptz, DB>,
//     {
//         fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
//         where
//             W: std::io::Write,
//         {
//             self.to_string()
//             // let v = match *self {
//             //     RecordType::A => 1,
//             //     RecordType::B => 2,
//             // };
//             // v.to_sql(out)
//         }
//     }

#[derive(DieselNewType, Serialize, Deserialize, Debug)]
pub struct MessageId(i32);
