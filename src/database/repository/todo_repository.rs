use crate::api::model::todo::Todo;
use sqlx::SqlitePool;

pub async fn create(
    pool: &SqlitePool,
    title: &str,
    description: Option<&str>,
) -> Result<Todo, sqlx::Error> {
    let result = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, description) VALUES (?, ?) RETURNING id, title, description, completed, created_at, updated_at",
    )
    .bind(title)
    .bind(description)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at, updated_at FROM todos ORDER BY id",
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn find_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Todo>, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>(
        "SELECT id, title, description, completed, created_at, updated_at FROM todos WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(todo)
}

pub async fn update(
    pool: &SqlitePool,
    id: i64,
    title: &str,
    description: Option<&str>,
    completed: bool,
) -> Result<Option<Todo>, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET title = ?, description = ?, completed = ?, updated_at = strftime('%Y-%m-%dT%H:%M:%S', 'now') WHERE id = ? RETURNING id, title, description, completed, created_at, updated_at",
    )
    .bind(title)
    .bind(description)
    .bind(completed)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(todo)
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn setup_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn create_returns_todo_with_title() {
        let pool = setup_pool().await;
        let todo = create(&pool, "Test todo", None).await.unwrap();
        assert_eq!(todo.title(), "Test todo");
        assert!(todo.description().is_none());
        assert!(!todo.completed());
    }

    #[tokio::test]
    async fn create_returns_todo_with_description() {
        let pool = setup_pool().await;
        let todo = create(&pool, "Test", Some("A description")).await.unwrap();
        assert_eq!(todo.description(), Some("A description"));
    }

    #[tokio::test]
    async fn find_all_returns_empty_when_no_todos() {
        let pool = setup_pool().await;
        let todos = find_all(&pool).await.unwrap();
        assert!(todos.is_empty());
    }

    #[tokio::test]
    async fn find_all_returns_all_todos() {
        let pool = setup_pool().await;
        create(&pool, "First", None).await.unwrap();
        create(&pool, "Second", None).await.unwrap();
        let todos = find_all(&pool).await.unwrap();
        assert_eq!(todos.len(), 2);
    }

    #[tokio::test]
    async fn find_by_id_returns_existing_todo() {
        let pool = setup_pool().await;
        let created = create(&pool, "Find me", None).await.unwrap();
        let found = find_by_id(&pool, created.id()).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().title(), "Find me");
    }

    #[tokio::test]
    async fn find_by_id_returns_none_for_missing() {
        let pool = setup_pool().await;
        let found = find_by_id(&pool, 999).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn update_modifies_existing_todo() {
        let pool = setup_pool().await;
        let created = create(&pool, "Original", None).await.unwrap();
        let updated = update(&pool, created.id(), "Updated", Some("desc"), true)
            .await
            .unwrap();
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.title(), "Updated");
        assert_eq!(updated.description(), Some("desc"));
        assert!(updated.completed());
    }

    #[tokio::test]
    async fn update_returns_none_for_missing() {
        let pool = setup_pool().await;
        let result = update(&pool, 999, "Nope", None, false).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_removes_existing_todo() {
        let pool = setup_pool().await;
        let created = create(&pool, "Delete me", None).await.unwrap();
        let deleted = delete(&pool, created.id()).await.unwrap();
        assert!(deleted);
        let found = find_by_id(&pool, created.id()).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn delete_returns_false_for_missing() {
        let pool = setup_pool().await;
        let deleted = delete(&pool, 999).await.unwrap();
        assert!(!deleted);
    }
}
