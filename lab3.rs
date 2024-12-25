use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    due_date: String,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks("tasks.txt");
    let mut next_id = tasks.len() + 1;

    loop {
        println!("\n--- Список справ ---");
        println!("1. Показати завдання");
        println!("2. Додати завдання");
        println!("3. Редагувати завдання");
        println!("4. Видалити завдання");
        println!("5. Позначити завдання як виконане");
        println!("6. Зберегти і вийти");

        print!("Оберіть опцію (1-6): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => show_tasks(&tasks),
            "2" => {
                add_task(&mut tasks, &mut next_id);
            }
            "3" => edit_task(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => mark_task_completed(&mut tasks),
            "6" => {
                save_tasks("tasks.txt", &tasks);
                println!("Список завдань збережено. До побачення!");
                break;
            }
            _ => println!("Невірний вибір. Спробуйте ще раз."),
        }
    }
}

fn show_tasks(tasks: &[Task]) {
    println!("\n--- Ваші завдання ---");
    if tasks.is_empty() {
        println!("Список завдань порожній!");
    } else {
        for task in tasks {
            println!(
                "[{}] {} - {} (Час: {})",
                task.id,
                if task.completed { "✔" } else { " " },
                task.description,
                task.due_date
            );
        }
    }
}

fn add_task(tasks: &mut Vec<Task>, next_id: &mut usize) {
    print!("Введіть опис завдання: ");
    io::stdout().flush().unwrap();

    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    print!("Введіть час для виконання завдання (у форматі YYYY-MM-DD HH:MM): ");
    io::stdout().flush().unwrap();

    let mut due_date = String::new();
    io::stdin().read_line(&mut due_date).unwrap();

    tasks.push(Task {
        id: *next_id,
        description: description.trim().to_string(),
        completed: false,
        due_date: due_date.trim().to_string(),
    });

    *next_id += 1;
    println!("Завдання додано.");
}

fn edit_task(tasks: &mut Vec<Task>) {
    print!("Введіть ID завдання, яке хочете редагувати: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(id) = input.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
            print!("Введіть новий опис завдання: ");
            io::stdout().flush().unwrap();

            let mut new_description = String::new();
            io::stdin().read_line(&mut new_description).unwrap();

            print!("Введіть новий час виконання (у форматі YYYY-MM-DD HH:MM): ");
            io::stdout().flush().unwrap();

            let mut new_due_date = String::new();
            io::stdin().read_line(&mut new_due_date).unwrap();

            task.description = new_description.trim().to_string();
            task.due_date = new_due_date.trim().to_string();

            println!("Завдання оновлено.");
        } else {
            println!("Завдання з ID {} не знайдено.", id);
        }
    } else {
        println!("Невірний ID.");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    print!("Введіть ID завдання, яке хочете видалити: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(id) = input.trim().parse::<usize>() {
        if let Some(pos) = tasks.iter().position(|task| task.id == id) {
            tasks.remove(pos);
            println!("Завдання видалено.");
        } else {
            println!("Завдання з ID {} не знайдено.", id);
        }
    } else {
        println!("Невірний ID.");
    }
}

fn mark_task_completed(tasks: &mut Vec<Task>) {
    print!("Введіть ID завдання, яке хочете позначити як виконане: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(id) = input.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
            task.completed = true;
            println!("Завдання позначено як виконане.");
        } else {
            println!("Завдання з ID {} не знайдено.", id);
        }
    } else {
        println!("Невірний ID.");
    }
}

fn save_tasks(filename: &str, tasks: &[Task]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .unwrap();

    for task in tasks {
        writeln!(
            file,
            "{},{},{},{}",
            task.id,
            task.description,
            task.completed,
            task.due_date
        )
        .unwrap();
    }
}

fn load_tasks(filename: &str) -> Vec<Task> {
    let mut tasks = Vec::new();

    if let Ok(file) = OpenOptions::new().read(true).open(filename) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() == 4 {
                    if let (Ok(id), description, Ok(completed), due_date) = (
                        parts[0].parse::<usize>(),
                        parts[1].to_string(),
                        parts[2].parse::<bool>(),
                        parts[3].to_string(),
                    ) {
                        tasks.push(Task {
                            id,
                            description,
                            completed,
                            due_date,
                        });
                    }
                }
            }
        }
    }

    tasks
}
