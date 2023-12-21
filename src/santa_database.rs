use std::sync::Arc;

use axum::{
    extract::{Json, State},
    response::{self, IntoResponse},
    routing::{get, post},
    Router,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};

use crate::AppState;

#[derive(Deserialize, Serialize, Debug)]
struct Order {
    id: i16,
    region_id: i16,
    gift_name: String,
    quantity: i16,
}

enum OrderCreationError {
    Database(String),
    Internal(String),
}

#[derive(Serialize, Debug)]
struct CreationErrorDto {
    message: String,
    orders: Vec<(Order, String)>,
}

#[derive(Serialize, FromRow)]
struct TotalOrders {
    total: i64,
}

#[derive(Serialize)]
struct MostPopularResponse {
    popular: Option<String>,
}

impl Order {
    async fn create(&self, pool: &PgPool) -> Result<(), OrderCreationError> {
        let result = sqlx::query(
            "INSERT INTO orders(id, region_id, gift_name, quantity) VALUES($1, $2, $3, $4)",
        )
        .bind(self.id)
        .bind(self.region_id)
        .bind(&self.gift_name)
        .bind(self.quantity)
        .fetch_optional(pool)
        .await;

        if let Err(e) = result {
            match e {
                sqlx::Error::Database(e) => {
                    println!("{}", e.message());
                    return Err(OrderCreationError::Database(e.message().into()));
                }
                _ => {
                    println!("{e:#?}");
                    return Err(OrderCreationError::Internal(
                        "Creation failed from the server".into(),
                    ));
                }
            }
        }

        Ok(())
    }
}

async fn sql_handler(
    State(pool): State<PgPool>,
) -> response::Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query("SELECT 20231213").fetch_one(&pool).await {
        Ok(result) => Ok((StatusCode::OK, result.get::<i32, usize>(0).to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn reset_database(State(pool): State<PgPool>) -> StatusCode {
    let drop_query = "DROP TABLE IF EXISTS orders";
    let create_query =
        "CREATE TABLE orders (id INT PRIMARY KEY,region_id INT,gift_name VARCHAR(50),quantity INT)";

    match sqlx::query(drop_query).execute(&pool).await {
        Ok(_) => match sqlx::query(create_query).execute(&pool).await {
            Ok(_) => StatusCode::OK,
            Err(_) => {
                println!("Failed create orders");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        },
        Err(_) => {
            println!("Failed delete orders");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn get_total_orders(State(pool): State<PgPool>) -> response::Result<Json<TotalOrders>> {
    let query =
        sqlx::query_as::<_, TotalOrders>("SELECT COALESCE(SUM(quantity),0) AS total FROM orders");
    let result = query.fetch_one(&pool).await;

    match result {
        Ok(total) => Ok(Json(total)),
        Err(_) => Err("Something was wrong".into()),
    }
}

async fn create_orders(
    State(pool): State<PgPool>,
    Json(orders): Json<Vec<Order>>,
) -> response::Result<(), (StatusCode, Json<CreationErrorDto>)> {
    let pool = Arc::new(pool);
    let queries = orders
        .into_iter()
        .map(|order| {
            let pool = Arc::clone(&pool);
            tokio::spawn(async move { order.create(&pool).await.map_err(|e| (order, e)) })
        })
        .collect::<Vec<_>>();

    let mut database_error_counter = 0;
    let mut failed_orders = CreationErrorDto {
        message: "The following orders failed:".to_string(),
        orders: Vec::new(),
    };

    for query in queries {
        if let Err((order, error)) = query.await.unwrap() {
            match error {
                OrderCreationError::Database(message) => {
                    database_error_counter += 1;
                    failed_orders.orders.push((order, message));
                }
                OrderCreationError::Internal(message) => {
                    failed_orders.orders.push((order, message));
                }
            }
        }
    }

    if database_error_counter > 0 {
        Err((StatusCode::BAD_REQUEST, Json(failed_orders)))
    } else if !failed_orders.orders.is_empty() {
        Err((StatusCode::INTERNAL_SERVER_ERROR, Json(failed_orders)))
    } else {
        Ok(())
    }
}

async fn get_most_popular(State(pool): State<PgPool>) -> Json<MostPopularResponse> {
    let query = sqlx::query_as::<_, (String, i64)>(
        "SELECT gift_name, SUM(quantity) AS total FROM orders GROUP BY gift_name",
    );
    let total_orders_by_name = query.fetch_all(&pool).await.unwrap();
    let most_popular = total_orders_by_name.iter().max_by_key(|(_, total)| total);

    match most_popular {
        Some((name, max_total)) => {
            let max_orders = total_orders_by_name
                .iter()
                .filter(|(_, total)| total == max_total)
                .count();

            if max_orders == 1 {
                Json(MostPopularResponse {
                    popular: Some(name.clone()),
                })
            } else {
                Json(MostPopularResponse { popular: None })
            }
        }
        None => Json(MostPopularResponse { popular: None }),
    }
}

pub fn make_santa_database_api() -> Router<AppState> {
    Router::new()
        .route("/sql", get(sql_handler))
        .route("/reset", post(reset_database))
        .route("/orders", post(create_orders))
        .route("/orders/total", get(get_total_orders))
        .route("/orders/popular", get(get_most_popular))
}
