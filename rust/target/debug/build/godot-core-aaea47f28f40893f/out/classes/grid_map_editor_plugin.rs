#![doc = "Sidecar module for class [`GridMapEditorPlugin`][crate::classes::GridMapEditorPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GridMapEditorPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_gridmapeditorplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GridMapEditorPlugin`.\n\nInherits [`EditorPlugin`][crate::classes::EditorPlugin].\n\nRelated symbols:\n\n* [`IGridMapEditorPlugin`][crate::classes::IGridMapEditorPlugin]: virtual methods\n\n\nSee also [Godot docs for `GridMapEditorPlugin`](https://docs.godotengine.org/en/stable/classes/class_gridmapeditorplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GridMapEditorPlugin::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nGridMapEditorPlugin provides access to the [`GridMap`][crate::classes::GridMap] editor functionality."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GridMapEditorPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GridMapEditorPlugin`][crate::classes::GridMapEditorPlugin].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IEditorPlugin`][crate::classes::IEditorPlugin] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GridMapEditorPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_gridmapeditorplugin.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGridMapEditorPlugin: crate::obj::GodotClass < Base = GridMapEditorPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GridMapEditorPlugin {
        #[doc = "Returns the [`GridMap`][crate::classes::GridMap] node currently edited by the grid map editor."]
        pub fn get_current_grid_map(&self,) -> Option < Gd < crate::classes::GridMap > > {
            type CallRet = Option < Gd < crate::classes::GridMap > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(5usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMapEditorPlugin", "get_current_grid_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects the cells inside the given bounds from `begin` to `end`."]
        pub fn set_selection(&mut self, begin: Vector3i, end: Vector3i,) {
            type CallRet = ();
            type CallParams = (Vector3i, Vector3i,);
            let args = (begin, end,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(6usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMapEditorPlugin", "set_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deselects any currently selected cells."]
        pub fn clear_selection(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(7usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMapEditorPlugin", "clear_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the cell coordinate bounds of the current selection. Use [`has_selection`][`crate::classes::GridMapEditorPlugin::has_selection`] to check if there is an active selection."]
        pub fn get_selection(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(8usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMapEditorPlugin", "get_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there are selected cells."]
        pub fn has_selection(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(9usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMapEditorPlugin", "has_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of [`Vector3i`][crate::builtin::Vector3i]s with the selected cells' coordinates."]
        pub fn get_selected_cells(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(10usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMapEditorPlugin", "get_selected_cells", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects the [`MeshLibrary`][crate::classes::MeshLibrary] item with the given index in the grid map editor's palette. If a negative index is given, no item will be selected. If a value greater than the last index is given, the last item will be selected.\n\n**Note:** The indices might not be in the same order as they appear in the editor's interface."]
        pub fn set_selected_palette_item(&self, item: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(11usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMapEditorPlugin", "set_selected_palette_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the selected [`MeshLibrary`][crate::classes::MeshLibrary] item in the grid map editor's palette or `-1` if no item is selected.\n\n**Note:** The indices might not be in the same order as they appear in the editor's interface."]
        pub fn get_selected_palette_item(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(12usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMapEditorPlugin", "get_selected_palette_item", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for GridMapEditorPlugin {
        type Base = crate::classes::EditorPlugin;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GridMapEditorPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for GridMapEditorPlugin {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::EditorPlugin > for GridMapEditorPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GridMapEditorPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GridMapEditorPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for GridMapEditorPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GridMapEditorPlugin {
        type Target = crate::classes::EditorPlugin;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GridMapEditorPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GridMapEditorPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GridMapEditorPlugin__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GridMapEditorPlugin > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorPlugin > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GridMapEditorPlugin;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::editor_plugin::SignalsOfEditorPlugin;
    impl WithSignals for GridMapEditorPlugin {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfEditorPlugin < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}