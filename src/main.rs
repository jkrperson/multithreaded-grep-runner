use std::collections::LinkedList;
use std::process::{Command, Stdio};
use std::fs;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

fn any_running(status: &Mutex<Vec<bool>>)->bool{
    let statusvec = status.lock().unwrap();

    for i in 0..statusvec.len() {
        if statusvec[i]{
            return statusvec[i]
        }
    }

    return false
}

fn pop_task(tasks: &Mutex<LinkedList<String>>, status: &Mutex<Vec<bool>>, worker_num: i32)->Option<String>{
    let mut taskqueue = tasks.lock().unwrap();
    let mut statusvec = status.lock().unwrap();
    
    if taskqueue.is_empty() {
        statusvec[worker_num as usize] = false;
        taskqueue.pop_front()
    } else {
        statusvec[worker_num as usize] = true;
        taskqueue.pop_front()
    }
}

fn push_task(tasks: & Mutex<LinkedList<String>>, fullpath: String){
    let mut taskqueue = tasks.lock().unwrap();
    taskqueue.push_back(fullpath);
}

fn readentry(path: fs::DirEntry, parent: &String, tasks: & Mutex<LinkedList<String>>, worker_num: i32){
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
            println!("[{}] PRESENT {}", worker_num, full_path);
        } else {
            println!("[{}] ABSENT {}", worker_num, full_path);
        }

    } else if path_type.is_dir(){
        println!("[{}] ENQ {}", worker_num, full_path);
        push_task(tasks, full_path);
    }
}

fn navigate(tasks: & Mutex<LinkedList<String>>, status: & Mutex<Vec<bool>>, worker_num: i32){    
    while any_running(status) {
        let task_option = pop_task(tasks, status, worker_num);
        
        if task_option.is_some(){
            let task = task_option.unwrap();
            let paths = fs::read_dir(&task).unwrap();
            println!("[{}] DIR {}", worker_num, task);

            for path in paths{
                readentry(path.unwrap(), &task, tasks, worker_num);
            }
        }
    }
}

fn main() {
    
    let threads = 10;

    // Task queue Initialization

    let mut taskqueue: LinkedList<String> = LinkedList::new();

    let cwd_buff = env::current_dir().unwrap();
    let cwd = cwd_buff.to_str().unwrap();

    taskqueue.push_back(format!("{}/{}", cwd, String::from("testdir")));

    let tasks: Mutex<LinkedList<String>> = Mutex::new(taskqueue);
    let tasks_reference = Arc::new(tasks);


    // Status array Initialization
    // true = running, false = standby

    let mut statusvec: Vec<bool> = Vec::new();
    
    for _ in 0..threads {
        statusvec.push(true);
    }

    let status: Mutex<Vec<bool>> = Mutex::new(statusvec);
    let status_reference: Arc<Mutex<Vec<bool>>> = Arc::new(status);

    let mut handles = vec![];

    for i in 0..threads{
        let tasks = Arc::clone(&tasks_reference);
        let status = Arc::clone(&status_reference);
        let handle = thread::spawn(move || {navigate(&tasks, &status, i);});

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

}
