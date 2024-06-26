use shared::todos::repository::TodoRepositoryTrait;
use shared::todos::service::TodoServiceTrait;

// TODO: should be depended on only service..?
#[derive(Clone)]
pub struct TodoDependency<TS, TR>
where
    TS: TodoServiceTrait,
    TR: TodoRepositoryTrait,
{
    pub todo_repository: TR,
    pub todo_service: TS,
}
