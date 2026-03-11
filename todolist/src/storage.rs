//! storage.rs : contient les instructions pour ouvrir un fichier texte pour lire et écrire.

use crate::models::{Task, TaskState};
use std::fs;

const FILE_PATH: &str = "todo_list.json";


/// Permet de save les taches dans un fichier JSON
/// # Param
/// - `tasks` &Vec<Task> : vecteur de tâches
pub fn save_task(tasks: &Vec<Task>){
    let json_text = serde_json::to_string_pretty(tasks).expect("Error serializing");

    fs::write(FILE_PATH, json_text).expect("Error writing file");
}

/// Permet de chargé les tâche depuis un fichier JSON
/// # Return
/// - Vec<Task> : un vecteur de tâches
pub fn load_tasks() -> Vec<Task>{
    let file_contents = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| String::from("[]"));

    let tasks : Vec<Task> = serde_json::from_str(&file_contents).unwrap_or_else(|_| Vec::new());

    tasks
}

/// Permet de supprimer une tâche via sont id.
/// # Param
/// - `id` usize : l'id de la tâche
pub fn delete_task(id: usize){
    let mut tasks = load_tasks();
    tasks.retain(|task| task.id != id);
    save_task(&tasks);
}

pub fn change_state(id: usize, new_state: TaskState){
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id){
        task.state = new_state;
        save_task(&tasks);
    } else {
        println!("Erreur : la tache {} n'existe pas", id);

    }
}