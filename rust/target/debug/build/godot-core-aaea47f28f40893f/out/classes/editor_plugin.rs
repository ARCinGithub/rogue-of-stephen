#![doc = "Sidecar module for class [`EditorPlugin`][crate::classes::EditorPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, ClassId, CowArg, InParamTuple, OutParamTuple, ParamTuple, RawPtr, RefArg
};
use crate::private::Signature;
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::classes::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `EditorPlugin`.\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`editor_plugin`][crate::classes::editor_plugin]: sidecar module with related enum/flag types\n* [`IEditorPlugin`][crate::classes::IEditorPlugin]: virtual methods\n* [`SignalsOfEditorPlugin`][crate::classes::editor_plugin::SignalsOfEditorPlugin]: signal collection\n\n\nSee also [Godot docs for `EditorPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`EditorPlugin::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nPlugins are used by the editor to extend functionality. The most common types of plugins are those which edit a given node or resource type, import plugins and export plugins. See also [`EditorScript`][crate::classes::EditorScript] to add functions to the editor.\n\n**Note:** Some names in this class contain \"left\" or \"right\" (e.g. [`DockSlot::LEFT_UL`][`crate::classes::editor_plugin::DockSlot::LEFT_UL`]). These APIs assume left-to-right layout, and would be backwards when using right-to-left layout. These names are kept for compatibility reasons."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorPlugin`][crate::classes::EditorPlugin].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorPlugin: crate::obj::GodotClass < Base = EditorPlugin > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = "Called when the node enters the [`SceneTree`][crate::classes::SceneTree] (e.g. upon instantiating, scene changing, or after calling [`add_child`][`crate::classes::Node::add_child`] in a script). If the node has children, its [`enter_tree`][`crate::classes::INode::enter_tree`] callback will be called first, and then that of the children.\n\nCorresponds to the [`NodeNotification::ENTER_TREE`][`crate::classes::notify::NodeNotification::ENTER_TREE`] notification in [`on_notification`][`crate::classes::IObject::on_notification`]."]
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the node is \"ready\", i.e. when both the node and its children have entered the scene tree. If the node has children, their [`ready`][`crate::classes::INode::ready`] callbacks get triggered first, and the parent node will receive the ready notification afterwards.\n\nCorresponds to the [`NodeNotification::READY`][`crate::classes::notify::NodeNotification::READY`] notification in [`on_notification`][`crate::classes::IObject::on_notification`]. See also the `@onready` annotation for variables.\n\nUsually used for initialization. For even earlier initialization, [`init`][`crate::classes::IObject::init`] may be used. See also [`enter_tree`][`crate::classes::INode::enter_tree`].\n\n**Note:** This method may be called only once for each node. After removing a node from the scene tree and adding it again, [`ready`][`crate::classes::INode::ready`] will **not** be called a second time. This can be bypassed by requesting another call with [`request_ready`][`crate::classes::Node::request_ready`], which may be called anywhere before adding the node again."]
        fn ready(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the node is about to leave the [`SceneTree`][crate::classes::SceneTree] (e.g. upon freeing, scene changing, or after calling [`remove_child`][`crate::classes::Node::remove_child`] in a script). If the node has children, its [`exit_tree`][`crate::classes::INode::exit_tree`] callback will be called last, after all its children have left the tree.\n\nCorresponds to the [`NodeNotification::EXIT_TREE`][`crate::classes::notify::NodeNotification::EXIT_TREE`] notification in [`on_notification`][`crate::classes::IObject::on_notification`] and signal `tree_exiting`. To get notified when the node has already left the active tree, connect to the `tree_exited`."]
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called on each idle frame, prior to rendering, and after physics ticks have been processed. `delta` is the time between frames in seconds.\n\nIt is only called if processing is enabled for this Node, which is done automatically if this method is overridden, and can be toggled with [`set_process`][`crate::classes::Node::set_process`].\n\nProcessing happens in order of \\[member process_priority], lower priority values are called first. Nodes with the same priority are processed in tree order, or top to bottom as seen in the editor (also known as pre-order traversal).\n\nCorresponds to the [`NodeNotification::PROCESS`][`crate::classes::notify::NodeNotification::PROCESS`] notification in [`on_notification`][`crate::classes::IObject::on_notification`].\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan).\n\n**Note:** When the engine is struggling and the frame rate is lowered, `delta` will increase. When `delta` is increased, it's capped at a maximum of \\[member Engine.time_scale] * \\[member Engine.max_physics_steps_per_frame] / \\[member Engine.physics_ticks_per_second]. As a result, accumulated `delta` may not represent real world time.\n\n**Note:** When `--fixed-fps` is enabled or the engine is running in Movie Maker mode (see [`MovieWriter`][crate::classes::MovieWriter]), process `delta` will always be the same for every frame, regardless of how much time the frame took to render.\n\n**Note:** Frame delta may be post-processed by \\[member OS.delta_smoothing] if this is enabled for the project."]
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        #[doc = "Called once on each physics tick, and allows Nodes to synchronize their logic with physics ticks. `delta` is the logical time between physics ticks in seconds and is equal to \\[member Engine.time_scale] / \\[member Engine.physics_ticks_per_second].\n\nIt is only called if physics processing is enabled for this Node, which is done automatically if this method is overridden, and can be toggled with [`set_physics_process`][`crate::classes::Node::set_physics_process`].\n\nProcessing happens in order of \\[member process_physics_priority], lower priority values are called first. Nodes with the same priority are processed in tree order, or top to bottom as seen in the editor (also known as pre-order traversal).\n\nCorresponds to the [`NodeNotification::PHYSICS_PROCESS`][`crate::classes::notify::NodeNotification::PHYSICS_PROCESS`] notification in [`on_notification`][`crate::classes::IObject::on_notification`].\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan).\n\n**Note:** Accumulated `delta` may diverge from real world seconds."]
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        #[doc = "Called when there is an input event. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if input processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_input`][`crate::classes::Node::set_process_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nFor gameplay input, [`unhandled_input`][`crate::classes::INode::unhandled_input`] and [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`] are usually a better fit as they allow the GUI to intercept the events first.\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan)."]
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = "Called when an [`InputEventKey`][crate::classes::InputEventKey], [`InputEventShortcut`][crate::classes::InputEventShortcut], or [`InputEventJoypadButton`][crate::classes::InputEventJoypadButton] hasn't been consumed by [`input`][`crate::classes::INode::input`] or any GUI [`Control`][crate::classes::Control] item. It is called before [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`] and [`unhandled_input`][`crate::classes::INode::unhandled_input`]. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if shortcut processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_shortcut_input`][`crate::classes::Node::set_process_shortcut_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nThis method can be used to handle shortcuts. For generic GUI events, use [`input`][`crate::classes::INode::input`] instead. Gameplay events should usually be handled with either [`unhandled_input`][`crate::classes::INode::unhandled_input`] or [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`].\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not orphan)."]
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = "Called when an [`InputEventKey`][crate::classes::InputEventKey] hasn't been consumed by [`input`][`crate::classes::INode::input`] or any GUI [`Control`][crate::classes::Control] item. It is called after [`shortcut_input`][`crate::classes::INode::shortcut_input`] but before [`unhandled_input`][`crate::classes::INode::unhandled_input`]. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if unhandled key input processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_unhandled_key_input`][`crate::classes::Node::set_process_unhandled_key_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nThis method can be used to handle Unicode character input with `Alt`, `Alt + Ctrl`, and `Alt + Shift` modifiers, after shortcuts were handled.\n\nFor gameplay input, this and [`unhandled_input`][`crate::classes::INode::unhandled_input`] are usually a better fit than [`input`][`crate::classes::INode::input`], as GUI events should be handled first. This method also performs better than [`unhandled_input`][`crate::classes::INode::unhandled_input`], since unrelated events such as [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] are automatically filtered. For shortcuts, consider using [`shortcut_input`][`crate::classes::INode::shortcut_input`] instead.\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan)."]
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = "Called when an [`InputEvent`][crate::classes::InputEvent] hasn't been consumed by [`input`][`crate::classes::INode::input`] or any GUI [`Control`][crate::classes::Control] item. It is called after [`shortcut_input`][`crate::classes::INode::shortcut_input`] and after [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`]. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if unhandled input processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_unhandled_input`][`crate::classes::Node::set_process_unhandled_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nFor gameplay input, this method is usually a better fit than [`input`][`crate::classes::INode::input`], as GUI events need a higher priority. For keyboard shortcuts, consider using [`shortcut_input`][`crate::classes::INode::shortcut_input`] instead, as it is called before this method. Finally, to handle keyboard events, consider using [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`] for performance reasons.\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan)."]
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: NodeNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn on_get(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn on_set(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `on_get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        fn on_validate_property(&self, property: &mut crate::registry::info::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        #[cfg_attr(published_docs, doc(cfg(since_api = "4.3")))]
        fn on_get_property_list(&mut self) -> Vec < crate::registry::info::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn on_property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = "Called when there is a root node in the current edited scene, [`handles`][`crate::classes::IEditorPlugin::handles`] is implemented, and an [`InputEvent`][crate::classes::InputEvent] happens in the 2D viewport. If this method returns `true`, `event` is intercepted by this `EditorPlugin`, otherwise `event` is forwarded to other Editor classes.\n\n\n```gdscript\n# Prevents the InputEvent from reaching other Editor classes.\nfunc _forward_canvas_gui_input(event):\n\treturn true\n```\n\n\nThis method must return `false` in order to forward the [`InputEvent`][crate::classes::InputEvent] to other Editor classes.\n\n\n```gdscript\n# Consumes InputEventMouseMotion and forwards other InputEvent types.\nfunc _forward_canvas_gui_input(event):\n\tif (event is InputEventMouseMotion):\n\t\treturn true\n\treturn false\n```\n"]
        fn forward_canvas_gui_input(&mut self, event: Option < Gd < crate::classes::InputEvent > >,) -> bool {
            unimplemented !()
        }
        #[doc = "Called by the engine when the 2D editor's viewport is updated. `viewport_control` is an overlay on top of the viewport and it can be used for drawing. You can update the viewport manually by calling [`update_overlays`][`crate::classes::EditorPlugin::update_overlays`].\n\n\n```gdscript\nfunc _forward_canvas_draw_over_viewport(overlay):\n\t# Draw a circle at the cursor's position.\n\toverlay.draw_circle(overlay.get_local_mouse_position(), 64, Color.WHITE)\n\nfunc _forward_canvas_gui_input(event):\n\tif event is InputEventMouseMotion:\n\t\t# Redraw the viewport when the cursor is moved.\n\t\tupdate_overlays()\n\t\treturn true\n\treturn false\n```\n"]
        fn forward_canvas_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        #[doc = "This method is the same as [`forward_canvas_draw_over_viewport`][`crate::classes::IEditorPlugin::forward_canvas_draw_over_viewport`], except it draws on top of everything. Useful when you need an extra layer that shows over anything else.\n\nYou need to enable calling of this method by using [`set_force_draw_over_forwarding_enabled`][`crate::classes::EditorPlugin::set_force_draw_over_forwarding_enabled`]."]
        fn forward_canvas_force_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        #[doc = "Called when there is a root node in the current edited scene, [`handles`][`crate::classes::IEditorPlugin::handles`] is implemented, and an [`InputEvent`][crate::classes::InputEvent] happens in the 3D viewport. The return value decides whether the [`InputEvent`][crate::classes::InputEvent] is consumed or forwarded to other `EditorPlugin`s. See \\[enum AfterGUIInput] for options.\n\n\n```gdscript\n# Prevents the InputEvent from reaching other Editor classes.\nfunc _forward_3d_gui_input(camera, event):\n\treturn EditorPlugin.AFTER_GUI_INPUT_STOP\n```\n\n\nThis method must return [`AfterGuiInput::PASS`][`crate::classes::editor_plugin::AfterGuiInput::PASS`] in order to forward the [`InputEvent`][crate::classes::InputEvent] to other Editor classes.\n\n\n```gdscript\n# Consumes InputEventMouseMotion and forwards other InputEvent types.\nfunc _forward_3d_gui_input(camera, event):\n\treturn EditorPlugin.AFTER_GUI_INPUT_STOP if event is InputEventMouseMotion else EditorPlugin.AFTER_GUI_INPUT_PASS\n```\n"]
        fn forward_3d_gui_input(&mut self, viewport_camera: Option < Gd < crate::classes::Camera3D > >, event: Option < Gd < crate::classes::InputEvent > >,) -> i32 {
            unimplemented !()
        }
        #[doc = "Called by the engine when the 3D editor's viewport is updated. `viewport_control` is an overlay on top of the viewport and it can be used for drawing. You can update the viewport manually by calling [`update_overlays`][`crate::classes::EditorPlugin::update_overlays`].\n\n\n```gdscript\nfunc _forward_3d_draw_over_viewport(overlay):\n\t# Draw a circle at the cursor's position.\n\toverlay.draw_circle(overlay.get_local_mouse_position(), 64, Color.WHITE)\n\nfunc _forward_3d_gui_input(camera, event):\n\tif event is InputEventMouseMotion:\n\t\t# Redraw the viewport when the cursor is moved.\n\t\tupdate_overlays()\n\t\treturn EditorPlugin.AFTER_GUI_INPUT_STOP\n\treturn EditorPlugin.AFTER_GUI_INPUT_PASS\n```\n"]
        fn forward_3d_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        #[doc = "This method is the same as [`forward_3d_draw_over_viewport`][`crate::classes::IEditorPlugin::forward_3d_draw_over_viewport`], except it draws on top of everything. Useful when you need an extra layer that shows over anything else.\n\nYou need to enable calling of this method by using [`set_force_draw_over_forwarding_enabled`][`crate::classes::EditorPlugin::set_force_draw_over_forwarding_enabled`]."]
        fn forward_3d_force_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        #[doc = "Override this method in your plugin to provide the name of the plugin when displayed in the Godot editor.\n\nFor main screen plugins, this appears at the top of the screen, to the right of the \"2D\", \"3D\", \"Script\", \"Game\", and \"AssetLib\" buttons."]
        fn get_plugin_name(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method in your plugin to return a [`Texture2D`][crate::classes::Texture2D] in order to give it an icon.\n\nFor main screen plugins, this appears at the top of the screen, to the right of the \"2D\", \"3D\", \"Script\", \"Game\", and \"AssetLib\" buttons.\n\nIdeally, the plugin icon should be white with a transparent background and 16×16 pixels in size.\n\n\n```gdscript\nfunc _get_plugin_icon():\n\t# You can use a custom icon:\n\treturn preload(\"res://addons/my_plugin/my_plugin_icon.svg\")\n\t# Or use a built-in icon:\n\treturn EditorInterface.get_editor_theme().get_icon(\"Node\", \"EditorIcons\")\n```\n"]
        fn get_plugin_icon(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            unimplemented !()
        }
        #[doc = "Returns `true` if this is a main screen editor plugin (it goes in the workspace selector together with **2D**, **3D**, **Script**, **Game**, and **AssetLib**).\n\nWhen the plugin's workspace is selected, other main screen plugins will be hidden, but your plugin will not appear automatically. It needs to be added as a child of [`get_editor_main_screen`][`crate::classes::EditorInterface::get_editor_main_screen`] and made visible inside [`make_visible`][`crate::classes::IEditorPlugin::make_visible`].\n\nUse [`get_plugin_name`][`crate::classes::IEditorPlugin::get_plugin_name`] and [`get_plugin_icon`][`crate::classes::IEditorPlugin::get_plugin_icon`] to customize the plugin button's appearance.\n\n```gdscript\nvar plugin_control\n\nfunc _enter_tree():\n\tplugin_control = preload(\"my_plugin_control.tscn\").instantiate()\n\tEditorInterface.get_editor_main_screen().add_child(plugin_control)\n\tplugin_control.hide()\n\nfunc _has_main_screen():\n\treturn true\n\nfunc _make_visible(visible):\n\tplugin_control.visible = visible\n\nfunc _get_plugin_name():\n\treturn \"My Super Cool Plugin 3000\"\n\nfunc _get_plugin_icon():\n\treturn EditorInterface.get_editor_theme().get_icon(\"Node\", \"EditorIcons\")\n```"]
        fn has_main_screen(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "This function will be called when the editor is requested to become visible. It is used for plugins that edit a specific object type.\n\nRemember that you have to manage the visibility of all your editor controls manually."]
        fn make_visible(&mut self, visible: bool,) {
            unimplemented !()
        }
        #[doc = "This function is used for plugins that edit specific object types (nodes or resources). It requests the editor to edit the given object.\n\n`object` can be `null` if the plugin was editing an object, but there is no longer any selected object handled by this plugin. It can be used to cleanup editing state."]
        fn edit(&mut self, object: Option < Gd < crate::classes::Object > >,) {
            unimplemented !()
        }
        #[doc = "Implement this function if your plugin edits a specific type of object (Resource or Node). If you return `true`, then you will get the functions [`edit`][`crate::classes::IEditorPlugin::edit`] and [`make_visible`][`crate::classes::IEditorPlugin::make_visible`] called when the editor requests them. If you have declared the methods [`forward_canvas_gui_input`][`crate::classes::IEditorPlugin::forward_canvas_gui_input`] and [`forward_3d_gui_input`][`crate::classes::IEditorPlugin::forward_3d_gui_input`] these will be called too.\n\n**Note:** Each plugin should handle only one type of objects at a time. If a plugin handles more types of objects and they are edited at the same time, it will result in errors."]
        fn handles(&self, object: Gd < crate::classes::Object >,) -> bool {
            unimplemented !()
        }
        #[doc = "Override this method to provide a state data you want to be saved, like view position, grid settings, folding, etc. This is used when saving the scene (so state is kept when opening it again) and for switching tabs (so state can be restored when the tab returns). This data is automatically saved for each scene in an `editstate` file in the editor metadata folder. If you want to store global (scene-independent) editor data for your plugin, you can use [`get_window_layout`][`crate::classes::IEditorPlugin::get_window_layout`] instead.\n\nUse [`set_state`][`crate::classes::IEditorPlugin::set_state`] to restore your saved state.\n\n**Note:** This method should not be used to save important settings that should persist with the project.\n\n**Note:** You must implement [`get_plugin_name`][`crate::classes::IEditorPlugin::get_plugin_name`] for the state to be stored and restored correctly.\n\n```gdscript\nfunc _get_state():\n\tvar state = { \"zoom\": zoom, \"preferred_color\": my_color }\n\treturn state\n```"]
        fn get_state(&self,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "Restore the state saved by [`get_state`][`crate::classes::IEditorPlugin::get_state`]. This method is called when the current scene tab is changed in the editor.\n\n**Note:** Your plugin must implement [`get_plugin_name`][`crate::classes::IEditorPlugin::get_plugin_name`], otherwise it will not be recognized and this method will not be called.\n\n```gdscript\nfunc _set_state(data):\n\tzoom = data.get(\"zoom\", 1.0)\n\tpreferred_color = data.get(\"my_color\", Color.WHITE)\n```"]
        fn set_state(&mut self, state: VarDictionary,) {
            unimplemented !()
        }
        #[doc = "Clear all the state and reset the object being edited to zero. This ensures your plugin does not keep editing a currently existing node, or a node from the wrong scene."]
        fn clear(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to provide a custom message that lists unsaved changes. The editor will call this method when exiting or when closing a scene, and display the returned string in a confirmation dialog. Return empty string if the plugin has no unsaved changes.\n\nWhen closing a scene, `for_scene` is the path to the scene being closed. You can use it to handle built-in resources in that scene.\n\nIf the user confirms saving, [`save_external_data`][`crate::classes::IEditorPlugin::save_external_data`] will be called, before closing the editor.\n\n```gdscript\nfunc _get_unsaved_status(for_scene):\n\tif not unsaved:\n\t\treturn \"\"\n\n\tif for_scene.is_empty():\n\t\treturn \"Save changes in MyCustomPlugin before closing?\"\n\telse:\n\t\treturn \"Scene %s has changes from MyCustomPlugin. Save before closing?\" % for_scene.get_file()\n\nfunc _save_external_data():\n\tunsaved = false\n```\n\nIf the plugin has no scene-specific changes, you can ignore the calls when closing scenes:\n\n```gdscript\nfunc _get_unsaved_status(for_scene):\n\tif not for_scene.is_empty():\n\t\treturn \"\"\n```"]
        fn get_unsaved_status(&self, for_scene: GString,) -> GString {
            unimplemented !()
        }
        #[doc = "This method is called after the editor saves the project or when it's closed. It asks the plugin to save edited external scenes/resources."]
        fn save_external_data(&mut self,) {
            unimplemented !()
        }
        #[doc = "This method is called when the editor is about to save the project, switch to another tab, etc. It asks the plugin to apply any pending state changes to ensure consistency.\n\nThis is used, for example, in shader editors to let the plugin know that it must apply the shader code being written by the user to the object."]
        fn apply_changes(&mut self,) {
            unimplemented !()
        }
        #[doc = "This is for editors that edit script-based objects. You can return a list of breakpoints in the format (`script:line`), for example: `res://path_to_script.gd:25`."]
        fn get_breakpoints(&self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Restore the plugin GUI layout and data saved by [`get_window_layout`][`crate::classes::IEditorPlugin::get_window_layout`]. This method is called for every plugin on editor startup. Use the provided `configuration` file to read your saved data.\n\n```gdscript\nfunc _set_window_layout(configuration):\n\t$Window.position = configuration.get_value(\"MyPlugin\", \"window_position\", Vector2())\n\t$Icon.modulate = configuration.get_value(\"MyPlugin\", \"icon_color\", Color.WHITE)\n```"]
        fn set_window_layout(&mut self, configuration: Option < Gd < crate::classes::ConfigFile > >,) {
            unimplemented !()
        }
        #[doc = "Override this method to provide the GUI layout of the plugin or any other data you want to be stored. This is used to save the project's editor layout when [`queue_save_layout`][`crate::classes::EditorPlugin::queue_save_layout`] is called or the editor layout was changed (for example changing the position of a dock). The data is stored in the `editor_layout.cfg` file in the editor metadata directory.\n\nUse [`set_window_layout`][`crate::classes::IEditorPlugin::set_window_layout`] to restore your saved layout.\n\n```gdscript\nfunc _get_window_layout(configuration):\n\tconfiguration.set_value(\"MyPlugin\", \"window_position\", $Window.position)\n\tconfiguration.set_value(\"MyPlugin\", \"icon_color\", $Icon.modulate)\n```"]
        fn get_window_layout(&mut self, configuration: Option < Gd < crate::classes::ConfigFile > >,) {
            unimplemented !()
        }
        #[doc = "This method is called when the editor is about to run the project. The plugin can then perform required operations before the project runs.\n\nThis method must return a boolean. If this method returns `false`, the project will not run. The run is aborted immediately, so this also prevents all other plugins' [`build`][`crate::classes::IEditorPlugin::build`] methods from running."]
        fn build(&mut self,) -> bool {
            unimplemented !()
        }
        #[doc = "This function is called when an individual scene is about to be played in the editor. `args` is a list of command line arguments that will be passed to the new Godot instance, which will be replaced by the list returned by this function.\n\n```gdscript\nfunc _run_scene(scene, args):\n\targs.append(\"--an-extra-argument\")\n\treturn args\n```\n\n**Note:** Text that is printed in this method will not be visible in the editor's Output panel unless \\[member EditorSettings.run/output/always_clear_output_on_play] is `false`."]
        fn run_scene(&self, scene: GString, args: PackedStringArray,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Called by the engine when the user enables the `EditorPlugin` in the Plugin tab of the project settings window."]
        fn enable_plugin(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called by the engine when the user disables the `EditorPlugin` in the Plugin tab of the project settings window."]
        fn disable_plugin(&mut self,) {
            unimplemented !()
        }
        #[doc = "The elements in the array returned from this method are displayed as warnings in the Scene dock if the script that overrides it is a `tool` script.\n\nReturning an empty array produces no warnings.\n\nCall [`update_configuration_warnings`][`crate::classes::Node::update_configuration_warnings`] when the warnings need to be updated for this node.\n\n```gdscript\n@export var energy = 0:\n\tset(value):\n\t\tenergy = value\n\t\tupdate_configuration_warnings()\n\nfunc _get_configuration_warnings():\n\tif energy < 0:\n\t\treturn [\"Energy must be 0 or greater.\"]\n\telse:\n\t\treturn []\n```"]
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "The elements in the array returned from this method are displayed as warnings in the Scene dock if the script that overrides it is a `tool` script, and accessibility warnings are enabled in the editor settings.\n\nReturning an empty array produces no warnings."]
        fn get_accessibility_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Called during accessibility information updates to determine the currently focused sub-element, should return a sub-element RID or the value returned by [`get_accessibility_element`][`crate::classes::Node::get_accessibility_element`]."]
        fn get_focused_accessibility_element(&self,) -> Rid {
            unimplemented !()
        }
    }
    impl EditorPlugin {
        #[doc = "Adds a new dock.\n\nWhen your plugin is deactivated, make sure to remove your custom dock with [`remove_dock`][`crate::classes::EditorPlugin::remove_dock`] and free it with [`queue_free`][`crate::classes::Node::queue_free`]."]
        pub fn add_dock(&mut self, dock: impl AsArg < Option < Gd < crate::classes::EditorDock >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorDock > > >,);
            let args = (dock.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_dock", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes `dock` from the available docks. You should manually call [`queue_free`][`crate::classes::Node::queue_free`] to free it."]
        pub fn remove_dock(&mut self, dock: impl AsArg < Option < Gd < crate::classes::EditorDock >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorDock > > >,);
            let args = (dock.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_dock", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a custom control to a container in the editor UI.\n\nPlease remember that you have to manage the visibility of your custom controls yourself (and likely hide it after adding it).\n\nWhen your plugin is deactivated, make sure to remove your custom control with [`remove_control_from_container`][`crate::classes::EditorPlugin::remove_control_from_container`] and free it with [`queue_free`][`crate::classes::Node::queue_free`]."]
        pub fn add_control_to_container(&mut self, container: crate::classes::editor_plugin::CustomControlContainer, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::editor_plugin::CustomControlContainer, CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (container, control.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_control_to_container", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the control from the specified container. You have to manually [`queue_free`][`crate::classes::Node::queue_free`] the control."]
        pub fn remove_control_from_container(&mut self, container: crate::classes::editor_plugin::CustomControlContainer, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::editor_plugin::CustomControlContainer, CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (container, control.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_control_from_container", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a custom menu item to **Project > Tools** named `name`. When clicked, the provided `callable` will be called."]
        pub fn add_tool_menu_item(&mut self, name: impl AsArg < GString >, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Callable >,);
            let args = (name.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_tool_menu_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a custom [`PopupMenu`][crate::classes::PopupMenu] submenu under **Project > Tools >** `name`. Use [`remove_tool_menu_item`][`crate::classes::EditorPlugin::remove_tool_menu_item`] on plugin clean up to remove the menu."]
        pub fn add_tool_submenu_item(&mut self, name: impl AsArg < GString >, submenu: impl AsArg < Option < Gd < crate::classes::PopupMenu >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::PopupMenu > > >,);
            let args = (name.into_arg(), submenu.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_tool_submenu_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a menu `name` from **Project > Tools**."]
        pub fn remove_tool_menu_item(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_tool_menu_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`PopupMenu`][crate::classes::PopupMenu] under **Scene > Export As...**."]
        pub fn get_export_as_menu(&self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_export_as_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a custom type, which will appear in the list of nodes or resources.\n\nWhen a given node or resource is selected, the base type will be instantiated (e.g. \"Node3D\", \"Control\", \"Resource\"), then the script will be loaded and set to this object.\n\n**Note:** The base type is the base engine class which this type's class hierarchy inherits, not any custom type parent classes.\n\nYou can use the virtual method [`handles`][`crate::classes::IEditorPlugin::handles`] to check if your custom object is being edited by checking the script or using the `is` keyword.\n\nDuring run-time, this will be a simple object with a script so this function does not need to be called then.\n\n**Note:** Custom types added this way are not true classes. They are just a helper to create a node with specific script."]
        pub fn add_custom_type(&mut self, type_: impl AsArg < GString >, base: impl AsArg < GString >, script: impl AsArg < Option < Gd < crate::classes::Script >> >, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, Option < Gd < crate::classes::Script > > >, CowArg < 'a3, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (type_.into_arg(), base.into_arg(), script.into_arg(), icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_custom_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a custom type added by [`add_custom_type`][`crate::classes::EditorPlugin::add_custom_type`]."]
        pub fn remove_custom_type(&mut self, type_: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (type_.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_custom_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds the control to a specific dock slot.\n\nIf the dock is repositioned and as long as the plugin is active, the editor will save the dock position on further sessions.\n\nWhen your plugin is deactivated, make sure to remove your custom control with [`remove_control_from_docks`][`crate::classes::EditorPlugin::remove_control_from_docks`] and free it with [`queue_free`][`crate::classes::Node::queue_free`].\n\nOptionally, you can specify a shortcut parameter. When pressed, this shortcut will open and focus the dock."]
        pub(crate) fn add_control_to_dock_full(&mut self, slot: crate::classes::editor_plugin::DockSlot, control: CowArg < Option < Gd < crate::classes::Control > > >, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (crate::classes::editor_plugin::DockSlot, CowArg < 'a0, Option < Gd < crate::classes::Control > > >, CowArg < 'a1, Option < Gd < crate::classes::Shortcut > > >,);
            let args = (slot, control, shortcut,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_control_to_dock", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_control_to_dock_ex`][Self::add_control_to_dock_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds the control to a specific dock slot.\n\nIf the dock is repositioned and as long as the plugin is active, the editor will save the dock position on further sessions.\n\nWhen your plugin is deactivated, make sure to remove your custom control with [`remove_control_from_docks`][`crate::classes::EditorPlugin::remove_control_from_docks`] and free it with [`queue_free`][`crate::classes::Node::queue_free`].\n\nOptionally, you can specify a shortcut parameter. When pressed, this shortcut will open and focus the dock."]
        #[inline]
        pub fn add_control_to_dock(&mut self, slot: crate::classes::editor_plugin::DockSlot, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            self.add_control_to_dock_ex(slot, control,) . done()
        }
        #[doc = "Adds the control to a specific dock slot.\n\nIf the dock is repositioned and as long as the plugin is active, the editor will save the dock position on further sessions.\n\nWhen your plugin is deactivated, make sure to remove your custom control with [`remove_control_from_docks`][`crate::classes::EditorPlugin::remove_control_from_docks`] and free it with [`queue_free`][`crate::classes::Node::queue_free`].\n\nOptionally, you can specify a shortcut parameter. When pressed, this shortcut will open and focus the dock."]
        #[inline]
        pub fn add_control_to_dock_ex < 'ex > (&'ex mut self, slot: crate::classes::editor_plugin::DockSlot, control: impl AsArg < Option < Gd < crate::classes::Control >> > + 'ex,) -> ExAddControlToDock < 'ex > {
            ExAddControlToDock::new(self, slot, control,)
        }
        #[doc = "Removes the control from the dock. You have to manually [`queue_free`][`crate::classes::Node::queue_free`] the control."]
        pub fn remove_control_from_docks(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_control_from_docks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the tab icon for the given control in a dock slot. Setting to `null` removes the icon."]
        pub fn set_dock_tab_icon(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (control.into_arg(), icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "set_dock_tab_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a control to the bottom panel (together with Output, Debug, Animation, etc.). Returns a reference to a button that is outside the scene tree. It's up to you to hide/show the button when needed. When your plugin is deactivated, make sure to remove your custom control with [`remove_control_from_bottom_panel`][`crate::classes::EditorPlugin::remove_control_from_bottom_panel`] and free it with [`queue_free`][`crate::classes::Node::queue_free`].\n\n`shortcut` is a shortcut that, when activated, will toggle the bottom panel's visibility. The shortcut object is only set when this control is added to the bottom panel.\n\n**Note** See the default editor bottom panel shortcuts in the Editor Settings for inspiration. By convention, they all use `Alt` modifier."]
        pub(crate) fn add_control_to_bottom_panel_full(&mut self, control: CowArg < Option < Gd < crate::classes::Control > > >, title: CowArg < GString >, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >,) -> Option < Gd < crate::classes::Button > > {
            type CallRet = Option < Gd < crate::classes::Button > >;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >, CowArg < 'a1, GString >, CowArg < 'a2, Option < Gd < crate::classes::Shortcut > > >,);
            let args = (control, title, shortcut,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_control_to_bottom_panel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_control_to_bottom_panel_ex`][Self::add_control_to_bottom_panel_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a control to the bottom panel (together with Output, Debug, Animation, etc.). Returns a reference to a button that is outside the scene tree. It's up to you to hide/show the button when needed. When your plugin is deactivated, make sure to remove your custom control with [`remove_control_from_bottom_panel`][`crate::classes::EditorPlugin::remove_control_from_bottom_panel`] and free it with [`queue_free`][`crate::classes::Node::queue_free`].\n\n`shortcut` is a shortcut that, when activated, will toggle the bottom panel's visibility. The shortcut object is only set when this control is added to the bottom panel.\n\n**Note** See the default editor bottom panel shortcuts in the Editor Settings for inspiration. By convention, they all use `Alt` modifier."]
        #[inline]
        pub fn add_control_to_bottom_panel(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >, title: impl AsArg < GString >,) -> Option < Gd < crate::classes::Button > > {
            self.add_control_to_bottom_panel_ex(control, title,) . done()
        }
        #[doc = "Adds a control to the bottom panel (together with Output, Debug, Animation, etc.). Returns a reference to a button that is outside the scene tree. It's up to you to hide/show the button when needed. When your plugin is deactivated, make sure to remove your custom control with [`remove_control_from_bottom_panel`][`crate::classes::EditorPlugin::remove_control_from_bottom_panel`] and free it with [`queue_free`][`crate::classes::Node::queue_free`].\n\n`shortcut` is a shortcut that, when activated, will toggle the bottom panel's visibility. The shortcut object is only set when this control is added to the bottom panel.\n\n**Note** See the default editor bottom panel shortcuts in the Editor Settings for inspiration. By convention, they all use `Alt` modifier."]
        #[inline]
        pub fn add_control_to_bottom_panel_ex < 'ex > (&'ex mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> > + 'ex, title: impl AsArg < GString > + 'ex,) -> ExAddControlToBottomPanel < 'ex > {
            ExAddControlToBottomPanel::new(self, control, title,)
        }
        #[doc = "Removes the control from the bottom panel. You have to manually [`queue_free`][`crate::classes::Node::queue_free`] the control."]
        pub fn remove_control_from_bottom_panel(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_control_from_bottom_panel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a script at `path` to the Autoload list as `name`."]
        pub fn add_autoload_singleton(&mut self, name: impl AsArg < GString >, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (name.into_arg(), path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_autoload_singleton", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes an Autoload `name` from the list."]
        pub fn remove_autoload_singleton(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_autoload_singleton", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Updates the overlays of the 2D and 3D editor viewport. Causes methods [`forward_canvas_draw_over_viewport`][`crate::classes::IEditorPlugin::forward_canvas_draw_over_viewport`], [`forward_canvas_force_draw_over_viewport`][`crate::classes::IEditorPlugin::forward_canvas_force_draw_over_viewport`], [`forward_3d_draw_over_viewport`][`crate::classes::IEditorPlugin::forward_3d_draw_over_viewport`] and [`forward_3d_force_draw_over_viewport`][`crate::classes::IEditorPlugin::forward_3d_force_draw_over_viewport`] to be called."]
        pub fn update_overlays(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "update_overlays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Makes a specific item in the bottom panel visible."]
        pub fn make_bottom_panel_item_visible(&mut self, item: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (item.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "make_bottom_panel_item_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Minimizes the bottom panel."]
        pub fn hide_bottom_panel(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "hide_bottom_panel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the undo/redo object. Most actions in the editor can be undoable, so use this object to make sure this happens when it's worth it."]
        pub fn get_undo_redo(&self,) -> Option < Gd < crate::classes::EditorUndoRedoManager > > {
            type CallRet = Option < Gd < crate::classes::EditorUndoRedoManager > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_undo_redo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Hooks a callback into the undo/redo action creation when a property is modified in the inspector. This allows, for example, to save other properties that may be lost when a given property is modified.\n\nThe callback should have 4 arguments: [`Object`][crate::classes::Object] `undo_redo`, [`Object`][crate::classes::Object] `modified_object`, [`String`][crate::builtin::GString] `property` and [`Variant`][crate::builtin::Variant] `new_value`. They are, respectively, the [`UndoRedo`][crate::classes::UndoRedo] object used by the inspector, the currently modified object, the name of the modified property and the new value the property is about to take."]
        pub fn add_undo_redo_inspector_hook_callback(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_undo_redo_inspector_hook_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a callback previously added by [`add_undo_redo_inspector_hook_callback`][`crate::classes::EditorPlugin::add_undo_redo_inspector_hook_callback`]."]
        pub fn remove_undo_redo_inspector_hook_callback(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_undo_redo_inspector_hook_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Queue save the project's editor layout."]
        pub fn queue_save_layout(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "queue_save_layout", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a custom translation parser plugin for extracting translatable strings from custom files."]
        pub fn add_translation_parser_plugin(&mut self, parser: impl AsArg < Option < Gd < crate::classes::EditorTranslationParserPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorTranslationParserPlugin > > >,);
            let args = (parser.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_translation_parser_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a custom translation parser plugin registered by [`add_translation_parser_plugin`][`crate::classes::EditorPlugin::add_translation_parser_plugin`]."]
        pub fn remove_translation_parser_plugin(&mut self, parser: impl AsArg < Option < Gd < crate::classes::EditorTranslationParserPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorTranslationParserPlugin > > >,);
            let args = (parser.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_translation_parser_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a new [`EditorImportPlugin`][crate::classes::EditorImportPlugin]. Import plugins are used to import custom and unsupported assets as a custom [`Resource`][crate::classes::Resource] type.\n\nIf `first_priority` is `true`, the new import plugin is inserted first in the list and takes precedence over pre-existing plugins.\n\n**Note:** If you want to import custom 3D asset formats use [`add_scene_format_importer_plugin`][`crate::classes::EditorPlugin::add_scene_format_importer_plugin`] instead.\n\nSee [`add_inspector_plugin`][`crate::classes::EditorPlugin::add_inspector_plugin`] for an example of how to register a plugin."]
        pub(crate) fn add_import_plugin_full(&mut self, importer: CowArg < Option < Gd < crate::classes::EditorImportPlugin > > >, first_priority: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorImportPlugin > > >, bool,);
            let args = (importer, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_import_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_import_plugin_ex`][Self::add_import_plugin_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Registers a new [`EditorImportPlugin`][crate::classes::EditorImportPlugin]. Import plugins are used to import custom and unsupported assets as a custom [`Resource`][crate::classes::Resource] type.\n\nIf `first_priority` is `true`, the new import plugin is inserted first in the list and takes precedence over pre-existing plugins.\n\n**Note:** If you want to import custom 3D asset formats use [`add_scene_format_importer_plugin`][`crate::classes::EditorPlugin::add_scene_format_importer_plugin`] instead.\n\nSee [`add_inspector_plugin`][`crate::classes::EditorPlugin::add_inspector_plugin`] for an example of how to register a plugin."]
        #[inline]
        pub fn add_import_plugin(&mut self, importer: impl AsArg < Option < Gd < crate::classes::EditorImportPlugin >> >,) {
            self.add_import_plugin_ex(importer,) . done()
        }
        #[doc = "Registers a new [`EditorImportPlugin`][crate::classes::EditorImportPlugin]. Import plugins are used to import custom and unsupported assets as a custom [`Resource`][crate::classes::Resource] type.\n\nIf `first_priority` is `true`, the new import plugin is inserted first in the list and takes precedence over pre-existing plugins.\n\n**Note:** If you want to import custom 3D asset formats use [`add_scene_format_importer_plugin`][`crate::classes::EditorPlugin::add_scene_format_importer_plugin`] instead.\n\nSee [`add_inspector_plugin`][`crate::classes::EditorPlugin::add_inspector_plugin`] for an example of how to register a plugin."]
        #[inline]
        pub fn add_import_plugin_ex < 'ex > (&'ex mut self, importer: impl AsArg < Option < Gd < crate::classes::EditorImportPlugin >> > + 'ex,) -> ExAddImportPlugin < 'ex > {
            ExAddImportPlugin::new(self, importer,)
        }
        #[doc = "Removes an import plugin registered by [`add_import_plugin`][`crate::classes::EditorPlugin::add_import_plugin`]."]
        pub fn remove_import_plugin(&mut self, importer: impl AsArg < Option < Gd < crate::classes::EditorImportPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorImportPlugin > > >,);
            let args = (importer.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_import_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a new [`EditorSceneFormatImporter`][crate::classes::EditorSceneFormatImporter]. Scene importers are used to import custom 3D asset formats as scenes.\n\nIf `first_priority` is `true`, the new import plugin is inserted first in the list and takes precedence over pre-existing plugins."]
        pub(crate) fn add_scene_format_importer_plugin_full(&mut self, scene_format_importer: CowArg < Option < Gd < crate::classes::EditorSceneFormatImporter > > >, first_priority: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorSceneFormatImporter > > >, bool,);
            let args = (scene_format_importer, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_scene_format_importer_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_scene_format_importer_plugin_ex`][Self::add_scene_format_importer_plugin_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Registers a new [`EditorSceneFormatImporter`][crate::classes::EditorSceneFormatImporter]. Scene importers are used to import custom 3D asset formats as scenes.\n\nIf `first_priority` is `true`, the new import plugin is inserted first in the list and takes precedence over pre-existing plugins."]
        #[inline]
        pub fn add_scene_format_importer_plugin(&mut self, scene_format_importer: impl AsArg < Option < Gd < crate::classes::EditorSceneFormatImporter >> >,) {
            self.add_scene_format_importer_plugin_ex(scene_format_importer,) . done()
        }
        #[doc = "Registers a new [`EditorSceneFormatImporter`][crate::classes::EditorSceneFormatImporter]. Scene importers are used to import custom 3D asset formats as scenes.\n\nIf `first_priority` is `true`, the new import plugin is inserted first in the list and takes precedence over pre-existing plugins."]
        #[inline]
        pub fn add_scene_format_importer_plugin_ex < 'ex > (&'ex mut self, scene_format_importer: impl AsArg < Option < Gd < crate::classes::EditorSceneFormatImporter >> > + 'ex,) -> ExAddSceneFormatImporterPlugin < 'ex > {
            ExAddSceneFormatImporterPlugin::new(self, scene_format_importer,)
        }
        #[doc = "Removes a scene format importer registered by [`add_scene_format_importer_plugin`][`crate::classes::EditorPlugin::add_scene_format_importer_plugin`]."]
        pub fn remove_scene_format_importer_plugin(&mut self, scene_format_importer: impl AsArg < Option < Gd < crate::classes::EditorSceneFormatImporter >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorSceneFormatImporter > > >,);
            let args = (scene_format_importer.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_scene_format_importer_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Add an [`EditorScenePostImportPlugin`][crate::classes::EditorScenePostImportPlugin]. These plugins allow customizing the import process of 3D assets by adding new options to the import dialogs.\n\nIf `first_priority` is `true`, the new import plugin is inserted first in the list and takes precedence over pre-existing plugins."]
        pub(crate) fn add_scene_post_import_plugin_full(&mut self, scene_import_plugin: CowArg < Option < Gd < crate::classes::EditorScenePostImportPlugin > > >, first_priority: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorScenePostImportPlugin > > >, bool,);
            let args = (scene_import_plugin, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_scene_post_import_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_scene_post_import_plugin_ex`][Self::add_scene_post_import_plugin_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Add an [`EditorScenePostImportPlugin`][crate::classes::EditorScenePostImportPlugin]. These plugins allow customizing the import process of 3D assets by adding new options to the import dialogs.\n\nIf `first_priority` is `true`, the new import plugin is inserted first in the list and takes precedence over pre-existing plugins."]
        #[inline]
        pub fn add_scene_post_import_plugin(&mut self, scene_import_plugin: impl AsArg < Option < Gd < crate::classes::EditorScenePostImportPlugin >> >,) {
            self.add_scene_post_import_plugin_ex(scene_import_plugin,) . done()
        }
        #[doc = "Add an [`EditorScenePostImportPlugin`][crate::classes::EditorScenePostImportPlugin]. These plugins allow customizing the import process of 3D assets by adding new options to the import dialogs.\n\nIf `first_priority` is `true`, the new import plugin is inserted first in the list and takes precedence over pre-existing plugins."]
        #[inline]
        pub fn add_scene_post_import_plugin_ex < 'ex > (&'ex mut self, scene_import_plugin: impl AsArg < Option < Gd < crate::classes::EditorScenePostImportPlugin >> > + 'ex,) -> ExAddScenePostImportPlugin < 'ex > {
            ExAddScenePostImportPlugin::new(self, scene_import_plugin,)
        }
        #[doc = "Remove the [`EditorScenePostImportPlugin`][crate::classes::EditorScenePostImportPlugin], added with [`add_scene_post_import_plugin`][`crate::classes::EditorPlugin::add_scene_post_import_plugin`]."]
        pub fn remove_scene_post_import_plugin(&mut self, scene_import_plugin: impl AsArg < Option < Gd < crate::classes::EditorScenePostImportPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorScenePostImportPlugin > > >,);
            let args = (scene_import_plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_scene_post_import_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a new [`EditorExportPlugin`][crate::classes::EditorExportPlugin]. Export plugins are used to perform tasks when the project is being exported.\n\nSee [`add_inspector_plugin`][`crate::classes::EditorPlugin::add_inspector_plugin`] for an example of how to register a plugin."]
        pub fn add_export_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorExportPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_export_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes an export plugin registered by [`add_export_plugin`][`crate::classes::EditorPlugin::add_export_plugin`]."]
        pub fn remove_export_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorExportPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_export_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a new [`EditorExportPlatform`][crate::classes::EditorExportPlatform]. Export platforms provides functionality of exporting to the specific platform."]
        pub fn add_export_platform(&mut self, platform: impl AsArg < Option < Gd < crate::classes::EditorExportPlatform >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPlatform > > >,);
            let args = (platform.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_export_platform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes an export platform registered by [`add_export_platform`][`crate::classes::EditorPlugin::add_export_platform`]."]
        pub fn remove_export_platform(&mut self, platform: impl AsArg < Option < Gd < crate::classes::EditorExportPlatform >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPlatform > > >,);
            let args = (platform.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_export_platform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a new [`EditorNode3DGizmoPlugin`][crate::classes::EditorNode3DGizmoPlugin]. Gizmo plugins are used to add custom gizmos to the 3D preview viewport for a [`Node3D`][crate::classes::Node3D].\n\nSee [`add_inspector_plugin`][`crate::classes::EditorPlugin::add_inspector_plugin`] for an example of how to register a plugin."]
        pub fn add_node_3d_gizmo_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorNode3DGizmoPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorNode3DGizmoPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_node_3d_gizmo_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a gizmo plugin registered by [`add_node_3d_gizmo_plugin`][`crate::classes::EditorPlugin::add_node_3d_gizmo_plugin`]."]
        pub fn remove_node_3d_gizmo_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorNode3DGizmoPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorNode3DGizmoPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_node_3d_gizmo_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a new [`EditorInspectorPlugin`][crate::classes::EditorInspectorPlugin]. Inspector plugins are used to extend [`EditorInspector`][crate::classes::EditorInspector] and provide custom configuration tools for your object's properties.\n\n**Note:** Always use [`remove_inspector_plugin`][`crate::classes::EditorPlugin::remove_inspector_plugin`] to remove the registered [`EditorInspectorPlugin`][crate::classes::EditorInspectorPlugin] when your `EditorPlugin` is disabled to prevent leaks and an unexpected behavior.\n\n\n```gdscript\nconst MyInspectorPlugin = preload(\"res://addons/your_addon/path/to/your/script.gd\")\nvar inspector_plugin = MyInspectorPlugin.new()\n\nfunc _enter_tree():\n\tadd_inspector_plugin(inspector_plugin)\n\nfunc _exit_tree():\n\tremove_inspector_plugin(inspector_plugin)\n```\n"]
        pub fn add_inspector_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorInspectorPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorInspectorPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_inspector_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes an inspector plugin registered by [`add_inspector_plugin`][`crate::classes::EditorPlugin::add_inspector_plugin`]."]
        pub fn remove_inspector_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorInspectorPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorInspectorPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_inspector_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a new [`EditorResourceConversionPlugin`][crate::classes::EditorResourceConversionPlugin]. Resource conversion plugins are used to add custom resource converters to the editor inspector.\n\nSee [`EditorResourceConversionPlugin`][crate::classes::EditorResourceConversionPlugin] for an example of how to create a resource conversion plugin."]
        pub fn add_resource_conversion_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorResourceConversionPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorResourceConversionPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_resource_conversion_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a resource conversion plugin registered by [`add_resource_conversion_plugin`][`crate::classes::EditorPlugin::add_resource_conversion_plugin`]."]
        pub fn remove_resource_conversion_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorResourceConversionPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorResourceConversionPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_resource_conversion_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Use this method if you always want to receive inputs from 3D view screen inside [`forward_3d_gui_input`][`crate::classes::IEditorPlugin::forward_3d_gui_input`]. It might be especially usable if your plugin will want to use raycast in the scene."]
        pub fn set_input_event_forwarding_always_enabled(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "set_input_event_forwarding_always_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables calling of [`forward_canvas_force_draw_over_viewport`][`crate::classes::IEditorPlugin::forward_canvas_force_draw_over_viewport`] for the 2D editor and [`forward_3d_force_draw_over_viewport`][`crate::classes::IEditorPlugin::forward_3d_force_draw_over_viewport`] for the 3D editor when their viewports are updated. You need to call this method only once and it will work permanently for this plugin."]
        pub fn set_force_draw_over_forwarding_enabled(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "set_force_draw_over_forwarding_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a plugin to the context menu. `slot` is the context menu where the plugin will be added.\n\n**Note:** A plugin instance can belong only to a single context menu slot."]
        pub fn add_context_menu_plugin(&mut self, slot: crate::classes::editor_context_menu_plugin::ContextMenuSlot, plugin: impl AsArg < Option < Gd < crate::classes::EditorContextMenuPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::editor_context_menu_plugin::ContextMenuSlot, CowArg < 'a0, Option < Gd < crate::classes::EditorContextMenuPlugin > > >,);
            let args = (slot, plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_context_menu_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the specified context menu plugin."]
        pub fn remove_context_menu_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorContextMenuPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorContextMenuPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_context_menu_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`EditorInterface`][crate::classes::EditorInterface] singleton instance."]
        pub fn get_editor_interface(&self,) -> Option < Gd < crate::classes::EditorInterface > > {
            type CallRet = Option < Gd < crate::classes::EditorInterface > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_editor_interface", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the Editor's dialog used for making scripts.\n\n**Note:** Users can configure it before use.\n\n**Warning:** Removing and freeing this node will render a part of the editor useless and may cause a crash."]
        pub fn get_script_create_dialog(&self,) -> Option < Gd < crate::classes::ScriptCreateDialog > > {
            type CallRet = Option < Gd < crate::classes::ScriptCreateDialog > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_script_create_dialog", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a [`Script`][crate::classes::Script] as debugger plugin to the Debugger. The script must extend [`EditorDebuggerPlugin`][crate::classes::EditorDebuggerPlugin]."]
        pub fn add_debugger_plugin(&mut self, script: impl AsArg < Option < Gd < crate::classes::EditorDebuggerPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorDebuggerPlugin > > >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_debugger_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the debugger plugin with given script from the Debugger."]
        pub fn remove_debugger_plugin(&mut self, script: impl AsArg < Option < Gd < crate::classes::EditorDebuggerPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorDebuggerPlugin > > >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_debugger_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Provide the version of the plugin declared in the `plugin.cfg` config file."]
        pub fn get_plugin_version(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_plugin_version", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" Creates a validated object for FFI boundary crossing."]
        #[doc = r""]
        #[doc = r" Low-level internal method. Validation (liveness/type checks) depend on safeguard level."]
        fn __validated_obj(&self) -> crate::obj::ValidatedObject {
            let raw_gd = unsafe {
                std::mem::transmute::< &Self, &crate::obj::RawGd < Self >> (self)
            };
            raw_gd.validated_object()
        }
        #[doc(hidden)]
        pub fn __object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
    }
    impl crate::obj::GodotClass for EditorPlugin {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorPlugin {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorPlugin {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorPlugin__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorPlugin > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_control_to_dock_ex`][super::EditorPlugin::add_control_to_dock_ex]."]
#[must_use]
pub struct ExAddControlToDock < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorPlugin, slot: crate::classes::editor_plugin::DockSlot, control: CowArg < 'ex, Option < Gd < crate::classes::Control > > >, shortcut: CowArg < 'ex, Option < Gd < crate::classes::Shortcut > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddControlToDock < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorPlugin, slot: crate::classes::editor_plugin::DockSlot, control: impl AsArg < Option < Gd < crate::classes::Control >> > + 'ex,) -> Self {
        let shortcut = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, slot: slot, control: control.into_arg(), shortcut: shortcut.into_arg(),
        }
    }
    #[inline]
    pub fn shortcut(self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex) -> Self {
        Self {
            shortcut: shortcut.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, slot, control, shortcut,
        }
        = self;
        re_export::EditorPlugin::add_control_to_dock_full(surround_object, slot, control, shortcut,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_control_to_bottom_panel_ex`][super::EditorPlugin::add_control_to_bottom_panel_ex]."]
#[must_use]
pub struct ExAddControlToBottomPanel < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorPlugin, control: CowArg < 'ex, Option < Gd < crate::classes::Control > > >, title: CowArg < 'ex, GString >, shortcut: CowArg < 'ex, Option < Gd < crate::classes::Shortcut > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddControlToBottomPanel < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorPlugin, control: impl AsArg < Option < Gd < crate::classes::Control >> > + 'ex, title: impl AsArg < GString > + 'ex,) -> Self {
        let shortcut = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, control: control.into_arg(), title: title.into_arg(), shortcut: shortcut.into_arg(),
        }
    }
    #[inline]
    pub fn shortcut(self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex) -> Self {
        Self {
            shortcut: shortcut.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Button > > {
        let Self {
            _phantom, surround_object, control, title, shortcut,
        }
        = self;
        re_export::EditorPlugin::add_control_to_bottom_panel_full(surround_object, control, title, shortcut,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_import_plugin_ex`][super::EditorPlugin::add_import_plugin_ex]."]
#[must_use]
pub struct ExAddImportPlugin < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorPlugin, importer: CowArg < 'ex, Option < Gd < crate::classes::EditorImportPlugin > > >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddImportPlugin < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorPlugin, importer: impl AsArg < Option < Gd < crate::classes::EditorImportPlugin >> > + 'ex,) -> Self {
        let first_priority = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, importer: importer.into_arg(), first_priority: first_priority,
        }
    }
    #[inline]
    pub fn first_priority(self, first_priority: bool) -> Self {
        Self {
            first_priority: first_priority, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, importer, first_priority,
        }
        = self;
        re_export::EditorPlugin::add_import_plugin_full(surround_object, importer, first_priority,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_scene_format_importer_plugin_ex`][super::EditorPlugin::add_scene_format_importer_plugin_ex]."]
#[must_use]
pub struct ExAddSceneFormatImporterPlugin < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorPlugin, scene_format_importer: CowArg < 'ex, Option < Gd < crate::classes::EditorSceneFormatImporter > > >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddSceneFormatImporterPlugin < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorPlugin, scene_format_importer: impl AsArg < Option < Gd < crate::classes::EditorSceneFormatImporter >> > + 'ex,) -> Self {
        let first_priority = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, scene_format_importer: scene_format_importer.into_arg(), first_priority: first_priority,
        }
    }
    #[inline]
    pub fn first_priority(self, first_priority: bool) -> Self {
        Self {
            first_priority: first_priority, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, scene_format_importer, first_priority,
        }
        = self;
        re_export::EditorPlugin::add_scene_format_importer_plugin_full(surround_object, scene_format_importer, first_priority,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_scene_post_import_plugin_ex`][super::EditorPlugin::add_scene_post_import_plugin_ex]."]
#[must_use]
pub struct ExAddScenePostImportPlugin < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorPlugin, scene_import_plugin: CowArg < 'ex, Option < Gd < crate::classes::EditorScenePostImportPlugin > > >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddScenePostImportPlugin < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorPlugin, scene_import_plugin: impl AsArg < Option < Gd < crate::classes::EditorScenePostImportPlugin >> > + 'ex,) -> Self {
        let first_priority = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, scene_import_plugin: scene_import_plugin.into_arg(), first_priority: first_priority,
        }
    }
    #[inline]
    pub fn first_priority(self, first_priority: bool) -> Self {
        Self {
            first_priority: first_priority, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, scene_import_plugin, first_priority,
        }
        = self;
        re_export::EditorPlugin::add_scene_post_import_plugin_full(surround_object, scene_import_plugin, first_priority,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CustomControlContainer {
    ord: i32
}
impl CustomControlContainer {
    #[doc(alias = "CONTAINER_TOOLBAR")]
    #[doc = "Godot enumerator name: `CONTAINER_TOOLBAR`"]
    pub const TOOLBAR: CustomControlContainer = CustomControlContainer {
        ord: 0i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_MENU")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_MENU`"]
    pub const SPATIAL_EDITOR_MENU: CustomControlContainer = CustomControlContainer {
        ord: 1i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_SIDE_LEFT")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_SIDE_LEFT`"]
    pub const SPATIAL_EDITOR_SIDE_LEFT: CustomControlContainer = CustomControlContainer {
        ord: 2i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_SIDE_RIGHT")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_SIDE_RIGHT`"]
    pub const SPATIAL_EDITOR_SIDE_RIGHT: CustomControlContainer = CustomControlContainer {
        ord: 3i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_BOTTOM")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_BOTTOM`"]
    pub const SPATIAL_EDITOR_BOTTOM: CustomControlContainer = CustomControlContainer {
        ord: 4i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_MENU")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_MENU`"]
    pub const CANVAS_EDITOR_MENU: CustomControlContainer = CustomControlContainer {
        ord: 5i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_SIDE_LEFT")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_SIDE_LEFT`"]
    pub const CANVAS_EDITOR_SIDE_LEFT: CustomControlContainer = CustomControlContainer {
        ord: 6i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_SIDE_RIGHT")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_SIDE_RIGHT`"]
    pub const CANVAS_EDITOR_SIDE_RIGHT: CustomControlContainer = CustomControlContainer {
        ord: 7i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_BOTTOM")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_BOTTOM`"]
    pub const CANVAS_EDITOR_BOTTOM: CustomControlContainer = CustomControlContainer {
        ord: 8i32
    };
    #[doc(alias = "CONTAINER_INSPECTOR_BOTTOM")]
    #[doc = "Godot enumerator name: `CONTAINER_INSPECTOR_BOTTOM`"]
    pub const INSPECTOR_BOTTOM: CustomControlContainer = CustomControlContainer {
        ord: 9i32
    };
    #[doc(alias = "CONTAINER_PROJECT_SETTING_TAB_LEFT")]
    #[doc = "Godot enumerator name: `CONTAINER_PROJECT_SETTING_TAB_LEFT`"]
    pub const PROJECT_SETTING_TAB_LEFT: CustomControlContainer = CustomControlContainer {
        ord: 10i32
    };
    #[doc(alias = "CONTAINER_PROJECT_SETTING_TAB_RIGHT")]
    #[doc = "Godot enumerator name: `CONTAINER_PROJECT_SETTING_TAB_RIGHT`"]
    pub const PROJECT_SETTING_TAB_RIGHT: CustomControlContainer = CustomControlContainer {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for CustomControlContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CustomControlContainer") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CustomControlContainer {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::TOOLBAR => "TOOLBAR", Self::SPATIAL_EDITOR_MENU => "SPATIAL_EDITOR_MENU", Self::SPATIAL_EDITOR_SIDE_LEFT => "SPATIAL_EDITOR_SIDE_LEFT", Self::SPATIAL_EDITOR_SIDE_RIGHT => "SPATIAL_EDITOR_SIDE_RIGHT", Self::SPATIAL_EDITOR_BOTTOM => "SPATIAL_EDITOR_BOTTOM", Self::CANVAS_EDITOR_MENU => "CANVAS_EDITOR_MENU", Self::CANVAS_EDITOR_SIDE_LEFT => "CANVAS_EDITOR_SIDE_LEFT", Self::CANVAS_EDITOR_SIDE_RIGHT => "CANVAS_EDITOR_SIDE_RIGHT", Self::CANVAS_EDITOR_BOTTOM => "CANVAS_EDITOR_BOTTOM", Self::INSPECTOR_BOTTOM => "INSPECTOR_BOTTOM", Self::PROJECT_SETTING_TAB_LEFT => "PROJECT_SETTING_TAB_LEFT", Self::PROJECT_SETTING_TAB_RIGHT => "PROJECT_SETTING_TAB_RIGHT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CustomControlContainer::TOOLBAR, CustomControlContainer::SPATIAL_EDITOR_MENU, CustomControlContainer::SPATIAL_EDITOR_SIDE_LEFT, CustomControlContainer::SPATIAL_EDITOR_SIDE_RIGHT, CustomControlContainer::SPATIAL_EDITOR_BOTTOM, CustomControlContainer::CANVAS_EDITOR_MENU, CustomControlContainer::CANVAS_EDITOR_SIDE_LEFT, CustomControlContainer::CANVAS_EDITOR_SIDE_RIGHT, CustomControlContainer::CANVAS_EDITOR_BOTTOM, CustomControlContainer::INSPECTOR_BOTTOM, CustomControlContainer::PROJECT_SETTING_TAB_LEFT, CustomControlContainer::PROJECT_SETTING_TAB_RIGHT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CustomControlContainer >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOOLBAR", "CONTAINER_TOOLBAR", CustomControlContainer::TOOLBAR), crate::meta::inspect::EnumConstant::new("SPATIAL_EDITOR_MENU", "CONTAINER_SPATIAL_EDITOR_MENU", CustomControlContainer::SPATIAL_EDITOR_MENU), crate::meta::inspect::EnumConstant::new("SPATIAL_EDITOR_SIDE_LEFT", "CONTAINER_SPATIAL_EDITOR_SIDE_LEFT", CustomControlContainer::SPATIAL_EDITOR_SIDE_LEFT), crate::meta::inspect::EnumConstant::new("SPATIAL_EDITOR_SIDE_RIGHT", "CONTAINER_SPATIAL_EDITOR_SIDE_RIGHT", CustomControlContainer::SPATIAL_EDITOR_SIDE_RIGHT), crate::meta::inspect::EnumConstant::new("SPATIAL_EDITOR_BOTTOM", "CONTAINER_SPATIAL_EDITOR_BOTTOM", CustomControlContainer::SPATIAL_EDITOR_BOTTOM), crate::meta::inspect::EnumConstant::new("CANVAS_EDITOR_MENU", "CONTAINER_CANVAS_EDITOR_MENU", CustomControlContainer::CANVAS_EDITOR_MENU), crate::meta::inspect::EnumConstant::new("CANVAS_EDITOR_SIDE_LEFT", "CONTAINER_CANVAS_EDITOR_SIDE_LEFT", CustomControlContainer::CANVAS_EDITOR_SIDE_LEFT), crate::meta::inspect::EnumConstant::new("CANVAS_EDITOR_SIDE_RIGHT", "CONTAINER_CANVAS_EDITOR_SIDE_RIGHT", CustomControlContainer::CANVAS_EDITOR_SIDE_RIGHT), crate::meta::inspect::EnumConstant::new("CANVAS_EDITOR_BOTTOM", "CONTAINER_CANVAS_EDITOR_BOTTOM", CustomControlContainer::CANVAS_EDITOR_BOTTOM), crate::meta::inspect::EnumConstant::new("INSPECTOR_BOTTOM", "CONTAINER_INSPECTOR_BOTTOM", CustomControlContainer::INSPECTOR_BOTTOM), crate::meta::inspect::EnumConstant::new("PROJECT_SETTING_TAB_LEFT", "CONTAINER_PROJECT_SETTING_TAB_LEFT", CustomControlContainer::PROJECT_SETTING_TAB_LEFT), crate::meta::inspect::EnumConstant::new("PROJECT_SETTING_TAB_RIGHT", "CONTAINER_PROJECT_SETTING_TAB_RIGHT", CustomControlContainer::PROJECT_SETTING_TAB_RIGHT)]
        }
    }
}
impl crate::meta::GodotConvert for CustomControlContainer {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Container Toolbar", 0i64), EnumeratorShape::new_int("Container Spatial Editor Menu", 1i64), EnumeratorShape::new_int("Container Spatial Editor Side Left", 2i64), EnumeratorShape::new_int("Container Spatial Editor Side Right", 3i64), EnumeratorShape::new_int("Container Spatial Editor Bottom", 4i64), EnumeratorShape::new_int("Container Canvas Editor Menu", 5i64), EnumeratorShape::new_int("Container Canvas Editor Side Left", 6i64), EnumeratorShape::new_int("Container Canvas Editor Side Right", 7i64), EnumeratorShape::new_int("Container Canvas Editor Bottom", 8i64), EnumeratorShape::new_int("Container Inspector Bottom", 9i64), EnumeratorShape::new_int("Container Project Setting Tab Left", 10i64), EnumeratorShape::new_int("Container Project Setting Tab Right", 11i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("EditorPlugin.CustomControlContainer")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CustomControlContainer {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CustomControlContainer {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CustomControlContainer {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for CustomControlContainer {
    
}
impl crate::meta::Element for CustomControlContainer {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DockSlot {
    ord: i32
}
impl DockSlot {
    #[doc(alias = "DOCK_SLOT_NONE")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_NONE`"]
    pub const NONE: DockSlot = DockSlot {
        ord: - 1i32
    };
    #[doc(alias = "DOCK_SLOT_LEFT_UL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_UL`"]
    pub const LEFT_UL: DockSlot = DockSlot {
        ord: 0i32
    };
    #[doc(alias = "DOCK_SLOT_LEFT_BL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_BL`"]
    pub const LEFT_BL: DockSlot = DockSlot {
        ord: 1i32
    };
    #[doc(alias = "DOCK_SLOT_LEFT_UR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_UR`"]
    pub const LEFT_UR: DockSlot = DockSlot {
        ord: 2i32
    };
    #[doc(alias = "DOCK_SLOT_LEFT_BR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_BR`"]
    pub const LEFT_BR: DockSlot = DockSlot {
        ord: 3i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_UL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_UL`"]
    pub const RIGHT_UL: DockSlot = DockSlot {
        ord: 4i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_BL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_BL`"]
    pub const RIGHT_BL: DockSlot = DockSlot {
        ord: 5i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_UR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_UR`"]
    pub const RIGHT_UR: DockSlot = DockSlot {
        ord: 6i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_BR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_BR`"]
    pub const RIGHT_BR: DockSlot = DockSlot {
        ord: 7i32
    };
    #[doc(alias = "DOCK_SLOT_BOTTOM")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_BOTTOM`"]
    pub const BOTTOM: DockSlot = DockSlot {
        ord: 8i32
    };
    #[doc(alias = "DOCK_SLOT_MAX")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_MAX`"]
    pub const MAX: DockSlot = DockSlot {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for DockSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DockSlot") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DockSlot {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ - 1i32 | ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "NONE", Self::LEFT_UL => "LEFT_UL", Self::LEFT_BL => "LEFT_BL", Self::LEFT_UR => "LEFT_UR", Self::LEFT_BR => "LEFT_BR", Self::RIGHT_UL => "RIGHT_UL", Self::RIGHT_BL => "RIGHT_BL", Self::RIGHT_UR => "RIGHT_UR", Self::RIGHT_BR => "RIGHT_BR", Self::BOTTOM => "BOTTOM", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DockSlot::NONE, DockSlot::LEFT_UL, DockSlot::LEFT_BL, DockSlot::LEFT_UR, DockSlot::LEFT_BR, DockSlot::RIGHT_UL, DockSlot::RIGHT_BL, DockSlot::RIGHT_UR, DockSlot::RIGHT_BR, DockSlot::BOTTOM, DockSlot::MAX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DockSlot >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "DOCK_SLOT_NONE", DockSlot::NONE), crate::meta::inspect::EnumConstant::new("LEFT_UL", "DOCK_SLOT_LEFT_UL", DockSlot::LEFT_UL), crate::meta::inspect::EnumConstant::new("LEFT_BL", "DOCK_SLOT_LEFT_BL", DockSlot::LEFT_BL), crate::meta::inspect::EnumConstant::new("LEFT_UR", "DOCK_SLOT_LEFT_UR", DockSlot::LEFT_UR), crate::meta::inspect::EnumConstant::new("LEFT_BR", "DOCK_SLOT_LEFT_BR", DockSlot::LEFT_BR), crate::meta::inspect::EnumConstant::new("RIGHT_UL", "DOCK_SLOT_RIGHT_UL", DockSlot::RIGHT_UL), crate::meta::inspect::EnumConstant::new("RIGHT_BL", "DOCK_SLOT_RIGHT_BL", DockSlot::RIGHT_BL), crate::meta::inspect::EnumConstant::new("RIGHT_UR", "DOCK_SLOT_RIGHT_UR", DockSlot::RIGHT_UR), crate::meta::inspect::EnumConstant::new("RIGHT_BR", "DOCK_SLOT_RIGHT_BR", DockSlot::RIGHT_BR), crate::meta::inspect::EnumConstant::new("BOTTOM", "DOCK_SLOT_BOTTOM", DockSlot::BOTTOM), crate::meta::inspect::EnumConstant::new("MAX", "DOCK_SLOT_MAX", DockSlot::MAX)]
        }
    }
}
impl crate::meta::GodotConvert for DockSlot {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Dock Slot None", - 1i64), EnumeratorShape::new_int("Dock Slot Left Ul", 0i64), EnumeratorShape::new_int("Dock Slot Left Bl", 1i64), EnumeratorShape::new_int("Dock Slot Left Ur", 2i64), EnumeratorShape::new_int("Dock Slot Left Br", 3i64), EnumeratorShape::new_int("Dock Slot Right Ul", 4i64), EnumeratorShape::new_int("Dock Slot Right Bl", 5i64), EnumeratorShape::new_int("Dock Slot Right Ur", 6i64), EnumeratorShape::new_int("Dock Slot Right Br", 7i64), EnumeratorShape::new_int("Dock Slot Bottom", 8i64), EnumeratorShape::new_int("Dock Slot Max", 9i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("EditorPlugin.DockSlot")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DockSlot {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DockSlot {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DockSlot {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for DockSlot {
    
}
impl crate::meta::Element for DockSlot {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `AfterGUIInput`."]
pub struct AfterGuiInput {
    ord: i32
}
impl AfterGuiInput {
    #[doc(alias = "AFTER_GUI_INPUT_PASS")]
    #[doc = "Godot enumerator name: `AFTER_GUI_INPUT_PASS`"]
    pub const PASS: AfterGuiInput = AfterGuiInput {
        ord: 0i32
    };
    #[doc(alias = "AFTER_GUI_INPUT_STOP")]
    #[doc = "Godot enumerator name: `AFTER_GUI_INPUT_STOP`"]
    pub const STOP: AfterGuiInput = AfterGuiInput {
        ord: 1i32
    };
    #[doc(alias = "AFTER_GUI_INPUT_CUSTOM")]
    #[doc = "Godot enumerator name: `AFTER_GUI_INPUT_CUSTOM`"]
    pub const CUSTOM: AfterGuiInput = AfterGuiInput {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AfterGuiInput {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AfterGuiInput") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AfterGuiInput {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PASS => "PASS", Self::STOP => "STOP", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AfterGuiInput::PASS, AfterGuiInput::STOP, AfterGuiInput::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AfterGuiInput >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PASS", "AFTER_GUI_INPUT_PASS", AfterGuiInput::PASS), crate::meta::inspect::EnumConstant::new("STOP", "AFTER_GUI_INPUT_STOP", AfterGuiInput::STOP), crate::meta::inspect::EnumConstant::new("CUSTOM", "AFTER_GUI_INPUT_CUSTOM", AfterGuiInput::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for AfterGuiInput {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("After Gui Input Pass", 0i64), EnumeratorShape::new_int("After Gui Input Stop", 1i64), EnumeratorShape::new_int("After Gui Input Custom", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("EditorPlugin.AfterGUIInput")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AfterGuiInput {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AfterGuiInput {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AfterGuiInput {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for AfterGuiInput {
    
}
impl crate::meta::Element for AfterGuiInput {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorPlugin;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`EditorPlugin`][crate::classes::EditorPlugin] class."]
    pub struct SignalsOfEditorPlugin < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfEditorPlugin < 'c, C > {
        #[doc = "Signature: `(scene_root: Gd<Node>)`"]
        pub fn scene_changed(&mut self) -> SigSceneChanged < 'c, C > {
            SigSceneChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "scene_changed")
            }
        }
        #[doc = "Signature: `(filepath: GString)`"]
        pub fn scene_closed(&mut self) -> SigSceneClosed < 'c, C > {
            SigSceneClosed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "scene_closed")
            }
        }
        #[doc = "Signature: `(screen_name: GString)`"]
        pub fn main_screen_changed(&mut self) -> SigMainScreenChanged < 'c, C > {
            SigMainScreenChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "main_screen_changed")
            }
        }
        #[doc = "Signature: `(resource: Gd<Resource>)`"]
        pub fn resource_saved(&mut self) -> SigResourceSaved < 'c, C > {
            SigResourceSaved {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "resource_saved")
            }
        }
        #[doc = "Signature: `(filepath: GString)`"]
        pub fn scene_saved(&mut self) -> SigSceneSaved < 'c, C > {
            SigSceneSaved {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "scene_saved")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn project_settings_changed(&mut self) -> SigProjectSettingsChanged < 'c, C > {
            SigProjectSettingsChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "project_settings_changed")
            }
        }
    }
    type TypedSigSceneChanged < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigSceneChanged < 'c, C: WithSignals > {
        typed: TypedSigSceneChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSceneChanged < 'c, C > {
        pub fn emit(&mut self, scene_root: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((scene_root,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSceneChanged < 'c, C > {
        type Target = TypedSigSceneChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSceneChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSceneClosed < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigSceneClosed < 'c, C: WithSignals > {
        typed: TypedSigSceneClosed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSceneClosed < 'c, C > {
        pub fn emit(&mut self, filepath: GString,) {
            self.typed.emit_tuple((filepath,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSceneClosed < 'c, C > {
        type Target = TypedSigSceneClosed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSceneClosed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMainScreenChanged < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigMainScreenChanged < 'c, C: WithSignals > {
        typed: TypedSigMainScreenChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMainScreenChanged < 'c, C > {
        pub fn emit(&mut self, screen_name: GString,) {
            self.typed.emit_tuple((screen_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMainScreenChanged < 'c, C > {
        type Target = TypedSigMainScreenChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMainScreenChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigResourceSaved < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Resource >,) >;
    pub struct SigResourceSaved < 'c, C: WithSignals > {
        typed: TypedSigResourceSaved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigResourceSaved < 'c, C > {
        pub fn emit(&mut self, resource: Gd < crate::classes::Resource >,) {
            self.typed.emit_tuple((resource,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigResourceSaved < 'c, C > {
        type Target = TypedSigResourceSaved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigResourceSaved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSceneSaved < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigSceneSaved < 'c, C: WithSignals > {
        typed: TypedSigSceneSaved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSceneSaved < 'c, C > {
        pub fn emit(&mut self, filepath: GString,) {
            self.typed.emit_tuple((filepath,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSceneSaved < 'c, C > {
        type Target = TypedSigSceneSaved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSceneSaved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigProjectSettingsChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigProjectSettingsChanged < 'c, C: WithSignals > {
        typed: TypedSigProjectSettingsChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigProjectSettingsChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigProjectSettingsChanged < 'c, C > {
        type Target = TypedSigProjectSettingsChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigProjectSettingsChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for EditorPlugin {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfEditorPlugin < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfEditorPlugin < 'c, C > {
        type Target = < < EditorPlugin as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = EditorPlugin;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfEditorPlugin < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = EditorPlugin;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}