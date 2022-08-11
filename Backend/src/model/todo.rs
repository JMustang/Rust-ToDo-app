use sqlb::HasFields;

use super::db::Db;
use crate::{model, security::UserCtx};

// region: Todo Types
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64, // Create id
    pub title: String,
    pub status: TodoStatus, // Status
}

#[derive(sqlb::Fields, Default, Clone)]
pub struct TodoPatch {
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
sqlb::bindable!(TodoStatus);

// endregion: Todo Types

// region: TodoMac
pub struct TodoMac;

impl TodoMac {
    pub async fn create(db: &Db, utx: &UserCtx, data: TodoPatch) -> Result<Todo, model::Error> {
        let mut fields = data.fields();
        fields.push(("cid", 123).into());

        let sb = sqlb::insert()
            .table("todo")
            .data(fields)
            .returning(&["id", "cid", "title", "status"]);

        let todo = sb.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Todo>, model::Error> {
        let sb = sqlb::select()
            .table("todo")
            .columns(&["id", "cid", "title", "status"])
            .order_by("!id");

        // execute the query
        let todos = sb.fetch_all(db).await?;

        Ok(todos)
    }
}
// endregion: TodoMac

// region:    Test
#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
// endregion: Test
