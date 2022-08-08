use super::db::Db;
use crate::model;

// region: Todo Types
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64, // Create id
    pub title: String,
    pub status: TodoStatus, // Status
}

#[derive(Default, Clone)]
pub struct TodoPatch {
    pub cid: Option<i64>,
    pub title: Option<String>,
    pub status: Option<TodoStatus>, // Status
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Close,
}

// endregion: Todo Types

// region: TodoMac
pub struct TodoMac;

impl TodoMac {
    pub async fn create(db: &Db, data: TodoPatch) -> Result<Todo, model::Error> {
        let sql = "INSERT INTO Todo (cid, title) VALUES ($1, $2) returning id, cid, title, status";
        let query = sqlx::query_as::<_, Todo>(&sql)
            .bind(123 as i64)
            .bind(data.title.unwrap_or_else(|| "untitled".to_string()));

        let todo = query.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn list(db: &Db) -> Result<Vec<Todo>, model::Error> {
        let sql = "SELECT id, cid, title, status FROM todo ORDER BY id DESC";

        // build the sqlx-query
        let query = sqlx::query_as(&sql);
        // execute the query
        let todos = query.fetch_all(db).await?;

        Ok(todos)
    }
}
// endregion: TodoMac

// region:    Test
#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
// endregion: Test
