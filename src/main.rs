use std::fs;
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
}
fn main(){
    println!("welcome to to-do-list app 🎉");
    let mut  list = TodoList::new();
    list.add_task(String::from("learning rust"));
    
    list.add_task(String::from("compeliting project"));
    list.list_all_task();
}
