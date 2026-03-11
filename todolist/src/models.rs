//! models.rs : Défini ce qu'est une tâche et une liste de tâches.
//! Contient la logique CRUD.

use serde::{Serialize, Deserialize};

/// Structure pour une tâche
#[derive(Debug, Serialize, Deserialize)]
pub struct Task{
    pub id: usize,
    pub title:String,
    pub description:String,
    pub state: TaskState,
}

/// Enumeration pour le type `TaskState` utiliser dans la structure `Task`
#[derive(Debug, Serialize, Deserialize)]
pub enum TaskState{
    Done,
    InProgress,
    Failed,
}

/// Implémentation de la structure `Task`
impl Task{
    /// new : donne vie la structure `Task`
    /// # Param
    /// - `title` String : le nom de la tâche a créé.
    /// - `description` String : la description de la tâche.
    /// # Return
    /// - `Task` : la nouvelle tâche.
    pub fn new(id:usize, title:String,description:String, state:TaskState)->Self{
        Task{
            id,
            title,
            description,
            state,
        }
    }
}