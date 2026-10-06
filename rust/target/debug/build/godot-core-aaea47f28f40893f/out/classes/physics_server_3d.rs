#![doc = "Sidecar module for class [`PhysicsServer3D`][crate::classes::PhysicsServer3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsServer3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsServer3D`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`physics_server_3d`][crate::classes::physics_server_3d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `PhysicsServer3D`](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nPhysicsServer3D is the server responsible for all 3D physics. It can directly create and manipulate all physics objects:\n\n- A _space_ is a self-contained world for a physics simulation. It contains bodies, areas, and joints. Its state can be queried for collision and intersection information, and several parameters of the simulation can be modified.\n\n- A _shape_ is a geometric shape such as a sphere, a box, a cylinder, or a polygon. It can be used for collision detection by adding it to a body/area, possibly with an extra transformation relative to the body/area's origin. Bodies/areas can have multiple (transformed) shapes added to them, and a single shape can be added to bodies/areas multiple times with different local transformations.\n\n- A _body_ is a physical object which can be in static, kinematic, or rigid mode. Its state (such as position and velocity) can be queried and updated. A force integration callback can be set to customize the body's physics.\n\n- An _area_ is a region in space which can be used to detect bodies and areas entering and exiting it. A body monitoring callback can be set to report entering/exiting body shapes, and similarly an area monitoring callback can be set. Gravity and damping can be overridden within the area by setting area parameters.\n\n- A _joint_ is a constraint, either between two bodies or on one body relative to a point. Parameters such as the joint bias and the rest length of a spring joint can be adjusted.\n\nPhysics objects in `PhysicsServer3D` may be created and manipulated independently; they do not have to be tied to nodes in the scene tree.\n\n**Note:** All the 3D physics nodes use the physics server internally. Adding a physics node to the scene tree will cause a corresponding physics object to be created in the physics server. A rigid body node registers a callback that updates the node's transform with the transform of the respective body object in the physics server (every physics update). An area node registers a callback to inform the area node about overlaps with the respective area object in the physics server. The raycast node queries the direct state of the relevant space in the physics server."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsServer3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PhysicsServer3D {
        #[doc = "Creates a 3D world boundary shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer3D::shape_set_data`] to set the shape's normal direction and distance properties."]
        pub fn world_boundary_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "world_boundary_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 3D separation ray shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer3D::shape_set_data`] to set the shape's `length` and `slide_on_slope` properties."]
        pub fn separation_ray_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "separation_ray_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 3D sphere shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer3D::shape_set_data`] to set the sphere's radius."]
        pub fn sphere_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "sphere_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 3D box shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer3D::shape_set_data`] to set the box's half-extents."]
        pub fn box_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "box_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 3D capsule shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer3D::shape_set_data`] to set the capsule's height and radius."]
        pub fn capsule_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "capsule_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 3D cylinder shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer3D::shape_set_data`] to set the cylinder's height and radius."]
        pub fn cylinder_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "cylinder_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 3D convex polygon shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer3D::shape_set_data`] to set the convex polygon's points."]
        pub fn convex_polygon_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "convex_polygon_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 3D concave polygon shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer3D::shape_set_data`] to set the concave polygon's triangles."]
        pub fn concave_polygon_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(619usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "concave_polygon_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 3D heightmap shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer3D::shape_set_data`] to set the heightmap's data."]
        pub fn heightmap_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "heightmap_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a custom shape in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. Use [`shape_set_data`][`crate::classes::PhysicsServer3D::shape_set_data`] to set the shape's data.\n\n**Note:** Custom shapes are not supported by the built-in physics servers, so calling this method always produces an error when using Godot Physics or Jolt Physics. Custom physics servers implemented as GDExtensions may support a custom shape."]
        pub fn custom_shape_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "custom_shape_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the shape data that configures the shape. The `data` to be passed depends on the shape's type (see [`shape_get_type`][`crate::classes::PhysicsServer3D::shape_get_type`]):\n\n- [`ShapeType::WORLD_BOUNDARY`][`crate::classes::physics_server_3d::ShapeType::WORLD_BOUNDARY`]: a [`Plane`][crate::builtin::Plane],\n\n- [`ShapeType::SEPARATION_RAY`][`crate::classes::physics_server_3d::ShapeType::SEPARATION_RAY`]: a dictionary containing the key `\"length\"` with a `float` value and the key `\"slide_on_slope\"` with a `bool` value,\n\n- [`ShapeType::SPHERE`][`crate::classes::physics_server_3d::ShapeType::SPHERE`]: a `float` that is the radius of the sphere,\n\n- [`ShapeType::BOX`][`crate::classes::physics_server_3d::ShapeType::BOX`]: a [`Vector3`][crate::builtin::Vector3] containing the half-extents of the box,\n\n- [`ShapeType::CAPSULE`][`crate::classes::physics_server_3d::ShapeType::CAPSULE`]: a dictionary containing the keys `\"height\"` and `\"radius\"` with `float` values,\n\n- [`ShapeType::CYLINDER`][`crate::classes::physics_server_3d::ShapeType::CYLINDER`]: a dictionary containing the keys `\"height\"` and `\"radius\"` with `float` values,\n\n- [`ShapeType::CONVEX_POLYGON`][`crate::classes::physics_server_3d::ShapeType::CONVEX_POLYGON`]: a [`PackedVector3Array`][crate::builtin::PackedVector3Array] of points defining a convex polygon (the shape will be the convex hull of the points),\n\n- [`ShapeType::CONCAVE_POLYGON`][`crate::classes::physics_server_3d::ShapeType::CONCAVE_POLYGON`]: a dictionary containing the key `\"faces\"` with a [`PackedVector3Array`][crate::builtin::PackedVector3Array] value (with a length divisible by 3, so that each 3-tuple of points forms a face) and the key `\"backface_collision\"` with a `bool` value,\n\n- [`ShapeType::HEIGHTMAP`][`crate::classes::physics_server_3d::ShapeType::HEIGHTMAP`]: a dictionary containing the keys `\"width\"` and `\"depth\"` with `int` values, and the key `\"heights\"` with a value that is a packed array of `float`s of length `width * depth` (that is a [`PackedFloat32Array`][crate::builtin::PackedFloat32Array], or a [`PackedFloat64Array`][crate::builtin::PackedFloat64Array] if Godot was compiled with the `precision=double` option), and optionally the keys `\"min_height\"` and `\"max_height\"` with `float` values,\n\n- [`ShapeType::SOFT_BODY`][`crate::classes::physics_server_3d::ShapeType::SOFT_BODY`]: the input `data` is ignored and this method has no effect,\n\n- [`ShapeType::CUSTOM`][`crate::classes::physics_server_3d::ShapeType::CUSTOM`]: the input `data` is interpreted by a custom physics server, if it supports custom shapes."]
        pub fn shape_set_data(&mut self, shape: Rid, data: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (shape, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(622usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "shape_set_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the collision margin for the shape.\n\n**Note:** This is not used in Godot Physics."]
        pub fn shape_set_margin(&mut self, shape: Rid, margin: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (shape, margin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "shape_set_margin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the shape's type."]
        pub fn shape_get_type(&self, shape: Rid,) -> crate::classes::physics_server_3d::ShapeType {
            type CallRet = crate::classes::physics_server_3d::ShapeType;
            type CallParams = (Rid,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "shape_get_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the shape data that configures the shape, such as the half-extents of a box or the triangles of a concave (trimesh) shape. See [`shape_set_data`][`crate::classes::PhysicsServer3D::shape_set_data`] for the precise format of this data in each case."]
        pub fn shape_get_data(&self, shape: Rid,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "shape_get_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the collision margin for the shape.\n\n**Note:** This is not used in Godot Physics, so will always return `0`."]
        pub fn shape_get_margin(&self, shape: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "shape_get_margin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a space. A space is a collection of parameters for the physics engine that can be assigned to an area or a body. It can be assigned to an area with [`area_set_space`][`crate::classes::PhysicsServer3D::area_set_space`], or to a body with [`body_set_space`][`crate::classes::PhysicsServer3D::body_set_space`]."]
        pub fn space_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Marks a space as active. It will not have an effect, unless it is assigned to an area or body."]
        pub fn space_set_active(&mut self, space: Rid, active: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (space, active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_set_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the space is active."]
        pub fn space_is_active(&self, space: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_is_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value for a space parameter. A list of available parameters is on the \\[enum SpaceParameter] constants."]
        pub fn space_set_param(&mut self, space: Rid, param: crate::classes::physics_server_3d::SpaceParameter, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_3d::SpaceParameter, f32,);
            let args = (space, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of a space parameter."]
        pub fn space_get_param(&self, space: Rid, param: crate::classes::physics_server_3d::SpaceParameter,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, crate::classes::physics_server_3d::SpaceParameter,);
            let args = (space, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the state of a space, a [`PhysicsDirectSpaceState3D`][crate::classes::PhysicsDirectSpaceState3D]. This object can be used to make collision/intersection queries."]
        pub fn space_get_direct_state(&mut self, space: Rid,) -> Option < Gd < crate::classes::PhysicsDirectSpaceState3D > > {
            type CallRet = Option < Gd < crate::classes::PhysicsDirectSpaceState3D > >;
            type CallParams = (Rid,);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "space_get_direct_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 3D area object in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. The default settings for the created area include a collision layer and mask set to `1`, and `monitorable` set to `false`.\n\nUse [`area_add_shape`][`crate::classes::PhysicsServer3D::area_add_shape`] to add shapes to it, use [`area_set_transform`][`crate::classes::PhysicsServer3D::area_set_transform`] to set its transform, and use [`area_set_space`][`crate::classes::PhysicsServer3D::area_set_space`] to add the area to a space. If you want the area to be detectable use [`area_set_monitorable`][`crate::classes::PhysicsServer3D::area_set_monitorable`]."]
        pub fn area_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns a space to the area."]
        pub fn area_set_space(&mut self, area: Rid, space: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (area, space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the space assigned to the area."]
        pub fn area_get_space(&self, area: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a shape to the area, along with a transform matrix. Shapes are usually referenced by their index, so you should track which shape has a given index."]
        pub(crate) fn area_add_shape_full(&mut self, area: Rid, shape: Rid, transform: Transform3D, disabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform3D, bool,);
            let args = (area, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_add_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`area_add_shape_ex`][Self::area_add_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a shape to the area, along with a transform matrix. Shapes are usually referenced by their index, so you should track which shape has a given index."]
        #[inline]
        pub fn area_add_shape(&mut self, area: Rid, shape: Rid,) {
            self.area_add_shape_ex(area, shape,) . done()
        }
        #[doc = "Adds a shape to the area, along with a transform matrix. Shapes are usually referenced by their index, so you should track which shape has a given index."]
        #[inline]
        pub fn area_add_shape_ex < 'ex > (&'ex mut self, area: Rid, shape: Rid,) -> ExAreaAddShape < 'ex > {
            ExAreaAddShape::new(self, area, shape,)
        }
        #[doc = "Substitutes a given area shape by another. The old shape is selected by its index, the new one by its [`RID`][crate::builtin::Rid]."]
        pub fn area_set_shape(&mut self, area: Rid, shape_idx: i32, shape: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (area, shape_idx, shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the transform matrix for an area shape."]
        pub fn area_set_shape_transform(&mut self, area: Rid, shape_idx: i32, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform3D,);
            let args = (area, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_shape_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn area_set_shape_disabled(&mut self, area: Rid, shape_idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (area, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_shape_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of shapes assigned to an area."]
        pub fn area_get_shape_count(&self, area: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_shape_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the nth shape of an area."]
        pub fn area_get_shape(&self, area: Rid, shape_idx: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i32,);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform matrix of a shape within an area."]
        pub fn area_get_shape_transform(&self, area: Rid, shape_idx: i32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (Rid, i32,);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_shape_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a shape from an area. It does not delete the shape, so it can be reassigned later."]
        pub fn area_remove_shape(&mut self, area: Rid, shape_idx: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (area, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_remove_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all shapes from an area. It does not delete the shapes, so they can be reassigned later."]
        pub fn area_clear_shapes(&mut self, area: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_clear_shapes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns the area to one or many physics layers."]
        pub fn area_set_collision_layer(&mut self, area: Rid, layer: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (area, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics layer or layers an area belongs to."]
        pub fn area_get_collision_layer(&self, area: Rid,) -> u32 {
            type CallRet = u32;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets which physics layers the area will monitor."]
        pub fn area_set_collision_mask(&mut self, area: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (area, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics layer or layers an area can contact with."]
        pub fn area_get_collision_mask(&self, area: Rid,) -> u32 {
            type CallRet = u32;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value for an area parameter. A list of available parameters is on the \\[enum AreaParameter] constants."]
        pub fn area_set_param(&mut self, area: Rid, param: crate::classes::physics_server_3d::AreaParameter, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, crate::classes::physics_server_3d::AreaParameter, RefArg < 'a0, Variant >,);
            let args = (area, param, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the transform matrix for an area."]
        pub fn area_set_transform(&mut self, area: Rid, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform3D,);
            let args = (area, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an area parameter value. A list of available parameters is on the \\[enum AreaParameter] constants."]
        pub fn area_get_param(&self, area: Rid, param: crate::classes::physics_server_3d::AreaParameter,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, crate::classes::physics_server_3d::AreaParameter,);
            let args = (area, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform matrix for an area."]
        pub fn area_get_transform(&self, area: Rid,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns the area to a descendant of [`Object`][crate::classes::Object], so it can exist in the node tree."]
        pub fn area_attach_object_instance_id(&mut self, area: Rid, id: u64,) {
            type CallRet = ();
            type CallParams = (Rid, u64,);
            let args = (area, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_attach_object_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the instance ID of the object the area is assigned to."]
        pub fn area_get_object_instance_id(&self, area: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_get_object_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the area's body monitor callback. This callback will be called when any other (shape of a) body enters or exits (a shape of) the given area, and must take the following five parameters:\n\n1. an integer `status`: either [`AreaBodyStatus::ADDED`][`crate::classes::physics_server_3d::AreaBodyStatus::ADDED`] or [`AreaBodyStatus::REMOVED`][`crate::classes::physics_server_3d::AreaBodyStatus::REMOVED`] depending on whether the other body shape entered or exited the area,\n\n2. an [`RID`][crate::builtin::Rid] `body_rid`: the [`RID`][crate::builtin::Rid] of the body that entered or exited the area,\n\n3. an integer `instance_id`: the `ObjectID` attached to the body,\n\n4. an integer `body_shape_idx`: the index of the shape of the body that entered or exited the area,\n\n5. an integer `self_shape_idx`: the index of the shape of the area where the body entered or exited.\n\nBy counting (or keeping track of) the shapes that enter and exit, it can be determined if a body (with all its shapes) is entering for the first time or exiting for the last time."]
        pub fn area_set_monitor_callback(&mut self, area: Rid, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Callable >,);
            let args = (area, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_monitor_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the area's area monitor callback. This callback will be called when any other (shape of an) area enters or exits (a shape of) the given area, and must take the following five parameters:\n\n1. an integer `status`: either [`AreaBodyStatus::ADDED`][`crate::classes::physics_server_3d::AreaBodyStatus::ADDED`] or [`AreaBodyStatus::REMOVED`][`crate::classes::physics_server_3d::AreaBodyStatus::REMOVED`] depending on whether the other area's shape entered or exited the area,\n\n2. an [`RID`][crate::builtin::Rid] `area_rid`: the [`RID`][crate::builtin::Rid] of the other area that entered or exited the area,\n\n3. an integer `instance_id`: the `ObjectID` attached to the other area,\n\n4. an integer `area_shape_idx`: the index of the shape of the other area that entered or exited the area,\n\n5. an integer `self_shape_idx`: the index of the shape of the area where the other area entered or exited.\n\nBy counting (or keeping track of) the shapes that enter and exit, it can be determined if an area (with all its shapes) is entering for the first time or exiting for the last time."]
        pub fn area_set_area_monitor_callback(&mut self, area: Rid, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Callable >,);
            let args = (area, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_area_monitor_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn area_set_monitorable(&mut self, area: Rid, monitorable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (area, monitorable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_monitorable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets object pickable with rays."]
        pub fn area_set_ray_pickable(&mut self, area: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (area, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "area_set_ray_pickable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 3D body object in the physics server, and returns the [`RID`][crate::builtin::Rid] that identifies it. The default settings for the created area include a collision layer and mask set to `1`, and body mode set to [`BodyMode::RIGID`][`crate::classes::physics_server_3d::BodyMode::RIGID`].\n\nUse [`body_add_shape`][`crate::classes::PhysicsServer3D::body_add_shape`] to add shapes to it, use [`body_set_state`][`crate::classes::PhysicsServer3D::body_set_state`] to set its transform, and use [`body_set_space`][`crate::classes::PhysicsServer3D::body_set_space`] to add the body to a space."]
        pub fn body_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns a space to the body (see [`space_create`][`crate::classes::PhysicsServer3D::space_create`])."]
        pub fn body_set_space(&mut self, body: Rid, space: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (body, space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the space assigned to a body."]
        pub fn body_get_space(&self, body: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body mode."]
        pub fn body_set_mode(&mut self, body: Rid, mode: crate::classes::physics_server_3d::BodyMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_3d::BodyMode,);
            let args = (body, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body mode."]
        pub fn body_get_mode(&self, body: Rid,) -> crate::classes::physics_server_3d::BodyMode {
            type CallRet = crate::classes::physics_server_3d::BodyMode;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the physics layer or layers a body belongs to."]
        pub fn body_set_collision_layer(&mut self, body: Rid, layer: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (body, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics layer or layers a body belongs to."]
        pub fn body_get_collision_layer(&self, body: Rid,) -> u32 {
            type CallRet = u32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the physics layer or layers a body can collide with."]
        pub fn body_set_collision_mask(&mut self, body: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (body, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics layer or layers a body can collide with."]
        pub fn body_get_collision_mask(&self, body: Rid,) -> u32 {
            type CallRet = u32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's collision priority."]
        pub fn body_set_collision_priority(&mut self, body: Rid, priority: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, priority,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body's collision priority."]
        pub fn body_get_collision_priority(&self, body: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a shape to the body, along with a transform matrix. Shapes are usually referenced by their index, so you should track which shape has a given index."]
        pub(crate) fn body_add_shape_full(&mut self, body: Rid, shape: Rid, transform: Transform3D, disabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform3D, bool,);
            let args = (body, shape, transform, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_add_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_add_shape_ex`][Self::body_add_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a shape to the body, along with a transform matrix. Shapes are usually referenced by their index, so you should track which shape has a given index."]
        #[inline]
        pub fn body_add_shape(&mut self, body: Rid, shape: Rid,) {
            self.body_add_shape_ex(body, shape,) . done()
        }
        #[doc = "Adds a shape to the body, along with a transform matrix. Shapes are usually referenced by their index, so you should track which shape has a given index."]
        #[inline]
        pub fn body_add_shape_ex < 'ex > (&'ex mut self, body: Rid, shape: Rid,) -> ExBodyAddShape < 'ex > {
            ExBodyAddShape::new(self, body, shape,)
        }
        #[doc = "Substitutes a given body shape by another. The old shape is selected by its index, the new one by its [`RID`][crate::builtin::Rid]."]
        pub fn body_set_shape(&mut self, body: Rid, shape_idx: i32, shape: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (body, shape_idx, shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the transform matrix for a body shape."]
        pub fn body_set_shape_transform(&mut self, body: Rid, shape_idx: i32, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform3D,);
            let args = (body, shape_idx, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_shape_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn body_set_shape_disabled(&mut self, body: Rid, shape_idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (body, shape_idx, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_shape_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of shapes assigned to a body."]
        pub fn body_get_shape_count(&self, body: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_shape_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the nth shape of a body."]
        pub fn body_get_shape(&self, body: Rid, shape_idx: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i32,);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform matrix of a body shape."]
        pub fn body_get_shape_transform(&self, body: Rid, shape_idx: i32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (Rid, i32,);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_shape_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a shape from a body. The shape is not deleted, so it can be reused afterwards."]
        pub fn body_remove_shape(&mut self, body: Rid, shape_idx: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (body, shape_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_remove_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all shapes from a body."]
        pub fn body_clear_shapes(&mut self, body: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_clear_shapes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns the area to a descendant of [`Object`][crate::classes::Object], so it can exist in the node tree."]
        pub fn body_attach_object_instance_id(&mut self, body: Rid, id: u64,) {
            type CallRet = ();
            type CallParams = (Rid, u64,);
            let args = (body, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_attach_object_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the instance ID of the object the area is assigned to."]
        pub fn body_get_object_instance_id(&self, body: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_object_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the continuous collision detection mode is enabled.\n\nContinuous collision detection tries to predict where a moving body will collide, instead of moving it and correcting its movement if it collided."]
        pub fn body_set_enable_continuous_collision_detection(&mut self, body: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_enable_continuous_collision_detection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the continuous collision detection mode is enabled."]
        pub fn body_is_continuous_collision_detection_enabled(&self, body: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_is_continuous_collision_detection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a body parameter. A list of available parameters is on the \\[enum BodyParameter] constants."]
        pub fn body_set_param(&mut self, body: Rid, param: crate::classes::physics_server_3d::BodyParameter, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, crate::classes::physics_server_3d::BodyParameter, RefArg < 'a0, Variant >,);
            let args = (body, param, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of a body parameter. A list of available parameters is on the \\[enum BodyParameter] constants."]
        pub fn body_get_param(&self, body: Rid, param: crate::classes::physics_server_3d::BodyParameter,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, crate::classes::physics_server_3d::BodyParameter,);
            let args = (body, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Restores the default inertia and center of mass based on shapes to cancel any custom values previously set using [`body_set_param`][`crate::classes::PhysicsServer3D::body_set_param`]."]
        pub fn body_reset_mass_properties(&mut self, body: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_reset_mass_properties", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a body state."]
        pub fn body_set_state(&mut self, body: Rid, state: crate::classes::physics_server_3d::BodyState, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, crate::classes::physics_server_3d::BodyState, RefArg < 'a0, Variant >,);
            let args = (body, state, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a body state."]
        pub fn body_get_state(&self, body: Rid, state: crate::classes::physics_server_3d::BodyState,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, crate::classes::physics_server_3d::BodyState,);
            let args = (body, state,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a directional impulse without affecting rotation.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\nThis is equivalent to using [`body_apply_impulse`][`crate::classes::PhysicsServer3D::body_apply_impulse`] at the body's center of mass."]
        pub fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_central_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a positioned impulse to the body.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn body_apply_impulse_full(&mut self, body: Rid, impulse: Vector3, position: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3, Vector3,);
            let args = (body, impulse, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_apply_impulse_ex`][Self::body_apply_impulse_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a positioned impulse to the body.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_apply_impulse(&mut self, body: Rid, impulse: Vector3,) {
            self.body_apply_impulse_ex(body, impulse,) . done()
        }
        #[doc = "Applies a positioned impulse to the body.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_apply_impulse_ex < 'ex > (&'ex mut self, body: Rid, impulse: Vector3,) -> ExBodyApplyImpulse < 'ex > {
            ExBodyApplyImpulse::new(self, body, impulse,)
        }
        #[doc = "Applies a rotational impulse to the body without affecting the position.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise)."]
        pub fn body_apply_torque_impulse(&mut self, body: Rid, impulse: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_torque_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a directional force without affecting rotation. A force is time dependent and meant to be applied every physics update.\n\nThis is equivalent to using [`body_apply_force`][`crate::classes::PhysicsServer3D::body_apply_force`] at the body's center of mass."]
        pub fn body_apply_central_force(&mut self, body: Rid, force: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_central_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a positioned force to the body. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn body_apply_force_full(&mut self, body: Rid, force: Vector3, position: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3, Vector3,);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_apply_force_ex`][Self::body_apply_force_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a positioned force to the body. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_apply_force(&mut self, body: Rid, force: Vector3,) {
            self.body_apply_force_ex(body, force,) . done()
        }
        #[doc = "Applies a positioned force to the body. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_apply_force_ex < 'ex > (&'ex mut self, body: Rid, force: Vector3,) -> ExBodyApplyForce < 'ex > {
            ExBodyApplyForce::new(self, body, force,)
        }
        #[doc = "Applies a rotational force without affecting position. A force is time dependent and meant to be applied every physics update."]
        pub fn body_apply_torque(&mut self, body: Rid, torque: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_apply_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a constant directional force without affecting rotation that keeps being applied over time until cleared with `body_set_constant_force(body, Vector3(0, 0, 0))`.\n\nThis is equivalent to using [`body_add_constant_force`][`crate::classes::PhysicsServer3D::body_add_constant_force`] at the body's center of mass."]
        pub fn body_add_constant_central_force(&mut self, body: Rid, force: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_add_constant_central_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a constant positioned force to the body that keeps being applied over time until cleared with `body_set_constant_force(body, Vector3(0, 0, 0))`.\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn body_add_constant_force_full(&mut self, body: Rid, force: Vector3, position: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3, Vector3,);
            let args = (body, force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_add_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_add_constant_force_ex`][Self::body_add_constant_force_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a constant positioned force to the body that keeps being applied over time until cleared with `body_set_constant_force(body, Vector3(0, 0, 0))`.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_add_constant_force(&mut self, body: Rid, force: Vector3,) {
            self.body_add_constant_force_ex(body, force,) . done()
        }
        #[doc = "Adds a constant positioned force to the body that keeps being applied over time until cleared with `body_set_constant_force(body, Vector3(0, 0, 0))`.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn body_add_constant_force_ex < 'ex > (&'ex mut self, body: Rid, force: Vector3,) -> ExBodyAddConstantForce < 'ex > {
            ExBodyAddConstantForce::new(self, body, force,)
        }
        #[doc = "Adds a constant rotational force without affecting position that keeps being applied over time until cleared with `body_set_constant_torque(body, Vector3(0, 0, 0))`."]
        pub fn body_add_constant_torque(&mut self, body: Rid, torque: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_add_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's total constant positional forces applied during each physics update.\n\nSee [`body_add_constant_force`][`crate::classes::PhysicsServer3D::body_add_constant_force`] and [`body_add_constant_central_force`][`crate::classes::PhysicsServer3D::body_add_constant_central_force`]."]
        pub fn body_set_constant_force(&mut self, body: Rid, force: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body's total constant positional forces applied during each physics update.\n\nSee [`body_add_constant_force`][`crate::classes::PhysicsServer3D::body_add_constant_force`] and [`body_add_constant_central_force`][`crate::classes::PhysicsServer3D::body_add_constant_central_force`]."]
        pub fn body_get_constant_force(&self, body: Rid,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's total constant rotational forces applied during each physics update.\n\nSee [`body_add_constant_torque`][`crate::classes::PhysicsServer3D::body_add_constant_torque`]."]
        pub fn body_set_constant_torque(&mut self, body: Rid, torque: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (body, torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body's total constant rotational forces applied during each physics update.\n\nSee [`body_add_constant_torque`][`crate::classes::PhysicsServer3D::body_add_constant_torque`]."]
        pub fn body_get_constant_torque(&self, body: Rid,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets an axis velocity. The velocity in the given vector axis will be set as the given vector length. This is useful for jumping behavior."]
        pub fn body_set_axis_velocity(&mut self, body: Rid, axis_velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (body, axis_velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_axis_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn body_set_axis_lock(&mut self, body: Rid, axis: crate::classes::physics_server_3d::BodyAxis, lock: bool,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_3d::BodyAxis, bool,);
            let args = (body, axis, lock,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_axis_lock", Some(self.__validated_obj()), args,)
            }
        }
        pub fn body_is_axis_locked(&self, body: Rid, axis: crate::classes::physics_server_3d::BodyAxis,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, crate::classes::physics_server_3d::BodyAxis,);
            let args = (body, axis,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_is_axis_locked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a body to the list of bodies exempt from collisions."]
        pub fn body_add_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_add_collision_exception", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a body from the list of bodies exempt from collisions.\n\nContinuous collision detection tries to predict where a moving body will collide, instead of moving it and correcting its movement if it collided."]
        pub fn body_remove_collision_exception(&mut self, body: Rid, excepted_body: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (body, excepted_body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_remove_collision_exception", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the maximum contacts to report. Bodies can keep a log of the contacts with other bodies. This is enabled by setting the maximum number of contacts reported to a number greater than 0."]
        pub fn body_set_max_contacts_reported(&mut self, body: Rid, amount: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (body, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_max_contacts_reported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the maximum contacts that can be reported. See [`body_set_max_contacts_reported`][`crate::classes::PhysicsServer3D::body_set_max_contacts_reported`]."]
        pub fn body_get_max_contacts_reported(&self, body: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_max_contacts_reported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the body omits the standard force integration. If `enable` is `true`, the body will not automatically use applied forces, torques, and damping to update the body's linear and angular velocity. In this case, [`body_set_force_integration_callback`][`crate::classes::PhysicsServer3D::body_set_force_integration_callback`] can be used to manually update the linear and angular velocity instead.\n\nThis method is called when the property \\[member RigidBody3D.custom_integrator] is set."]
        pub fn body_set_omit_force_integration(&mut self, body: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_omit_force_integration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the body is omitting the standard force integration. See [`body_set_omit_force_integration`][`crate::classes::PhysicsServer3D::body_set_omit_force_integration`]."]
        pub fn body_is_omitting_force_integration(&self, body: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_is_omitting_force_integration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's state synchronization callback function to `callable`. Use an empty [`Callable`][crate::builtin::Callable] (`Callable()`) to clear the callback.\n\nThe function `callable` will be called every physics frame, assuming that the body was active during the previous physics tick, and can be used to fetch the latest state from the physics server.\n\nThe function `callable` must take the following parameters:\n\n1. `state`: a [`PhysicsDirectBodyState3D`][crate::classes::PhysicsDirectBodyState3D], used to retrieve the body's state."]
        pub fn body_set_state_sync_callback(&mut self, body: Rid, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Callable >,);
            let args = (body, RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_state_sync_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's custom force integration callback function to `callable`. Use an empty [`Callable`][crate::builtin::Callable] (`Callable()`) to clear the custom callback.\n\nThe function `callable` will be called every physics tick, before the standard force integration (see [`body_set_omit_force_integration`][`crate::classes::PhysicsServer3D::body_set_omit_force_integration`]). It can be used for example to update the body's linear and angular velocity based on contact with other bodies.\n\nIf `userdata` is not `null`, the function `callable` must take the following two parameters:\n\n1. `state`: a [`PhysicsDirectBodyState3D`][crate::classes::PhysicsDirectBodyState3D], used to retrieve and modify the body's state,\n\n2. `userdata`: a [`Variant`][crate::builtin::Variant]; its value will be the `userdata` passed into this method.\n\nIf `userdata` is `null`, then `callable` must take only the `state` parameter."]
        pub(crate) fn body_set_force_integration_callback_full(&mut self, body: Rid, callable: RefArg < Callable >, userdata: RefArg < Variant >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, Callable >, RefArg < 'a1, Variant >,);
            let args = (body, callable, userdata,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_force_integration_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_set_force_integration_callback_ex`][Self::body_set_force_integration_callback_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the body's custom force integration callback function to `callable`. Use an empty [`Callable`][crate::builtin::Callable] (`Callable()`) to clear the custom callback.\n\nThe function `callable` will be called every physics tick, before the standard force integration (see [`body_set_omit_force_integration`][`crate::classes::PhysicsServer3D::body_set_omit_force_integration`]). It can be used for example to update the body's linear and angular velocity based on contact with other bodies.\n\nIf `userdata` is not `null`, the function `callable` must take the following two parameters:\n\n1. `state`: a [`PhysicsDirectBodyState3D`][crate::classes::PhysicsDirectBodyState3D], used to retrieve and modify the body's state,\n\n2. `userdata`: a [`Variant`][crate::builtin::Variant]; its value will be the `userdata` passed into this method.\n\nIf `userdata` is `null`, then `callable` must take only the `state` parameter."]
        #[inline]
        pub fn body_set_force_integration_callback(&mut self, body: Rid, callable: &Callable,) {
            self.body_set_force_integration_callback_ex(body, callable,) . done()
        }
        #[doc = "Sets the body's custom force integration callback function to `callable`. Use an empty [`Callable`][crate::builtin::Callable] (`Callable()`) to clear the custom callback.\n\nThe function `callable` will be called every physics tick, before the standard force integration (see [`body_set_omit_force_integration`][`crate::classes::PhysicsServer3D::body_set_omit_force_integration`]). It can be used for example to update the body's linear and angular velocity based on contact with other bodies.\n\nIf `userdata` is not `null`, the function `callable` must take the following two parameters:\n\n1. `state`: a [`PhysicsDirectBodyState3D`][crate::classes::PhysicsDirectBodyState3D], used to retrieve and modify the body's state,\n\n2. `userdata`: a [`Variant`][crate::builtin::Variant]; its value will be the `userdata` passed into this method.\n\nIf `userdata` is `null`, then `callable` must take only the `state` parameter."]
        #[inline]
        pub fn body_set_force_integration_callback_ex < 'ex > (&'ex mut self, body: Rid, callable: &'ex Callable,) -> ExBodySetForceIntegrationCallback < 'ex > {
            ExBodySetForceIntegrationCallback::new(self, body, callable,)
        }
        #[doc = "Sets the body pickable with rays if `enable` is set."]
        pub fn body_set_ray_pickable(&mut self, body: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_set_ray_pickable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a collision would result from moving along a motion vector from a given point in space. [`PhysicsTestMotionParameters3D`][crate::classes::PhysicsTestMotionParameters3D] is passed to set motion parameters. [`PhysicsTestMotionResult3D`][crate::classes::PhysicsTestMotionResult3D] can be passed to return additional information."]
        pub(crate) fn body_test_motion_full(&mut self, body: Rid, parameters: CowArg < Gd < crate::classes::PhysicsTestMotionParameters3D > >, result: CowArg < Option < Gd < crate::classes::PhysicsTestMotionResult3D > > >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (Rid, CowArg < 'a0, Gd < crate::classes::PhysicsTestMotionParameters3D > >, CowArg < 'a1, Option < Gd < crate::classes::PhysicsTestMotionResult3D > > >,);
            let args = (body, parameters, result,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_test_motion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`body_test_motion_ex`][Self::body_test_motion_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if a collision would result from moving along a motion vector from a given point in space. [`PhysicsTestMotionParameters3D`][crate::classes::PhysicsTestMotionParameters3D] is passed to set motion parameters. [`PhysicsTestMotionResult3D`][crate::classes::PhysicsTestMotionResult3D] can be passed to return additional information."]
        #[inline]
        pub fn body_test_motion(&mut self, body: Rid, parameters: impl AsArg < Gd < crate::classes::PhysicsTestMotionParameters3D >>,) -> bool {
            self.body_test_motion_ex(body, parameters,) . done()
        }
        #[doc = "Returns `true` if a collision would result from moving along a motion vector from a given point in space. [`PhysicsTestMotionParameters3D`][crate::classes::PhysicsTestMotionParameters3D] is passed to set motion parameters. [`PhysicsTestMotionResult3D`][crate::classes::PhysicsTestMotionResult3D] can be passed to return additional information."]
        #[inline]
        pub fn body_test_motion_ex < 'ex > (&'ex mut self, body: Rid, parameters: impl AsArg < Gd < crate::classes::PhysicsTestMotionParameters3D >> + 'ex,) -> ExBodyTestMotion < 'ex > {
            ExBodyTestMotion::new(self, body, parameters,)
        }
        #[doc = "Returns the [`PhysicsDirectBodyState3D`][crate::classes::PhysicsDirectBodyState3D] of the body. Returns `null` if the body is destroyed or removed from the physics space."]
        pub fn body_get_direct_state(&mut self, body: Rid,) -> Option < Gd < crate::classes::PhysicsDirectBodyState3D > > {
            type CallRet = Option < Gd < crate::classes::PhysicsDirectBodyState3D > >;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "body_get_direct_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new soft body and returns its internal [`RID`][crate::builtin::Rid]."]
        pub fn soft_body_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Requests that the physics server updates the rendering server with the latest positions of the given soft body's points through the `rendering_server_handler` interface."]
        pub fn soft_body_update_rendering_server(&mut self, body: Rid, rendering_server_handler: impl AsArg < Gd < crate::classes::PhysicsServer3DRenderingServerHandler >>,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, Gd < crate::classes::PhysicsServer3DRenderingServerHandler > >,);
            let args = (body, rendering_server_handler.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_update_rendering_server", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns a space to the given soft body (see [`space_create`][`crate::classes::PhysicsServer3D::space_create`])."]
        pub fn soft_body_set_space(&mut self, body: Rid, space: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (body, space,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the space assigned to the given soft body."]
        pub fn soft_body_get_space(&self, body: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(718usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the mesh of the given soft body."]
        pub fn soft_body_set_mesh(&mut self, body: Rid, mesh: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (body, mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the bounds of the given soft body in global coordinates."]
        pub fn soft_body_get_bounds(&self, body: Rid,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_bounds", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the physics layer or layers the given soft body belongs to."]
        pub fn soft_body_set_collision_layer(&mut self, body: Rid, layer: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (body, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics layer or layers that the given soft body belongs to."]
        pub fn soft_body_get_collision_layer(&self, body: Rid,) -> u32 {
            type CallRet = u32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the physics layer or layers the given soft body can collide with."]
        pub fn soft_body_set_collision_mask(&mut self, body: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (body, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(723usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics layer or layers that the given soft body can collide with."]
        pub fn soft_body_get_collision_mask(&self, body: Rid,) -> u32 {
            type CallRet = u32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds the given body to the list of bodies exempt from collisions."]
        pub fn soft_body_add_collision_exception(&mut self, body: Rid, body_b: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (body, body_b,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_add_collision_exception", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the given body from the list of bodies exempt from collisions."]
        pub fn soft_body_remove_collision_exception(&mut self, body: Rid, body_b: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (body, body_b,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_remove_collision_exception", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given body state for the given body.\n\n**Note:** Godot's default physics implementation does not support [`BodyState::LINEAR_VELOCITY`][`crate::classes::physics_server_3d::BodyState::LINEAR_VELOCITY`], [`BodyState::ANGULAR_VELOCITY`][`crate::classes::physics_server_3d::BodyState::ANGULAR_VELOCITY`], [`BodyState::SLEEPING`][`crate::classes::physics_server_3d::BodyState::SLEEPING`], or [`BodyState::CAN_SLEEP`][`crate::classes::physics_server_3d::BodyState::CAN_SLEEP`]."]
        pub fn soft_body_set_state(&mut self, body: Rid, state: crate::classes::physics_server_3d::BodyState, variant: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, crate::classes::physics_server_3d::BodyState, RefArg < 'a0, Variant >,);
            let args = (body, state, RefArg::new(variant),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(727usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given soft body state.\n\n**Note:** Godot's default physics implementation does not support [`BodyState::LINEAR_VELOCITY`][`crate::classes::physics_server_3d::BodyState::LINEAR_VELOCITY`], [`BodyState::ANGULAR_VELOCITY`][`crate::classes::physics_server_3d::BodyState::ANGULAR_VELOCITY`], [`BodyState::SLEEPING`][`crate::classes::physics_server_3d::BodyState::SLEEPING`], or [`BodyState::CAN_SLEEP`][`crate::classes::physics_server_3d::BodyState::CAN_SLEEP`]."]
        pub fn soft_body_get_state(&self, body: Rid, state: crate::classes::physics_server_3d::BodyState,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, crate::classes::physics_server_3d::BodyState,);
            let args = (body, state,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(728usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the global transform of the given soft body."]
        pub fn soft_body_set_transform(&mut self, body: Rid, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform3D,);
            let args = (body, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(729usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the given soft body will be pickable when using object picking."]
        pub fn soft_body_set_ray_pickable(&mut self, body: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (body, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(730usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_ray_pickable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the simulation precision of the given soft body. Increasing this value will improve the resulting simulation, but can affect performance. Use with care."]
        pub fn soft_body_set_simulation_precision(&mut self, body: Rid, simulation_precision: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (body, simulation_precision,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_simulation_precision", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the simulation precision of the given soft body."]
        pub fn soft_body_get_simulation_precision(&self, body: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_simulation_precision", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the total mass for the given soft body."]
        pub fn soft_body_set_total_mass(&mut self, body: Rid, total_mass: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, total_mass,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_total_mass", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total mass assigned to the given soft body."]
        pub fn soft_body_get_total_mass(&self, body: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_total_mass", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the linear stiffness of the given soft body. Higher values will result in a stiffer body, while lower values will increase the body's ability to bend. The value can be between `0.0` and `1.0` (inclusive)."]
        pub fn soft_body_set_linear_stiffness(&mut self, body: Rid, stiffness: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, stiffness,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_linear_stiffness", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the linear stiffness of the given soft body."]
        pub fn soft_body_get_linear_stiffness(&self, body: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(736usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_linear_stiffness", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the shrinking factor of the given soft body."]
        pub fn soft_body_set_shrinking_factor(&mut self, body: Rid, shrinking_factor: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, shrinking_factor,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(737usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_shrinking_factor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the shrinking factor of the given soft body."]
        pub fn soft_body_get_shrinking_factor(&self, body: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(738usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_shrinking_factor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the pressure coefficient of the given soft body. Simulates pressure build-up from inside this body. Higher values increase the strength of this effect."]
        pub fn soft_body_set_pressure_coefficient(&mut self, body: Rid, pressure_coefficient: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, pressure_coefficient,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_pressure_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the pressure coefficient of the given soft body."]
        pub fn soft_body_get_pressure_coefficient(&self, body: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_pressure_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the damping coefficient of the given soft body. Higher values will slow down the body more noticeably when forces are applied."]
        pub fn soft_body_set_damping_coefficient(&mut self, body: Rid, damping_coefficient: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, damping_coefficient,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_damping_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the damping coefficient of the given soft body."]
        pub fn soft_body_get_damping_coefficient(&self, body: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_damping_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the drag coefficient of the given soft body. Higher values increase this body's air resistance.\n\n**Note:** This value is currently unused by Godot's default physics implementation."]
        pub fn soft_body_set_drag_coefficient(&mut self, body: Rid, drag_coefficient: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (body, drag_coefficient,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_set_drag_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the drag coefficient of the given soft body."]
        pub fn soft_body_get_drag_coefficient(&self, body: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(744usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_drag_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves the given soft body point to a position in global coordinates."]
        pub fn soft_body_move_point(&mut self, body: Rid, point_index: i32, global_position: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Vector3,);
            let args = (body, point_index, global_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(745usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_move_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current position of the given soft body point in global coordinates."]
        pub fn soft_body_get_point_global_position(&self, body: Rid, point_index: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Rid, i32,);
            let args = (body, point_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(746usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_get_point_global_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unpins all points of the given soft body."]
        pub fn soft_body_remove_all_pinned_points(&mut self, body: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(747usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_remove_all_pinned_points", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Pins or unpins the given soft body point based on the value of `pin`.\n\n**Note:** Pinning a point effectively makes it kinematic, preventing it from being affected by forces, but you can still move it using [`soft_body_move_point`][`crate::classes::PhysicsServer3D::soft_body_move_point`]."]
        pub fn soft_body_pin_point(&mut self, body: Rid, point_index: i32, pin: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (body, point_index, pin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(748usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_pin_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the given soft body point is pinned."]
        pub fn soft_body_is_point_pinned(&self, body: Rid, point_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i32,);
            let args = (body, point_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(749usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_is_point_pinned", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies an impulse to a point.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise)."]
        pub fn soft_body_apply_point_impulse(&mut self, body: Rid, point_index: i32, impulse: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Vector3,);
            let args = (body, point_index, impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(750usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_apply_point_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a force to a point. A force is time dependent and meant to be applied every physics update."]
        pub fn soft_body_apply_point_force(&mut self, body: Rid, point_index: i32, force: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Vector3,);
            let args = (body, point_index, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_apply_point_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Distributes and applies an impulse to all points.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise)."]
        pub fn soft_body_apply_central_impulse(&mut self, body: Rid, impulse: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (body, impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_apply_central_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Distributes and applies a force to all points. A force is time dependent and meant to be applied every physics update."]
        pub fn soft_body_apply_central_force(&mut self, body: Rid, force: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (body, force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "soft_body_apply_central_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn joint_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn joint_clear(&mut self, joint: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn joint_make_pin(&mut self, joint: Rid, body_A: Rid, local_A: Vector3, body_B: Rid, local_B: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Vector3, Rid, Vector3,);
            let args = (joint, body_A, local_A, body_B, local_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_make_pin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a pin joint parameter."]
        pub fn pin_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_3d::PinJointParam, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_3d::PinJointParam, f32,);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a pin joint parameter."]
        pub fn pin_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_3d::PinJointParam,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, crate::classes::physics_server_3d::PinJointParam,);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets position of the joint in the local space of body a of the joint."]
        pub fn pin_joint_set_local_a(&mut self, joint: Rid, local_A: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (joint, local_A,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_set_local_a", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns position of the joint in the local space of body a of the joint."]
        pub fn pin_joint_get_local_a(&self, joint: Rid,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Rid,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_get_local_a", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets position of the joint in the local space of body b of the joint."]
        pub fn pin_joint_set_local_b(&mut self, joint: Rid, local_B: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (joint, local_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_set_local_b", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns position of the joint in the local space of body b of the joint."]
        pub fn pin_joint_get_local_b(&self, joint: Rid,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Rid,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "pin_joint_get_local_b", Some(self.__validated_obj()), args,)
            }
        }
        pub fn joint_make_hinge(&mut self, joint: Rid, body_A: Rid, hinge_A: Transform3D, body_B: Rid, hinge_B: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform3D, Rid, Transform3D,);
            let args = (joint, body_A, hinge_A, body_B, hinge_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_make_hinge", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a hinge joint parameter."]
        pub fn hinge_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_3d::HingeJointParam, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_3d::HingeJointParam, f32,);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "hinge_joint_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a hinge joint parameter."]
        pub fn hinge_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_3d::HingeJointParam,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, crate::classes::physics_server_3d::HingeJointParam,);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "hinge_joint_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a hinge joint flag."]
        pub fn hinge_joint_set_flag(&mut self, joint: Rid, flag: crate::classes::physics_server_3d::HingeJointFlag, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_3d::HingeJointFlag, bool,);
            let args = (joint, flag, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "hinge_joint_set_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a hinge joint flag."]
        pub fn hinge_joint_get_flag(&self, joint: Rid, flag: crate::classes::physics_server_3d::HingeJointFlag,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, crate::classes::physics_server_3d::HingeJointFlag,);
            let args = (joint, flag,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "hinge_joint_get_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn joint_make_slider(&mut self, joint: Rid, body_A: Rid, local_ref_A: Transform3D, body_B: Rid, local_ref_B: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform3D, Rid, Transform3D,);
            let args = (joint, body_A, local_ref_A, body_B, local_ref_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_make_slider", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a slider joint parameter."]
        pub fn slider_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_3d::SliderJointParam, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_3d::SliderJointParam, f32,);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "slider_joint_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a slider joint parameter."]
        pub fn slider_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_3d::SliderJointParam,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, crate::classes::physics_server_3d::SliderJointParam,);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "slider_joint_get_param", Some(self.__validated_obj()), args,)
            }
        }
        pub fn joint_make_cone_twist(&mut self, joint: Rid, body_A: Rid, local_ref_A: Transform3D, body_B: Rid, local_ref_B: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform3D, Rid, Transform3D,);
            let args = (joint, body_A, local_ref_A, body_B, local_ref_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_make_cone_twist", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a cone twist joint parameter."]
        pub fn cone_twist_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_3d::ConeTwistJointParam, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::physics_server_3d::ConeTwistJointParam, f32,);
            let args = (joint, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "cone_twist_joint_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a cone twist joint parameter."]
        pub fn cone_twist_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_3d::ConeTwistJointParam,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, crate::classes::physics_server_3d::ConeTwistJointParam,);
            let args = (joint, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "cone_twist_joint_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the type of the Joint3D."]
        pub fn joint_get_type(&self, joint: Rid,) -> crate::classes::physics_server_3d::JointType {
            type CallRet = crate::classes::physics_server_3d::JointType;
            type CallParams = (Rid,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_get_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the priority value of the Joint3D."]
        pub fn joint_set_solver_priority(&mut self, joint: Rid, priority: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (joint, priority,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_set_solver_priority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the priority value of the Joint3D."]
        pub fn joint_get_solver_priority(&self, joint: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_get_solver_priority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the bodies attached to the [`Joint3D`][crate::classes::Joint3D] will collide with each other."]
        pub fn joint_disable_collisions_between_bodies(&mut self, joint: Rid, disable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (joint, disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_disable_collisions_between_bodies", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the bodies attached to the [`Joint3D`][crate::classes::Joint3D] will collide with each other."]
        pub fn joint_is_disabled_collisions_between_bodies(&self, joint: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_is_disabled_collisions_between_bodies", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Make the joint a generic six degrees of freedom (6DOF) joint. Use [`generic_6dof_joint_set_flag`][`crate::classes::PhysicsServer3D::generic_6dof_joint_set_flag`] and [`generic_6dof_joint_set_param`][`crate::classes::PhysicsServer3D::generic_6dof_joint_set_param`] to set the joint's flags and parameters respectively."]
        pub fn joint_make_generic_6dof(&mut self, joint: Rid, body_A: Rid, local_ref_A: Transform3D, body_B: Rid, local_ref_B: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform3D, Rid, Transform3D,);
            let args = (joint, body_A, local_ref_A, body_B, local_ref_B,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "joint_make_generic_6dof", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of a given generic 6DOF joint parameter."]
        pub fn generic_6dof_joint_set_param(&mut self, joint: Rid, axis: Vector3Axis, param: crate::classes::physics_server_3d::G6dofJointAxisParam, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisParam, f32,);
            let args = (joint, axis, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "generic_6dof_joint_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of a generic 6DOF joint parameter."]
        pub fn generic_6dof_joint_get_param(&self, joint: Rid, axis: Vector3Axis, param: crate::classes::physics_server_3d::G6dofJointAxisParam,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisParam,);
            let args = (joint, axis, param,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "generic_6dof_joint_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of a given generic 6DOF joint flag."]
        pub fn generic_6dof_joint_set_flag(&mut self, joint: Rid, axis: Vector3Axis, flag: crate::classes::physics_server_3d::G6dofJointAxisFlag, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisFlag, bool,);
            let args = (joint, axis, flag, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "generic_6dof_joint_set_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of a generic 6DOF joint flag."]
        pub fn generic_6dof_joint_get_flag(&self, joint: Rid, axis: Vector3Axis, flag: crate::classes::physics_server_3d::G6dofJointAxisFlag,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisFlag,);
            let args = (joint, axis, flag,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "generic_6dof_joint_get_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Destroys any of the objects created by PhysicsServer3D. If the [`RID`][crate::builtin::Rid] passed is not one of the objects that can be created by PhysicsServer3D, an error will be sent to the console."]
        pub fn free_rid(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "free_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Activates or deactivates the 3D physics engine."]
        pub fn set_active(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "set_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of a physics engine state specified by `process_info`."]
        pub fn get_process_info(&self, process_info: crate::classes::physics_server_3d::ProcessInfo,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::classes::physics_server_3d::ProcessInfo,);
            let args = (process_info,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer3D", "get_process_info", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsServer3D {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsServer3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsServer3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsServer3D {
        
    }
    impl crate::obj::Singleton for PhysicsServer3D {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"PhysicsServer3D"))
            }
        }
    }
    impl std::ops::Deref for PhysicsServer3D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsServer3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsServer3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PhysicsServer3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::area_add_shape_ex`][super::PhysicsServer3D::area_add_shape_ex]."]
#[must_use]
pub struct ExAreaAddShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer3D, area: Rid, shape: Rid, transform: Transform3D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAreaAddShape < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer3D, area: Rid, shape: Rid,) -> Self {
        let transform = Transform3D::__internal_codegen(1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _);
        let disabled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, area: area, shape: shape, transform: transform, disabled: disabled,
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform3D) -> Self {
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
        re_export::PhysicsServer3D::area_add_shape_full(surround_object, area, shape, transform, disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_add_shape_ex`][super::PhysicsServer3D::body_add_shape_ex]."]
#[must_use]
pub struct ExBodyAddShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, shape: Rid, transform: Transform3D, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodyAddShape < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, shape: Rid,) -> Self {
        let transform = Transform3D::__internal_codegen(1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _);
        let disabled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, shape: shape, transform: transform, disabled: disabled,
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform3D) -> Self {
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
        re_export::PhysicsServer3D::body_add_shape_full(surround_object, body, shape, transform, disabled,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_apply_impulse_ex`][super::PhysicsServer3D::body_apply_impulse_ex]."]
#[must_use]
pub struct ExBodyApplyImpulse < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, impulse: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodyApplyImpulse < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, impulse: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
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
        re_export::PhysicsServer3D::body_apply_impulse_full(surround_object, body, impulse, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_apply_force_ex`][super::PhysicsServer3D::body_apply_force_ex]."]
#[must_use]
pub struct ExBodyApplyForce < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodyApplyForce < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, force: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
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
        re_export::PhysicsServer3D::body_apply_force_full(surround_object, body, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_add_constant_force_ex`][super::PhysicsServer3D::body_add_constant_force_ex]."]
#[must_use]
pub struct ExBodyAddConstantForce < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodyAddConstantForce < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, force: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
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
        re_export::PhysicsServer3D::body_add_constant_force_full(surround_object, body, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_set_force_integration_callback_ex`][super::PhysicsServer3D::body_set_force_integration_callback_ex]."]
#[must_use]
pub struct ExBodySetForceIntegrationCallback < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, callable: CowArg < 'ex, Callable >, userdata: CowArg < 'ex, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodySetForceIntegrationCallback < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, callable: &'ex Callable,) -> Self {
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
        re_export::PhysicsServer3D::body_set_force_integration_callback_full(surround_object, body, callable.cow_as_arg(), userdata.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`PhysicsServer3D::body_test_motion_ex`][super::PhysicsServer3D::body_test_motion_ex]."]
#[must_use]
pub struct ExBodyTestMotion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, parameters: CowArg < 'ex, Gd < crate::classes::PhysicsTestMotionParameters3D > >, result: CowArg < 'ex, Option < Gd < crate::classes::PhysicsTestMotionResult3D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBodyTestMotion < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsServer3D, body: Rid, parameters: impl AsArg < Gd < crate::classes::PhysicsTestMotionParameters3D >> + 'ex,) -> Self {
        let result = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, body: body, parameters: parameters.into_arg(), result: result.into_arg(),
        }
    }
    #[inline]
    pub fn result(self, result: impl AsArg < Option < Gd < crate::classes::PhysicsTestMotionResult3D >> > + 'ex) -> Self {
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
        re_export::PhysicsServer3D::body_test_motion_full(surround_object, body, parameters, result,)
    }
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
    #[doc(alias = "JOINT_TYPE_HINGE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_HINGE`"]
    pub const HINGE: JointType = JointType {
        ord: 1i32
    };
    #[doc(alias = "JOINT_TYPE_SLIDER")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_SLIDER`"]
    pub const SLIDER: JointType = JointType {
        ord: 2i32
    };
    #[doc(alias = "JOINT_TYPE_CONE_TWIST")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_CONE_TWIST`"]
    pub const CONE_TWIST: JointType = JointType {
        ord: 3i32
    };
    #[doc(alias = "JOINT_TYPE_6DOF")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_6DOF`"]
    pub const TYPE_6DOF: JointType = JointType {
        ord: 4i32
    };
    #[doc(alias = "JOINT_TYPE_MAX")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_MAX`"]
    pub const MAX: JointType = JointType {
        ord: 5i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::PIN => "PIN", Self::HINGE => "HINGE", Self::SLIDER => "SLIDER", Self::CONE_TWIST => "CONE_TWIST", Self::TYPE_6DOF => "TYPE_6DOF", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[JointType::PIN, JointType::HINGE, JointType::SLIDER, JointType::CONE_TWIST, JointType::TYPE_6DOF]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < JointType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PIN", "JOINT_TYPE_PIN", JointType::PIN), crate::meta::inspect::EnumConstant::new("HINGE", "JOINT_TYPE_HINGE", JointType::HINGE), crate::meta::inspect::EnumConstant::new("SLIDER", "JOINT_TYPE_SLIDER", JointType::SLIDER), crate::meta::inspect::EnumConstant::new("CONE_TWIST", "JOINT_TYPE_CONE_TWIST", JointType::CONE_TWIST), crate::meta::inspect::EnumConstant::new("TYPE_6DOF", "JOINT_TYPE_6DOF", JointType::TYPE_6DOF), crate::meta::inspect::EnumConstant::new("MAX", "JOINT_TYPE_MAX", JointType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for JointType {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for JointType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Joint Type Pin", 0i64), EnumeratorShape::new_int("Joint Type Hinge", 1i64), EnumeratorShape::new_int("Joint Type Slider", 2i64), EnumeratorShape::new_int("Joint Type Cone Twist", 3i64), EnumeratorShape::new_int("Joint Type 6dof", 4i64), EnumeratorShape::new_int("Joint Type Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.JointType")), is_bitfield: false,
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
pub struct PinJointParam {
    ord: i32
}
impl PinJointParam {
    #[doc(alias = "PIN_JOINT_BIAS")]
    #[doc = "Godot enumerator name: `PIN_JOINT_BIAS`"]
    pub const BIAS: PinJointParam = PinJointParam {
        ord: 0i32
    };
    #[doc(alias = "PIN_JOINT_DAMPING")]
    #[doc = "Godot enumerator name: `PIN_JOINT_DAMPING`"]
    pub const DAMPING: PinJointParam = PinJointParam {
        ord: 1i32
    };
    #[doc(alias = "PIN_JOINT_IMPULSE_CLAMP")]
    #[doc = "Godot enumerator name: `PIN_JOINT_IMPULSE_CLAMP`"]
    pub const IMPULSE_CLAMP: PinJointParam = PinJointParam {
        ord: 2i32
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
            Self::BIAS => "BIAS", Self::DAMPING => "DAMPING", Self::IMPULSE_CLAMP => "IMPULSE_CLAMP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PinJointParam::BIAS, PinJointParam::DAMPING, PinJointParam::IMPULSE_CLAMP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PinJointParam >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BIAS", "PIN_JOINT_BIAS", PinJointParam::BIAS), crate::meta::inspect::EnumConstant::new("DAMPING", "PIN_JOINT_DAMPING", PinJointParam::DAMPING), crate::meta::inspect::EnumConstant::new("IMPULSE_CLAMP", "PIN_JOINT_IMPULSE_CLAMP", PinJointParam::IMPULSE_CLAMP)]
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
            &[EnumeratorShape::new_int("Pin Joint Bias", 0i64), EnumeratorShape::new_int("Pin Joint Damping", 1i64), EnumeratorShape::new_int("Pin Joint Impulse Clamp", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.PinJointParam")), is_bitfield: false,
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
pub struct HingeJointParam {
    ord: i32
}
impl HingeJointParam {
    #[doc(alias = "HINGE_JOINT_BIAS")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_BIAS`"]
    pub const BIAS: HingeJointParam = HingeJointParam {
        ord: 0i32
    };
    #[doc(alias = "HINGE_JOINT_LIMIT_UPPER")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_LIMIT_UPPER`"]
    pub const LIMIT_UPPER: HingeJointParam = HingeJointParam {
        ord: 1i32
    };
    #[doc(alias = "HINGE_JOINT_LIMIT_LOWER")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_LIMIT_LOWER`"]
    pub const LIMIT_LOWER: HingeJointParam = HingeJointParam {
        ord: 2i32
    };
    #[doc(alias = "HINGE_JOINT_LIMIT_BIAS")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_LIMIT_BIAS`"]
    pub const LIMIT_BIAS: HingeJointParam = HingeJointParam {
        ord: 3i32
    };
    #[doc(alias = "HINGE_JOINT_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_LIMIT_SOFTNESS`"]
    pub const LIMIT_SOFTNESS: HingeJointParam = HingeJointParam {
        ord: 4i32
    };
    #[doc(alias = "HINGE_JOINT_LIMIT_RELAXATION")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_LIMIT_RELAXATION`"]
    pub const LIMIT_RELAXATION: HingeJointParam = HingeJointParam {
        ord: 5i32
    };
    #[doc(alias = "HINGE_JOINT_MOTOR_TARGET_VELOCITY")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_MOTOR_TARGET_VELOCITY`"]
    pub const MOTOR_TARGET_VELOCITY: HingeJointParam = HingeJointParam {
        ord: 6i32
    };
    #[doc(alias = "HINGE_JOINT_MOTOR_MAX_IMPULSE")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_MOTOR_MAX_IMPULSE`"]
    pub const MOTOR_MAX_IMPULSE: HingeJointParam = HingeJointParam {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for HingeJointParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HingeJointParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HingeJointParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::BIAS => "BIAS", Self::LIMIT_UPPER => "LIMIT_UPPER", Self::LIMIT_LOWER => "LIMIT_LOWER", Self::LIMIT_BIAS => "LIMIT_BIAS", Self::LIMIT_SOFTNESS => "LIMIT_SOFTNESS", Self::LIMIT_RELAXATION => "LIMIT_RELAXATION", Self::MOTOR_TARGET_VELOCITY => "MOTOR_TARGET_VELOCITY", Self::MOTOR_MAX_IMPULSE => "MOTOR_MAX_IMPULSE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[HingeJointParam::BIAS, HingeJointParam::LIMIT_UPPER, HingeJointParam::LIMIT_LOWER, HingeJointParam::LIMIT_BIAS, HingeJointParam::LIMIT_SOFTNESS, HingeJointParam::LIMIT_RELAXATION, HingeJointParam::MOTOR_TARGET_VELOCITY, HingeJointParam::MOTOR_MAX_IMPULSE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HingeJointParam >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BIAS", "HINGE_JOINT_BIAS", HingeJointParam::BIAS), crate::meta::inspect::EnumConstant::new("LIMIT_UPPER", "HINGE_JOINT_LIMIT_UPPER", HingeJointParam::LIMIT_UPPER), crate::meta::inspect::EnumConstant::new("LIMIT_LOWER", "HINGE_JOINT_LIMIT_LOWER", HingeJointParam::LIMIT_LOWER), crate::meta::inspect::EnumConstant::new("LIMIT_BIAS", "HINGE_JOINT_LIMIT_BIAS", HingeJointParam::LIMIT_BIAS), crate::meta::inspect::EnumConstant::new("LIMIT_SOFTNESS", "HINGE_JOINT_LIMIT_SOFTNESS", HingeJointParam::LIMIT_SOFTNESS), crate::meta::inspect::EnumConstant::new("LIMIT_RELAXATION", "HINGE_JOINT_LIMIT_RELAXATION", HingeJointParam::LIMIT_RELAXATION), crate::meta::inspect::EnumConstant::new("MOTOR_TARGET_VELOCITY", "HINGE_JOINT_MOTOR_TARGET_VELOCITY", HingeJointParam::MOTOR_TARGET_VELOCITY), crate::meta::inspect::EnumConstant::new("MOTOR_MAX_IMPULSE", "HINGE_JOINT_MOTOR_MAX_IMPULSE", HingeJointParam::MOTOR_MAX_IMPULSE)]
        }
    }
}
impl crate::meta::GodotConvert for HingeJointParam {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Hinge Joint Bias", 0i64), EnumeratorShape::new_int("Hinge Joint Limit Upper", 1i64), EnumeratorShape::new_int("Hinge Joint Limit Lower", 2i64), EnumeratorShape::new_int("Hinge Joint Limit Bias", 3i64), EnumeratorShape::new_int("Hinge Joint Limit Softness", 4i64), EnumeratorShape::new_int("Hinge Joint Limit Relaxation", 5i64), EnumeratorShape::new_int("Hinge Joint Motor Target Velocity", 6i64), EnumeratorShape::new_int("Hinge Joint Motor Max Impulse", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.HingeJointParam")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for HingeJointParam {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HingeJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for HingeJointParam {
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
impl crate::registry::property::Export for HingeJointParam {
    
}
impl crate::meta::Element for HingeJointParam {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HingeJointFlag {
    ord: i32
}
impl HingeJointFlag {
    #[doc(alias = "HINGE_JOINT_FLAG_USE_LIMIT")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_FLAG_USE_LIMIT`"]
    pub const USE_LIMIT: HingeJointFlag = HingeJointFlag {
        ord: 0i32
    };
    #[doc(alias = "HINGE_JOINT_FLAG_ENABLE_MOTOR")]
    #[doc = "Godot enumerator name: `HINGE_JOINT_FLAG_ENABLE_MOTOR`"]
    pub const ENABLE_MOTOR: HingeJointFlag = HingeJointFlag {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for HingeJointFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HingeJointFlag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HingeJointFlag {
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
            Self::USE_LIMIT => "USE_LIMIT", Self::ENABLE_MOTOR => "ENABLE_MOTOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[HingeJointFlag::USE_LIMIT, HingeJointFlag::ENABLE_MOTOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HingeJointFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("USE_LIMIT", "HINGE_JOINT_FLAG_USE_LIMIT", HingeJointFlag::USE_LIMIT), crate::meta::inspect::EnumConstant::new("ENABLE_MOTOR", "HINGE_JOINT_FLAG_ENABLE_MOTOR", HingeJointFlag::ENABLE_MOTOR)]
        }
    }
}
impl crate::meta::GodotConvert for HingeJointFlag {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Hinge Joint Flag Use Limit", 0i64), EnumeratorShape::new_int("Hinge Joint Flag Enable Motor", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.HingeJointFlag")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for HingeJointFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HingeJointFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for HingeJointFlag {
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
impl crate::registry::property::Export for HingeJointFlag {
    
}
impl crate::meta::Element for HingeJointFlag {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SliderJointParam {
    ord: i32
}
impl SliderJointParam {
    #[doc(alias = "SLIDER_JOINT_LINEAR_LIMIT_UPPER")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_LIMIT_UPPER`"]
    pub const LINEAR_LIMIT_UPPER: SliderJointParam = SliderJointParam {
        ord: 0i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_LIMIT_LOWER")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_LIMIT_LOWER`"]
    pub const LINEAR_LIMIT_LOWER: SliderJointParam = SliderJointParam {
        ord: 1i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_LIMIT_SOFTNESS`"]
    pub const LINEAR_LIMIT_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 2i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_LIMIT_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_LIMIT_RESTITUTION`"]
    pub const LINEAR_LIMIT_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 3i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_LIMIT_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_LIMIT_DAMPING`"]
    pub const LINEAR_LIMIT_DAMPING: SliderJointParam = SliderJointParam {
        ord: 4i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_MOTION_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_MOTION_SOFTNESS`"]
    pub const LINEAR_MOTION_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 5i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_MOTION_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_MOTION_RESTITUTION`"]
    pub const LINEAR_MOTION_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 6i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_MOTION_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_MOTION_DAMPING`"]
    pub const LINEAR_MOTION_DAMPING: SliderJointParam = SliderJointParam {
        ord: 7i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_ORTHOGONAL_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_ORTHOGONAL_SOFTNESS`"]
    pub const LINEAR_ORTHOGONAL_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 8i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_ORTHOGONAL_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_ORTHOGONAL_RESTITUTION`"]
    pub const LINEAR_ORTHOGONAL_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 9i32
    };
    #[doc(alias = "SLIDER_JOINT_LINEAR_ORTHOGONAL_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_LINEAR_ORTHOGONAL_DAMPING`"]
    pub const LINEAR_ORTHOGONAL_DAMPING: SliderJointParam = SliderJointParam {
        ord: 10i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_LIMIT_UPPER")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_LIMIT_UPPER`"]
    pub const ANGULAR_LIMIT_UPPER: SliderJointParam = SliderJointParam {
        ord: 11i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_LIMIT_LOWER")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_LIMIT_LOWER`"]
    pub const ANGULAR_LIMIT_LOWER: SliderJointParam = SliderJointParam {
        ord: 12i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_LIMIT_SOFTNESS`"]
    pub const ANGULAR_LIMIT_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 13i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_LIMIT_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_LIMIT_RESTITUTION`"]
    pub const ANGULAR_LIMIT_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 14i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_LIMIT_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_LIMIT_DAMPING`"]
    pub const ANGULAR_LIMIT_DAMPING: SliderJointParam = SliderJointParam {
        ord: 15i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_MOTION_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_MOTION_SOFTNESS`"]
    pub const ANGULAR_MOTION_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 16i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_MOTION_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_MOTION_RESTITUTION`"]
    pub const ANGULAR_MOTION_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 17i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_MOTION_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_MOTION_DAMPING`"]
    pub const ANGULAR_MOTION_DAMPING: SliderJointParam = SliderJointParam {
        ord: 18i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_ORTHOGONAL_SOFTNESS")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_ORTHOGONAL_SOFTNESS`"]
    pub const ANGULAR_ORTHOGONAL_SOFTNESS: SliderJointParam = SliderJointParam {
        ord: 19i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_ORTHOGONAL_RESTITUTION")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_ORTHOGONAL_RESTITUTION`"]
    pub const ANGULAR_ORTHOGONAL_RESTITUTION: SliderJointParam = SliderJointParam {
        ord: 20i32
    };
    #[doc(alias = "SLIDER_JOINT_ANGULAR_ORTHOGONAL_DAMPING")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_ANGULAR_ORTHOGONAL_DAMPING`"]
    pub const ANGULAR_ORTHOGONAL_DAMPING: SliderJointParam = SliderJointParam {
        ord: 21i32
    };
    #[doc(alias = "SLIDER_JOINT_MAX")]
    #[doc = "Godot enumerator name: `SLIDER_JOINT_MAX`"]
    pub const MAX: SliderJointParam = SliderJointParam {
        ord: 22i32
    };
    
}
impl std::fmt::Debug for SliderJointParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SliderJointParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SliderJointParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
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
            Self::LINEAR_LIMIT_UPPER => "LINEAR_LIMIT_UPPER", Self::LINEAR_LIMIT_LOWER => "LINEAR_LIMIT_LOWER", Self::LINEAR_LIMIT_SOFTNESS => "LINEAR_LIMIT_SOFTNESS", Self::LINEAR_LIMIT_RESTITUTION => "LINEAR_LIMIT_RESTITUTION", Self::LINEAR_LIMIT_DAMPING => "LINEAR_LIMIT_DAMPING", Self::LINEAR_MOTION_SOFTNESS => "LINEAR_MOTION_SOFTNESS", Self::LINEAR_MOTION_RESTITUTION => "LINEAR_MOTION_RESTITUTION", Self::LINEAR_MOTION_DAMPING => "LINEAR_MOTION_DAMPING", Self::LINEAR_ORTHOGONAL_SOFTNESS => "LINEAR_ORTHOGONAL_SOFTNESS", Self::LINEAR_ORTHOGONAL_RESTITUTION => "LINEAR_ORTHOGONAL_RESTITUTION", Self::LINEAR_ORTHOGONAL_DAMPING => "LINEAR_ORTHOGONAL_DAMPING", Self::ANGULAR_LIMIT_UPPER => "ANGULAR_LIMIT_UPPER", Self::ANGULAR_LIMIT_LOWER => "ANGULAR_LIMIT_LOWER", Self::ANGULAR_LIMIT_SOFTNESS => "ANGULAR_LIMIT_SOFTNESS", Self::ANGULAR_LIMIT_RESTITUTION => "ANGULAR_LIMIT_RESTITUTION", Self::ANGULAR_LIMIT_DAMPING => "ANGULAR_LIMIT_DAMPING", Self::ANGULAR_MOTION_SOFTNESS => "ANGULAR_MOTION_SOFTNESS", Self::ANGULAR_MOTION_RESTITUTION => "ANGULAR_MOTION_RESTITUTION", Self::ANGULAR_MOTION_DAMPING => "ANGULAR_MOTION_DAMPING", Self::ANGULAR_ORTHOGONAL_SOFTNESS => "ANGULAR_ORTHOGONAL_SOFTNESS", Self::ANGULAR_ORTHOGONAL_RESTITUTION => "ANGULAR_ORTHOGONAL_RESTITUTION", Self::ANGULAR_ORTHOGONAL_DAMPING => "ANGULAR_ORTHOGONAL_DAMPING", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SliderJointParam::LINEAR_LIMIT_UPPER, SliderJointParam::LINEAR_LIMIT_LOWER, SliderJointParam::LINEAR_LIMIT_SOFTNESS, SliderJointParam::LINEAR_LIMIT_RESTITUTION, SliderJointParam::LINEAR_LIMIT_DAMPING, SliderJointParam::LINEAR_MOTION_SOFTNESS, SliderJointParam::LINEAR_MOTION_RESTITUTION, SliderJointParam::LINEAR_MOTION_DAMPING, SliderJointParam::LINEAR_ORTHOGONAL_SOFTNESS, SliderJointParam::LINEAR_ORTHOGONAL_RESTITUTION, SliderJointParam::LINEAR_ORTHOGONAL_DAMPING, SliderJointParam::ANGULAR_LIMIT_UPPER, SliderJointParam::ANGULAR_LIMIT_LOWER, SliderJointParam::ANGULAR_LIMIT_SOFTNESS, SliderJointParam::ANGULAR_LIMIT_RESTITUTION, SliderJointParam::ANGULAR_LIMIT_DAMPING, SliderJointParam::ANGULAR_MOTION_SOFTNESS, SliderJointParam::ANGULAR_MOTION_RESTITUTION, SliderJointParam::ANGULAR_MOTION_DAMPING, SliderJointParam::ANGULAR_ORTHOGONAL_SOFTNESS, SliderJointParam::ANGULAR_ORTHOGONAL_RESTITUTION, SliderJointParam::ANGULAR_ORTHOGONAL_DAMPING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SliderJointParam >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINEAR_LIMIT_UPPER", "SLIDER_JOINT_LINEAR_LIMIT_UPPER", SliderJointParam::LINEAR_LIMIT_UPPER), crate::meta::inspect::EnumConstant::new("LINEAR_LIMIT_LOWER", "SLIDER_JOINT_LINEAR_LIMIT_LOWER", SliderJointParam::LINEAR_LIMIT_LOWER), crate::meta::inspect::EnumConstant::new("LINEAR_LIMIT_SOFTNESS", "SLIDER_JOINT_LINEAR_LIMIT_SOFTNESS", SliderJointParam::LINEAR_LIMIT_SOFTNESS), crate::meta::inspect::EnumConstant::new("LINEAR_LIMIT_RESTITUTION", "SLIDER_JOINT_LINEAR_LIMIT_RESTITUTION", SliderJointParam::LINEAR_LIMIT_RESTITUTION), crate::meta::inspect::EnumConstant::new("LINEAR_LIMIT_DAMPING", "SLIDER_JOINT_LINEAR_LIMIT_DAMPING", SliderJointParam::LINEAR_LIMIT_DAMPING), crate::meta::inspect::EnumConstant::new("LINEAR_MOTION_SOFTNESS", "SLIDER_JOINT_LINEAR_MOTION_SOFTNESS", SliderJointParam::LINEAR_MOTION_SOFTNESS), crate::meta::inspect::EnumConstant::new("LINEAR_MOTION_RESTITUTION", "SLIDER_JOINT_LINEAR_MOTION_RESTITUTION", SliderJointParam::LINEAR_MOTION_RESTITUTION), crate::meta::inspect::EnumConstant::new("LINEAR_MOTION_DAMPING", "SLIDER_JOINT_LINEAR_MOTION_DAMPING", SliderJointParam::LINEAR_MOTION_DAMPING), crate::meta::inspect::EnumConstant::new("LINEAR_ORTHOGONAL_SOFTNESS", "SLIDER_JOINT_LINEAR_ORTHOGONAL_SOFTNESS", SliderJointParam::LINEAR_ORTHOGONAL_SOFTNESS), crate::meta::inspect::EnumConstant::new("LINEAR_ORTHOGONAL_RESTITUTION", "SLIDER_JOINT_LINEAR_ORTHOGONAL_RESTITUTION", SliderJointParam::LINEAR_ORTHOGONAL_RESTITUTION), crate::meta::inspect::EnumConstant::new("LINEAR_ORTHOGONAL_DAMPING", "SLIDER_JOINT_LINEAR_ORTHOGONAL_DAMPING", SliderJointParam::LINEAR_ORTHOGONAL_DAMPING), crate::meta::inspect::EnumConstant::new("ANGULAR_LIMIT_UPPER", "SLIDER_JOINT_ANGULAR_LIMIT_UPPER", SliderJointParam::ANGULAR_LIMIT_UPPER), crate::meta::inspect::EnumConstant::new("ANGULAR_LIMIT_LOWER", "SLIDER_JOINT_ANGULAR_LIMIT_LOWER", SliderJointParam::ANGULAR_LIMIT_LOWER), crate::meta::inspect::EnumConstant::new("ANGULAR_LIMIT_SOFTNESS", "SLIDER_JOINT_ANGULAR_LIMIT_SOFTNESS", SliderJointParam::ANGULAR_LIMIT_SOFTNESS), crate::meta::inspect::EnumConstant::new("ANGULAR_LIMIT_RESTITUTION", "SLIDER_JOINT_ANGULAR_LIMIT_RESTITUTION", SliderJointParam::ANGULAR_LIMIT_RESTITUTION), crate::meta::inspect::EnumConstant::new("ANGULAR_LIMIT_DAMPING", "SLIDER_JOINT_ANGULAR_LIMIT_DAMPING", SliderJointParam::ANGULAR_LIMIT_DAMPING), crate::meta::inspect::EnumConstant::new("ANGULAR_MOTION_SOFTNESS", "SLIDER_JOINT_ANGULAR_MOTION_SOFTNESS", SliderJointParam::ANGULAR_MOTION_SOFTNESS), crate::meta::inspect::EnumConstant::new("ANGULAR_MOTION_RESTITUTION", "SLIDER_JOINT_ANGULAR_MOTION_RESTITUTION", SliderJointParam::ANGULAR_MOTION_RESTITUTION), crate::meta::inspect::EnumConstant::new("ANGULAR_MOTION_DAMPING", "SLIDER_JOINT_ANGULAR_MOTION_DAMPING", SliderJointParam::ANGULAR_MOTION_DAMPING), crate::meta::inspect::EnumConstant::new("ANGULAR_ORTHOGONAL_SOFTNESS", "SLIDER_JOINT_ANGULAR_ORTHOGONAL_SOFTNESS", SliderJointParam::ANGULAR_ORTHOGONAL_SOFTNESS), crate::meta::inspect::EnumConstant::new("ANGULAR_ORTHOGONAL_RESTITUTION", "SLIDER_JOINT_ANGULAR_ORTHOGONAL_RESTITUTION", SliderJointParam::ANGULAR_ORTHOGONAL_RESTITUTION), crate::meta::inspect::EnumConstant::new("ANGULAR_ORTHOGONAL_DAMPING", "SLIDER_JOINT_ANGULAR_ORTHOGONAL_DAMPING", SliderJointParam::ANGULAR_ORTHOGONAL_DAMPING), crate::meta::inspect::EnumConstant::new("MAX", "SLIDER_JOINT_MAX", SliderJointParam::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for SliderJointParam {
    const ENUMERATOR_COUNT: usize = 22usize;
    
}
impl crate::meta::GodotConvert for SliderJointParam {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Slider Joint Linear Limit Upper", 0i64), EnumeratorShape::new_int("Slider Joint Linear Limit Lower", 1i64), EnumeratorShape::new_int("Slider Joint Linear Limit Softness", 2i64), EnumeratorShape::new_int("Slider Joint Linear Limit Restitution", 3i64), EnumeratorShape::new_int("Slider Joint Linear Limit Damping", 4i64), EnumeratorShape::new_int("Slider Joint Linear Motion Softness", 5i64), EnumeratorShape::new_int("Slider Joint Linear Motion Restitution", 6i64), EnumeratorShape::new_int("Slider Joint Linear Motion Damping", 7i64), EnumeratorShape::new_int("Slider Joint Linear Orthogonal Softness", 8i64), EnumeratorShape::new_int("Slider Joint Linear Orthogonal Restitution", 9i64), EnumeratorShape::new_int("Slider Joint Linear Orthogonal Damping", 10i64), EnumeratorShape::new_int("Slider Joint Angular Limit Upper", 11i64), EnumeratorShape::new_int("Slider Joint Angular Limit Lower", 12i64), EnumeratorShape::new_int("Slider Joint Angular Limit Softness", 13i64), EnumeratorShape::new_int("Slider Joint Angular Limit Restitution", 14i64), EnumeratorShape::new_int("Slider Joint Angular Limit Damping", 15i64), EnumeratorShape::new_int("Slider Joint Angular Motion Softness", 16i64), EnumeratorShape::new_int("Slider Joint Angular Motion Restitution", 17i64), EnumeratorShape::new_int("Slider Joint Angular Motion Damping", 18i64), EnumeratorShape::new_int("Slider Joint Angular Orthogonal Softness", 19i64), EnumeratorShape::new_int("Slider Joint Angular Orthogonal Restitution", 20i64), EnumeratorShape::new_int("Slider Joint Angular Orthogonal Damping", 21i64), EnumeratorShape::new_int("Slider Joint Max", 22i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.SliderJointParam")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SliderJointParam {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SliderJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SliderJointParam {
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
impl crate::registry::property::Export for SliderJointParam {
    
}
impl crate::meta::Element for SliderJointParam {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ConeTwistJointParam {
    ord: i32
}
impl ConeTwistJointParam {
    #[doc(alias = "CONE_TWIST_JOINT_SWING_SPAN")]
    #[doc = "Godot enumerator name: `CONE_TWIST_JOINT_SWING_SPAN`"]
    pub const SWING_SPAN: ConeTwistJointParam = ConeTwistJointParam {
        ord: 0i32
    };
    #[doc(alias = "CONE_TWIST_JOINT_TWIST_SPAN")]
    #[doc = "Godot enumerator name: `CONE_TWIST_JOINT_TWIST_SPAN`"]
    pub const TWIST_SPAN: ConeTwistJointParam = ConeTwistJointParam {
        ord: 1i32
    };
    #[doc(alias = "CONE_TWIST_JOINT_BIAS")]
    #[doc = "Godot enumerator name: `CONE_TWIST_JOINT_BIAS`"]
    pub const BIAS: ConeTwistJointParam = ConeTwistJointParam {
        ord: 2i32
    };
    #[doc(alias = "CONE_TWIST_JOINT_SOFTNESS")]
    #[doc = "Godot enumerator name: `CONE_TWIST_JOINT_SOFTNESS`"]
    pub const SOFTNESS: ConeTwistJointParam = ConeTwistJointParam {
        ord: 3i32
    };
    #[doc(alias = "CONE_TWIST_JOINT_RELAXATION")]
    #[doc = "Godot enumerator name: `CONE_TWIST_JOINT_RELAXATION`"]
    pub const RELAXATION: ConeTwistJointParam = ConeTwistJointParam {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ConeTwistJointParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ConeTwistJointParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ConeTwistJointParam {
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
            Self::SWING_SPAN => "SWING_SPAN", Self::TWIST_SPAN => "TWIST_SPAN", Self::BIAS => "BIAS", Self::SOFTNESS => "SOFTNESS", Self::RELAXATION => "RELAXATION", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ConeTwistJointParam::SWING_SPAN, ConeTwistJointParam::TWIST_SPAN, ConeTwistJointParam::BIAS, ConeTwistJointParam::SOFTNESS, ConeTwistJointParam::RELAXATION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ConeTwistJointParam >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SWING_SPAN", "CONE_TWIST_JOINT_SWING_SPAN", ConeTwistJointParam::SWING_SPAN), crate::meta::inspect::EnumConstant::new("TWIST_SPAN", "CONE_TWIST_JOINT_TWIST_SPAN", ConeTwistJointParam::TWIST_SPAN), crate::meta::inspect::EnumConstant::new("BIAS", "CONE_TWIST_JOINT_BIAS", ConeTwistJointParam::BIAS), crate::meta::inspect::EnumConstant::new("SOFTNESS", "CONE_TWIST_JOINT_SOFTNESS", ConeTwistJointParam::SOFTNESS), crate::meta::inspect::EnumConstant::new("RELAXATION", "CONE_TWIST_JOINT_RELAXATION", ConeTwistJointParam::RELAXATION)]
        }
    }
}
impl crate::meta::GodotConvert for ConeTwistJointParam {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Cone Twist Joint Swing Span", 0i64), EnumeratorShape::new_int("Cone Twist Joint Twist Span", 1i64), EnumeratorShape::new_int("Cone Twist Joint Bias", 2i64), EnumeratorShape::new_int("Cone Twist Joint Softness", 3i64), EnumeratorShape::new_int("Cone Twist Joint Relaxation", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.ConeTwistJointParam")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ConeTwistJointParam {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ConeTwistJointParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ConeTwistJointParam {
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
impl crate::registry::property::Export for ConeTwistJointParam {
    
}
impl crate::meta::Element for ConeTwistJointParam {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `G6DOFJointAxisParam`."]
pub struct G6dofJointAxisParam {
    ord: i32
}
impl G6dofJointAxisParam {
    #[doc(alias = "G6DOF_JOINT_LINEAR_LOWER_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_LOWER_LIMIT`"]
    pub const LINEAR_LOWER_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 0i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_UPPER_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_UPPER_LIMIT`"]
    pub const LINEAR_UPPER_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 1i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_LIMIT_SOFTNESS`"]
    pub const LINEAR_LIMIT_SOFTNESS: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 2i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_RESTITUTION")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_RESTITUTION`"]
    pub const LINEAR_RESTITUTION: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 3i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_DAMPING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_DAMPING`"]
    pub const LINEAR_DAMPING: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 4i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_MOTOR_TARGET_VELOCITY")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_MOTOR_TARGET_VELOCITY`"]
    pub const LINEAR_MOTOR_TARGET_VELOCITY: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 5i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_MOTOR_FORCE_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_MOTOR_FORCE_LIMIT`"]
    pub const LINEAR_MOTOR_FORCE_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 6i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_SPRING_STIFFNESS")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_SPRING_STIFFNESS`"]
    pub const LINEAR_SPRING_STIFFNESS: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 7i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_SPRING_DAMPING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_SPRING_DAMPING`"]
    pub const LINEAR_SPRING_DAMPING: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 8i32
    };
    #[doc(alias = "G6DOF_JOINT_LINEAR_SPRING_EQUILIBRIUM_POINT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_LINEAR_SPRING_EQUILIBRIUM_POINT`"]
    pub const LINEAR_SPRING_EQUILIBRIUM_POINT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 9i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_LOWER_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_LOWER_LIMIT`"]
    pub const ANGULAR_LOWER_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 10i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_UPPER_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_UPPER_LIMIT`"]
    pub const ANGULAR_UPPER_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 11i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_LIMIT_SOFTNESS`"]
    pub const ANGULAR_LIMIT_SOFTNESS: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 12i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_DAMPING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_DAMPING`"]
    pub const ANGULAR_DAMPING: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 13i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_RESTITUTION")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_RESTITUTION`"]
    pub const ANGULAR_RESTITUTION: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 14i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_FORCE_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_FORCE_LIMIT`"]
    pub const ANGULAR_FORCE_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 15i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_ERP")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_ERP`"]
    pub const ANGULAR_ERP: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 16i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_MOTOR_TARGET_VELOCITY")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_MOTOR_TARGET_VELOCITY`"]
    pub const ANGULAR_MOTOR_TARGET_VELOCITY: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 17i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_MOTOR_FORCE_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_MOTOR_FORCE_LIMIT`"]
    pub const ANGULAR_MOTOR_FORCE_LIMIT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 18i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_SPRING_STIFFNESS")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_SPRING_STIFFNESS`"]
    pub const ANGULAR_SPRING_STIFFNESS: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 19i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_SPRING_DAMPING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_SPRING_DAMPING`"]
    pub const ANGULAR_SPRING_DAMPING: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 20i32
    };
    #[doc(alias = "G6DOF_JOINT_ANGULAR_SPRING_EQUILIBRIUM_POINT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_ANGULAR_SPRING_EQUILIBRIUM_POINT`"]
    pub const ANGULAR_SPRING_EQUILIBRIUM_POINT: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 21i32
    };
    #[doc(alias = "G6DOF_JOINT_MAX")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_MAX`"]
    pub const MAX: G6dofJointAxisParam = G6dofJointAxisParam {
        ord: 22i32
    };
    
}
impl std::fmt::Debug for G6dofJointAxisParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("G6dofJointAxisParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for G6dofJointAxisParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
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
            Self::LINEAR_LOWER_LIMIT => "LINEAR_LOWER_LIMIT", Self::LINEAR_UPPER_LIMIT => "LINEAR_UPPER_LIMIT", Self::LINEAR_LIMIT_SOFTNESS => "LINEAR_LIMIT_SOFTNESS", Self::LINEAR_RESTITUTION => "LINEAR_RESTITUTION", Self::LINEAR_DAMPING => "LINEAR_DAMPING", Self::LINEAR_MOTOR_TARGET_VELOCITY => "LINEAR_MOTOR_TARGET_VELOCITY", Self::LINEAR_MOTOR_FORCE_LIMIT => "LINEAR_MOTOR_FORCE_LIMIT", Self::LINEAR_SPRING_STIFFNESS => "LINEAR_SPRING_STIFFNESS", Self::LINEAR_SPRING_DAMPING => "LINEAR_SPRING_DAMPING", Self::LINEAR_SPRING_EQUILIBRIUM_POINT => "LINEAR_SPRING_EQUILIBRIUM_POINT", Self::ANGULAR_LOWER_LIMIT => "ANGULAR_LOWER_LIMIT", Self::ANGULAR_UPPER_LIMIT => "ANGULAR_UPPER_LIMIT", Self::ANGULAR_LIMIT_SOFTNESS => "ANGULAR_LIMIT_SOFTNESS", Self::ANGULAR_DAMPING => "ANGULAR_DAMPING", Self::ANGULAR_RESTITUTION => "ANGULAR_RESTITUTION", Self::ANGULAR_FORCE_LIMIT => "ANGULAR_FORCE_LIMIT", Self::ANGULAR_ERP => "ANGULAR_ERP", Self::ANGULAR_MOTOR_TARGET_VELOCITY => "ANGULAR_MOTOR_TARGET_VELOCITY", Self::ANGULAR_MOTOR_FORCE_LIMIT => "ANGULAR_MOTOR_FORCE_LIMIT", Self::ANGULAR_SPRING_STIFFNESS => "ANGULAR_SPRING_STIFFNESS", Self::ANGULAR_SPRING_DAMPING => "ANGULAR_SPRING_DAMPING", Self::ANGULAR_SPRING_EQUILIBRIUM_POINT => "ANGULAR_SPRING_EQUILIBRIUM_POINT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[G6dofJointAxisParam::LINEAR_LOWER_LIMIT, G6dofJointAxisParam::LINEAR_UPPER_LIMIT, G6dofJointAxisParam::LINEAR_LIMIT_SOFTNESS, G6dofJointAxisParam::LINEAR_RESTITUTION, G6dofJointAxisParam::LINEAR_DAMPING, G6dofJointAxisParam::LINEAR_MOTOR_TARGET_VELOCITY, G6dofJointAxisParam::LINEAR_MOTOR_FORCE_LIMIT, G6dofJointAxisParam::LINEAR_SPRING_STIFFNESS, G6dofJointAxisParam::LINEAR_SPRING_DAMPING, G6dofJointAxisParam::LINEAR_SPRING_EQUILIBRIUM_POINT, G6dofJointAxisParam::ANGULAR_LOWER_LIMIT, G6dofJointAxisParam::ANGULAR_UPPER_LIMIT, G6dofJointAxisParam::ANGULAR_LIMIT_SOFTNESS, G6dofJointAxisParam::ANGULAR_DAMPING, G6dofJointAxisParam::ANGULAR_RESTITUTION, G6dofJointAxisParam::ANGULAR_FORCE_LIMIT, G6dofJointAxisParam::ANGULAR_ERP, G6dofJointAxisParam::ANGULAR_MOTOR_TARGET_VELOCITY, G6dofJointAxisParam::ANGULAR_MOTOR_FORCE_LIMIT, G6dofJointAxisParam::ANGULAR_SPRING_STIFFNESS, G6dofJointAxisParam::ANGULAR_SPRING_DAMPING, G6dofJointAxisParam::ANGULAR_SPRING_EQUILIBRIUM_POINT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < G6dofJointAxisParam >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINEAR_LOWER_LIMIT", "G6DOF_JOINT_LINEAR_LOWER_LIMIT", G6dofJointAxisParam::LINEAR_LOWER_LIMIT), crate::meta::inspect::EnumConstant::new("LINEAR_UPPER_LIMIT", "G6DOF_JOINT_LINEAR_UPPER_LIMIT", G6dofJointAxisParam::LINEAR_UPPER_LIMIT), crate::meta::inspect::EnumConstant::new("LINEAR_LIMIT_SOFTNESS", "G6DOF_JOINT_LINEAR_LIMIT_SOFTNESS", G6dofJointAxisParam::LINEAR_LIMIT_SOFTNESS), crate::meta::inspect::EnumConstant::new("LINEAR_RESTITUTION", "G6DOF_JOINT_LINEAR_RESTITUTION", G6dofJointAxisParam::LINEAR_RESTITUTION), crate::meta::inspect::EnumConstant::new("LINEAR_DAMPING", "G6DOF_JOINT_LINEAR_DAMPING", G6dofJointAxisParam::LINEAR_DAMPING), crate::meta::inspect::EnumConstant::new("LINEAR_MOTOR_TARGET_VELOCITY", "G6DOF_JOINT_LINEAR_MOTOR_TARGET_VELOCITY", G6dofJointAxisParam::LINEAR_MOTOR_TARGET_VELOCITY), crate::meta::inspect::EnumConstant::new("LINEAR_MOTOR_FORCE_LIMIT", "G6DOF_JOINT_LINEAR_MOTOR_FORCE_LIMIT", G6dofJointAxisParam::LINEAR_MOTOR_FORCE_LIMIT), crate::meta::inspect::EnumConstant::new("LINEAR_SPRING_STIFFNESS", "G6DOF_JOINT_LINEAR_SPRING_STIFFNESS", G6dofJointAxisParam::LINEAR_SPRING_STIFFNESS), crate::meta::inspect::EnumConstant::new("LINEAR_SPRING_DAMPING", "G6DOF_JOINT_LINEAR_SPRING_DAMPING", G6dofJointAxisParam::LINEAR_SPRING_DAMPING), crate::meta::inspect::EnumConstant::new("LINEAR_SPRING_EQUILIBRIUM_POINT", "G6DOF_JOINT_LINEAR_SPRING_EQUILIBRIUM_POINT", G6dofJointAxisParam::LINEAR_SPRING_EQUILIBRIUM_POINT), crate::meta::inspect::EnumConstant::new("ANGULAR_LOWER_LIMIT", "G6DOF_JOINT_ANGULAR_LOWER_LIMIT", G6dofJointAxisParam::ANGULAR_LOWER_LIMIT), crate::meta::inspect::EnumConstant::new("ANGULAR_UPPER_LIMIT", "G6DOF_JOINT_ANGULAR_UPPER_LIMIT", G6dofJointAxisParam::ANGULAR_UPPER_LIMIT), crate::meta::inspect::EnumConstant::new("ANGULAR_LIMIT_SOFTNESS", "G6DOF_JOINT_ANGULAR_LIMIT_SOFTNESS", G6dofJointAxisParam::ANGULAR_LIMIT_SOFTNESS), crate::meta::inspect::EnumConstant::new("ANGULAR_DAMPING", "G6DOF_JOINT_ANGULAR_DAMPING", G6dofJointAxisParam::ANGULAR_DAMPING), crate::meta::inspect::EnumConstant::new("ANGULAR_RESTITUTION", "G6DOF_JOINT_ANGULAR_RESTITUTION", G6dofJointAxisParam::ANGULAR_RESTITUTION), crate::meta::inspect::EnumConstant::new("ANGULAR_FORCE_LIMIT", "G6DOF_JOINT_ANGULAR_FORCE_LIMIT", G6dofJointAxisParam::ANGULAR_FORCE_LIMIT), crate::meta::inspect::EnumConstant::new("ANGULAR_ERP", "G6DOF_JOINT_ANGULAR_ERP", G6dofJointAxisParam::ANGULAR_ERP), crate::meta::inspect::EnumConstant::new("ANGULAR_MOTOR_TARGET_VELOCITY", "G6DOF_JOINT_ANGULAR_MOTOR_TARGET_VELOCITY", G6dofJointAxisParam::ANGULAR_MOTOR_TARGET_VELOCITY), crate::meta::inspect::EnumConstant::new("ANGULAR_MOTOR_FORCE_LIMIT", "G6DOF_JOINT_ANGULAR_MOTOR_FORCE_LIMIT", G6dofJointAxisParam::ANGULAR_MOTOR_FORCE_LIMIT), crate::meta::inspect::EnumConstant::new("ANGULAR_SPRING_STIFFNESS", "G6DOF_JOINT_ANGULAR_SPRING_STIFFNESS", G6dofJointAxisParam::ANGULAR_SPRING_STIFFNESS), crate::meta::inspect::EnumConstant::new("ANGULAR_SPRING_DAMPING", "G6DOF_JOINT_ANGULAR_SPRING_DAMPING", G6dofJointAxisParam::ANGULAR_SPRING_DAMPING), crate::meta::inspect::EnumConstant::new("ANGULAR_SPRING_EQUILIBRIUM_POINT", "G6DOF_JOINT_ANGULAR_SPRING_EQUILIBRIUM_POINT", G6dofJointAxisParam::ANGULAR_SPRING_EQUILIBRIUM_POINT), crate::meta::inspect::EnumConstant::new("MAX", "G6DOF_JOINT_MAX", G6dofJointAxisParam::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for G6dofJointAxisParam {
    const ENUMERATOR_COUNT: usize = 22usize;
    
}
impl crate::meta::GodotConvert for G6dofJointAxisParam {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("G6dof Joint Linear Lower Limit", 0i64), EnumeratorShape::new_int("G6dof Joint Linear Upper Limit", 1i64), EnumeratorShape::new_int("G6dof Joint Linear Limit Softness", 2i64), EnumeratorShape::new_int("G6dof Joint Linear Restitution", 3i64), EnumeratorShape::new_int("G6dof Joint Linear Damping", 4i64), EnumeratorShape::new_int("G6dof Joint Linear Motor Target Velocity", 5i64), EnumeratorShape::new_int("G6dof Joint Linear Motor Force Limit", 6i64), EnumeratorShape::new_int("G6dof Joint Linear Spring Stiffness", 7i64), EnumeratorShape::new_int("G6dof Joint Linear Spring Damping", 8i64), EnumeratorShape::new_int("G6dof Joint Linear Spring Equilibrium Point", 9i64), EnumeratorShape::new_int("G6dof Joint Angular Lower Limit", 10i64), EnumeratorShape::new_int("G6dof Joint Angular Upper Limit", 11i64), EnumeratorShape::new_int("G6dof Joint Angular Limit Softness", 12i64), EnumeratorShape::new_int("G6dof Joint Angular Damping", 13i64), EnumeratorShape::new_int("G6dof Joint Angular Restitution", 14i64), EnumeratorShape::new_int("G6dof Joint Angular Force Limit", 15i64), EnumeratorShape::new_int("G6dof Joint Angular Erp", 16i64), EnumeratorShape::new_int("G6dof Joint Angular Motor Target Velocity", 17i64), EnumeratorShape::new_int("G6dof Joint Angular Motor Force Limit", 18i64), EnumeratorShape::new_int("G6dof Joint Angular Spring Stiffness", 19i64), EnumeratorShape::new_int("G6dof Joint Angular Spring Damping", 20i64), EnumeratorShape::new_int("G6dof Joint Angular Spring Equilibrium Point", 21i64), EnumeratorShape::new_int("G6dof Joint Max", 22i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.G6DOFJointAxisParam")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for G6dofJointAxisParam {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for G6dofJointAxisParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for G6dofJointAxisParam {
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
impl crate::registry::property::Export for G6dofJointAxisParam {
    
}
impl crate::meta::Element for G6dofJointAxisParam {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `G6DOFJointAxisFlag`."]
pub struct G6dofJointAxisFlag {
    ord: i32
}
impl G6dofJointAxisFlag {
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_LINEAR_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_LINEAR_LIMIT`"]
    pub const ENABLE_LINEAR_LIMIT: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 0i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_ANGULAR_LIMIT")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_ANGULAR_LIMIT`"]
    pub const ENABLE_ANGULAR_LIMIT: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 1i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_ANGULAR_SPRING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_ANGULAR_SPRING`"]
    pub const ENABLE_ANGULAR_SPRING: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 2i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_LINEAR_SPRING")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_LINEAR_SPRING`"]
    pub const ENABLE_LINEAR_SPRING: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 3i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_MOTOR")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_MOTOR`"]
    pub const ENABLE_MOTOR: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 4i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_ENABLE_LINEAR_MOTOR")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_ENABLE_LINEAR_MOTOR`"]
    pub const ENABLE_LINEAR_MOTOR: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 5i32
    };
    #[doc(alias = "G6DOF_JOINT_FLAG_MAX")]
    #[doc = "Godot enumerator name: `G6DOF_JOINT_FLAG_MAX`"]
    pub const MAX: G6dofJointAxisFlag = G6dofJointAxisFlag {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for G6dofJointAxisFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("G6dofJointAxisFlag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for G6dofJointAxisFlag {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::ENABLE_LINEAR_LIMIT => "ENABLE_LINEAR_LIMIT", Self::ENABLE_ANGULAR_LIMIT => "ENABLE_ANGULAR_LIMIT", Self::ENABLE_ANGULAR_SPRING => "ENABLE_ANGULAR_SPRING", Self::ENABLE_LINEAR_SPRING => "ENABLE_LINEAR_SPRING", Self::ENABLE_MOTOR => "ENABLE_MOTOR", Self::ENABLE_LINEAR_MOTOR => "ENABLE_LINEAR_MOTOR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[G6dofJointAxisFlag::ENABLE_LINEAR_LIMIT, G6dofJointAxisFlag::ENABLE_ANGULAR_LIMIT, G6dofJointAxisFlag::ENABLE_ANGULAR_SPRING, G6dofJointAxisFlag::ENABLE_LINEAR_SPRING, G6dofJointAxisFlag::ENABLE_MOTOR, G6dofJointAxisFlag::ENABLE_LINEAR_MOTOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < G6dofJointAxisFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ENABLE_LINEAR_LIMIT", "G6DOF_JOINT_FLAG_ENABLE_LINEAR_LIMIT", G6dofJointAxisFlag::ENABLE_LINEAR_LIMIT), crate::meta::inspect::EnumConstant::new("ENABLE_ANGULAR_LIMIT", "G6DOF_JOINT_FLAG_ENABLE_ANGULAR_LIMIT", G6dofJointAxisFlag::ENABLE_ANGULAR_LIMIT), crate::meta::inspect::EnumConstant::new("ENABLE_ANGULAR_SPRING", "G6DOF_JOINT_FLAG_ENABLE_ANGULAR_SPRING", G6dofJointAxisFlag::ENABLE_ANGULAR_SPRING), crate::meta::inspect::EnumConstant::new("ENABLE_LINEAR_SPRING", "G6DOF_JOINT_FLAG_ENABLE_LINEAR_SPRING", G6dofJointAxisFlag::ENABLE_LINEAR_SPRING), crate::meta::inspect::EnumConstant::new("ENABLE_MOTOR", "G6DOF_JOINT_FLAG_ENABLE_MOTOR", G6dofJointAxisFlag::ENABLE_MOTOR), crate::meta::inspect::EnumConstant::new("ENABLE_LINEAR_MOTOR", "G6DOF_JOINT_FLAG_ENABLE_LINEAR_MOTOR", G6dofJointAxisFlag::ENABLE_LINEAR_MOTOR), crate::meta::inspect::EnumConstant::new("MAX", "G6DOF_JOINT_FLAG_MAX", G6dofJointAxisFlag::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for G6dofJointAxisFlag {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for G6dofJointAxisFlag {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("G6dof Joint Flag Enable Linear Limit", 0i64), EnumeratorShape::new_int("G6dof Joint Flag Enable Angular Limit", 1i64), EnumeratorShape::new_int("G6dof Joint Flag Enable Angular Spring", 2i64), EnumeratorShape::new_int("G6dof Joint Flag Enable Linear Spring", 3i64), EnumeratorShape::new_int("G6dof Joint Flag Enable Motor", 4i64), EnumeratorShape::new_int("G6dof Joint Flag Enable Linear Motor", 5i64), EnumeratorShape::new_int("G6dof Joint Flag Max", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.G6DOFJointAxisFlag")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for G6dofJointAxisFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for G6dofJointAxisFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for G6dofJointAxisFlag {
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
impl crate::registry::property::Export for G6dofJointAxisFlag {
    
}
impl crate::meta::Element for G6dofJointAxisFlag {
    
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
    #[doc(alias = "SHAPE_SPHERE")]
    #[doc = "Godot enumerator name: `SHAPE_SPHERE`"]
    pub const SPHERE: ShapeType = ShapeType {
        ord: 2i32
    };
    #[doc(alias = "SHAPE_BOX")]
    #[doc = "Godot enumerator name: `SHAPE_BOX`"]
    pub const BOX: ShapeType = ShapeType {
        ord: 3i32
    };
    #[doc(alias = "SHAPE_CAPSULE")]
    #[doc = "Godot enumerator name: `SHAPE_CAPSULE`"]
    pub const CAPSULE: ShapeType = ShapeType {
        ord: 4i32
    };
    #[doc(alias = "SHAPE_CYLINDER")]
    #[doc = "Godot enumerator name: `SHAPE_CYLINDER`"]
    pub const CYLINDER: ShapeType = ShapeType {
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
    #[doc(alias = "SHAPE_HEIGHTMAP")]
    #[doc = "Godot enumerator name: `SHAPE_HEIGHTMAP`"]
    pub const HEIGHTMAP: ShapeType = ShapeType {
        ord: 8i32
    };
    #[doc(alias = "SHAPE_SOFT_BODY")]
    #[doc = "Godot enumerator name: `SHAPE_SOFT_BODY`"]
    pub const SOFT_BODY: ShapeType = ShapeType {
        ord: 9i32
    };
    #[doc(alias = "SHAPE_CUSTOM")]
    #[doc = "Godot enumerator name: `SHAPE_CUSTOM`"]
    pub const CUSTOM: ShapeType = ShapeType {
        ord: 10i32
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
            Self::WORLD_BOUNDARY => "WORLD_BOUNDARY", Self::SEPARATION_RAY => "SEPARATION_RAY", Self::SPHERE => "SPHERE", Self::BOX => "BOX", Self::CAPSULE => "CAPSULE", Self::CYLINDER => "CYLINDER", Self::CONVEX_POLYGON => "CONVEX_POLYGON", Self::CONCAVE_POLYGON => "CONCAVE_POLYGON", Self::HEIGHTMAP => "HEIGHTMAP", Self::SOFT_BODY => "SOFT_BODY", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShapeType::WORLD_BOUNDARY, ShapeType::SEPARATION_RAY, ShapeType::SPHERE, ShapeType::BOX, ShapeType::CAPSULE, ShapeType::CYLINDER, ShapeType::CONVEX_POLYGON, ShapeType::CONCAVE_POLYGON, ShapeType::HEIGHTMAP, ShapeType::SOFT_BODY, ShapeType::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShapeType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("WORLD_BOUNDARY", "SHAPE_WORLD_BOUNDARY", ShapeType::WORLD_BOUNDARY), crate::meta::inspect::EnumConstant::new("SEPARATION_RAY", "SHAPE_SEPARATION_RAY", ShapeType::SEPARATION_RAY), crate::meta::inspect::EnumConstant::new("SPHERE", "SHAPE_SPHERE", ShapeType::SPHERE), crate::meta::inspect::EnumConstant::new("BOX", "SHAPE_BOX", ShapeType::BOX), crate::meta::inspect::EnumConstant::new("CAPSULE", "SHAPE_CAPSULE", ShapeType::CAPSULE), crate::meta::inspect::EnumConstant::new("CYLINDER", "SHAPE_CYLINDER", ShapeType::CYLINDER), crate::meta::inspect::EnumConstant::new("CONVEX_POLYGON", "SHAPE_CONVEX_POLYGON", ShapeType::CONVEX_POLYGON), crate::meta::inspect::EnumConstant::new("CONCAVE_POLYGON", "SHAPE_CONCAVE_POLYGON", ShapeType::CONCAVE_POLYGON), crate::meta::inspect::EnumConstant::new("HEIGHTMAP", "SHAPE_HEIGHTMAP", ShapeType::HEIGHTMAP), crate::meta::inspect::EnumConstant::new("SOFT_BODY", "SHAPE_SOFT_BODY", ShapeType::SOFT_BODY), crate::meta::inspect::EnumConstant::new("CUSTOM", "SHAPE_CUSTOM", ShapeType::CUSTOM)]
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
            &[EnumeratorShape::new_int("Shape World Boundary", 0i64), EnumeratorShape::new_int("Shape Separation Ray", 1i64), EnumeratorShape::new_int("Shape Sphere", 2i64), EnumeratorShape::new_int("Shape Box", 3i64), EnumeratorShape::new_int("Shape Capsule", 4i64), EnumeratorShape::new_int("Shape Cylinder", 5i64), EnumeratorShape::new_int("Shape Convex Polygon", 6i64), EnumeratorShape::new_int("Shape Concave Polygon", 7i64), EnumeratorShape::new_int("Shape Heightmap", 8i64), EnumeratorShape::new_int("Shape Soft Body", 9i64), EnumeratorShape::new_int("Shape Custom", 10i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.ShapeType")), is_bitfield: false,
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
    #[doc(alias = "AREA_PARAM_WIND_FORCE_MAGNITUDE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_WIND_FORCE_MAGNITUDE`"]
    pub const WIND_FORCE_MAGNITUDE: AreaParameter = AreaParameter {
        ord: 10i32
    };
    #[doc(alias = "AREA_PARAM_WIND_SOURCE")]
    #[doc = "Godot enumerator name: `AREA_PARAM_WIND_SOURCE`"]
    pub const WIND_SOURCE: AreaParameter = AreaParameter {
        ord: 11i32
    };
    #[doc(alias = "AREA_PARAM_WIND_DIRECTION")]
    #[doc = "Godot enumerator name: `AREA_PARAM_WIND_DIRECTION`"]
    pub const WIND_DIRECTION: AreaParameter = AreaParameter {
        ord: 12i32
    };
    #[doc(alias = "AREA_PARAM_WIND_ATTENUATION_FACTOR")]
    #[doc = "Godot enumerator name: `AREA_PARAM_WIND_ATTENUATION_FACTOR`"]
    pub const WIND_ATTENUATION_FACTOR: AreaParameter = AreaParameter {
        ord: 13i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::GRAVITY_OVERRIDE_MODE => "GRAVITY_OVERRIDE_MODE", Self::GRAVITY => "GRAVITY", Self::GRAVITY_VECTOR => "GRAVITY_VECTOR", Self::GRAVITY_IS_POINT => "GRAVITY_IS_POINT", Self::GRAVITY_POINT_UNIT_DISTANCE => "GRAVITY_POINT_UNIT_DISTANCE", Self::LINEAR_DAMP_OVERRIDE_MODE => "LINEAR_DAMP_OVERRIDE_MODE", Self::LINEAR_DAMP => "LINEAR_DAMP", Self::ANGULAR_DAMP_OVERRIDE_MODE => "ANGULAR_DAMP_OVERRIDE_MODE", Self::ANGULAR_DAMP => "ANGULAR_DAMP", Self::PRIORITY => "PRIORITY", Self::WIND_FORCE_MAGNITUDE => "WIND_FORCE_MAGNITUDE", Self::WIND_SOURCE => "WIND_SOURCE", Self::WIND_DIRECTION => "WIND_DIRECTION", Self::WIND_ATTENUATION_FACTOR => "WIND_ATTENUATION_FACTOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AreaParameter::GRAVITY_OVERRIDE_MODE, AreaParameter::GRAVITY, AreaParameter::GRAVITY_VECTOR, AreaParameter::GRAVITY_IS_POINT, AreaParameter::GRAVITY_POINT_UNIT_DISTANCE, AreaParameter::LINEAR_DAMP_OVERRIDE_MODE, AreaParameter::LINEAR_DAMP, AreaParameter::ANGULAR_DAMP_OVERRIDE_MODE, AreaParameter::ANGULAR_DAMP, AreaParameter::PRIORITY, AreaParameter::WIND_FORCE_MAGNITUDE, AreaParameter::WIND_SOURCE, AreaParameter::WIND_DIRECTION, AreaParameter::WIND_ATTENUATION_FACTOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AreaParameter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GRAVITY_OVERRIDE_MODE", "AREA_PARAM_GRAVITY_OVERRIDE_MODE", AreaParameter::GRAVITY_OVERRIDE_MODE), crate::meta::inspect::EnumConstant::new("GRAVITY", "AREA_PARAM_GRAVITY", AreaParameter::GRAVITY), crate::meta::inspect::EnumConstant::new("GRAVITY_VECTOR", "AREA_PARAM_GRAVITY_VECTOR", AreaParameter::GRAVITY_VECTOR), crate::meta::inspect::EnumConstant::new("GRAVITY_IS_POINT", "AREA_PARAM_GRAVITY_IS_POINT", AreaParameter::GRAVITY_IS_POINT), crate::meta::inspect::EnumConstant::new("GRAVITY_POINT_UNIT_DISTANCE", "AREA_PARAM_GRAVITY_POINT_UNIT_DISTANCE", AreaParameter::GRAVITY_POINT_UNIT_DISTANCE), crate::meta::inspect::EnumConstant::new("LINEAR_DAMP_OVERRIDE_MODE", "AREA_PARAM_LINEAR_DAMP_OVERRIDE_MODE", AreaParameter::LINEAR_DAMP_OVERRIDE_MODE), crate::meta::inspect::EnumConstant::new("LINEAR_DAMP", "AREA_PARAM_LINEAR_DAMP", AreaParameter::LINEAR_DAMP), crate::meta::inspect::EnumConstant::new("ANGULAR_DAMP_OVERRIDE_MODE", "AREA_PARAM_ANGULAR_DAMP_OVERRIDE_MODE", AreaParameter::ANGULAR_DAMP_OVERRIDE_MODE), crate::meta::inspect::EnumConstant::new("ANGULAR_DAMP", "AREA_PARAM_ANGULAR_DAMP", AreaParameter::ANGULAR_DAMP), crate::meta::inspect::EnumConstant::new("PRIORITY", "AREA_PARAM_PRIORITY", AreaParameter::PRIORITY), crate::meta::inspect::EnumConstant::new("WIND_FORCE_MAGNITUDE", "AREA_PARAM_WIND_FORCE_MAGNITUDE", AreaParameter::WIND_FORCE_MAGNITUDE), crate::meta::inspect::EnumConstant::new("WIND_SOURCE", "AREA_PARAM_WIND_SOURCE", AreaParameter::WIND_SOURCE), crate::meta::inspect::EnumConstant::new("WIND_DIRECTION", "AREA_PARAM_WIND_DIRECTION", AreaParameter::WIND_DIRECTION), crate::meta::inspect::EnumConstant::new("WIND_ATTENUATION_FACTOR", "AREA_PARAM_WIND_ATTENUATION_FACTOR", AreaParameter::WIND_ATTENUATION_FACTOR)]
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
            &[EnumeratorShape::new_int("Area Param Gravity Override Mode", 0i64), EnumeratorShape::new_int("Area Param Gravity", 1i64), EnumeratorShape::new_int("Area Param Gravity Vector", 2i64), EnumeratorShape::new_int("Area Param Gravity Is Point", 3i64), EnumeratorShape::new_int("Area Param Gravity Point Unit Distance", 4i64), EnumeratorShape::new_int("Area Param Linear Damp Override Mode", 5i64), EnumeratorShape::new_int("Area Param Linear Damp", 6i64), EnumeratorShape::new_int("Area Param Angular Damp Override Mode", 7i64), EnumeratorShape::new_int("Area Param Angular Damp", 8i64), EnumeratorShape::new_int("Area Param Priority", 9i64), EnumeratorShape::new_int("Area Param Wind Force Magnitude", 10i64), EnumeratorShape::new_int("Area Param Wind Source", 11i64), EnumeratorShape::new_int("Area Param Wind Direction", 12i64), EnumeratorShape::new_int("Area Param Wind Attenuation Factor", 13i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.AreaParameter")), is_bitfield: false,
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.AreaSpaceOverrideMode")), is_bitfield: false,
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.BodyMode")), is_bitfield: false,
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.BodyParameter")), is_bitfield: false,
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.BodyDampMode")), is_bitfield: false,
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.BodyState")), is_bitfield: false,
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.AreaBodyStatus")), is_bitfield: false,
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.ProcessInfo")), is_bitfield: false,
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
    #[doc(alias = "SPACE_PARAM_SOLVER_ITERATIONS")]
    #[doc = "Godot enumerator name: `SPACE_PARAM_SOLVER_ITERATIONS`"]
    pub const SOLVER_ITERATIONS: SpaceParameter = SpaceParameter {
        ord: 7i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::CONTACT_RECYCLE_RADIUS => "CONTACT_RECYCLE_RADIUS", Self::CONTACT_MAX_SEPARATION => "CONTACT_MAX_SEPARATION", Self::CONTACT_MAX_ALLOWED_PENETRATION => "CONTACT_MAX_ALLOWED_PENETRATION", Self::CONTACT_DEFAULT_BIAS => "CONTACT_DEFAULT_BIAS", Self::BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD => "BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD => "BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD", Self::BODY_TIME_TO_SLEEP => "BODY_TIME_TO_SLEEP", Self::SOLVER_ITERATIONS => "SOLVER_ITERATIONS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SpaceParameter::CONTACT_RECYCLE_RADIUS, SpaceParameter::CONTACT_MAX_SEPARATION, SpaceParameter::CONTACT_MAX_ALLOWED_PENETRATION, SpaceParameter::CONTACT_DEFAULT_BIAS, SpaceParameter::BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD, SpaceParameter::BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD, SpaceParameter::BODY_TIME_TO_SLEEP, SpaceParameter::SOLVER_ITERATIONS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SpaceParameter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CONTACT_RECYCLE_RADIUS", "SPACE_PARAM_CONTACT_RECYCLE_RADIUS", SpaceParameter::CONTACT_RECYCLE_RADIUS), crate::meta::inspect::EnumConstant::new("CONTACT_MAX_SEPARATION", "SPACE_PARAM_CONTACT_MAX_SEPARATION", SpaceParameter::CONTACT_MAX_SEPARATION), crate::meta::inspect::EnumConstant::new("CONTACT_MAX_ALLOWED_PENETRATION", "SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION", SpaceParameter::CONTACT_MAX_ALLOWED_PENETRATION), crate::meta::inspect::EnumConstant::new("CONTACT_DEFAULT_BIAS", "SPACE_PARAM_CONTACT_DEFAULT_BIAS", SpaceParameter::CONTACT_DEFAULT_BIAS), crate::meta::inspect::EnumConstant::new("BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD", "SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD", SpaceParameter::BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD), crate::meta::inspect::EnumConstant::new("BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD", "SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD", SpaceParameter::BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD), crate::meta::inspect::EnumConstant::new("BODY_TIME_TO_SLEEP", "SPACE_PARAM_BODY_TIME_TO_SLEEP", SpaceParameter::BODY_TIME_TO_SLEEP), crate::meta::inspect::EnumConstant::new("SOLVER_ITERATIONS", "SPACE_PARAM_SOLVER_ITERATIONS", SpaceParameter::SOLVER_ITERATIONS)]
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
            &[EnumeratorShape::new_int("Space Param Contact Recycle Radius", 0i64), EnumeratorShape::new_int("Space Param Contact Max Separation", 1i64), EnumeratorShape::new_int("Space Param Contact Max Allowed Penetration", 2i64), EnumeratorShape::new_int("Space Param Contact Default Bias", 3i64), EnumeratorShape::new_int("Space Param Body Linear Velocity Sleep Threshold", 4i64), EnumeratorShape::new_int("Space Param Body Angular Velocity Sleep Threshold", 5i64), EnumeratorShape::new_int("Space Param Body Time To Sleep", 6i64), EnumeratorShape::new_int("Space Param Solver Iterations", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.SpaceParameter")), is_bitfield: false,
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
pub struct BodyAxis {
    ord: i32
}
impl BodyAxis {
    #[doc(alias = "BODY_AXIS_LINEAR_X")]
    #[doc = "Godot enumerator name: `BODY_AXIS_LINEAR_X`"]
    pub const LINEAR_X: BodyAxis = BodyAxis {
        ord: 1i32
    };
    #[doc(alias = "BODY_AXIS_LINEAR_Y")]
    #[doc = "Godot enumerator name: `BODY_AXIS_LINEAR_Y`"]
    pub const LINEAR_Y: BodyAxis = BodyAxis {
        ord: 2i32
    };
    #[doc(alias = "BODY_AXIS_LINEAR_Z")]
    #[doc = "Godot enumerator name: `BODY_AXIS_LINEAR_Z`"]
    pub const LINEAR_Z: BodyAxis = BodyAxis {
        ord: 4i32
    };
    #[doc(alias = "BODY_AXIS_ANGULAR_X")]
    #[doc = "Godot enumerator name: `BODY_AXIS_ANGULAR_X`"]
    pub const ANGULAR_X: BodyAxis = BodyAxis {
        ord: 8i32
    };
    #[doc(alias = "BODY_AXIS_ANGULAR_Y")]
    #[doc = "Godot enumerator name: `BODY_AXIS_ANGULAR_Y`"]
    pub const ANGULAR_Y: BodyAxis = BodyAxis {
        ord: 16i32
    };
    #[doc(alias = "BODY_AXIS_ANGULAR_Z")]
    #[doc = "Godot enumerator name: `BODY_AXIS_ANGULAR_Z`"]
    pub const ANGULAR_Z: BodyAxis = BodyAxis {
        ord: 32i32
    };
    
}
impl std::fmt::Debug for BodyAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BodyAxis") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BodyAxis {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 => Some(Self {
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
            Self::LINEAR_X => "LINEAR_X", Self::LINEAR_Y => "LINEAR_Y", Self::LINEAR_Z => "LINEAR_Z", Self::ANGULAR_X => "ANGULAR_X", Self::ANGULAR_Y => "ANGULAR_Y", Self::ANGULAR_Z => "ANGULAR_Z", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BodyAxis::LINEAR_X, BodyAxis::LINEAR_Y, BodyAxis::LINEAR_Z, BodyAxis::ANGULAR_X, BodyAxis::ANGULAR_Y, BodyAxis::ANGULAR_Z]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BodyAxis >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINEAR_X", "BODY_AXIS_LINEAR_X", BodyAxis::LINEAR_X), crate::meta::inspect::EnumConstant::new("LINEAR_Y", "BODY_AXIS_LINEAR_Y", BodyAxis::LINEAR_Y), crate::meta::inspect::EnumConstant::new("LINEAR_Z", "BODY_AXIS_LINEAR_Z", BodyAxis::LINEAR_Z), crate::meta::inspect::EnumConstant::new("ANGULAR_X", "BODY_AXIS_ANGULAR_X", BodyAxis::ANGULAR_X), crate::meta::inspect::EnumConstant::new("ANGULAR_Y", "BODY_AXIS_ANGULAR_Y", BodyAxis::ANGULAR_Y), crate::meta::inspect::EnumConstant::new("ANGULAR_Z", "BODY_AXIS_ANGULAR_Z", BodyAxis::ANGULAR_Z)]
        }
    }
}
impl crate::meta::GodotConvert for BodyAxis {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Body Axis Linear X", 1i64), EnumeratorShape::new_int("Body Axis Linear Y", 2i64), EnumeratorShape::new_int("Body Axis Linear Z", 4i64), EnumeratorShape::new_int("Body Axis Angular X", 8i64), EnumeratorShape::new_int("Body Axis Angular Y", 16i64), EnumeratorShape::new_int("Body Axis Angular Z", 32i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicsServer3D.BodyAxis")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BodyAxis {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BodyAxis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BodyAxis {
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
impl crate::registry::property::Export for BodyAxis {
    
}
impl crate::meta::Element for BodyAxis {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicsServer3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PhysicsServer3D {
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