#![doc = "Sidecar module for class [`Object`][crate::classes::Object].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Object` enums](https://docs.godotengine.org/en/stable/classes/class_object.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Object`.\n\nThis is the base class for all other classes at the root of the hierarchy. Every instance of `Object` can be stored in a [`Gd`][crate::obj::Gd] smart pointer.\n\nRelated symbols:\n\n* [`object`][crate::classes::object]: sidecar module with related enum/flag types\n* [`IObject`][crate::classes::IObject]: virtual methods\n* [`SignalsOfObject`][crate::classes::object::SignalsOfObject]: signal collection\n* [`ObjectNotification`][crate::classes::notify::ObjectNotification]: notification type\n\n\nSee also [Godot docs for `Object`](https://docs.godotengine.org/en/stable/classes/class_object.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Object::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nAn advanced [`Variant`][crate::builtin::Variant] type. All classes in the engine inherit from Object. Each class may define new properties, methods or signals, which are available to all inheriting classes. For example, a [`Sprite2D`][crate::classes::Sprite2D] instance is able to call [`add_child`][`crate::classes::Node::add_child`] because it inherits from [`Node`][crate::classes::Node].\n\nYou can create new instances, using `Object.new()` in GDScript, or `new GodotObject` in C#.\n\nTo delete an Object instance, call [`free`][`crate::obj::Gd::free`]. This is necessary for most classes inheriting Object, because they do not manage memory on their own, and will otherwise cause memory leaks when no longer in use. There are a few classes that perform memory management. For example, [`RefCounted`][crate::classes::RefCounted] (and by extension [`Resource`][crate::classes::Resource]) deletes itself when no longer referenced, and [`Node`][crate::classes::Node] deletes its children when freed.\n\nObjects can have a [`Script`][crate::classes::Script] attached to them. Once the [`Script`][crate::classes::Script] is instantiated, it effectively acts as an extension to the base class, allowing it to define and inherit new properties, methods and signals.\n\nInside a [`Script`][crate::classes::Script], [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`] may be overridden to customize properties in several ways. This allows them to be available to the editor, display as lists of options, sub-divide into groups, save on disk, etc. Scripting languages offer easier ways to customize properties, such as with the `@GDScript.@export` annotation.\n\nGodot is very dynamic. An object's script, and therefore its properties, methods and signals, can be changed at run-time. Because of this, there can be occasions where, for example, a property required by a method may not exist. To prevent run-time errors, see methods such as [`set`][`crate::classes::Object::set`], [`get`][`crate::classes::Object::get`], [`call`][`crate::classes::Object::call`], [`has_method`][`crate::classes::Object::has_method`], [`has_signal`][`crate::classes::Object::has_signal`], etc. Note that these methods are **much** slower than direct references.\n\nIn GDScript, you can also check if a given property, method, or signal name exists in an object with the `in` operator:\n\n```gdscript\nvar node = Node.new()\nprint(\"name\" in node)         # Prints true\nprint(\"get_parent\" in node)   # Prints true\nprint(\"tree_entered\" in node) # Prints true\nprint(\"unknown\" in node)      # Prints false\n```\n\nNotifications are `int` constants commonly sent and received by objects. For example, on every rendered frame, the [`SceneTree`][crate::classes::SceneTree] notifies nodes inside the tree with a [`NodeNotification::PROCESS`][`crate::classes::notify::NodeNotification::PROCESS`]. The nodes receive it and may call [`process`][`crate::classes::INode::process`] to update. To make use of notifications, see [`notify`][`crate::classes::Object::notify`] and [`on_notification`][`crate::classes::IObject::on_notification`].\n\nLastly, every object can also contain metadata (data about data). [`set_meta`][`crate::classes::Object::set_meta`] can be useful to store information that the object itself does not depend on. To keep your code clean, making excessive use of metadata is discouraged.\n\n**Note:** Unlike references to a [`RefCounted`][crate::classes::RefCounted], references to an object stored in a variable can become invalid without being set to `null`. To check if an object has been deleted, do _not_ compare it against `null`. Instead, use [`is_instance_valid`][`crate::obj::Gd::is_instance_valid`]. It's also recommended to inherit from [`RefCounted`][crate::classes::RefCounted] for classes storing data instead of `Object`.\n\n**Note:** The `script` is not exposed like most properties. To set or get an object's [`Script`][crate::classes::Script] in code, use [`set_script`][`crate::classes::Object::set_script`] and [`get_script`][`crate::classes::Object::get_script`], respectively.\n\n**Note:** In a boolean context, an `Object` will evaluate to `false` if it is equal to `null` or it has been freed. Otherwise, an `Object` will always evaluate to `true`. See also [`is_instance_valid`][`crate::obj::Gd::is_instance_valid`]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Object {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Object`][crate::classes::Object].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nSee also [Godot docs for `Object` methods](https://docs.godotengine.org/en/stable/classes/class_object.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IObject: crate::obj::GodotClass < Base = Object > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    #[doc = "Notification type for class [`Object`][crate::classes::Object]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum ObjectNotification {
        POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for ObjectNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < ObjectNotification > for i32 {
        fn from(notification: ObjectNotification) -> i32 {
            match notification {
                ObjectNotification::POSTINITIALIZE => 0i32, ObjectNotification::PREDELETE => 1i32, ObjectNotification::EXTENSION_RELOADED => 2i32, ObjectNotification::Unknown(int) => int,
            }
        }
    }
    impl Object {
        #[doc = "Returns the object's built-in class name, as a [`String`][crate::builtin::GString]. See also [`is_class`][`crate::classes::Object::is_class`].\n\n**Note:** This method ignores `class_name` declarations. If this object's script has defined a `class_name`, the base, built-in class name is returned instead."]
        pub fn get_class(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_class", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the object inherits from the given `class`. See also [`get_class`][`crate::classes::Object::get_class`].\n\n\n```gdscript\nvar sprite2d = Sprite2D.new()\nsprite2d.is_class(\"Sprite2D\") # Returns true\nsprite2d.is_class(\"Node\")     # Returns true\nsprite2d.is_class(\"Node3D\")   # Returns false\n```\n\n\n**Note:** This method ignores `class_name` declarations in the object's script."]
        pub fn is_class(&self, class: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "is_class", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns `value` to the given `property`. If the property does not exist or the given `value`'s type doesn't match, nothing happens.\n\n\n```gdscript\nvar node = Node2D.new()\nnode.set(\"global_scale\", Vector2(8, 2.5))\nprint(node.global_scale) # Prints (8.0, 2.5)\n```\n\n\n**Note:** In C#, `property` must be in snake_case when referring to built-in Godot properties. Prefer using the names exposed in the `PropertyName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        pub fn set(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Variant`][crate::builtin::Variant] value of the given `property`. If the `property` does not exist, this method returns `null`.\n\n\n```gdscript\nvar node = Node2D.new()\nnode.rotation = 1.5\nvar a = node.get(\"rotation\") # a is 1.5\n```\n\n\n**Note:** In C#, `property` must be in snake_case when referring to built-in Godot properties. Prefer using the names exposed in the `PropertyName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        pub fn get(&self, property: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns a new `value` to the property identified by the `property_path`. The path should be a [`NodePath`][crate::builtin::NodePath] relative to this object, and can use the colon character (`:`) to access nested properties.\n\n\n```gdscript\nvar node = Node2D.new()\nnode.set_indexed(\"position\", Vector2(42, 0))\nnode.set_indexed(\"position:y\", -10)\nprint(node.position) # Prints (42.0, -10.0)\n```\n\n\n**Note:** In C#, `property_path` must be in snake_case when referring to built-in Godot properties. Prefer using the names exposed in the `PropertyName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        pub fn set_indexed(&mut self, property_path: impl AsArg < NodePath >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, NodePath >, RefArg < 'a1, Variant >,);
            let args = (property_path.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_indexed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the object's property indexed by the given `property_path`. The path should be a [`NodePath`][crate::builtin::NodePath] relative to the current object and can use the colon character (`:`) to access nested properties.\n\n**Examples:** `\"position:x\"` or `\"material:next_pass:blend_mode\"`.\n\n\n```gdscript\nvar node = Node2D.new()\nnode.position = Vector2(5, -10)\nvar a = node.get_indexed(\"position\")   # a is Vector2(5, -10)\nvar b = node.get_indexed(\"position:y\") # b is -10\n```\n\n\n**Note:** In C#, `property_path` must be in snake_case when referring to built-in Godot properties. Prefer using the names exposed in the `PropertyName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call.\n\n**Note:** This method does not support actual paths to nodes in the [`SceneTree`][crate::classes::SceneTree], only sub-property paths. In the context of nodes, use [`get_node_and_resource`][`crate::classes::Node::get_node_and_resource`] instead."]
        pub fn get_indexed(&self, property_path: impl AsArg < NodePath >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (property_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_indexed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the object's property list as an [`Array`][crate::builtin::Array] of dictionaries. Each [`Dictionary`][crate::builtin::Dictionary] contains the following entries:\n\n- `name` is the property's name, as a [`String`][crate::builtin::GString];\n\n- `class_name` is an empty [`StringName`][crate::builtin::StringName], unless the property is [`VariantType::OBJECT`][`crate::builtin::VariantType::OBJECT`] and it inherits from a class;\n\n- `type` is the property's type, as an `int` (see \\[enum Variant.Type]);\n\n- `hint` is _how_ the property is meant to be edited (see \\[enum PropertyHint]);\n\n- `hint_string` depends on the hint (see \\[enum PropertyHint]);\n\n- `usage` is a combination of \\[enum PropertyUsageFlags].\n\n**Note:** In GDScript, all class members are treated as properties. In C# and GDExtension, it may be necessary to explicitly mark class members as Godot properties using decorators or attributes."]
        pub fn get_property_list(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_property_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns this object's methods and their signatures as an [`Array`][crate::builtin::Array] of dictionaries. Each [`Dictionary`][crate::builtin::Dictionary] contains the following entries:\n\n- `name` is the name of the method, as a [`String`][crate::builtin::GString];\n\n- `args` is an [`Array`][crate::builtin::Array] of dictionaries representing the arguments;\n\n- `default_args` is the default arguments as an [`Array`][crate::builtin::Array] of variants;\n\n- `flags` is a combination of \\[enum MethodFlags];\n\n- `id` is the method's internal identifier `int`;\n\n- `return` is the returned value, as a [`Dictionary`][crate::builtin::Dictionary];\n\n**Note:** The dictionaries of `args` and `return` are formatted identically to the results of [`get_property_list`][`crate::classes::Object::get_property_list`], although not all entries are used."]
        pub fn get_method_list(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_method_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `property` has a custom default value. Use [`property_get_revert`][`crate::classes::Object::property_get_revert`] to get the `property`'s default value.\n\n**Note:** This method is used by the Inspector dock to display a revert icon. The object must implement \\[method _property_can_revert] to customize the default value. If \\[method _property_can_revert] is not implemented, this method returns `false`."]
        pub fn property_can_revert(&self, property: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "property_can_revert", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the custom default value of the given `property`. Use [`property_can_revert`][`crate::classes::Object::property_can_revert`] to check if the `property` has a custom default value.\n\n**Note:** This method is used by the Inspector dock to display a revert icon. The object must implement \\[method _property_get_revert] to customize the default value. If \\[method _property_get_revert] is not implemented, this method returns `null`."]
        pub fn property_get_revert(&self, property: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "property_get_revert", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sends the given `what` notification to all classes inherited by the object, triggering calls to [`on_notification`][`crate::classes::IObject::on_notification`], starting from the highest ancestor (the `Object` class) and going down to the object's script.\n\nIf `reversed` is `true`, the call order is reversed.\n\n\n```gdscript\nvar player = Node2D.new()\nplayer.set_script(load(\"res://player.gd\"))\n\nplayer.notification(NOTIFICATION_ENTER_TREE)\n# The call order is Object -> Node -> Node2D -> player.gd.\n\nplayer.notification(NOTIFICATION_ENTER_TREE, true)\n# The call order is player.gd -> Node2D -> Node -> Object.\n```\n"]
        pub(crate) fn notification(&mut self, what: i32, reversed: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (what, reversed,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "notification", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`String`][crate::builtin::GString] representing the object. Defaults to `\"<ClassName#RID>\"`. Override \\[method _to_string] to customize the string representation of the object."]
        pub(crate) fn to_string(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "to_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Attaches `script` to the object, and instantiates it. As a result, the script's [`init`][`crate::classes::IObject::init`] is called. A [`Script`][crate::classes::Script] is used to extend the object's functionality.\n\nIf a script already exists, its instance is detached, and its property values and state are lost. Built-in property values are still kept."]
        pub(crate) fn raw_set_script(&mut self, script: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(script),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "raw_set_script", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the object's [`Script`][crate::classes::Script] instance, or `null` if no script is attached."]
        pub(crate) fn raw_get_script(&self,) -> Variant {
            type CallRet = Variant;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "raw_get_script", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds or changes the entry `name` inside the object's metadata. The metadata `value` can be any [`Variant`][crate::builtin::Variant], although some types cannot be serialized correctly.\n\nIf `value` is `null`, the entry is removed. This is the equivalent of using [`remove_meta`][`crate::classes::Object::remove_meta`]. See also [`has_meta`][`crate::classes::Object::has_meta`] and [`get_meta`][`crate::classes::Object::get_meta`].\n\n**Note:** A metadata's name must be a valid identifier as per [`is_valid_identifier`][`crate::builtin::StringName::is_valid_identifier`] method.\n\n**Note:** Metadata that has a name starting with an underscore (`_`) is considered editor-only. Editor-only metadata is not displayed in the Inspector and should not be edited, although it can still be found by this method."]
        pub fn set_meta(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the given entry `name` from the object's metadata. See also [`has_meta`][`crate::classes::Object::has_meta`], [`get_meta`][`crate::classes::Object::get_meta`] and [`set_meta`][`crate::classes::Object::set_meta`].\n\n**Note:** A metadata's name must be a valid identifier as per [`is_valid_identifier`][`crate::builtin::StringName::is_valid_identifier`] method.\n\n**Note:** Metadata that has a name starting with an underscore (`_`) is considered editor-only. Editor-only metadata is not displayed in the Inspector and should not be edited, although it can still be found by this method."]
        pub fn remove_meta(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "remove_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the object's metadata value for the given entry `name`. If the entry does not exist, returns `default`. If `default` is `null`, an error is also generated.\n\n**Note:** A metadata's name must be a valid identifier as per [`is_valid_identifier`][`crate::builtin::StringName::is_valid_identifier`] method.\n\n**Note:** Metadata that has a name starting with an underscore (`_`) is considered editor-only. Editor-only metadata is not displayed in the Inspector and should not be edited, although it can still be found by this method."]
        pub(crate) fn get_meta_full(&self, name: CowArg < StringName >, default: RefArg < Variant >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name, default,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_meta_ex`][Self::get_meta_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the object's metadata value for the given entry `name`. If the entry does not exist, returns `default`. If `default` is `null`, an error is also generated.\n\n**Note:** A metadata's name must be a valid identifier as per [`is_valid_identifier`][`crate::builtin::StringName::is_valid_identifier`] method.\n\n**Note:** Metadata that has a name starting with an underscore (`_`) is considered editor-only. Editor-only metadata is not displayed in the Inspector and should not be edited, although it can still be found by this method."]
        #[inline]
        pub fn get_meta(&self, name: impl AsArg < StringName >,) -> Variant {
            self.get_meta_ex(name,) . done()
        }
        #[doc = "Returns the object's metadata value for the given entry `name`. If the entry does not exist, returns `default`. If `default` is `null`, an error is also generated.\n\n**Note:** A metadata's name must be a valid identifier as per [`is_valid_identifier`][`crate::builtin::StringName::is_valid_identifier`] method.\n\n**Note:** Metadata that has a name starting with an underscore (`_`) is considered editor-only. Editor-only metadata is not displayed in the Inspector and should not be edited, although it can still be found by this method."]
        #[inline]
        pub fn get_meta_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExGetMeta < 'ex > {
            ExGetMeta::new(self, name,)
        }
        #[doc = "Returns `true` if a metadata entry is found with the given `name`. See also [`get_meta`][`crate::classes::Object::get_meta`], [`set_meta`][`crate::classes::Object::set_meta`] and [`remove_meta`][`crate::classes::Object::remove_meta`].\n\n**Note:** A metadata's name must be a valid identifier as per [`is_valid_identifier`][`crate::builtin::StringName::is_valid_identifier`] method.\n\n**Note:** Metadata that has a name starting with an underscore (`_`) is considered editor-only. Editor-only metadata is not displayed in the Inspector and should not be edited, although it can still be found by this method."]
        pub fn has_meta(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "has_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the object's metadata entry names as an [`Array`][crate::builtin::Array] of [`StringName`][crate::builtin::StringName]s."]
        pub fn get_meta_list(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_meta_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a user-defined signal named `signal`. Optional arguments for the signal can be added as an [`Array`][crate::builtin::Array] of dictionaries, each defining a `name` [`String`][crate::builtin::GString] and a `type` `int` (see \\[enum Variant.Type]). See also [`has_user_signal`][`crate::classes::Object::has_user_signal`] and [`remove_user_signal`][`crate::classes::Object::remove_user_signal`].\n\n\n```gdscript\nadd_user_signal(\"hurt\", [\n\t{ \"name\": \"damage\", \"type\": TYPE_INT },\n\t{ \"name\": \"source\", \"type\": TYPE_OBJECT }\n])\n```\n"]
        pub(crate) fn add_user_signal_full(&mut self, signal: CowArg < GString >, arguments: RefArg < AnyArray >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, AnyArray >,);
            let args = (signal, arguments,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "add_user_signal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_user_signal_ex`][Self::add_user_signal_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a user-defined signal named `signal`. Optional arguments for the signal can be added as an [`Array`][crate::builtin::Array] of dictionaries, each defining a `name` [`String`][crate::builtin::GString] and a `type` `int` (see \\[enum Variant.Type]). See also [`has_user_signal`][`crate::classes::Object::has_user_signal`] and [`remove_user_signal`][`crate::classes::Object::remove_user_signal`].\n\n\n```gdscript\nadd_user_signal(\"hurt\", [\n\t{ \"name\": \"damage\", \"type\": TYPE_INT },\n\t{ \"name\": \"source\", \"type\": TYPE_OBJECT }\n])\n```\n"]
        #[inline]
        pub fn add_user_signal(&mut self, signal: impl AsArg < GString >,) {
            self.add_user_signal_ex(signal,) . done()
        }
        #[doc = "Adds a user-defined signal named `signal`. Optional arguments for the signal can be added as an [`Array`][crate::builtin::Array] of dictionaries, each defining a `name` [`String`][crate::builtin::GString] and a `type` `int` (see \\[enum Variant.Type]). See also [`has_user_signal`][`crate::classes::Object::has_user_signal`] and [`remove_user_signal`][`crate::classes::Object::remove_user_signal`].\n\n\n```gdscript\nadd_user_signal(\"hurt\", [\n\t{ \"name\": \"damage\", \"type\": TYPE_INT },\n\t{ \"name\": \"source\", \"type\": TYPE_OBJECT }\n])\n```\n"]
        #[inline]
        pub fn add_user_signal_ex < 'ex > (&'ex mut self, signal: impl AsArg < GString > + 'ex,) -> ExAddUserSignal < 'ex > {
            ExAddUserSignal::new(self, signal,)
        }
        #[doc = "Returns `true` if the given user-defined `signal` name exists. Only signals added with [`add_user_signal`][`crate::classes::Object::add_user_signal`] are included. See also [`remove_user_signal`][`crate::classes::Object::remove_user_signal`]."]
        pub fn has_user_signal(&self, signal: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "has_user_signal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the given user signal `signal` from the object. See also [`add_user_signal`][`crate::classes::Object::add_user_signal`] and [`has_user_signal`][`crate::classes::Object::has_user_signal`]."]
        pub fn remove_user_signal(&mut self, signal: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "remove_user_signal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Emits the given `signal` by name. The signal must exist, so it should be a built-in signal of this class or one of its inherited classes, or a user-defined signal (see [`add_user_signal`][`crate::classes::Object::add_user_signal`]). This method supports a variable number of arguments, so parameters can be passed as a comma separated list.\n\nReturns [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`] if `signal` does not exist or the parameters are invalid.\n\n\n```gdscript\nemit_signal(\"hit\", \"sword\", 100)\nemit_signal(\"game_over\")\n```\n\n\n**Note:** In C#, `signal` must be in snake_case when referring to built-in Godot signals. Prefer using the names exposed in the `SignalName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn emit_signal(&mut self, signal: impl AsArg < StringName >, varargs: &[Variant]) -> crate::global::Error {
            Self::try_emit_signal(self, signal, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_emit_signal(&mut self, signal: impl AsArg < StringName >, varargs: &[Variant]) -> Result < crate::global::Error, crate::meta::error::CallError > {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(247usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Object", "emit_signal", Some(self.__validated_obj()), args, varargs)
            }
        }
        #[doc = "Calls the `method` on the object and returns the result. This method supports a variable number of arguments, so parameters can be passed as a comma separated list.\n\n\n```gdscript\nvar node = Node3D.new()\nnode.call(\"rotate\", Vector3(1.0, 0.0, 0.0), 1.571)\n```\n\n\n**Note:** In C#, `method` must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the `MethodName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(248usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Object", "call", Some(self.__validated_obj()), args, varargs)
            }
        }
        #[doc = "Calls the `method` on the object during idle time. Always returns `null`, **not** the method's result.\n\nIdle time happens mainly at the end of process and physics frames. In it, deferred calls will be run until there are none left, which means you can defer calls from other deferred calls and they'll still be run in the current idle time cycle. This means you should not call a method deferred from itself (or from a method called by it), as this causes infinite recursion the same way as if you had called the method directly.\n\nThis method supports a variable number of arguments, so parameters can be passed as a comma separated list.\n\n\n```gdscript\nvar node = Node3D.new()\nnode.call_deferred(\"rotate\", Vector3(1.0, 0.0, 0.0), 1.571)\n```\n\n\nFor methods that are deferred from the same thread, the order of execution at idle time is identical to the order in which `call_deferred` was called.\n\nSee also [`call_deferred`][`crate::builtin::Callable::call_deferred`].\n\n**Note:** In C#, `method` must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the `MethodName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call.\n\n**Note:** If you're looking to delay the function call by a frame, refer to the `SceneTree.process_frame` and `SceneTree.physics_frame` signals.\n\n```gdscript\nvar node = Node3D.new()\n# Make a Callable and bind the arguments to the node's rotate() call.\nvar callable = node.rotate.bind(Vector3(1.0, 0.0, 0.0), 1.571)\n# Connect the callable to the process_frame signal, so it gets called in the next process frame.\n# CONNECT_ONE_SHOT makes sure it only gets called once instead of every frame.\nget_tree().process_frame.connect(callable, CONNECT_ONE_SHOT)\n```"]
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_deferred(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call_deferred(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_deferred(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(249usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Object", "call_deferred", Some(self.__validated_obj()), args, varargs)
            }
        }
        #[doc = "Assigns `value` to the given `property`, at the end of the current frame. This is equivalent to calling [`set`][`crate::classes::Object::set`] through [`call_deferred`][`crate::classes::Object::call_deferred`].\n\n\n```gdscript\nvar node = Node2D.new()\nadd_child(node)\n\nnode.rotation = 1.5\nnode.set_deferred(\"rotation\", 3.0)\nprint(node.rotation) # Prints 1.5\n\nawait get_tree().process_frame\nprint(node.rotation) # Prints 3.0\n```\n\n\n**Note:** In C#, `property` must be in snake_case when referring to built-in Godot properties. Prefer using the names exposed in the `PropertyName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        pub fn set_deferred(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_deferred", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calls the `method` on the object and returns the result. Unlike [`call`][`crate::classes::Object::call`], this method expects all parameters to be contained inside `arg_array`.\n\n\n```gdscript\nvar node = Node3D.new()\nnode.callv(\"rotate\", [Vector3(1.0, 0.0, 0.0), 1.571])\n```\n\n\n**Note:** In C#, `method` must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the `MethodName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        pub fn callv(&mut self, method: impl AsArg < StringName >, arg_array: &AnyArray,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, AnyArray >,);
            let args = (method.into_arg(), RefArg::new(arg_array),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "callv", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `method` name exists in the object.\n\n**Note:** In C#, `method` must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the `MethodName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        pub fn has_method(&self, method: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "has_method", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of arguments of the given `method` by name.\n\n**Note:** In C#, `method` must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the `MethodName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        pub fn get_method_argument_count(&self, method: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_method_argument_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `signal` name exists in the object.\n\n**Note:** In C#, `signal` must be in snake_case when referring to built-in Godot signals. Prefer using the names exposed in the `SignalName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        pub fn has_signal(&self, signal: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "has_signal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of existing signals as an [`Array`][crate::builtin::Array] of dictionaries.\n\n**Note:** Due to the implementation, each [`Dictionary`][crate::builtin::Dictionary] is formatted very similarly to the returned values of [`get_method_list`][`crate::classes::Object::get_method_list`]."]
        pub fn get_signal_list(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_signal_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of connections for the given `signal` name. Each connection is represented as a [`Dictionary`][crate::builtin::Dictionary] that contains three entries:\n\n- `signal` is a reference to the [`Signal`][crate::builtin::Signal];\n\n- `callable` is a reference to the connected [`Callable`][crate::builtin::Callable];\n\n- `flags` is a combination of \\[enum ConnectFlags]."]
        pub fn get_signal_connection_list(&self, signal: impl AsArg < StringName >,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_signal_connection_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of signal connections received by this object. Each connection is represented as a [`Dictionary`][crate::builtin::Dictionary] that contains three entries:\n\n- `signal` is a reference to the [`Signal`][crate::builtin::Signal];\n\n- `callable` is a reference to the [`Callable`][crate::builtin::Callable];\n\n- `flags` is a combination of \\[enum ConnectFlags]."]
        pub fn get_incoming_connections(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_incoming_connections", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Connects a `signal` by name to a `callable`. Optional `flags` can be also added to configure the connection's behavior (see \\[enum ConnectFlags] constants).\n\nA signal can only be connected once to the same [`Callable`][crate::builtin::Callable]. If the signal is already connected, this method returns [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] and generates an error, unless the signal is connected with [`ConnectFlags::REFERENCE_COUNTED`][`crate::classes::object::ConnectFlags::REFERENCE_COUNTED`]. To prevent this, use [`is_connected`][`crate::classes::Object::is_connected`] first to check for existing connections.\n\n**Note:** If the `callable`'s object is freed, the connection will be lost.\n\n**Note:** In GDScript, it is generally recommended to connect signals with [`connect`][`crate::builtin::Signal::connect`] instead.\n\n**Note:** This method, and all other signal-related methods, are thread-safe."]
        pub(crate) fn raw_connect_full(&mut self, signal: CowArg < StringName >, callable: RefArg < Callable >, flags: u32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Callable >, u32,);
            let args = (signal, callable, flags,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "raw_connect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`raw_connect_ex`][Self::raw_connect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Connects a `signal` by name to a `callable`. Optional `flags` can be also added to configure the connection's behavior (see \\[enum ConnectFlags] constants).\n\nA signal can only be connected once to the same [`Callable`][crate::builtin::Callable]. If the signal is already connected, this method returns [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] and generates an error, unless the signal is connected with [`ConnectFlags::REFERENCE_COUNTED`][`crate::classes::object::ConnectFlags::REFERENCE_COUNTED`]. To prevent this, use [`is_connected`][`crate::classes::Object::is_connected`] first to check for existing connections.\n\n**Note:** If the `callable`'s object is freed, the connection will be lost.\n\n**Note:** In GDScript, it is generally recommended to connect signals with [`connect`][`crate::builtin::Signal::connect`] instead.\n\n**Note:** This method, and all other signal-related methods, are thread-safe."]
        #[inline]
        pub(crate) fn raw_connect(&mut self, signal: impl AsArg < StringName >, callable: &Callable,) -> crate::global::Error {
            self.raw_connect_ex(signal, callable,) . done()
        }
        #[doc = "Connects a `signal` by name to a `callable`. Optional `flags` can be also added to configure the connection's behavior (see \\[enum ConnectFlags] constants).\n\nA signal can only be connected once to the same [`Callable`][crate::builtin::Callable]. If the signal is already connected, this method returns [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] and generates an error, unless the signal is connected with [`ConnectFlags::REFERENCE_COUNTED`][`crate::classes::object::ConnectFlags::REFERENCE_COUNTED`]. To prevent this, use [`is_connected`][`crate::classes::Object::is_connected`] first to check for existing connections.\n\n**Note:** If the `callable`'s object is freed, the connection will be lost.\n\n**Note:** In GDScript, it is generally recommended to connect signals with [`connect`][`crate::builtin::Signal::connect`] instead.\n\n**Note:** This method, and all other signal-related methods, are thread-safe."]
        #[inline]
        pub(crate) fn raw_connect_ex < 'ex > (&'ex mut self, signal: impl AsArg < StringName > + 'ex, callable: &'ex Callable,) -> ExRawConnect < 'ex > {
            ExRawConnect::new(self, signal, callable,)
        }
        #[doc = "Disconnects a `signal` by name from a given `callable`. If the connection does not exist, generates an error. Use [`is_connected`][`crate::classes::Object::is_connected`] to make sure that the connection exists."]
        pub fn disconnect(&mut self, signal: impl AsArg < StringName >, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Callable >,);
            let args = (signal.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "disconnect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a connection exists between the given `signal` name and `callable`.\n\n**Note:** In C#, `signal` must be in snake_case when referring to built-in Godot signals. Prefer using the names exposed in the `SignalName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        pub fn is_connected(&self, signal: impl AsArg < StringName >, callable: &Callable,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Callable >,);
            let args = (signal.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "is_connected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if any connection exists on the given `signal` name.\n\n**Note:** In C#, `signal` must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the `SignalName` class to avoid allocating a new [`StringName`][crate::builtin::StringName] on each call."]
        pub fn has_connections(&self, signal: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "has_connections", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, the object becomes unable to emit signals. As such, [`emit_signal`][`crate::classes::Object::emit_signal`] and signal connections will not work, until it is set to `false`."]
        pub fn set_block_signals(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_block_signals", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the object is blocking its signals from being emitted. See [`set_block_signals`][`crate::classes::Object::set_block_signals`]."]
        pub fn is_blocking_signals(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "is_blocking_signals", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Emits the `property_list_changed` signal. This is mainly used to refresh the editor, so that the Inspector and editor plugins are properly updated."]
        pub fn notify_property_list_changed(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "notify_property_list_changed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, allows the object to translate messages with [`tr`][`crate::classes::Object::tr`] and [`tr_n`][`crate::classes::Object::tr_n`]. Enabled by default. See also [`can_translate_messages`][`crate::classes::Object::can_translate_messages`]."]
        pub fn set_message_translation(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_message_translation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the object is allowed to translate messages with [`tr`][`crate::classes::Object::tr`] and [`tr_n`][`crate::classes::Object::tr_n`]. See also [`set_message_translation`][`crate::classes::Object::set_message_translation`]."]
        pub fn can_translate_messages(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "can_translate_messages", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Translates a `message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation. Note that most [`Control`][crate::classes::Control] nodes automatically translate their strings, so this method is mostly useful for formatted strings or custom drawn text.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns the `message` without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nFor detailed examples, see [Internationalizing games]($DOCS_URL/tutorials/i18n/internationalizing_games.html).\n\n**Note:** This method can't be used without an `Object` instance, as it requires the [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] method. To translate strings in a static context, use [`translate`][`crate::classes::TranslationServer::translate`]."]
        pub(crate) fn tr_full(&self, message: CowArg < StringName >, context: CowArg < StringName >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "tr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`tr_ex`][Self::tr_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Translates a `message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation. Note that most [`Control`][crate::classes::Control] nodes automatically translate their strings, so this method is mostly useful for formatted strings or custom drawn text.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns the `message` without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nFor detailed examples, see [Internationalizing games]($DOCS_URL/tutorials/i18n/internationalizing_games.html).\n\n**Note:** This method can't be used without an `Object` instance, as it requires the [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] method. To translate strings in a static context, use [`translate`][`crate::classes::TranslationServer::translate`]."]
        #[inline]
        pub fn tr(&self, message: impl AsArg < StringName >,) -> GString {
            self.tr_ex(message,) . done()
        }
        #[doc = "Translates a `message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation. Note that most [`Control`][crate::classes::Control] nodes automatically translate their strings, so this method is mostly useful for formatted strings or custom drawn text.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns the `message` without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nFor detailed examples, see [Internationalizing games]($DOCS_URL/tutorials/i18n/internationalizing_games.html).\n\n**Note:** This method can't be used without an `Object` instance, as it requires the [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] method. To translate strings in a static context, use [`translate`][`crate::classes::TranslationServer::translate`]."]
        #[inline]
        pub fn tr_ex < 'ex > (&'ex self, message: impl AsArg < StringName > + 'ex,) -> ExTr < 'ex > {
            ExTr::new(self, message,)
        }
        #[doc = "Translates a `message` or `plural_message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns `message` or `plural_message`, without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nThe `n` is the number, or amount, of the message's subject. It is used by the translation system to fetch the correct plural form for the current language.\n\nFor detailed examples, see [Localization using gettext]($DOCS_URL/tutorials/i18n/localization_using_gettext.html).\n\n**Note:** Negative and `float` numbers may not properly apply to some countable subjects. It's recommended to handle these cases with [`tr`][`crate::classes::Object::tr`].\n\n**Note:** This method can't be used without an `Object` instance, as it requires the [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] method. To translate strings in a static context, use [`translate_plural`][`crate::classes::TranslationServer::translate_plural`]."]
        pub(crate) fn tr_n_full(&self, message: CowArg < StringName >, plural_message: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >,);
            let args = (message, plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "tr_n", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`tr_n_ex`][Self::tr_n_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Translates a `message` or `plural_message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns `message` or `plural_message`, without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nThe `n` is the number, or amount, of the message's subject. It is used by the translation system to fetch the correct plural form for the current language.\n\nFor detailed examples, see [Localization using gettext]($DOCS_URL/tutorials/i18n/localization_using_gettext.html).\n\n**Note:** Negative and `float` numbers may not properly apply to some countable subjects. It's recommended to handle these cases with [`tr`][`crate::classes::Object::tr`].\n\n**Note:** This method can't be used without an `Object` instance, as it requires the [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] method. To translate strings in a static context, use [`translate_plural`][`crate::classes::TranslationServer::translate_plural`]."]
        #[inline]
        pub fn tr_n(&self, message: impl AsArg < StringName >, plural_message: impl AsArg < StringName >, n: i32,) -> GString {
            self.tr_n_ex(message, plural_message, n,) . done()
        }
        #[doc = "Translates a `message` or `plural_message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns `message` or `plural_message`, without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nThe `n` is the number, or amount, of the message's subject. It is used by the translation system to fetch the correct plural form for the current language.\n\nFor detailed examples, see [Localization using gettext]($DOCS_URL/tutorials/i18n/localization_using_gettext.html).\n\n**Note:** Negative and `float` numbers may not properly apply to some countable subjects. It's recommended to handle these cases with [`tr`][`crate::classes::Object::tr`].\n\n**Note:** This method can't be used without an `Object` instance, as it requires the [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] method. To translate strings in a static context, use [`translate_plural`][`crate::classes::TranslationServer::translate_plural`]."]
        #[inline]
        pub fn tr_n_ex < 'ex > (&'ex self, message: impl AsArg < StringName > + 'ex, plural_message: impl AsArg < StringName > + 'ex, n: i32,) -> ExTrN < 'ex > {
            ExTrN::new(self, message, plural_message, n,)
        }
        #[doc = "Returns the name of the translation domain used by [`tr`][`crate::classes::Object::tr`] and [`tr_n`][`crate::classes::Object::tr_n`]. See also [`TranslationServer`][crate::classes::TranslationServer]."]
        pub fn get_translation_domain(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_translation_domain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the name of the translation domain used by [`tr`][`crate::classes::Object::tr`] and [`tr_n`][`crate::classes::Object::tr_n`]. See also [`TranslationServer`][crate::classes::TranslationServer]."]
        pub fn set_translation_domain(&mut self, domain: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (domain.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_translation_domain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the [`queue_free`][`crate::classes::Node::queue_free`] method was called for the object."]
        pub fn is_queued_for_deletion(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "is_queued_for_deletion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If this method is called during [`ObjectNotification::PREDELETE`][`crate::classes::notify::ObjectNotification::PREDELETE`], this object will reject being freed and will remain allocated. This is mostly an internal function used for error handling to avoid the user from freeing objects when they are not intended to."]
        pub fn cancel_free(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "cancel_free", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what), true);
            
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
        pub(crate) const NOTIFICATION_POSTINITIALIZE: i32 = 0i32;
        pub(crate) const NOTIFICATION_PREDELETE: i32 = 1i32;
        pub(crate) const NOTIFICATION_EXTENSION_RELOADED: i32 = 2i32;
        
    }
    impl crate::obj::GodotClass for Object {
        type Base = crate::obj::NoBase;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Object"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for Object {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemDynamic;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    impl crate::obj::cap::GodotDefault for Object {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Object`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Object__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Object::get_meta_ex`][super::Object::get_meta_ex]."]
#[must_use]
pub struct ExGetMeta < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Object, name: CowArg < 'ex, StringName >, default: CowArg < 'ex, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetMeta < 'ex > {
    fn new(surround_object: &'ex re_export::Object, name: impl AsArg < StringName > + 'ex,) -> Self {
        let default = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), default: CowArg::Owned(default),
        }
    }
    #[inline]
    pub fn default(self, default: &'ex Variant) -> Self {
        Self {
            default: CowArg::Borrowed(default), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, name, default,
        }
        = self;
        re_export::Object::get_meta_full(surround_object, name, default.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`Object::add_user_signal_ex`][super::Object::add_user_signal_ex]."]
#[must_use]
pub struct ExAddUserSignal < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Object, signal: CowArg < 'ex, GString >, arguments: CowArg < 'ex, AnyArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddUserSignal < 'ex > {
    fn new(surround_object: &'ex mut re_export::Object, signal: impl AsArg < GString > + 'ex,) -> Self {
        let arguments = AnyArray::new_untyped();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, signal: signal.into_arg(), arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'ex AnyArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, signal, arguments,
        }
        = self;
        re_export::Object::add_user_signal_full(surround_object, signal, arguments.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`Object::raw_connect_ex`][super::Object::raw_connect_ex]."]
#[must_use]
pub(crate) struct ExRawConnect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Object, signal: CowArg < 'ex, StringName >, callable: CowArg < 'ex, Callable >, flags: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExRawConnect < 'ex > {
    fn new(surround_object: &'ex mut re_export::Object, signal: impl AsArg < StringName > + 'ex, callable: &'ex Callable,) -> Self {
        let flags = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, signal: signal.into_arg(), callable: CowArg::Borrowed(callable), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: u32) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, signal, callable, flags,
        }
        = self;
        re_export::Object::raw_connect_full(surround_object, signal, callable.cow_as_arg(), flags,)
    }
}
#[doc = "Default-param extender for [`Object::tr_ex`][super::Object::tr_ex]."]
#[must_use]
pub struct ExTr < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Object, message: CowArg < 'ex, StringName >, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTr < 'ex > {
    fn new(surround_object: &'ex re_export::Object, message: impl AsArg < StringName > + 'ex,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, context,
        }
        = self;
        re_export::Object::tr_full(surround_object, message, context,)
    }
}
#[doc = "Default-param extender for [`Object::tr_n_ex`][super::Object::tr_n_ex]."]
#[must_use]
pub struct ExTrN < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Object, message: CowArg < 'ex, StringName >, plural_message: CowArg < 'ex, StringName >, n: i32, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTrN < 'ex > {
    fn new(surround_object: &'ex re_export::Object, message: impl AsArg < StringName > + 'ex, plural_message: impl AsArg < StringName > + 'ex, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), plural_message: plural_message.into_arg(), n: n, context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, plural_message, n, context,
        }
        = self;
        re_export::Object::tr_n_full(surround_object, message, plural_message, n, context,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ConnectFlags {
    ord: u64
}
impl ConnectFlags {
    #[doc(alias = "CONNECT_DEFERRED")]
    #[doc = "Godot enumerator name: `CONNECT_DEFERRED`"]
    pub const DEFERRED: ConnectFlags = ConnectFlags {
        ord: 1u64
    };
    #[doc(alias = "CONNECT_PERSIST")]
    #[doc = "Godot enumerator name: `CONNECT_PERSIST`"]
    pub const PERSIST: ConnectFlags = ConnectFlags {
        ord: 2u64
    };
    #[doc(alias = "CONNECT_ONE_SHOT")]
    #[doc = "Godot enumerator name: `CONNECT_ONE_SHOT`"]
    pub const ONE_SHOT: ConnectFlags = ConnectFlags {
        ord: 4u64
    };
    #[doc(alias = "CONNECT_REFERENCE_COUNTED")]
    #[doc = "Godot enumerator name: `CONNECT_REFERENCE_COUNTED`"]
    pub const REFERENCE_COUNTED: ConnectFlags = ConnectFlags {
        ord: 8u64
    };
    #[doc(alias = "CONNECT_APPEND_SOURCE_OBJECT")]
    #[doc = "Godot enumerator name: `CONNECT_APPEND_SOURCE_OBJECT`"]
    pub const APPEND_SOURCE_OBJECT: ConnectFlags = ConnectFlags {
        ord: 16u64
    };
    
}
impl std::fmt::Debug for ConnectFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for ConnectFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ConnectFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFERRED", "CONNECT_DEFERRED", ConnectFlags::DEFERRED), crate::meta::inspect::EnumConstant::new("PERSIST", "CONNECT_PERSIST", ConnectFlags::PERSIST), crate::meta::inspect::EnumConstant::new("ONE_SHOT", "CONNECT_ONE_SHOT", ConnectFlags::ONE_SHOT), crate::meta::inspect::EnumConstant::new("REFERENCE_COUNTED", "CONNECT_REFERENCE_COUNTED", ConnectFlags::REFERENCE_COUNTED), crate::meta::inspect::EnumConstant::new("APPEND_SOURCE_OBJECT", "CONNECT_APPEND_SOURCE_OBJECT", ConnectFlags::APPEND_SOURCE_OBJECT)]
        }
    }
}
impl std::ops::BitOr for ConnectFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ConnectFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ConnectFlags {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Connect Deferred", 1i64), EnumeratorShape::new_int("Connect Persist", 2i64), EnumeratorShape::new_int("Connect One Shot", 4i64), EnumeratorShape::new_int("Connect Reference Counted", 8i64), EnumeratorShape::new_int("Connect Append Source Object", 16i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Object.ConnectFlags")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for ConnectFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ConnectFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ConnectFlags {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* field)
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
impl crate::registry::property::Export for ConnectFlags {
    
}
impl crate::meta::Element for ConnectFlags {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Object;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Object`][crate::classes::Object] class."]
    pub struct SignalsOfObject < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfObject < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn script_changed(&mut self) -> SigScriptChanged < 'c, C > {
            SigScriptChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "script_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn property_list_changed(&mut self) -> SigPropertyListChanged < 'c, C > {
            SigPropertyListChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "property_list_changed")
            }
        }
    }
    type TypedSigScriptChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigScriptChanged < 'c, C: WithSignals > {
        typed: TypedSigScriptChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigScriptChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigScriptChanged < 'c, C > {
        type Target = TypedSigScriptChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigScriptChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPropertyListChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigPropertyListChanged < 'c, C: WithSignals > {
        typed: TypedSigPropertyListChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPropertyListChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPropertyListChanged < 'c, C > {
        type Target = TypedSigPropertyListChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPropertyListChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Object {
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