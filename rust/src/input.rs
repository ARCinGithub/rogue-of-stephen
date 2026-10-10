use godot::classes::{
    INode, InputEvent, InputEventMouseButton, Node,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct InputManager {
    base: Base<Node>,
}

#[godot_api]
impl INode for InputManager {
    fn input(&mut self, event: Gd<InputEvent>) {
        let Ok(mouse_event) =
            event.try_cast::<InputEventMouseButton>()
        else {
            return;
        };

        if mouse_event.get_button_index()
            == godot::global::MouseButton::RIGHT
            && mouse_event.is_pressed()
        {
            let position = mouse_event.get_position();
            self.signals().right_clicked().emit(position);
        }
    }
}

#[godot_api]
impl InputManager {
    #[signal]
    fn right_clicked(screen_position: Vector2);
}
