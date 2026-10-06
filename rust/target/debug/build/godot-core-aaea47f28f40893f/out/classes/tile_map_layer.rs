#![doc = "Sidecar module for class [`TileMapLayer`][crate::classes::TileMapLayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileMapLayer` enums](https://docs.godotengine.org/en/stable/classes/class_tilemaplayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileMapLayer`.\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`tile_map_layer`][crate::classes::tile_map_layer]: sidecar module with related enum/flag types\n* [`ITileMapLayer`][crate::classes::ITileMapLayer]: virtual methods\n* [`SignalsOfTileMapLayer`][crate::classes::tile_map_layer::SignalsOfTileMapLayer]: signal collection\n\n\nSee also [Godot docs for `TileMapLayer`](https://docs.godotengine.org/en/stable/classes/class_tilemaplayer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TileMapLayer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nNode for 2D tile-based maps. A `TileMapLayer` uses a [`TileSet`][crate::classes::TileSet] which contain a list of tiles which are used to create grid-based maps. Unlike the [`TileMap`][crate::classes::TileMap] node, which is deprecated, `TileMapLayer` has only one layer of tiles. You can use several `TileMapLayer` to achieve the same result as a [`TileMap`][crate::classes::TileMap] node.\n\nFor performance reasons, all TileMap updates are batched at the end of a frame. Notably, this means that scene tiles from a [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource] are initialized after their parent. This is only queued when inside the scene tree.\n\nTo force an update earlier on, call [`update_internals`][`crate::classes::TileMapLayer::update_internals`].\n\n**Note:** For performance and compatibility reasons, the coordinates serialized by `TileMapLayer` are limited to 16-bit signed integers, i.e. the range for X and Y coordinates is from `-32768` to `32767`. When saving tile data, tiles outside this range are wrapped."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileMapLayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TileMapLayer`][crate::classes::TileMapLayer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TileMapLayer` methods](https://docs.godotengine.org/en/stable/classes/class_tilemaplayer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileMapLayer: crate::obj::GodotClass < Base = TileMapLayer > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Should return `true` if the tile at coordinates `coords` requires a runtime update.\n\n**Warning:** Make sure this function only returns `true` when needed. Any tile processed at runtime without a need for it will imply a significant performance penalty.\n\n**Note:** If the result of this function should change, use [`notify_runtime_tile_data_update`][`crate::classes::TileMapLayer::notify_runtime_tile_data_update`] to notify the `TileMapLayer` it needs an update."]
        fn use_tile_data_runtime_update(&mut self, coords: Vector2i,) -> bool {
            unimplemented !()
        }
        #[doc = "Called with a [`TileData`][crate::classes::TileData] object about to be used internally by the `TileMapLayer`, allowing its modification at runtime.\n\nThis method is only called if [`use_tile_data_runtime_update`][`crate::classes::ITileMapLayer::use_tile_data_runtime_update`] is implemented and returns `true` for the given tile `coords`.\n\n**Warning:** The `tile_data` object's sub-resources are the same as the one in the TileSet. Modifying them might impact the whole TileSet. Instead, make sure to duplicate those resources.\n\n**Note:** If the properties of `tile_data` object should change over time, use [`notify_runtime_tile_data_update`][`crate::classes::TileMapLayer::notify_runtime_tile_data_update`] to notify the `TileMapLayer` it needs an update."]
        fn tile_data_runtime_update(&mut self, coords: Vector2i, tile_data: Option < Gd < crate::classes::TileData > >,) {
            unimplemented !()
        }
        #[doc = "Called when this `TileMapLayer`'s cells need an internal update. This update may be caused from individual cells being modified or by a change in the \\[member tile_set] (causing all cells to be queued for an update). The first call to this function is always for initializing all the `TileMapLayer`'s cells. `coords` contains the coordinates of all modified cells, roughly in the order they were modified. `forced_cleanup` is `true` when the `TileMapLayer`'s internals should be fully cleaned up. This is the case when:\n\n- The layer is disabled;\n\n- The layer is not visible;\n\n- \\[member tile_set] is set to `null`;\n\n- The node is removed from the tree;\n\n- The node is freed.\n\nNote that any internal update happening while one of these conditions is verified is considered to be a \"cleanup\". See also [`update_internals`][`crate::classes::TileMapLayer::update_internals`].\n\n**Warning:** Implementing this method may degrade the `TileMapLayer`'s performance."]
        fn update_cells(&mut self, coords: Array < Vector2i >, forced_cleanup: bool,) {
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
    impl TileMapLayer {
        #[doc = "Sets the tile identifiers for the cell at coordinates `coords`. Each tile of the [`TileSet`][crate::classes::TileSet] is identified using three parts:\n\n- The source identifier `source_id` identifies a [`TileSetSource`][crate::classes::TileSetSource] identifier. See [`set_source_id`][`crate::classes::TileSet::set_source_id`],\n\n- The atlas coordinate identifier `atlas_coords` identifies a tile coordinates in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]). For [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource] it should always be `Vector2i(0, 0)`,\n\n- The alternative tile identifier `alternative_tile` identifies a tile alternative in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]), and the scene for a [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource].\n\nIf `source_id` is set to `-1`, `atlas_coords` to `Vector2i(-1, -1)`, or `alternative_tile` to `-1`, the cell will be erased. An erased cell gets **all** its identifiers automatically set to their respective invalid values, namely `-1`, `Vector2i(-1, -1)` and `-1`."]
        pub(crate) fn set_cell_full(&mut self, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32, Vector2i, i32,);
            let args = (coords, source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_cell", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_cell_ex`][Self::set_cell_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the tile identifiers for the cell at coordinates `coords`. Each tile of the [`TileSet`][crate::classes::TileSet] is identified using three parts:\n\n- The source identifier `source_id` identifies a [`TileSetSource`][crate::classes::TileSetSource] identifier. See [`set_source_id`][`crate::classes::TileSet::set_source_id`],\n\n- The atlas coordinate identifier `atlas_coords` identifies a tile coordinates in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]). For [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource] it should always be `Vector2i(0, 0)`,\n\n- The alternative tile identifier `alternative_tile` identifies a tile alternative in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]), and the scene for a [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource].\n\nIf `source_id` is set to `-1`, `atlas_coords` to `Vector2i(-1, -1)`, or `alternative_tile` to `-1`, the cell will be erased. An erased cell gets **all** its identifiers automatically set to their respective invalid values, namely `-1`, `Vector2i(-1, -1)` and `-1`."]
        #[inline]
        pub fn set_cell(&mut self, coords: Vector2i,) {
            self.set_cell_ex(coords,) . done()
        }
        #[doc = "Sets the tile identifiers for the cell at coordinates `coords`. Each tile of the [`TileSet`][crate::classes::TileSet] is identified using three parts:\n\n- The source identifier `source_id` identifies a [`TileSetSource`][crate::classes::TileSetSource] identifier. See [`set_source_id`][`crate::classes::TileSet::set_source_id`],\n\n- The atlas coordinate identifier `atlas_coords` identifies a tile coordinates in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]). For [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource] it should always be `Vector2i(0, 0)`,\n\n- The alternative tile identifier `alternative_tile` identifies a tile alternative in the atlas (if the source is a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource]), and the scene for a [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource].\n\nIf `source_id` is set to `-1`, `atlas_coords` to `Vector2i(-1, -1)`, or `alternative_tile` to `-1`, the cell will be erased. An erased cell gets **all** its identifiers automatically set to their respective invalid values, namely `-1`, `Vector2i(-1, -1)` and `-1`."]
        #[inline]
        pub fn set_cell_ex < 'ex > (&'ex mut self, coords: Vector2i,) -> ExSetCell < 'ex > {
            ExSetCell::new(self, coords,)
        }
        #[doc = "Erases the cell at coordinates `coords`."]
        pub fn erase_cell(&mut self, coords: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "erase_cell", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears cells containing tiles that do not exist in the \\[member tile_set]."]
        pub fn fix_invalid_tiles(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "fix_invalid_tiles", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all cells."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tile source ID of the cell at coordinates `coords`. Returns `-1` if the cell does not exist."]
        pub fn get_cell_source_id(&self, coords: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_cell_source_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tile atlas coordinates ID of the cell at coordinates `coords`. Returns `Vector2i(-1, -1)` if the cell does not exist."]
        pub fn get_cell_atlas_coords(&self, coords: Vector2i,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_cell_atlas_coords", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tile alternative ID of the cell at coordinates `coords`."]
        pub fn get_cell_alternative_tile(&self, coords: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_cell_alternative_tile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`TileData`][crate::classes::TileData] object associated with the given cell, or `null` if the cell does not exist or is not a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource].\n\n```gdscript\nfunc get_clicked_tile_power():\n\tvar clicked_cell = tile_map_layer.local_to_map(tile_map_layer.get_local_mouse_position())\n\tvar data = tile_map_layer.get_cell_tile_data(clicked_cell)\n\tif data:\n\t\treturn data.get_custom_data(\"power\")\n\telse:\n\t\treturn 0\n```"]
        pub fn get_cell_tile_data(&self, coords: Vector2i,) -> Option < Gd < crate::classes::TileData > > {
            type CallRet = Option < Gd < crate::classes::TileData > >;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_cell_tile_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the cell at coordinates `coords` is flipped horizontally. The result is valid only for atlas sources."]
        pub fn is_cell_flipped_h(&self, coords: Vector2i,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_cell_flipped_h", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the cell at coordinates `coords` is flipped vertically. The result is valid only for atlas sources."]
        pub fn is_cell_flipped_v(&self, coords: Vector2i,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_cell_flipped_v", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the cell at coordinates `coords` is transposed. The result is valid only for atlas sources."]
        pub fn is_cell_transposed(&self, coords: Vector2i,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_cell_transposed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Vector2i`][crate::builtin::Vector2i] array with the positions of all cells containing a tile. A cell is considered empty if its source identifier equals `-1`, its atlas coordinate identifier is `Vector2(-1, -1)` and its alternative identifier is `-1`."]
        pub fn get_used_cells(&self,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_used_cells", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Vector2i`][crate::builtin::Vector2i] array with the positions of all cells containing a tile. Tiles may be filtered according to their source (`source_id`), their atlas coordinates (`atlas_coords`), or alternative id (`alternative_tile`).\n\nIf a parameter has its value set to the default one, this parameter is not used to filter a cell. Thus, if all parameters have their respective default values, this method returns the same result as [`get_used_cells`][`crate::classes::TileMapLayer::get_used_cells`].\n\nA cell is considered empty if its source identifier equals `-1`, its atlas coordinate identifier is `Vector2(-1, -1)` and its alternative identifier is `-1`."]
        pub(crate) fn get_used_cells_by_id_full(&self, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (i32, Vector2i, i32,);
            let args = (source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_used_cells_by_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_used_cells_by_id_ex`][Self::get_used_cells_by_id_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a [`Vector2i`][crate::builtin::Vector2i] array with the positions of all cells containing a tile. Tiles may be filtered according to their source (`source_id`), their atlas coordinates (`atlas_coords`), or alternative id (`alternative_tile`).\n\nIf a parameter has its value set to the default one, this parameter is not used to filter a cell. Thus, if all parameters have their respective default values, this method returns the same result as [`get_used_cells`][`crate::classes::TileMapLayer::get_used_cells`].\n\nA cell is considered empty if its source identifier equals `-1`, its atlas coordinate identifier is `Vector2(-1, -1)` and its alternative identifier is `-1`."]
        #[inline]
        pub fn get_used_cells_by_id(&self,) -> Array < Vector2i > {
            self.get_used_cells_by_id_ex() . done()
        }
        #[doc = "Returns a [`Vector2i`][crate::builtin::Vector2i] array with the positions of all cells containing a tile. Tiles may be filtered according to their source (`source_id`), their atlas coordinates (`atlas_coords`), or alternative id (`alternative_tile`).\n\nIf a parameter has its value set to the default one, this parameter is not used to filter a cell. Thus, if all parameters have their respective default values, this method returns the same result as [`get_used_cells`][`crate::classes::TileMapLayer::get_used_cells`].\n\nA cell is considered empty if its source identifier equals `-1`, its atlas coordinate identifier is `Vector2(-1, -1)` and its alternative identifier is `-1`."]
        #[inline]
        pub fn get_used_cells_by_id_ex < 'ex > (&'ex self,) -> ExGetUsedCellsById < 'ex > {
            ExGetUsedCellsById::new(self,)
        }
        #[doc = "Returns a rectangle enclosing the used (non-empty) tiles of the map."]
        pub fn get_used_rect(&self,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_used_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates and returns a new [`TileMapPattern`][crate::classes::TileMapPattern] from the given array of cells. See also [`set_pattern`][`crate::classes::TileMapLayer::set_pattern`]."]
        pub fn get_pattern(&self, coords_array: &Array < Vector2i >,) -> Option < Gd < crate::classes::TileMapPattern > > {
            type CallRet = Option < Gd < crate::classes::TileMapPattern > >;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Vector2i > >,);
            let args = (RefArg::new(coords_array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_pattern", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Pastes the [`TileMapPattern`][crate::classes::TileMapPattern] at the given `position` in the tile map. See also [`get_pattern`][`crate::classes::TileMapLayer::get_pattern`]."]
        pub fn set_pattern(&mut self, position: Vector2i, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Vector2i, CowArg < 'a0, Option < Gd < crate::classes::TileMapPattern > > >,);
            let args = (position, pattern.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_pattern", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Update all the cells in the `cells` coordinates array so that they use the given `terrain` for the given `terrain_set`. If an updated cell has the same terrain as one of its neighboring cells, this function tries to join the two. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\n**Note:** To work correctly, this method requires the `TileMapLayer`'s TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        pub(crate) fn set_cells_terrain_connect_full(&mut self, cells: RefArg < Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Vector2i > >, i32, i32, bool,);
            let args = (cells, terrain_set, terrain, ignore_empty_terrains,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_cells_terrain_connect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_cells_terrain_connect_ex`][Self::set_cells_terrain_connect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Update all the cells in the `cells` coordinates array so that they use the given `terrain` for the given `terrain_set`. If an updated cell has the same terrain as one of its neighboring cells, this function tries to join the two. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\n**Note:** To work correctly, this method requires the `TileMapLayer`'s TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        #[inline]
        pub fn set_cells_terrain_connect(&mut self, cells: &Array < Vector2i >, terrain_set: i32, terrain: i32,) {
            self.set_cells_terrain_connect_ex(cells, terrain_set, terrain,) . done()
        }
        #[doc = "Update all the cells in the `cells` coordinates array so that they use the given `terrain` for the given `terrain_set`. If an updated cell has the same terrain as one of its neighboring cells, this function tries to join the two. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\n**Note:** To work correctly, this method requires the `TileMapLayer`'s TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        #[inline]
        pub fn set_cells_terrain_connect_ex < 'ex > (&'ex mut self, cells: &'ex Array < Vector2i >, terrain_set: i32, terrain: i32,) -> ExSetCellsTerrainConnect < 'ex > {
            ExSetCellsTerrainConnect::new(self, cells, terrain_set, terrain,)
        }
        #[doc = "Update all the cells in the `path` coordinates array so that they use the given `terrain` for the given `terrain_set`. The function will also connect two successive cell in the path with the same terrain. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\n**Note:** To work correctly, this method requires the `TileMapLayer`'s TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        pub(crate) fn set_cells_terrain_path_full(&mut self, path: RefArg < Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Vector2i > >, i32, i32, bool,);
            let args = (path, terrain_set, terrain, ignore_empty_terrains,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_cells_terrain_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_cells_terrain_path_ex`][Self::set_cells_terrain_path_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Update all the cells in the `path` coordinates array so that they use the given `terrain` for the given `terrain_set`. The function will also connect two successive cell in the path with the same terrain. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\n**Note:** To work correctly, this method requires the `TileMapLayer`'s TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        #[inline]
        pub fn set_cells_terrain_path(&mut self, path: &Array < Vector2i >, terrain_set: i32, terrain: i32,) {
            self.set_cells_terrain_path_ex(path, terrain_set, terrain,) . done()
        }
        #[doc = "Update all the cells in the `path` coordinates array so that they use the given `terrain` for the given `terrain_set`. The function will also connect two successive cell in the path with the same terrain. This function might update neighboring tiles if needed to create correct terrain transitions.\n\nIf `ignore_empty_terrains` is `true`, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.\n\n**Note:** To work correctly, this method requires the `TileMapLayer`'s TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results."]
        #[inline]
        pub fn set_cells_terrain_path_ex < 'ex > (&'ex mut self, path: &'ex Array < Vector2i >, terrain_set: i32, terrain: i32,) -> ExSetCellsTerrainPath < 'ex > {
            ExSetCellsTerrainPath::new(self, path, terrain_set, terrain,)
        }
        #[doc = "Returns whether the provided `body` [`RID`][crate::builtin::Rid] belongs to one of this `TileMapLayer`'s cells."]
        pub fn has_body_rid(&self, body: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "has_body_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the coordinates of the physics quadrant (see \\[member physics_quadrant_size]) for given physics body [`RID`][crate::builtin::Rid]. Such an [`RID`][crate::builtin::Rid] can be retrieved from [`get_collider_rid`][`crate::classes::KinematicCollision2D::get_collider_rid`], when colliding with a tile.\n\n**Note:** Higher values of \\[member physics_quadrant_size] will make this function less precise. To get the exact cell coordinates, you need to set \\[member physics_quadrant_size] to `1`, which disables physics chunking."]
        pub fn get_coords_for_body_rid(&self, body: Rid,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_coords_for_body_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Triggers a direct update of the `TileMapLayer`. Usually, calling this function is not needed, as `TileMapLayer` node updates automatically when one of its properties or cells is modified.\n\nHowever, for performance reasons, those updates are batched and delayed to the end of the frame. Calling this function will force the `TileMapLayer` to update right away instead.\n\n**Warning:** Updating the `TileMapLayer` is computationally expensive and may impact performance. Try to limit the number of updates and how many tiles they impact."]
        pub fn update_internals(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "update_internals", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Notifies the `TileMapLayer` node that calls to [`use_tile_data_runtime_update`][`crate::classes::ITileMapLayer::use_tile_data_runtime_update`] or [`tile_data_runtime_update`][`crate::classes::ITileMapLayer::tile_data_runtime_update`] will lead to different results. This will thus trigger a `TileMapLayer` update.\n\n**Warning:** Updating the `TileMapLayer` is computationally expensive and may impact performance. Try to limit the number of calls to this function to avoid unnecessary update.\n\n**Note:** This does not trigger a direct update of the `TileMapLayer`, the update will be done at the end of the frame as usual (unless you call [`update_internals`][`crate::classes::TileMapLayer::update_internals`])."]
        pub fn notify_runtime_tile_data_update(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "notify_runtime_tile_data_update", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns for the given coordinates `coords_in_pattern` in a [`TileMapPattern`][crate::classes::TileMapPattern] the corresponding cell coordinates if the pattern was pasted at the `position_in_tilemap` coordinates (see [`set_pattern`][`crate::classes::TileMapLayer::set_pattern`]). This mapping is required as in half-offset tile shapes, the mapping might not work by calculating `position_in_tile_map + coords_in_pattern`."]
        pub fn map_pattern(&mut self, position_in_tilemap: Vector2i, coords_in_pattern: Vector2i, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> >,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams < 'a0, > = (Vector2i, Vector2i, CowArg < 'a0, Option < Gd < crate::classes::TileMapPattern > > >,);
            let args = (position_in_tilemap, coords_in_pattern, pattern.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "map_pattern", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of all neighboring cells to the one at `coords`. Any neighboring cell is one that is touching edges, so for a square cell 4 cells would be returned, for a hexagon 6 cells are returned."]
        pub fn get_surrounding_cells(&self, coords: Vector2i,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_surrounding_cells", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the neighboring cell to the one at coordinates `coords`, identified by the `neighbor` direction. This method takes into account the different layouts a TileMap can take."]
        pub fn get_neighbor_cell(&self, coords: Vector2i, neighbor: crate::classes::tile_set::CellNeighbor,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2i, crate::classes::tile_set::CellNeighbor,);
            let args = (coords, neighbor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_neighbor_cell", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the centered position of a cell in the `TileMapLayer`'s local coordinate space. To convert the returned value into global coordinates, use [`to_global`][`crate::classes::Node2D::to_global`]. See also [`local_to_map`][`crate::classes::TileMapLayer::local_to_map`].\n\n**Note:** This may not correspond to the visual position of the tile, i.e. it ignores the \\[member TileData.texture_origin] property of individual tiles."]
        pub fn map_to_local(&self, map_position: Vector2i,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2i,);
            let args = (map_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "map_to_local", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the map coordinates of the cell containing the given `local_position`. If `local_position` is in global coordinates, consider using [`to_local`][`crate::classes::Node2D::to_local`] before passing it to this method. See also [`map_to_local`][`crate::classes::TileMapLayer::map_to_local`]."]
        pub fn local_to_map(&self, local_position: Vector2,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2,);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "local_to_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_map_data_from_array(&mut self, tile_map_layer_data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(tile_map_layer_data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_tile_map_data_from_array", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tile_map_data_as_array(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_tile_map_data_as_array", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_set(&mut self, tile_set: impl AsArg < Option < Gd < crate::classes::TileSet >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TileSet > > >,);
            let args = (tile_set.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_tile_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tile_set(&self,) -> Option < Gd < crate::classes::TileSet > > {
            type CallRet = Option < Gd < crate::classes::TileSet > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_tile_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_y_sort_origin(&mut self, y_sort_origin: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (y_sort_origin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_y_sort_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_y_sort_origin(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_y_sort_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_x_draw_order_reversed(&mut self, x_draw_order_reversed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (x_draw_order_reversed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_x_draw_order_reversed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_x_draw_order_reversed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_x_draw_order_reversed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rendering_quadrant_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_rendering_quadrant_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rendering_quadrant_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_rendering_quadrant_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_collision_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_collision_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_collision_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_kinematic_bodies(&mut self, use_kinematic_bodies: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_kinematic_bodies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_use_kinematic_bodies", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_kinematic_bodies(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_using_kinematic_bodies", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_visibility_mode(&mut self, visibility_mode: crate::classes::tile_map_layer::DebugVisibilityMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_map_layer::DebugVisibilityMode,);
            let args = (visibility_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_collision_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_visibility_mode(&self,) -> crate::classes::tile_map_layer::DebugVisibilityMode {
            type CallRet = crate::classes::tile_map_layer::DebugVisibilityMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_collision_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_quadrant_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_physics_quadrant_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_quadrant_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_physics_quadrant_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_occlusion_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_occlusion_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_occlusion_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_occlusion_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_navigation_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_navigation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_navigation_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_navigation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a custom `map` as a `NavigationServer2D` navigation map. If not set, uses the default [`World2D`][crate::classes::World2D] navigation map instead."]
        pub fn set_navigation_map(&mut self, map: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the `NavigationServer2D` navigation used by this `TileMapLayer`.\n\nBy default this returns the default [`World2D`][crate::classes::World2D] navigation map, unless a custom map was provided using [`set_navigation_map`][`crate::classes::TileMapLayer::set_navigation_map`]."]
        pub fn get_navigation_map(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_navigation_visibility_mode(&mut self, show_navigation: crate::classes::tile_map_layer::DebugVisibilityMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_map_layer::DebugVisibilityMode,);
            let args = (show_navigation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_navigation_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_navigation_visibility_mode(&self,) -> crate::classes::tile_map_layer::DebugVisibilityMode {
            type CallRet = crate::classes::tile_map_layer::DebugVisibilityMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_navigation_visibility_mode", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TileMapLayer {
        type Base = crate::classes::Node2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TileMapLayer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileMapLayer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for TileMapLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TileMapLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TileMapLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileMapLayer {
        
    }
    impl crate::obj::cap::GodotDefault for TileMapLayer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileMapLayer {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileMapLayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileMapLayer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TileMapLayer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileMapLayer > for $Class {
                
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
#[doc = "Default-param extender for [`TileMapLayer::set_cell_ex`][super::TileMapLayer::set_cell_ex]."]
#[must_use]
pub struct ExSetCell < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileMapLayer, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCell < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileMapLayer, coords: Vector2i,) -> Self {
        let source_id = - 1i32;
        let atlas_coords = Vector2i::new(- 1 as _, - 1 as _);
        let alternative_tile = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, coords: coords, source_id: source_id, atlas_coords: atlas_coords, alternative_tile: alternative_tile,
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
            _phantom, surround_object, coords, source_id, atlas_coords, alternative_tile,
        }
        = self;
        re_export::TileMapLayer::set_cell_full(surround_object, coords, source_id, atlas_coords, alternative_tile,)
    }
}
#[doc = "Default-param extender for [`TileMapLayer::get_used_cells_by_id_ex`][super::TileMapLayer::get_used_cells_by_id_ex]."]
#[must_use]
pub struct ExGetUsedCellsById < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileMapLayer, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetUsedCellsById < 'ex > {
    fn new(surround_object: &'ex re_export::TileMapLayer,) -> Self {
        let source_id = - 1i32;
        let atlas_coords = Vector2i::new(- 1 as _, - 1 as _);
        let alternative_tile = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, source_id: source_id, atlas_coords: atlas_coords, alternative_tile: alternative_tile,
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
            _phantom, surround_object, source_id, atlas_coords, alternative_tile,
        }
        = self;
        re_export::TileMapLayer::get_used_cells_by_id_full(surround_object, source_id, atlas_coords, alternative_tile,)
    }
}
#[doc = "Default-param extender for [`TileMapLayer::set_cells_terrain_connect_ex`][super::TileMapLayer::set_cells_terrain_connect_ex]."]
#[must_use]
pub struct ExSetCellsTerrainConnect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileMapLayer, cells: CowArg < 'ex, Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCellsTerrainConnect < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileMapLayer, cells: &'ex Array < Vector2i >, terrain_set: i32, terrain: i32,) -> Self {
        let ignore_empty_terrains = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, cells: CowArg::Borrowed(cells), terrain_set: terrain_set, terrain: terrain, ignore_empty_terrains: ignore_empty_terrains,
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
            _phantom, surround_object, cells, terrain_set, terrain, ignore_empty_terrains,
        }
        = self;
        re_export::TileMapLayer::set_cells_terrain_connect_full(surround_object, cells.cow_as_arg(), terrain_set, terrain, ignore_empty_terrains,)
    }
}
#[doc = "Default-param extender for [`TileMapLayer::set_cells_terrain_path_ex`][super::TileMapLayer::set_cells_terrain_path_ex]."]
#[must_use]
pub struct ExSetCellsTerrainPath < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileMapLayer, path: CowArg < 'ex, Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCellsTerrainPath < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileMapLayer, path: &'ex Array < Vector2i >, terrain_set: i32, terrain: i32,) -> Self {
        let ignore_empty_terrains = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: CowArg::Borrowed(path), terrain_set: terrain_set, terrain: terrain, ignore_empty_terrains: ignore_empty_terrains,
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
            _phantom, surround_object, path, terrain_set, terrain, ignore_empty_terrains,
        }
        = self;
        re_export::TileMapLayer::set_cells_terrain_path_full(surround_object, path.cow_as_arg(), terrain_set, terrain, ignore_empty_terrains,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DebugVisibilityMode {
    ord: i32
}
impl DebugVisibilityMode {
    #[doc(alias = "DEBUG_VISIBILITY_MODE_DEFAULT")]
    #[doc = "Godot enumerator name: `DEBUG_VISIBILITY_MODE_DEFAULT`"]
    pub const DEFAULT: DebugVisibilityMode = DebugVisibilityMode {
        ord: 0i32
    };
    #[doc(alias = "DEBUG_VISIBILITY_MODE_FORCE_HIDE")]
    #[doc = "Godot enumerator name: `DEBUG_VISIBILITY_MODE_FORCE_HIDE`"]
    pub const FORCE_HIDE: DebugVisibilityMode = DebugVisibilityMode {
        ord: 2i32
    };
    #[doc(alias = "DEBUG_VISIBILITY_MODE_FORCE_SHOW")]
    #[doc = "Godot enumerator name: `DEBUG_VISIBILITY_MODE_FORCE_SHOW`"]
    pub const FORCE_SHOW: DebugVisibilityMode = DebugVisibilityMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DebugVisibilityMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DebugVisibilityMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DebugVisibilityMode {
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
        &[DebugVisibilityMode::DEFAULT, DebugVisibilityMode::FORCE_HIDE, DebugVisibilityMode::FORCE_SHOW]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DebugVisibilityMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "DEBUG_VISIBILITY_MODE_DEFAULT", DebugVisibilityMode::DEFAULT), crate::meta::inspect::EnumConstant::new("FORCE_HIDE", "DEBUG_VISIBILITY_MODE_FORCE_HIDE", DebugVisibilityMode::FORCE_HIDE), crate::meta::inspect::EnumConstant::new("FORCE_SHOW", "DEBUG_VISIBILITY_MODE_FORCE_SHOW", DebugVisibilityMode::FORCE_SHOW)]
        }
    }
}
impl crate::meta::GodotConvert for DebugVisibilityMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Debug Visibility Mode Default", 0i64), EnumeratorShape::new_int("Debug Visibility Mode Force Hide", 2i64), EnumeratorShape::new_int("Debug Visibility Mode Force Show", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TileMapLayer.DebugVisibilityMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DebugVisibilityMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DebugVisibilityMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DebugVisibilityMode {
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
impl crate::registry::property::Export for DebugVisibilityMode {
    
}
impl crate::meta::Element for DebugVisibilityMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TileMapLayer;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`TileMapLayer`][crate::classes::TileMapLayer] class."]
    pub struct SignalsOfTileMapLayer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTileMapLayer < 'c, C > {
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
    impl WithSignals for TileMapLayer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTileMapLayer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTileMapLayer < 'c, C > {
        type Target = < < TileMapLayer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = TileMapLayer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTileMapLayer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = TileMapLayer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}