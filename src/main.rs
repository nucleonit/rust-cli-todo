use clearscreen;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use owo_colors::OwoColorize;
use owo_colors::colors::White;
use std::process::exit;
use std::time::Duration;

fn main() {
    let mut tasks = Tasker::default();
    tasks.main_menu().unwrap();
}

struct Tasker {
    accomplished: Vec<String>,
    unfulled: Vec<String>,
}

impl Default for Tasker {
    fn default() -> Self {
        Self {
            accomplished: vec![],
            unfulled: vec![],
        }
    }
}

impl Tasker {
    fn main_menu(&mut self) -> std::io::Result<()> {
        let mut index = 0;
        const MAX_ITEMS: usize = 4;
        enable_raw_mode()?;
        Self::raw_menu(index);
        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Down | KeyCode::PageDown => {
                            if index < MAX_ITEMS - 1 {
                                index += 1;
                            } else {
                                index = 0;
                            }
                            Self::raw_menu(index);
                        }
                        KeyCode::Up | KeyCode::PageUp => {
                            if index > 0 {
                                index -= 1;
                            } else {
                                index = MAX_ITEMS - 1;
                            }
                            Self::raw_menu(index);
                        }
                        KeyCode::Enter => {
                            if index == 0 {
                                disable_raw_mode()?;
                                let t_name = my_io::input("\rВведите название задачи: ")
                                    .trim()
                                    .to_string();
                                self.add_task(&t_name);
                                enable_raw_mode()?;
                            } else if index == 1 {
                                disable_raw_mode()?;
                                self.print_uf_vector();
                                let index_str = my_io::input("\rВведите индекс задачи: ");
                                match index_str.trim().parse::<usize>() {
                                    Ok(_) => {
                                        print!("\r[ LOG ] {}", "Успешно\n".to_string().green());
                                        match self
                                            .complete(index_str.trim().parse::<usize>().unwrap())
                                        {
                                            Ok(_) => {
                                                print!(
                                                    "\r[ LOG ] {}",
                                                    "Успешно\n".to_string().green()
                                                )
                                            }
                                            Err(err) => {
                                                println!("\r[ ERR ] {}\n", err.to_string().red())
                                            }
                                        };
                                    }
                                    Err(err) => println!("\r[ ERR ] {}\n", err.to_string().red()),
                                }
                                enable_raw_mode()?;
                            } else if index == 2 {
                                self.print_vectors();
                            } else if index == 3 {
                                break;
                            }
                            //// println!("{}", index + 1);
                        }
                        KeyCode::Esc => {
                            disable_raw_mode()?;
                            exit(0);
                        }
                        _ => Self::raw_menu(index),
                    }
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
    fn raw_menu(index: usize) {
        // * индекс от 0 до 4 не включительно
        let choices: Vec<&str> = vec![
            "1. Добавить задачу",
            "2. Закончить задачу",
            "3. Список всех задач",
            "4. Выход",
        ];
        clearscreen::clear().unwrap();
        for i in 0..choices.len() {
            let current: &str = choices[i];
            if i == index {
                print!("\r{}\n", current.to_string().bg::<White>());
            } else {
                print!("\r{}\n", current);
            }
        }
    }
    fn add_task(&mut self, task_name: &str) {
        self.unfulled.push(task_name.to_string());
    }
    fn complete(&mut self, index: usize) -> Result<(), &str> {
        if index < self.unfulled.len() {
            self.accomplished.push(self.unfulled[index].clone());
            self.unfulled.remove(index);
            return Ok(());
        }
        Err("Выход за границы массива (Перемещение отклонено)")
    }
    fn print_vectors(&self) {
        if self.accomplished.is_empty() {
            print!("\rAccomplished massive is empty\n");
        } else {
            for i in 0..self.accomplished.len() {
                print!(
                    "\r{}: {} [ {} ]\n",
                    unicode::S_TRUE.to_string().green(),
                    self.accomplished[i],
                    i
                );
            }
        }
        if self.unfulled.is_empty() {
            print!("\rUnfulled massive is empty\n");
        } else {
            for i in 0..self.unfulled.len() {
                print!(
                    "\r{}: {} [ {} ]\n",
                    unicode::S_FALSE.to_string().red(),
                    self.unfulled[i],
                    i
                );
            }
        }
    }
    fn print_uf_vector(&self) {
        if self.unfulled.is_empty() {
            print!("\rUnfulled massive is empty\n");
        } else {
            for i in 0..self.unfulled.len() {
                print!(
                    "\r{}: {} [ {} ]\n",
                    unicode::S_FALSE.to_string().red(),
                    self.unfulled[i],
                    i
                );
            }
        }
    }
}

mod my_io {
    use std::io::{self, Write};

    pub fn input(prompt: &str) -> String {
        let mut var = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut var).unwrap();
        var
    }
}

mod unicode {
    pub const S_TRUE: &str = "✓";
    pub const S_FALSE: &str = "✗";
}
