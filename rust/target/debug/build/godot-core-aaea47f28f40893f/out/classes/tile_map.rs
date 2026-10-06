#![doc = "Sidecar module for class [`TileMap`][crate::classes::TileMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileMap` enums](https://docs.godotengine.org/en/stable/classes/class_tilemap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileMap`.\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`tile_map`][crate::classes::tile_map]: sidecar module with related enum/flag types\n* [`ITileMap`][crate::classes::ITileMap]: virtual methods\n* [`SignalsOfTileMap`][crate::classes::tile_map::SignalsOfTileMap]: signal collection\n\n\nSee also [Godot docs for `TileMap`](https://docs.godotengine.org/en/stable/classes/class_tilemap.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TileMap::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nNode for 2D tile-based maps. Tilemaps use a [`TileSet`][crate::classes::TileSet] which contain a list of tiles which are used to create grid-based maps. A TileMap may have several layers, layouting tiles on top of each other.\n\nFor performance reasons, all TileMap updates are batched at the end of a frame. Notably, this means that scene tiles from a [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource] may be initialized after their parent. This is only queued when inside the scene tree.\n\nTo force an update earlier on, call [`update_internals`][`crate::classes::TileMap::update_internals`].\n\n**Note:** For performance and compatibility reasons, the coordinates serialized by `TileMap` are limited to 16-bit signed integers, i.e. the range for X and Y coordinates is from `-32768` to `32767`. When saving tile data, tiles outside this range are wrapped."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TileMap`][crate::classes::TileMap].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TileMap` methods](https://docs.godotengine.org/en/stable/classes/class_tilemap.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileMap: crate::obj::GodotClass < Base = TileMap > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
        #[doc = "Should return `true` if the tile at coordinates `coords` on layer `layer` requires a runtime update.\n\n**Warning:** Make sure this function only return `true` when needed. Any tile processed at runtime without a need for it will imply a significant performance penalty.\n\n**Note:** If the result of this function should changed, use [`notify_runtime_tile_data_update`][`crate::classes::TileMap::notify_runtime_tile_data_update`] to notify the TileMap it needs an update."]
        fn use_tile_data_runtime_update(&mut self, layer: i32, coords: Vector2i,) -> bool {
            unimplemented !()
        }
        #[doc = "Called with a TileData object about to be used internally by the TileMap, allowing its modification at runtime.\n\nThis method is only called if [`use_tile_data_runtime_update`][`crate::classes::ITileMap::use_tile_data_runtime_update`] is implemented and returns `true` for the given tile `coords` and `layer`.\n\n**Warning:** The `tile_data` object's sub-resources are the same as the one in the TileSet. Modifying them might impact the whole TileSet. Instead, make sure to duplicate those resources.\n\n**Note:** If the properties of `tile_data` object should change over time, use [`notify_runtime_tile_data_update`][`crate::classes::TileMap::notify_runtime_tile_data_update`] to notify the TileMap it needs an update."]
        fn tile_data_runtime_update(&mut self, layer: i32, coords: Vector2i, tile_data: Option < Gd < crate::classes::TileData > >,) {
            unimplemented !()
        }
        #[doc = "Called when `CanvasItem` has been requested to redraw (after [`queue_redraw`][`crate::classes::CanvasItem::queue_redraw`] is called, either manually or by the engine).\n\nCorresponds to the [`CanvasItemNotification::DRAW`][`crate::classes::notify::CanvasItemNotification::DRAW`] notification in [`on_notification`][`crate::classes::IObject::on_notification`]."]
        fn draw(&mut self,) {
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
    impl TileMap {
        #[doc = "Assigns `map` as a `NavigationServer2D` navigation map for the specified TileMap layer `layer`."]
        pub fn set_navigation_map(&mut self, layer: i32, map: Rid,) {
            type CallRet = ();
            type CallParams = (i32, Rid,);
            let args = (layer, map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the `NavigationServer2D` navigation map assigned to the specified TileMap layer `layer`."]
        pub fn get_navigation_map(&self, layer: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Forces the TileMap and the layer `layer` to update."]
        pub(crate) fn force_update_full(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "force_update", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`force_update_ex`][Self::force_update_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Forces the TileMap and the layer `layer` to update."]
        #[inline]
        pub fn force_update(&mut self,) {
            self.force_update_ex() . done()
        }
        #[doc = "Forces the TileMap and the layer `layer` to update."]
        #[inline]
        pub fn force_update_ex < 'ex > (&'ex mut self,) -> ExForceUpdate < 'ex > {
            ExForceUpdate::new(self,)
        }
        pub fn set_tileset(&mut self, tileset: impl AsArg < Option < Gd < crate::classes::TileSet >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TileSet > > >,);
            let args = (tileset.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_tileset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tileset(&self,) -> Option < Gd < crate::classes::TileSet > > {
            type CallRet = Option < Gd < crate::classes::TileSet > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7718usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_tileset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rendering_quadrant_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_rendering_quadrant_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rendering_quadrant_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_rendering_quadrant_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of layers in the TileMap."]
        pub fn get_layers_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layers_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a layer at the given position `to_position` in the array. If `to_position` is negative, the position is counted from the end, with `-1` adding the layer at the end of the array."]
        pub fn add_layer(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "add_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves the layer at index `layer` to the given position `to_position` in the array."]
        pub fn move_layer(&mut self, layer: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7723usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "move_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the layer at index `layer`."]
        pub fn remove_layer(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "remove_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a layer's name. This is mostly useful in the editor.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn set_layer_name(&mut self, layer: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (layer, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a TileMap layer's name.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn get_layer_name(&self, layer: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables or disables the layer `layer`. A disabled layer is not processed at all (no rendering, no physics, etc.).\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn set_layer_enabled(&mut self, layer: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7727usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if a layer is enabled.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn is_layer_enabled(&self, layer: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7728usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_layer_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a layer's color. It will be multiplied by tile's color and TileMap's modulate.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn set_layer_modulate(&mut self, layer: i32, modulate: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (layer, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7729usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_modulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a TileMap layer's modulate.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn get_layer_modulate(&self, layer: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7730usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_modulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables or disables a layer's Y-sorting. If a layer is Y-sorted, the layer will behave as a CanvasItem node where each of its tile gets Y-sorted.\n\nY-sorted layers should usually be on different Z-index values than not Y-sorted layers, otherwise, each of those layer will be Y-sorted as whole with the Y-sorted one. This is usually an undesired behavior.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn set_layer_y_sort_enabled(&mut self, layer: i32, y_sort_enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer, y_sort_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_y_sort_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if a layer Y-sorts its tiles.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn is_layer_y_sort_enabled(&self, layer: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_layer_y_sort_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a layer's Y-sort origin value. This Y-sort origin value is added to each tile's Y-sort origin value.\n\nThis allows, for example, to fake a different height level on each layer. This can be useful for top-down view games.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn set_layer_y_sort_origin(&mut self, layer: i32, y_sort_origin: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer, y_sort_origin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_y_sort_origin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a TileMap layer's Y sort origin.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn get_layer_y_sort_origin(&self, layer: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_y_sort_origin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a layers Z-index value. This Z-index is added to each tile's Z-index value.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn set_layer_z_index(&mut self, layer: i32, z_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer, z_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_z_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a TileMap layer's Z-index value.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn get_layer_z_index(&self, layer: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7736usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_z_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables or disables a layer's built-in navigation regions generation. Disable this if you need to bake navigation regions from a TileMap using a `NavigationRegion2D` node."]
        pub fn set_layer_navigation_enabled(&mut self, layer: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7737usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_navigation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if a layer's built-in navigation regions generation is enabled."]
        pub fn is_layer_navigation_enabled(&self, layer: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7738usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_layer_navigation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns `map` as a `NavigationServer2D` navigation map for the specified TileMap layer `layer`.\n\nBy default the TileMap uses the default [`World2D`][crate::classes::World2D] navigation map for the first TileMap layer. For each additional TileMap layer a new navigation map is created for the additional layer.\n\nIn order to make `NavigationAgent2D` switch between TileMap layer navigation maps use \\[method NavigationAgent2D.set_navigation_map] with the navigation map received from [`get_layer_navigation_map`][`crate::classes::TileMap::get_layer_navigation_map`].\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn set_layer_navigation_map(&mut self, layer: i32, map: Rid,) {
            type CallRet = ();
            type CallParams = (i32, Rid,);
            let args = (layer, map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the `NavigationServer2D` navigation map assigned to the specified TileMap layer `layer`.\n\nBy default the TileMap uses the default [`World2D`][crate::classes::World2D] navigation map for the first TileMap layer. For each additional TileMap layer a new navigation map is created for the additional layer.\n\nIn order to make `NavigationAgent2D` switch between TileMap layer navigation maps use \\[method NavigationAgent2D.set_navigation_map] with the navigation map received from [`get_layer_navigation_map`][`crate::classes::TileMap::get_layer_navigation_map`].\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn get_layer_navigation_map(&self, layer: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_animatable(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_collision_animatable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_collision_animatable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_collision_animatable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_visibility_mode(&mut self, collision_visibility_mode: crate::classes::tile_map::VisibilityMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_map::VisibilityMode,);
            let args = (collision_visibility_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_collision_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_visibility_mode(&self,) -> crate::classes::tile_map::VisibilityMode {
            type CallRet = crate::classes::tile_map::VisibilityMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7744usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_collision_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_navigation_visibility_mode(&mut self, navigation_visibility_mode: crate::classes::tile_map::VisibilityMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_map::VisibilityMode,);
            let args = (navigation_visibility_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7745usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_navigation_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_navigation_visibility_mode(&self,) -> crate::classes::tile_map::VisibilityMode {
            type CallRet = crate::classes::tile_map::VisibilityMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7746usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_navigation_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the tile identifiers for the cell on layer `layer` at coordinates `coords`. Each tile of the [`TileSet`][crate::classes::TileSet] is identified using three parts:\n\n- The source identifier `source_id` identifies a [`TileSetSource`][crate::classes::TileSetSource] identifier. See [`set_source_id`][`crate::classes::TileSet::set_source_id`],\n\n- The atlas coordinates identifier `atlas_coords` identifies a tile coordinates in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]). For [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource] it should always be `Vector2i(0, 0)`),\n\n- The alternative tile identifier `alternative_tile` identifies a tile alternative in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]), and the scene for a [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource].\n\nIf `source_id` is set to `-1`, `atlas_coords` to `Vector2i(-1, -1)` or `alternative_tile` to `-1`, the cell will be erased. An erased cell gets **all** its identifiers automatically set to their respective invalid values, namely `-1`, `Vector2i(-1, -1)` and `-1`.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub(crate) fn set_cell_full(&mut self, layer: i32, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, Vector2i, i32,);
            let args = (layer, coords, source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7747usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_cell", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_cell_ex`][Self::set_cell_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the tile identifiers for the cell on layer `layer` at coordinates `coords`. Each tile of the [`TileSet`][crate::classes::TileSet] is identified using three parts:\n\n- The source identifier `source_id` identifies a [`TileSetSource`][crate::classes::TileSetSource] identifier. See [`set_source_id`][`crate::classes::TileSet::set_source_id`],\n\n- The atlas coordinates identifier `atlas_coords` identifies a tile coordinates in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]). For [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource] it should always be `Vector2i(0, 0)`),\n\n- The alternative tile identifier `alternative_tile` identifies a tile alternative in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]), and the scene for a [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource].\n\nIf `source_id` is set to `-1`, `atlas_coords` to `Vector2i(-1, -1)` or `alternative_tile` to `-1`, the cell will be erased. An erased cell gets **all** its identifiers automatically set to their respective invalid values, namely `-1`, `Vector2i(-1, -1)` and `-1`.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        #[inline]
        pub fn set_cell(&mut self, layer: i32, coords: Vector2i,) {
            self.set_cell_ex(layer, coords,) . done()
        }
        #[doc = "Sets the tile identifiers for the cell on layer `layer` at coordinates `coords`. Each tile of the [`TileSet`][crate::classes::TileSet] is identified using three parts:\n\n- The source identifier `source_id` identifies a [`TileSetSource`][crate::classes::TileSetSource] identifier. See [`set_source_id`][`crate::classes::TileSet::set_source_id`],\n\n- The atlas coordinates identifier `atlas_coords` identifies a tile coordinates in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]). For [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource] it should always be `Vector2i(0, 0)`),\n\n- The alternative tile identifier `alternative_tile` identifies a tile alternative in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]), and the scene for a [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource].\n\nIf `source_id` is set to `-1`, `atlas_coords` to `Vector2i(-1, -1)` or `alternative_tile` to `-1`, the cell will be erased. An erased cell gets **all** its identifiers automatically set to their respective invalid values, namely `-1`, `Vector2i(-1, -1)` and `-1`.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        #[inline]
        pub fn set_cell_ex < 'ex > (&'ex mut self, layer: i32, coords: Vector2i,) -> ExSetCell < 'ex > {
            ExSetCell::new(self, layer, coords,)
        }
        #[doc = "Erases the cell on layer `layer` at coordinates `coords`.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn erase_cell(&mut self, layer: i32, coords: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i,);
            let args = (layer, coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7748usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "erase_cell", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tile source ID of the cell on layer `layer` at coordinates `coords`. Returns `-1` if the cell does not exist.\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies, returning the raw source identifier. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`].\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub(crate) fn get_cell_source_id_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7749usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_cell_source_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_cell_source_id_ex`][Self::get_cell_source_id_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the tile source ID of the cell on layer `layer` at coordinates `coords`. Returns `-1` if the cell does not exist.\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies, returning the raw source identifier. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`].\n\nIf `layer` is negative, the layers are accessed from the last one."]
        #[inline]
        pub fn get_cell_source_id(&self, layer: i32, coords: Vector2i,) -> i32 {
            self.get_cell_source_id_ex(layer, coords,) . done()
        }
        #[doc = "Returns the tile source ID of the cell on layer `layer` at coordinates `coords`. Returns `-1` if the cell does not exist.\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies, returning the raw source identifier. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`].\n\nIf `layer` is negative, the layers are accessed from the last one."]
        #[inline]
        pub fn get_cell_source_id_ex < 'ex > (&'ex self, layer: i32, coords: Vector2i,) -> ExGetCellSourceId < 'ex > {
            ExGetCellSourceId::new(self, layer, coords,)
        }
        #[doc = "Returns the tile atlas coordinates ID of the cell on layer `layer` at coordinates `coords`. Returns `Vector2i(-1, -1)` if the cell does not exist.\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies, returning the raw atlas coordinate identifier. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`].\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub(crate) fn get_cell_atlas_coords_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7750usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_cell_atlas_coords", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_cell_atlas_coords_ex`][Self::get_cell_atlas_coords_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the tile atlas coordinates ID of the cell on layer `layer` at coordinates `coords`. Returns `Vector2i(-1, -1)` if the cell does not exist.\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies, returning the raw atlas coordinate identifier. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`].\n\nIf `layer` is negative, the layers are accessed from the last one."]
        #[inline]
        pub fn get_cell_atlas_coords(&self, layer: i32, coords: Vector2i,) -> Vector2i {
            self.get_cell_atlas_coords_ex(layer, coords,) . done()
        }
        #[doc = "Returns the tile atlas coordinates ID of the cell on layer `layer` at coordinates `coords`. Returns `Vector2i(-1, -1)` if the cell does not exist.\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies, returning the raw atlas coordinate identifier. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`].\n\nIf `layer` is negative, the layers are accessed from the last one."]
        #[inline]
        pub fn get_cell_atlas_coords_ex < 'ex > (&'ex self, layer: i32, coords: Vector2i,) -> ExGetCellAtlasCoords < 'ex > {
            ExGetCellAtlasCoords::new(self, layer, coords,)
        }
        #[doc = "Returns the tile alternative ID of the cell on layer `layer` at `coords`.\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies, returning the raw alternative identifier. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`].\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub(crate) fn get_cell_alternative_tile_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_cell_alternative_tile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_cell_alternative_tile_ex`][Self::get_cell_alternative_tile_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the tile alternative ID of the cell on layer `layer` at `coords`.\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies, returning the raw alternative identifier. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`].\n\nIf `layer` is negative, the layers are accessed from the last one."]
        #[inline]
        pub fn get_cell_alternative_tile(&self, layer: i32, coords: Vector2i,) -> i32 {
            self.get_cell_alternative_tile_ex(layer, coords,) . done()
        }
        #[doc = "Returns the tile alternative ID of the cell on layer `layer` at `coords`.\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies, returning the raw alternative identifier. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`].\n\nIf `layer` is negative, the layers are accessed from the last one."]
        #[inline]
        pub fn get_cell_alternative_tile_ex < 'ex > (&'ex self, layer: i32, coords: Vector2i,) -> ExGetCellAlternativeTile < 'ex > {
            ExGetCellAlternativeTile::new(self, layer, coords,)
        }
        #[doc = "Returns the [`TileData`][crate::classes::TileData] object associated with the given cell, or `null` if the cell does not exist or is not a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource].\n\nIf `layer` is negative, the layers are accessed from the last one.\n\n```gdscript\nfunc get_clicked_tile_power():\n\tvar clicked_cell = tile_map.local_to_map(tile_map.get_local_mouse_position())\n\tvar data = tile_map.get_cell_tile_data(0, clicked_cell)\n\tif data:\n\t\treturn data.get_custom_data(\"power\")\n\telse:\n\t\treturn 0\n```\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`]."]
        pub(crate) fn get_cell_tile_data_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> Option < Gd < crate::classes::TileData > > {
            type CallRet = Option < Gd < crate::classes::TileData > >;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_cell_tile_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_cell_tile_data_ex`][Self::get_cell_tile_data_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the [`TileData`][crate::classes::TileData] object associated with the given cell, or `null` if the cell does not exist or is not a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource].\n\nIf `layer` is negative, the layers are accessed from the last one.\n\n```gdscript\nfunc get_clicked_tile_power():\n\tvar clicked_cell = tile_map.local_to_map(tile_map.get_local_mouse_position())\n\tvar data = tile_map.get_cell_tile_data(0, clicked_cell)\n\tif data:\n\t\treturn data.get_custom_data(\"power\")\n\telse:\n\t\treturn 0\n```\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`]."]
        #[inline]
        pub fn get_cell_tile_data(&self, layer: i32, coords: Vector2i,) -> Option < Gd < crate::classes::TileData > > {
            self.get_cell_tile_data_ex(layer, coords,) . done()
        }
        #[doc = "Returns the [`TileData`][crate::classes::TileData] object associated with the given cell, or `null` if the cell does not exist or is not a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource].\n\nIf `layer` is negative, the layers are accessed from the last one.\n\n```gdscript\nfunc get_clicked_tile_power():\n\tvar clicked_cell = tile_map.local_to_map(tile_map.get_local_mouse_position())\n\tvar data = tile_map.get_cell_tile_data(0, clicked_cell)\n\tif data:\n\t\treturn data.get_custom_data(\"power\")\n\telse:\n\t\treturn 0\n```\n\nIf `use_proxies` is `false`, ignores the [`TileSet`][crate::classes::TileSet]'s tile proxies. See [`map_tile_proxy`][`crate::classes::TileSet::map_tile_proxy`]."]
        #[inline]
        pub fn get_cell_tile_data_ex < 'ex > (&'ex self, layer: i32, coords: Vector2i,) -> ExGetCellTileData < 'ex > {
            ExGetCellTileData::new(self, layer, coords,)
        }
        #[doc = "Returns `true` if the cell on layer `layer` at coordinates `coords` is flipped horizontally. The result is valid only for atlas sources."]
        pub(crate) fn is_cell_flipped_h_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_cell_flipped_h", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_cell_flipped_h_ex`][Self::is_cell_flipped_h_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the cell on layer `layer` at coordinates `coords` is flipped horizontally. The result is valid only for atlas sources."]
        #[inline]
        pub fn is_cell_flipped_h(&self, layer: i32, coords: Vector2i,) -> bool {
            self.is_cell_flipped_h_ex(layer, coords,) . done()
        }
        #[doc = "Returns `true` if the cell on layer `layer` at coordinates `coords` is flipped horizontally. The result is valid only for atlas sources."]
        #[inline]
        pub fn is_cell_flipped_h_ex < 'ex > (&'ex self, layer: i32, coords: Vector2i,) -> ExIsCellFlippedH < 'ex > {
            ExIsCellFlippedH::new(self, layer, coords,)
        }
        #[doc = "Returns `true` if the cell on layer `layer` at coordinates `coords` is flipped vertically. The result is valid only for atlas sources."]
        pub(crate) fn is_cell_flipped_v_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_cell_flipped_v", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_cell_flipped_v_ex`][Self::is_cell_flipped_v_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the cell on layer `layer` at coordinates `coords` is flipped vertically. The result is valid only for atlas sources."]
        #[inline]
        pub fn is_cell_flipped_v(&self, layer: i32, coords: Vector2i,) -> bool {
            self.is_cell_flipped_v_ex(layer, coords,) . done()
        }
        #[doc = "Returns `true` if the cell on layer `layer` at coordinates `coords` is flipped vertically. The result is valid only for atlas sources."]
        #[inline]
        pub fn is_cell_flipped_v_ex < 'ex > (&'ex self, layer: i32, coords: Vector2i,) -> ExIsCellFlippedV < 'ex > {
            ExIsCellFlippedV::new(self, layer, coords,)
        }
        #[doc = "Returns `true` if the cell on layer `layer` at coordinates `coords` is transposed. The result is valid only for atlas sources."]
        pub(crate) fn is_cell_transposed_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_cell_transposed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_cell_transposed_ex`][Self::is_cell_transposed_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the cell on layer `layer` at coordinates `coords` is transposed. The result is valid only for atlas sources."]
        #[inline]
        pub fn is_cell_transposed(&self, layer: i32, coords: Vector2i,) -> bool {
            self.is_cell_transposed_ex(layer, coords,) . done()
        }
        #[doc = "Returns `true` if the cell on layer `layer` at coordinates `coords` is transposed. The result is valid only for atlas sources."]
        #[inline]
        pub fn is_cell_transposed_ex < 'ex > (&'ex self, layer: i32, coords: Vector2i,) -> ExIsCellTransposed < 'ex > {
            ExIsCellTransposed::new(self, layer, coords,)
        }
        #[doc = "Returns the coordinates of the tile for given physics body RID. Such RID can be retrieved from [`get_collider_rid`][`crate::classes::KinematicCollision2D::get_collider_rid`], when colliding with a tile."]
        pub fn get_coords_for_body_rid(&self, body: Rid,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_coords_for_body_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tilemap layer of the tile for given physics body RID. Such RID can be retrieved from [`get_collider_rid`][`crate::classes::KinematicCollision2D::get_collider_rid`], when colliding with a tile."]
        pub fn get_layer_for_body_rid(&self, body: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_for_body_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new [`TileMapPattern`][crate::classes::TileMapPattern] from the given layer and set of cells.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn get_pattern(&self, layer: i32, coords_array: &Array < Vector2i >,) -> Option < Gd < crate::classes::TileMapPattern > > {
            type CallRet = Option < Gd < crate::classes::TileMapPattern > >;
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Array < Vector2i > >,);
            let args = (layer, RefArg::new(coords_array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_pattern", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns for the given coordinate `coords_in_pattern` in a [`TileMapPattern`][crate::classes::TileMapPattern] the corresponding cell coordinates if the pattern was pasted at the `position_in_tilemap` coordinates (see [`set_pattern`][`crate::classes::TileMap::set_pattern`]). This mapping is required as in half-offset tile shapes, the mapping might not work by calculating `position_in_tile_map + coords_in_pattern`."]
        pub fn map_pattern(&mut self, position_in_tilemap: Vector2i, coords_in_pattern: Vector2i, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> >,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams < 'a0, > = (Vector2i, Vector2i, CowArg < 'a0, Option < Gd < crate::classes::TileMapPattern > > >,);
            let args = (position_in_tilemap, coords_in_pattern, pattern.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "map_pattern", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Paste the given [`TileMapPattern`][crate::classes::TileMapPattern] at the given `position` and `layer` in the tile map.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn set_pattern(&mut self, layer: i32, position: Vector2i, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, Vector2i, CowArg < 'a0, Option < Gd < crate::classes::TileMapPattern > > >,);
            let args = (layer, position, pattern.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_pattern", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Update all the cells in the `cells` coordinates array so that they use the given `terrain` for the given `terrain_set`. If an updated cell has the same terrain as one of its neighboring cells, this function tries to join the two. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\nIf `layer` is negative, the layers are accessed from the last one.\n\n**Note:** To work correctly, this method requires the TileMap's TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        pub(crate) fn set_cells_terrain_connect_full(&mut self, layer: i32, cells: RefArg < Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Array < Vector2i > >, i32, i32, bool,);
            let args = (layer, cells, terrain_set, terrain, ignore_empty_terrains,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_cells_terrain_connect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_cells_terrain_connect_ex`][Self::set_cells_terrain_connect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Update all the cells in the `cells` coordinates array so that they use the given `terrain` for the given `terrain_set`. If an updated cell has the same terrain as one of its neighboring cells, this function tries to join the two. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\nIf `layer` is negative, the layers are accessed from the last one.\n\n**Note:** To work correctly, this method requires the TileMap's TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        #[inline]
        pub fn set_cells_terrain_connect(&mut self, layer: i32, cells: &Array < Vector2i >, terrain_set: i32, terrain: i32,) {
            self.set_cells_terrain_connect_ex(layer, cells, terrain_set, terrain,) . done()
        }
        #[doc = "Update all the cells in the `cells` coordinates array so that they use the given `terrain` for the given `terrain_set`. If an updated cell has the same terrain as one of its neighboring cells, this function tries to join the two. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\nIf `layer` is negative, the layers are accessed from the last one.\n\n**Note:** To work correctly, this method requires the TileMap's TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        #[inline]
        pub fn set_cells_terrain_connect_ex < 'ex > (&'ex mut self, layer: i32, cells: &'ex Array < Vector2i >, terrain_set: i32, terrain: i32,) -> ExSetCellsTerrainConnect < 'ex > {
            ExSetCellsTerrainConnect::new(self, layer, cells, terrain_set, terrain,)
        }
        #[doc = "Update all the cells in the `path` coordinates array so that they use the given `terrain` for the given `terrain_set`. The function will also connect two successive cell in the path with the same terrain. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\nIf `layer` is negative, the layers are accessed from the last one.\n\n**Note:** To work correctly, this method requires the TileMap's TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        pub(crate) fn set_cells_terrain_path_full(&mut self, layer: i32, path: RefArg < Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Array < Vector2i > >, i32, i32, bool,);
            let args = (layer, path, terrain_set, terrain, ignore_empty_terrains,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_cells_terrain_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_cells_terrain_path_ex`][Self::set_cells_terrain_path_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Update all the cells in the `path` coordinates array so that they use the given `terrain` for the given `terrain_set`. The function will also connect two successive cell in the path with the same terrain. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\nIf `layer` is negative, the layers are accessed from the last one.\n\n**Note:** To work correctly, this method requires the TileMap's TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        #[inline]
        pub fn set_cells_terrain_path(&mut self, layer: i32, path: &Array < Vector2i >, terrain_set: i32, terrain: i32,) {
            self.set_cells_terrain_path_ex(layer, path, terrain_set, terrain,) . done()
        }
        #[doc = "Update all the cells in the `path` coordinates array so that they use the given `terrain` for the given `terrain_set`. The function will also connect two successive cell in the path with the same terrain. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\nIf `layer` is negative, the layers are accessed from the last one.\n\n**Note:** To work correctly, this method requires the TileMap's TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        #[inline]
        pub fn set_cells_terrain_path_ex < 'ex > (&'ex mut self, layer: i32, path: &'ex Array < Vector2i >, terrain_set: i32, terrain: i32,) -> ExSetCellsTerrainPath < 'ex > {
            ExSetCellsTerrainPath::new(self, layer, path, terrain_set, terrain,)
        }
        #[doc = "Clears cells that do not exist in the tileset."]
        pub fn fix_invalid_tiles(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "fix_invalid_tiles", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all cells on the given layer.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn clear_layer(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "clear_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all cells."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Triggers a direct update of the TileMap. Usually, calling this function is not needed, as TileMap node updates automatically when one of its properties or cells is modified.\n\nHowever, for performance reasons, those updates are batched and delayed to the end of the frame. Calling this function will force the TileMap to update right away instead.\n\n**Warning:** Updating the TileMap is computationally expensive and may impact performance. Try to limit the number of updates and how many tiles they impact."]
        pub fn update_internals(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "update_internals", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Notifies the TileMap node that calls to [`use_tile_data_runtime_update`][`crate::classes::ITileMap::use_tile_data_runtime_update`] or [`tile_data_runtime_update`][`crate::classes::ITileMap::tile_data_runtime_update`] will lead to different results. This will thus trigger a TileMap update.\n\nIf `layer` is provided, only notifies changes for the given layer. Providing the `layer` argument (when applicable) is usually preferred for performance reasons.\n\n**Warning:** Updating the TileMap is computationally expensive and may impact performance. Try to limit the number of calls to this function to avoid unnecessary update.\n\n**Note:** This does not trigger a direct update of the TileMap, the update will be done at the end of the frame as usual (unless you call [`update_internals`][`crate::classes::TileMap::update_internals`])."]
        pub(crate) fn notify_runtime_tile_data_update_full(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "notify_runtime_tile_data_update", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`notify_runtime_tile_data_update_ex`][Self::notify_runtime_tile_data_update_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Notifies the TileMap node that calls to [`use_tile_data_runtime_update`][`crate::classes::ITileMap::use_tile_data_runtime_update`] or [`tile_data_runtime_update`][`crate::classes::ITileMap::tile_data_runtime_update`] will lead to different results. This will thus trigger a TileMap update.\n\nIf `layer` is provided, only notifies changes for the given layer. Providing the `layer` argument (when applicable) is usually preferred for performance reasons.\n\n**Warning:** Updating the TileMap is computationally expensive and may impact performance. Try to limit the number of calls to this function to avoid unnecessary update.\n\n**Note:** This does not trigger a direct update of the TileMap, the update will be done at the end of the frame as usual (unless you call [`update_internals`][`crate::classes::TileMap::update_internals`])."]
        #[inline]
        pub fn notify_runtime_tile_data_update(&mut self,) {
            self.notify_runtime_tile_data_update_ex() . done()
        }
        #[doc = "Notifies the TileMap node that calls to [`use_tile_data_runtime_update`][`crate::classes::ITileMap::use_tile_data_runtime_update`] or [`tile_data_runtime_update`][`crate::classes::ITileMap::tile_data_runtime_update`] will lead to different results. This will thus trigger a TileMap update.\n\nIf `layer` is provided, only notifies changes for the given layer. Providing the `layer` argument (when applicable) is usually preferred for performance reasons.\n\n**Warning:** Updating the TileMap is computationally expensive and may impact performance. Try to limit the number of calls to this function to avoid unnecessary update.\n\n**Note:** This does not trigger a direct update of the TileMap, the update will be done at the end of the frame as usual (unless you call [`update_internals`][`crate::classes::TileMap::update_internals`])."]
        #[inline]
        pub fn notify_runtime_tile_data_update_ex < 'ex > (&'ex mut self,) -> ExNotifyRuntimeTileDataUpdate < 'ex > {
            ExNotifyRuntimeTileDataUpdate::new(self,)
        }
        #[doc = "Returns the list of all neighbourings cells to the one at `coords`."]
        pub fn get_surrounding_cells(&self, coords: Vector2i,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_surrounding_cells", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Vector2i`][crate::builtin::Vector2i] array with the positions of all cells containing a tile in the given layer. A cell is considered empty if its source identifier equals -1, its atlas coordinates identifiers is `Vector2(-1, -1)` and its alternative identifier is -1.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub fn get_used_cells(&self, layer: i32,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_used_cells", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Vector2i`][crate::builtin::Vector2i] array with the positions of all cells containing a tile in the given layer. Tiles may be filtered according to their source (`source_id`), their atlas coordinates (`atlas_coords`) or alternative id (`alternative_tile`).\n\nIf a parameter has its value set to the default one, this parameter is not used to filter a cell. Thus, if all parameters have their respective default value, this method returns the same result as [`get_used_cells`][`crate::classes::TileMap::get_used_cells`].\n\nA cell is considered empty if its source identifier equals -1, its atlas coordinates identifiers is `Vector2(-1, -1)` and its alternative identifier is -1.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        pub(crate) fn get_used_cells_by_id_full(&self, layer: i32, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (i32, i32, Vector2i, i32,);
            let args = (layer, source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_used_cells_by_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_used_cells_by_id_ex`][Self::get_used_cells_by_id_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a [`Vector2i`][crate::builtin::Vector2i] array with the positions of all cells containing a tile in the given layer. Tiles may be filtered according to their source (`source_id`), their atlas coordinates (`atlas_coords`) or alternative id (`alternative_tile`).\n\nIf a parameter has its value set to the default one, this parameter is not used to filter a cell. Thus, if all parameters have their respective default value, this method returns the same result as [`get_used_cells`][`crate::classes::TileMap::get_used_cells`].\n\nA cell is considered empty if its source identifier equals -1, its atlas coordinates identifiers is `Vector2(-1, -1)` and its alternative identifier is -1.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        #[inline]
        pub fn get_used_cells_by_id(&self, layer: i32,) -> Array < Vector2i > {
            self.get_used_cells_by_id_ex(layer,) . done()
        }
        #[doc = "Returns a [`Vector2i`][crate::builtin::Vector2i] array with the positions of all cells containing a tile in the given layer. Tiles may be filtered according to their source (`source_id`), their atlas coordinates (`atlas_coords`) or alternative id (`alternative_tile`).\n\nIf a parameter has its value set to the default one, this parameter is not used to filter a cell. Thus, if all parameters have their respective default value, this method returns the same result as [`get_used_cells`][`crate::classes::TileMap::get_used_cells`].\n\nA cell is considered empty if its source identifier equals -1, its atlas coordinates identifiers is `Vector2(-1, -1)` and its alternative identifier is -1.\n\nIf `layer` is negative, the layers are accessed from the last one."]
        #[inline]
        pub fn get_used_cells_by_id_ex < 'ex > (&'ex self, layer: i32,) -> ExGetUsedCellsById < 'ex > {
            ExGetUsedCellsById::new(self, layer,)
        }
        #[doc = "Returns a rectangle enclosing the used (non-empty) tiles of the map, including all layers."]
        pub fn get_used_rect(&self,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_used_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the centered position of a cell in the TileMap's local coordinate space. To convert the returned value into global coordinates, use [`to_global`][`crate::classes::Node2D::to_global`]. See also [`local_to_map`][`crate::classes::TileMap::local_to_map`].\n\n**Note:** This may not correspond to the visual position of the tile, i.e. it ignores the \\[member TileData.texture_origin] property of individual tiles."]
        pub fn map_to_local(&self, map_position: Vector2i,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2i,);
            let args = (map_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "map_to_local", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the map coordinates of the cell containing the given `local_position`. If `local_position` is in global coordinates, consider using [`to_local`][`crate::classes::Node2D::to_local`] before passing it to this method. See also [`map_to_local`][`crate::classes::TileMap::map_to_local`]."]
        pub fn local_to_map(&self, local_position: Vector2,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2,);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "local_to_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the neighboring cell to the one at coordinates `coords`, identified by the `neighbor` direction. This method takes into account the different layouts a TileMap can take."]
        pub fn get_neighbor_cell(&self, coords: Vector2i, neighbor: crate::classes::tile_set::CellNeighbor,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2i, crate::classes::tile_set::CellNeighbor,);
            let args = (coords, neighbor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_neighbor_cell", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TileMap {
        type Base = crate::classes::Node2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TileMap"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileMap {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for TileMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TileMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TileMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileMap {
        
    }
    impl crate::obj::cap::GodotDefault for TileMap {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileMap {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileMap`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TileMap__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileMap > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TileMap::force_update_ex`][super::TileMap::force_update_ex]."]
#[must_use]
pub struct ExForceUpdate < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileMap, layer: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExForceUpdate < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileMap,) -> Self {
        let layer = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer,
        }
    }
    #[inline]
    pub fn layer(self, layer: i32) -> Self {
        Self {
            layer: layer, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, layer,
        }
        = self;
        re_export::TileMap::force_update_full(surround_object, layer,)
    }
}
#[doc = "Default-param extender for [`TileMap::set_cell_ex`][super::TileMap::set_cell_ex]."]
#[must_use]
pub struct ExSetCell < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileMap, layer: i32, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCell < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let source_id = - 1i32;
        let atlas_coords = Vector2i::new(- 1 as _, - 1 as _);
        let alternative_tile = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, source_id: source_id, atlas_coords: atlas_coords, alternative_tile: alternative_tile,
        }
    }
    #[inline]
    pub fn source_id(self, source_id: i32) -> Self {
        Self {
            source_id: source_id, .. self
        }
    }
    #[inline]
    pub fn atlas_coords(self, atlas_coords: Vector2i) -> Self {
        Self {
            atlas_coords: atlas_coords, .. self
        }
    }
    #[inline]
    pub fn alternative_tile(self, alternative_tile: i32) -> Self {
        Self {
            alternative_tile: alternative_tile, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, layer, coords, source_id, atlas_coords, alternative_tile,
        }
        = self;
        re_export::TileMap::set_cell_full(surround_object, layer, coords, source_id, atlas_coords, alternative_tile,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_source_id_ex`][super::TileMap::get_cell_source_id_ex]."]
#[must_use]
pub struct ExGetCellSourceId < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCellSourceId < 'ex > {
    fn new(surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::get_cell_source_id_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_atlas_coords_ex`][super::TileMap::get_cell_atlas_coords_ex]."]
#[must_use]
pub struct ExGetCellAtlasCoords < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCellAtlasCoords < 'ex > {
    fn new(surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::get_cell_atlas_coords_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_alternative_tile_ex`][super::TileMap::get_cell_alternative_tile_ex]."]
#[must_use]
pub struct ExGetCellAlternativeTile < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCellAlternativeTile < 'ex > {
    fn new(surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::get_cell_alternative_tile_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_tile_data_ex`][super::TileMap::get_cell_tile_data_ex]."]
#[must_use]
pub struct ExGetCellTileData < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCellTileData < 'ex > {
    fn new(surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TileData > > {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::get_cell_tile_data_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::is_cell_flipped_h_ex`][super::TileMap::is_cell_flipped_h_ex]."]
#[must_use]
pub struct ExIsCellFlippedH < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsCellFlippedH < 'ex > {
    fn new(surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::is_cell_flipped_h_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::is_cell_flipped_v_ex`][super::TileMap::is_cell_flipped_v_ex]."]
#[must_use]
pub struct ExIsCellFlippedV < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsCellFlippedV < 'ex > {
    fn new(surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::is_cell_flipped_v_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::is_cell_transposed_ex`][super::TileMap::is_cell_transposed_ex]."]
#[must_use]
pub struct ExIsCellTransposed < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsCellTransposed < 'ex > {
    fn new(surround_object: &'ex re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::is_cell_transposed_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::set_cells_terrain_connect_ex`][super::TileMap::set_cells_terrain_connect_ex]."]
#[must_use]
pub struct ExSetCellsTerrainConnect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileMap, layer: i32, cells: CowArg < 'ex, Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCellsTerrainConnect < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileMap, layer: i32, cells: &'ex Array < Vector2i >, terrain_set: i32, terrain: i32,) -> Self {
        let ignore_empty_terrains = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, cells: CowArg::Borrowed(cells), terrain_set: terrain_set, terrain: terrain, ignore_empty_terrains: ignore_empty_terrains,
        }
    }
    #[inline]
    pub fn ignore_empty_terrains(self, ignore_empty_terrains: bool) -> Self {
        Self {
            ignore_empty_terrains: ignore_empty_terrains, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, layer, cells, terrain_set, terrain, ignore_empty_terrains,
        }
        = self;
        re_export::TileMap::set_cells_terrain_connect_full(surround_object, layer, cells.cow_as_arg(), terrain_set, terrain, ignore_empty_terrains,)
    }
}
#[doc = "Default-param extender for [`TileMap::set_cells_terrain_path_ex`][super::TileMap::set_cells_terrain_path_ex]."]
#[must_use]
pub struct ExSetCellsTerrainPath < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileMap, layer: i32, path: CowArg < 'ex, Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCellsTerrainPath < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileMap, layer: i32, path: &'ex Array < Vector2i >, terrain_set: i32, terrain: i32,) -> Self {
        let ignore_empty_terrains = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, path: CowArg::Borrowed(path), terrain_set: terrain_set, terrain: terrain, ignore_empty_terrains: ignore_empty_terrains,
        }
    }
    #[inline]
    pub fn ignore_empty_terrains(self, ignore_empty_terrains: bool) -> Self {
        Self {
            ignore_empty_terrains: ignore_empty_terrains, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, layer, path, terrain_set, terrain, ignore_empty_terrains,
        }
        = self;
        re_export::TileMap::set_cells_terrain_path_full(surround_object, layer, path.cow_as_arg(), terrain_set, terrain, ignore_empty_terrains,)
    }
}
#[doc = "Default-param extender for [`TileMap::notify_runtime_tile_data_update_ex`][super::TileMap::notify_runtime_tile_data_update_ex]."]
#[must_use]
pub struct ExNotifyRuntimeTileDataUpdate < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileMap, layer: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExNotifyRuntimeTileDataUpdate < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileMap,) -> Self {
        let layer = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer,
        }
    }
    #[inline]
    pub fn layer(self, layer: i32) -> Self {
        Self {
            layer: layer, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, layer,
        }
        = self;
        re_export::TileMap::notify_runtime_tile_data_update_full(surround_object, layer,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_used_cells_by_id_ex`][super::TileMap::get_used_cells_by_id_ex]."]
#[must_use]
pub struct ExGetUsedCellsById < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileMap, layer: i32, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetUsedCellsById < 'ex > {
    fn new(surround_object: &'ex re_export::TileMap, layer: i32,) -> Self {
        let source_id = - 1i32;
        let atlas_coords = Vector2i::new(- 1 as _, - 1 as _);
        let alternative_tile = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, source_id: source_id, atlas_coords: atlas_coords, alternative_tile: alternative_tile,
        }
    }
    #[inline]
    pub fn source_id(self, source_id: i32) -> Self {
        Self {
            source_id: source_id, .. self
        }
    }
    #[inline]
    pub fn atlas_coords(self, atlas_coords: Vector2i) -> Self {
        Self {
            atlas_coords: atlas_coords, .. self
        }
    }
    #[inline]
    pub fn alternative_tile(self, alternative_tile: i32) -> Self {
        Self {
            alternative_tile: alternative_tile, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Vector2i > {
        let Self {
            _phantom, surround_object, layer, source_id, atlas_coords, alternative_tile,
        }
        = self;
        re_export::TileMap::get_used_cells_by_id_full(surround_object, layer, source_id, atlas_coords, alternative_tile,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibilityMode {
    ord: i32
}
impl VisibilityMode {
    #[doc(alias = "VISIBILITY_MODE_DEFAULT")]
    #[doc = "Godot enumerator name: `VISIBILITY_MODE_DEFAULT`"]
    pub const DEFAULT: VisibilityMode = VisibilityMode {
        ord: 0i32
    };
    #[doc(alias = "VISIBILITY_MODE_FORCE_HIDE")]
    #[doc = "Godot enumerator name: `VISIBILITY_MODE_FORCE_HIDE`"]
    pub const FORCE_HIDE: VisibilityMode = VisibilityMode {
        ord: 2i32
    };
    #[doc(alias = "VISIBILITY_MODE_FORCE_SHOW")]
    #[doc = "Godot enumerator name: `VISIBILITY_MODE_FORCE_SHOW`"]
    pub const FORCE_SHOW: VisibilityMode = VisibilityMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for VisibilityMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibilityMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibilityMode {
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
            Self::DEFAULT => "DEFAULT", Self::FORCE_HIDE => "FORCE_HIDE", Self::FORCE_SHOW => "FORCE_SHOW", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VisibilityMode::DEFAULT, VisibilityMode::FORCE_HIDE, VisibilityMode::FORCE_SHOW]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VisibilityMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "VISIBILITY_MODE_DEFAULT", VisibilityMode::DEFAULT), crate::meta::inspect::EnumConstant::new("FORCE_HIDE", "VISIBILITY_MODE_FORCE_HIDE", VisibilityMode::FORCE_HIDE), crate::meta::inspect::EnumConstant::new("FORCE_SHOW", "VISIBILITY_MODE_FORCE_SHOW", VisibilityMode::FORCE_SHOW)]
        }
    }
}
impl crate::meta::GodotConvert for VisibilityMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Visibility Mode Default", 0i64), EnumeratorShape::new_int("Visibility Mode Force Hide", 2i64), EnumeratorShape::new_int("Visibility Mode Force Show", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TileMap.VisibilityMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for VisibilityMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibilityMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for VisibilityMode {
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
impl crate::registry::property::Export for VisibilityMode {
    
}
impl crate::meta::Element for VisibilityMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TileMap;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`TileMap`][crate::classes::TileMap] class."]
    pub struct SignalsOfTileMap < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTileMap < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn changed(&mut self) -> SigChanged < 'c, C > {
            SigChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "changed")
            }
        }
    }
    type TypedSigChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigChanged < 'c, C: WithSignals > {
        typed: TypedSigChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigChanged < 'c, C > {
        type Target = TypedSigChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for TileMap {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTileMap < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTileMap < 'c, C > {
        type Target = < < TileMap as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = TileMap;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTileMap < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = TileMap;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}