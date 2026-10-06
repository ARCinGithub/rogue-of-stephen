#![doc = "Sidecar module for class [`OpenXrApiExtension`][crate::classes::OpenXrApiExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRAPIExtension` enums](https://docs.godotengine.org/en/stable/classes/class_openxrapiextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRAPIExtension`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`open_xr_api_extension`][crate::classes::open_xr_api_extension]: sidecar module with related enum/flag types\n* [`IOpenXrApiExtension`][crate::classes::IOpenXrApiExtension]: virtual methods\n\n\nSee also [Godot docs for `OpenXRAPIExtension`](https://docs.godotengine.org/en/stable/classes/class_openxrapiextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`OpenXrApiExtension::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`OpenXRAPIExtension` makes OpenXR available for GDExtension. It provides the OpenXR API to GDExtension through the [`get_instance_proc_addr`][`crate::classes::OpenXrApiExtension::get_instance_proc_addr`] method, and the OpenXR instance through [`get_instance`][`crate::classes::OpenXrApiExtension::get_instance`].\n\nIt also provides methods for querying the status of OpenXR initialization, and helper methods for ease of use of the API with GDExtension."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrApiExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrApiExtension`][crate::classes::OpenXrApiExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `OpenXRAPIExtension` methods](https://docs.godotengine.org/en/stable/classes/class_openxrapiextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrApiExtension: crate::obj::GodotClass < Base = OpenXrApiExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrApiExtension {
        #[doc = "Returns the version of OpenXR that was initialized. Only valid after the OpenXR instance has been created. See [XR_MAKE_VERSION](https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_MAKE_VERSION) for how the version is calculated."]
        pub fn get_openxr_version(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_openxr_version", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [XrInstance](https://registry.khronos.org/OpenXR/specs/1.0/man/html/XrInstance.html) created during the initialization of the OpenXR API."]
        pub fn get_instance(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_instance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID of the system, which is an [XrSystemId](https://registry.khronos.org/OpenXR/specs/1.0/man/html/XrSystemId.html) cast to an integer."]
        pub fn get_system_id(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_system_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the OpenXR session, which is an [XrSession](https://registry.khronos.org/OpenXR/specs/1.0/man/html/XrSession.html) cast to an integer."]
        pub fn get_session(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_session", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "\n# Godot docs\nCreates a [`Transform3D`][crate::builtin::Transform3D] from an [XrPosef](https://registry.khronos.org/OpenXR/specs/1.0/man/html/XrPosef.html)."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        pub unsafe fn transform_from_pose(&mut self, pose: crate::meta::RawPtr < * const c_void >,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (crate::meta::RawPtr < * const c_void >,);
            let args = (pose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "transform_from_pose", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the provided [XrResult](https://registry.khronos.org/OpenXR/specs/1.0/man/html/XrResult.html) (cast to an integer) is successful. Otherwise returns `false` and prints the [XrResult](https://registry.khronos.org/OpenXR/specs/1.0/man/html/XrResult.html) converted to a string, with the specified additional information."]
        pub fn xr_result(&mut self, result: u64, format: impl AsArg < GString >, args: &AnyArray,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (u64, CowArg < 'a0, GString >, RefArg < 'a1, AnyArray >,);
            let args = (result, format.into_arg(), RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "xr_result", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if OpenXR is enabled."]
        pub fn openxr_is_enabled(check_run_in_editor: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (bool,);
            let args = (check_run_in_editor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_is_enabled", None, args,)
            }
        }
        #[doc = "Returns the function pointer of the OpenXR function with the specified name, cast to an integer. If the function with the given name does not exist, the method returns `0`.\n\n**Note:** `openxr/util.h` contains utility macros for acquiring OpenXR functions, e.g. `GDEXTENSION_INIT_XR_FUNC_V(xrCreateAction)`."]
        pub fn get_instance_proc_addr(&self, name: impl AsArg < GString >,) -> u64 {
            type CallRet = u64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_instance_proc_addr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an error string for the given [XrResult](https://registry.khronos.org/OpenXR/specs/1.0/man/html/XrResult.html)."]
        pub fn get_error_string(&self, result: u64,) -> GString {
            type CallRet = GString;
            type CallParams = (u64,);
            let args = (result,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_error_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the specified swapchain format."]
        pub fn get_swapchain_format_name(&self, swapchain_format: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (swapchain_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_swapchain_format_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set the object name of an OpenXR object, used for debug output. `object_type` must be a valid OpenXR `XrObjectType` enum and `object_handle` must be a valid OpenXR object handle."]
        pub fn set_object_name(&mut self, object_type: i64, object_handle: u64, object_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i64, u64, CowArg < 'a0, GString >,);
            let args = (object_type, object_handle, object_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_object_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Begins a new debug label region, this label will be reported in debug messages for any calls following this until [`end_debug_label_region`][`crate::classes::OpenXrApiExtension::end_debug_label_region`] is called. Debug labels can be stacked."]
        pub fn begin_debug_label_region(&mut self, label_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (label_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "begin_debug_label_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Marks the end of a debug label region. Removes the latest debug label region added by calling [`begin_debug_label_region`][`crate::classes::OpenXrApiExtension::begin_debug_label_region`]."]
        pub fn end_debug_label_region(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "end_debug_label_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Inserts a debug label, this label is reported in any debug message resulting from the OpenXR calls that follows, until any of [`begin_debug_label_region`][`crate::classes::OpenXrApiExtension::begin_debug_label_region`], [`end_debug_label_region`][`crate::classes::OpenXrApiExtension::end_debug_label_region`], or [`insert_debug_label`][`crate::classes::OpenXrApiExtension::insert_debug_label`] is called."]
        pub fn insert_debug_label(&mut self, label_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (label_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "insert_debug_label", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if OpenXR is initialized."]
        pub fn is_initialized(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "is_initialized", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if OpenXR is running ([xrBeginSession](https://registry.khronos.org/OpenXR/specs/1.0/man/html/xrBeginSession.html) was successfully called and the swapchains were created)."]
        pub fn is_running(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "is_running", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "\n# Godot docs\nSets the reference space used by OpenXR to the given [XrSpace](https://registry.khronos.org/OpenXR/specs/1.0/man/html/XrSpace.html) (cast to a `void *`)."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        pub unsafe fn set_custom_play_space(&mut self, space: crate::meta::RawPtr < * const c_void >,) {
            type CallRet = ();
            type CallParams = (crate::meta::RawPtr < * const c_void >,);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_custom_play_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the play space, which is an [XrSpace](https://registry.khronos.org/OpenXR/specs/1.0/man/html/XrSpace.html) cast to an integer."]
        pub fn get_play_space(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_play_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the predicted display timing for the current frame."]
        pub fn get_predicted_display_time(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_predicted_display_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the predicted display timing for the next frame."]
        pub fn get_next_frame_time(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_next_frame_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if OpenXR is initialized for rendering with an XR viewport."]
        pub fn can_render(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "can_render", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] corresponding to an `Action` of a matching name, optionally limited to a specified action set."]
        pub fn find_action(&mut self, name: impl AsArg < GString >, action_set: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Rid,);
            let args = (name.into_arg(), action_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "find_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the corresponding `XrAction` OpenXR handle for the given action RID."]
        pub fn action_get_handle(&mut self, action: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "action_get_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the corresponding `XRHandTrackerEXT` handle for the given hand index value."]
        pub fn get_hand_tracker(&self, hand_index: i32,) -> u64 {
            type CallRet = u64;
            type CallParams = (i32,);
            let args = (hand_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_hand_tracker", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers the given extension as a composition layer provider.\n\n**Note:** This cannot be called after the OpenXR session has started. However, it can be called in [`on_session_created`][`crate::classes::IOpenXrExtensionWrapper::on_session_created`]."]
        pub fn register_composition_layer_provider(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "register_composition_layer_provider", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unregisters the given extension as a composition layer provider.\n\n**Note:** This cannot be called while the OpenXR session is still running."]
        pub fn unregister_composition_layer_provider(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "unregister_composition_layer_provider", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers the given extension as a provider of additional data structures to projections views.\n\n**Note:** This cannot be called after the OpenXR session has started. However, it can be called in [`on_session_created`][`crate::classes::IOpenXrExtensionWrapper::on_session_created`]."]
        pub fn register_projection_views_extension(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "register_projection_views_extension", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unregisters the given extension as a provider of additional data structures to projections views.\n\n**Note:** This cannot be called while the OpenXR session is still running."]
        pub fn unregister_projection_views_extension(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "unregister_projection_views_extension", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers the given extension as modifying frame info via the [`set_frame_wait_info_and_get_next_pointer_rawptr`][`crate::classes::IOpenXrExtensionWrapper::set_frame_wait_info_and_get_next_pointer_rawptr`], [`set_view_locate_info_and_get_next_pointer_rawptr`][`crate::classes::IOpenXrExtensionWrapper::set_view_locate_info_and_get_next_pointer_rawptr`], or [`set_frame_end_info_and_get_next_pointer_rawptr`][`crate::classes::IOpenXrExtensionWrapper::set_frame_end_info_and_get_next_pointer_rawptr`] virtual methods.\n\n**Note:** This cannot be called after the OpenXR session has started. However, it can be called in [`on_session_created`][`crate::classes::IOpenXrExtensionWrapper::on_session_created`]."]
        pub fn register_frame_info_extension(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "register_frame_info_extension", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unregisters the given extension as modifying frame info.\n\n**Note:** This cannot be called while the OpenXR session is still running."]
        pub fn unregister_frame_info_extension(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "unregister_frame_info_extension", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the near boundary value of the camera frustum.\n\n**Note:** This is only accessible in the render thread."]
        pub fn get_render_state_z_near(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_render_state_z_near", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the far boundary value of the camera frustum.\n\n**Note:** This is only accessible in the render thread."]
        pub fn get_render_state_z_far(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_render_state_z_far", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the render target of the velocity texture."]
        pub fn set_velocity_texture(&mut self, render_target: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (render_target,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_velocity_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the render target of the velocity depth texture."]
        pub fn set_velocity_depth_texture(&mut self, render_target: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (render_target,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_velocity_depth_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the target size of the velocity and velocity depth textures."]
        pub fn set_velocity_target_size(&mut self, target_size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (target_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_velocity_target_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of supported swapchain formats."]
        pub fn get_supported_swapchain_formats(&self,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_supported_swapchain_formats", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a pointer to a new swapchain created using the provided parameters."]
        pub fn openxr_swapchain_create(&mut self, create_flags: u64, usage_flags: u64, swapchain_format: i64, width: u32, height: u32, sample_count: u32, array_size: u32,) -> u64 {
            type CallRet = u64;
            type CallParams = (u64, u64, i64, u32, u32, u32, u32,);
            let args = (create_flags, usage_flags, swapchain_format, width, height, sample_count, array_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Destroys the provided swapchain and frees it from memory."]
        pub fn openxr_swapchain_free(&mut self, swapchain: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (swapchain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_free", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the `XrSwapchain` handle of the provided swapchain."]
        pub fn openxr_swapchain_get_swapchain(&mut self, swapchain: u64,) -> u64 {
            type CallRet = u64;
            type CallParams = (u64,);
            let args = (swapchain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_get_swapchain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Acquires the image of the provided swapchain."]
        pub fn openxr_swapchain_acquire(&mut self, swapchain: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (swapchain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_acquire", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the RID of the provided swapchain's image."]
        pub fn openxr_swapchain_get_image(&mut self, swapchain: u64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (u64,);
            let args = (swapchain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_get_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Releases the image of the provided swapchain."]
        pub fn openxr_swapchain_release(&mut self, swapchain: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (swapchain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_release", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a pointer to the render state's `XrCompositionLayerProjection` struct.\n\n**Note:** This method should only be called from the rendering thread."]
        pub fn get_projection_layer(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_projection_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the render region to `render_region`, overriding the normal render target's rect."]
        pub fn set_render_region(&mut self, render_region: Rect2i,) {
            type CallRet = ();
            type CallParams = (Rect2i,);
            let args = (render_region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_render_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, an OpenXR extension is loaded which is capable of emulating the [`EnvironmentBlendMode::ALPHA_BLEND`][`crate::classes::xr_interface::EnvironmentBlendMode::ALPHA_BLEND`] blend mode."]
        pub fn set_emulate_environment_blend_mode_alpha_blend(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_emulate_environment_blend_mode_alpha_blend", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns \\[enum OpenXRAPIExtension.OpenXRAlphaBlendModeSupport] denoting if [`EnvironmentBlendMode::ALPHA_BLEND`][`crate::classes::xr_interface::EnvironmentBlendMode::ALPHA_BLEND`] is really supported, emulated or not supported at all."]
        pub fn is_environment_blend_mode_alpha_supported(&self,) -> crate::classes::open_xr_api_extension::OpenXrAlphaBlendModeSupport {
            type CallRet = crate::classes::open_xr_api_extension::OpenXrAlphaBlendModeSupport;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "is_environment_blend_mode_alpha_supported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Request the recommended resolution from the OpenXR runtime and update the main swapchain size if it has changed."]
        pub fn update_main_swapchain_size(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "update_main_swapchain_size", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrApiExtension {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRAPIExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrApiExtension {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrApiExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrApiExtension {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrApiExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrApiExtension {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrApiExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrApiExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrApiExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrApiExtension > for $Class {
                
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
#[doc = "Godot enum name: `OpenXRAlphaBlendModeSupport`."]
pub struct OpenXrAlphaBlendModeSupport {
    ord: i32
}
impl OpenXrAlphaBlendModeSupport {
    #[doc(alias = "OPENXR_ALPHA_BLEND_MODE_SUPPORT_NONE")]
    #[doc = "Godot enumerator name: `OPENXR_ALPHA_BLEND_MODE_SUPPORT_NONE`"]
    pub const NONE: OpenXrAlphaBlendModeSupport = OpenXrAlphaBlendModeSupport {
        ord: 0i32
    };
    #[doc(alias = "OPENXR_ALPHA_BLEND_MODE_SUPPORT_REAL")]
    #[doc = "Godot enumerator name: `OPENXR_ALPHA_BLEND_MODE_SUPPORT_REAL`"]
    pub const REAL: OpenXrAlphaBlendModeSupport = OpenXrAlphaBlendModeSupport {
        ord: 1i32
    };
    #[doc(alias = "OPENXR_ALPHA_BLEND_MODE_SUPPORT_EMULATING")]
    #[doc = "Godot enumerator name: `OPENXR_ALPHA_BLEND_MODE_SUPPORT_EMULATING`"]
    pub const EMULATING: OpenXrAlphaBlendModeSupport = OpenXrAlphaBlendModeSupport {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for OpenXrAlphaBlendModeSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("OpenXrAlphaBlendModeSupport") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for OpenXrAlphaBlendModeSupport {
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
            Self::NONE => "NONE", Self::REAL => "REAL", Self::EMULATING => "EMULATING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[OpenXrAlphaBlendModeSupport::NONE, OpenXrAlphaBlendModeSupport::REAL, OpenXrAlphaBlendModeSupport::EMULATING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < OpenXrAlphaBlendModeSupport >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "OPENXR_ALPHA_BLEND_MODE_SUPPORT_NONE", OpenXrAlphaBlendModeSupport::NONE), crate::meta::inspect::EnumConstant::new("REAL", "OPENXR_ALPHA_BLEND_MODE_SUPPORT_REAL", OpenXrAlphaBlendModeSupport::REAL), crate::meta::inspect::EnumConstant::new("EMULATING", "OPENXR_ALPHA_BLEND_MODE_SUPPORT_EMULATING", OpenXrAlphaBlendModeSupport::EMULATING)]
        }
    }
}
impl crate::meta::GodotConvert for OpenXrAlphaBlendModeSupport {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Openxr Alpha Blend Mode Support None", 0i64), EnumeratorShape::new_int("Openxr Alpha Blend Mode Support Real", 1i64), EnumeratorShape::new_int("Openxr Alpha Blend Mode Support Emulating", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("OpenXRAPIExtension.OpenXRAlphaBlendModeSupport")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for OpenXrAlphaBlendModeSupport {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for OpenXrAlphaBlendModeSupport {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for OpenXrAlphaBlendModeSupport {
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
impl crate::registry::property::Export for OpenXrAlphaBlendModeSupport {
    
}
impl crate::meta::Element for OpenXrAlphaBlendModeSupport {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrApiExtension;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for OpenXrApiExtension {
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