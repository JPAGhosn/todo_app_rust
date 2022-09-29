use todo_app_rust::models::Todo;
use todo_app_rust::models::TodoList;
use todo_app_rust::init_file;
use todo_app_rust::write_to_do_list;

fn main() {
    init_file();

    let mut todo_list = TodoList::new(String::from("My Todo List"));
    todo_list.add_new_todo_item(Todo::new(String::from("Todo Item 1")));
    todo_list.add_new_todo_item(Todo::new(String::from("Todo Item 2")));
    todo_list.add_new_todo_item(Todo::new(String::from("Todo Item 3")));

    write_to_do_list(&todo_list);
}
