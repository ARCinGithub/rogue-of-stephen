#![doc = "Sidecar module for class [`WebXrInterface`][crate::classes::WebXrInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebXRInterface` enums](https://docs.godotengine.org/en/stable/classes/class_webxrinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebXRInterface`.\n\nInherits [`XrInterface`][crate::classes::XrInterface].\n\nRelated symbols:\n\n* [`web_xr_interface`][crate::classes::web_xr_interface]: sidecar module with related enum/flag types\n* [`SignalsOfWebXrInterface`][crate::classes::web_xr_interface::SignalsOfWebXrInterface]: signal collection\n\n\nSee also [Godot docs for `WebXRInterface`](https://docs.godotengine.org/en/stable/classes/class_webxrinterface.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<WebXrInterface>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nWebXR is an open standard that allows creating VR and AR applications that run in the web browser.\n\nAs such, this interface is only available when running in Web exports.\n\nWebXR supports a wide range of devices, from the very capable (like Valve Index, HTC Vive, Oculus Rift and Quest) down to the much less capable (like Google Cardboard, Oculus Go, GearVR, or plain smartphones).\n\nSince WebXR is based on JavaScript, it makes extensive use of callbacks, which means that `WebXRInterface` is forced to use signals, where other XR interfaces would instead use functions that return a result immediately. This makes `WebXRInterface` quite a bit more complicated to initialize than other XR interfaces.\n\nHere's the minimum code required to start an immersive VR session:\n\n```gdscript\nextends Node3D\n\nvar webxr_interface\nvar vr_supported = false\n\nfunc _ready():\n\t# We assume this node has a button as a child.\n\t# This button is for the user to consent to entering immersive VR mode.\n\t$Button.pressed.connect(self._on_button_pressed)\n\n\twebxr_interface = XRServer.find_interface(\"WebXR\")\n\tif webxr_interface:\n\t\t# WebXR uses a lot of asynchronous callbacks, so we connect to various\n\t\t# signals in order to receive them.\n\t\twebxr_interface.session_supported.connect(self._webxr_session_supported)\n\t\twebxr_interface.session_started.connect(self._webxr_session_started)\n\t\twebxr_interface.session_ended.connect(self._webxr_session_ended)\n\t\twebxr_interface.session_failed.connect(self._webxr_session_failed)\n\n\t\t# This returns immediately - our _webxr_session_supported() method\n\t\t# (which we connected to the \"session_supported\" signal above) will\n\t\t# be called sometime later to let us know if it's supported or not.\n\t\twebxr_interface.is_session_supported(\"immersive-vr\")\n\nfunc _webxr_session_supported(session_mode, supported):\n\tif session_mode == 'immersive-vr':\n\t\tvr_supported = supported\n\nfunc _on_button_pressed():\n\tif not vr_supported:\n\t\tOS.alert(\"Your browser doesn't support VR\")\n\t\treturn\n\n\t# We want an immersive VR session, as opposed to AR ('immersive-ar') or a\n\t# simple 3DoF viewer ('viewer').\n\twebxr_interface.session_mode = 'immersive-vr'\n\t# 'bounded-floor' is room scale, 'local-floor' is a standing or sitting\n\t# experience (it puts you 1.6m above the ground if you have 3DoF headset),\n\t# whereas as 'local' puts you down at the XROrigin.\n\t# This list means it'll first try to request 'bounded-floor', then\n\t# fallback on 'local-floor' and ultimately 'local', if nothing else is\n\t# supported.\n\twebxr_interface.requested_reference_space_types = 'bounded-floor, local-floor, local'\n\t# In order to use 'local-floor' or 'bounded-floor' we must also\n\t# mark the features as required or optional. By including 'hand-tracking'\n\t# as an optional feature, it will be enabled if supported.\n\twebxr_interface.required_features = 'local-floor'\n\twebxr_interface.optional_features = 'bounded-floor, hand-tracking'\n\n\t# This will return false if we're unable to even request the session,\n\t# however, it can still fail asynchronously later in the process, so we\n\t# only know if it's really succeeded or failed when our\n\t# _webxr_session_started() or _webxr_session_failed() methods are called.\n\tif not webxr_interface.initialize():\n\t\tOS.alert(\"Failed to initialize\")\n\t\treturn\n\nfunc _webxr_session_started():\n\t$Button.visible = false\n\t# This tells Godot to start rendering to the headset.\n\tget_viewport().use_xr = true\n\t# This will be the reference space type you ultimately got, out of the\n\t# types that you requested above. This is useful if you want the game to\n\t# work a little differently in 'bounded-floor' versus 'local-floor'.\n\tprint(\"Reference space type: \", webxr_interface.reference_space_type)\n\t# This will be the list of features that were successfully enabled\n\t# (except on browsers that don't support this property).\n\tprint(\"Enabled features: \", webxr_interface.enabled_features)\n\nfunc _webxr_session_ended():\n\t$Button.visible = true\n\t# If the user exits immersive mode, then we tell Godot to render to the web\n\t# page again.\n\tget_viewport().use_xr = false\n\nfunc _webxr_session_failed(message):\n\tOS.alert(\"Failed to initialize: \" + message)\n```\n\nThere are a couple ways to handle \"controller\" input:\n\n- Using [`XRController3D`][crate::classes::XrController3D] nodes and their `XRController3D.button_pressed` and `XRController3D.button_released` signals. This is how controllers are typically handled in XR apps in Godot, however, this will only work with advanced VR controllers like the Oculus Touch or Index controllers, for example.\n\n- Using the `select`, `squeeze` and related signals. This method will work for both advanced VR controllers, and non-traditional input sources like a tap on the screen, a spoken voice command or a button press on the device itself.\n\nYou can use both methods to allow your game or app to support a wider or narrower set of devices and input methods, or to allow more advanced interactions with more advanced devices."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebXrInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl WebXrInterface {
        #[doc = "Checks if the given `session_mode` is supported by the user's browser.\n\nPossible values come from [WebXR's XRSessionMode](https://developer.mozilla.org/en-US/docs/Web/API/XRSessionMode), including: `\"immersive-vr\"`, `\"immersive-ar\"`, and `\"inline\"`.\n\nThis method returns nothing, instead it emits the `session_supported` signal with the result."]
        pub fn is_session_supported(&self, session_mode: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (session_mode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "is_session_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_session_mode(&mut self, session_mode: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (session_mode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_session_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_session_mode(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5622usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_session_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_required_features(&mut self, required_features: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (required_features.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_required_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_required_features(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_required_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_optional_features(&mut self, optional_features: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (optional_features.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_optional_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_optional_features(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_optional_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_reference_space_type(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_reference_space_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_enabled_features(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_enabled_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_requested_reference_space_types(&mut self, requested_reference_space_types: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (requested_reference_space_types.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_requested_reference_space_types", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_requested_reference_space_types(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_requested_reference_space_types", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there is an active input source with the given `input_source_id`."]
        pub fn is_input_source_active(&self, input_source_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "is_input_source_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets an [`XRControllerTracker`][crate::classes::XrControllerTracker] for the given `input_source_id`.\n\nIn the context of WebXR, an input source can be an advanced VR controller like the Oculus Touch or Index controllers, or even a tap on the screen, a spoken voice command or a button press on the device itself. When a non-traditional input source is used, interpret the position and orientation of the [`XRPositionalTracker`][crate::classes::XrPositionalTracker] as a ray pointing at the object the user wishes to interact with.\n\nUse this method to get information about the input source that triggered one of these signals:\n\n- `selectstart`\n\n- `select`\n\n- `selectend`\n\n- `squeezestart`\n\n- `squeeze`\n\n- `squeezestart`"]
        pub fn get_input_source_tracker(&self, input_source_id: i32,) -> Option < Gd < crate::classes::XrControllerTracker > > {
            type CallRet = Option < Gd < crate::classes::XrControllerTracker > >;
            type CallParams = (i32,);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_input_source_tracker", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the target ray mode for the given `input_source_id`.\n\nThis can help interpret the input coming from that input source. See [XRInputSource.targetRayMode](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/targetRayMode) for more information."]
        pub fn get_input_source_target_ray_mode(&self, input_source_id: i32,) -> crate::classes::web_xr_interface::TargetRayMode {
            type CallRet = crate::classes::web_xr_interface::TargetRayMode;
            type CallParams = (i32,);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_input_source_target_ray_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_state(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_visibility_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the display refresh rate for the current HMD. Not supported on all HMDs and browsers. It may not report an accurate value until after using [`set_display_refresh_rate`][`crate::classes::WebXrInterface::set_display_refresh_rate`]."]
        pub fn get_display_refresh_rate(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_display_refresh_rate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the display refresh rate for the current HMD. Not supported on all HMDs and browsers. It won't take effect right away until after `display_refresh_rate_changed` is emitted."]
        pub fn set_display_refresh_rate(&mut self, refresh_rate: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (refresh_rate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_display_refresh_rate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns display refresh rates supported by the current HMD. Only returned if this feature is supported by the web browser and after the interface has been initialized."]
        pub fn get_available_display_refresh_rates(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_available_display_refresh_rates", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for WebXrInterface {
        type Base = crate::classes::XrInterface;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("WebXRInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebXrInterface {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrInterface > for WebXrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for WebXrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WebXrInterface {
        
    }
    impl std::ops::Deref for WebXrInterface {
        type Target = crate::classes::XrInterface;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebXrInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_WebXrInterface__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `WebXrInterface` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TargetRayMode {
    ord: i32
}
impl TargetRayMode {
    #[doc(alias = "TARGET_RAY_MODE_UNKNOWN")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_UNKNOWN`"]
    pub const UNKNOWN: TargetRayMode = TargetRayMode {
        ord: 0i32
    };
    #[doc(alias = "TARGET_RAY_MODE_GAZE")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_GAZE`"]
    pub const GAZE: TargetRayMode = TargetRayMode {
        ord: 1i32
    };
    #[doc(alias = "TARGET_RAY_MODE_TRACKED_POINTER")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_TRACKED_POINTER`"]
    pub const TRACKED_POINTER: TargetRayMode = TargetRayMode {
        ord: 2i32
    };
    #[doc(alias = "TARGET_RAY_MODE_SCREEN")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_SCREEN`"]
    pub const SCREEN: TargetRayMode = TargetRayMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TargetRayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TargetRayMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TargetRayMode {
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
            Self::UNKNOWN => "UNKNOWN", Self::GAZE => "GAZE", Self::TRACKED_POINTER => "TRACKED_POINTER", Self::SCREEN => "SCREEN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TargetRayMode::UNKNOWN, TargetRayMode::GAZE, TargetRayMode::TRACKED_POINTER, TargetRayMode::SCREEN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TargetRayMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNKNOWN", "TARGET_RAY_MODE_UNKNOWN", TargetRayMode::UNKNOWN), crate::meta::inspect::EnumConstant::new("GAZE", "TARGET_RAY_MODE_GAZE", TargetRayMode::GAZE), crate::meta::inspect::EnumConstant::new("TRACKED_POINTER", "TARGET_RAY_MODE_TRACKED_POINTER", TargetRayMode::TRACKED_POINTER), crate::meta::inspect::EnumConstant::new("SCREEN", "TARGET_RAY_MODE_SCREEN", TargetRayMode::SCREEN)]
        }
    }
}
impl crate::meta::GodotConvert for TargetRayMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Target Ray Mode Unknown", 0i64), EnumeratorShape::new_int("Target Ray Mode Gaze", 1i64), EnumeratorShape::new_int("Target Ray Mode Tracked Pointer", 2i64), EnumeratorShape::new_int("Target Ray Mode Screen", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("WebXRInterface.TargetRayMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TargetRayMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TargetRayMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TargetRayMode {
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
impl crate::registry::property::Export for TargetRayMode {
    
}
impl crate::meta::Element for TargetRayMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::WebXrInterface;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`WebXrInterface`][crate::classes::WebXrInterface] class."]
    pub struct SignalsOfWebXrInterface < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfWebXrInterface < 'c, C > {
        #[doc = "Signature: `(session_mode: GString, supported: bool)`"]
        pub fn session_supported(&mut self) -> SigSessionSupported < 'c, C > {
            SigSessionSupported {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "session_supported")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn session_started(&mut self) -> SigSessionStarted < 'c, C > {
            SigSessionStarted {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "session_started")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn session_ended(&mut self) -> SigSessionEnded < 'c, C > {
            SigSessionEnded {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "session_ended")
            }
        }
        #[doc = "Signature: `(message: GString)`"]
        pub fn session_failed(&mut self) -> SigSessionFailed < 'c, C > {
            SigSessionFailed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "session_failed")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn selectstart(&mut self) -> SigSelectstart < 'c, C > {
            SigSelectstart {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "selectstart")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn select(&mut self) -> SigSelect < 'c, C > {
            SigSelect {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "select")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn selectend(&mut self) -> SigSelectend < 'c, C > {
            SigSelectend {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "selectend")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn squeezestart(&mut self) -> SigSqueezestart < 'c, C > {
            SigSqueezestart {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "squeezestart")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn squeeze(&mut self) -> SigSqueeze < 'c, C > {
            SigSqueeze {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "squeeze")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn squeezeend(&mut self) -> SigSqueezeend < 'c, C > {
            SigSqueezeend {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "squeezeend")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn visibility_state_changed(&mut self) -> SigVisibilityStateChanged < 'c, C > {
            SigVisibilityStateChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "visibility_state_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn reference_space_reset(&mut self) -> SigReferenceSpaceReset < 'c, C > {
            SigReferenceSpaceReset {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "reference_space_reset")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn display_refresh_rate_changed(&mut self) -> SigDisplayRefreshRateChanged < 'c, C > {
            SigDisplayRefreshRateChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "display_refresh_rate_changed")
            }
        }
    }
    type TypedSigSessionSupported < 'c, C > = TypedSignal < 'c, C, (GString, bool,) >;
    pub struct SigSessionSupported < 'c, C: WithSignals > {
        typed: TypedSigSessionSupported < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionSupported < 'c, C > {
        pub fn emit(&mut self, session_mode: GString, supported: bool,) {
            self.typed.emit_tuple((session_mode, supported,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionSupported < 'c, C > {
        type Target = TypedSigSessionSupported < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionSupported < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSessionStarted < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSessionStarted < 'c, C: WithSignals > {
        typed: TypedSigSessionStarted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionStarted < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionStarted < 'c, C > {
        type Target = TypedSigSessionStarted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionStarted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSessionEnded < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSessionEnded < 'c, C: WithSignals > {
        typed: TypedSigSessionEnded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionEnded < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionEnded < 'c, C > {
        type Target = TypedSigSessionEnded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionEnded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSessionFailed < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigSessionFailed < 'c, C: WithSignals > {
        typed: TypedSigSessionFailed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionFailed < 'c, C > {
        pub fn emit(&mut self, message: GString,) {
            self.typed.emit_tuple((message,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionFailed < 'c, C > {
        type Target = TypedSigSessionFailed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionFailed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSelectstart < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSelectstart < 'c, C: WithSignals > {
        typed: TypedSigSelectstart < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSelectstart < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSelectstart < 'c, C > {
        type Target = TypedSigSelectstart < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSelectstart < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSelect < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSelect < 'c, C: WithSignals > {
        typed: TypedSigSelect < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSelect < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSelect < 'c, C > {
        type Target = TypedSigSelect < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSelect < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSelectend < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSelectend < 'c, C: WithSignals > {
        typed: TypedSigSelectend < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSelectend < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSelectend < 'c, C > {
        type Target = TypedSigSelectend < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSelectend < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSqueezestart < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSqueezestart < 'c, C: WithSignals > {
        typed: TypedSigSqueezestart < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSqueezestart < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSqueezestart < 'c, C > {
        type Target = TypedSigSqueezestart < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSqueezestart < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSqueeze < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSqueeze < 'c, C: WithSignals > {
        typed: TypedSigSqueeze < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSqueeze < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSqueeze < 'c, C > {
        type Target = TypedSigSqueeze < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSqueeze < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSqueezeend < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSqueezeend < 'c, C: WithSignals > {
        typed: TypedSigSqueezeend < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSqueezeend < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSqueezeend < 'c, C > {
        type Target = TypedSigSqueezeend < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSqueezeend < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigVisibilityStateChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigVisibilityStateChanged < 'c, C: WithSignals > {
        typed: TypedSigVisibilityStateChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigVisibilityStateChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigVisibilityStateChanged < 'c, C > {
        type Target = TypedSigVisibilityStateChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigVisibilityStateChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigReferenceSpaceReset < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigReferenceSpaceReset < 'c, C: WithSignals > {
        typed: TypedSigReferenceSpaceReset < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigReferenceSpaceReset < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigReferenceSpaceReset < 'c, C > {
        type Target = TypedSigReferenceSpaceReset < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigReferenceSpaceReset < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDisplayRefreshRateChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigDisplayRefreshRateChanged < 'c, C: WithSignals > {
        typed: TypedSigDisplayRefreshRateChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDisplayRefreshRateChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDisplayRefreshRateChanged < 'c, C > {
        type Target = TypedSigDisplayRefreshRateChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDisplayRefreshRateChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for WebXrInterface {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfWebXrInterface < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfWebXrInterface < 'c, C > {
        type Target = < < WebXrInterface as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = WebXrInterface;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfWebXrInterface < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = WebXrInterface;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}