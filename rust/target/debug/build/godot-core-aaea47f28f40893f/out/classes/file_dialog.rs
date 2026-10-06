#![doc = "Sidecar module for class [`FileDialog`][crate::classes::FileDialog].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FileDialog` enums](https://docs.godotengine.org/en/stable/classes/class_filedialog.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FileDialog`.\n\nInherits [`ConfirmationDialog`][crate::classes::ConfirmationDialog].\n\nRelated symbols:\n\n* [`file_dialog`][crate::classes::file_dialog]: sidecar module with related enum/flag types\n* [`IFileDialog`][crate::classes::IFileDialog]: virtual methods\n* [`SignalsOfFileDialog`][crate::classes::file_dialog::SignalsOfFileDialog]: signal collection\n\n\nSee also [Godot docs for `FileDialog`](https://docs.godotengine.org/en/stable/classes/class_filedialog.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`FileDialog::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\n`FileDialog` is a preset dialog used to choose files and directories in the filesystem. It supports filter masks. `FileDialog` automatically sets its window title according to the \\[member file_mode]. If you want to use a custom title, disable this by setting \\[member mode_overrides_title] to `false`.\n\n**Note:** `FileDialog` is invisible by default. To make it visible, call one of the `popup_*` methods from [`Window`][crate::classes::Window] on the node, such as [`popup_centered_clamped`][`crate::classes::Window::popup_centered_clamped`]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FileDialog {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`FileDialog`][crate::classes::FileDialog].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IConfirmationDialog`][crate::classes::IConfirmationDialog] > [`IAcceptDialog`][crate::classes::IAcceptDialog] > [`IWindow`][crate::classes::IWindow] > ~~`IViewport`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `FileDialog` methods](https://docs.godotengine.org/en/stable/classes/class_filedialog.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFileDialog: crate::obj::GodotClass < Base = FileDialog > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: WindowNotification) {
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
        #[doc = "Virtual method to be implemented by the user. Overrides the value returned by [`get_contents_minimum_size`][`crate::classes::Window::get_contents_minimum_size`]."]
        fn get_contents_minimum_size(&self,) -> Vector2 {
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
    impl FileDialog {
        #[doc = "Clear all the added filters in the dialog."]
        pub fn clear_filters(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9929usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "clear_filters", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a comma-separated file extension `filter` and comma-separated MIME type `mime_type` option to the `FileDialog` with an optional `description`, which restricts what files can be picked.\n\nA `filter` should be of the form `\"filename.extension\"`, where filename and extension can be `*` to match any string. Filters starting with `.` (i.e. empty filenames) are not allowed.\n\nFor example, a `filter` of `\"*.png, *.jpg\"`, a `mime_type` of `image/png, image/jpeg`, and a `description` of `\"Images\"` results in filter text \"Images (*.png, *.jpg)\".\n\n**Note:** Embedded file dialogs and Windows file dialogs support only file extensions, while Android, Linux, and macOS file dialogs also support MIME types."]
        pub(crate) fn add_filter_full(&mut self, filter: CowArg < GString >, description: CowArg < GString >, mime_type: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >,);
            let args = (filter, description, mime_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9930usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "add_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_filter_ex`][Self::add_filter_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a comma-separated file extension `filter` and comma-separated MIME type `mime_type` option to the `FileDialog` with an optional `description`, which restricts what files can be picked.\n\nA `filter` should be of the form `\"filename.extension\"`, where filename and extension can be `*` to match any string. Filters starting with `.` (i.e. empty filenames) are not allowed.\n\nFor example, a `filter` of `\"*.png, *.jpg\"`, a `mime_type` of `image/png, image/jpeg`, and a `description` of `\"Images\"` results in filter text \"Images (*.png, *.jpg)\".\n\n**Note:** Embedded file dialogs and Windows file dialogs support only file extensions, while Android, Linux, and macOS file dialogs also support MIME types."]
        #[inline]
        pub fn add_filter(&mut self, filter: impl AsArg < GString >,) {
            self.add_filter_ex(filter,) . done()
        }
        #[doc = "Adds a comma-separated file extension `filter` and comma-separated MIME type `mime_type` option to the `FileDialog` with an optional `description`, which restricts what files can be picked.\n\nA `filter` should be of the form `\"filename.extension\"`, where filename and extension can be `*` to match any string. Filters starting with `.` (i.e. empty filenames) are not allowed.\n\nFor example, a `filter` of `\"*.png, *.jpg\"`, a `mime_type` of `image/png, image/jpeg`, and a `description` of `\"Images\"` results in filter text \"Images (*.png, *.jpg)\".\n\n**Note:** Embedded file dialogs and Windows file dialogs support only file extensions, while Android, Linux, and macOS file dialogs also support MIME types."]
        #[inline]
        pub fn add_filter_ex < 'ex > (&'ex mut self, filter: impl AsArg < GString > + 'ex,) -> ExAddFilter < 'ex > {
            ExAddFilter::new(self, filter,)
        }
        pub fn set_filters(&mut self, filters: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(filters),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9931usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_filters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_filters(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9932usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_filters", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clear the filter for file names."]
        pub fn clear_filename_filter(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "clear_filename_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_filename_filter(&mut self, filter: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (filter.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_filename_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_filename_filter(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_filename_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the [`OptionButton`][crate::classes::OptionButton] or [`CheckBox`][crate::classes::CheckBox] with index `option`."]
        pub fn get_option_name(&self, option: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_option_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of values of the [`OptionButton`][crate::classes::OptionButton] with index `option`."]
        pub fn get_option_values(&self, option: i32,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_option_values", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the default value index of the [`OptionButton`][crate::classes::OptionButton] or [`CheckBox`][crate::classes::CheckBox] with index `option`."]
        pub fn get_option_default(&self, option: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_option_default", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the name of the [`OptionButton`][crate::classes::OptionButton] or [`CheckBox`][crate::classes::CheckBox] with index `option`."]
        pub fn set_option_name(&mut self, option: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (option, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_option_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the option values of the [`OptionButton`][crate::classes::OptionButton] with index `option`."]
        pub fn set_option_values(&mut self, option: i32, values: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, PackedStringArray >,);
            let args = (option, RefArg::new(values),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_option_values", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default value index of the [`OptionButton`][crate::classes::OptionButton] or [`CheckBox`][crate::classes::CheckBox] with index `option`."]
        pub fn set_option_default(&mut self, option: i32, default_value_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (option, default_value_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_option_default", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_option_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_option_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_option_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_option_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an additional [`OptionButton`][crate::classes::OptionButton] to the file dialog. If `values` is empty, a [`CheckBox`][crate::classes::CheckBox] is added instead.\n\n`default_value_index` should be an index of the value in the `values`. If `values` is empty it should be either `1` (checked), or `0` (unchecked)."]
        pub fn add_option(&mut self, name: impl AsArg < GString >, values: &PackedStringArray, default_value_index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, i32,);
            let args = (name.into_arg(), RefArg::new(values), default_value_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "add_option", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Dictionary`][crate::builtin::Dictionary] with the selected values of the additional [`OptionButton`][crate::classes::OptionButton]s and/or [`CheckBox`][crate::classes::CheckBox]es. [`Dictionary`][crate::builtin::Dictionary] keys are names and values are selected value indices."]
        pub fn get_selected_options(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_selected_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_current_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_file(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_current_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_current_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_dir(&mut self, dir: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (dir.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_current_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_file(&mut self, file: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_current_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_path(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9951usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_current_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mode_overrides_title(&mut self, override_: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (override_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9952usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_mode_overrides_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_mode_overriding_title(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9953usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "is_mode_overriding_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_file_mode(&mut self, mode: crate::classes::file_dialog::FileMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::file_dialog::FileMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9954usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_file_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_file_mode(&self,) -> crate::classes::file_dialog::FileMode {
            type CallRet = crate::classes::file_dialog::FileMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9955usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_file_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_display_mode(&mut self, mode: crate::classes::file_dialog::DisplayMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::file_dialog::DisplayMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9956usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_display_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_display_mode(&self,) -> crate::classes::file_dialog::DisplayMode {
            type CallRet = crate::classes::file_dialog::DisplayMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9957usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_display_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the vertical box container of the dialog, custom controls can be added to it.\n\n**Warning:** This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their \\[member CanvasItem.visible] property.\n\n**Note:** Changes to this node are ignored by native file dialogs, use [`add_option`][`crate::classes::FileDialog::add_option`] to add custom elements to the dialog instead."]
        pub fn get_vbox(&self,) -> Option < Gd < crate::classes::VBoxContainer > > {
            type CallRet = Option < Gd < crate::classes::VBoxContainer > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9958usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_vbox", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the LineEdit for the selected file.\n\n**Warning:** This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their \\[member CanvasItem.visible] property."]
        pub fn get_line_edit(&self,) -> Option < Gd < crate::classes::LineEdit > > {
            type CallRet = Option < Gd < crate::classes::LineEdit > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9959usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_line_edit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_access(&mut self, access: crate::classes::file_dialog::Access,) {
            type CallRet = ();
            type CallParams = (crate::classes::file_dialog::Access,);
            let args = (access,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9960usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_access", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_access(&self,) -> crate::classes::file_dialog::Access {
            type CallRet = crate::classes::file_dialog::Access;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9961usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_access", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_root_subfolder(&mut self, dir: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (dir.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9962usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_root_subfolder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_subfolder(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9963usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_root_subfolder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_show_hidden_files(&mut self, show: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (show,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9964usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_show_hidden_files", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_showing_hidden_files(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9965usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "is_showing_hidden_files", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_native_dialog(&mut self, native: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (native,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9966usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_use_native_dialog", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_native_dialog(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9967usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_use_native_dialog", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the specified customization `flag`, allowing to customize the features available in this `FileDialog`."]
        pub fn set_customization_flag_enabled(&mut self, flag: crate::classes::file_dialog::Customization, enabled: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::file_dialog::Customization, bool,);
            let args = (flag, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9968usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_customization_flag_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the provided `flag` is enabled."]
        pub fn is_customization_flag_enabled(&self, flag: crate::classes::file_dialog::Customization,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::file_dialog::Customization,);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9969usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "is_customization_flag_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clear all currently selected items in the dialog."]
        pub fn deselect_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9970usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "deselect_all", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the list of favorite directories, which is shared by all `FileDialog` nodes. Useful to restore the list of favorites saved with [`get_favorite_list`][`crate::classes::FileDialog::get_favorite_list`]. This method can be called only from the main thread.\n\n**Note:** `FileDialog` will update its internal [`ItemList`][crate::classes::ItemList] of favorites when its visibility changes. Be sure to call this method earlier if you want your changes to have effect."]
        pub fn set_favorite_list(favorites: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(favorites),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9971usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_favorite_list", None, args,)
            }
        }
        #[doc = "Returns the list of favorite directories, which is shared by all `FileDialog` nodes. Useful to store the list of favorites between project sessions. This method can be called only from the main thread."]
        pub fn get_favorite_list() -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9972usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_favorite_list", None, args,)
            }
        }
        #[doc = "Sets the list of recent directories, which is shared by all `FileDialog` nodes. Useful to restore the list of recents saved with [`set_recent_list`][`crate::classes::FileDialog::set_recent_list`]. This method can be called only from the main thread.\n\n**Note:** `FileDialog` will update its internal [`ItemList`][crate::classes::ItemList] of recent directories when its visibility changes. Be sure to call this method earlier if you want your changes to have effect."]
        pub fn set_recent_list(recents: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(recents),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9973usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_recent_list", None, args,)
            }
        }
        #[doc = "Returns the list of recent directories, which is shared by all `FileDialog` nodes. Useful to store the list of recents between project sessions. This method can be called only from the main thread."]
        pub fn get_recent_list() -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9974usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "get_recent_list", None, args,)
            }
        }
        #[doc = "Sets the callback used by the `FileDialog` nodes to get a file icon, when [`DisplayMode::LIST`][`crate::classes::file_dialog::DisplayMode::LIST`] mode is used. The callback should take a single [`String`][crate::builtin::GString] argument (file path), and return a [`Texture2D`][crate::classes::Texture2D]. If an invalid texture is returned, the [theme_item file] icon will be used instead."]
        pub fn set_get_icon_callback(callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9975usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_get_icon_callback", None, args,)
            }
        }
        #[doc = "Sets the callback used by the `FileDialog` nodes to get a file icon, when [`DisplayMode::THUMBNAILS`][`crate::classes::file_dialog::DisplayMode::THUMBNAILS`] mode is used. The callback should take a single [`String`][crate::builtin::GString] argument (file path), and return a [`Texture2D`][crate::classes::Texture2D]. If an invalid texture is returned, the [theme_item file_thumbnail] icon will be used instead.\n\nThumbnails are usually more complex and may take a while to load. To avoid stalling the application, you can use [`ImageTexture`][crate::classes::ImageTexture] to asynchronously create the thumbnail.\n\n```gdscript\nfunc _ready():\n\tFileDialog.set_get_thumbnail_callback(thumbnail_method)\n\nfunc thumbnail_method(path):\n\tvar image_texture = ImageTexture.new()\n\tmake_thumbnail_async(path, image_texture)\n\treturn image_texture\n\nfunc make_thumbnail_async(path, image_texture):\n\tvar thumbnail_texture = await generate_thumbnail(path) # Some method that generates a thumbnail.\n\timage_texture.set_image(thumbnail_texture.get_image())\n```"]
        pub fn set_get_thumbnail_callback(callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9976usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "set_get_thumbnail_callback", None, args,)
            }
        }
        #[doc = "Shows the `FileDialog` using the default size and position for file dialogs, and selects the file name if there is a current file."]
        pub fn popup_file_dialog(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9977usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "popup_file_dialog", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Invalidates and updates this dialog's content list.\n\n**Note:** This method does nothing on native file dialogs."]
        pub fn invalidate(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileDialog", "invalidate", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for FileDialog {
        type Base = crate::classes::ConfirmationDialog;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("FileDialog"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FileDialog {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::ConfirmationDialog > for FileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AcceptDialog > for FileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Window > for FileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Viewport > for FileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for FileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FileDialog {
        
    }
    impl crate::obj::cap::GodotDefault for FileDialog {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FileDialog {
        type Target = crate::classes::ConfirmationDialog;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FileDialog {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`FileDialog`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_FileDialog__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::FileDialog > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::ConfirmationDialog > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AcceptDialog > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Window > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Viewport > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`FileDialog::add_filter_ex`][super::FileDialog::add_filter_ex]."]
#[must_use]
pub struct ExAddFilter < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::FileDialog, filter: CowArg < 'ex, GString >, description: CowArg < 'ex, GString >, mime_type: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddFilter < 'ex > {
    fn new(surround_object: &'ex mut re_export::FileDialog, filter: impl AsArg < GString > + 'ex,) -> Self {
        let description = GString::from("");
        let mime_type = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, filter: filter.into_arg(), description: CowArg::Owned(description), mime_type: CowArg::Owned(mime_type),
        }
    }
    #[inline]
    pub fn description(self, description: impl AsArg < GString > + 'ex) -> Self {
        Self {
            description: description.into_arg(), .. self
        }
    }
    #[inline]
    pub fn mime_type(self, mime_type: impl AsArg < GString > + 'ex) -> Self {
        Self {
            mime_type: mime_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, filter, description, mime_type,
        }
        = self;
        re_export::FileDialog::add_filter_full(surround_object, filter, description, mime_type,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileMode {
    ord: i32
}
impl FileMode {
    #[doc(alias = "FILE_MODE_OPEN_FILE")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_FILE`"]
    pub const OPEN_FILE: FileMode = FileMode {
        ord: 0i32
    };
    #[doc(alias = "FILE_MODE_OPEN_FILES")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_FILES`"]
    pub const OPEN_FILES: FileMode = FileMode {
        ord: 1i32
    };
    #[doc(alias = "FILE_MODE_OPEN_DIR")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_DIR`"]
    pub const OPEN_DIR: FileMode = FileMode {
        ord: 2i32
    };
    #[doc(alias = "FILE_MODE_OPEN_ANY")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_ANY`"]
    pub const OPEN_ANY: FileMode = FileMode {
        ord: 3i32
    };
    #[doc(alias = "FILE_MODE_SAVE_FILE")]
    #[doc = "Godot enumerator name: `FILE_MODE_SAVE_FILE`"]
    pub const SAVE_FILE: FileMode = FileMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for FileMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FileMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FileMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::OPEN_FILE => "OPEN_FILE", Self::OPEN_FILES => "OPEN_FILES", Self::OPEN_DIR => "OPEN_DIR", Self::OPEN_ANY => "OPEN_ANY", Self::SAVE_FILE => "SAVE_FILE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FileMode::OPEN_FILE, FileMode::OPEN_FILES, FileMode::OPEN_DIR, FileMode::OPEN_ANY, FileMode::SAVE_FILE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FileMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OPEN_FILE", "FILE_MODE_OPEN_FILE", FileMode::OPEN_FILE), crate::meta::inspect::EnumConstant::new("OPEN_FILES", "FILE_MODE_OPEN_FILES", FileMode::OPEN_FILES), crate::meta::inspect::EnumConstant::new("OPEN_DIR", "FILE_MODE_OPEN_DIR", FileMode::OPEN_DIR), crate::meta::inspect::EnumConstant::new("OPEN_ANY", "FILE_MODE_OPEN_ANY", FileMode::OPEN_ANY), crate::meta::inspect::EnumConstant::new("SAVE_FILE", "FILE_MODE_SAVE_FILE", FileMode::SAVE_FILE)]
        }
    }
}
impl crate::meta::GodotConvert for FileMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("File Mode Open File", 0i64), EnumeratorShape::new_int("File Mode Open Files", 1i64), EnumeratorShape::new_int("File Mode Open Dir", 2i64), EnumeratorShape::new_int("File Mode Open Any", 3i64), EnumeratorShape::new_int("File Mode Save File", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("FileDialog.FileMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for FileMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FileMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FileMode {
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
impl crate::registry::property::Export for FileMode {
    
}
impl crate::meta::Element for FileMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Access {
    ord: i32
}
impl Access {
    #[doc(alias = "ACCESS_RESOURCES")]
    #[doc = "Godot enumerator name: `ACCESS_RESOURCES`"]
    pub const RESOURCES: Access = Access {
        ord: 0i32
    };
    #[doc(alias = "ACCESS_USERDATA")]
    #[doc = "Godot enumerator name: `ACCESS_USERDATA`"]
    pub const USERDATA: Access = Access {
        ord: 1i32
    };
    #[doc(alias = "ACCESS_FILESYSTEM")]
    #[doc = "Godot enumerator name: `ACCESS_FILESYSTEM`"]
    pub const FILESYSTEM: Access = Access {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Access") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Access {
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
            Self::RESOURCES => "RESOURCES", Self::USERDATA => "USERDATA", Self::FILESYSTEM => "FILESYSTEM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Access::RESOURCES, Access::USERDATA, Access::FILESYSTEM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Access >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RESOURCES", "ACCESS_RESOURCES", Access::RESOURCES), crate::meta::inspect::EnumConstant::new("USERDATA", "ACCESS_USERDATA", Access::USERDATA), crate::meta::inspect::EnumConstant::new("FILESYSTEM", "ACCESS_FILESYSTEM", Access::FILESYSTEM)]
        }
    }
}
impl crate::meta::GodotConvert for Access {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Access Resources", 0i64), EnumeratorShape::new_int("Access Userdata", 1i64), EnumeratorShape::new_int("Access Filesystem", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("FileDialog.Access")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Access {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Access {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Access {
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
impl crate::registry::property::Export for Access {
    
}
impl crate::meta::Element for Access {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DisplayMode {
    ord: i32
}
impl DisplayMode {
    #[doc(alias = "DISPLAY_THUMBNAILS")]
    #[doc = "Godot enumerator name: `DISPLAY_THUMBNAILS`"]
    pub const THUMBNAILS: DisplayMode = DisplayMode {
        ord: 0i32
    };
    #[doc(alias = "DISPLAY_LIST")]
    #[doc = "Godot enumerator name: `DISPLAY_LIST`"]
    pub const LIST: DisplayMode = DisplayMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DisplayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DisplayMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DisplayMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::THUMBNAILS => "THUMBNAILS", Self::LIST => "LIST", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DisplayMode::THUMBNAILS, DisplayMode::LIST]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DisplayMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("THUMBNAILS", "DISPLAY_THUMBNAILS", DisplayMode::THUMBNAILS), crate::meta::inspect::EnumConstant::new("LIST", "DISPLAY_LIST", DisplayMode::LIST)]
        }
    }
}
impl crate::meta::GodotConvert for DisplayMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Display Thumbnails", 0i64), EnumeratorShape::new_int("Display List", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("FileDialog.DisplayMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DisplayMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DisplayMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DisplayMode {
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
impl crate::registry::property::Export for DisplayMode {
    
}
impl crate::meta::Element for DisplayMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Customization {
    ord: i32
}
impl Customization {
    #[doc(alias = "CUSTOMIZATION_HIDDEN_FILES")]
    #[doc = "Godot enumerator name: `CUSTOMIZATION_HIDDEN_FILES`"]
    pub const HIDDEN_FILES: Customization = Customization {
        ord: 0i32
    };
    #[doc(alias = "CUSTOMIZATION_CREATE_FOLDER")]
    #[doc = "Godot enumerator name: `CUSTOMIZATION_CREATE_FOLDER`"]
    pub const CREATE_FOLDER: Customization = Customization {
        ord: 1i32
    };
    #[doc(alias = "CUSTOMIZATION_FILE_FILTER")]
    #[doc = "Godot enumerator name: `CUSTOMIZATION_FILE_FILTER`"]
    pub const FILE_FILTER: Customization = Customization {
        ord: 2i32
    };
    #[doc(alias = "CUSTOMIZATION_FILE_SORT")]
    #[doc = "Godot enumerator name: `CUSTOMIZATION_FILE_SORT`"]
    pub const FILE_SORT: Customization = Customization {
        ord: 3i32
    };
    #[doc(alias = "CUSTOMIZATION_FAVORITES")]
    #[doc = "Godot enumerator name: `CUSTOMIZATION_FAVORITES`"]
    pub const FAVORITES: Customization = Customization {
        ord: 4i32
    };
    #[doc(alias = "CUSTOMIZATION_RECENT")]
    #[doc = "Godot enumerator name: `CUSTOMIZATION_RECENT`"]
    pub const RECENT: Customization = Customization {
        ord: 5i32
    };
    #[doc(alias = "CUSTOMIZATION_LAYOUT")]
    #[doc = "Godot enumerator name: `CUSTOMIZATION_LAYOUT`"]
    pub const LAYOUT: Customization = Customization {
        ord: 6i32
    };
    #[doc(alias = "CUSTOMIZATION_OVERWRITE_WARNING")]
    #[doc = "Godot enumerator name: `CUSTOMIZATION_OVERWRITE_WARNING`"]
    pub const OVERWRITE_WARNING: Customization = Customization {
        ord: 7i32
    };
    #[doc(alias = "CUSTOMIZATION_DELETE")]
    #[doc = "Godot enumerator name: `CUSTOMIZATION_DELETE`"]
    pub const DELETE: Customization = Customization {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for Customization {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Customization") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Customization {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::HIDDEN_FILES => "HIDDEN_FILES", Self::CREATE_FOLDER => "CREATE_FOLDER", Self::FILE_FILTER => "FILE_FILTER", Self::FILE_SORT => "FILE_SORT", Self::FAVORITES => "FAVORITES", Self::RECENT => "RECENT", Self::LAYOUT => "LAYOUT", Self::OVERWRITE_WARNING => "OVERWRITE_WARNING", Self::DELETE => "DELETE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Customization::HIDDEN_FILES, Customization::CREATE_FOLDER, Customization::FILE_FILTER, Customization::FILE_SORT, Customization::FAVORITES, Customization::RECENT, Customization::LAYOUT, Customization::OVERWRITE_WARNING, Customization::DELETE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Customization >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HIDDEN_FILES", "CUSTOMIZATION_HIDDEN_FILES", Customization::HIDDEN_FILES), crate::meta::inspect::EnumConstant::new("CREATE_FOLDER", "CUSTOMIZATION_CREATE_FOLDER", Customization::CREATE_FOLDER), crate::meta::inspect::EnumConstant::new("FILE_FILTER", "CUSTOMIZATION_FILE_FILTER", Customization::FILE_FILTER), crate::meta::inspect::EnumConstant::new("FILE_SORT", "CUSTOMIZATION_FILE_SORT", Customization::FILE_SORT), crate::meta::inspect::EnumConstant::new("FAVORITES", "CUSTOMIZATION_FAVORITES", Customization::FAVORITES), crate::meta::inspect::EnumConstant::new("RECENT", "CUSTOMIZATION_RECENT", Customization::RECENT), crate::meta::inspect::EnumConstant::new("LAYOUT", "CUSTOMIZATION_LAYOUT", Customization::LAYOUT), crate::meta::inspect::EnumConstant::new("OVERWRITE_WARNING", "CUSTOMIZATION_OVERWRITE_WARNING", Customization::OVERWRITE_WARNING), crate::meta::inspect::EnumConstant::new("DELETE", "CUSTOMIZATION_DELETE", Customization::DELETE)]
        }
    }
}
impl crate::meta::GodotConvert for Customization {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Customization Hidden Files", 0i64), EnumeratorShape::new_int("Customization Create Folder", 1i64), EnumeratorShape::new_int("Customization File Filter", 2i64), EnumeratorShape::new_int("Customization File Sort", 3i64), EnumeratorShape::new_int("Customization Favorites", 4i64), EnumeratorShape::new_int("Customization Recent", 5i64), EnumeratorShape::new_int("Customization Layout", 6i64), EnumeratorShape::new_int("Customization Overwrite Warning", 7i64), EnumeratorShape::new_int("Customization Delete", 8i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("FileDialog.Customization")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Customization {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Customization {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Customization {
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
impl crate::registry::property::Export for Customization {
    
}
impl crate::meta::Element for Customization {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::FileDialog;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`FileDialog`][crate::classes::FileDialog] class."]
    pub struct SignalsOfFileDialog < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfFileDialog < 'c, C > {
        #[doc = "Signature: `(path: GString)`"]
        pub fn file_selected(&mut self) -> SigFileSelected < 'c, C > {
            SigFileSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "file_selected")
            }
        }
        #[doc = "Signature: `(paths: PackedStringArray)`"]
        pub fn files_selected(&mut self) -> SigFilesSelected < 'c, C > {
            SigFilesSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "files_selected")
            }
        }
        #[doc = "Signature: `(dir: GString)`"]
        pub fn dir_selected(&mut self) -> SigDirSelected < 'c, C > {
            SigDirSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "dir_selected")
            }
        }
        #[doc = "Signature: `(filter: GString)`"]
        pub fn filename_filter_changed(&mut self) -> SigFilenameFilterChanged < 'c, C > {
            SigFilenameFilterChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "filename_filter_changed")
            }
        }
    }
    type TypedSigFileSelected < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigFileSelected < 'c, C: WithSignals > {
        typed: TypedSigFileSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFileSelected < 'c, C > {
        pub fn emit(&mut self, path: GString,) {
            self.typed.emit_tuple((path,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFileSelected < 'c, C > {
        type Target = TypedSigFileSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFileSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFilesSelected < 'c, C > = TypedSignal < 'c, C, (PackedStringArray,) >;
    pub struct SigFilesSelected < 'c, C: WithSignals > {
        typed: TypedSigFilesSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFilesSelected < 'c, C > {
        pub fn emit(&mut self, paths: PackedStringArray,) {
            self.typed.emit_tuple((paths,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFilesSelected < 'c, C > {
        type Target = TypedSigFilesSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFilesSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDirSelected < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigDirSelected < 'c, C: WithSignals > {
        typed: TypedSigDirSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDirSelected < 'c, C > {
        pub fn emit(&mut self, dir: GString,) {
            self.typed.emit_tuple((dir,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDirSelected < 'c, C > {
        type Target = TypedSigDirSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDirSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFilenameFilterChanged < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigFilenameFilterChanged < 'c, C: WithSignals > {
        typed: TypedSigFilenameFilterChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFilenameFilterChanged < 'c, C > {
        pub fn emit(&mut self, filter: GString,) {
            self.typed.emit_tuple((filter,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFilenameFilterChanged < 'c, C > {
        type Target = TypedSigFilenameFilterChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFilenameFilterChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for FileDialog {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfFileDialog < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfFileDialog < 'c, C > {
        type Target = < < FileDialog as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = FileDialog;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfFileDialog < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = FileDialog;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}