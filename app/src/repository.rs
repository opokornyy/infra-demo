use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, migrate::MigrateDatabase, MySql};
use std::io::{self, Write};
use base64::engine::general_purpose;
use base64::Engine;  // Import the Engine trait
use std::str;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub is_done: bool,
}

#[derive(Clone)]
pub struct Repository {
    pool: sqlx::MySqlPool,
}

impl Repository {
    pub async fn try_init() -> anyhow::Result<Repository> {
        let encoded_url =
            std::env::var("DATABASE_URL").expect("You probably forgot to create .env file");

        let decoded_bytes = general_purpose::STANDARD.decode(encoded_url).expect("Failed to decode base64");
            
        // Convert bytes to string
        let database_url = str::from_utf8(&decoded_bytes).expect("Failed to convert bytes to string");
        

        println!("Database url: {}", database_url);
        io::stdout().flush().unwrap();


        if !MySql::database_exists(&database_url).await? {
            MySql::create_database(&database_url).await?;
        }

        let pool = MySqlPoolOptions::new()
            .max_connections(5) // Set your desired maximum connections
            .connect(&database_url)
            .await?;

        sqlx::migrate!("./db/migrations").run(&pool).await?;

        Ok(Repository { pool })
    }

    pub async fn insert(&self, text: String) -> Result<Todo, sqlx::Error> {
        let todo = sqlx::query(
            r#"
            INSERT INTO todos (text)
            VALUES (?)
            "#,
        )
        .bind(text.clone())
        .execute(&self.pool)
        .await?;

        // Fetch the inserted row by its id
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            SELECT id, text, is_done
            FROM todos
            WHERE text = ?
            ORDER BY id DESC
            LIMIT 1
            "#,
        )
        .bind(text) // Bind the text to retrieve the correct row
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let todos = sqlx::query_as(
            r#"
            SELECT id, text, is_done
            FROM todos
            ORDER BY id DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(todos)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Todo, sqlx::Error> {
        let todo = sqlx::query_as(
            r#"
            SELECT id, text, is_done
            FROM todos
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    pub async fn update(&self, todo: Todo) -> Result<Todo, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE todos
            SET text = ?, is_done = ?
            WHERE id = ?
            "#,
        )
        .bind(&todo.text)
        .bind(todo.is_done)
        .bind(todo.id)
        .execute(&self.pool)
        .await?;

        Ok(todo)
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM todos
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_all_done(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM todos
            WHERE is_done = 1
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}