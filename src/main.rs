use std::io::{Read, Write, BufRead};

fn main() {
    let mut tasks : Vec<Task> = vec![];

    'running: loop{
        println!("-------Todo App-------");
        if tasks.len() == 0 {
            println!("You don't have any tasks, try to create one!");
        }
        tasks.iter().enumerate().for_each(|(i,task)|{
            println!(
                "#{} [{}] {}",
                i + 1,
                match task.completed {
                    false => " ",
                    true => "X"
                },
                task.text
            )
        });
        
        print!("(A)dd, (D)elete, (E)dit, (C)ompleted, (Q)uit: ");
        std::io::stdout().flush();
        let action = std::io::stdin().lock().lines().next()
                .and_then(|r|r.ok())
                .map(|s| s.chars().nth(0).unwrap_or('\n'));
        match action
        {
            Some('A') | Some('a') => {
                print!("Task title: ");
                std::io::stdout().flush();
                let mut taskText = String::new();
                std::io::stdin().read_line(&mut taskText);
                tasks.push(Task {
                    text: taskText.trim().to_string(),
                    completed: false,
                });
            },
            Some('D') | Some('d') => {
                print!("Task id: ");
                std::io::stdout().flush();
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf);
                tasks.remove(buf.trim().parse::<usize>().unwrap() - 1);
            },
            Some('E') | Some('e') => {
                print!("Task id: ");
                std::io::stdout().flush();
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf);
                let index = match buf.trim().parse::<usize>(){
                    Ok(id) => id - 1,
                    Err(e) => {
                        println!("Please enter a valid number!");
                        continue 'running;
                    }
                };
                if index >= tasks.len() {
                    println!("Please enter a valid id!");
                    continue 'running;
                }
                buf.clear();
                print!("Task title: ");
                std::io::stdout().flush();
                std::io::stdin().read_line(&mut buf);
                tasks.get_mut(index).unwrap().text = buf.trim().to_string();
            },
            Some('C') | Some('c') => {
                print!("Task id: ");
                std::io::stdout().flush();
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf);
                let index = match buf.trim().parse::<usize>(){
                    Ok(id) => id - 1,
                    Err(e) => {
                        println!("Please enter a valid number!");
                        continue 'running;
                    }
                };
                if index >= tasks.len() {
                    println!("Please enter a valid id!");
                    continue 'running;
                }
                tasks.get_mut(index).unwrap().completed = true;
            },
            Some('Q') | Some('q') => break 'running,
            _ => {}
        }
        println!();
    }
}

struct Task {
    text: String,
    completed: bool,
}
