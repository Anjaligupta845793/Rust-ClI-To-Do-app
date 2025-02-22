use std::{fs, string};
use std::io::{Write,self};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
#[derive(Debug)]
struct  Task {
    description:String,
    completed:bool

}
impl Task {
    fn new(description:String) -> Self {
        Self {
           description,
           completed:false
        }
    }
}

#[derive(Serialize,Deserialize)]
struct TodoList {
    tasks:Vec<Task>
}
impl  TodoList {
    fn new() -> Self{
          Self { tasks: Vec::new() }
    }
    fn add_task(&mut self ,description:String)  {
          let task = Task::new(description);
          self.tasks.push(task);
          
    }
    fn list_all_task(&mut self) {
       if self.tasks.is_empty() {
          println!("no tasks yet ");
          return ;
       }
       else {
         for (i,task) in self.tasks.iter().enumerate() {
            let status = if task.completed {"✅"}else {"❌"};
            println!("{} {} {:?} ",i,status,task.description);
            
         }
       }
    }

    fn remove_item(&mut self,index:usize){
        if index < self.tasks.len(){
            self.tasks.remove(index);
        }else{
            println!("invalid index")
        }
    }
    fn save_to_file(&mut self){
        let json = serde_json::to_string(&self.tasks).unwrap();
        fs::write("todolist.json", json);
    }
    fn load_from_file(&mut self){
        if let Ok(json) = fs::read_to_string("todolist.json"){
             self.tasks =   serde_json::from_str(&json).unwrap_or_else(|_| Vec::new());
        }

    }
}
fn main(){
    let mut  todolist = TodoList::new();
    todolist.load_from_file();

    loop {
        println!("welcome to our ToDoApplication 💫");
        println!("\n1. Add Task\n2. List Tasks\n3. Remove Task\n4. Save & Exit");
        println!("choose a option:");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input ).unwrap();
        let  choice = input.trim();

        match choice {
            "1" => {
                println!("add discription for your task");
                io::stdout().flush().unwrap();
                let mut  description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                todolist.add_task(description);


            }
            "2" => {
                println!("these are all your tasks 🚀");
                io::stdout().flush().unwrap();
                todolist.list_all_task();
            }
            "3" => {
                println!("inter index of the tasks 😊");
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                if let Ok(i) = index.trim().parse::<usize>(){
                    todolist.remove_item(i);
                }
            }
            "4" => {
                todolist.save_to_file();
                println!("Tasks saved. Exiting...");
                break;
            }
            _=> {
                println!("invalid index❌");
            }
        }

        

    }
    
    
    
}
