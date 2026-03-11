//! models.rs : Défini ce qu'est une tâche et une liste de tâches.
//! Contient la logique CRUD.

struct Task{
    title:String,
    description:String,
    state: TaskState,
}

enum TaskState{
    Done,
    InProgress,
    Failed,
}

