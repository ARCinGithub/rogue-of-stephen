#![doc = "Sidecar module for class [`PhysicsServer2D`][crate::classes::PhysicsServer2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsServer2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsserver2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsServer2D`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`physics_server_2d`][crate::classes::physics_server_2d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `PhysicsServer2D`](https://docs.godotengine.org/en/stable/classes/class_physicsserver2d.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nPhysicsServer2D is the server responsible for all 2D physics. It can directly create and manipulate all physics objects:\n\n- A _space_ is a self-contained world for a physics simulation. It contains bodies, areas, and joints. Its state can be queried for collision and intersection information, and several parameters of the simulation can be modified.\n\n- A _shape_ is a geometric shape such as a circle, a rectangle, a capsule, or a polygon. It can be used for collision detection by adding it to a body/area, possibly with an extra transformation relative to the body/area's origin. Bodies/areas can have multiple (transformed) shapes added to them, and a single shape can be added to bodies/areas multiple times with different local transformations.\n\n- A _body_ is a physical object which can be in static, kinematic, or rigid mode. Its state (such as position and velocity) can be queried and updated. A force integration callback can be set to customize the body's physics.\n\n- An _area_ is a region in space which can be used to detect bodies and areas entering and exiting it. A body monitoring callback can be set to report entering/exiting body shapes, and similarly an area monitoring callback can be set. Gravity and damping can be overridden within the area by setting area parameters.\n\n- A _joint_ is a constraint, either between two bodies or on one body relative to a point. Parameters such as the joint bias and the rest length of a spring joint can be adjusted.\n\nPhysics objects in `PhysicsServer2D` may be created and manipulated independently; they do not have to be tied to nodes in the scene tree.\n\n**Note:** All the 2D physics nodes use the physics server internally. Adding a physics node to the scene tree will cause a corresponding physics object to be created in the physics server. A rigid body node registers a callback that updates the node's transform with the transform of the respective body object in the physics server (every physics update). An area node registers a callback to inform the area node about overlaps with the respective area object in the physics server. The raycast node queries the direct state of the relevant space in the physics server."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsServer2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PhysicsServer2D {
        #[doc = "Creates a 2D world boundary shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer2D::shape_set_data`] to set the shape's normal direction and distance properties."]
        pub fn world_boundary_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "world_boundary_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2D separation ray shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer2D::shape_set_data`] to set the shape's `length` and `slide_on_slope` properties."]
        pub fn separation_ray_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "separation_ray_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2D segment shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer2D::shape_set_data`] to set the segment's start and end points."]
        pub fn segment_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "segment_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2D circle shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer2D::shape_set_data`] to set the circle's radius."]
        pub fn circle_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "circle_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2D rectangle shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer2D::shape_set_data`] to set the rectangle's half-extents."]
        pub fn rectangle_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "rectangle_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2D capsule shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer2D::shape_set_data`] to set the capsule's height and radius."]
        pub fn capsule_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "capsule_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2D convex polygon shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer2D::shape_set_data`] to set the convex polygon's points."]
        pub fn convex_polygon_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "convex_polygon_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2D concave polygon shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer2D::shape_set_data`] to set the concave polygon's segments."]
        pub fn concave_polygon_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "concave_polygon_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the shape data that defines the configuration of the shape. The `data` to be passed depends on the shape's type (see [`shape_get_type`][`crate::classes::PhysicsServer2D::shape_get_type`]):\n\n- [`ShapeType::WORLD_BOUNDARY`][`crate::classes::physics_server_2d::ShapeType::WORLD_BOUNDARY`]: an array of length two containing a [`Vector2`][crate::builtin::Vector2] `normal` direction and a `float` distance `d`,\n\n- [`ShapeType::SEPARATION_RAY`][`crate::classes::physics_server_2d::ShapeType::SEPARATION_RAY`]: a dictionary containing the key `length` with a `float` value and the key `slide_on_slope` with a `bool` value,\n\n- [`ShapeType::SEGMENT`][`crate::classes::physics_server_2d::ShapeType::SEGMENT`]: a [`Rect2`][crate::builtin::Rect2] `rect` containing the first point of the segment in `rect.position` and the second point of the segment in `rect.size`,\n\n- [`ShapeType::CIRCLE`][`crate::classes::physics_server_2d::ShapeType::CIRCLE`]: a `float` `radius`,\n\n- [`ShapeType::RECTANGLE`][`crate::classes::physics_server_2d::ShapeType::RECTANGLE`]: a [`Vector2`][crate::builtin::Vector2] `half_extents`,\n\n- [`ShapeType::CAPSULE`][`crate::classes::physics_server_2d::ShapeType::CAPSULE`]: an array of length two (or a [`Vector2`][crate::builtin::Vector2]) containing a `float` `height` and a `float` `radius`,\n\n- [`ShapeType::CONVEX_POLYGON`][`crate::classes::physics_server_2d::ShapeType::CONVEX_POLYGON`]: either a [`PackedVector2Array`][crate::builtin::PackedVector2Array] of points defining a convex polygon in counterclockwise order (the clockwise outward normal of each segment formed by consecutive points is calculated internally), or a [`PackedFloat32Array`][crate::builtin::PackedFloat32Array] of length divisible by four so that every 4-tuple of `float`s contains the coordinates of a point followed by the coordinates of the clockwise outward normal vector to the segment between the current point and the next point,\n\n- [`ShapeType::CONCAVE_POLYGON`][`crate::classes::physics_server_2d::ShapeType::CONCAVE_POLYGON`]: a [`PackedVector2Array`][crate::builtin::PackedVector2Array] of length divisible by two (each pair of points forms one segment).\n\n**Warning:** In the case of [`ShapeType::CONVEX_POLYGON`][`crate::classes::physics_server_2d::ShapeType::CONVEX_POLYGON`], this method does not check if the points supplied actually form a convex polygon (unlike the \\[member CollisionPolygon2D.polygon] property)."]
        pub fn shape_set_data(&mut self, shape: Rid, data: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (shape, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "shape_set_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the shape's type."]
        pub fn shape_get_type(&self, shape: Rid,) -> crate::classes::physics_server_2d::ShapeType {
            type CallRet = crate::classes::physics_server_2d::ShapeType;
            type CallParams = (Rid,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "shape_get_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the shape data that defines the configuration of the shape, such as the half-extents of a rectangle or the segments of a concave shape. See [`shape_set_data`][`crate::classes::PhysicsServer2D::shape_set_data`] for the precise format of this data in each case."]
        pub fn shape_get_data(&self, shape: Rid,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "shape_get_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2D space in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. A space contains bodies and areas, and controls the stepping of the physics simulation of the objects in it."]
        pub fn space_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Activates or deactivates the space. If `active` is `false`, then the physics server will not do anything with this space in its physics step."]
        pub fn space_set_active(&mut self, space: Rid, active: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (space, active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_set_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the space is active."]
        pub fn space_is_active(&self, space: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_is_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of the given space parameter."]
        pub fn space_set_param(&mut self, space: Rid, param: crate::classes::physics_server_2d::SpaceParameter, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_2d::SpaceParameter, f32,);
            let args = (space, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the given space parameter."]
        pub fn space_get_param(&self, space: Rid, param: crate::classes::physics_server_2d::SpaceParameter,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, crate::classes::physics_server_2d::SpaceParameter,);
            let args = (space, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the state of a space, a [`PhysicsDirectSpaceState2D`][crate::classes::PhysicsDirectSpaceState2D]. This object can be used for collision/intersection queries."]
        pub fn space_get_direct_state(&mut self, space: Rid,) -> Option < Gd < crate::classes::PhysicsDirectSpaceState2D > > {
            type CallRet = Option < Gd < crate::classes::PhysicsDirectSpaceState2D > >;
            type CallParams = (Rid,);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "space_get_direct_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2D area object in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. The default settings for the created area include a collision layer and mask set to `1`, and `monitorable` set to `false`.\n\nUse [`area_add_shape`][`crate::classes::PhysicsServer2D::area_add_shape`] to add shapes to it, use [`area_set_transform`][`crate::classes::PhysicsServer2D::area_set_transform`] to set its transform, and use [`area_set_space`][`crate::classes::PhysicsServer2D::area_set_space`] to add the area to a space. If you want the area to be detectable use [`area_set_monitorable`][`crate::classes::PhysicsServer2D::area_set_monitorable`]."]
        pub fn area_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds the area to the given space, after removing the area from the previously assigned space (if any).\n\n**Note:** To remove an area from a space without immediately adding it back elsewhere, use `PhysicsServer2D.area_set_space(area, RID())`."]
        pub fn area_set_space(&mut self, area: Rid, space: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (area, space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the space assigned to the area. Returns an empty [`RID`][crate::builtin::Rid] if no space is assigned."]
        pub fn area_get_space(&self, area: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a shape to the area, with the given local transform. The shape (together with its `transform` and `disabled` properties) is added to an array of shapes, and the shapes of an area are usually referenced by their index in this array."]
        pub(crate) fn area_add_shape_full(&mut self, area: Rid, shape: Rid, transform: Transform2D, disabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform2D, bool,);
            let args = (area, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_add_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`area_add_shape_ex`][Self::area_add_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a shape to the area, with the given local transform. The shape (together with its `transform` and `disabled` properties) is added to an array of shapes, and the shapes of an area are usually referenced by their index in this array."]
        #[inline]
        pub fn area_add_shape(&mut self, area: Rid, shape: Rid,) {
            self.area_add_shape_ex(area, shape,) . done()
        }
        #[doc = "Adds a shape to the area, with the given local transform. The shape (together with its `transform` and `disabled` properties) is added to an array of shapes, and the shapes of an area are usually referenced by their index in this array."]
        #[inline]
        pub fn area_add_shape_ex < 'ex > (&'ex mut self, area: Rid, shape: Rid,) -> ExAreaAddShape < 'ex > {
            ExAreaAddShape::new(self, area, shape,)
        }
        #[doc = "Replaces the area's shape at the given index by another shape, while not affecting the `transform` and `disabled` properties at the same index."]
        pub fn area_set_shape(&mut self, area: Rid, shape_idx: i32, shape: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (area, shape_idx, shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(808usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the local transform matrix of the area's shape with the given index."]
        pub fn area_set_shape_transform(&mut self, area: Rid, shape_idx: i32, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform2D,);
            let args = (area, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_shape_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the disabled property of the area's shape with the given index. If `disabled` is `true`, then the shape will not detect any other shapes entering or exiting it."]
        pub fn area_set_shape_disabled(&mut self, area: Rid, shape_idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (area, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_shape_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of shapes added to the area."]
        pub fn area_get_shape_count(&self, area: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_shape_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the shape with the given index in the area's array of shapes."]
        pub fn area_get_shape(&self, area: Rid, shape_idx: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i32,);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the local transform matrix of the shape with the given index in the area's array of shapes."]
        pub fn area_get_shape_transform(&self, area: Rid, shape_idx: i32,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (Rid, i32,);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_shape_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the shape with the given index from the area's array of shapes. The shape itself is not deleted, so it can continue to be used elsewhere or added back later. As a result of this operation, the area's shapes which used to have indices higher than `shape_idx` will have their index decreased by one."]
        pub fn area_remove_shape(&mut self, area: Rid, shape_idx: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_remove_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all shapes from the area. This does not delete the shapes themselves, so they can continue to be used elsewhere or added back later."]
        pub fn area_clear_shapes(&mut self, area: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_clear_shapes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns the area to one or many physics layers, via a bitmask."]
        pub fn area_set_collision_layer(&mut self, area: Rid, layer: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (area, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics layer or layers the area belongs to, as a bitmask."]
        pub fn area_get_collision_layer(&self, area: Rid,) -> u32 {
            type CallRet = u32;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets which physics layers the area will monitor, via a bitmask."]
        pub fn area_set_collision_mask(&mut self, area: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (area, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics layer or layers the area can contact with, as a bitmask."]
        pub fn area_get_collision_mask(&self, area: Rid,) -> u32 {
            type CallRet = u32;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of the given area parameter."]
        pub fn area_set_param(&mut self, area: Rid, param: crate::classes::physics_server_2d::AreaParameter, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, crate::classes::physics_server_2d::AreaParameter, RefArg < 'a0, Variant >,);
            let args = (area, param, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the transform matrix of the area."]
        pub fn area_set_transform(&mut self, area: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (area, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the given area parameter."]
        pub fn area_get_param(&self, area: Rid, param: crate::classes::physics_server_2d::AreaParameter,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, crate::classes::physics_server_2d::AreaParameter,);
            let args = (area, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform matrix of the area."]
        pub fn area_get_transform(&self, area: Rid,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Attaches the `ObjectID` of an [`Object`][crate::classes::Object] to the area. Use [`instance_id`][`crate::obj::Gd::instance_id`] to get the `ObjectID` of a [`CollisionObject2D`][crate::classes::CollisionObject2D]."]
        pub fn area_attach_object_instance_id(&mut self, area: Rid, id: u64,) {
            type CallRet = ();
            type CallParams = (Rid, u64,);
            let args = (area, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_attach_object_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the `ObjectID` attached to the area. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to retrieve an [`Object`][crate::classes::Object] from a nonzero `ObjectID`."]
        pub fn area_get_object_instance_id(&self, area: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_object_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Attaches the `ObjectID` of a canvas to the area. Use [`instance_id`][`crate::obj::Gd::instance_id`] to get the `ObjectID` of a [`CanvasLayer`][crate::classes::CanvasLayer]."]
        pub fn area_attach_canvas_instance_id(&mut self, area: Rid, id: u64,) {
            type CallRet = ();
            type CallParams = (Rid, u64,);
            let args = (area, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_attach_canvas_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the `ObjectID` of the canvas attached to the area. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to retrieve a [`CanvasLayer`][crate::classes::CanvasLayer] from a nonzero `ObjectID`."]
        pub fn area_get_canvas_instance_id(&self, area: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_get_canvas_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the area's body monitor callback. This callback will be called when any other (shape of a) body enters or exits (a shape of) the given area, and must take the following five parameters:\n\n1. an integer `status`: either [`AreaBodyStatus::ADDED`][`crate::classes::physics_server_2d::AreaBodyStatus::ADDED`] or [`AreaBodyStatus::REMOVED`][`crate::classes::physics_server_2d::AreaBodyStatus::REMOVED`] depending on whether the other body shape entered or exited the area,\n\n2. an [`RID`][crate::builtin::Rid] `body_rid`: the [`RID`][crate::builtin::Rid] of the body that entered or exited the area,\n\n3. an integer `instance_id`: the `ObjectID` attached to the body,\n\n4. an integer `body_shape_idx`: the index of the shape of the body that entered or exited the area,\n\n5. an integer `self_shape_idx`: the index of the shape of the area where the body entered or exited.\n\nBy counting (or keeping track of) the shapes that enter and exit, it can be determined if a body (with all its shapes) is entering for the first time or exiting for the last time."]
        pub fn area_set_monitor_callback(&mut self, area: Rid, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Callable >,);
            let args = (area, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_monitor_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the area's area monitor callback. This callback will be called when any other (shape of an) area enters or exits (a shape of) the given area, and must take the following five parameters:\n\n1. an integer `status`: either [`AreaBodyStatus::ADDED`][`crate::classes::physics_server_2d::AreaBodyStatus::ADDED`] or [`AreaBodyStatus::REMOVED`][`crate::classes::physics_server_2d::AreaBodyStatus::REMOVED`] depending on whether the other area's shape entered or exited the area,\n\n2. an [`RID`][crate::builtin::Rid] `area_rid`: the [`RID`][crate::builtin::Rid] of the other area that entered or exited the area,\n\n3. an integer `instance_id`: the `ObjectID` attached to the other area,\n\n4. an integer `area_shape_idx`: the index of the shape of the other area that entered or exited the area,\n\n5. an integer `self_shape_idx`: the index of the shape of the area where the other area entered or exited.\n\nBy counting (or keeping track of) the shapes that enter and exit, it can be determined if an area (with all its shapes) is entering for the first time or exiting for the last time."]
        pub fn area_set_area_monitor_callback(&mut self, area: Rid, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Callable >,);
            let args = (area, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_area_monitor_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the area is monitorable or not. If `monitorable` is `true`, the area monitoring callback of other areas will be called when this area enters or exits them."]
        pub fn area_set_monitorable(&mut self, area: Rid, monitorable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (area, monitorable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "area_set_monitorable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2D body object in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. The default settings for the created area include a collision layer and mask set to `1`, and body mode set to [`BodyMode::RIGID`][`crate::classes::physics_server_2d::BodyMode::RIGID`].\n\nUse [`body_add_shape`][`crate::classes::PhysicsServer2D::body_add_shape`] to add shapes to it, use [`body_set_state`][`crate::classes::PhysicsServer2D::body_set_state`] to set its transform, and use [`body_set_space`][`crate::classes::PhysicsServer2D::body_set_space`] to add the body to a space."]
        pub fn body_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds the body to the given space, after removing the body from the previously assigned space (if any). If the body's mode is set to [`BodyMode::RIGID`][`crate::classes::physics_server_2d::BodyMode::RIGID`], then adding the body to a space will have the following additional effects:\n\n- If the parameter [`BodyParameter::CENTER_OF_MASS`][`crate::classes::physics_server_2d::BodyParameter::CENTER_OF_MASS`] has never been set explicitly, then the value of that parameter will be recalculated based on the body's shapes.\n\n- If the parameter [`BodyParameter::INERTIA`][`crate::classes::physics_server_2d::BodyParameter::INERTIA`] is set to a value `<= 0.0`, then the value of that parameter will be recalculated based on the body's shapes, mass, and center of mass.\n\n**Note:** To remove a body from a space without immediately adding it back elsewhere, use `PhysicsServer2D.body_set_space(body, RID())`."]
        pub fn body_set_space(&mut self, body: Rid, space: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (body, space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the space assigned to the body. Returns an empty [`RID`][crate::builtin::Rid] if no space is assigned."]
        pub fn body_get_space(&self, body: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's mode."]
        pub fn body_set_mode(&mut self, body: Rid, mode: crate::classes::physics_server_2d::BodyMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_2d::BodyMode,);
            let args = (body, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body's mode."]
        pub fn body_get_mode(&self, body: Rid,) -> crate::classes::physics_server_2d::BodyMode {
            type CallRet = crate::classes::physics_server_2d::BodyMode;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a shape to the area, with the given local transform. The shape (together with its `transform` and `disabled` properties) is added to an array of shapes, and the shapes of a body are usually referenced by their index in this array."]
        pub(crate) fn body_add_shape_full(&mut self, body: Rid, shape: Rid, transform: Transform2D, disabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform2D, bool,);
            let args = (body, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_add_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_add_shape_ex`][Self::body_add_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a shape to the area, with the given local transform. The shape (together with its `transform` and `disabled` properties) is added to an array of shapes, and the shapes of a body are usually referenced by their index in this array."]
        #[inline]
        pub fn body_add_shape(&mut self, body: Rid, shape: Rid,) {
            self.body_add_shape_ex(body, shape,) . done()
        }
        #[doc = "Adds a shape to the area, with the given local transform. The shape (together with its `transform` and `disabled` properties) is added to an array of shapes, and the shapes of a body are usually referenced by their index in this array."]
        #[inline]
        pub fn body_add_shape_ex < 'ex > (&'ex mut self, body: Rid, shape: Rid,) -> ExBodyAddShape < 'ex > {
            ExBodyAddShape::new(self, body, shape,)
        }
        #[doc = "Replaces the body's shape at the given index by another shape, while not affecting the `transform`, `disabled`, and one-way collision properties at the same index."]
        pub fn body_set_shape(&mut self, body: Rid, shape_idx: i32, shape: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (body, shape_idx, shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the local transform matrix of the body's shape with the given index."]
        pub fn body_set_shape_transform(&mut self, body: Rid, shape_idx: i32, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform2D,);
            let args = (body, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_shape_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of shapes added to the body."]
        pub fn body_get_shape_count(&self, body: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_shape_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the shape with the given index in the body's array of shapes."]
        pub fn body_get_shape(&self, body: Rid, shape_idx: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i32,);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the local transform matrix of the shape with the given index in the area's array of shapes."]
        pub fn body_get_shape_transform(&self, body: Rid, shape_idx: i32,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (Rid, i32,);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_shape_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the shape with the given index from the body's array of shapes. The shape itself is not deleted, so it can continue to be used elsewhere or added back later. As a result of this operation, the body's shapes which used to have indices higher than `shape_idx` will have their index decreased by one."]
        pub fn body_remove_shape(&mut self, body: Rid, shape_idx: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_remove_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all shapes from the body. This does not delete the shapes themselves, so they can continue to be used elsewhere or added back later."]
        pub fn body_clear_shapes(&mut self, body: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_clear_shapes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the disabled property of the body's shape with the given index. If `disabled` is `true`, then the shape will be ignored in all collision detection."]
        pub fn body_set_shape_disabled(&mut self, body: Rid, shape_idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (body, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_shape_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the one-way collision properties of the body's shape with the given index. If `enable` is `true`, the one-way collision direction given by the shape's local upward axis `body_get_shape_transform(body, shape_idx).y` will be used to ignore collisions with the shape in the opposite direction, and to ensure depenetration of kinematic bodies happens in this direction."]
        pub fn body_set_shape_as_one_way_collision(&mut self, body: Rid, shape_idx: i32, enable: bool, margin: f32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool, f32,);
            let args = (body, shape_idx, enable, margin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_shape_as_one_way_collision", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Attaches the `ObjectID` of an [`Object`][crate::classes::Object] to the body. Use [`instance_id`][`crate::obj::Gd::instance_id`] to get the `ObjectID` of a [`CollisionObject2D`][crate::classes::CollisionObject2D]."]
        pub fn body_attach_object_instance_id(&mut self, body: Rid, id: u64,) {
            type CallRet = ();
            type CallParams = (Rid, u64,);
            let args = (body, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_attach_object_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the `ObjectID` attached to the body. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to retrieve an [`Object`][crate::classes::Object] from a nonzero `ObjectID`."]
        pub fn body_get_object_instance_id(&self, body: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_object_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Attaches the `ObjectID` of a canvas to the body. Use [`instance_id`][`crate::obj::Gd::instance_id`] to get the `ObjectID` of a [`CanvasLayer`][crate::classes::CanvasLayer]."]
        pub fn body_attach_canvas_instance_id(&mut self, body: Rid, id: u64,) {
            type CallRet = ();
            type CallParams = (Rid, u64,);
            let args = (body, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_attach_canvas_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the `ObjectID` of the canvas attached to the body. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to retrieve a [`CanvasLayer`][crate::classes::CanvasLayer] from a nonzero `ObjectID`."]
        pub fn body_get_canvas_instance_id(&self, body: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_canvas_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the continuous collision detection mode.\n\nContinuous collision detection tries to predict where a moving body would collide in between physics updates, instead of moving it and correcting its movement if it collided."]
        pub fn body_set_continuous_collision_detection_mode(&mut self, body: Rid, mode: crate::classes::physics_server_2d::CcdMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_2d::CcdMode,);
            let args = (body, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_continuous_collision_detection_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body's continuous collision detection mode."]
        pub fn body_get_continuous_collision_detection_mode(&self, body: Rid,) -> crate::classes::physics_server_2d::CcdMode {
            type CallRet = crate::classes::physics_server_2d::CcdMode;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_continuous_collision_detection_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the physics layer or layers the body belongs to, via a bitmask."]
        pub fn body_set_collision_layer(&mut self, body: Rid, layer: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (body, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics layer or layers the body belongs to, as a bitmask."]
        pub fn body_get_collision_layer(&self, body: Rid,) -> u32 {
            type CallRet = u32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the physics layer or layers the body can collide with, via a bitmask."]
        pub fn body_set_collision_mask(&mut self, body: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (body, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics layer or layers the body can collide with, as a bitmask."]
        pub fn body_get_collision_mask(&self, body: Rid,) -> u32 {
            type CallRet = u32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's collision priority. This is used in the depenetration phase of [`body_test_motion`][`crate::classes::PhysicsServer2D::body_test_motion`]. The higher the priority is, the lower the penetration into the body will be."]
        pub fn body_set_collision_priority(&mut self, body: Rid, priority: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, priority,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body's collision priority. This is used in the depenetration phase of [`body_test_motion`][`crate::classes::PhysicsServer2D::body_test_motion`]. The higher the priority is, the lower the penetration into the body will be."]
        pub fn body_get_collision_priority(&self, body: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of the given body parameter."]
        pub fn body_set_param(&mut self, body: Rid, param: crate::classes::physics_server_2d::BodyParameter, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, crate::classes::physics_server_2d::BodyParameter, RefArg < 'a0, Variant >,);
            let args = (body, param, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the given body parameter."]
        pub fn body_get_param(&self, body: Rid, param: crate::classes::physics_server_2d::BodyParameter,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, crate::classes::physics_server_2d::BodyParameter,);
            let args = (body, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Restores the default inertia and center of mass of the body based on its shapes. This undoes any custom values previously set using [`body_set_param`][`crate::classes::PhysicsServer2D::body_set_param`]."]
        pub fn body_reset_mass_properties(&mut self, body: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_reset_mass_properties", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of a body's state.\n\n**Note:** The state change doesn't take effect immediately. The state will change on the next physics frame."]
        pub fn body_set_state(&mut self, body: Rid, state: crate::classes::physics_server_2d::BodyState, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, crate::classes::physics_server_2d::BodyState, RefArg < 'a0, Variant >,);
            let args = (body, state, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the given state of the body."]
        pub fn body_get_state(&self, body: Rid, state: crate::classes::physics_server_2d::BodyState,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, crate::classes::physics_server_2d::BodyState,);
            let args = (body, state,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a directional impulse to the body, at the body's center of mass. The impulse does not affect rotation.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\nThis is equivalent to using [`body_apply_impulse`][`crate::classes::PhysicsServer2D::body_apply_impulse`] at the body's center of mass."]
        pub fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2,);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_central_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a rotational impulse to the body. The impulse does not affect position.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise)."]
        pub fn body_apply_torque_impulse(&mut self, body: Rid, impulse: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_torque_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a positioned impulse to the body. The impulse can affect rotation if `position` is different from the body's center of mass.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn body_apply_impulse_full(&mut self, body: Rid, impulse: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Vector2,);
            let args = (body, impulse, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_apply_impulse_ex`][Self::body_apply_impulse_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a positioned impulse to the body. The impulse can affect rotation if `position` is different from the body's center of mass.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_apply_impulse(&mut self, body: Rid, impulse: Vector2,) {
            self.body_apply_impulse_ex(body, impulse,) . done()
        }
        #[doc = "Applies a positioned impulse to the body. The impulse can affect rotation if `position` is different from the body's center of mass.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_apply_impulse_ex < 'ex > (&'ex mut self, body: Rid, impulse: Vector2,) -> ExBodyApplyImpulse < 'ex > {
            ExBodyApplyImpulse::new(self, body, impulse,)
        }
        #[doc = "Applies a directional force to the body, at the body's center of mass. The force does not affect rotation. A force is time dependent and meant to be applied every physics update.\n\nThis is equivalent to using [`body_apply_force`][`crate::classes::PhysicsServer2D::body_apply_force`] at the body's center of mass."]
        pub fn body_apply_central_force(&mut self, body: Rid, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2,);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_central_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a positioned force to the body. The force can affect rotation if `position` is different from the body's center of mass. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn body_apply_force_full(&mut self, body: Rid, force: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Vector2,);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_apply_force_ex`][Self::body_apply_force_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a positioned force to the body. The force can affect rotation if `position` is different from the body's center of mass. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_apply_force(&mut self, body: Rid, force: Vector2,) {
            self.body_apply_force_ex(body, force,) . done()
        }
        #[doc = "Applies a positioned force to the body. The force can affect rotation if `position` is different from the body's center of mass. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_apply_force_ex < 'ex > (&'ex mut self, body: Rid, force: Vector2,) -> ExBodyApplyForce < 'ex > {
            ExBodyApplyForce::new(self, body, force,)
        }
        #[doc = "Applies a rotational force to the body. The force does not affect position. A force is time dependent and meant to be applied every physics update."]
        pub fn body_apply_torque(&mut self, body: Rid, torque: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_apply_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a constant directional force to the body. The force does not affect rotation. The force remains applied over time until cleared with `PhysicsServer2D.body_set_constant_force(body, Vector2(0, 0))`.\n\nThis is equivalent to using [`body_add_constant_force`][`crate::classes::PhysicsServer2D::body_add_constant_force`] at the body's center of mass."]
        pub fn body_add_constant_central_force(&mut self, body: Rid, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2,);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_add_constant_central_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a constant positioned force to the body. The force can affect rotation if `position` is different from the body's center of mass. The force remains applied over time until cleared with `PhysicsServer2D.body_set_constant_force(body, Vector2(0, 0))`.\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn body_add_constant_force_full(&mut self, body: Rid, force: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Vector2,);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_add_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_add_constant_force_ex`][Self::body_add_constant_force_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a constant positioned force to the body. The force can affect rotation if `position` is different from the body's center of mass. The force remains applied over time until cleared with `PhysicsServer2D.body_set_constant_force(body, Vector2(0, 0))`.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_add_constant_force(&mut self, body: Rid, force: Vector2,) {
            self.body_add_constant_force_ex(body, force,) . done()
        }
        #[doc = "Adds a constant positioned force to the body. The force can affect rotation if `position` is different from the body's center of mass. The force remains applied over time until cleared with `PhysicsServer2D.body_set_constant_force(body, Vector2(0, 0))`.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_add_constant_force_ex < 'ex > (&'ex mut self, body: Rid, force: Vector2,) -> ExBodyAddConstantForce < 'ex > {
            ExBodyAddConstantForce::new(self, body, force,)
        }
        #[doc = "Adds a constant rotational force to the body. The force does not affect position. The force remains applied over time until cleared with `PhysicsServer2D.body_set_constant_torque(body, 0)`."]
        pub fn body_add_constant_torque(&mut self, body: Rid, torque: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_add_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's total constant positional force applied during each physics update.\n\nSee [`body_add_constant_force`][`crate::classes::PhysicsServer2D::body_add_constant_force`] and [`body_add_constant_central_force`][`crate::classes::PhysicsServer2D::body_add_constant_central_force`]."]
        pub fn body_set_constant_force(&mut self, body: Rid, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2,);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body's total constant positional force applied during each physics update.\n\nSee [`body_add_constant_force`][`crate::classes::PhysicsServer2D::body_add_constant_force`] and [`body_add_constant_central_force`][`crate::classes::PhysicsServer2D::body_add_constant_central_force`]."]
        pub fn body_get_constant_force(&self, body: Rid,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's total constant rotational force applied during each physics update.\n\nSee [`body_add_constant_torque`][`crate::classes::PhysicsServer2D::body_add_constant_torque`]."]
        pub fn body_set_constant_torque(&mut self, body: Rid, torque: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body's total constant rotational force applied during each physics update.\n\nSee [`body_add_constant_torque`][`crate::classes::PhysicsServer2D::body_add_constant_torque`]."]
        pub fn body_get_constant_torque(&self, body: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Modifies the body's linear velocity so that its projection to the axis `axis_velocity.normalized()` is exactly `axis_velocity.length()`. This is useful for jumping behavior."]
        pub fn body_set_axis_velocity(&mut self, body: Rid, axis_velocity: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2,);
            let args = (body, axis_velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_axis_velocity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds `excepted_body` to the body's list of collision exceptions, so that collisions with it are ignored."]
        pub fn body_add_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_add_collision_exception", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes `excepted_body` from the body's list of collision exceptions, so that collisions with it are no longer ignored."]
        pub fn body_remove_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_remove_collision_exception", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the maximum number of contacts that the body can report. If `amount` is greater than zero, then the body will keep track of at most this many contacts with other bodies."]
        pub fn body_set_max_contacts_reported(&mut self, body: Rid, amount: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (body, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_max_contacts_reported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the maximum number of contacts that the body can report. See [`body_set_max_contacts_reported`][`crate::classes::PhysicsServer2D::body_set_max_contacts_reported`]."]
        pub fn body_get_max_contacts_reported(&self, body: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_max_contacts_reported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the body omits the standard force integration. If `enable` is `true`, the body will not automatically use applied forces, torques, and damping to update the body's linear and angular velocity. In this case, [`body_set_force_integration_callback`][`crate::classes::PhysicsServer2D::body_set_force_integration_callback`] can be used to manually update the linear and angular velocity instead.\n\nThis method is called when the property \\[member RigidBody2D.custom_integrator] is set."]
        pub fn body_set_omit_force_integration(&mut self, body: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_omit_force_integration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the body is omitting the standard force integration. See [`body_set_omit_force_integration`][`crate::classes::PhysicsServer2D::body_set_omit_force_integration`]."]
        pub fn body_is_omitting_force_integration(&self, body: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_is_omitting_force_integration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's state synchronization callback function to `callable`. Use an empty [`Callable`][crate::builtin::Callable] (`Callable()`) to clear the callback.\n\nThe function `callable` will be called every physics frame, assuming that the body was active during the previous physics tick, and can be used to fetch the latest state from the physics server.\n\nThe function `callable` must take the following parameters:\n\n1. `state`: a [`PhysicsDirectBodyState2D`][crate::classes::PhysicsDirectBodyState2D], used to retrieve the body's state."]
        pub fn body_set_state_sync_callback(&mut self, body: Rid, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Callable >,);
            let args = (body, RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_state_sync_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's custom force integration callback function to `callable`. Use an empty [`Callable`][crate::builtin::Callable] (`Callable()`) to clear the custom callback.\n\nThe function `callable` will be called every physics tick, before the standard force integration (see [`body_set_omit_force_integration`][`crate::classes::PhysicsServer2D::body_set_omit_force_integration`]). It can be used for example to update the body's linear and angular velocity based on contact with other bodies.\n\nIf `userdata` is not `null`, the function `callable` must take the following two parameters:\n\n1. `state`: a [`PhysicsDirectBodyState2D`][crate::classes::PhysicsDirectBodyState2D] used to retrieve and modify the body's state,\n\n2. `userdata`: a [`Variant`][crate::builtin::Variant]; its value will be the `userdata` passed into this method.\n\nIf `userdata` is `null`, then `callable` must take only the `state` parameter."]
        pub(crate) fn body_set_force_integration_callback_full(&mut self, body: Rid, callable: RefArg < Callable >, userdata: RefArg < Variant >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, Callable >, RefArg < 'a1, Variant >,);
            let args = (body, callable, userdata,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_set_force_integration_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_set_force_integration_callback_ex`][Self::body_set_force_integration_callback_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the body's custom force integration callback function to `callable`. Use an empty [`Callable`][crate::builtin::Callable] (`Callable()`) to clear the custom callback.\n\nThe function `callable` will be called every physics tick, before the standard force integration (see [`body_set_omit_force_integration`][`crate::classes::PhysicsServer2D::body_set_omit_force_integration`]). It can be used for example to update the body's linear and angular velocity based on contact with other bodies.\n\nIf `userdata` is not `null`, the function `callable` must take the following two parameters:\n\n1. `state`: a [`PhysicsDirectBodyState2D`][crate::classes::PhysicsDirectBodyState2D] used to retrieve and modify the body's state,\n\n2. `userdata`: a [`Variant`][crate::builtin::Variant]; its value will be the `userdata` passed into this method.\n\nIf `userdata` is `null`, then `callable` must take only the `state` parameter."]
        #[inline]
        pub fn body_set_force_integration_callback(&mut self, body: Rid, callable: &Callable,) {
            self.body_set_force_integration_callback_ex(body, callable,) . done()
        }
        #[doc = "Sets the body's custom force integration callback function to `callable`. Use an empty [`Callable`][crate::builtin::Callable] (`Callable()`) to clear the custom callback.\n\nThe function `callable` will be called every physics tick, before the standard force integration (see [`body_set_omit_force_integration`][`crate::classes::PhysicsServer2D::body_set_omit_force_integration`]). It can be used for example to update the body's linear and angular velocity based on contact with other bodies.\n\nIf `userdata` is not `null`, the function `callable` must take the following two parameters:\n\n1. `state`: a [`PhysicsDirectBodyState2D`][crate::classes::PhysicsDirectBodyState2D] used to retrieve and modify the body's state,\n\n2. `userdata`: a [`Variant`][crate::builtin::Variant]; its value will be the `userdata` passed into this method.\n\nIf `userdata` is `null`, then `callable` must take only the `state` parameter."]
        #[inline]
        pub fn body_set_force_integration_callback_ex < 'ex > (&'ex mut self, body: Rid, callable: &'ex Callable,) -> ExBodySetForceIntegrationCallback < 'ex > {
            ExBodySetForceIntegrationCallback::new(self, body, callable,)
        }
        #[doc = "Returns `true` if a collision would result from moving the body along a motion vector from a given point in space. See [`PhysicsTestMotionParameters2D`][crate::classes::PhysicsTestMotionParameters2D] for the available motion parameters. Optionally a [`PhysicsTestMotionResult2D`][crate::classes::PhysicsTestMotionResult2D] object can be passed, which will be used to store the information about the resulting collision."]
        pub(crate) fn body_test_motion_full(&mut self, body: Rid, parameters: CowArg < Gd < crate::classes::PhysicsTestMotionParameters2D > >, result: CowArg < Option < Gd < crate::classes::PhysicsTestMotionResult2D > > >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (Rid, CowArg < 'a0, Gd < crate::classes::PhysicsTestMotionParameters2D > >, CowArg < 'a1, Option < Gd < crate::classes::PhysicsTestMotionResult2D > > >,);
            let args = (body, parameters, result,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_test_motion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_test_motion_ex`][Self::body_test_motion_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if a collision would result from moving the body along a motion vector from a given point in space. See [`PhysicsTestMotionParameters2D`][crate::classes::PhysicsTestMotionParameters2D] for the available motion parameters. Optionally a [`PhysicsTestMotionResult2D`][crate::classes::PhysicsTestMotionResult2D] object can be passed, which will be used to store the information about the resulting collision."]
        #[inline]
        pub fn body_test_motion(&mut self, body: Rid, parameters: impl AsArg < Gd < crate::classes::PhysicsTestMotionParameters2D >>,) -> bool {
            self.body_test_motion_ex(body, parameters,) . done()
        }
        #[doc = "Returns `true` if a collision would result from moving the body along a motion vector from a given point in space. See [`PhysicsTestMotionParameters2D`][crate::classes::PhysicsTestMotionParameters2D] for the available motion parameters. Optionally a [`PhysicsTestMotionResult2D`][crate::classes::PhysicsTestMotionResult2D] object can be passed, which will be used to store the information about the resulting collision."]
        #[inline]
        pub fn body_test_motion_ex < 'ex > (&'ex mut self, body: Rid, parameters: impl AsArg < Gd < crate::classes::PhysicsTestMotionParameters2D >> + 'ex,) -> ExBodyTestMotion < 'ex > {
            ExBodyTestMotion::new(self, body, parameters,)
        }
        #[doc = "Returns the [`PhysicsDirectBodyState2D`][crate::classes::PhysicsDirectBodyState2D] of the body. Returns `null` if the body is destroyed or not assigned to a space."]
        pub fn body_get_direct_state(&mut self, body: Rid,) -> Option < Gd < crate::classes::PhysicsDirectBodyState2D > > {
            type CallRet = Option < Gd < crate::classes::PhysicsDirectBodyState2D > >;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "body_get_direct_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2D joint in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. To set the joint type, use [`joint_make_damped_spring`][`crate::classes::PhysicsServer2D::joint_make_damped_spring`], [`joint_make_groove`][`crate::classes::PhysicsServer2D::joint_make_groove`] or [`joint_make_pin`][`crate::classes::PhysicsServer2D::joint_make_pin`]. Use [`joint_set_param`][`crate::classes::PhysicsServer2D::joint_set_param`] to set generic joint parameters."]
        pub fn joint_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Destroys the joint with the given [`RID`][crate::builtin::Rid], creates a new uninitialized joint, and makes the [`RID`][crate::builtin::Rid] refer to this new joint."]
        pub fn joint_clear(&mut self, joint: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of the given joint parameter."]
        pub fn joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::JointParam, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_2d::JointParam, f32,);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the given joint parameter."]
        pub fn joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::JointParam,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, crate::classes::physics_server_2d::JointParam,);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the bodies attached to the [`Joint2D`][crate::classes::Joint2D] will collide with each other."]
        pub fn joint_disable_collisions_between_bodies(&mut self, joint: Rid, disable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (joint, disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_disable_collisions_between_bodies", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the bodies attached to the [`Joint2D`][crate::classes::Joint2D] will collide with each other."]
        pub fn joint_is_disabled_collisions_between_bodies(&self, joint: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_is_disabled_collisions_between_bodies", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Makes the joint a pin joint. If `body_b` is an empty [`RID`][crate::builtin::Rid], then `body_a` is pinned to the point `anchor` (given in global coordinates); otherwise, `body_a` is pinned to `body_b` at the point `anchor` (given in global coordinates). To set the parameters which are specific to the pin joint, see [`pin_joint_set_param`][`crate::classes::PhysicsServer2D::pin_joint_set_param`]."]
        pub(crate) fn joint_make_pin_full(&mut self, joint: Rid, anchor: Vector2, body_a: Rid, body_b: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Rid, Rid,);
            let args = (joint, anchor, body_a, body_b,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_make_pin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`joint_make_pin_ex`][Self::joint_make_pin_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Makes the joint a pin joint. If `body_b` is an empty [`RID`][crate::builtin::Rid], then `body_a` is pinned to the point `anchor` (given in global coordinates); otherwise, `body_a` is pinned to `body_b` at the point `anchor` (given in global coordinates). To set the parameters which are specific to the pin joint, see [`pin_joint_set_param`][`crate::classes::PhysicsServer2D::pin_joint_set_param`]."]
        #[inline]
        pub fn joint_make_pin(&mut self, joint: Rid, anchor: Vector2, body_a: Rid,) {
            self.joint_make_pin_ex(joint, anchor, body_a,) . done()
        }
        #[doc = "Makes the joint a pin joint. If `body_b` is an empty [`RID`][crate::builtin::Rid], then `body_a` is pinned to the point `anchor` (given in global coordinates); otherwise, `body_a` is pinned to `body_b` at the point `anchor` (given in global coordinates). To set the parameters which are specific to the pin joint, see [`pin_joint_set_param`][`crate::classes::PhysicsServer2D::pin_joint_set_param`]."]
        #[inline]
        pub fn joint_make_pin_ex < 'ex > (&'ex mut self, joint: Rid, anchor: Vector2, body_a: Rid,) -> ExJointMakePin < 'ex > {
            ExJointMakePin::new(self, joint, anchor, body_a,)
        }
        #[doc = "Makes the joint a groove joint."]
        pub(crate) fn joint_make_groove_full(&mut self, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Vector2, Vector2, Rid, Rid,);
            let args = (joint, groove1_a, groove2_a, anchor_b, body_a, body_b,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_make_groove", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`joint_make_groove_ex`][Self::joint_make_groove_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Makes the joint a groove joint."]
        #[inline]
        pub fn joint_make_groove(&mut self, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2,) {
            self.joint_make_groove_ex(joint, groove1_a, groove2_a, anchor_b,) . done()
        }
        #[doc = "Makes the joint a groove joint."]
        #[inline]
        pub fn joint_make_groove_ex < 'ex > (&'ex mut self, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2,) -> ExJointMakeGroove < 'ex > {
            ExJointMakeGroove::new(self, joint, groove1_a, groove2_a, anchor_b,)
        }
        #[doc = "Makes the joint a damped spring joint, attached at the point `anchor_a` (given in global coordinates) on the body `body_a` and at the point `anchor_b` (given in global coordinates) on the body `body_b`. To set the parameters which are specific to the damped spring, see [`damped_spring_joint_set_param`][`crate::classes::PhysicsServer2D::damped_spring_joint_set_param`]."]
        pub(crate) fn joint_make_damped_spring_full(&mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Vector2, Rid, Rid,);
            let args = (joint, anchor_a, anchor_b, body_a, body_b,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_make_damped_spring", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`joint_make_damped_spring_ex`][Self::joint_make_damped_spring_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Makes the joint a damped spring joint, attached at the point `anchor_a` (given in global coordinates) on the body `body_a` and at the point `anchor_b` (given in global coordinates) on the body `body_b`. To set the parameters which are specific to the damped spring, see [`damped_spring_joint_set_param`][`crate::classes::PhysicsServer2D::damped_spring_joint_set_param`]."]
        #[inline]
        pub fn joint_make_damped_spring(&mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid,) {
            self.joint_make_damped_spring_ex(joint, anchor_a, anchor_b, body_a,) . done()
        }
        #[doc = "Makes the joint a damped spring joint, attached at the point `anchor_a` (given in global coordinates) on the body `body_a` and at the point `anchor_b` (given in global coordinates) on the body `body_b`. To set the parameters which are specific to the damped spring, see [`damped_spring_joint_set_param`][`crate::classes::PhysicsServer2D::damped_spring_joint_set_param`]."]
        #[inline]
        pub fn joint_make_damped_spring_ex < 'ex > (&'ex mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid,) -> ExJointMakeDampedSpring < 'ex > {
            ExJointMakeDampedSpring::new(self, joint, anchor_a, anchor_b, body_a,)
        }
        #[doc = "Sets a pin joint flag."]
        pub fn pin_joint_set_flag(&mut self, joint: Rid, flag: crate::classes::physics_server_2d::PinJointFlag, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_2d::PinJointFlag, bool,);
            let args = (joint, flag, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "pin_joint_set_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a pin joint flag."]
        pub fn pin_joint_get_flag(&self, joint: Rid, flag: crate::classes::physics_server_2d::PinJointFlag,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, crate::classes::physics_server_2d::PinJointFlag,);
            let args = (joint, flag,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "pin_joint_get_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a pin joint parameter."]
        pub fn pin_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::PinJointParam, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_2d::PinJointParam, f32,);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "pin_joint_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of a pin joint parameter."]
        pub fn pin_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::PinJointParam,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, crate::classes::physics_server_2d::PinJointParam,);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "pin_joint_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of the given damped spring joint parameter."]
        pub fn damped_spring_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::DampedSpringParam, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_2d::DampedSpringParam, f32,);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "damped_spring_joint_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the given damped spring joint parameter."]
        pub fn damped_spring_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::DampedSpringParam,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, crate::classes::physics_server_2d::DampedSpringParam,);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "damped_spring_joint_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the joint's type."]
        pub fn joint_get_type(&self, joint: Rid,) -> crate::classes::physics_server_2d::JointType {
            type CallRet = crate::classes::physics_server_2d::JointType;
            type CallParams = (Rid,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "joint_get_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Destroys any of the objects created by PhysicsServer2D. If the [`RID`][crate::builtin::Rid] passed is not one of the objects that can be created by PhysicsServer2D, an error will be printed to the console."]
        pub fn free_rid(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "free_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Activates or deactivates the 2D physics server. If `active` is `false`, then the physics server will not do anything in its physics step."]
        pub fn set_active(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "set_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of a physics engine state specified by `process_info`."]
        pub fn get_process_info(&self, process_info: crate::classes::physics_server_2d::ProcessInfo,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::classes::physics_server_2d::ProcessInfo,);
            let args = (process_info,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2D", "get_process_info", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsServer2D {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsServer2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsServer2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsServer2D {
        
    }
    impl crate::obj::Singleton for PhysicsServer2D {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"PhysicsServer2D"))
            }
        }
    }
    impl std::ops::Deref for PhysicsServer2D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsServer2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsServer2D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PhysicsServer2D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::area_add_shape_ex`][super::PhysicsServer2D::area_add_shape_ex]."]
#[must_use]
pub struct ExAreaAddShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer2D, area: Rid, shape: Rid, transform: Transform2D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAreaAddShape < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer2D, area: Rid, shape: Rid,) -> Self {
        let transform = Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _);
        let disabled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, area: area, shape: shape, transform: transform, disabled: disabled,
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform2D) -> Self {
        Self {
            transform: transform, .. self
        }
    }
    #[inline]
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled: disabled, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, area, shape, transform, disabled,
        }
        = self;
        re_export::PhysicsServer2D::area_add_shape_full(surround_object, area, shape, transform, disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_add_shape_ex`][super::PhysicsServer2D::body_add_shape_ex]."]
#[must_use]
pub struct ExBodyAddShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, shape: Rid, transform: Transform2D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodyAddShape < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, shape: Rid,) -> Self {
        let transform = Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _);
        let disabled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, shape: shape, transform: transform, disabled: disabled,
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform2D) -> Self {
        Self {
            transform: transform, .. self
        }
    }
    #[inline]
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled: disabled, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, body, shape, transform, disabled,
        }
        = self;
        re_export::PhysicsServer2D::body_add_shape_full(surround_object, body, shape, transform, disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_apply_impulse_ex`][super::PhysicsServer2D::body_apply_impulse_ex]."]
#[must_use]
pub struct ExBodyApplyImpulse < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, impulse: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodyApplyImpulse < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, impulse: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, body, impulse, position,
        }
        = self;
        re_export::PhysicsServer2D::body_apply_impulse_full(surround_object, body, impulse, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_apply_force_ex`][super::PhysicsServer2D::body_apply_force_ex]."]
#[must_use]
pub struct ExBodyApplyForce < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodyApplyForce < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, body, force, position,
        }
        = self;
        re_export::PhysicsServer2D::body_apply_force_full(surround_object, body, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_add_constant_force_ex`][super::PhysicsServer2D::body_add_constant_force_ex]."]
#[must_use]
pub struct ExBodyAddConstantForce < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodyAddConstantForce < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, body, force, position,
        }
        = self;
        re_export::PhysicsServer2D::body_add_constant_force_full(surround_object, body, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_set_force_integration_callback_ex`][super::PhysicsServer2D::body_set_force_integration_callback_ex]."]
#[must_use]
pub struct ExBodySetForceIntegrationCallback < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, callable: CowArg < 'ex, Callable >, userdata: CowArg < 'ex, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodySetForceIntegrationCallback < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, callable: &'ex Callable,) -> Self {
        let userdata = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, callable: CowArg::Borrowed(callable), userdata: CowArg::Owned(userdata),
        }
    }
    #[inline]
    pub fn userdata(self, userdata: &'ex Variant) -> Self {
        Self {
            userdata: CowArg::Borrowed(userdata), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, body, callable, userdata,
        }
        = self;
        re_export::PhysicsServer2D::body_set_force_integration_callback_full(surround_object, body, callable.cow_as_arg(), userdata.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::body_test_motion_ex`][super::PhysicsServer2D::body_test_motion_ex]."]
#[must_use]
pub struct ExBodyTestMotion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, parameters: CowArg < 'ex, Gd < crate::classes::PhysicsTestMotionParameters2D > >, result: CowArg < 'ex, Option < Gd < crate::classes::PhysicsTestMotionResult2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodyTestMotion < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer2D, body: Rid, parameters: impl AsArg < Gd < crate::classes::PhysicsTestMotionParameters2D >> + 'ex,) -> Self {
        let result = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, parameters: parameters.into_arg(), result: result.into_arg(),
        }
    }
    #[inline]
    pub fn result(self, result: impl AsArg < Option < Gd < crate::classes::PhysicsTestMotionResult2D >> > + 'ex) -> Self {
        Self {
            result: result.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, body, parameters, result,
        }
        = self;
        re_export::PhysicsServer2D::body_test_motion_full(surround_object, body, parameters, result,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::joint_make_pin_ex`][super::PhysicsServer2D::joint_make_pin_ex]."]
#[must_use]
pub struct ExJointMakePin < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer2D, joint: Rid, anchor: Vector2, body_a: Rid, body_b: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExJointMakePin < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer2D, joint: Rid, anchor: Vector2, body_a: Rid,) -> Self {
        let body_b = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, joint: joint, anchor: anchor, body_a: body_a, body_b: body_b,
        }
    }
    #[inline]
    pub fn body_b(self, body_b: Rid) -> Self {
        Self {
            body_b: body_b, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, joint, anchor, body_a, body_b,
        }
        = self;
        re_export::PhysicsServer2D::joint_make_pin_full(surround_object, joint, anchor, body_a, body_b,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::joint_make_groove_ex`][super::PhysicsServer2D::joint_make_groove_ex]."]
#[must_use]
pub struct ExJointMakeGroove < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer2D, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExJointMakeGroove < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer2D, joint: Rid, groove1_a: Vector2, groove2_a: Vector2, anchor_b: Vector2,) -> Self {
        let body_a = Rid::Invalid;
        let body_b = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, joint: joint, groove1_a: groove1_a, groove2_a: groove2_a, anchor_b: anchor_b, body_a: body_a, body_b: body_b,
        }
    }
    #[inline]
    pub fn body_a(self, body_a: Rid) -> Self {
        Self {
            body_a: body_a, .. self
        }
    }
    #[inline]
    pub fn body_b(self, body_b: Rid) -> Self {
        Self {
            body_b: body_b, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, joint, groove1_a, groove2_a, anchor_b, body_a, body_b,
        }
        = self;
        re_export::PhysicsServer2D::joint_make_groove_full(surround_object, joint, groove1_a, groove2_a, anchor_b, body_a, body_b,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer2D::joint_make_damped_spring_ex`][super::PhysicsServer2D::joint_make_damped_spring_ex]."]
#[must_use]
pub struct ExJointMakeDampedSpring < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer2D, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExJointMakeDampedSpring < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer2D, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid,) -> Self {
        let body_b = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, joint: joint, anchor_a: anchor_a, anchor_b: anchor_b, body_a: body_a, body_b: body_b,
        }
    }
    #[inline]
    pub fn body_b(self, body_b: Rid) -> Self {
        Self {
            body_b: body_b, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, joint, anchor_a, anchor_b, body_a, body_b,
        }
        = self;
        re_export::PhysicsServer2D::joint_make_damped_spring_full(surround_object, joint, anchor_a, anchor_b, body_a, body_b,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpaceParameter {
    ord: i32
}
impl SpaceParameter {
    #[doc(alias = "SPACE_PARAM_CONTACT_RECYCLE_RADIUS")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_CONTACT_RECYCLE_RADIUS`"]
    pub const CONTACT_RECYCLE_RADIUS: SpaceParameter = SpaceParameter {
        ord: 0i32
    };
    #[doc(alias = "SPACE_PARAM_CONTACT_MAX_SEPARATION")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_CONTACT_MAX_SEPARATION`"]
    pub const CONTACT_MAX_SEPARATION: SpaceParameter = SpaceParameter {
        ord: 1i32
    };
    #[doc(alias = "SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION`"]
    pub const CONTACT_MAX_ALLOWED_PENETRATION: SpaceParameter = SpaceParameter {
        ord: 2i32
    };
    #[doc(alias = "SPACE_PARAM_CONTACT_DEFAULT_BIAS")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_CONTACT_DEFAULT_BIAS`"]
    pub const CONTACT_DEFAULT_BIAS: SpaceParameter = SpaceParameter {
        ord: 3i32
    };
    #[doc(alias = "SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD`"]
    pub const BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD: SpaceParameter = SpaceParameter {
        ord: 4i32
    };
    #[doc(alias = "SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD`"]
    pub const BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD: SpaceParameter = SpaceParameter {
        ord: 5i32
    };
    #[doc(alias = "SPACE_PARAM_BODY_TIME_TO_SLEEP")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_BODY_TIME_TO_SLEEP`"]
    pub const BODY_TIME_TO_SLEEP: SpaceParameter = SpaceParameter {
        ord: 6i32
    };
    #[doc(alias = "SPACE_PARAM_CONSTRAINT_DEFAULT_BIAS")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_CONSTRAINT_DEFAULT_BIAS`"]
    pub const CONSTRAINT_DEFAULT_BIAS: SpaceParameter = SpaceParameter {
        ord: 7i32
    };
    #[doc(alias = "SPACE_PARAM_SOLVER_ITERATIONS")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_SOLVER_ITERATIONS`"]
    pub const SOLVER_ITERATIONS: SpaceParameter = SpaceParameter {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for SpaceParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpaceParameter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpaceParameter {
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
            Self::CONTACT_RECYCLE_RADIUS => "CONTACT_RECYCLE_RADIUS", Self::CONTACT_MAX_SEPARATION => "CONTACT_MAX_SEPARATION", Self::CONTACT_MAX_ALLOWED_PENETRATION => "CONTACT_MAX_ALLOWED_PENETRATION", Self::CONTACT_DEFAULT_BIAS => "CONTACT_DEFAULT_BIAS", Self::BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD => "BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD => "BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_TIME_TO_SLEEP => "BODY_TIME_TO_SLEEP", Self::CONSTRAINT_DEFAULT_BIAS => "CONSTRAINT_DEFAULT_BIAS", Self::SOLVER_ITERATIONS => "SOLVER_ITERATIONS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SpaceParameter::CONTACT_RECYCLE_RADIUS, SpaceParameter::CONTACT_MAX_SEPARATION, SpaceParameter::CONTACT_MAX_ALLOWED_PENETRATION, SpaceParameter::CONTACT_DEFAULT_BIAS, SpaceParameter::BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD, SpaceParameter::BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD, SpaceParameter::BODY_TIME_TO_SLEEP, SpaceParameter::CONSTRAINT_DEFAULT_BIAS, SpaceParameter::SOLVER_ITERATIONS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SpaceParameter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CONTACT_RECYCLE_RADIUS", "SPACE_PARAM_CONTACT_RECYCLE_RADIUS", SpaceParameter::CONTACT_RECYCLE_RADIUS), crate::meta::inspect::EnumConstant::new("CONTACT_MAX_SEPARATION", "SPACE_PARAM_CONTACT_MAX_SEPARATION", SpaceParameter::CONTACT_MAX_SEPARATION), crate::meta::inspect::EnumConstant::new("CONTACT_MAX_ALLOWED_PENETRATION", "SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION", SpaceParameter::CONTACT_MAX_ALLOWED_PENETRATION), crate::meta::inspect::EnumConstant::new("CONTACT_DEFAULT_BIAS", "SPACE_PARAM_CONTACT_DEFAULT_BIAS", SpaceParameter::CONTACT_DEFAULT_BIAS), crate::meta::inspect::EnumConstant::new("BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD", "SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD", SpaceParameter::BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD), crate::meta::inspect::EnumConstant::new("BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD", "SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD", SpaceParameter::BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD), crate::meta::inspect::EnumConstant::new("BODY_TIME_TO_SLEEP", "SPACE_PARAM_BODY_TIME_TO_SLEEP", SpaceParameter::BODY_TIME_TO_SLEEP), crate::meta::inspect::EnumConstant::new("CONSTRAINT_DEFAULT_BIAS", "SPACE_PARAM_CONSTRAINT_DEFAULT_BIAS", SpaceParameter::CONSTRAINT_DEFAULT_BIAS), crate::meta::inspect::EnumConstant::new("SOLVER_ITERATIONS", "SPACE_PARAM_SOLVER_ITERATIONS", SpaceParameter::SOLVER_ITERATIONS)]
        }
    }
}
impl crate::meta::GodotConvert for SpaceParameter {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Space Param Contact Recycle Radius", 0i64), EnumeratorShape::new_int("Space Param Contact Max Separation", 1i64), EnumeratorShape::new_int("Space Param Contact Max Allowed Penetration", 2i64), EnumeratorShape::new_int("Space Param Contact Default Bias", 3i64), EnumeratorShape::new_int("Space Param Body Linear Velocity Sleep Threshold", 4i64), EnumeratorShape::new_int("Space Param Body Angular Velocity Sleep Threshold", 5i64), EnumeratorShape::new_int("Space Param Body Time To Sleep", 6i64), EnumeratorShape::new_int("Space Param Constraint Default Bias", 7i64), EnumeratorShape::new_int("Space Param Solver Iterations", 8i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.SpaceParameter")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SpaceParameter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpaceParameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SpaceParameter {
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
impl crate::registry::property::Export for SpaceParameter {
    
}
impl crate::meta::Element for SpaceParameter {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShapeType {
    ord: i32
}
impl ShapeType {
    #[doc(alias = "SHAPE_WORLD_BOUNDARY")]
    #[doc = "Godot enumerator name: `SHAPE_WORLD_BOUNDARY`"]
    pub const WORLD_BOUNDARY: ShapeType = ShapeType {
        ord: 0i32
    };
    #[doc(alias = "SHAPE_SEPARATION_RAY")]
    #[doc = "Godot enumerator name: `SHAPE_SEPARATION_RAY`"]
    pub const SEPARATION_RAY: ShapeType = ShapeType {
        ord: 1i32
    };
    #[doc(alias = "SHAPE_SEGMENT")]
    #[doc = "Godot enumerator name: `SHAPE_SEGMENT`"]
    pub const SEGMENT: ShapeType = ShapeType {
        ord: 2i32
    };
    #[doc(alias = "SHAPE_CIRCLE")]
    #[doc = "Godot enumerator name: `SHAPE_CIRCLE`"]
    pub const CIRCLE: ShapeType = ShapeType {
        ord: 3i32
    };
    #[doc(alias = "SHAPE_RECTANGLE")]
    #[doc = "Godot enumerator name: `SHAPE_RECTANGLE`"]
    pub const RECTANGLE: ShapeType = ShapeType {
        ord: 4i32
    };
    #[doc(alias = "SHAPE_CAPSULE")]
    #[doc = "Godot enumerator name: `SHAPE_CAPSULE`"]
    pub const CAPSULE: ShapeType = ShapeType {
        ord: 5i32
    };
    #[doc(alias = "SHAPE_CONVEX_POLYGON")]
    #[doc = "Godot enumerator name: `SHAPE_CONVEX_POLYGON`"]
    pub const CONVEX_POLYGON: ShapeType = ShapeType {
        ord: 6i32
    };
    #[doc(alias = "SHAPE_CONCAVE_POLYGON")]
    #[doc = "Godot enumerator name: `SHAPE_CONCAVE_POLYGON`"]
    pub const CONCAVE_POLYGON: ShapeType = ShapeType {
        ord: 7i32
    };
    #[doc(alias = "SHAPE_CUSTOM")]
    #[doc = "Godot enumerator name: `SHAPE_CUSTOM`"]
    pub const CUSTOM: ShapeType = ShapeType {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for ShapeType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShapeType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShapeType {
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
            Self::WORLD_BOUNDARY => "WORLD_BOUNDARY", Self::SEPARATION_RAY => "SEPARATION_RAY", Self::SEGMENT => "SEGMENT", Self::CIRCLE => "CIRCLE", Self::RECTANGLE => "RECTANGLE", Self::CAPSULE => "CAPSULE", Self::CONVEX_POLYGON => "CONVEX_POLYGON", Self::CONCAVE_POLYGON => "CONCAVE_POLYGON", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShapeType::WORLD_BOUNDARY, ShapeType::SEPARATION_RAY, ShapeType::SEGMENT, ShapeType::CIRCLE, ShapeType::RECTANGLE, ShapeType::CAPSULE, ShapeType::CONVEX_POLYGON, ShapeType::CONCAVE_POLYGON, ShapeType::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShapeType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("WORLD_BOUNDARY", "SHAPE_WORLD_BOUNDARY", ShapeType::WORLD_BOUNDARY), crate::meta::inspect::EnumConstant::new("SEPARATION_RAY", "SHAPE_SEPARATION_RAY", ShapeType::SEPARATION_RAY), crate::meta::inspect::EnumConstant::new("SEGMENT", "SHAPE_SEGMENT", ShapeType::SEGMENT), crate::meta::inspect::EnumConstant::new("CIRCLE", "SHAPE_CIRCLE", ShapeType::CIRCLE), crate::meta::inspect::EnumConstant::new("RECTANGLE", "SHAPE_RECTANGLE", ShapeType::RECTANGLE), crate::meta::inspect::EnumConstant::new("CAPSULE", "SHAPE_CAPSULE", ShapeType::CAPSULE), crate::meta::inspect::EnumConstant::new("CONVEX_POLYGON", "SHAPE_CONVEX_POLYGON", ShapeType::CONVEX_POLYGON), crate::meta::inspect::EnumConstant::new("CONCAVE_POLYGON", "SHAPE_CONCAVE_POLYGON", ShapeType::CONCAVE_POLYGON), crate::meta::inspect::EnumConstant::new("CUSTOM", "SHAPE_CUSTOM", ShapeType::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for ShapeType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Shape World Boundary", 0i64), EnumeratorShape::new_int("Shape Separation Ray", 1i64), EnumeratorShape::new_int("Shape Segment", 2i64), EnumeratorShape::new_int("Shape Circle", 3i64), EnumeratorShape::new_int("Shape Rectangle", 4i64), EnumeratorShape::new_int("Shape Capsule", 5i64), EnumeratorShape::new_int("Shape Convex Polygon", 6i64), EnumeratorShape::new_int("Shape Concave Polygon", 7i64), EnumeratorShape::new_int("Shape Custom", 8i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.ShapeType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ShapeType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShapeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ShapeType {
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
impl crate::registry::property::Export for ShapeType {
    
}
impl crate::meta::Element for ShapeType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AreaParameter {
    ord: i32
}
impl AreaParameter {
    #[doc(alias = "AREA_PARAM_GRAVITY_OVERRIDE_MODE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_GRAVITY_OVERRIDE_MODE`"]
    pub const GRAVITY_OVERRIDE_MODE: AreaParameter = AreaParameter {
        ord: 0i32
    };
    #[doc(alias = "AREA_PARAM_GRAVITY")]
    #[doc = "Godot enumerator name: `AREA_PARAM_GRAVITY`"]
    pub const GRAVITY: AreaParameter = AreaParameter {
        ord: 1i32
    };
    #[doc(alias = "AREA_PARAM_GRAVITY_VECTOR")]
    #[doc = "Godot enumerator name: `AREA_PARAM_GRAVITY_VECTOR`"]
    pub const GRAVITY_VECTOR: AreaParameter = AreaParameter {
        ord: 2i32
    };
    #[doc(alias = "AREA_PARAM_GRAVITY_IS_POINT")]
    #[doc = "Godot enumerator name: `AREA_PARAM_GRAVITY_IS_POINT`"]
    pub const GRAVITY_IS_POINT: AreaParameter = AreaParameter {
        ord: 3i32
    };
    #[doc(alias = "AREA_PARAM_GRAVITY_POINT_UNIT_DISTANCE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_GRAVITY_POINT_UNIT_DISTANCE`"]
    pub const GRAVITY_POINT_UNIT_DISTANCE: AreaParameter = AreaParameter {
        ord: 4i32
    };
    #[doc(alias = "AREA_PARAM_LINEAR_DAMP_OVERRIDE_MODE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_LINEAR_DAMP_OVERRIDE_MODE`"]
    pub const LINEAR_DAMP_OVERRIDE_MODE: AreaParameter = AreaParameter {
        ord: 5i32
    };
    #[doc(alias = "AREA_PARAM_LINEAR_DAMP")]
    #[doc = "Godot enumerator name: `AREA_PARAM_LINEAR_DAMP`"]
    pub const LINEAR_DAMP: AreaParameter = AreaParameter {
        ord: 6i32
    };
    #[doc(alias = "AREA_PARAM_ANGULAR_DAMP_OVERRIDE_MODE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_ANGULAR_DAMP_OVERRIDE_MODE`"]
    pub const ANGULAR_DAMP_OVERRIDE_MODE: AreaParameter = AreaParameter {
        ord: 7i32
    };
    #[doc(alias = "AREA_PARAM_ANGULAR_DAMP")]
    #[doc = "Godot enumerator name: `AREA_PARAM_ANGULAR_DAMP`"]
    pub const ANGULAR_DAMP: AreaParameter = AreaParameter {
        ord: 8i32
    };
    #[doc(alias = "AREA_PARAM_PRIORITY")]
    #[doc = "Godot enumerator name: `AREA_PARAM_PRIORITY`"]
    pub const PRIORITY: AreaParameter = AreaParameter {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for AreaParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AreaParameter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AreaParameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::GRAVITY_OVERRIDE_MODE => "GRAVITY_OVERRIDE_MODE", Self::GRAVITY => "GRAVITY", Self::GRAVITY_VECTOR => "GRAVITY_VECTOR", Self::GRAVITY_IS_POINT => "GRAVITY_IS_POINT", Self::GRAVITY_POINT_UNIT_DISTANCE => "GRAVITY_POINT_UNIT_DISTANCE", Self::LINEAR_DAMP_OVERRIDE_MODE => "LINEAR_DAMP_OVERRIDE_MODE", Self::LINEAR_DAMP => "LINEAR_DAMP", Self::ANGULAR_DAMP_OVERRIDE_MODE => "ANGULAR_DAMP_OVERRIDE_MODE", Self::ANGULAR_DAMP => "ANGULAR_DAMP", Self::PRIORITY => "PRIORITY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AreaParameter::GRAVITY_OVERRIDE_MODE, AreaParameter::GRAVITY, AreaParameter::GRAVITY_VECTOR, AreaParameter::GRAVITY_IS_POINT, AreaParameter::GRAVITY_POINT_UNIT_DISTANCE, AreaParameter::LINEAR_DAMP_OVERRIDE_MODE, AreaParameter::LINEAR_DAMP, AreaParameter::ANGULAR_DAMP_OVERRIDE_MODE, AreaParameter::ANGULAR_DAMP, AreaParameter::PRIORITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AreaParameter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GRAVITY_OVERRIDE_MODE", "AREA_PARAM_GRAVITY_OVERRIDE_MODE", AreaParameter::GRAVITY_OVERRIDE_MODE), crate::meta::inspect::EnumConstant::new("GRAVITY", "AREA_PARAM_GRAVITY", AreaParameter::GRAVITY), crate::meta::inspect::EnumConstant::new("GRAVITY_VECTOR", "AREA_PARAM_GRAVITY_VECTOR", AreaParameter::GRAVITY_VECTOR), crate::meta::inspect::EnumConstant::new("GRAVITY_IS_POINT", "AREA_PARAM_GRAVITY_IS_POINT", AreaParameter::GRAVITY_IS_POINT), crate::meta::inspect::EnumConstant::new("GRAVITY_POINT_UNIT_DISTANCE", "AREA_PARAM_GRAVITY_POINT_UNIT_DISTANCE", AreaParameter::GRAVITY_POINT_UNIT_DISTANCE), crate::meta::inspect::EnumConstant::new("LINEAR_DAMP_OVERRIDE_MODE", "AREA_PARAM_LINEAR_DAMP_OVERRIDE_MODE", AreaParameter::LINEAR_DAMP_OVERRIDE_MODE), crate::meta::inspect::EnumConstant::new("LINEAR_DAMP", "AREA_PARAM_LINEAR_DAMP", AreaParameter::LINEAR_DAMP), crate::meta::inspect::EnumConstant::new("ANGULAR_DAMP_OVERRIDE_MODE", "AREA_PARAM_ANGULAR_DAMP_OVERRIDE_MODE", AreaParameter::ANGULAR_DAMP_OVERRIDE_MODE), crate::meta::inspect::EnumConstant::new("ANGULAR_DAMP", "AREA_PARAM_ANGULAR_DAMP", AreaParameter::ANGULAR_DAMP), crate::meta::inspect::EnumConstant::new("PRIORITY", "AREA_PARAM_PRIORITY", AreaParameter::PRIORITY)]
        }
    }
}
impl crate::meta::GodotConvert for AreaParameter {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Area Param Gravity Override Mode", 0i64), EnumeratorShape::new_int("Area Param Gravity", 1i64), EnumeratorShape::new_int("Area Param Gravity Vector", 2i64), EnumeratorShape::new_int("Area Param Gravity Is Point", 3i64), EnumeratorShape::new_int("Area Param Gravity Point Unit Distance", 4i64), EnumeratorShape::new_int("Area Param Linear Damp Override Mode", 5i64), EnumeratorShape::new_int("Area Param Linear Damp", 6i64), EnumeratorShape::new_int("Area Param Angular Damp Override Mode", 7i64), EnumeratorShape::new_int("Area Param Angular Damp", 8i64), EnumeratorShape::new_int("Area Param Priority", 9i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.AreaParameter")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AreaParameter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AreaParameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AreaParameter {
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
impl crate::registry::property::Export for AreaParameter {
    
}
impl crate::meta::Element for AreaParameter {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AreaSpaceOverrideMode {
    ord: i32
}
impl AreaSpaceOverrideMode {
    #[doc(alias = "AREA_SPACE_OVERRIDE_DISABLED")]
    #[doc = "Godot enumerator name: `AREA_SPACE_OVERRIDE_DISABLED`"]
    pub const DISABLED: AreaSpaceOverrideMode = AreaSpaceOverrideMode {
        ord: 0i32
    };
    #[doc(alias = "AREA_SPACE_OVERRIDE_COMBINE")]
    #[doc = "Godot enumerator name: `AREA_SPACE_OVERRIDE_COMBINE`"]
    pub const COMBINE: AreaSpaceOverrideMode = AreaSpaceOverrideMode {
        ord: 1i32
    };
    #[doc(alias = "AREA_SPACE_OVERRIDE_COMBINE_REPLACE")]
    #[doc = "Godot enumerator name: `AREA_SPACE_OVERRIDE_COMBINE_REPLACE`"]
    pub const COMBINE_REPLACE: AreaSpaceOverrideMode = AreaSpaceOverrideMode {
        ord: 2i32
    };
    #[doc(alias = "AREA_SPACE_OVERRIDE_REPLACE")]
    #[doc = "Godot enumerator name: `AREA_SPACE_OVERRIDE_REPLACE`"]
    pub const REPLACE: AreaSpaceOverrideMode = AreaSpaceOverrideMode {
        ord: 3i32
    };
    #[doc(alias = "AREA_SPACE_OVERRIDE_REPLACE_COMBINE")]
    #[doc = "Godot enumerator name: `AREA_SPACE_OVERRIDE_REPLACE_COMBINE`"]
    pub const REPLACE_COMBINE: AreaSpaceOverrideMode = AreaSpaceOverrideMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for AreaSpaceOverrideMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AreaSpaceOverrideMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AreaSpaceOverrideMode {
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
            Self::DISABLED => "DISABLED", Self::COMBINE => "COMBINE", Self::COMBINE_REPLACE => "COMBINE_REPLACE", Self::REPLACE => "REPLACE", Self::REPLACE_COMBINE => "REPLACE_COMBINE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AreaSpaceOverrideMode::DISABLED, AreaSpaceOverrideMode::COMBINE, AreaSpaceOverrideMode::COMBINE_REPLACE, AreaSpaceOverrideMode::REPLACE, AreaSpaceOverrideMode::REPLACE_COMBINE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AreaSpaceOverrideMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "AREA_SPACE_OVERRIDE_DISABLED", AreaSpaceOverrideMode::DISABLED), crate::meta::inspect::EnumConstant::new("COMBINE", "AREA_SPACE_OVERRIDE_COMBINE", AreaSpaceOverrideMode::COMBINE), crate::meta::inspect::EnumConstant::new("COMBINE_REPLACE", "AREA_SPACE_OVERRIDE_COMBINE_REPLACE", AreaSpaceOverrideMode::COMBINE_REPLACE), crate::meta::inspect::EnumConstant::new("REPLACE", "AREA_SPACE_OVERRIDE_REPLACE", AreaSpaceOverrideMode::REPLACE), crate::meta::inspect::EnumConstant::new("REPLACE_COMBINE", "AREA_SPACE_OVERRIDE_REPLACE_COMBINE", AreaSpaceOverrideMode::REPLACE_COMBINE)]
        }
    }
}
impl crate::meta::GodotConvert for AreaSpaceOverrideMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Area Space Override Disabled", 0i64), EnumeratorShape::new_int("Area Space Override Combine", 1i64), EnumeratorShape::new_int("Area Space Override Combine Replace", 2i64), EnumeratorShape::new_int("Area Space Override Replace", 3i64), EnumeratorShape::new_int("Area Space Override Replace Combine", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.AreaSpaceOverrideMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AreaSpaceOverrideMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AreaSpaceOverrideMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AreaSpaceOverrideMode {
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
impl crate::registry::property::Export for AreaSpaceOverrideMode {
    
}
impl crate::meta::Element for AreaSpaceOverrideMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyMode {
    ord: i32
}
impl BodyMode {
    #[doc(alias = "BODY_MODE_STATIC")]
    #[doc = "Godot enumerator name: `BODY_MODE_STATIC`"]
    pub const STATIC: BodyMode = BodyMode {
        ord: 0i32
    };
    #[doc(alias = "BODY_MODE_KINEMATIC")]
    #[doc = "Godot enumerator name: `BODY_MODE_KINEMATIC`"]
    pub const KINEMATIC: BodyMode = BodyMode {
        ord: 1i32
    };
    #[doc(alias = "BODY_MODE_RIGID")]
    #[doc = "Godot enumerator name: `BODY_MODE_RIGID`"]
    pub const RIGID: BodyMode = BodyMode {
        ord: 2i32
    };
    #[doc(alias = "BODY_MODE_RIGID_LINEAR")]
    #[doc = "Godot enumerator name: `BODY_MODE_RIGID_LINEAR`"]
    pub const RIGID_LINEAR: BodyMode = BodyMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for BodyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BodyMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BodyMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::STATIC => "STATIC", Self::KINEMATIC => "KINEMATIC", Self::RIGID => "RIGID", Self::RIGID_LINEAR => "RIGID_LINEAR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BodyMode::STATIC, BodyMode::KINEMATIC, BodyMode::RIGID, BodyMode::RIGID_LINEAR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BodyMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STATIC", "BODY_MODE_STATIC", BodyMode::STATIC), crate::meta::inspect::EnumConstant::new("KINEMATIC", "BODY_MODE_KINEMATIC", BodyMode::KINEMATIC), crate::meta::inspect::EnumConstant::new("RIGID", "BODY_MODE_RIGID", BodyMode::RIGID), crate::meta::inspect::EnumConstant::new("RIGID_LINEAR", "BODY_MODE_RIGID_LINEAR", BodyMode::RIGID_LINEAR)]
        }
    }
}
impl crate::meta::GodotConvert for BodyMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Body Mode Static", 0i64), EnumeratorShape::new_int("Body Mode Kinematic", 1i64), EnumeratorShape::new_int("Body Mode Rigid", 2i64), EnumeratorShape::new_int("Body Mode Rigid Linear", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.BodyMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BodyMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BodyMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BodyMode {
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
impl crate::registry::property::Export for BodyMode {
    
}
impl crate::meta::Element for BodyMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyParameter {
    ord: i32
}
impl BodyParameter {
    #[doc(alias = "BODY_PARAM_BOUNCE")]
    #[doc = "Godot enumerator name: `BODY_PARAM_BOUNCE`"]
    pub const BOUNCE: BodyParameter = BodyParameter {
        ord: 0i32
    };
    #[doc(alias = "BODY_PARAM_FRICTION")]
    #[doc = "Godot enumerator name: `BODY_PARAM_FRICTION`"]
    pub const FRICTION: BodyParameter = BodyParameter {
        ord: 1i32
    };
    #[doc(alias = "BODY_PARAM_MASS")]
    #[doc = "Godot enumerator name: `BODY_PARAM_MASS`"]
    pub const MASS: BodyParameter = BodyParameter {
        ord: 2i32
    };
    #[doc(alias = "BODY_PARAM_INERTIA")]
    #[doc = "Godot enumerator name: `BODY_PARAM_INERTIA`"]
    pub const INERTIA: BodyParameter = BodyParameter {
        ord: 3i32
    };
    #[doc(alias = "BODY_PARAM_CENTER_OF_MASS")]
    #[doc = "Godot enumerator name: `BODY_PARAM_CENTER_OF_MASS`"]
    pub const CENTER_OF_MASS: BodyParameter = BodyParameter {
        ord: 4i32
    };
    #[doc(alias = "BODY_PARAM_GRAVITY_SCALE")]
    #[doc = "Godot enumerator name: `BODY_PARAM_GRAVITY_SCALE`"]
    pub const GRAVITY_SCALE: BodyParameter = BodyParameter {
        ord: 5i32
    };
    #[doc(alias = "BODY_PARAM_LINEAR_DAMP_MODE")]
    #[doc = "Godot enumerator name: `BODY_PARAM_LINEAR_DAMP_MODE`"]
    pub const LINEAR_DAMP_MODE: BodyParameter = BodyParameter {
        ord: 6i32
    };
    #[doc(alias = "BODY_PARAM_ANGULAR_DAMP_MODE")]
    #[doc = "Godot enumerator name: `BODY_PARAM_ANGULAR_DAMP_MODE`"]
    pub const ANGULAR_DAMP_MODE: BodyParameter = BodyParameter {
        ord: 7i32
    };
    #[doc(alias = "BODY_PARAM_LINEAR_DAMP")]
    #[doc = "Godot enumerator name: `BODY_PARAM_LINEAR_DAMP`"]
    pub const LINEAR_DAMP: BodyParameter = BodyParameter {
        ord: 8i32
    };
    #[doc(alias = "BODY_PARAM_ANGULAR_DAMP")]
    #[doc = "Godot enumerator name: `BODY_PARAM_ANGULAR_DAMP`"]
    pub const ANGULAR_DAMP: BodyParameter = BodyParameter {
        ord: 9i32
    };
    #[doc(alias = "BODY_PARAM_MAX")]
    #[doc = "Godot enumerator name: `BODY_PARAM_MAX`"]
    pub const MAX: BodyParameter = BodyParameter {
        ord: 10i32
    };
    
}
impl std::fmt::Debug for BodyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BodyParameter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BodyParameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
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
            Self::BOUNCE => "BOUNCE", Self::FRICTION => "FRICTION", Self::MASS => "MASS", Self::INERTIA => "INERTIA", Self::CENTER_OF_MASS => "CENTER_OF_MASS", Self::GRAVITY_SCALE => "GRAVITY_SCALE", Self::LINEAR_DAMP_MODE => "LINEAR_DAMP_MODE", Self::ANGULAR_DAMP_MODE => "ANGULAR_DAMP_MODE", Self::LINEAR_DAMP => "LINEAR_DAMP", Self::ANGULAR_DAMP => "ANGULAR_DAMP", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BodyParameter::BOUNCE, BodyParameter::FRICTION, BodyParameter::MASS, BodyParameter::INERTIA, BodyParameter::CENTER_OF_MASS, BodyParameter::GRAVITY_SCALE, BodyParameter::LINEAR_DAMP_MODE, BodyParameter::ANGULAR_DAMP_MODE, BodyParameter::LINEAR_DAMP, BodyParameter::ANGULAR_DAMP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BodyParameter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOUNCE", "BODY_PARAM_BOUNCE", BodyParameter::BOUNCE), crate::meta::inspect::EnumConstant::new("FRICTION", "BODY_PARAM_FRICTION", BodyParameter::FRICTION), crate::meta::inspect::EnumConstant::new("MASS", "BODY_PARAM_MASS", BodyParameter::MASS), crate::meta::inspect::EnumConstant::new("INERTIA", "BODY_PARAM_INERTIA", BodyParameter::INERTIA), crate::meta::inspect::EnumConstant::new("CENTER_OF_MASS", "BODY_PARAM_CENTER_OF_MASS", BodyParameter::CENTER_OF_MASS), crate::meta::inspect::EnumConstant::new("GRAVITY_SCALE", "BODY_PARAM_GRAVITY_SCALE", BodyParameter::GRAVITY_SCALE), crate::meta::inspect::EnumConstant::new("LINEAR_DAMP_MODE", "BODY_PARAM_LINEAR_DAMP_MODE", BodyParameter::LINEAR_DAMP_MODE), crate::meta::inspect::EnumConstant::new("ANGULAR_DAMP_MODE", "BODY_PARAM_ANGULAR_DAMP_MODE", BodyParameter::ANGULAR_DAMP_MODE), crate::meta::inspect::EnumConstant::new("LINEAR_DAMP", "BODY_PARAM_LINEAR_DAMP", BodyParameter::LINEAR_DAMP), crate::meta::inspect::EnumConstant::new("ANGULAR_DAMP", "BODY_PARAM_ANGULAR_DAMP", BodyParameter::ANGULAR_DAMP), crate::meta::inspect::EnumConstant::new("MAX", "BODY_PARAM_MAX", BodyParameter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for BodyParameter {
    const ENUMERATOR_COUNT: usize = 10usize;
    
}
impl crate::meta::GodotConvert for BodyParameter {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Body Param Bounce", 0i64), EnumeratorShape::new_int("Body Param Friction", 1i64), EnumeratorShape::new_int("Body Param Mass", 2i64), EnumeratorShape::new_int("Body Param Inertia", 3i64), EnumeratorShape::new_int("Body Param Center Of Mass", 4i64), EnumeratorShape::new_int("Body Param Gravity Scale", 5i64), EnumeratorShape::new_int("Body Param Linear Damp Mode", 6i64), EnumeratorShape::new_int("Body Param Angular Damp Mode", 7i64), EnumeratorShape::new_int("Body Param Linear Damp", 8i64), EnumeratorShape::new_int("Body Param Angular Damp", 9i64), EnumeratorShape::new_int("Body Param Max", 10i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.BodyParameter")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BodyParameter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BodyParameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BodyParameter {
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
impl crate::registry::property::Export for BodyParameter {
    
}
impl crate::meta::Element for BodyParameter {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyDampMode {
    ord: i32
}
impl BodyDampMode {
    #[doc(alias = "BODY_DAMP_MODE_COMBINE")]
    #[doc = "Godot enumerator name: `BODY_DAMP_MODE_COMBINE`"]
    pub const COMBINE: BodyDampMode = BodyDampMode {
        ord: 0i32
    };
    #[doc(alias = "BODY_DAMP_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `BODY_DAMP_MODE_REPLACE`"]
    pub const REPLACE: BodyDampMode = BodyDampMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for BodyDampMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BodyDampMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BodyDampMode {
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
            Self::COMBINE => "COMBINE", Self::REPLACE => "REPLACE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BodyDampMode::COMBINE, BodyDampMode::REPLACE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BodyDampMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("COMBINE", "BODY_DAMP_MODE_COMBINE", BodyDampMode::COMBINE), crate::meta::inspect::EnumConstant::new("REPLACE", "BODY_DAMP_MODE_REPLACE", BodyDampMode::REPLACE)]
        }
    }
}
impl crate::meta::GodotConvert for BodyDampMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Body Damp Mode Combine", 0i64), EnumeratorShape::new_int("Body Damp Mode Replace", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.BodyDampMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BodyDampMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BodyDampMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BodyDampMode {
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
impl crate::registry::property::Export for BodyDampMode {
    
}
impl crate::meta::Element for BodyDampMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyState {
    ord: i32
}
impl BodyState {
    #[doc(alias = "BODY_STATE_TRANSFORM")]
    #[doc = "Godot enumerator name: `BODY_STATE_TRANSFORM`"]
    pub const TRANSFORM: BodyState = BodyState {
        ord: 0i32
    };
    #[doc(alias = "BODY_STATE_LINEAR_VELOCITY")]
    #[doc = "Godot enumerator name: `BODY_STATE_LINEAR_VELOCITY`"]
    pub const LINEAR_VELOCITY: BodyState = BodyState {
        ord: 1i32
    };
    #[doc(alias = "BODY_STATE_ANGULAR_VELOCITY")]
    #[doc = "Godot enumerator name: `BODY_STATE_ANGULAR_VELOCITY`"]
    pub const ANGULAR_VELOCITY: BodyState = BodyState {
        ord: 2i32
    };
    #[doc(alias = "BODY_STATE_SLEEPING")]
    #[doc = "Godot enumerator name: `BODY_STATE_SLEEPING`"]
    pub const SLEEPING: BodyState = BodyState {
        ord: 3i32
    };
    #[doc(alias = "BODY_STATE_CAN_SLEEP")]
    #[doc = "Godot enumerator name: `BODY_STATE_CAN_SLEEP`"]
    pub const CAN_SLEEP: BodyState = BodyState {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for BodyState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BodyState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BodyState {
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
            Self::TRANSFORM => "TRANSFORM", Self::LINEAR_VELOCITY => "LINEAR_VELOCITY", Self::ANGULAR_VELOCITY => "ANGULAR_VELOCITY", Self::SLEEPING => "SLEEPING", Self::CAN_SLEEP => "CAN_SLEEP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BodyState::TRANSFORM, BodyState::LINEAR_VELOCITY, BodyState::ANGULAR_VELOCITY, BodyState::SLEEPING, BodyState::CAN_SLEEP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BodyState >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TRANSFORM", "BODY_STATE_TRANSFORM", BodyState::TRANSFORM), crate::meta::inspect::EnumConstant::new("LINEAR_VELOCITY", "BODY_STATE_LINEAR_VELOCITY", BodyState::LINEAR_VELOCITY), crate::meta::inspect::EnumConstant::new("ANGULAR_VELOCITY", "BODY_STATE_ANGULAR_VELOCITY", BodyState::ANGULAR_VELOCITY), crate::meta::inspect::EnumConstant::new("SLEEPING", "BODY_STATE_SLEEPING", BodyState::SLEEPING), crate::meta::inspect::EnumConstant::new("CAN_SLEEP", "BODY_STATE_CAN_SLEEP", BodyState::CAN_SLEEP)]
        }
    }
}
impl crate::meta::GodotConvert for BodyState {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Body State Transform", 0i64), EnumeratorShape::new_int("Body State Linear Velocity", 1i64), EnumeratorShape::new_int("Body State Angular Velocity", 2i64), EnumeratorShape::new_int("Body State Sleeping", 3i64), EnumeratorShape::new_int("Body State Can Sleep", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.BodyState")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BodyState {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BodyState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BodyState {
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
impl crate::registry::property::Export for BodyState {
    
}
impl crate::meta::Element for BodyState {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct JointType {
    ord: i32
}
impl JointType {
    #[doc(alias = "JOINT_TYPE_PIN")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_PIN`"]
    pub const PIN: JointType = JointType {
        ord: 0i32
    };
    #[doc(alias = "JOINT_TYPE_GROOVE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_GROOVE`"]
    pub const GROOVE: JointType = JointType {
        ord: 1i32
    };
    #[doc(alias = "JOINT_TYPE_DAMPED_SPRING")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_DAMPED_SPRING`"]
    pub const DAMPED_SPRING: JointType = JointType {
        ord: 2i32
    };
    #[doc(alias = "JOINT_TYPE_MAX")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_MAX`"]
    pub const MAX: JointType = JointType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for JointType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("JointType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for JointType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::PIN => "PIN", Self::GROOVE => "GROOVE", Self::DAMPED_SPRING => "DAMPED_SPRING", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[JointType::PIN, JointType::GROOVE, JointType::DAMPED_SPRING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < JointType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PIN", "JOINT_TYPE_PIN", JointType::PIN), crate::meta::inspect::EnumConstant::new("GROOVE", "JOINT_TYPE_GROOVE", JointType::GROOVE), crate::meta::inspect::EnumConstant::new("DAMPED_SPRING", "JOINT_TYPE_DAMPED_SPRING", JointType::DAMPED_SPRING), crate::meta::inspect::EnumConstant::new("MAX", "JOINT_TYPE_MAX", JointType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for JointType {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for JointType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Joint Type Pin", 0i64), EnumeratorShape::new_int("Joint Type Groove", 1i64), EnumeratorShape::new_int("Joint Type Damped Spring", 2i64), EnumeratorShape::new_int("Joint Type Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.JointType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for JointType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for JointType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for JointType {
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
impl crate::registry::property::Export for JointType {
    
}
impl crate::meta::Element for JointType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct JointParam {
    ord: i32
}
impl JointParam {
    #[doc(alias = "JOINT_PARAM_BIAS")]
    #[doc = "Godot enumerator name: `JOINT_PARAM_BIAS`"]
    pub const BIAS: JointParam = JointParam {
        ord: 0i32
    };
    #[doc(alias = "JOINT_PARAM_MAX_BIAS")]
    #[doc = "Godot enumerator name: `JOINT_PARAM_MAX_BIAS`"]
    pub const MAX_BIAS: JointParam = JointParam {
        ord: 1i32
    };
    #[doc(alias = "JOINT_PARAM_MAX_FORCE")]
    #[doc = "Godot enumerator name: `JOINT_PARAM_MAX_FORCE`"]
    pub const MAX_FORCE: JointParam = JointParam {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for JointParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("JointParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for JointParam {
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
            Self::BIAS => "BIAS", Self::MAX_BIAS => "MAX_BIAS", Self::MAX_FORCE => "MAX_FORCE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[JointParam::BIAS, JointParam::MAX_BIAS, JointParam::MAX_FORCE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < JointParam >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BIAS", "JOINT_PARAM_BIAS", JointParam::BIAS), crate::meta::inspect::EnumConstant::new("MAX_BIAS", "JOINT_PARAM_MAX_BIAS", JointParam::MAX_BIAS), crate::meta::inspect::EnumConstant::new("MAX_FORCE", "JOINT_PARAM_MAX_FORCE", JointParam::MAX_FORCE)]
        }
    }
}
impl crate::meta::GodotConvert for JointParam {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Joint Param Bias", 0i64), EnumeratorShape::new_int("Joint Param Max Bias", 1i64), EnumeratorShape::new_int("Joint Param Max Force", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.JointParam")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for JointParam {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for JointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for JointParam {
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
impl crate::registry::property::Export for JointParam {
    
}
impl crate::meta::Element for JointParam {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PinJointParam {
    ord: i32
}
impl PinJointParam {
    #[doc(alias = "PIN_JOINT_SOFTNESS")]
    #[doc = "Godot enumerator name: `PIN_JOINT_SOFTNESS`"]
    pub const SOFTNESS: PinJointParam = PinJointParam {
        ord: 0i32
    };
    #[doc(alias = "PIN_JOINT_LIMIT_UPPER")]
    #[doc = "Godot enumerator name: `PIN_JOINT_LIMIT_UPPER`"]
    pub const LIMIT_UPPER: PinJointParam = PinJointParam {
        ord: 1i32
    };
    #[doc(alias = "PIN_JOINT_LIMIT_LOWER")]
    #[doc = "Godot enumerator name: `PIN_JOINT_LIMIT_LOWER`"]
    pub const LIMIT_LOWER: PinJointParam = PinJointParam {
        ord: 2i32
    };
    #[doc(alias = "PIN_JOINT_MOTOR_TARGET_VELOCITY")]
    #[doc = "Godot enumerator name: `PIN_JOINT_MOTOR_TARGET_VELOCITY`"]
    pub const MOTOR_TARGET_VELOCITY: PinJointParam = PinJointParam {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for PinJointParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PinJointParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PinJointParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::SOFTNESS => "SOFTNESS", Self::LIMIT_UPPER => "LIMIT_UPPER", Self::LIMIT_LOWER => "LIMIT_LOWER", Self::MOTOR_TARGET_VELOCITY => "MOTOR_TARGET_VELOCITY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PinJointParam::SOFTNESS, PinJointParam::LIMIT_UPPER, PinJointParam::LIMIT_LOWER, PinJointParam::MOTOR_TARGET_VELOCITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PinJointParam >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SOFTNESS", "PIN_JOINT_SOFTNESS", PinJointParam::SOFTNESS), crate::meta::inspect::EnumConstant::new("LIMIT_UPPER", "PIN_JOINT_LIMIT_UPPER", PinJointParam::LIMIT_UPPER), crate::meta::inspect::EnumConstant::new("LIMIT_LOWER", "PIN_JOINT_LIMIT_LOWER", PinJointParam::LIMIT_LOWER), crate::meta::inspect::EnumConstant::new("MOTOR_TARGET_VELOCITY", "PIN_JOINT_MOTOR_TARGET_VELOCITY", PinJointParam::MOTOR_TARGET_VELOCITY)]
        }
    }
}
impl crate::meta::GodotConvert for PinJointParam {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Pin Joint Softness", 0i64), EnumeratorShape::new_int("Pin Joint Limit Upper", 1i64), EnumeratorShape::new_int("Pin Joint Limit Lower", 2i64), EnumeratorShape::new_int("Pin Joint Motor Target Velocity", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.PinJointParam")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PinJointParam {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PinJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PinJointParam {
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
impl crate::registry::property::Export for PinJointParam {
    
}
impl crate::meta::Element for PinJointParam {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PinJointFlag {
    ord: i32
}
impl PinJointFlag {
    #[doc(alias = "PIN_JOINT_FLAG_ANGULAR_LIMIT_ENABLED")]
    #[doc = "Godot enumerator name: `PIN_JOINT_FLAG_ANGULAR_LIMIT_ENABLED`"]
    pub const ANGULAR_LIMIT_ENABLED: PinJointFlag = PinJointFlag {
        ord: 0i32
    };
    #[doc(alias = "PIN_JOINT_FLAG_MOTOR_ENABLED")]
    #[doc = "Godot enumerator name: `PIN_JOINT_FLAG_MOTOR_ENABLED`"]
    pub const MOTOR_ENABLED: PinJointFlag = PinJointFlag {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for PinJointFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PinJointFlag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PinJointFlag {
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
            Self::ANGULAR_LIMIT_ENABLED => "ANGULAR_LIMIT_ENABLED", Self::MOTOR_ENABLED => "MOTOR_ENABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PinJointFlag::ANGULAR_LIMIT_ENABLED, PinJointFlag::MOTOR_ENABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PinJointFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ANGULAR_LIMIT_ENABLED", "PIN_JOINT_FLAG_ANGULAR_LIMIT_ENABLED", PinJointFlag::ANGULAR_LIMIT_ENABLED), crate::meta::inspect::EnumConstant::new("MOTOR_ENABLED", "PIN_JOINT_FLAG_MOTOR_ENABLED", PinJointFlag::MOTOR_ENABLED)]
        }
    }
}
impl crate::meta::GodotConvert for PinJointFlag {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Pin Joint Flag Angular Limit Enabled", 0i64), EnumeratorShape::new_int("Pin Joint Flag Motor Enabled", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.PinJointFlag")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PinJointFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PinJointFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PinJointFlag {
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
impl crate::registry::property::Export for PinJointFlag {
    
}
impl crate::meta::Element for PinJointFlag {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DampedSpringParam {
    ord: i32
}
impl DampedSpringParam {
    #[doc(alias = "DAMPED_SPRING_REST_LENGTH")]
    #[doc = "Godot enumerator name: `DAMPED_SPRING_REST_LENGTH`"]
    pub const REST_LENGTH: DampedSpringParam = DampedSpringParam {
        ord: 0i32
    };
    #[doc(alias = "DAMPED_SPRING_STIFFNESS")]
    #[doc = "Godot enumerator name: `DAMPED_SPRING_STIFFNESS`"]
    pub const STIFFNESS: DampedSpringParam = DampedSpringParam {
        ord: 1i32
    };
    #[doc(alias = "DAMPED_SPRING_DAMPING")]
    #[doc = "Godot enumerator name: `DAMPED_SPRING_DAMPING`"]
    pub const DAMPING: DampedSpringParam = DampedSpringParam {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DampedSpringParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DampedSpringParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DampedSpringParam {
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
            Self::REST_LENGTH => "REST_LENGTH", Self::STIFFNESS => "STIFFNESS", Self::DAMPING => "DAMPING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DampedSpringParam::REST_LENGTH, DampedSpringParam::STIFFNESS, DampedSpringParam::DAMPING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DampedSpringParam >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("REST_LENGTH", "DAMPED_SPRING_REST_LENGTH", DampedSpringParam::REST_LENGTH), crate::meta::inspect::EnumConstant::new("STIFFNESS", "DAMPED_SPRING_STIFFNESS", DampedSpringParam::STIFFNESS), crate::meta::inspect::EnumConstant::new("DAMPING", "DAMPED_SPRING_DAMPING", DampedSpringParam::DAMPING)]
        }
    }
}
impl crate::meta::GodotConvert for DampedSpringParam {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Damped Spring Rest Length", 0i64), EnumeratorShape::new_int("Damped Spring Stiffness", 1i64), EnumeratorShape::new_int("Damped Spring Damping", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.DampedSpringParam")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DampedSpringParam {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DampedSpringParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DampedSpringParam {
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
impl crate::registry::property::Export for DampedSpringParam {
    
}
impl crate::meta::Element for DampedSpringParam {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `CCDMode`."]
pub struct CcdMode {
    ord: i32
}
impl CcdMode {
    #[doc(alias = "CCD_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `CCD_MODE_DISABLED`"]
    pub const DISABLED: CcdMode = CcdMode {
        ord: 0i32
    };
    #[doc(alias = "CCD_MODE_CAST_RAY")]
    #[doc = "Godot enumerator name: `CCD_MODE_CAST_RAY`"]
    pub const CAST_RAY: CcdMode = CcdMode {
        ord: 1i32
    };
    #[doc(alias = "CCD_MODE_CAST_SHAPE")]
    #[doc = "Godot enumerator name: `CCD_MODE_CAST_SHAPE`"]
    pub const CAST_SHAPE: CcdMode = CcdMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CcdMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CcdMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CcdMode {
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
            Self::DISABLED => "DISABLED", Self::CAST_RAY => "CAST_RAY", Self::CAST_SHAPE => "CAST_SHAPE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CcdMode::DISABLED, CcdMode::CAST_RAY, CcdMode::CAST_SHAPE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CcdMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "CCD_MODE_DISABLED", CcdMode::DISABLED), crate::meta::inspect::EnumConstant::new("CAST_RAY", "CCD_MODE_CAST_RAY", CcdMode::CAST_RAY), crate::meta::inspect::EnumConstant::new("CAST_SHAPE", "CCD_MODE_CAST_SHAPE", CcdMode::CAST_SHAPE)]
        }
    }
}
impl crate::meta::GodotConvert for CcdMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Ccd Mode Disabled", 0i64), EnumeratorShape::new_int("Ccd Mode Cast Ray", 1i64), EnumeratorShape::new_int("Ccd Mode Cast Shape", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.CCDMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CcdMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CcdMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CcdMode {
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
impl crate::registry::property::Export for CcdMode {
    
}
impl crate::meta::Element for CcdMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AreaBodyStatus {
    ord: i32
}
impl AreaBodyStatus {
    #[doc(alias = "AREA_BODY_ADDED")]
    #[doc = "Godot enumerator name: `AREA_BODY_ADDED`"]
    pub const ADDED: AreaBodyStatus = AreaBodyStatus {
        ord: 0i32
    };
    #[doc(alias = "AREA_BODY_REMOVED")]
    #[doc = "Godot enumerator name: `AREA_BODY_REMOVED`"]
    pub const REMOVED: AreaBodyStatus = AreaBodyStatus {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AreaBodyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AreaBodyStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AreaBodyStatus {
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
            Self::ADDED => "ADDED", Self::REMOVED => "REMOVED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AreaBodyStatus::ADDED, AreaBodyStatus::REMOVED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AreaBodyStatus >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ADDED", "AREA_BODY_ADDED", AreaBodyStatus::ADDED), crate::meta::inspect::EnumConstant::new("REMOVED", "AREA_BODY_REMOVED", AreaBodyStatus::REMOVED)]
        }
    }
}
impl crate::meta::GodotConvert for AreaBodyStatus {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Area Body Added", 0i64), EnumeratorShape::new_int("Area Body Removed", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.AreaBodyStatus")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AreaBodyStatus {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AreaBodyStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AreaBodyStatus {
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
impl crate::registry::property::Export for AreaBodyStatus {
    
}
impl crate::meta::Element for AreaBodyStatus {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProcessInfo {
    ord: i32
}
impl ProcessInfo {
    #[doc(alias = "INFO_ACTIVE_OBJECTS")]
    #[doc = "Godot enumerator name: `INFO_ACTIVE_OBJECTS`"]
    pub const ACTIVE_OBJECTS: ProcessInfo = ProcessInfo {
        ord: 0i32
    };
    #[doc(alias = "INFO_COLLISION_PAIRS")]
    #[doc = "Godot enumerator name: `INFO_COLLISION_PAIRS`"]
    pub const COLLISION_PAIRS: ProcessInfo = ProcessInfo {
        ord: 1i32
    };
    #[doc(alias = "INFO_ISLAND_COUNT")]
    #[doc = "Godot enumerator name: `INFO_ISLAND_COUNT`"]
    pub const ISLAND_COUNT: ProcessInfo = ProcessInfo {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ProcessInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ProcessInfo") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ProcessInfo {
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
            Self::ACTIVE_OBJECTS => "ACTIVE_OBJECTS", Self::COLLISION_PAIRS => "COLLISION_PAIRS", Self::ISLAND_COUNT => "ISLAND_COUNT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ProcessInfo::ACTIVE_OBJECTS, ProcessInfo::COLLISION_PAIRS, ProcessInfo::ISLAND_COUNT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ProcessInfo >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ACTIVE_OBJECTS", "INFO_ACTIVE_OBJECTS", ProcessInfo::ACTIVE_OBJECTS), crate::meta::inspect::EnumConstant::new("COLLISION_PAIRS", "INFO_COLLISION_PAIRS", ProcessInfo::COLLISION_PAIRS), crate::meta::inspect::EnumConstant::new("ISLAND_COUNT", "INFO_ISLAND_COUNT", ProcessInfo::ISLAND_COUNT)]
        }
    }
}
impl crate::meta::GodotConvert for ProcessInfo {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Info Active Objects", 0i64), EnumeratorShape::new_int("Info Collision Pairs", 1i64), EnumeratorShape::new_int("Info Island Count", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer2D.ProcessInfo")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ProcessInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ProcessInfo {
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
impl crate::registry::property::Export for ProcessInfo {
    
}
impl crate::meta::Element for ProcessInfo {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicsServer2D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PhysicsServer2D {
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