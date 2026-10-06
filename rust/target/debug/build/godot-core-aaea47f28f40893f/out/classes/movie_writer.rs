#![doc = "Sidecar module for class [`MovieWriter`][crate::classes::MovieWriter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MovieWriter` enums](https://docs.godotengine.org/en/stable/classes/class_moviewriter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MovieWriter`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`IMovieWriter`][crate::classes::IMovieWriter]: virtual methods\n\n\nSee also [Godot docs for `MovieWriter`](https://docs.godotengine.org/en/stable/classes/class_moviewriter.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`MovieWriter::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nGodot can record videos with non-real-time simulation. Like the `--fixed-fps` [command line argument]($DOCS_URL/tutorials/editor/command_line_tutorial.html), this forces the reported `delta` in [`process`][`crate::classes::INode::process`] functions to be identical across frames, regardless of how long it actually took to render the frame. This can be used to record high-quality videos with perfect frame pacing regardless of your hardware's capabilities.\n\nGodot has 3 built-in `MovieWriter`s:\n\n- OGV container with Theora for video and Vorbis for audio (`.ogv` file extension). Lossy compression, medium file sizes, fast encoding. The lossy compression quality can be adjusted by changing \\[member ProjectSettings.editor/movie_writer/video_quality] and \\[member ProjectSettings.editor/movie_writer/ogv/audio_quality]. The resulting file can be viewed in Godot with [`VideoStreamPlayer`][crate::classes::VideoStreamPlayer] and most video players, but not web browsers as they don't support Theora.\n\n- AVI container with MJPEG for video and uncompressed audio (`.avi` file extension). Lossy compression, medium file sizes, fast encoding. The lossy compression quality can be adjusted by changing \\[member ProjectSettings.editor/movie_writer/video_quality]. The resulting file can be viewed in most video players, but it must be converted to another format for viewing on the web or by Godot with [`VideoStreamPlayer`][crate::classes::VideoStreamPlayer]. MJPEG does not support transparency. AVI output is currently limited to a file of 4 GB in size at most.\n\n- PNG image sequence for video and WAV for audio (`.png` file extension). Lossless compression, large file sizes, slow encoding. Designed to be encoded to a video file with another tool such as [FFmpeg](https://ffmpeg.org/) after recording. Transparency is currently not supported, even if the root viewport is set to be transparent.\n\nIf you need to encode to a different format or pipe a stream through third-party software, you can extend the `MovieWriter` class to create your own movie writers. This should typically be done using GDExtension for performance reasons.\n\n**Editor usage:** A default movie file path can be specified in \\[member ProjectSettings.editor/movie_writer/movie_file]. Alternatively, for running single scenes, a `movie_file` metadata can be added to the root node, specifying the path to a movie file that will be used when recording that scene. Once a path is set, click the video reel icon in the top-right corner of the editor to enable Movie Maker mode, then run any scene as usual. The engine will start recording as soon as the splash screen is finished, and it will only stop recording when the engine quits. Click the video reel icon again to disable Movie Maker mode. Note that toggling Movie Maker mode does not affect project instances that are already running.\n\n**Note:** MovieWriter is available for use in both the editor and exported projects, but it is _not_ designed for use by end users to record videos while playing. Players wishing to record gameplay videos should install tools such as [OBS Studio](https://obsproject.com/) or [SimpleScreenRecorder](https://www.maartenbaert.be/simplescreenrecorder/) instead.\n\n**Note:** MJPEG support (`.avi` file extension) depends on the `jpg` module being enabled at compile time (default behavior).\n\n**Note:** OGV support (`.ogv` file extension) depends on the `theora` module being enabled at compile time (default behavior). Theora compression is only available in editor binaries."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MovieWriter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MovieWriter`][crate::classes::MovieWriter].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `MovieWriter` methods](https://docs.godotengine.org/en/stable/classes/class_moviewriter.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMovieWriter: crate::obj::GodotClass < Base = MovieWriter > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Called when the audio sample rate used for recording the audio is requested by the engine. The value returned must be specified in Hz. Defaults to 48000 Hz if [`get_audio_mix_rate`][`crate::classes::IMovieWriter::get_audio_mix_rate`] is not overridden."]
        fn get_audio_mix_rate(&self,) -> u32;
        #[doc = "Called when the audio speaker mode used for recording the audio is requested by the engine. This can affect the number of output channels in the resulting audio file/stream. Defaults to [`SpeakerMode::STEREO`][`crate::classes::audio_server::SpeakerMode::STEREO`] if [`get_audio_speaker_mode`][`crate::classes::IMovieWriter::get_audio_speaker_mode`] is not overridden."]
        fn get_audio_speaker_mode(&self,) -> crate::classes::audio_server::SpeakerMode;
        #[doc = "Called when the engine determines whether this `MovieWriter` is able to handle the file at `path`. Must return `true` if this `MovieWriter` is able to handle the given file path, `false` otherwise. Typically, [`handles_file`][`crate::classes::IMovieWriter::handles_file`] is overridden as follows to allow the user to record a file at any path with a given file extension:\n\n```gdscript\nfunc _handles_file(path):\n\t# Allows specifying an output file with a `.mkv` file extension (case-insensitive),\n\t# either in the Project Settings or with the `--write-movie <path>` command line argument.\n\treturn path.get_extension().to_lower() == \"mkv\"\n```"]
        fn handles_file(&self, path: GString,) -> bool;
        #[doc = "Called once before the engine starts writing video and audio data. `movie_size` is the width and height of the video to save. `fps` is the number of frames per second specified in the project settings or using the `--fixed-fps <fps>` [command line argument]($DOCS_URL/tutorials/editor/command_line_tutorial.html)."]
        fn write_begin(&mut self, movie_size: Vector2i, fps: u32, base_path: GString,) -> crate::global::Error;
        #[doc = "\n# Godot docs\nCalled at the end of every rendered frame. The `frame_image` and `audio_frame_block` function arguments should be written to."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn write_frame_rawptr(&mut self, frame_image: Option < Gd < crate::classes::Image > >, audio_frame_block: crate::meta::RawPtr < * const c_void >,) -> crate::global::Error;
        #[doc = "Called when the engine finishes writing. This occurs when the engine quits by pressing the window manager's close button, or when [`quit`][`crate::classes::SceneTree::quit`] is called.\n\n**Note:** Pressing `Ctrl + C` on the terminal running the editor/project does _not_ result in [`write_end`][`crate::classes::IMovieWriter::write_end`] being called."]
        fn write_end(&mut self,);
        
    }
    impl MovieWriter {
        #[doc = "Adds a writer to be usable by the engine. The supported file extensions can be set by overriding [`handles_file`][`crate::classes::IMovieWriter::handles_file`].\n\n**Note:** [`add_writer`][`crate::classes::MovieWriter::add_writer`] must be called early enough in the engine initialization to work, as movie writing is designed to start at the same time as the rest of the engine."]
        pub fn add_writer(writer: impl AsArg < Option < Gd < crate::classes::MovieWriter >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::MovieWriter > > >,);
            let args = (writer.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MovieWriter", "add_writer", None, args,)
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
    impl crate::obj::GodotClass for MovieWriter {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MovieWriter"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MovieWriter {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MovieWriter {
        
    }
    impl crate::obj::cap::GodotDefault for MovieWriter {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MovieWriter {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MovieWriter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MovieWriter`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MovieWriter__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MovieWriter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MovieWriter;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for MovieWriter {
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