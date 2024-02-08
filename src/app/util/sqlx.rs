// pub struct QueryBuilder {
//     select: String,
//     from: String,
//     where_clause: Option<String>,
// }

// impl QueryBuilder {
//     pub fn new() -> Self {
//         QueryBuilder {
//             select: String::new(),
//             from: String::new(),
//             where_clause: None,
//         }
//     }

//     pub fn select(&mut self, columns: &str) -> &mut Self {
//         self.select = format!("SELECT {}", columns);
//         self
//     }

//     pub fn from(&mut self, table: &str) -> &mut Self {
//         self.from = format!("FROM {}", table);
//         self
//     }

//     fn where_clause<T>(&mut self, field: &str, value: T) -> &mut Self
//     where
//         T: ToString,
//     {
//         let condition = format!("{} = {}", field, value.to_string());
//         self.where_clause = Some(condition);
//         self
//     }

//     pub fn build(&self) -> String {
//         let mut query = format!("{} {}", self.select, self.from);
//         if let Some(where_clause) = &self.where_clause {
//             query.push_str(&format!(" {}", where_clause));
//         }
//         query
//     }
// }
