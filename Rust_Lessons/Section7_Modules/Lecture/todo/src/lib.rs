//Installed cargo-modules with "cargo install cargo-modules"
//The lecture uses the keyword "generate tree", but the new equivalent for this is "structure", i.e. "cargo modules structure"
//This allows you to view the functions and structures from the command line.

//The pub keyword makes an item public, meaning it can be accessed outside the module.
//One function is marked pub(crate), meaning it is available anywhere inside the create, but it isn't exposed outside the crate.
//pub(self) is equivalent to private in C++, and it is the default choice.

mod list
{
    //This task is now public instead of private
    pub struct pubTasks{
        pub item:String,
    }
    
    struct Tasks{
        item:String,
    }

    // pub mod things_todo{
    //     pub fn add_activity(){

    //     }
    //     fn update_activity(){

    //     }
    //     fn marked_completed(){

    //     }
    // }
    // mod items_completed{
    //     fn remove_task(){

    //     }
    //     fn move_back_todo(){

    //     }
    // }


}

//This tells Rust, I want you to find a file called things_todo
mod things_todo;
//This is equivalent to calling a namespace from things_todo for the add_activity method
use crate::things_todo::add_activity;
use things_todo::items_completed;
use things_todo::items_completed::test::test;

fn lets_add_task(){
    let task = list::pubTasks{item:String::from("pubTasks")};
    //let task = list::Tasks{item:String::from("Tasks")};
    //This command will not build because the struct and its features are not public.

    //This would be the command to use if things_todo was still within the list mod
    // list::things_todo::add_activity(); //relative path
    // crate::list::things_todo::add_activity(); // absolute path because we start at the root crate

    add_activity();
    items_completed::remove_task();
    test();
}

