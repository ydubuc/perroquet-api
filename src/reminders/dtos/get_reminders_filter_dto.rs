use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetRemindersFilterDto {
    pub id: Option<String>,
    pub search: Option<String>,
    pub limit: Option<u8>,
}

// impl GetRemindersFilterDto {
//     pub fn to_sqlx<'q>(&self) -> QueryAs<'q, Postgres, Reminder, PgArguments> {
//         let mut sql = "SELECT * FROM reminders".to_string();
//         let mut clauses = Vec::new();
//         let mut page_limit: u8 = 80;
//         let mut index: u8 = 0;

//         // WHERE CLAUSES
//         if self.id.is_some() {
//             index += 1;
//             clauses.push(format!("reminders.id = ${}", index))
//         }
//         if self.search.is_some() {
//             index += 1;
//             clauses.push(format!("reminders.body LIKE ${}", index));
//         }

//         // CLAUSES BUILDER
//         let mut has_inserted_where = false;

//         for clause in clauses {
//             if !has_inserted_where {
//                 sql.push_str(" WHERE ");
//                 has_inserted_where = true;
//             } else {
//                 sql.push_str(" AND ");
//             }

//             sql.push_str(&clause);
//         }

//         // LIMIT
//         if let Some(limit) = self.limit {
//             page_limit = limit;
//         }
//         sql.push_str(&format!(" LIMIT {}", page_limit));

//         // SQLX
//         let mut sqlx = sqlx::query_as::<Postgres, Reminder>(&sql);

//         if let Some(id) = &self.id {
//             sqlx = sqlx.bind(id);
//         }
//         if let Some(search) = &self.search {
//             sqlx = sqlx.bind(search);
//         }

//         return sqlx;
//     }
// }
