use clap::{Parser, Subcommand};
use sqlx::mysql::MySqlPool;
use std::env;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Add { description: String },
    Done { id: u64 },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        Some(Command::Add { description }) => {
            println!("Adding new todo with description '{description}'");
            let todo_id = add_todo(&pool, description).await?;
            println!("Added new todo with id {todo_id}");
        }
        Some(Command::Done { id }) => {
            println!("Marking todo {id} as done");
            if complete_todo(&pool, id).await? {
                println!("Todo {id} is marked as done");
            } else {
                println!("Invalid id {id}");
            }
        }
        None => {
            println!("Printing list of all todos");
            list_todos(&pool).await?;
        }
    }

    Ok(())
}

async fn add_todo(pool: &MySqlPool, description: String) -> anyhow::Result<u64> {
    // Insert the task, then obtain the ID of this row
    let todo_id = sqlx::query(
        r#"
INSERT INTO todos ( description, done )
VALUES ( ?, FALSE )
        "#
    )
    .bind(description)
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(todo_id)
}

async fn complete_todo(pool: &MySqlPool, id: u64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query(
        r#"
UPDATE todos
SET done = TRUE
WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

#[derive(sqlx::FromRow, Debug)]
struct TodosRow {
    id: u64,
    description: String,
    done: bool,
}

async fn list_todos(pool: &MySqlPool) -> anyhow::Result<()> {
    let recs = sqlx::query_as::<_, TodosRow>(
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    println!("Listing all todos:");
    for todo in &recs {
        println!("ID: {}, Description: {}, Done: {}", todo.id, todo.description, todo.done);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "./migrations", fixtures("test_list_todos"))]
    async fn test_list_todos(pool: MySqlPool) -> anyhow::Result<()> {
        // test list_todos
        list_todos(&pool).await?;
        
        // Verify that the test was successful
        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_complete_todo"))]
    async fn test_complete_todo(pool: MySqlPool) -> anyhow::Result<()> {
        // 1. Get an incomplete task
        let todo_id = 1; // ID of 'Fix bug in login form'
        
        // 2. Verify that it's initially incomplete
        let todos_before = sqlx::query_as::<_, TodosRow>("SELECT * FROM todos WHERE id = ?")
            .bind(todo_id)
            .fetch_one(&pool)
            .await?;
        assert_eq!(todos_before.done, false);
        
        // 3. Execute the complete_todo function
        let result = complete_todo(&pool, todo_id).await?;
        
        // 4. Verify that the result returns success (true)
        assert_eq!(result, true);
        
        // 5. Verify that the task status changed to completed
        let todos_after = sqlx::query_as::<_, TodosRow>("SELECT * FROM todos WHERE id = ?")
            .bind(todo_id)
            .fetch_one(&pool)
            .await?;
        assert_eq!(todos_after.done, true);
        
        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_add_todo"))]
    async fn test_add_todo(pool: MySqlPool) -> anyhow::Result<()> {
        // 1. Description for the task to be added
        let description = "Test new task";
        
        // 2. Verify that the task doesn't exist initially
        let todos_before = sqlx::query_as::<_, TodosRow>("SELECT * FROM todos")
            .fetch_all(&pool)
            .await?;
        assert_eq!(todos_before.len(), 0);
        
        // 3. Execute the add_todo function
        let todo_id = add_todo(&pool, description.to_string()).await?;
        
        // 4. Verify that the returned ID is as expected
        assert_eq!(todo_id, 1);
        
        // 5. Verify that the task was added
        let todos_after = sqlx::query_as::<_, TodosRow>("SELECT * FROM todos")
            .fetch_all(&pool)
            .await?;
        assert_eq!(todos_after.len(), 1);
        assert_eq!(todos_after[0].description, description);
        assert_eq!(todos_after[0].done, false);
        
        Ok(())
    }
}