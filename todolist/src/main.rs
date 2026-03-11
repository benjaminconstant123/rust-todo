use crate::models::{Task, TaskState};

mod cli;
mod models;
mod storage;

fn main() {
    cli::start();
}