use askama::Template;

use crate::repository::Repository;
use crate::templates;

use actix_web::{
    delete,
    error::{ErrorBadRequest, ErrorInternalServerError},
    get, post, put, web, HttpResponse, Responder, Result as ActixResult,
};
use serde_json::json;

#[get("/")]
async fn index(repo: web::Data<Repository>) -> ActixResult<HttpResponse> {
    let todos = repo.get_all().await.map_err(ErrorInternalServerError)?;

    let template = templates::IndexTemplate { tasks: todos };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[derive(serde::Deserialize)]
struct TaskForm {
    task: String,
}

#[post("/task")]
async fn add_task(
    user_input: web::Form<TaskForm>,
    repo: web::Data<Repository>,
) -> ActixResult<impl Responder> {
    if user_input.task.is_empty() {
        return Err(ErrorBadRequest("Task is empty"));
    }

    let todo = repo
        .insert(user_input.task.clone())
        .await
        .map_err(ErrorInternalServerError)?;


    let template = templates::TaskTemplate { task: todo };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[delete("/task/{id}")]
async fn delete_task(
    id: web::Path<i64>,
    repo: web::Data<Repository>,
) -> ActixResult<impl Responder> {
    repo.delete(id.into_inner())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("json")
        .json(json!({"ok":"task deleted"})))
}

#[delete("/tasks")]
async fn delete_done(repo: web::Data<Repository>) -> ActixResult<impl Responder> {
    repo.delete_all_done()
        .await
        .map_err(ErrorInternalServerError)?;

    let todos = repo.get_all().await.map_err(ErrorInternalServerError)?;

    let template = templates::RemainingTasksTemplate { tasks: todos };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[put("/done/{id}")]
async fn change_done(
    id: web::Path<i64>,
    repo: web::Data<Repository>,
) -> ActixResult<impl Responder> {
    let mut todo = repo
        .get_by_id(id.into_inner())
        .await
        .map_err(ErrorInternalServerError)?;

    todo.is_done = !todo.is_done;

    let updated_todo = repo.update(todo).await.map_err(ErrorInternalServerError)?;

    let template = templates::TaskTemplate { task: updated_todo };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/edit/{id}")]
async fn edit_task(id: web::Path<i64>, repo: web::Data<Repository>) -> ActixResult<impl Responder> {
    let todo = repo
        .get_by_id(id.into_inner())
        .await
        .map_err(ErrorInternalServerError)?;

    let template = templates::EditTaskTemplate { task: todo };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[put("/update/{id}")]
async fn update_task(
    id: web::Path<i64>,
    user_input: web::Form<TaskForm>,
    repo: web::Data<Repository>,
) -> ActixResult<impl Responder> {
    if user_input.task.is_empty() {
        return Err(ErrorBadRequest("Task is empty"));
    }

    let mut todo = repo
        .get_by_id(id.into_inner())
        .await
        .map_err(ErrorInternalServerError)?;
    todo.text = user_input.task.clone();

    let updated_todo = repo.update(todo).await.map_err(ErrorInternalServerError)?;

    let template = templates::TaskTemplate { task: updated_todo };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
