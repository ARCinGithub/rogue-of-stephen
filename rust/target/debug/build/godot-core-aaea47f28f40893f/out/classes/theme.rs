#![doc = "Sidecar module for class [`Theme`][crate::classes::Theme].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Theme` enums](https://docs.godotengine.org/en/stable/classes/class_theme.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Theme`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`theme`][crate::classes::theme]: sidecar module with related enum/flag types\n* [`ITheme`][crate::classes::ITheme]: virtual methods\n\n\nSee also [Godot docs for `Theme`](https://docs.godotengine.org/en/stable/classes/class_theme.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Theme::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA resource used for styling/skinning [`Control`][crate::classes::Control] and [`Window`][crate::classes::Window] nodes. While individual controls can be styled using their local theme overrides (see [`add_theme_color_override`][`crate::classes::Control::add_theme_color_override`]), theme resources allow you to store and apply the same settings across all controls sharing the same type (e.g. style all [`Button`][crate::classes::Button]s the same). One theme resource can be used for the entire project, but you can also set a separate theme resource to a branch of control nodes. A theme resource assigned to a control applies to the control itself, as well as all of its direct and indirect children (as long as a chain of controls is uninterrupted).\n\nUse \\[member ProjectSettings.gui/theme/custom] to set up a project-scope theme that will be available to every control in your project.\n\nUse \\[member Control.theme] of any control node to set up a theme that will be available to that control and all of its direct and indirect children."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Theme {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Theme`][crate::classes::Theme].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Theme` methods](https://docs.godotengine.org/en/stable/classes/class_theme.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITheme: crate::obj::GodotClass < Base = Theme > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Theme {
        #[doc = "Creates or changes the value of the icon property defined by `name` and `theme_type`. Use [`clear_icon`][`crate::classes::Theme::clear_icon`] to remove the property."]
        pub fn set_icon(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (name.into_arg(), theme_type.into_arg(), texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the icon property defined by `name` and `theme_type`, if it exists.\n\nReturns the engine fallback icon value if the property doesn't exist (see \\[member ThemeDB.fallback_icon]). Use [`has_icon`][`crate::classes::Theme::has_icon`] to check for existence."]
        pub fn get_icon(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the icon property defined by `name` and `theme_type` exists.\n\nReturns `false` if it doesn't exist. Use [`set_icon`][`crate::classes::Theme::set_icon`] to define it."]
        pub fn has_icon(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renames the icon property defined by `old_name` and `theme_type` to `name`, if it exists.\n\nFails if it doesn't exist, or if a similar property with the new name already exists. Use [`has_icon`][`crate::classes::Theme::has_icon`] to check for existence, and [`clear_icon`][`crate::classes::Theme::clear_icon`] to remove the existing property."]
        pub fn rename_icon(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the icon property defined by `name` and `theme_type`, if it exists.\n\nFails if it doesn't exist. Use [`has_icon`][`crate::classes::Theme::has_icon`] to check for existence."]
        pub fn clear_icon(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of names for icon properties defined with `theme_type`. Use [`get_icon_type_list`][`crate::classes::Theme::get_icon_type_list`] to get a list of possible theme type names."]
        pub fn get_icon_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_icon_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of all unique theme type names for icon properties. Use [`get_type_list`][`crate::classes::Theme::get_type_list`] to get a list of all unique theme types."]
        pub fn get_icon_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_icon_type_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates or changes the value of the [`StyleBox`][crate::classes::StyleBox] property defined by `name` and `theme_type`. Use [`clear_stylebox`][`crate::classes::Theme::clear_stylebox`] to remove the property."]
        pub fn set_stylebox(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, texture: impl AsArg < Option < Gd < crate::classes::StyleBox >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, Option < Gd < crate::classes::StyleBox > > >,);
            let args = (name.into_arg(), theme_type.into_arg(), texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`StyleBox`][crate::classes::StyleBox] property defined by `name` and `theme_type`, if it exists.\n\nReturns the engine fallback stylebox value if the property doesn't exist (see \\[member ThemeDB.fallback_stylebox]). Use [`has_stylebox`][`crate::classes::Theme::has_stylebox`] to check for existence."]
        pub fn get_stylebox(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> Option < Gd < crate::classes::StyleBox > > {
            type CallRet = Option < Gd < crate::classes::StyleBox > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the [`StyleBox`][crate::classes::StyleBox] property defined by `name` and `theme_type` exists.\n\nReturns `false` if it doesn't exist. Use [`set_stylebox`][`crate::classes::Theme::set_stylebox`] to define it."]
        pub fn has_stylebox(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renames the [`StyleBox`][crate::classes::StyleBox] property defined by `old_name` and `theme_type` to `name`, if it exists.\n\nFails if it doesn't exist, or if a similar property with the new name already exists. Use [`has_stylebox`][`crate::classes::Theme::has_stylebox`] to check for existence, and [`clear_stylebox`][`crate::classes::Theme::clear_stylebox`] to remove the existing property."]
        pub fn rename_stylebox(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the [`StyleBox`][crate::classes::StyleBox] property defined by `name` and `theme_type`, if it exists.\n\nFails if it doesn't exist. Use [`has_stylebox`][`crate::classes::Theme::has_stylebox`] to check for existence."]
        pub fn clear_stylebox(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of names for [`StyleBox`][crate::classes::StyleBox] properties defined with `theme_type`. Use [`get_stylebox_type_list`][`crate::classes::Theme::get_stylebox_type_list`] to get a list of possible theme type names."]
        pub fn get_stylebox_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_stylebox_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of all unique theme type names for [`StyleBox`][crate::classes::StyleBox] properties. Use [`get_type_list`][`crate::classes::Theme::get_type_list`] to get a list of all unique theme types."]
        pub fn get_stylebox_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_stylebox_type_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates or changes the value of the [`Font`][crate::classes::Font] property defined by `name` and `theme_type`. Use [`clear_font`][`crate::classes::Theme::clear_font`] to remove the property."]
        pub fn set_font(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, Option < Gd < crate::classes::Font > > >,);
            let args = (name.into_arg(), theme_type.into_arg(), font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Font`][crate::classes::Font] property defined by `name` and `theme_type`, if it exists.\n\nReturns the default theme font if the property doesn't exist and the default theme font is set up (see \\[member default_font]). Use [`has_font`][`crate::classes::Theme::has_font`] to check for existence of the property and [`has_default_font`][`crate::classes::Theme::has_default_font`] to check for existence of the default theme font.\n\nReturns the engine fallback font value, if neither exist (see \\[member ThemeDB.fallback_font])."]
        pub fn get_font(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the [`Font`][crate::classes::Font] property defined by `name` and `theme_type` exists, or if the default theme font is set up (see [`has_default_font`][`crate::classes::Theme::has_default_font`]).\n\nReturns `false` if neither exist. Use [`set_font`][`crate::classes::Theme::set_font`] to define the property."]
        pub fn has_font(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renames the [`Font`][crate::classes::Font] property defined by `old_name` and `theme_type` to `name`, if it exists.\n\nFails if it doesn't exist, or if a similar property with the new name already exists. Use [`has_font`][`crate::classes::Theme::has_font`] to check for existence, and [`clear_font`][`crate::classes::Theme::clear_font`] to remove the existing property."]
        pub fn rename_font(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the [`Font`][crate::classes::Font] property defined by `name` and `theme_type`, if it exists.\n\nFails if it doesn't exist. Use [`has_font`][`crate::classes::Theme::has_font`] to check for existence."]
        pub fn clear_font(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of names for [`Font`][crate::classes::Font] properties defined with `theme_type`. Use [`get_font_type_list`][`crate::classes::Theme::get_font_type_list`] to get a list of possible theme type names."]
        pub fn get_font_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of all unique theme type names for [`Font`][crate::classes::Font] properties. Use [`get_type_list`][`crate::classes::Theme::get_type_list`] to get a list of all unique theme types."]
        pub fn get_font_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font_type_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates or changes the value of the font size property defined by `name` and `theme_type`. Use [`clear_font_size`][`crate::classes::Theme::clear_font_size`] to remove the property."]
        pub fn set_font_size(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, font_size: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32,);
            let args = (name.into_arg(), theme_type.into_arg(), font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the font size property defined by `name` and `theme_type`, if it exists.\n\nReturns the default theme font size if the property doesn't exist and the default theme font size is set up (see \\[member default_font_size]). Use [`has_font_size`][`crate::classes::Theme::has_font_size`] to check for existence of the property and [`has_default_font_size`][`crate::classes::Theme::has_default_font_size`] to check for existence of the default theme font.\n\nReturns the engine fallback font size value, if neither exist (see \\[member ThemeDB.fallback_font_size])."]
        pub fn get_font_size(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the font size property defined by `name` and `theme_type` exists, or if the default theme font size is set up (see [`has_default_font_size`][`crate::classes::Theme::has_default_font_size`]).\n\nReturns `false` if neither exist. Use [`set_font_size`][`crate::classes::Theme::set_font_size`] to define the property."]
        pub fn has_font_size(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renames the font size property defined by `old_name` and `theme_type` to `name`, if it exists.\n\nFails if it doesn't exist, or if a similar property with the new name already exists. Use [`has_font_size`][`crate::classes::Theme::has_font_size`] to check for existence, and [`clear_font_size`][`crate::classes::Theme::clear_font_size`] to remove the existing property."]
        pub fn rename_font_size(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the font size property defined by `name` and `theme_type`, if it exists.\n\nFails if it doesn't exist. Use [`has_font_size`][`crate::classes::Theme::has_font_size`] to check for existence."]
        pub fn clear_font_size(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of names for font size properties defined with `theme_type`. Use [`get_font_size_type_list`][`crate::classes::Theme::get_font_size_type_list`] to get a list of possible theme type names."]
        pub fn get_font_size_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font_size_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of all unique theme type names for font size properties. Use [`get_type_list`][`crate::classes::Theme::get_type_list`] to get a list of all unique theme types."]
        pub fn get_font_size_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font_size_type_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates or changes the value of the [`Color`][crate::builtin::Color] property defined by `name` and `theme_type`. Use [`clear_color`][`crate::classes::Theme::clear_color`] to remove the property."]
        pub fn set_color(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, Color,);
            let args = (name.into_arg(), theme_type.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Color`][crate::builtin::Color] property defined by `name` and `theme_type`, if it exists.\n\nReturns the default color value if the property doesn't exist. Use [`has_color`][`crate::classes::Theme::has_color`] to check for existence."]
        pub fn get_color(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> Color {
            type CallRet = Color;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the [`Color`][crate::builtin::Color] property defined by `name` and `theme_type` exists.\n\nReturns `false` if it doesn't exist. Use [`set_color`][`crate::classes::Theme::set_color`] to define it."]
        pub fn has_color(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renames the [`Color`][crate::builtin::Color] property defined by `old_name` and `theme_type` to `name`, if it exists.\n\nFails if it doesn't exist, or if a similar property with the new name already exists. Use [`has_color`][`crate::classes::Theme::has_color`] to check for existence, and [`clear_color`][`crate::classes::Theme::clear_color`] to remove the existing property."]
        pub fn rename_color(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the [`Color`][crate::builtin::Color] property defined by `name` and `theme_type`, if it exists.\n\nFails if it doesn't exist. Use [`has_color`][`crate::classes::Theme::has_color`] to check for existence."]
        pub fn clear_color(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of names for [`Color`][crate::builtin::Color] properties defined with `theme_type`. Use [`get_color_type_list`][`crate::classes::Theme::get_color_type_list`] to get a list of possible theme type names."]
        pub fn get_color_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_color_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of all unique theme type names for [`Color`][crate::builtin::Color] properties. Use [`get_type_list`][`crate::classes::Theme::get_type_list`] to get a list of all unique theme types."]
        pub fn get_color_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_color_type_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates or changes the value of the constant property defined by `name` and `theme_type`. Use [`clear_constant`][`crate::classes::Theme::clear_constant`] to remove the property."]
        pub fn set_constant(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, constant: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32,);
            let args = (name.into_arg(), theme_type.into_arg(), constant,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_constant", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the constant property defined by `name` and `theme_type`, if it exists.\n\nReturns `0` if the property doesn't exist. Use [`has_constant`][`crate::classes::Theme::has_constant`] to check for existence."]
        pub fn get_constant(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_constant", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the constant property defined by `name` and `theme_type` exists.\n\nReturns `false` if it doesn't exist. Use [`set_constant`][`crate::classes::Theme::set_constant`] to define it."]
        pub fn has_constant(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_constant", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renames the constant property defined by `old_name` and `theme_type` to `name`, if it exists.\n\nFails if it doesn't exist, or if a similar property with the new name already exists. Use [`has_constant`][`crate::classes::Theme::has_constant`] to check for existence, and [`clear_constant`][`crate::classes::Theme::clear_constant`] to remove the existing property."]
        pub fn rename_constant(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_constant", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the constant property defined by `name` and `theme_type`, if it exists.\n\nFails if it doesn't exist. Use [`has_constant`][`crate::classes::Theme::has_constant`] to check for existence."]
        pub fn clear_constant(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_constant", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of names for constant properties defined with `theme_type`. Use [`get_constant_type_list`][`crate::classes::Theme::get_constant_type_list`] to get a list of possible theme type names."]
        pub fn get_constant_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_constant_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of all unique theme type names for constant properties. Use [`get_type_list`][`crate::classes::Theme::get_type_list`] to get a list of all unique theme types."]
        pub fn get_constant_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_constant_type_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_base_scale(&mut self, base_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (base_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(912usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_default_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_base_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(913usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_default_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if \\[member default_base_scale] has a valid value.\n\nReturns `false` if it doesn't. The value must be greater than `0.0` to be considered valid."]
        pub fn has_default_base_scale(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(914usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_default_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_font(&mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >,);
            let args = (font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_default_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_default_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if \\[member default_font] has a valid value.\n\nReturns `false` if it doesn't."]
        pub fn has_default_font(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_default_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_font_size(&mut self, font_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_default_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_font_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_default_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if \\[member default_font_size] has a valid value.\n\nReturns `false` if it doesn't. The value must be greater than `0` to be considered valid."]
        pub fn has_default_font_size(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_default_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates or changes the value of the theme property of `data_type` defined by `name` and `theme_type`. Use [`clear_theme_item`][`crate::classes::Theme::clear_theme_item`] to remove the property.\n\nFails if the `value` type is not accepted by `data_type`.\n\n**Note:** This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic."]
        pub fn set_theme_item(&mut self, data_type: crate::classes::theme::DataType, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (crate::classes::theme::DataType, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, RefArg < 'a2, Variant >,);
            let args = (data_type, name.into_arg(), theme_type.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_theme_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the theme property of `data_type` defined by `name` and `theme_type`, if it exists.\n\nReturns the engine fallback value if the property doesn't exist (see [`ThemeDB`][crate::classes::ThemeDb]). Use [`has_theme_item`][`crate::classes::Theme::has_theme_item`] to check for existence.\n\n**Note:** This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic."]
        pub fn get_theme_item(&self, data_type: crate::classes::theme::DataType, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (crate::classes::theme::DataType, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (data_type, name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_theme_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the theme property of `data_type` defined by `name` and `theme_type` exists.\n\nReturns `false` if it doesn't exist. Use [`set_theme_item`][`crate::classes::Theme::set_theme_item`] to define it.\n\n**Note:** This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic."]
        pub fn has_theme_item(&self, data_type: crate::classes::theme::DataType, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (crate::classes::theme::DataType, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (data_type, name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_theme_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renames the theme property of `data_type` defined by `old_name` and `theme_type` to `name`, if it exists.\n\nFails if it doesn't exist, or if a similar property with the new name already exists. Use [`has_theme_item`][`crate::classes::Theme::has_theme_item`] to check for existence, and [`clear_theme_item`][`crate::classes::Theme::clear_theme_item`] to remove the existing property.\n\n**Note:** This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic."]
        pub fn rename_theme_item(&mut self, data_type: crate::classes::theme::DataType, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (crate::classes::theme::DataType, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (data_type, old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_theme_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the theme property of `data_type` defined by `name` and `theme_type`, if it exists.\n\nFails if it doesn't exist. Use [`has_theme_item`][`crate::classes::Theme::has_theme_item`] to check for existence.\n\n**Note:** This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic."]
        pub fn clear_theme_item(&mut self, data_type: crate::classes::theme::DataType, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (crate::classes::theme::DataType, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (data_type, name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_theme_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of names for properties of `data_type` defined with `theme_type`. Use [`get_theme_item_type_list`][`crate::classes::Theme::get_theme_item_type_list`] to get a list of possible theme type names.\n\n**Note:** This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic."]
        pub fn get_theme_item_list(&self, data_type: crate::classes::theme::DataType, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (crate::classes::theme::DataType, CowArg < 'a0, GString >,);
            let args = (data_type, theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_theme_item_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of all unique theme type names for `data_type` properties. Use [`get_type_list`][`crate::classes::Theme::get_type_list`] to get a list of all unique theme types.\n\n**Note:** This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic."]
        pub fn get_theme_item_type_list(&self, data_type: crate::classes::theme::DataType,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (crate::classes::theme::DataType,);
            let args = (data_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_theme_item_type_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Marks `theme_type` as a variation of `base_type`.\n\nThis adds `theme_type` as a suggested option for \\[member Control.theme_type_variation] on a [`Control`][crate::classes::Control] that is of the `base_type` class.\n\nVariations can also be nested, i.e. `base_type` can be another variation. If a chain of variations ends with a `base_type` matching the class of the [`Control`][crate::classes::Control], the whole chain is going to be suggested as options.\n\n**Note:** Suggestions only show up if this theme resource is set as the project default theme. See \\[member ProjectSettings.gui/theme/custom]."]
        pub fn set_type_variation(&mut self, theme_type: impl AsArg < StringName >, base_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (theme_type.into_arg(), base_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_type_variation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if `theme_type` is marked as a variation of `base_type`."]
        pub fn is_type_variation(&self, theme_type: impl AsArg < StringName >, base_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (theme_type.into_arg(), base_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(929usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "is_type_variation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unmarks `theme_type` as being a variation of another theme type. See [`set_type_variation`][`crate::classes::Theme::set_type_variation`]."]
        pub fn clear_type_variation(&mut self, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(930usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_type_variation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the base theme type if `theme_type` is a valid variation type. Returns an empty string otherwise."]
        pub fn get_type_variation_base(&self, theme_type: impl AsArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(931usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_type_variation_base", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of all type variations for the given `base_type`."]
        pub fn get_type_variation_list(&self, base_type: impl AsArg < StringName >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (base_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(932usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_type_variation_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an empty theme type for every valid data type.\n\n**Note:** Empty types are not saved with the theme. This method only exists to perform in-memory changes to the resource. Use available `set_*` methods to add theme items."]
        pub fn add_type(&mut self, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "add_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the theme type, gracefully discarding defined theme items. If the type is a variation, this information is also erased. If the type is a base for type variations, those variations lose their base."]
        pub fn remove_type(&mut self, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "remove_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renames the theme type `old_theme_type` to `theme_type`, if the old type exists and the new one doesn't exist.\n\n**Note:** Renaming a theme type to an empty name or a variation to a type associated with a built-in class removes type variation connections in a way that cannot be undone by reversing the rename alone."]
        pub fn rename_type(&mut self, old_theme_type: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (old_theme_type.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of all unique theme type names. Use the appropriate `get_*_type_list` method to get a list of unique theme types for a single data type."]
        pub fn get_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_type_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds missing and overrides existing definitions with values from the `other` theme resource.\n\n**Note:** This modifies the current theme. If you want to merge two themes together without modifying either one, create a new empty theme and merge the other two into it one after another."]
        pub fn merge_with(&mut self, other: impl AsArg < Option < Gd < crate::classes::Theme >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Theme > > >,);
            let args = (other.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "merge_with", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all the theme properties defined on the theme resource."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Theme {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Theme"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Theme {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Theme {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Theme {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Theme {
        
    }
    impl crate::obj::cap::GodotDefault for Theme {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Theme {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Theme {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Theme`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Theme__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Theme > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DataType {
    ord: i32
}
impl DataType {
    #[doc(alias = "DATA_TYPE_COLOR")]
    #[doc = "Godot enumerator name: `DATA_TYPE_COLOR`"]
    pub const COLOR: DataType = DataType {
        ord: 0i32
    };
    #[doc(alias = "DATA_TYPE_CONSTANT")]
    #[doc = "Godot enumerator name: `DATA_TYPE_CONSTANT`"]
    pub const CONSTANT: DataType = DataType {
        ord: 1i32
    };
    #[doc(alias = "DATA_TYPE_FONT")]
    #[doc = "Godot enumerator name: `DATA_TYPE_FONT`"]
    pub const FONT: DataType = DataType {
        ord: 2i32
    };
    #[doc(alias = "DATA_TYPE_FONT_SIZE")]
    #[doc = "Godot enumerator name: `DATA_TYPE_FONT_SIZE`"]
    pub const FONT_SIZE: DataType = DataType {
        ord: 3i32
    };
    #[doc(alias = "DATA_TYPE_ICON")]
    #[doc = "Godot enumerator name: `DATA_TYPE_ICON`"]
    pub const ICON: DataType = DataType {
        ord: 4i32
    };
    #[doc(alias = "DATA_TYPE_STYLEBOX")]
    #[doc = "Godot enumerator name: `DATA_TYPE_STYLEBOX`"]
    pub const STYLEBOX: DataType = DataType {
        ord: 5i32
    };
    #[doc(alias = "DATA_TYPE_MAX")]
    #[doc = "Godot enumerator name: `DATA_TYPE_MAX`"]
    pub const MAX: DataType = DataType {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DataType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DataType {
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
            Self::COLOR => "COLOR", Self::CONSTANT => "CONSTANT", Self::FONT => "FONT", Self::FONT_SIZE => "FONT_SIZE", Self::ICON => "ICON", Self::STYLEBOX => "STYLEBOX", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DataType::COLOR, DataType::CONSTANT, DataType::FONT, DataType::FONT_SIZE, DataType::ICON, DataType::STYLEBOX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DataType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("COLOR", "DATA_TYPE_COLOR", DataType::COLOR), crate::meta::inspect::EnumConstant::new("CONSTANT", "DATA_TYPE_CONSTANT", DataType::CONSTANT), crate::meta::inspect::EnumConstant::new("FONT", "DATA_TYPE_FONT", DataType::FONT), crate::meta::inspect::EnumConstant::new("FONT_SIZE", "DATA_TYPE_FONT_SIZE", DataType::FONT_SIZE), crate::meta::inspect::EnumConstant::new("ICON", "DATA_TYPE_ICON", DataType::ICON), crate::meta::inspect::EnumConstant::new("STYLEBOX", "DATA_TYPE_STYLEBOX", DataType::STYLEBOX), crate::meta::inspect::EnumConstant::new("MAX", "DATA_TYPE_MAX", DataType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DataType {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for DataType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Data Type Color", 0i64), EnumeratorShape::new_int("Data Type Constant", 1i64), EnumeratorShape::new_int("Data Type Font", 2i64), EnumeratorShape::new_int("Data Type Font Size", 3i64), EnumeratorShape::new_int("Data Type Icon", 4i64), EnumeratorShape::new_int("Data Type Stylebox", 5i64), EnumeratorShape::new_int("Data Type Max", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Theme.DataType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DataType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DataType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DataType {
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
impl crate::registry::property::Export for DataType {
    
}
impl crate::meta::Element for DataType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Theme;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Theme {
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