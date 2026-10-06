#![doc = "Sidecar module for class [`PhysicsBody2D`][crate::classes::PhysicsBody2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsBody2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsbody2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsBody2D`.\n\nInherits [`CollisionObject2D`][crate::classes::CollisionObject2D].\n\nRelated symbols:\n\n* [`physics_body_2d`][crate::classes::physics_body_2d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `PhysicsBody2D`](https://docs.godotengine.org/en/stable/classes/class_physicsbody2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsBody2D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\n`PhysicsBody2D` is an abstract base class for 2D game objects affected by physics. All 2D physics bodies inherit from it."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsBody2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PhysicsBody2D {
        #[doc = "Moves the body along the vector `motion`. In order to be frame rate independent in [`physics_process`][`crate::classes::INode::physics_process`] or [`process`][`crate::classes::INode::process`], `motion` should be computed using `delta`.\n\nReturns a [`KinematicCollision2D`][crate::classes::KinematicCollision2D], which contains information about the collision when stopped, or when touching another body along the motion.\n\nIf `test_only` is `true`, the body does not move but the would-be collision information is given.\n\n`safe_margin` is the extra margin used for collision recovery (see \\[member CharacterBody2D.safe_margin] for more details).\n\nIf `recovery_as_collision` is `true`, any depenetration from the recovery phase is also reported as a collision; this is used e.g. by [`CharacterBody2D`][crate::classes::CharacterBody2D] for improving floor detection during floor snapping."]
        pub(crate) fn move_and_collide_full(&mut self, motion: Vector2, test_only: bool, safe_margin: f32, recovery_as_collision: bool,) -> Option < Gd < crate::classes::KinematicCollision2D > > {
            type CallRet = Option < Gd < crate::classes::KinematicCollision2D > >;
            type CallParams = (Vector2, bool, f32, bool,);
            let args = (motion, test_only, safe_margin, recovery_as_collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "move_and_collide", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`move_and_collide_ex`][Self::move_and_collide_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Moves the body along the vector `motion`. In order to be frame rate independent in [`physics_process`][`crate::classes::INode::physics_process`] or [`process`][`crate::classes::INode::process`], `motion` should be computed using `delta`.\n\nReturns a [`KinematicCollision2D`][crate::classes::KinematicCollision2D], which contains information about the collision when stopped, or when touching another body along the motion.\n\nIf `test_only` is `true`, the body does not move but the would-be collision information is given.\n\n`safe_margin` is the extra margin used for collision recovery (see \\[member CharacterBody2D.safe_margin] for more details).\n\nIf `recovery_as_collision` is `true`, any depenetration from the recovery phase is also reported as a collision; this is used e.g. by [`CharacterBody2D`][crate::classes::CharacterBody2D] for improving floor detection during floor snapping."]
        #[inline]
        pub fn move_and_collide(&mut self, motion: Vector2,) -> Option < Gd < crate::classes::KinematicCollision2D > > {
            self.move_and_collide_ex(motion,) . done()
        }
        #[doc = "Moves the body along the vector `motion`. In order to be frame rate independent in [`physics_process`][`crate::classes::INode::physics_process`] or [`process`][`crate::classes::INode::process`], `motion` should be computed using `delta`.\n\nReturns a [`KinematicCollision2D`][crate::classes::KinematicCollision2D], which contains information about the collision when stopped, or when touching another body along the motion.\n\nIf `test_only` is `true`, the body does not move but the would-be collision information is given.\n\n`safe_margin` is the extra margin used for collision recovery (see \\[member CharacterBody2D.safe_margin] for more details).\n\nIf `recovery_as_collision` is `true`, any depenetration from the recovery phase is also reported as a collision; this is used e.g. by [`CharacterBody2D`][crate::classes::CharacterBody2D] for improving floor detection during floor snapping."]
        #[inline]
        pub fn move_and_collide_ex < 'ex > (&'ex mut self, motion: Vector2,) -> ExMoveAndCollide < 'ex > {
            ExMoveAndCollide::new(self, motion,)
        }
        #[doc = "Checks for collisions without moving the body. In order to be frame rate independent in [`physics_process`][`crate::classes::INode::physics_process`] or [`process`][`crate::classes::INode::process`], `motion` should be computed using `delta`.\n\nVirtually sets the node's position, scale and rotation to that of the given [`Transform2D`][crate::builtin::Transform2D], then tries to move the body along the vector `motion`. Returns `true` if a collision would stop the body from moving along the whole path.\n\n`collision` is an optional object of type [`KinematicCollision2D`][crate::classes::KinematicCollision2D], which contains additional information about the collision when stopped, or when touching another body along the motion.\n\n`safe_margin` is the extra margin used for collision recovery (see \\[member CharacterBody2D.safe_margin] for more details).\n\nIf `recovery_as_collision` is `true`, any depenetration from the recovery phase is also reported as a collision; this is useful for checking whether the body would _touch_ any other bodies."]
        pub(crate) fn test_move_full(&mut self, from: Transform2D, motion: Vector2, collision: CowArg < Option < Gd < crate::classes::KinematicCollision2D > > >, safe_margin: f32, recovery_as_collision: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Transform2D, Vector2, CowArg < 'a0, Option < Gd < crate::classes::KinematicCollision2D > > >, f32, bool,);
            let args = (from, motion, collision, safe_margin, recovery_as_collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "test_move", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`test_move_ex`][Self::test_move_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Checks for collisions without moving the body. In order to be frame rate independent in [`physics_process`][`crate::classes::INode::physics_process`] or [`process`][`crate::classes::INode::process`], `motion` should be computed using `delta`.\n\nVirtually sets the node's position, scale and rotation to that of the given [`Transform2D`][crate::builtin::Transform2D], then tries to move the body along the vector `motion`. Returns `true` if a collision would stop the body from moving along the whole path.\n\n`collision` is an optional object of type [`KinematicCollision2D`][crate::classes::KinematicCollision2D], which contains additional information about the collision when stopped, or when touching another body along the motion.\n\n`safe_margin` is the extra margin used for collision recovery (see \\[member CharacterBody2D.safe_margin] for more details).\n\nIf `recovery_as_collision` is `true`, any depenetration from the recovery phase is also reported as a collision; this is useful for checking whether the body would _touch_ any other bodies."]
        #[inline]
        pub fn test_move(&mut self, from: Transform2D, motion: Vector2,) -> bool {
            self.test_move_ex(from, motion,) . done()
        }
        #[doc = "Checks for collisions without moving the body. In order to be frame rate independent in [`physics_process`][`crate::classes::INode::physics_process`] or [`process`][`crate::classes::INode::process`], `motion` should be computed using `delta`.\n\nVirtually sets the node's position, scale and rotation to that of the given [`Transform2D`][crate::builtin::Transform2D], then tries to move the body along the vector `motion`. Returns `true` if a collision would stop the body from moving along the whole path.\n\n`collision` is an optional object of type [`KinematicCollision2D`][crate::classes::KinematicCollision2D], which contains additional information about the collision when stopped, or when touching another body along the motion.\n\n`safe_margin` is the extra margin used for collision recovery (see \\[member CharacterBody2D.safe_margin] for more details).\n\nIf `recovery_as_collision` is `true`, any depenetration from the recovery phase is also reported as a collision; this is useful for checking whether the body would _touch_ any other bodies."]
        #[inline]
        pub fn test_move_ex < 'ex > (&'ex mut self, from: Transform2D, motion: Vector2,) -> ExTestMove < 'ex > {
            ExTestMove::new(self, from, motion,)
        }
        #[doc = "Returns the gravity vector computed from all sources that can affect the body, including all gravity overrides from [`Area2D`][crate::classes::Area2D] nodes and the global world gravity."]
        pub fn get_gravity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "get_gravity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of nodes that were added as collision exceptions for this body."]
        pub fn get_collision_exceptions(&self,) -> Array < Gd < crate::classes::PhysicsBody2D > > {
            type CallRet = Array < Gd < crate::classes::PhysicsBody2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "get_collision_exceptions", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a body to the list of bodies that this body can't collide with."]
        pub fn add_collision_exception_with(&mut self, body: impl AsArg < Gd < crate::classes::Node >>,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >,);
            let args = (body.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "add_collision_exception_with", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a body from the list of bodies that this body can't collide with."]
        pub fn remove_collision_exception_with(&mut self, body: impl AsArg < Gd < crate::classes::Node >>,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >,);
            let args = (body.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "remove_collision_exception_with", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsBody2D {
        type Base = crate::classes::CollisionObject2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsBody2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsBody2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject2D > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsBody2D {
        
    }
    impl std::ops::Deref for PhysicsBody2D {
        type Target = crate::classes::CollisionObject2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsBody2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsBody2D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PhysicsBody2D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`PhysicsBody2D::move_and_collide_ex`][super::PhysicsBody2D::move_and_collide_ex]."]
#[must_use]
pub struct ExMoveAndCollide < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsBody2D, motion: Vector2, test_only: bool, safe_margin: f32, recovery_as_collision: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExMoveAndCollide < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsBody2D, motion: Vector2,) -> Self {
        let test_only = false;
        let safe_margin = 0.08f32;
        let recovery_as_collision = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, motion: motion, test_only: test_only, safe_margin: safe_margin, recovery_as_collision: recovery_as_collision,
        }
    }
    #[inline]
    pub fn test_only(self, test_only: bool) -> Self {
        Self {
            test_only: test_only, .. self
        }
    }
    #[inline]
    pub fn safe_margin(self, safe_margin: f32) -> Self {
        Self {
            safe_margin: safe_margin, .. self
        }
    }
    #[inline]
    pub fn recovery_as_collision(self, recovery_as_collision: bool) -> Self {
        Self {
            recovery_as_collision: recovery_as_collision, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::KinematicCollision2D > > {
        let Self {
            _phantom, surround_object, motion, test_only, safe_margin, recovery_as_collision,
        }
        = self;
        re_export::PhysicsBody2D::move_and_collide_full(surround_object, motion, test_only, safe_margin, recovery_as_collision,)
    }
}
#[doc = "Default-param extender for [`PhysicsBody2D::test_move_ex`][super::PhysicsBody2D::test_move_ex]."]
#[must_use]
pub struct ExTestMove < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsBody2D, from: Transform2D, motion: Vector2, collision: CowArg < 'ex, Option < Gd < crate::classes::KinematicCollision2D > > >, safe_margin: f32, recovery_as_collision: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTestMove < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsBody2D, from: Transform2D, motion: Vector2,) -> Self {
        let collision = Gd::null_arg();
        let safe_margin = 0.08f32;
        let recovery_as_collision = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, motion: motion, collision: collision.into_arg(), safe_margin: safe_margin, recovery_as_collision: recovery_as_collision,
        }
    }
    #[inline]
    pub fn collision(self, collision: impl AsArg < Option < Gd < crate::classes::KinematicCollision2D >> > + 'ex) -> Self {
        Self {
            collision: collision.into_arg(), .. self
        }
    }
    #[inline]
    pub fn safe_margin(self, safe_margin: f32) -> Self {
        Self {
            safe_margin: safe_margin, .. self
        }
    }
    #[inline]
    pub fn recovery_as_collision(self, recovery_as_collision: bool) -> Self {
        Self {
            recovery_as_collision: recovery_as_collision, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, from, motion, collision, safe_margin, recovery_as_collision,
        }
        = self;
        re_export::PhysicsBody2D::test_move_full(surround_object, from, motion, collision, safe_margin, recovery_as_collision,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicsBody2D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::collision_object_2d::SignalsOfCollisionObject2D;
    impl WithSignals for PhysicsBody2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCollisionObject2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}