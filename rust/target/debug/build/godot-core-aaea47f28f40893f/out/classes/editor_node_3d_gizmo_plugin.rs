#![doc = "Sidecar module for class [`EditorNode3DGizmoPlugin`][crate::classes::EditorNode3DGizmoPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorNode3DGizmoPlugin`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`editor_node_3d_gizmo_plugin`][crate::classes::editor_node_3d_gizmo_plugin]: sidecar module with related enum/flag types\n* [`IEditorNode3DGizmoPlugin`][crate::classes::IEditorNode3DGizmoPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin`](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorNode3DGizmoPlugin::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`EditorNode3DGizmoPlugin` allows you to define a new type of Gizmo. There are two main ways to do so: extending `EditorNode3DGizmoPlugin` for the simpler gizmos, or creating a new [`EditorNode3DGizmo`][crate::classes::EditorNode3DGizmo] type. See the tutorial in the documentation for more info.\n\nTo use `EditorNode3DGizmoPlugin`, register it using the [`add_node_3d_gizmo_plugin`][`crate::classes::EditorPlugin::add_node_3d_gizmo_plugin`] method first."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorNode3DGizmoPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorNode3DGizmoPlugin`][crate::classes::EditorNode3DGizmoPlugin].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorNode3DGizmoPlugin: crate::obj::GodotClass < Base = EditorNode3DGizmoPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ObjectNotification) {
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
        #[doc = "Override this method to define which Node3D nodes have a gizmo from this plugin. Whenever a [`Node3D`][crate::classes::Node3D] node is added to a scene this method is called, if it returns `true` the node gets a generic [`EditorNode3DGizmo`][crate::classes::EditorNode3DGizmo] assigned and is added to this plugin's list of active gizmos."]
        fn has_gizmo(&self, for_node_3d: Option < Gd < crate::classes::Node3D > >,) -> bool {
            unimplemented !()
        }
        #[doc = "Override this method to return a custom [`EditorNode3DGizmo`][crate::classes::EditorNode3DGizmo] for the 3D nodes of your choice, return `null` for the rest of nodes. See also [`has_gizmo`][`crate::classes::IEditorNode3DGizmoPlugin::has_gizmo`]."]
        fn create_gizmo(&self, for_node_3d: Option < Gd < crate::classes::Node3D > >,) -> Option < Gd < crate::classes::EditorNode3DGizmo > > {
            unimplemented !()
        }
        #[doc = "Override this method to provide the name that will appear in the gizmo visibility menu."]
        fn get_gizmo_name(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to set the gizmo's priority. Gizmos with higher priority will have precedence when processing inputs like handles or subgizmos selection.\n\nAll built-in editor gizmos return a priority of `-1`. If not overridden, this method will return `0`, which means custom gizmos will automatically get higher priority than built-in gizmos."]
        fn get_priority(&self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Override this method to define whether the gizmos handled by this plugin can be hidden or not. Returns `true` if not overridden."]
        fn can_be_hidden(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Override this method to define whether Node3D with this gizmo should be selectable even when the gizmo is hidden."]
        fn is_selectable_when_hidden(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Override this method to add all the gizmo elements whenever a gizmo update is requested. It's common to call [`clear`][`crate::classes::EditorNode3DGizmo::clear`] at the beginning of this method and then add visual elements depending on the node's properties."]
        fn redraw(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >,) {
            unimplemented !()
        }
        #[doc = "Override this method to provide gizmo's handle names. The `secondary` argument is `true` when the requested handle is secondary (see [`add_handles`][`crate::classes::EditorNode3DGizmo::add_handles`] for more information). Called for this plugin's active gizmos."]
        fn get_handle_name(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to return `true` whenever to given handle should be highlighted in the editor. The `secondary` argument is `true` when the requested handle is secondary (see [`add_handles`][`crate::classes::EditorNode3DGizmo::add_handles`] for more information). Called for this plugin's active gizmos."]
        fn is_handle_highlighted(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool,) -> bool {
            unimplemented !()
        }
        #[doc = "Override this method to return the current value of a handle. This value will be requested at the start of an edit and used as the `restore` argument in [`commit_handle`][`crate::classes::IEditorNode3DGizmoPlugin::commit_handle`].\n\nThe `secondary` argument is `true` when the requested handle is secondary (see [`add_handles`][`crate::classes::EditorNode3DGizmo::add_handles`] for more information).\n\nCalled for this plugin's active gizmos."]
        fn get_handle_value(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool,) -> Variant {
            unimplemented !()
        }
        fn begin_handle_action(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool,) {
            unimplemented !()
        }
        #[doc = "Override this method to update the node's properties when the user drags a gizmo handle (previously added with [`add_handles`][`crate::classes::EditorNode3DGizmo::add_handles`]). The provided `screen_pos` is the mouse position in screen coordinates and the `camera` can be used to convert it to raycasts.\n\nThe `secondary` argument is `true` when the edited handle is secondary (see [`add_handles`][`crate::classes::EditorNode3DGizmo::add_handles`] for more information).\n\nCalled for this plugin's active gizmos."]
        fn set_handle(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool, camera: Option < Gd < crate::classes::Camera3D > >, screen_pos: Vector2,) {
            unimplemented !()
        }
        #[doc = "Override this method to commit a handle being edited (handles must have been previously added by [`add_handles`][`crate::classes::EditorNode3DGizmo::add_handles`] during [`redraw`][`crate::classes::IEditorNode3DGizmoPlugin::redraw`]). This usually means creating an [`UndoRedo`][crate::classes::UndoRedo] action for the change, using the current handle value as \"do\" and the `restore` argument as \"undo\".\n\nIf the `cancel` argument is `true`, the `restore` value should be directly set, without any [`UndoRedo`][crate::classes::UndoRedo] action.\n\nThe `secondary` argument is `true` when the committed handle is secondary (see [`add_handles`][`crate::classes::EditorNode3DGizmo::add_handles`] for more information).\n\nCalled for this plugin's active gizmos."]
        fn commit_handle(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool, restore: Variant, cancel: bool,) {
            unimplemented !()
        }
        #[doc = "Override this method to allow selecting subgizmos using mouse clicks. Given a `camera` and a `screen_pos` in screen coordinates, this method should return which subgizmo should be selected. The returned value should be a unique subgizmo identifier, which can have any non-negative value and will be used in other virtual methods like [`get_subgizmo_transform`][`crate::classes::IEditorNode3DGizmoPlugin::get_subgizmo_transform`] or [`commit_subgizmos`][`crate::classes::IEditorNode3DGizmoPlugin::commit_subgizmos`]. Called for this plugin's active gizmos."]
        fn subgizmos_intersect_ray(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, camera: Option < Gd < crate::classes::Camera3D > >, screen_pos: Vector2,) -> i32 {
            unimplemented !()
        }
        #[doc = "Override this method to allow selecting subgizmos using mouse drag box selection. Given a `camera` and `frustum_planes`, this method should return which subgizmos are contained within the frustums. The `frustum_planes` argument consists of an array with all the [`Plane`][crate::builtin::Plane]s that make up the selection frustum. The returned value should contain a list of unique subgizmo identifiers, these identifiers can have any non-negative value and will be used in other virtual methods like [`get_subgizmo_transform`][`crate::classes::IEditorNode3DGizmoPlugin::get_subgizmo_transform`] or [`commit_subgizmos`][`crate::classes::IEditorNode3DGizmoPlugin::commit_subgizmos`]. Called for this plugin's active gizmos."]
        fn subgizmos_intersect_frustum(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, camera: Option < Gd < crate::classes::Camera3D > >, frustum_planes: Array < Plane >,) -> PackedInt32Array {
            unimplemented !()
        }
        #[doc = "Override this method to return the current transform of a subgizmo. As with all subgizmo methods, the transform should be in local space respect to the gizmo's Node3D. This transform will be requested at the start of an edit and used in the `restore` argument in [`commit_subgizmos`][`crate::classes::IEditorNode3DGizmoPlugin::commit_subgizmos`]. Called for this plugin's active gizmos."]
        fn get_subgizmo_transform(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, subgizmo_id: i32,) -> Transform3D {
            unimplemented !()
        }
        #[doc = "Override this method to update the node properties during subgizmo editing (see [`subgizmos_intersect_ray`][`crate::classes::IEditorNode3DGizmoPlugin::subgizmos_intersect_ray`] and [`subgizmos_intersect_frustum`][`crate::classes::IEditorNode3DGizmoPlugin::subgizmos_intersect_frustum`]). The `transform` is given in the Node3D's local coordinate system. Called for this plugin's active gizmos."]
        fn set_subgizmo_transform(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, subgizmo_id: i32, transform: Transform3D,) {
            unimplemented !()
        }
        #[doc = "Override this method to commit a group of subgizmos being edited (see [`subgizmos_intersect_ray`][`crate::classes::IEditorNode3DGizmoPlugin::subgizmos_intersect_ray`] and [`subgizmos_intersect_frustum`][`crate::classes::IEditorNode3DGizmoPlugin::subgizmos_intersect_frustum`]). This usually means creating an [`UndoRedo`][crate::classes::UndoRedo] action for the change, using the current transforms as \"do\" and the `restores` transforms as \"undo\".\n\nIf the `cancel` argument is `true`, the `restores` transforms should be directly set, without any [`UndoRedo`][crate::classes::UndoRedo] action. As with all subgizmo methods, transforms are given in local space respect to the gizmo's Node3D. Called for this plugin's active gizmos."]
        fn commit_subgizmos(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, ids: PackedInt32Array, restores: Array < Transform3D >, cancel: bool,) {
            unimplemented !()
        }
        #[doc = "Override this method to customize the newly duplicated resource created from [`instantiate`][`crate::classes::PackedScene::instantiate`], if the original's \\[member resource_local_to_scene] is set to `true`.\n\n**Example:** Set a random `damage` value to every local resource from an instantiated scene:\n\n```gdscript\nextends Resource\n\nvar damage = 0\n\nfunc _setup_local_to_scene():\n\tdamage = randi_range(10, 40)\n```"]
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to return a custom [`RID`][crate::builtin::Rid] when [`get_rid`][`crate::classes::Resource::get_rid`] is called."]
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        #[doc = "For resources that store state in non-exported properties, such as via [`on_validate_property`][`crate::classes::IObject::on_validate_property`] or [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`], this method must be implemented to clear them."]
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to execute additional logic after [`set_path_cache`][`crate::classes::Resource::set_path_cache`] is called on this object."]
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl EditorNode3DGizmoPlugin {
        #[doc = "Creates an unshaded material with its variants (selected and/or editable) and adds them to the internal material list. They can then be accessed with [`get_material`][`crate::classes::EditorNode3DGizmoPlugin::get_material`] and used in [`add_mesh`][`crate::classes::EditorNode3DGizmo::add_mesh`] and [`add_lines`][`crate::classes::EditorNode3DGizmo::add_lines`]. Should not be overridden."]
        pub(crate) fn create_material_full(&mut self, name: CowArg < GString >, color: Color, billboard: bool, on_top: bool, use_vertex_color: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Color, bool, bool, bool,);
            let args = (name, color, billboard, on_top, use_vertex_color,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmoPlugin", "create_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_material_ex`][Self::create_material_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates an unshaded material with its variants (selected and/or editable) and adds them to the internal material list. They can then be accessed with [`get_material`][`crate::classes::EditorNode3DGizmoPlugin::get_material`] and used in [`add_mesh`][`crate::classes::EditorNode3DGizmo::add_mesh`] and [`add_lines`][`crate::classes::EditorNode3DGizmo::add_lines`]. Should not be overridden."]
        #[inline]
        pub fn create_material(&mut self, name: impl AsArg < GString >, color: Color,) {
            self.create_material_ex(name, color,) . done()
        }
        #[doc = "Creates an unshaded material with its variants (selected and/or editable) and adds them to the internal material list. They can then be accessed with [`get_material`][`crate::classes::EditorNode3DGizmoPlugin::get_material`] and used in [`add_mesh`][`crate::classes::EditorNode3DGizmo::add_mesh`] and [`add_lines`][`crate::classes::EditorNode3DGizmo::add_lines`]. Should not be overridden."]
        #[inline]
        pub fn create_material_ex < 'ex > (&'ex mut self, name: impl AsArg < GString > + 'ex, color: Color,) -> ExCreateMaterial < 'ex > {
            ExCreateMaterial::new(self, name, color,)
        }
        #[doc = "Creates an icon material with its variants (selected and/or editable) and adds them to the internal material list. They can then be accessed with [`get_material`][`crate::classes::EditorNode3DGizmoPlugin::get_material`] and used in [`add_unscaled_billboard`][`crate::classes::EditorNode3DGizmo::add_unscaled_billboard`]. Should not be overridden."]
        pub(crate) fn create_icon_material_full(&mut self, name: CowArg < GString >, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, on_top: bool, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, bool, Color,);
            let args = (name, texture, on_top, color,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmoPlugin", "create_icon_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_icon_material_ex`][Self::create_icon_material_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates an icon material with its variants (selected and/or editable) and adds them to the internal material list. They can then be accessed with [`get_material`][`crate::classes::EditorNode3DGizmoPlugin::get_material`] and used in [`add_unscaled_billboard`][`crate::classes::EditorNode3DGizmo::add_unscaled_billboard`]. Should not be overridden."]
        #[inline]
        pub fn create_icon_material(&mut self, name: impl AsArg < GString >, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.create_icon_material_ex(name, texture,) . done()
        }
        #[doc = "Creates an icon material with its variants (selected and/or editable) and adds them to the internal material list. They can then be accessed with [`get_material`][`crate::classes::EditorNode3DGizmoPlugin::get_material`] and used in [`add_unscaled_billboard`][`crate::classes::EditorNode3DGizmo::add_unscaled_billboard`]. Should not be overridden."]
        #[inline]
        pub fn create_icon_material_ex < 'ex > (&'ex mut self, name: impl AsArg < GString > + 'ex, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> ExCreateIconMaterial < 'ex > {
            ExCreateIconMaterial::new(self, name, texture,)
        }
        #[doc = "Creates a handle material with its variants (selected and/or editable) and adds them to the internal material list. They can then be accessed with [`get_material`][`crate::classes::EditorNode3DGizmoPlugin::get_material`] and used in [`add_handles`][`crate::classes::EditorNode3DGizmo::add_handles`]. Should not be overridden.\n\nYou can optionally provide a texture to use instead of the default icon."]
        pub(crate) fn create_handle_material_full(&mut self, name: CowArg < GString >, billboard: bool, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, bool, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (name, billboard, texture,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmoPlugin", "create_handle_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_handle_material_ex`][Self::create_handle_material_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a handle material with its variants (selected and/or editable) and adds them to the internal material list. They can then be accessed with [`get_material`][`crate::classes::EditorNode3DGizmoPlugin::get_material`] and used in [`add_handles`][`crate::classes::EditorNode3DGizmo::add_handles`]. Should not be overridden.\n\nYou can optionally provide a texture to use instead of the default icon."]
        #[inline]
        pub fn create_handle_material(&mut self, name: impl AsArg < GString >,) {
            self.create_handle_material_ex(name,) . done()
        }
        #[doc = "Creates a handle material with its variants (selected and/or editable) and adds them to the internal material list. They can then be accessed with [`get_material`][`crate::classes::EditorNode3DGizmoPlugin::get_material`] and used in [`add_handles`][`crate::classes::EditorNode3DGizmo::add_handles`]. Should not be overridden.\n\nYou can optionally provide a texture to use instead of the default icon."]
        #[inline]
        pub fn create_handle_material_ex < 'ex > (&'ex mut self, name: impl AsArg < GString > + 'ex,) -> ExCreateHandleMaterial < 'ex > {
            ExCreateHandleMaterial::new(self, name,)
        }
        #[doc = "Adds a new material to the internal material list for the plugin. It can then be accessed with [`get_material`][`crate::classes::EditorNode3DGizmoPlugin::get_material`]. Should not be overridden."]
        pub fn add_material(&mut self, name: impl AsArg < GString >, material: impl AsArg < Option < Gd < crate::classes::StandardMaterial3D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::StandardMaterial3D > > >,);
            let args = (name.into_arg(), material.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmoPlugin", "add_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets material from the internal list of materials. If an [`EditorNode3DGizmo`][crate::classes::EditorNode3DGizmo] is provided, it will try to get the corresponding variant (selected and/or editable)."]
        pub(crate) fn get_material_full(&self, name: CowArg < GString >, gizmo: CowArg < Option < Gd < crate::classes::EditorNode3DGizmo > > >,) -> Option < Gd < crate::classes::StandardMaterial3D > > {
            type CallRet = Option < Gd < crate::classes::StandardMaterial3D > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::EditorNode3DGizmo > > >,);
            let args = (name, gizmo,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmoPlugin", "get_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_material_ex`][Self::get_material_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Gets material from the internal list of materials. If an [`EditorNode3DGizmo`][crate::classes::EditorNode3DGizmo] is provided, it will try to get the corresponding variant (selected and/or editable)."]
        #[inline]
        pub fn get_material(&self, name: impl AsArg < GString >,) -> Option < Gd < crate::classes::StandardMaterial3D > > {
            self.get_material_ex(name,) . done()
        }
        #[doc = "Gets material from the internal list of materials. If an [`EditorNode3DGizmo`][crate::classes::EditorNode3DGizmo] is provided, it will try to get the corresponding variant (selected and/or editable)."]
        #[inline]
        pub fn get_material_ex < 'ex > (&'ex self, name: impl AsArg < GString > + 'ex,) -> ExGetMaterial < 'ex > {
            ExGetMaterial::new(self, name,)
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
    impl crate::obj::GodotClass for EditorNode3DGizmoPlugin {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorNode3DGizmoPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorNode3DGizmoPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for EditorNode3DGizmoPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorNode3DGizmoPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorNode3DGizmoPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorNode3DGizmoPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorNode3DGizmoPlugin {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorNode3DGizmoPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorNode3DGizmoPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorNode3DGizmoPlugin__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorNode3DGizmoPlugin > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::create_material_ex`][super::EditorNode3DGizmoPlugin::create_material_ex]."]
#[must_use]
pub struct ExCreateMaterial < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorNode3DGizmoPlugin, name: CowArg < 'ex, GString >, color: Color, billboard: bool, on_top: bool, use_vertex_color: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateMaterial < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorNode3DGizmoPlugin, name: impl AsArg < GString > + 'ex, color: Color,) -> Self {
        let billboard = false;
        let on_top = false;
        let use_vertex_color = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), color: color, billboard: billboard, on_top: on_top, use_vertex_color: use_vertex_color,
        }
    }
    #[inline]
    pub fn billboard(self, billboard: bool) -> Self {
        Self {
            billboard: billboard, .. self
        }
    }
    #[inline]
    pub fn on_top(self, on_top: bool) -> Self {
        Self {
            on_top: on_top, .. self
        }
    }
    #[inline]
    pub fn use_vertex_color(self, use_vertex_color: bool) -> Self {
        Self {
            use_vertex_color: use_vertex_color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, color, billboard, on_top, use_vertex_color,
        }
        = self;
        re_export::EditorNode3DGizmoPlugin::create_material_full(surround_object, name, color, billboard, on_top, use_vertex_color,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::create_icon_material_ex`][super::EditorNode3DGizmoPlugin::create_icon_material_ex]."]
#[must_use]
pub struct ExCreateIconMaterial < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorNode3DGizmoPlugin, name: CowArg < 'ex, GString >, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, on_top: bool, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateIconMaterial < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorNode3DGizmoPlugin, name: impl AsArg < GString > + 'ex, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> Self {
        let on_top = false;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), texture: texture.into_arg(), on_top: on_top, color: color,
        }
    }
    #[inline]
    pub fn on_top(self, on_top: bool) -> Self {
        Self {
            on_top: on_top, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, texture, on_top, color,
        }
        = self;
        re_export::EditorNode3DGizmoPlugin::create_icon_material_full(surround_object, name, texture, on_top, color,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::create_handle_material_ex`][super::EditorNode3DGizmoPlugin::create_handle_material_ex]."]
#[must_use]
pub struct ExCreateHandleMaterial < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorNode3DGizmoPlugin, name: CowArg < 'ex, GString >, billboard: bool, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateHandleMaterial < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorNode3DGizmoPlugin, name: impl AsArg < GString > + 'ex,) -> Self {
        let billboard = false;
        let texture = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), billboard: billboard, texture: texture.into_arg(),
        }
    }
    #[inline]
    pub fn billboard(self, billboard: bool) -> Self {
        Self {
            billboard: billboard, .. self
        }
    }
    #[inline]
    pub fn texture(self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex) -> Self {
        Self {
            texture: texture.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, billboard, texture,
        }
        = self;
        re_export::EditorNode3DGizmoPlugin::create_handle_material_full(surround_object, name, billboard, texture,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::get_material_ex`][super::EditorNode3DGizmoPlugin::get_material_ex]."]
#[must_use]
pub struct ExGetMaterial < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::EditorNode3DGizmoPlugin, name: CowArg < 'ex, GString >, gizmo: CowArg < 'ex, Option < Gd < crate::classes::EditorNode3DGizmo > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetMaterial < 'ex > {
    fn new(surround_object: &'ex re_export::EditorNode3DGizmoPlugin, name: impl AsArg < GString > + 'ex,) -> Self {
        let gizmo = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), gizmo: gizmo.into_arg(),
        }
    }
    #[inline]
    pub fn gizmo(self, gizmo: impl AsArg < Option < Gd < crate::classes::EditorNode3DGizmo >> > + 'ex) -> Self {
        Self {
            gizmo: gizmo.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::StandardMaterial3D > > {
        let Self {
            _phantom, surround_object, name, gizmo,
        }
        = self;
        re_export::EditorNode3DGizmoPlugin::get_material_full(surround_object, name, gizmo,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorNode3DGizmoPlugin;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for EditorNode3DGizmoPlugin {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}