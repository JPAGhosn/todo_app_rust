use todo_app_rust::models::Todo;

fn main() {

    let todo = Todo::new(
        String::from("Create a new project.")
    );

    println!("todo is {:?}", todo);
}
