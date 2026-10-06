#![doc = "Sidecar module for class [`AnimationMixer`][crate::classes::AnimationMixer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationMixer` enums](https://docs.godotengine.org/en/stable/classes/class_animationmixer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationMixer`.\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`animation_mixer`][crate::classes::animation_mixer]: sidecar module with related enum/flag types\n* [`SignalsOfAnimationMixer`][crate::classes::animation_mixer::SignalsOfAnimationMixer]: signal collection\n\n\nSee also [Godot docs for `AnimationMixer`](https://docs.godotengine.org/en/stable/classes/class_animationmixer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<AnimationMixer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nBase class for [`AnimationPlayer`][crate::classes::AnimationPlayer] and [`AnimationTree`][crate::classes::AnimationTree] to manage animation lists. It also has general properties and methods for playback and blending.\n\nAfter instantiating the playback information data within the extended class, the blending is processed by the `AnimationMixer`."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationMixer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl AnimationMixer {
        #[doc = "Adds `library` to the animation player, under the key `name`.\n\nAnimationMixer has a global library by default with an empty string as key. For adding an animation to the global library:\n\n\n```gdscript\nvar global_library = mixer.get_animation_library(\"\")\nglobal_library.add_animation(\"animation_name\", animation_resource)\n```\n"]
        pub fn add_animation_library(&mut self, name: impl AsArg < StringName >, library: impl AsArg < Option < Gd < crate::classes::AnimationLibrary >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::AnimationLibrary > > >,);
            let args = (name.into_arg(), library.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "add_animation_library", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the [`AnimationLibrary`][crate::classes::AnimationLibrary] associated with the key `name`."]
        pub fn remove_animation_library(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "remove_animation_library", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves the [`AnimationLibrary`][crate::classes::AnimationLibrary] associated with the key `name` to the key `newname`."]
        pub fn rename_animation_library(&mut self, name: impl AsArg < StringName >, newname: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), newname.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "rename_animation_library", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the `AnimationMixer` stores an [`AnimationLibrary`][crate::classes::AnimationLibrary] with key `name`."]
        pub fn has_animation_library(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "has_animation_library", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the first [`AnimationLibrary`][crate::classes::AnimationLibrary] with key `name` or `null` if not found.\n\nTo get the `AnimationMixer`'s global animation library, use `get_animation_library(\"\")`."]
        pub fn get_animation_library(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::AnimationLibrary > > {
            type CallRet = Option < Gd < crate::classes::AnimationLibrary > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(12usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation_library", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of stored library keys."]
        pub fn get_animation_library_list(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(13usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation_library_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the `AnimationMixer` stores an [`Animation`][crate::classes::Animation] with key `name`."]
        pub fn has_animation(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(14usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "has_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Animation`][crate::classes::Animation] with the key `name`. If the animation does not exist, `null` is returned and an error is logged."]
        pub fn get_animation(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Animation > > {
            type CallRet = Option < Gd < crate::classes::Animation > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(15usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of stored animation keys."]
        pub fn get_animation_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(16usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_active(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(17usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_active(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(18usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "is_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deterministic(&mut self, deterministic: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (deterministic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(19usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_deterministic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_deterministic(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(20usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "is_deterministic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_root_node(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(21usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_root_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_node(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(22usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_callback_mode_process(&mut self, mode: crate::classes::animation_mixer::AnimationCallbackModeProcess,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_mixer::AnimationCallbackModeProcess,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(23usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_callback_mode_process", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_callback_mode_process(&self,) -> crate::classes::animation_mixer::AnimationCallbackModeProcess {
            type CallRet = crate::classes::animation_mixer::AnimationCallbackModeProcess;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(24usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_callback_mode_process", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_callback_mode_method(&mut self, mode: crate::classes::animation_mixer::AnimationCallbackModeMethod,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_mixer::AnimationCallbackModeMethod,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(25usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_callback_mode_method", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_callback_mode_method(&self,) -> crate::classes::animation_mixer::AnimationCallbackModeMethod {
            type CallRet = crate::classes::animation_mixer::AnimationCallbackModeMethod;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(26usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_callback_mode_method", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_callback_mode_discrete(&mut self, mode: crate::classes::animation_mixer::AnimationCallbackModeDiscrete,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_mixer::AnimationCallbackModeDiscrete,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(27usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_callback_mode_discrete", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_callback_mode_discrete(&self,) -> crate::classes::animation_mixer::AnimationCallbackModeDiscrete {
            type CallRet = crate::classes::animation_mixer::AnimationCallbackModeDiscrete;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(28usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_callback_mode_discrete", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_audio_max_polyphony(&mut self, max_polyphony: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_polyphony,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(29usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_audio_max_polyphony", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_audio_max_polyphony(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(30usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_audio_max_polyphony", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_root_motion_track(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(31usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_root_motion_track", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_motion_track(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(32usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_track", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_root_motion_local(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(33usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_root_motion_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_root_motion_local(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(34usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "is_root_motion_local", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Retrieve the motion delta of position with the \\[member root_motion_track] as a [`Vector3`][crate::builtin::Vector3] that can be used elsewhere.\n\nIf \\[member root_motion_track] is not a path to a track of type [`TrackType::POSITION_3D`][`crate::classes::animation::TrackType::POSITION_3D`], returns `Vector3(0, 0, 0)`.\n\nSee also \\[member root_motion_track] and [`RootMotionView`][crate::classes::RootMotionView].\n\nThe most basic example is applying position to [`CharacterBody3D`][crate::classes::CharacterBody3D]:\n\n\n```gdscript\nvar current_rotation\n\nfunc _process(delta):\n\tif Input.is_action_just_pressed(\"animate\"):\n\t\tcurrent_rotation = get_quaternion()\n\t\tstate_machine.travel(\"Animate\")\n\tvar velocity = current_rotation * animation_tree.get_root_motion_position() / delta\n\tset_velocity(velocity)\n\tmove_and_slide()\n```\n\n\nBy using this in combination with [`get_root_motion_rotation_accumulator`][`crate::classes::AnimationMixer::get_root_motion_rotation_accumulator`], you can apply the root motion position more correctly to account for the rotation of the node.\n\n\n```gdscript\nfunc _process(delta):\n\tif Input.is_action_just_pressed(\"animate\"):\n\t\tstate_machine.travel(\"Animate\")\n\tset_quaternion(get_quaternion() * animation_tree.get_root_motion_rotation())\n\tvar velocity = (animation_tree.get_root_motion_rotation_accumulator().inverse() * get_quaternion()) * animation_tree.get_root_motion_position() / delta\n\tset_velocity(velocity)\n\tmove_and_slide()\n```\n\n\nIf \\[member root_motion_local] is `true`, returns the pre-multiplied translation value with the inverted rotation.\n\nIn this case, the code can be written as follows:\n\n\n```gdscript\nfunc _process(delta):\n\tif Input.is_action_just_pressed(\"animate\"):\n\t\tstate_machine.travel(\"Animate\")\n\tset_quaternion(get_quaternion() * animation_tree.get_root_motion_rotation())\n\tvar velocity = get_quaternion() * animation_tree.get_root_motion_position() / delta\n\tset_velocity(velocity)\n\tmove_and_slide()\n```\n"]
        pub fn get_root_motion_position(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(35usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Retrieve the motion delta of rotation with the \\[member root_motion_track] as a [`Quaternion`][crate::builtin::Quaternion] that can be used elsewhere.\n\nIf \\[member root_motion_track] is not a path to a track of type [`TrackType::ROTATION_3D`][`crate::classes::animation::TrackType::ROTATION_3D`], returns `Quaternion(0, 0, 0, 1)`.\n\nSee also \\[member root_motion_track] and [`RootMotionView`][crate::classes::RootMotionView].\n\nThe most basic example is applying rotation to [`CharacterBody3D`][crate::classes::CharacterBody3D]:\n\n\n```gdscript\nfunc _process(delta):\n\tif Input.is_action_just_pressed(\"animate\"):\n\t\tstate_machine.travel(\"Animate\")\n\tset_quaternion(get_quaternion() * animation_tree.get_root_motion_rotation())\n```\n"]
        pub fn get_root_motion_rotation(&self,) -> Quaternion {
            type CallRet = Quaternion;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(36usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_rotation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Retrieve the motion delta of scale with the \\[member root_motion_track] as a [`Vector3`][crate::builtin::Vector3] that can be used elsewhere.\n\nIf \\[member root_motion_track] is not a path to a track of type [`TrackType::SCALE_3D`][`crate::classes::animation::TrackType::SCALE_3D`], returns `Vector3(0, 0, 0)`.\n\nSee also \\[member root_motion_track] and [`RootMotionView`][crate::classes::RootMotionView].\n\nThe most basic example is applying scale to [`CharacterBody3D`][crate::classes::CharacterBody3D]:\n\n\n```gdscript\nvar current_scale = Vector3(1, 1, 1)\nvar scale_accum = Vector3(1, 1, 1)\n\nfunc _process(delta):\n\tif Input.is_action_just_pressed(\"animate\"):\n\t\tcurrent_scale = get_scale()\n\t\tscale_accum = Vector3(1, 1, 1)\n\t\tstate_machine.travel(\"Animate\")\n\tscale_accum += animation_tree.get_root_motion_scale()\n\tset_scale(current_scale * scale_accum)\n```\n"]
        pub fn get_root_motion_scale(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(37usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Retrieve the blended value of the position tracks with the \\[member root_motion_track] as a [`Vector3`][crate::builtin::Vector3] that can be used elsewhere.\n\nThis is useful in cases where you want to respect the initial key values of the animation.\n\nFor example, if an animation with only one key `Vector3(0, 0, 0)` is played in the previous frame and then an animation with only one key `Vector3(1, 0, 1)` is played in the next frame, the difference can be calculated as follows:\n\n\n```gdscript\nvar prev_root_motion_position_accumulator\n\nfunc _process(delta):\n\tif Input.is_action_just_pressed(\"animate\"):\n\t\tstate_machine.travel(\"Animate\")\n\tvar current_root_motion_position_accumulator = animation_tree.get_root_motion_position_accumulator()\n\tvar difference = current_root_motion_position_accumulator - prev_root_motion_position_accumulator\n\tprev_root_motion_position_accumulator = current_root_motion_position_accumulator\n\ttransform.origin += difference\n```\n\n\nHowever, if the animation loops, an unintended discrete change may occur, so this is only useful for some simple use cases."]
        pub fn get_root_motion_position_accumulator(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(38usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_position_accumulator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Retrieve the blended value of the rotation tracks with the \\[member root_motion_track] as a [`Quaternion`][crate::builtin::Quaternion] that can be used elsewhere.\n\nThis is necessary to apply the root motion position correctly, taking rotation into account. See also [`get_root_motion_position`][`crate::classes::AnimationMixer::get_root_motion_position`].\n\nAlso, this is useful in cases where you want to respect the initial key values of the animation.\n\nFor example, if an animation with only one key `Quaternion(0, 0, 0, 1)` is played in the previous frame and then an animation with only one key `Quaternion(0, 0.707, 0, 0.707)` is played in the next frame, the difference can be calculated as follows:\n\n\n```gdscript\nvar prev_root_motion_rotation_accumulator\n\nfunc _process(delta):\n\tif Input.is_action_just_pressed(\"animate\"):\n\t\tstate_machine.travel(\"Animate\")\n\tvar current_root_motion_rotation_accumulator = animation_tree.get_root_motion_rotation_accumulator()\n\tvar difference = prev_root_motion_rotation_accumulator.inverse() * current_root_motion_rotation_accumulator\n\tprev_root_motion_rotation_accumulator = current_root_motion_rotation_accumulator\n\ttransform.basis *=  Basis(difference)\n```\n\n\nHowever, if the animation loops, an unintended discrete change may occur, so this is only useful for some simple use cases."]
        pub fn get_root_motion_rotation_accumulator(&self,) -> Quaternion {
            type CallRet = Quaternion;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(39usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_rotation_accumulator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Retrieve the blended value of the scale tracks with the \\[member root_motion_track] as a [`Vector3`][crate::builtin::Vector3] that can be used elsewhere.\n\nFor example, if an animation with only one key `Vector3(1, 1, 1)` is played in the previous frame and then an animation with only one key `Vector3(2, 2, 2)` is played in the next frame, the difference can be calculated as follows:\n\n\n```gdscript\nvar prev_root_motion_scale_accumulator\n\nfunc _process(delta):\n\tif Input.is_action_just_pressed(\"animate\"):\n\t\tstate_machine.travel(\"Animate\")\n\tvar current_root_motion_scale_accumulator = animation_tree.get_root_motion_scale_accumulator()\n\tvar difference = current_root_motion_scale_accumulator - prev_root_motion_scale_accumulator\n\tprev_root_motion_scale_accumulator = current_root_motion_scale_accumulator\n\ttransform.basis = transform.basis.scaled(difference)\n```\n\n\nHowever, if the animation loops, an unintended discrete change may occur, so this is only useful for some simple use cases."]
        pub fn get_root_motion_scale_accumulator(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(40usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_scale_accumulator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "`AnimationMixer` caches animated nodes. It may not notice if a node disappears; [`clear_caches`][`crate::classes::AnimationMixer::clear_caches`] forces it to update the cache again."]
        pub fn clear_caches(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(41usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "clear_caches", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Manually advance the animations by the specified time (in seconds)."]
        pub fn advance(&mut self, delta: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (delta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(42usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "advance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If the animation track specified by `name` has an option [`UpdateMode::CAPTURE`][`crate::classes::animation::UpdateMode::CAPTURE`], stores current values of the objects indicated by the track path as a cache. If there is already a captured cache, the old cache is discarded.\n\nAfter this it will interpolate with current animation blending result during the playback process for the time specified by `duration`, working like a crossfade.\n\nYou can specify `trans_type` as the curve for the interpolation. For better results, it may be appropriate to specify [`TransitionType::LINEAR`][`crate::classes::tween::TransitionType::LINEAR`] for cases where the first key of the track begins with a non-zero value or where the key value does not change, and [`TransitionType::QUAD`][`crate::classes::tween::TransitionType::QUAD`] for cases where the key value changes linearly."]
        pub(crate) fn capture_full(&mut self, name: CowArg < StringName >, duration: f64, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f64, crate::classes::tween::TransitionType, crate::classes::tween::EaseType,);
            let args = (name, duration, trans_type, ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(43usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "capture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`capture_ex`][Self::capture_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "If the animation track specified by `name` has an option [`UpdateMode::CAPTURE`][`crate::classes::animation::UpdateMode::CAPTURE`], stores current values of the objects indicated by the track path as a cache. If there is already a captured cache, the old cache is discarded.\n\nAfter this it will interpolate with current animation blending result during the playback process for the time specified by `duration`, working like a crossfade.\n\nYou can specify `trans_type` as the curve for the interpolation. For better results, it may be appropriate to specify [`TransitionType::LINEAR`][`crate::classes::tween::TransitionType::LINEAR`] for cases where the first key of the track begins with a non-zero value or where the key value does not change, and [`TransitionType::QUAD`][`crate::classes::tween::TransitionType::QUAD`] for cases where the key value changes linearly."]
        #[inline]
        pub fn capture(&mut self, name: impl AsArg < StringName >, duration: f64,) {
            self.capture_ex(name, duration,) . done()
        }
        #[doc = "If the animation track specified by `name` has an option [`UpdateMode::CAPTURE`][`crate::classes::animation::UpdateMode::CAPTURE`], stores current values of the objects indicated by the track path as a cache. If there is already a captured cache, the old cache is discarded.\n\nAfter this it will interpolate with current animation blending result during the playback process for the time specified by `duration`, working like a crossfade.\n\nYou can specify `trans_type` as the curve for the interpolation. For better results, it may be appropriate to specify [`TransitionType::LINEAR`][`crate::classes::tween::TransitionType::LINEAR`] for cases where the first key of the track begins with a non-zero value or where the key value does not change, and [`TransitionType::QUAD`][`crate::classes::tween::TransitionType::QUAD`] for cases where the key value changes linearly."]
        #[inline]
        pub fn capture_ex < 'ex > (&'ex mut self, name: impl AsArg < StringName > + 'ex, duration: f64,) -> ExCapture < 'ex > {
            ExCapture::new(self, name, duration,)
        }
        pub fn set_reset_on_save_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(44usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_reset_on_save_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_reset_on_save_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(45usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "is_reset_on_save_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the key of `animation` or an empty [`StringName`][crate::builtin::StringName] if not found."]
        pub fn find_animation(&self, animation: impl AsArg < Option < Gd < crate::classes::Animation >> >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Animation > > >,);
            let args = (animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(46usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "find_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the key for the [`AnimationLibrary`][crate::classes::AnimationLibrary] that contains `animation` or an empty [`StringName`][crate::builtin::StringName] if not found."]
        pub fn find_animation_library(&self, animation: impl AsArg < Option < Gd < crate::classes::Animation >> >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Animation > > >,);
            let args = (animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(47usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "find_animation_library", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationMixer {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationMixer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationMixer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AnimationMixer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationMixer {
        
    }
    impl std::ops::Deref for AnimationMixer {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationMixer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationMixer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `AnimationMixer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`AnimationMixer::capture_ex`][super::AnimationMixer::capture_ex]."]
#[must_use]
pub struct ExCapture < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationMixer, name: CowArg < 'ex, StringName >, duration: f64, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCapture < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationMixer, name: impl AsArg < StringName > + 'ex, duration: f64,) -> Self {
        let trans_type = crate::obj::EngineEnum::from_ord(0);
        let ease_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), duration: duration, trans_type: trans_type, ease_type: ease_type,
        }
    }
    #[inline]
    pub fn trans_type(self, trans_type: crate::classes::tween::TransitionType) -> Self {
        Self {
            trans_type: trans_type, .. self
        }
    }
    #[inline]
    pub fn ease_type(self, ease_type: crate::classes::tween::EaseType) -> Self {
        Self {
            ease_type: ease_type, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, duration, trans_type, ease_type,
        }
        = self;
        re_export::AnimationMixer::capture_full(surround_object, name, duration, trans_type, ease_type,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationCallbackModeProcess {
    ord: i32
}
impl AnimationCallbackModeProcess {
    #[doc(alias = "ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS`"]
    pub const PHYSICS: AnimationCallbackModeProcess = AnimationCallbackModeProcess {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_PROCESS_IDLE`"]
    pub const IDLE: AnimationCallbackModeProcess = AnimationCallbackModeProcess {
        ord: 1i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_PROCESS_MANUAL")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_PROCESS_MANUAL`"]
    pub const MANUAL: AnimationCallbackModeProcess = AnimationCallbackModeProcess {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AnimationCallbackModeProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationCallbackModeProcess") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationCallbackModeProcess {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", Self::MANUAL => "MANUAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnimationCallbackModeProcess::PHYSICS, AnimationCallbackModeProcess::IDLE, AnimationCallbackModeProcess::MANUAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnimationCallbackModeProcess >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PHYSICS", "ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS", AnimationCallbackModeProcess::PHYSICS), crate::meta::inspect::EnumConstant::new("IDLE", "ANIMATION_CALLBACK_MODE_PROCESS_IDLE", AnimationCallbackModeProcess::IDLE), crate::meta::inspect::EnumConstant::new("MANUAL", "ANIMATION_CALLBACK_MODE_PROCESS_MANUAL", AnimationCallbackModeProcess::MANUAL)]
        }
    }
}
impl crate::meta::GodotConvert for AnimationCallbackModeProcess {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Animation Callback Mode Process Physics", 0i64), EnumeratorShape::new_int("Animation Callback Mode Process Idle", 1i64), EnumeratorShape::new_int("Animation Callback Mode Process Manual", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AnimationMixer.AnimationCallbackModeProcess")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AnimationCallbackModeProcess {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationCallbackModeProcess {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AnimationCallbackModeProcess {
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
impl crate::registry::property::Export for AnimationCallbackModeProcess {
    
}
impl crate::meta::Element for AnimationCallbackModeProcess {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationCallbackModeMethod {
    ord: i32
}
impl AnimationCallbackModeMethod {
    #[doc(alias = "ANIMATION_CALLBACK_MODE_METHOD_DEFERRED")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_METHOD_DEFERRED`"]
    pub const DEFERRED: AnimationCallbackModeMethod = AnimationCallbackModeMethod {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_METHOD_IMMEDIATE")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_METHOD_IMMEDIATE`"]
    pub const IMMEDIATE: AnimationCallbackModeMethod = AnimationCallbackModeMethod {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AnimationCallbackModeMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationCallbackModeMethod") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationCallbackModeMethod {
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
            Self::DEFERRED => "DEFERRED", Self::IMMEDIATE => "IMMEDIATE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnimationCallbackModeMethod::DEFERRED, AnimationCallbackModeMethod::IMMEDIATE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnimationCallbackModeMethod >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFERRED", "ANIMATION_CALLBACK_MODE_METHOD_DEFERRED", AnimationCallbackModeMethod::DEFERRED), crate::meta::inspect::EnumConstant::new("IMMEDIATE", "ANIMATION_CALLBACK_MODE_METHOD_IMMEDIATE", AnimationCallbackModeMethod::IMMEDIATE)]
        }
    }
}
impl crate::meta::GodotConvert for AnimationCallbackModeMethod {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Animation Callback Mode Method Deferred", 0i64), EnumeratorShape::new_int("Animation Callback Mode Method Immediate", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AnimationMixer.AnimationCallbackModeMethod")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AnimationCallbackModeMethod {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationCallbackModeMethod {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AnimationCallbackModeMethod {
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
impl crate::registry::property::Export for AnimationCallbackModeMethod {
    
}
impl crate::meta::Element for AnimationCallbackModeMethod {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationCallbackModeDiscrete {
    ord: i32
}
impl AnimationCallbackModeDiscrete {
    #[doc(alias = "ANIMATION_CALLBACK_MODE_DISCRETE_DOMINANT")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_DISCRETE_DOMINANT`"]
    pub const DOMINANT: AnimationCallbackModeDiscrete = AnimationCallbackModeDiscrete {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_DISCRETE_RECESSIVE")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_DISCRETE_RECESSIVE`"]
    pub const RECESSIVE: AnimationCallbackModeDiscrete = AnimationCallbackModeDiscrete {
        ord: 1i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_DISCRETE_FORCE_CONTINUOUS")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_DISCRETE_FORCE_CONTINUOUS`"]
    pub const FORCE_CONTINUOUS: AnimationCallbackModeDiscrete = AnimationCallbackModeDiscrete {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AnimationCallbackModeDiscrete {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationCallbackModeDiscrete") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationCallbackModeDiscrete {
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
            Self::DOMINANT => "DOMINANT", Self::RECESSIVE => "RECESSIVE", Self::FORCE_CONTINUOUS => "FORCE_CONTINUOUS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnimationCallbackModeDiscrete::DOMINANT, AnimationCallbackModeDiscrete::RECESSIVE, AnimationCallbackModeDiscrete::FORCE_CONTINUOUS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnimationCallbackModeDiscrete >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DOMINANT", "ANIMATION_CALLBACK_MODE_DISCRETE_DOMINANT", AnimationCallbackModeDiscrete::DOMINANT), crate::meta::inspect::EnumConstant::new("RECESSIVE", "ANIMATION_CALLBACK_MODE_DISCRETE_RECESSIVE", AnimationCallbackModeDiscrete::RECESSIVE), crate::meta::inspect::EnumConstant::new("FORCE_CONTINUOUS", "ANIMATION_CALLBACK_MODE_DISCRETE_FORCE_CONTINUOUS", AnimationCallbackModeDiscrete::FORCE_CONTINUOUS)]
        }
    }
}
impl crate::meta::GodotConvert for AnimationCallbackModeDiscrete {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Animation Callback Mode Discrete Dominant", 0i64), EnumeratorShape::new_int("Animation Callback Mode Discrete Recessive", 1i64), EnumeratorShape::new_int("Animation Callback Mode Discrete Force Continuous", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AnimationMixer.AnimationCallbackModeDiscrete")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AnimationCallbackModeDiscrete {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationCallbackModeDiscrete {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AnimationCallbackModeDiscrete {
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
impl crate::registry::property::Export for AnimationCallbackModeDiscrete {
    
}
impl crate::meta::Element for AnimationCallbackModeDiscrete {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationMixer;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AnimationMixer`][crate::classes::AnimationMixer] class."]
    pub struct SignalsOfAnimationMixer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAnimationMixer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn animation_list_changed(&mut self) -> SigAnimationListChanged < 'c, C > {
            SigAnimationListChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "animation_list_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn animation_libraries_updated(&mut self) -> SigAnimationLibrariesUpdated < 'c, C > {
            SigAnimationLibrariesUpdated {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "animation_libraries_updated")
            }
        }
        #[doc = "Signature: `(anim_name: StringName)`"]
        pub fn animation_finished(&mut self) -> SigAnimationFinished < 'c, C > {
            SigAnimationFinished {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "animation_finished")
            }
        }
        #[doc = "Signature: `(anim_name: StringName)`"]
        pub fn animation_started(&mut self) -> SigAnimationStarted < 'c, C > {
            SigAnimationStarted {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "animation_started")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn caches_cleared(&mut self) -> SigCachesCleared < 'c, C > {
            SigCachesCleared {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "caches_cleared")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn mixer_applied(&mut self) -> SigMixerApplied < 'c, C > {
            SigMixerApplied {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "mixer_applied")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn mixer_updated(&mut self) -> SigMixerUpdated < 'c, C > {
            SigMixerUpdated {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "mixer_updated")
            }
        }
    }
    type TypedSigAnimationListChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigAnimationListChanged < 'c, C: WithSignals > {
        typed: TypedSigAnimationListChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationListChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationListChanged < 'c, C > {
        type Target = TypedSigAnimationListChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationListChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationLibrariesUpdated < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigAnimationLibrariesUpdated < 'c, C: WithSignals > {
        typed: TypedSigAnimationLibrariesUpdated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationLibrariesUpdated < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationLibrariesUpdated < 'c, C > {
        type Target = TypedSigAnimationLibrariesUpdated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationLibrariesUpdated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationFinished < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigAnimationFinished < 'c, C: WithSignals > {
        typed: TypedSigAnimationFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationFinished < 'c, C > {
        pub fn emit(&mut self, anim_name: StringName,) {
            self.typed.emit_tuple((anim_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationFinished < 'c, C > {
        type Target = TypedSigAnimationFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationStarted < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigAnimationStarted < 'c, C: WithSignals > {
        typed: TypedSigAnimationStarted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationStarted < 'c, C > {
        pub fn emit(&mut self, anim_name: StringName,) {
            self.typed.emit_tuple((anim_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationStarted < 'c, C > {
        type Target = TypedSigAnimationStarted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationStarted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCachesCleared < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigCachesCleared < 'c, C: WithSignals > {
        typed: TypedSigCachesCleared < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCachesCleared < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCachesCleared < 'c, C > {
        type Target = TypedSigCachesCleared < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCachesCleared < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMixerApplied < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMixerApplied < 'c, C: WithSignals > {
        typed: TypedSigMixerApplied < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMixerApplied < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMixerApplied < 'c, C > {
        type Target = TypedSigMixerApplied < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMixerApplied < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMixerUpdated < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMixerUpdated < 'c, C: WithSignals > {
        typed: TypedSigMixerUpdated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMixerUpdated < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMixerUpdated < 'c, C > {
        type Target = TypedSigMixerUpdated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMixerUpdated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AnimationMixer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationMixer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAnimationMixer < 'c, C > {
        type Target = < < AnimationMixer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AnimationMixer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAnimationMixer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AnimationMixer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}