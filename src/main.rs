use std::collections::LinkedList;
use std::process::{Command, Stdio};
use std::fs;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

fn readentry(path: fs::DirEntry, parent: &String, taskqueue: &mut LinkedList<String>){
    let path_type = path.file_type().unwrap();
    let path_name = path.file_name().into_string().unwrap();
    let full_path = format!("{}/{}", parent, path_name);

    if path_type.is_file() {
        
        let status = Command::new("grep")
                    .arg("head")
                    .arg(&full_path)
                    .stdout(Stdio::null())
                    .status()
                    .expect("failed to run command");

        if status.code().unwrap() == 0 {
            println!("[0] PRESENT {}", full_path);
        } else {
            println!("[0] ABSENT {}", full_path);
        }

    } else if path_type.is_dir(){
        println!("[0] ENQ {}", full_path);
        taskqueue.push_back(full_path);
    }
}

fn navigate(taskqueue: &mut LinkedList<String>){    
    while taskqueue.len() != 0 {
        let task = taskqueue.pop_front().unwrap();
        let paths = fs::read_dir(&task).unwrap();

        println!("[0] DIR {}", task);

        for path in paths{
            readentry(path.unwrap(), &task, taskqueue);
        }
    }
}

fn main() {
    
    let mut taskqueue: LinkedList<String> = LinkedList::new();

    let cwd_buff = env::current_dir().unwrap();
    let cwd = cwd_buff.to_str().unwrap();

    taskqueue.push_back(format!("{}/{}", cwd, String::from("testdir")));

    let tasks: Mutex<LinkedList<String>> = Mutex::new(taskqueue);

    let reference_counter = Arc::new(tasks);

    for _ in 0..10{
        let tasks = Arc::clone(&reference_counter);
        
        
    }
    

    navigate(&mut taskqueue);

}
