#![doc = "Sidecar module for class [`LabelSettings`][crate::classes::LabelSettings].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `LabelSettings` enums](https://docs.godotengine.org/en/stable/classes/class_labelsettings.html#enumerations).\n\n"]
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
    #[doc = "Godot class `LabelSettings`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`label_settings`][crate::classes::label_settings]: sidecar module with related enum/flag types\n* [`ILabelSettings`][crate::classes::ILabelSettings]: virtual methods\n\n\nSee also [Godot docs for `LabelSettings`](https://docs.godotengine.org/en/stable/classes/class_labelsettings.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`LabelSettings::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`LabelSettings` is a resource that provides common settings to customize the text in a [`Label`][crate::classes::Label]. It will take priority over the properties defined in \\[member Control.theme]. The resource can be shared between multiple labels and changed on the fly, so it's convenient and flexible way to setup text style."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct LabelSettings {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`LabelSettings`][crate::classes::LabelSettings].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `LabelSettings` methods](https://docs.godotengine.org/en/stable/classes/class_labelsettings.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILabelSettings: crate::obj::GodotClass < Base = LabelSettings > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl LabelSettings {
        pub fn set_line_spacing(&mut self, spacing: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_line_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_spacing(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_line_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_paragraph_spacing(&mut self, spacing: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_paragraph_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_paragraph_spacing(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_paragraph_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font(&mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >,);
            let args = (font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(723usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(727usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_font_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(728usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_font_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outline_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(729usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_outline_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(730usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outline_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_outline_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_outline_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_outline_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_shadow_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_shadow_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(736usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(737usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_shadow_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(738usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_shadow_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stacked_outline_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_outline_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stacked_outline_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_outline_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a new stacked outline to the label at the given `index`. If `index` is `-1`, the new stacked outline will be added at the end of the list."]
        pub(crate) fn add_stacked_outline_full(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "add_stacked_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_stacked_outline_ex`][Self::add_stacked_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new stacked outline to the label at the given `index`. If `index` is `-1`, the new stacked outline will be added at the end of the list."]
        #[inline]
        pub fn add_stacked_outline(&mut self,) {
            self.add_stacked_outline_ex() . done()
        }
        #[doc = "Adds a new stacked outline to the label at the given `index`. If `index` is `-1`, the new stacked outline will be added at the end of the list."]
        #[inline]
        pub fn add_stacked_outline_ex < 'ex > (&'ex mut self,) -> ExAddStackedOutline < 'ex > {
            ExAddStackedOutline::new(self,)
        }
        #[doc = "Moves the stacked outline at index `from_index` to the given position `to_position` in the array."]
        pub fn move_stacked_outline(&mut self, from_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "move_stacked_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the stacked outline at index `index`."]
        pub fn remove_stacked_outline(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "remove_stacked_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the size of the stacked outline identified by the given `index` to `size`."]
        pub fn set_stacked_outline_size(&mut self, index: i32, size: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(744usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the size of the stacked outline at `index`."]
        pub fn get_stacked_outline_size(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(745usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the color of the stacked outline identified by the given `index` to `color`."]
        pub fn set_stacked_outline_color(&mut self, index: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(746usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_outline_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the color of the stacked outline at `index`."]
        pub fn get_stacked_outline_color(&self, index: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(747usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_outline_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stacked_shadow_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(748usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_shadow_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stacked_shadow_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(749usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_shadow_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a new stacked shadow to the label at the given `index`. If `index` is `-1`, the new stacked shadow will be added at the end of the list."]
        pub(crate) fn add_stacked_shadow_full(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(750usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "add_stacked_shadow", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_stacked_shadow_ex`][Self::add_stacked_shadow_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new stacked shadow to the label at the given `index`. If `index` is `-1`, the new stacked shadow will be added at the end of the list."]
        #[inline]
        pub fn add_stacked_shadow(&mut self,) {
            self.add_stacked_shadow_ex() . done()
        }
        #[doc = "Adds a new stacked shadow to the label at the given `index`. If `index` is `-1`, the new stacked shadow will be added at the end of the list."]
        #[inline]
        pub fn add_stacked_shadow_ex < 'ex > (&'ex mut self,) -> ExAddStackedShadow < 'ex > {
            ExAddStackedShadow::new(self,)
        }
        #[doc = "Moves the stacked shadow at index `from_index` to the given position `to_position` in the array."]
        pub fn move_stacked_shadow(&mut self, from_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "move_stacked_shadow", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the stacked shadow at index `index`."]
        pub fn remove_stacked_shadow(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "remove_stacked_shadow", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the offset of the stacked shadow identified by the given `index` to `offset`."]
        pub fn set_stacked_shadow_offset(&mut self, index: i32, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (index, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_shadow_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the offset of the stacked shadow at `index`."]
        pub fn get_stacked_shadow_offset(&self, index: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_shadow_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the color of the stacked shadow identified by the given `index` to `color`."]
        pub fn set_stacked_shadow_color(&mut self, index: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the color of the stacked shadow at `index`."]
        pub fn get_stacked_shadow_color(&self, index: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the outline size of the stacked shadow identified by the given `index` to `size`."]
        pub fn set_stacked_shadow_outline_size(&mut self, index: i32, size: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_shadow_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the outline size of the stacked shadow at `index`."]
        pub fn get_stacked_shadow_outline_size(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_shadow_outline_size", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for LabelSettings {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("LabelSettings"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for LabelSettings {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for LabelSettings {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for LabelSettings {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for LabelSettings {
        
    }
    impl crate::obj::cap::GodotDefault for LabelSettings {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for LabelSettings {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for LabelSettings {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`LabelSettings`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_LabelSettings__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::LabelSettings > for $Class {
                
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
#[doc = "Default-param extender for [`LabelSettings::add_stacked_outline_ex`][super::LabelSettings::add_stacked_outline_ex]."]
#[must_use]
pub struct ExAddStackedOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::LabelSettings, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddStackedOutline < 'ex > {
    fn new(surround_object: &'ex mut re_export::LabelSettings,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, index,
        }
        = self;
        re_export::LabelSettings::add_stacked_outline_full(surround_object, index,)
    }
}
#[doc = "Default-param extender for [`LabelSettings::add_stacked_shadow_ex`][super::LabelSettings::add_stacked_shadow_ex]."]
#[must_use]
pub struct ExAddStackedShadow < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::LabelSettings, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddStackedShadow < 'ex > {
    fn new(surround_object: &'ex mut re_export::LabelSettings,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, index,
        }
        = self;
        re_export::LabelSettings::add_stacked_shadow_full(surround_object, index,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::LabelSettings;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for LabelSettings {
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