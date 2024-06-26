mod domains;
mod routes;

use dotenv::dotenv;
use sqlx::PgPool;

use crate::routes::create_app;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    dotenv::dotenv().ok();

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed :(");

    dotenv().ok();

    let app = create_app(pool, secrets);

    Ok(app.into())
}

// TODO: move to todos domain
//#[cfg(test)]
//mod test {
//    use super::*;
//    use crate::repositories::{test_utils::TodoRepositoryForMemory, CreateTodo, Todo};
//    use axum::{
//        body::{to_bytes, Body},
//        http::{header, Method, Request, StatusCode},
//        response::Response,
//    };
//    use tower::ServiceExt;
//
//    fn build_todo_req_with_json(path: &str, method: Method, json_body: String) -> Request<Body> {
//        Request::builder()
//            .uri(path)
//            .method(method)
//            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
//            .body(Body::from(json_body))
//            .unwrap()
//    }
//
//    fn build_todo_req_with_empty(method: Method, path: &str) -> Request<Body> {
//        Request::builder()
//            .uri(path)
//            .method(method)
//            .body(Body::empty())
//            .unwrap()
//    }
//
//    async fn res_to_todo(res: Response) -> Todo {
//        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
//        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
//        let todo: Todo = serde_json::from_str(&body)
//            .expect(&format!("cannot convert Todo instance. Body: {}", body));
//        todo
//    }
//
//    #[tokio::test]
//    async fn should_return_hello_world() {
//        let repository = TodoRepositoryForMemory::new();
//        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
//
//        let res = create_app(repository).oneshot(req).await.unwrap();
//
//        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
//
//        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
//
//        assert_eq!(body, "Hello, world!");
//    }
//
//    #[tokio::test]
//    async fn should_create_todo() {
//        let expected = Todo::new(1, "should_return_created_todo".to_string());
//
//        let repository = TodoRepositoryForMemory::new();
//        let req = build_todo_req_with_json(
//            "/todos",
//            Method::POST,
//            r#"{ "text": "should_return_created_todo" }"#.to_string(),
//        );
//
//        let res = create_app(repository).oneshot(req).await.unwrap();
//
//        let todo = res_to_todo(res).await;
//        assert_eq!(expected, todo);
//    }
//
//    #[tokio::test]
//    async fn should_find_todo() {
//        let expected = Todo::new(1, "should_find_todo".to_string());
//
//        let repository = TodoRepositoryForMemory::new();
//
//        repository
//            .create(CreateTodo::new("should_find_todo".to_string()))
//            .await
//            .expect("failed to create a todo");
//
//        let req = build_todo_req_with_empty(Method::GET, "/todos/1");
//
//        let res = create_app(repository).oneshot(req).await.unwrap();
//
//        let todo = res_to_todo(res).await;
//        assert_eq!(expected, todo);
//    }
//
//    #[tokio::test]
//    async fn should_get_all_todos() {
//        let expected = Todo::new(1, "should_get_all_todos".to_string());
//
//        let repository = TodoRepositoryForMemory::new();
//        repository
//            .create(CreateTodo::new("should_get_all_todos".to_string()))
//            .await
//            .expect("failed to create a todo");
//
//        let req = build_todo_req_with_empty(Method::GET, "/todos");
//
//        let res = create_app(repository).oneshot(req).await.unwrap();
//
//        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
//
//        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
//
//        let todo: Vec<Todo> = serde_json::from_str(&body)
//            .expect(&format!("cannot convert Todo instance. body: {}", body));
//
//        assert_eq!(vec![expected], todo);
//    }
//
//    #[tokio::test]
//    async fn should_update_todo() {
//        let expected = Todo::new(1, "should_update_todo".to_string());
//
//        let repository = TodoRepositoryForMemory::new();
//
//        repository
//            .create(CreateTodo::new("before_update_todo".to_string()))
//            .await
//            .expect("failed to create a todo");
//
//        let req = build_todo_req_with_json(
//            "/todos/1",
//            Method::PATCH,
//            r#"{
//            "id": 1,
//            "text": "should_update_todo",
//            "completed": false
//            }"#
//            .to_string(),
//        );
//
//        let res = create_app(repository).oneshot(req).await.unwrap();
//        let todo = res_to_todo(res).await;
//        assert_eq!(expected, todo);
//    }
//
//    #[tokio::test]
//    async fn should_delete_todo() {
//        let repository = TodoRepositoryForMemory::new();
//
//        repository
//            .create(CreateTodo::new("should_delete_todo".to_string()))
//            .await
//            .expect("failed to create a todo");
//        let req = build_todo_req_with_empty(Method::DELETE, "/todos/1");
//        let res = create_app(repository).oneshot(req).await.unwrap();
//        assert_eq!(StatusCode::NO_CONTENT, res.status());
//    }
//}
