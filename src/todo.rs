use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct Todo {
    pub file: File,
}

impl Todo {
    pub fn new(file_path: String) -> Todo {
        Todo {
            file: OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(file_path).expect("Unable to open the file"),
        }
    }

    pub fn show_tasks(&mut self) {
        self.file.seek(SeekFrom::Start(0)).expect("Unable to seek to the beginning");

        let mut contents = String::new();
        self.file.read_to_string(&mut contents).expect("Unable to read the file");

        println!("Tasks: ");
        for (i, task) in contents.lines().enumerate() {
            println!("{} - {}", i, task);
        }
    }

    pub fn add_task(&mut self, content: String) {
        let content = format!("{}\n", content);
        self.file.write(content.as_bytes())
            .expect("Unable to write data in file");

        self.show_tasks();
    }

    pub fn complete_task(&mut self, task_index: usize) {
        let reader = BufReader::new(&self.file);
        let mut file_content: Vec<String> = vec![];

        for (i, line) in reader.lines().enumerate() {
            if i == task_index {
                file_content.push(format!("\x1B[9m{}\x1B[0m", line.unwrap().clone()));
            } else {
                file_content.push(line.unwrap().clone());
            }
        }

        self.rewrite_file(file_content);
        self.show_tasks();
    }

    pub fn delete_task(&mut self, task_index: usize) {
        let reader = BufReader::new(&self.file);
        let mut file_content: Vec<String> = vec![];

        for (i, line) in reader.lines().enumerate() {
            if i != task_index {
                file_content.push(line.unwrap().clone());
            }
        }

        self.rewrite_file(file_content);
        self.show_tasks();
    }

    fn rewrite_file(&mut self, file_content: Vec<String>) {
        self.file.set_len(0).expect("Unable to clean the file");
        self.file.write_all(file_content.join("\n").as_bytes()).expect("Unable to write the file");
        self.file.write_all(b"\n").expect("Unable to write the file");
    }

    pub fn clean_tasks(&mut self) {
        self.file.set_len(0).expect("Unable to clean the file");
        self.show_tasks();
    }
}

