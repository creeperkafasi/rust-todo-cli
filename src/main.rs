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
                let mut taskText = String::new();
                std::io::stdin().read_line(&mut taskText);
                tasks.push(Task {
                    text: taskText.trim_end().to_string(),
                    completed: false,
                });
            },
            Some('D') | Some('d') => {},
            Some('E') | Some('e') => {},
            Some('C') | Some('c') => {},
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
