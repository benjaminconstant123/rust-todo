//! cli.rs : permet d'analyser ce que l'utilisateur a tapé
use crate::storage;
use crate::models;
use std::env;
use crate::models::{Task, TaskState};


pub fn start(){
    //env::args() recup les arguments et les ranges dans le vecteur
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: ");
        println!("cargo run add \"Titre\" \"Description\"");
        println!("cargo run remove <id>");
        println!("cargo run add \"Titre\" \"Description\"");
        println!("cargo run list");
    }

    let commande = args[1].as_str();

    match commande {
        "add" => {
            if args.len() < 4 {
                println!("Erreur : Il manque le titre ou la description");
                return;
            }

            let titre = args[2].as_str();
            let description = args[3].as_str();

            let mut list = storage::load_tasks();
            let new_id = list.iter().map(|t| t.id).max().unwrap_or(0)+ 1;
            let new_task = Task::new(new_id, titre.to_string(), description.to_string(), TaskState::InProgress);

            list.push(new_task);
            storage::save_task(&list);
            println!("Tache ajoutee avec succes ! (ID : {} )", new_id);
        }

        "list" => {
            let tasks = storage::load_tasks();
            println!("----LISTE----");
            println!("tasks: {:#?}", tasks);
        }

        _ => println!("Erreur : commande inconnue : {}", commande),
    }



    println!("Debug CLI : {:?}", args);
}