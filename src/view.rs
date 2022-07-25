use crate::controllers::TodoItemController;
use druid::{
    widget::{Button, Checkbox, Flex, Label, List, TextBox},
    Widget, WidgetExt,
};

use crate::data::*;

fn todo_item() -> impl Widget<TodoItem> {
    let checkbox = Checkbox::new("").lens(TodoItem::done);
    let label = Label::raw().lens(TodoItem::text);
    let delete_button = Button::new("Delete").on_click(TodoItem::click_delete);

    Flex::row()
        .with_child(checkbox)
        .with_flex_child(label, 1.)
        .with_child(delete_button)
        .controller(TodoItemController)
}

pub fn build_ui() -> impl Widget<Todos> {
    let clear_completed_button = Button::new("Clear completed").on_click(Todos::clear_completed);

    Flex::column()
        .with_child(new_todo_textbox())
        .with_child(List::new(todo_item).lens(Todos::todos))
        .with_flex_spacer(1.)
        .with_child(clear_completed_button)
}

fn new_todo_textbox() -> impl Widget<Todos> {
    let new_todo_textbox = TextBox::new()
        .with_placeholder("Add a new todo")
        .expand_width()
        .lens(Todos::new_todo);

    let add_todo_button = Button::new("Add").on_click(Todos::click_add);

    Flex::row()
        .with_flex_child(new_todo_textbox, 1.)
        .with_child(add_todo_button)
}
