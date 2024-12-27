use std::io::{self, Write};
use rusqlite::{Connection, params};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write as IoWrite};

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

fn main() {
    // Підключення до бази даних
    let conn = Connection::open("users.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    ).unwrap();

    loop {
        println!("1. Авторизація");
        println!("2. Реєстрація");
        print!("Виберіть опцію: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                // Авторизація
                println!("Введіть ім'я користувача:");
                let username = read_input();
                println!("Введіть пароль:");
                let password = read_input();

                if authenticate_user(&conn, &username, &password) {
                    println!("Авторизація успішна!");
                    main_program(); // Перехід до основної частини програми
                    break;
                } else {
                    println!("Невірний логін або пароль. Завершення програми.");
                    break;
                }
            }
            "2" => {
                // Реєстрація
                println!("Введіть ім'я користувача:");
                let username = read_input();
                println!("Введіть пароль:");
                let password = read_input();

                if register_user(&conn, &username, &password) {
                    println!("Реєстрація успішна!");
                } else {
                    println!("Користувач з таким ім'ям вже існує.");
                }
            }
            _ => println!("Невірна опція! Спробуйте ще раз."),
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Функція для авторизації користувача
fn authenticate_user(conn: &Connection, username: &str, password: &str) -> bool {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username = ?1 AND password = ?2").unwrap();
    let count: i64 = stmt.query_row(params![username, password], |row| row.get(0)).unwrap();
    count > 0
}

// Функція для реєстрації користувача
fn register_user(conn: &Connection, username: &str, password: &str) -> bool {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username = ?1").unwrap();
    let count: i64 = stmt.query_row(params![username], |row| row.get(0)).unwrap();

    if count > 0 {
        return false;  // Користувач вже існує
    }

    conn.execute("INSERT INTO users (username, password) VALUES (?1, ?2)", params![username, password]).unwrap();
    true
}

fn main_program() {
    let mut tasks = load_tasks("tasks.txt");
    let mut next_id = tasks.len() as u32 + 1;

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
            "2" => add_task(&mut tasks, &mut next_id),
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
    for task in tasks {
        println!("{:?}", task);
    }
}

fn add_task(tasks: &mut Vec<Task>, next_id: &mut u32) {
    println!("Введіть опис завдання:");
    let description = read_input();
    tasks.push(Task {
        id: *next_id,
        description,
        completed: false,
    });
    *next_id += 1;
}

fn edit_task(tasks: &mut Vec<Task>) {
    println!("Введіть ID завдання, яке хочете редагувати:");
    let id: u32 = read_input().parse().unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        println!("Введіть новий опис завдання:");
        let description = read_input();
        task.description = description;
    } else {
        println!("Завдання з таким ID не знайдено.");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    println!("Введіть ID завдання, яке хочете видалити:");
    let id: u32 = read_input().parse().unwrap();
    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        println!("Завдання видалено.");
    } else {
        println!("Завдання з таким ID не знайдено.");
    }
}

fn mark_task_completed(tasks: &mut Vec<Task>) {
    println!("Введіть ID завдання, яке хочете позначити як виконане:");
    let id: u32 = read_input().parse().unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        println!("Завдання позначено як виконане.");
    } else {
        println!("Завдання з таким ID не знайдено.");
    }
}

fn save_tasks(filename: &str, tasks: &[Task]) {
    let mut file = OpenOptions::new().create(true).append(true).open(filename).unwrap();
    for task in tasks {
        writeln!(file, "{},{},{}", task.id, task.description, task.completed).unwrap();
    }
}

fn load_tasks(filename: &str) -> Vec<Task> {
    let mut tasks = Vec::new();
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return tasks,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let id = parts[0].parse().unwrap();
            let description = parts[1].to_string();
            let completed = parts[2].parse().unwrap();
            tasks.push(Task { id, description, completed });
        }
    }

    tasks
}
