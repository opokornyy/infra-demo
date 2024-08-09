use askama::Template;

use crate::repository::Todo;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub tasks: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "remaining-tasks.html")]
pub struct RemainingTasksTemplate {
    pub tasks: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "task.html")]
pub struct TaskTemplate {
    pub task: Todo,
}

#[derive(Template)]
#[template(path = "edit-task.html")]
pub struct EditTaskTemplate {
    pub task: Todo,
}
