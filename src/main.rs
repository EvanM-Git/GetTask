use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
    due_date: Option<String>,
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), std::io::Error> {
    let data = serde_json::to_string(tasks)?;
    fs::write("tasks.json", data)?;
    Ok(())
}

fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn add_task(tasks: &mut Vec<Task>, description: String, due_date: Option<String>) {
    let id = tasks.len() as u32 + 1;
    tasks.push(Task {
        id,
        description,
        completed: false,
        due_date,
    });
}

fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        println!(
            "ID: {}, Descripción: {}, Completada: {}, Fecha Límite: {:?}",
            task.id, task.description, task.completed, task.due_date
        );
    }
}

fn complete_task(tasks: &mut Vec<Task>, id: u32) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
    }
}

fn delete_task(tasks: &mut Vec<Task>, id: u32) {
    tasks.retain(|t| t.id != id);
}

fn main() {
    let mut tasks = load_tasks();

    loop {
        println!("1. Add a task");
        println!("2. List tasks");
        println!("3. Mark a task completed");
        println!("4. Delete a task");
        println!("5. Exit");

        print!("Selecciona una opción: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                print!("Descripción: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                print!("Fecha límite (opcional): ");
                io::stdout().flush().unwrap();
                let mut due_date = String::new();
                io::stdin().read_line(&mut due_date).unwrap();

                add_task(
                    &mut tasks,
                    description.trim().to_string(),
                    Some(due_date.trim().to_string()),
                );
            }
            2 => list_tasks(&tasks),
            3 => {
                print!("ID de la tarea a completar: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                let id: u32 = id_input.trim().parse().unwrap_or(0);

                complete_task(&mut tasks, id);
            }
            4 => {
                print!("ID de la tarea a eliminar: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                let id: u32 = id_input.trim().parse().unwrap_or(0);

                delete_task(&mut tasks, id);
            }
            5 => break,
            _ => println!("Opción no válida"),
        }

        if let Err(e) = save_tasks(&tasks) {
            eprintln!("Error al guardar tareas: {}", e);
        }
    }
}
