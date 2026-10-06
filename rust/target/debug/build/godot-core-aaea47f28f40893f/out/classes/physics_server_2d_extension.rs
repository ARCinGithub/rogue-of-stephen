#![doc = "Sidecar module for class [`PhysicsServer2DExtension`][crate::classes::PhysicsServer2DExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsServer2DExtension` enums](https://docs.godotengine.org/en/stable/classes/class_physicsserver2dextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsServer2DExtension`.\n\nInherits [`PhysicsServer2D`][crate::classes::PhysicsServer2D].\n\nRelated symbols:\n\n* [`IPhysicsServer2DExtension`][crate::classes::IPhysicsServer2DExtension]: virtual methods\n\n\nSee also [Godot docs for `PhysicsServer2DExtension`](https://docs.godotengine.org/en/stable/classes/class_physicsserver2dextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`PhysicsServer2DExtension::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nThis class extends [`PhysicsServer2D`][crate::classes::PhysicsServer2D] by providing additional virtual methods that can be overridden. When these methods are overridden, they will be called instead of the internal methods of the physics server.\n\nIntended for use with GDExtension to create custom implementations of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsServer2DExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PhysicsServer2DExtension`][crate::classes::PhysicsServer2DExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IPhysicsServer2D`~~ > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `PhysicsServer2DExtension` methods](https://docs.godotengine.org/en/stable/classes/class_physicsserver2dextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsServer2DExtension: crate::obj::GodotClass < Base = PhysicsServer2DExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Overridable version of [`world_boundary_shape_create`][`crate::classes::PhysicsServer2D::world_boundary_shape_create`]."]
        fn world_boundary_shape_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`separation_ray_shape_create`][`crate::classes::PhysicsServer2D::separation_ray_shape_create`]."]
        fn separation_ray_shape_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`segment_shape_create`][`crate::classes::PhysicsServer2D::segment_shape_create`]."]
        fn segment_shape_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`circle_shape_create`][`crate::classes::PhysicsServer2D::circle_shape_create`]."]
        fn circle_shape_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`rectangle_shape_create`][`crate::classes::PhysicsServer2D::rectangle_shape_create`]."]
        fn rectangle_shape_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`capsule_shape_create`][`crate::classes::PhysicsServer2D::capsule_shape_create`]."]
        fn capsule_shape_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`convex_polygon_shape_create`][`crate::classes::PhysicsServer2D::convex_polygon_shape_create`]."]
        fn convex_polygon_shape_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`concave_polygon_shape_create`][`crate::classes::PhysicsServer2D::concave_polygon_shape_create`]."]
        fn concave_polygon_shape_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`shape_set_data`][`crate::classes::PhysicsServer2D::shape_set_data`]."]
        fn shape_set_data(&mut self, shape: Rid, data: Variant,);
        #[doc = "Should set the custom solver bias for the given `shape`. It defines how much bodies are forced to separate on contact.\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `shape_get_custom_solver_bias` method. Corresponds to \\[member Shape2D.custom_solver_bias]."]
        fn shape_set_custom_solver_bias(&mut self, shape: Rid, bias: f32,);
        #[doc = "Overridable version of [`shape_get_type`][`crate::classes::PhysicsServer2D::shape_get_type`]."]
        fn shape_get_type(&self, shape: Rid,) -> crate::classes::physics_server_2d::ShapeType;
        #[doc = "Overridable version of [`shape_get_data`][`crate::classes::PhysicsServer2D::shape_get_data`]."]
        fn shape_get_data(&self, shape: Rid,) -> Variant;
        #[doc = "Should return the custom solver bias of the given `shape`, which defines how much bodies are forced to separate on contact when this shape is involved.\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `shape_get_custom_solver_bias` method. Corresponds to \\[member Shape2D.custom_solver_bias]."]
        fn shape_get_custom_solver_bias(&self, shape: Rid,) -> f32;
        #[doc = "\n# Godot docs\nGiven two shapes and their parameters, should return `true` if a collision between the two would occur, with additional details passed in `results`.\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `shape_collide` method. Corresponds to [`collide_shape`][`crate::classes::PhysicsDirectSpaceState2D::collide_shape`]."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn shape_collide_rawptr(&mut self, shape_A: Rid, xform_A: Transform2D, motion_A: Vector2, shape_B: Rid, xform_B: Transform2D, motion_B: Vector2, results: crate::meta::RawPtr < * mut c_void >, result_max: i32, result_count: crate::meta::RawPtr < * mut i32 >,) -> bool;
        #[doc = "Overridable version of [`space_create`][`crate::classes::PhysicsServer2D::space_create`]."]
        fn space_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`space_set_active`][`crate::classes::PhysicsServer2D::space_set_active`]."]
        fn space_set_active(&mut self, space: Rid, active: bool,);
        #[doc = "Overridable version of [`space_is_active`][`crate::classes::PhysicsServer2D::space_is_active`]."]
        fn space_is_active(&self, space: Rid,) -> bool;
        #[doc = "Overridable version of [`space_set_param`][`crate::classes::PhysicsServer2D::space_set_param`]."]
        fn space_set_param(&mut self, space: Rid, param: crate::classes::physics_server_2d::SpaceParameter, value: f32,);
        #[doc = "Overridable version of [`space_get_param`][`crate::classes::PhysicsServer2D::space_get_param`]."]
        fn space_get_param(&self, space: Rid, param: crate::classes::physics_server_2d::SpaceParameter,) -> f32;
        #[doc = "Overridable version of [`space_get_direct_state`][`crate::classes::PhysicsServer2D::space_get_direct_state`]."]
        fn space_get_direct_state(&mut self, space: Rid,) -> Option < Gd < crate::classes::PhysicsDirectSpaceState2D > >;
        #[doc = "Used internally to allow the given `space` to store contact points, up to `max_contacts`. This is automatically set for the main [`World2D`][crate::classes::World2D]'s space when \\[member SceneTree.debug_collisions_hint] is `true`, or by checking \"Visible Collision Shapes\" in the editor. Only works in debug builds.\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `space_set_debug_contacts` method."]
        fn space_set_debug_contacts(&mut self, space: Rid, max_contacts: i32,);
        #[doc = "Should return the positions of all contacts that have occurred during the last physics step in the given `space`. See also [`space_get_contact_count`][`crate::classes::IPhysicsServer2DExtension::space_get_contact_count`] and [`space_set_debug_contacts`][`crate::classes::IPhysicsServer2DExtension::space_set_debug_contacts`].\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `space_get_contacts` method."]
        fn space_get_contacts(&self, space: Rid,) -> PackedVector2Array;
        #[doc = "Should return how many contacts have occurred during the last physics step in the given `space`. See also [`space_get_contacts`][`crate::classes::IPhysicsServer2DExtension::space_get_contacts`] and [`space_set_debug_contacts`][`crate::classes::IPhysicsServer2DExtension::space_set_debug_contacts`].\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `space_get_contact_count` method."]
        fn space_get_contact_count(&self, space: Rid,) -> i32;
        #[doc = "Overridable version of [`area_create`][`crate::classes::PhysicsServer2D::area_create`]."]
        fn area_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`area_set_space`][`crate::classes::PhysicsServer2D::area_set_space`]."]
        fn area_set_space(&mut self, area: Rid, space: Rid,);
        #[doc = "Overridable version of [`area_get_space`][`crate::classes::PhysicsServer2D::area_get_space`]."]
        fn area_get_space(&self, area: Rid,) -> Rid;
        #[doc = "Overridable version of [`area_add_shape`][`crate::classes::PhysicsServer2D::area_add_shape`]."]
        fn area_add_shape(&mut self, area: Rid, shape: Rid, transform: Transform2D, disabled: bool,);
        #[doc = "Overridable version of [`area_set_shape`][`crate::classes::PhysicsServer2D::area_set_shape`]."]
        fn area_set_shape(&mut self, area: Rid, shape_idx: i32, shape: Rid,);
        #[doc = "Overridable version of [`area_set_shape_transform`][`crate::classes::PhysicsServer2D::area_set_shape_transform`]."]
        fn area_set_shape_transform(&mut self, area: Rid, shape_idx: i32, transform: Transform2D,);
        #[doc = "Overridable version of [`area_set_shape_disabled`][`crate::classes::PhysicsServer2D::area_set_shape_disabled`]."]
        fn area_set_shape_disabled(&mut self, area: Rid, shape_idx: i32, disabled: bool,);
        #[doc = "Overridable version of [`area_get_shape_count`][`crate::classes::PhysicsServer2D::area_get_shape_count`]."]
        fn area_get_shape_count(&self, area: Rid,) -> i32;
        #[doc = "Overridable version of [`area_get_shape`][`crate::classes::PhysicsServer2D::area_get_shape`]."]
        fn area_get_shape(&self, area: Rid, shape_idx: i32,) -> Rid;
        #[doc = "Overridable version of [`area_get_shape_transform`][`crate::classes::PhysicsServer2D::area_get_shape_transform`]."]
        fn area_get_shape_transform(&self, area: Rid, shape_idx: i32,) -> Transform2D;
        #[doc = "Overridable version of [`area_remove_shape`][`crate::classes::PhysicsServer2D::area_remove_shape`]."]
        fn area_remove_shape(&mut self, area: Rid, shape_idx: i32,);
        #[doc = "Overridable version of [`area_clear_shapes`][`crate::classes::PhysicsServer2D::area_clear_shapes`]."]
        fn area_clear_shapes(&mut self, area: Rid,);
        #[doc = "Overridable version of [`area_attach_object_instance_id`][`crate::classes::PhysicsServer2D::area_attach_object_instance_id`]."]
        fn area_attach_object_instance_id(&mut self, area: Rid, id: u64,);
        #[doc = "Overridable version of [`area_get_object_instance_id`][`crate::classes::PhysicsServer2D::area_get_object_instance_id`]."]
        fn area_get_object_instance_id(&self, area: Rid,) -> u64;
        #[doc = "Overridable version of [`area_attach_canvas_instance_id`][`crate::classes::PhysicsServer2D::area_attach_canvas_instance_id`]."]
        fn area_attach_canvas_instance_id(&mut self, area: Rid, id: u64,);
        #[doc = "Overridable version of [`area_get_canvas_instance_id`][`crate::classes::PhysicsServer2D::area_get_canvas_instance_id`]."]
        fn area_get_canvas_instance_id(&self, area: Rid,) -> u64;
        #[doc = "Overridable version of [`area_set_param`][`crate::classes::PhysicsServer2D::area_set_param`]."]
        fn area_set_param(&mut self, area: Rid, param: crate::classes::physics_server_2d::AreaParameter, value: Variant,);
        #[doc = "Overridable version of [`area_set_transform`][`crate::classes::PhysicsServer2D::area_set_transform`]."]
        fn area_set_transform(&mut self, area: Rid, transform: Transform2D,);
        #[doc = "Overridable version of [`area_get_param`][`crate::classes::PhysicsServer2D::area_get_param`]."]
        fn area_get_param(&self, area: Rid, param: crate::classes::physics_server_2d::AreaParameter,) -> Variant;
        #[doc = "Overridable version of [`area_get_transform`][`crate::classes::PhysicsServer2D::area_get_transform`]."]
        fn area_get_transform(&self, area: Rid,) -> Transform2D;
        #[doc = "Overridable version of [`area_set_collision_layer`][`crate::classes::PhysicsServer2D::area_set_collision_layer`]."]
        fn area_set_collision_layer(&mut self, area: Rid, layer: u32,);
        #[doc = "Overridable version of [`area_get_collision_layer`][`crate::classes::PhysicsServer2D::area_get_collision_layer`]."]
        fn area_get_collision_layer(&self, area: Rid,) -> u32;
        #[doc = "Overridable version of [`area_set_collision_mask`][`crate::classes::PhysicsServer2D::area_set_collision_mask`]."]
        fn area_set_collision_mask(&mut self, area: Rid, mask: u32,);
        #[doc = "Overridable version of [`area_get_collision_mask`][`crate::classes::PhysicsServer2D::area_get_collision_mask`]."]
        fn area_get_collision_mask(&self, area: Rid,) -> u32;
        #[doc = "Overridable version of [`area_set_monitorable`][`crate::classes::PhysicsServer2D::area_set_monitorable`]."]
        fn area_set_monitorable(&mut self, area: Rid, monitorable: bool,);
        #[doc = "If set to `true`, allows the area with the given [`RID`][crate::builtin::Rid] to detect mouse inputs when the mouse cursor is hovering on it.\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `area_set_pickable` method. Corresponds to \\[member CollisionObject2D.input_pickable]."]
        fn area_set_pickable(&mut self, area: Rid, pickable: bool,);
        #[doc = "Overridable version of [`area_set_monitor_callback`][`crate::classes::PhysicsServer2D::area_set_monitor_callback`]."]
        fn area_set_monitor_callback(&mut self, area: Rid, callback: Callable,);
        #[doc = "Overridable version of [`area_set_area_monitor_callback`][`crate::classes::PhysicsServer2D::area_set_area_monitor_callback`]."]
        fn area_set_area_monitor_callback(&mut self, area: Rid, callback: Callable,);
        #[doc = "Overridable version of [`body_create`][`crate::classes::PhysicsServer2D::body_create`]."]
        fn body_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`body_set_space`][`crate::classes::PhysicsServer2D::body_set_space`]."]
        fn body_set_space(&mut self, body: Rid, space: Rid,);
        #[doc = "Overridable version of [`body_get_space`][`crate::classes::PhysicsServer2D::body_get_space`]."]
        fn body_get_space(&self, body: Rid,) -> Rid;
        #[doc = "Overridable version of [`body_set_mode`][`crate::classes::PhysicsServer2D::body_set_mode`]."]
        fn body_set_mode(&mut self, body: Rid, mode: crate::classes::physics_server_2d::BodyMode,);
        #[doc = "Overridable version of [`body_get_mode`][`crate::classes::PhysicsServer2D::body_get_mode`]."]
        fn body_get_mode(&self, body: Rid,) -> crate::classes::physics_server_2d::BodyMode;
        #[doc = "Overridable version of [`body_add_shape`][`crate::classes::PhysicsServer2D::body_add_shape`]."]
        fn body_add_shape(&mut self, body: Rid, shape: Rid, transform: Transform2D, disabled: bool,);
        #[doc = "Overridable version of [`body_set_shape`][`crate::classes::PhysicsServer2D::body_set_shape`]."]
        fn body_set_shape(&mut self, body: Rid, shape_idx: i32, shape: Rid,);
        #[doc = "Overridable version of [`body_set_shape_transform`][`crate::classes::PhysicsServer2D::body_set_shape_transform`]."]
        fn body_set_shape_transform(&mut self, body: Rid, shape_idx: i32, transform: Transform2D,);
        #[doc = "Overridable version of [`body_get_shape_count`][`crate::classes::PhysicsServer2D::body_get_shape_count`]."]
        fn body_get_shape_count(&self, body: Rid,) -> i32;
        #[doc = "Overridable version of [`body_get_shape`][`crate::classes::PhysicsServer2D::body_get_shape`]."]
        fn body_get_shape(&self, body: Rid, shape_idx: i32,) -> Rid;
        #[doc = "Overridable version of [`body_get_shape_transform`][`crate::classes::PhysicsServer2D::body_get_shape_transform`]."]
        fn body_get_shape_transform(&self, body: Rid, shape_idx: i32,) -> Transform2D;
        #[doc = "Overridable version of [`body_set_shape_disabled`][`crate::classes::PhysicsServer2D::body_set_shape_disabled`]."]
        fn body_set_shape_disabled(&mut self, body: Rid, shape_idx: i32, disabled: bool,);
        #[doc = "Overridable version of [`body_set_shape_as_one_way_collision`][`crate::classes::PhysicsServer2D::body_set_shape_as_one_way_collision`]."]
        fn body_set_shape_as_one_way_collision(&mut self, body: Rid, shape_idx: i32, enable: bool, margin: f32,);
        #[doc = "Overridable version of [`body_remove_shape`][`crate::classes::PhysicsServer2D::body_remove_shape`]."]
        fn body_remove_shape(&mut self, body: Rid, shape_idx: i32,);
        #[doc = "Overridable version of [`body_clear_shapes`][`crate::classes::PhysicsServer2D::body_clear_shapes`]."]
        fn body_clear_shapes(&mut self, body: Rid,);
        #[doc = "Overridable version of [`body_attach_object_instance_id`][`crate::classes::PhysicsServer2D::body_attach_object_instance_id`]."]
        fn body_attach_object_instance_id(&mut self, body: Rid, id: u64,);
        #[doc = "Overridable version of [`body_get_object_instance_id`][`crate::classes::PhysicsServer2D::body_get_object_instance_id`]."]
        fn body_get_object_instance_id(&self, body: Rid,) -> u64;
        #[doc = "Overridable version of [`body_attach_canvas_instance_id`][`crate::classes::PhysicsServer2D::body_attach_canvas_instance_id`]."]
        fn body_attach_canvas_instance_id(&mut self, body: Rid, id: u64,);
        #[doc = "Overridable version of [`body_get_canvas_instance_id`][`crate::classes::PhysicsServer2D::body_get_canvas_instance_id`]."]
        fn body_get_canvas_instance_id(&self, body: Rid,) -> u64;
        #[doc = "Overridable version of [`body_set_continuous_collision_detection_mode`][`crate::classes::PhysicsServer2D::body_set_continuous_collision_detection_mode`]."]
        fn body_set_continuous_collision_detection_mode(&mut self, body: Rid, mode: crate::classes::physics_server_2d::CcdMode,);
        #[doc = "Overridable version of [`body_get_continuous_collision_detection_mode`][`crate::classes::PhysicsServer2D::body_get_continuous_collision_detection_mode`]."]
        fn body_get_continuous_collision_detection_mode(&self, body: Rid,) -> crate::classes::physics_server_2d::CcdMode;
        #[doc = "Overridable version of [`body_set_collision_layer`][`crate::classes::PhysicsServer2D::body_set_collision_layer`]."]
        fn body_set_collision_layer(&mut self, body: Rid, layer: u32,);
        #[doc = "Overridable version of [`body_get_collision_layer`][`crate::classes::PhysicsServer2D::body_get_collision_layer`]."]
        fn body_get_collision_layer(&self, body: Rid,) -> u32;
        #[doc = "Overridable version of [`body_set_collision_mask`][`crate::classes::PhysicsServer2D::body_set_collision_mask`]."]
        fn body_set_collision_mask(&mut self, body: Rid, mask: u32,);
        #[doc = "Overridable version of [`body_get_collision_mask`][`crate::classes::PhysicsServer2D::body_get_collision_mask`]."]
        fn body_get_collision_mask(&self, body: Rid,) -> u32;
        #[doc = "Overridable version of [`body_set_collision_priority`][`crate::classes::PhysicsServer2D::body_set_collision_priority`]."]
        fn body_set_collision_priority(&mut self, body: Rid, priority: f32,);
        #[doc = "Overridable version of [`body_get_collision_priority`][`crate::classes::PhysicsServer2D::body_get_collision_priority`]."]
        fn body_get_collision_priority(&self, body: Rid,) -> f32;
        #[doc = "Overridable version of [`body_set_param`][`crate::classes::PhysicsServer2D::body_set_param`]."]
        fn body_set_param(&mut self, body: Rid, param: crate::classes::physics_server_2d::BodyParameter, value: Variant,);
        #[doc = "Overridable version of [`body_get_param`][`crate::classes::PhysicsServer2D::body_get_param`]."]
        fn body_get_param(&self, body: Rid, param: crate::classes::physics_server_2d::BodyParameter,) -> Variant;
        #[doc = "Overridable version of [`body_reset_mass_properties`][`crate::classes::PhysicsServer2D::body_reset_mass_properties`]."]
        fn body_reset_mass_properties(&mut self, body: Rid,);
        #[doc = "Overridable version of [`body_set_state`][`crate::classes::PhysicsServer2D::body_set_state`]."]
        fn body_set_state(&mut self, body: Rid, state: crate::classes::physics_server_2d::BodyState, value: Variant,);
        #[doc = "Overridable version of [`body_get_state`][`crate::classes::PhysicsServer2D::body_get_state`]."]
        fn body_get_state(&self, body: Rid, state: crate::classes::physics_server_2d::BodyState,) -> Variant;
        #[doc = "Overridable version of [`body_apply_central_impulse`][`crate::classes::PhysicsServer2D::body_apply_central_impulse`]."]
        fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vector2,);
        #[doc = "Overridable version of [`body_apply_torque_impulse`][`crate::classes::PhysicsServer2D::body_apply_torque_impulse`]."]
        fn body_apply_torque_impulse(&mut self, body: Rid, impulse: f32,);
        #[doc = "Overridable version of [`body_apply_impulse`][`crate::classes::PhysicsServer2D::body_apply_impulse`]."]
        fn body_apply_impulse(&mut self, body: Rid, impulse: Vector2, position: Vector2,);
        #[doc = "Overridable version of [`body_apply_central_force`][`crate::classes::PhysicsServer2D::body_apply_central_force`]."]
        fn body_apply_central_force(&mut self, body: Rid, force: Vector2,);
        #[doc = "Overridable version of [`body_apply_force`][`crate::classes::PhysicsServer2D::body_apply_force`]."]
        fn body_apply_force(&mut self, body: Rid, force: Vector2, position: Vector2,);
        #[doc = "Overridable version of [`body_apply_torque`][`crate::classes::PhysicsServer2D::body_apply_torque`]."]
        fn body_apply_torque(&mut self, body: Rid, torque: f32,);
        #[doc = "Overridable version of [`body_add_constant_central_force`][`crate::classes::PhysicsServer2D::body_add_constant_central_force`]."]
        fn body_add_constant_central_force(&mut self, body: Rid, force: Vector2,);
        #[doc = "Overridable version of [`body_add_constant_force`][`crate::classes::PhysicsServer2D::body_add_constant_force`]."]
        fn body_add_constant_force(&mut self, body: Rid, force: Vector2, position: Vector2,);
        #[doc = "Overridable version of [`body_add_constant_torque`][`crate::classes::PhysicsServer2D::body_add_constant_torque`]."]
        fn body_add_constant_torque(&mut self, body: Rid, torque: f32,);
        #[doc = "Overridable version of [`body_set_constant_force`][`crate::classes::PhysicsServer2D::body_set_constant_force`]."]
        fn body_set_constant_force(&mut self, body: Rid, force: Vector2,);
        #[doc = "Overridable version of [`body_get_constant_force`][`crate::classes::PhysicsServer2D::body_get_constant_force`]."]
        fn body_get_constant_force(&self, body: Rid,) -> Vector2;
        #[doc = "Overridable version of [`body_set_constant_torque`][`crate::classes::PhysicsServer2D::body_set_constant_torque`]."]
        fn body_set_constant_torque(&mut self, body: Rid, torque: f32,);
        #[doc = "Overridable version of [`body_get_constant_torque`][`crate::classes::PhysicsServer2D::body_get_constant_torque`]."]
        fn body_get_constant_torque(&self, body: Rid,) -> f32;
        #[doc = "Overridable version of [`body_set_axis_velocity`][`crate::classes::PhysicsServer2D::body_set_axis_velocity`]."]
        fn body_set_axis_velocity(&mut self, body: Rid, axis_velocity: Vector2,);
        #[doc = "Overridable version of [`body_add_collision_exception`][`crate::classes::PhysicsServer2D::body_add_collision_exception`]."]
        fn body_add_collision_exception(&mut self, body: Rid, excepted_body: Rid,);
        #[doc = "Overridable version of [`body_remove_collision_exception`][`crate::classes::PhysicsServer2D::body_remove_collision_exception`]."]
        fn body_remove_collision_exception(&mut self, body: Rid, excepted_body: Rid,);
        #[doc = "Returns the [`RID`][crate::builtin::Rid]s of all bodies added as collision exceptions for the given `body`. See also [`body_add_collision_exception`][`crate::classes::IPhysicsServer2DExtension::body_add_collision_exception`] and [`body_remove_collision_exception`][`crate::classes::IPhysicsServer2DExtension::body_remove_collision_exception`].\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `body_get_collision_exceptions` method. Corresponds to [`get_collision_exceptions`][`crate::classes::PhysicsBody2D::get_collision_exceptions`]."]
        fn body_get_collision_exceptions(&self, body: Rid,) -> Array < Rid >;
        #[doc = "Overridable version of [`body_set_max_contacts_reported`][`crate::classes::PhysicsServer2D::body_set_max_contacts_reported`]."]
        fn body_set_max_contacts_reported(&mut self, body: Rid, amount: i32,);
        #[doc = "Overridable version of [`body_get_max_contacts_reported`][`crate::classes::PhysicsServer2D::body_get_max_contacts_reported`]."]
        fn body_get_max_contacts_reported(&self, body: Rid,) -> i32;
        #[doc = "Overridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `body_set_contacts_reported_depth_threshold` method.\n\n**Note:** This method is currently unused by Godot's default physics implementation."]
        fn body_set_contacts_reported_depth_threshold(&mut self, body: Rid, threshold: f32,);
        #[doc = "Overridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `body_get_contacts_reported_depth_threshold` method.\n\n**Note:** This method is currently unused by Godot's default physics implementation."]
        fn body_get_contacts_reported_depth_threshold(&self, body: Rid,) -> f32;
        #[doc = "Overridable version of [`body_set_omit_force_integration`][`crate::classes::PhysicsServer2D::body_set_omit_force_integration`]."]
        fn body_set_omit_force_integration(&mut self, body: Rid, enable: bool,);
        #[doc = "Overridable version of [`body_is_omitting_force_integration`][`crate::classes::PhysicsServer2D::body_is_omitting_force_integration`]."]
        fn body_is_omitting_force_integration(&self, body: Rid,) -> bool;
        #[doc = "Assigns the `body` to call the given `callable` during the synchronization phase of the loop, before [`step`][`crate::classes::IPhysicsServer2DExtension::step`] is called. See also [`sync`][`crate::classes::IPhysicsServer2DExtension::sync`].\n\nOverridable version of [`body_set_state_sync_callback`][`crate::classes::PhysicsServer2D::body_set_state_sync_callback`]."]
        fn body_set_state_sync_callback(&mut self, body: Rid, callable: Callable,);
        #[doc = "Overridable version of [`body_set_force_integration_callback`][`crate::classes::PhysicsServer2D::body_set_force_integration_callback`]."]
        fn body_set_force_integration_callback(&mut self, body: Rid, callable: Callable, userdata: Variant,);
        #[doc = "\n# Godot docs\nGiven a `body`, a `shape`, and their respective parameters, this method should return `true` if a collision between the two would occur, with additional details passed in `results`.\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `shape_collide` method. Corresponds to [`collide_shape`][`crate::classes::PhysicsDirectSpaceState2D::collide_shape`]."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn body_collide_shape_rawptr(&mut self, body: Rid, body_shape: i32, shape: Rid, shape_xform: Transform2D, motion: Vector2, results: crate::meta::RawPtr < * mut c_void >, result_max: i32, result_count: crate::meta::RawPtr < * mut i32 >,) -> bool;
        #[doc = "If set to `true`, allows the body with the given [`RID`][crate::builtin::Rid] to detect mouse inputs when the mouse cursor is hovering on it.\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `body_set_pickable` method. Corresponds to \\[member CollisionObject2D.input_pickable]."]
        fn body_set_pickable(&mut self, body: Rid, pickable: bool,);
        #[doc = "Overridable version of [`body_get_direct_state`][`crate::classes::PhysicsServer2D::body_get_direct_state`]."]
        fn body_get_direct_state(&mut self, body: Rid,) -> Option < Gd < crate::classes::PhysicsDirectBodyState2D > >;
        #[doc = "\n# Godot docs\nOverridable version of [`body_test_motion`][`crate::classes::PhysicsServer2D::body_test_motion`]. Unlike the exposed implementation, this method does not receive all of the arguments inside a [`PhysicsTestMotionParameters2D`][crate::classes::PhysicsTestMotionParameters2D]."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn body_test_motion_rawptr(&self, body: Rid, from: Transform2D, motion: Vector2, margin: f32, collide_separation_ray: bool, recovery_as_collision: bool, result: crate::meta::RawPtr < * mut PhysicsServer2DExtensionMotionResult >,) -> bool;
        #[doc = "Overridable version of [`joint_create`][`crate::classes::PhysicsServer2D::joint_create`]."]
        fn joint_create(&mut self,) -> Rid;
        #[doc = "Overridable version of [`joint_clear`][`crate::classes::PhysicsServer2D::joint_clear`]."]
        fn joint_clear(&mut self, joint: Rid,);
        #[doc = "Overridable version of [`joint_set_param`][`crate::classes::PhysicsServer2D::joint_set_param`]."]
        fn joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::JointParam, value: f32,);
        #[doc = "Overridable version of [`joint_get_param`][`crate::classes::PhysicsServer2D::joint_get_param`]."]
        fn joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::JointParam,) -> f32;
        #[doc = "Overridable version of [`joint_disable_collisions_between_bodies`][`crate::classes::PhysicsServer2D::joint_disable_collisions_between_bodies`]."]
        fn joint_disable_collisions_between_bodies(&mut self, joint: Rid, disable: bool,);
        #[doc = "Overridable version of [`joint_is_disabled_collisions_between_bodies`][`crate::classes::PhysicsServer2D::joint_is_disabled_collisions_between_bodies`]."]
        fn joint_is_disabled_collisions_between_bodies(&self, joint: Rid,) -> bool;
        #[doc = "Overridable version of [`joint_make_pin`][`crate::classes::PhysicsServer2D::joint_make_pin`]."]
        fn joint_make_pin(&mut self, joint: Rid, anchor: Vector2, body_a: Rid, body_b: Rid,);
        #[doc = "Overridable version of [`joint_make_groove`][`crate::classes::PhysicsServer2D::joint_make_groove`]."]
        fn joint_make_groove(&mut self, joint: Rid, a_groove1: Vector2, a_groove2: Vector2, b_anchor: Vector2, body_a: Rid, body_b: Rid,);
        #[doc = "Overridable version of [`joint_make_damped_spring`][`crate::classes::PhysicsServer2D::joint_make_damped_spring`]."]
        fn joint_make_damped_spring(&mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,);
        #[doc = "Overridable version of [`pin_joint_set_flag`][`crate::classes::PhysicsServer2D::pin_joint_set_flag`]."]
        fn pin_joint_set_flag(&mut self, joint: Rid, flag: crate::classes::physics_server_2d::PinJointFlag, enabled: bool,);
        #[doc = "Overridable version of [`pin_joint_get_flag`][`crate::classes::PhysicsServer2D::pin_joint_get_flag`]."]
        fn pin_joint_get_flag(&self, joint: Rid, flag: crate::classes::physics_server_2d::PinJointFlag,) -> bool;
        #[doc = "Overridable version of [`pin_joint_set_param`][`crate::classes::PhysicsServer2D::pin_joint_set_param`]."]
        fn pin_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::PinJointParam, value: f32,);
        #[doc = "Overridable version of [`pin_joint_get_param`][`crate::classes::PhysicsServer2D::pin_joint_get_param`]."]
        fn pin_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::PinJointParam,) -> f32;
        #[doc = "Overridable version of [`damped_spring_joint_set_param`][`crate::classes::PhysicsServer2D::damped_spring_joint_set_param`]."]
        fn damped_spring_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::DampedSpringParam, value: f32,);
        #[doc = "Overridable version of [`damped_spring_joint_get_param`][`crate::classes::PhysicsServer2D::damped_spring_joint_get_param`]."]
        fn damped_spring_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::DampedSpringParam,) -> f32;
        #[doc = "Overridable version of [`joint_get_type`][`crate::classes::PhysicsServer2D::joint_get_type`]."]
        fn joint_get_type(&self, joint: Rid,) -> crate::classes::physics_server_2d::JointType;
        #[doc = "Overridable version of [`free_rid`][`crate::classes::PhysicsServer2D::free_rid`]."]
        fn free_rid(&mut self, rid: Rid,);
        #[doc = "Overridable version of [`set_active`][`crate::classes::PhysicsServer2D::set_active`]."]
        fn set_active(&mut self, active: bool,);
        #[doc = "Called when the main loop is initialized and creates a new instance of this physics server. See also [`initialize`][`crate::classes::IMainLoop::initialize`] and [`finish`][`crate::classes::IPhysicsServer2DExtension::finish`].\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `init` method."]
        fn init_ext(&mut self,);
        #[doc = "Called every physics step to process the physics simulation. `step` is the time elapsed since the last physics step, in seconds. It is usually the same as the value returned by [`get_physics_process_delta_time`][`crate::classes::Node::get_physics_process_delta_time`].\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `step` method."]
        fn step(&mut self, step: f32,);
        #[doc = "Called to indicate that the physics server is synchronizing and cannot access physics states if running on a separate thread. See also [`end_sync`][`crate::classes::IPhysicsServer2DExtension::end_sync`].\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `sync` method."]
        fn sync(&mut self,);
        #[doc = "Called every physics step before [`step`][`crate::classes::IPhysicsServer2DExtension::step`] to process all remaining queries.\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `flush_queries` method."]
        fn flush_queries(&mut self,);
        #[doc = "Called to indicate that the physics server has stopped synchronizing. It is in the loop's iteration/physics phase, and can access physics objects even if running on a separate thread. See also [`sync`][`crate::classes::IPhysicsServer2DExtension::sync`].\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `end_sync` method."]
        fn end_sync(&mut self,);
        #[doc = "Called when the main loop finalizes to shut down the physics server. See also [`finalize`][`crate::classes::IMainLoop::finalize`] and [`init_ext`][`crate::classes::IPhysicsServer2DExtension::init_ext`].\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `finish` method."]
        fn finish(&mut self,);
        #[doc = "Overridable method that should return `true` when the physics server is processing queries. See also [`flush_queries`][`crate::classes::IPhysicsServer2DExtension::flush_queries`].\n\nOverridable version of [`PhysicsServer2D`][crate::classes::PhysicsServer2D]'s internal `is_flushing_queries` method."]
        fn is_flushing_queries(&self,) -> bool;
        #[doc = "Overridable version of [`get_process_info`][`crate::classes::PhysicsServer2D::get_process_info`]."]
        fn get_process_info(&mut self, process_info: crate::classes::physics_server_2d::ProcessInfo,) -> i32;
        
    }
    impl PhysicsServer2DExtension {
        #[doc = "Returns `true` if the body with the given [`RID`][crate::builtin::Rid] is being excluded from [`body_test_motion_rawptr`][`crate::classes::IPhysicsServer2DExtension::body_test_motion_rawptr`]. See also [`instance_id`][`crate::obj::Gd::instance_id`]."]
        pub fn body_test_motion_is_excluding_body(&self, body: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(29usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2DExtension", "body_test_motion_is_excluding_body", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the object with the given instance ID is being excluded from [`body_test_motion_rawptr`][`crate::classes::IPhysicsServer2DExtension::body_test_motion_rawptr`]. See also [`instance_id`][`crate::obj::Gd::instance_id`]."]
        pub fn body_test_motion_is_excluding_object(&self, object: u64,) -> bool {
            type CallRet = bool;
            type CallParams = (u64,);
            let args = (object,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(30usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2DExtension", "body_test_motion_is_excluding_object", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsServer2DExtension {
        type Base = crate::classes::PhysicsServer2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsServer2DExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsServer2DExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsServer2D > for PhysicsServer2DExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsServer2DExtension {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicsServer2DExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicsServer2DExtension {
        type Target = crate::classes::PhysicsServer2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsServer2DExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicsServer2DExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsServer2DExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsServer2DExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsServer2D > for $Class {
                
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
    use super::re_export::PhysicsServer2DExtension;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PhysicsServer2DExtension {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}