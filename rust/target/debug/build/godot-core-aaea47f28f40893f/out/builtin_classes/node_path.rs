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
pub(super) mod re_export {
    use super::*;
    #[doc(hidden)]
    #[repr(transparent)]
    pub struct InnerNodePath < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerNodePath < 'inner > {
    pub fn from_outer(outer: &NodePath) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the number of node names in the path. Property subnames are not included.\n\nFor example, `\"../RigidBody2D/Sprite2D:texture\"` contains 3 node names."]
    pub fn get_name_count(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(594usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "NodePath", "get_name_count", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the node name indicated by `idx`, starting from 0. If `idx` is out of bounds, an error is generated. See also \\[method get_subname_count] and \\[method get_name_count].\n\n\n```gdscript\nvar sprite_path = NodePath(\"../RigidBody2D/Sprite2D\")\nprint(sprite_path.get_name(0)) # Prints \"..\"\nprint(sprite_path.get_name(1)) # Prints \"RigidBody2D\"\nprint(sprite_path.get_name(2)) # Prints \"Sprite\"\n```\n"]
    pub fn get_name(&self, idx: i64,) -> StringName {
        type CallRet = StringName;
        type CallParams = (i64,);
        let args = (idx,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(595usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "NodePath", "get_name", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of property names (\"subnames\") in the path. Each subname in the node path is listed after a colon character (`:`).\n\nFor example, `\"Level/RigidBody2D/Sprite2D:texture:resource_name\"` contains 2 subnames."]
    pub fn get_subname_count(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(596usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "NodePath", "get_subname_count", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the 32-bit hash value representing the node path's contents.\n\n**Note:** Node paths with equal hash values are _not_ guaranteed to be the same, as a result of hash collisions. Node paths with different hash values are guaranteed to be different."]
    pub fn hash(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(597usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "NodePath", "hash", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the property name indicated by `idx`, starting from 0. If `idx` is out of bounds, an error is generated. See also \\[method get_subname_count].\n\n\n```gdscript\nvar path_to_name = NodePath(\"Sprite2D:texture:resource_name\")\nprint(path_to_name.get_subname(0)) # Prints \"texture\"\nprint(path_to_name.get_subname(1)) # Prints \"resource_name\"\n```\n"]
    pub fn get_subname(&self, idx: i64,) -> StringName {
        type CallRet = StringName;
        type CallParams = (i64,);
        let args = (idx,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(598usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "NodePath", "get_subname", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the slice of the [`NodePath`][crate::builtin::NodePath], from `begin` (inclusive) to `end` (exclusive), as a new [`NodePath`][crate::builtin::NodePath].\n\nThe absolute value of `begin` and `end` will be clamped to the sum of \\[method get_name_count] and \\[method get_subname_count], so the default value for `end` makes it slice to the end of the [`NodePath`][crate::builtin::NodePath] by default (i.e. `path.slice(1)` is a shorthand for `path.slice(1, path.get_name_count() + path.get_subname_count())`).\n\nIf either `begin` or `end` are negative, they will be relative to the end of the [`NodePath`][crate::builtin::NodePath] (i.e. `path.slice(0, -2)` is a shorthand for `path.slice(0, path.get_name_count() + path.get_subname_count() - 2)`)."]
    pub fn slice(&self, begin: i64, end: i64,) -> NodePath {
        type CallRet = NodePath;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(601usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "NodePath", "slice", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerNodePath;
impl NodePath {
    #[doc = "Returns `true` if the node path is absolute. Unlike a relative path, an absolute path is represented by a leading slash character (`/`) and always begins from the [`SceneTree`][crate::classes::SceneTree]. It can be used to reliably access nodes from the root node (e.g. `\"/root/Global\"` if an autoload named \"Global\" exists)."]
    pub fn is_absolute(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(593usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "NodePath", "is_absolute", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns all node names concatenated with a slash character (`/`) as a single [`StringName`][crate::builtin::StringName]."]
    pub fn get_concatenated_names(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(599usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "NodePath", "get_concatenated_names", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns all property subnames concatenated with a colon character (`:`) as a single [`StringName`][crate::builtin::StringName].\n\n\n```gdscript\nvar node_path = ^\"Sprite2D:texture:resource_name\"\nprint(node_path.get_concatenated_subnames()) # Prints \"texture:resource_name\"\n```\n"]
    pub fn get_concatenated_subnames(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(600usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "NodePath", "get_concatenated_subnames", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns a copy of this node path with a colon character (`:`) prefixed, transforming it to a pure property path with no node names (relative to the current node).\n\n\n```gdscript\n# node_path points to the \"x\" property of the child node named \"position\".\nvar node_path = ^\"position:x\"\n\n# property_path points to the \"position\" in the \"x\" axis of this node.\nvar property_path = node_path.get_as_property_path()\nprint(property_path) # Prints \":position:x\"\n```\n"]
    pub fn get_as_property_path(&self,) -> NodePath {
        type CallRet = NodePath;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(602usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "NodePath", "get_as_property_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if the node path has been constructed from an empty [`String`][crate::builtin::GString] (`\"\"`)."]
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(603usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "NodePath", "is_empty", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
}