use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

use crate::types::question::{Question, QuestionId};
use handle_errors::Error;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("DBに接続できない: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<Question>, Error> {
        match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct Store {
//     pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
//     pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
// }

// impl Store {
//     pub fn new() -> Self {
//         Store {
//             questions: Arc::new(RwLock::new(Self::init())),
//             answers: Arc::new(RwLock::new(HashMap::new())),
//         }
//     }

//     fn init() -> HashMap<QuestionId, Question> {
//         let file = include_str!("../questions.json");
//         serde_json::from_str(file).expect("can't read questions")
//     }
// }
