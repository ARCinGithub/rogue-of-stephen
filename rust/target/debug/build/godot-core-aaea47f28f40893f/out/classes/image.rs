#![doc = "Sidecar module for class [`Image`][crate::classes::Image].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Image` enums](https://docs.godotengine.org/en/stable/classes/class_image.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Image`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`image`][crate::classes::image]: sidecar module with related enum/flag types\n* [`IImage`][crate::classes::IImage]: virtual methods\n\n\nSee also [Godot docs for `Image`](https://docs.godotengine.org/en/stable/classes/class_image.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Image::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nNative image datatype. Contains image data which can be converted to an [`ImageTexture`][crate::classes::ImageTexture] and provides commonly used _image processing_ methods. The maximum width and height for an `Image` are `MAX_WIDTH` and `MAX_HEIGHT`.\n\nAn `Image` cannot be assigned to a texture property of an object directly (such as \\[member Sprite2D.texture]), and has to be converted manually to an [`ImageTexture`][crate::classes::ImageTexture] first.\n\n**Note:** Methods that modify the image data cannot be used on VRAM-compressed images. Use [`decompress`][`crate::classes::Image::decompress`] to convert the image to an uncompressed format first.\n\n**Note:** The maximum image size is 16384×16384 pixels due to graphics hardware limitations. Larger images may fail to import."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Image {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Image`][crate::classes::Image].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Image` methods](https://docs.godotengine.org/en/stable/classes/class_image.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IImage: crate::obj::GodotClass < Base = Image > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Image {
        #[doc = "Returns the image's width."]
        pub fn get_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11539usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the image's height."]
        pub fn get_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11540usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the image's size (width and height)."]
        pub fn get_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11541usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the image has generated mipmaps."]
        pub fn has_mipmaps(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "has_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns this image's format."]
        pub fn get_format(&self,) -> crate::classes::image::Format {
            type CallRet = crate::classes::image::Format;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_format", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a copy of the image's raw data."]
        pub fn get_data(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns size (in bytes) of the image's raw data."]
        pub fn get_data_size(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_data_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts this image's format to the given `format`."]
        pub fn convert(&mut self, format: crate::classes::image::Format,) {
            type CallRet = ();
            type CallParams = (crate::classes::image::Format,);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "convert", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of mipmap levels or 0 if the image has no mipmaps. The largest main level image is not counted as a mipmap level by this method, so if you want to include it you can add 1 to this count."]
        pub fn get_mipmap_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_mipmap_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the offset where the image's mipmap with index `mipmap` is stored in the \\[member data] dictionary."]
        pub fn get_mipmap_offset(&self, mipmap: i32,) -> i64 {
            type CallRet = i64;
            type CallParams = (i32,);
            let args = (mipmap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_mipmap_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Resizes the image to the nearest power of 2 for the width and height. If `square` is `true`, sets width and height to be the same. New pixels are calculated using the `interpolation` mode defined via \\[enum Interpolation] constants."]
        pub(crate) fn resize_to_po2_full(&mut self, square: bool, interpolation: crate::classes::image::Interpolation,) {
            type CallRet = ();
            type CallParams = (bool, crate::classes::image::Interpolation,);
            let args = (square, interpolation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "resize_to_po2", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`resize_to_po2_ex`][Self::resize_to_po2_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Resizes the image to the nearest power of 2 for the width and height. If `square` is `true`, sets width and height to be the same. New pixels are calculated using the `interpolation` mode defined via \\[enum Interpolation] constants."]
        #[inline]
        pub fn resize_to_po2(&mut self,) {
            self.resize_to_po2_ex() . done()
        }
        #[doc = "Resizes the image to the nearest power of 2 for the width and height. If `square` is `true`, sets width and height to be the same. New pixels are calculated using the `interpolation` mode defined via \\[enum Interpolation] constants."]
        #[inline]
        pub fn resize_to_po2_ex < 'ex > (&'ex mut self,) -> ExResizeToPo2 < 'ex > {
            ExResizeToPo2::new(self,)
        }
        #[doc = "Resizes the image to the given `width` and `height`. New pixels are calculated using the `interpolation` mode defined via \\[enum Interpolation] constants."]
        pub(crate) fn resize_full(&mut self, width: i32, height: i32, interpolation: crate::classes::image::Interpolation,) {
            type CallRet = ();
            type CallParams = (i32, i32, crate::classes::image::Interpolation,);
            let args = (width, height, interpolation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "resize", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`resize_ex`][Self::resize_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Resizes the image to the given `width` and `height`. New pixels are calculated using the `interpolation` mode defined via \\[enum Interpolation] constants."]
        #[inline]
        pub fn resize(&mut self, width: i32, height: i32,) {
            self.resize_ex(width, height,) . done()
        }
        #[doc = "Resizes the image to the given `width` and `height`. New pixels are calculated using the `interpolation` mode defined via \\[enum Interpolation] constants."]
        #[inline]
        pub fn resize_ex < 'ex > (&'ex mut self, width: i32, height: i32,) -> ExResize < 'ex > {
            ExResize::new(self, width, height,)
        }
        #[doc = "Shrinks the image by a factor of 2 on each axis (this divides the pixel count by 4)."]
        pub fn shrink_x2(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "shrink_x2", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Crops the image to the given `width` and `height`. If the specified size is larger than the current size, the extra area is filled with black pixels."]
        pub fn crop(&mut self, width: i32, height: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (width, height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "crop", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Flips the image horizontally."]
        pub fn flip_x(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "flip_x", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Flips the image vertically."]
        pub fn flip_y(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "flip_y", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generates mipmaps for the image. Mipmaps are precalculated lower-resolution copies of the image that are automatically used if the image needs to be scaled down when rendered. They help improve image quality and performance when rendering. This method returns an error if the image is compressed, in a custom format, or if the image's width/height is `0`. Enabling `renormalize` when generating mipmaps for normal map textures will make sure all resulting vector values are normalized.\n\nIt is possible to check if the image has mipmaps by calling [`has_mipmaps`][`crate::classes::Image::has_mipmaps`] or [`get_mipmap_count`][`crate::classes::Image::get_mipmap_count`]. Calling [`generate_mipmaps`][`crate::classes::Image::generate_mipmaps`] on an image that already has mipmaps will replace existing mipmaps in the image."]
        pub(crate) fn generate_mipmaps_full(&mut self, renormalize: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (bool,);
            let args = (renormalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`generate_mipmaps_ex`][Self::generate_mipmaps_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Generates mipmaps for the image. Mipmaps are precalculated lower-resolution copies of the image that are automatically used if the image needs to be scaled down when rendered. They help improve image quality and performance when rendering. This method returns an error if the image is compressed, in a custom format, or if the image's width/height is `0`. Enabling `renormalize` when generating mipmaps for normal map textures will make sure all resulting vector values are normalized.\n\nIt is possible to check if the image has mipmaps by calling [`has_mipmaps`][`crate::classes::Image::has_mipmaps`] or [`get_mipmap_count`][`crate::classes::Image::get_mipmap_count`]. Calling [`generate_mipmaps`][`crate::classes::Image::generate_mipmaps`] on an image that already has mipmaps will replace existing mipmaps in the image."]
        #[inline]
        pub fn generate_mipmaps(&mut self,) -> crate::global::Error {
            self.generate_mipmaps_ex() . done()
        }
        #[doc = "Generates mipmaps for the image. Mipmaps are precalculated lower-resolution copies of the image that are automatically used if the image needs to be scaled down when rendered. They help improve image quality and performance when rendering. This method returns an error if the image is compressed, in a custom format, or if the image's width/height is `0`. Enabling `renormalize` when generating mipmaps for normal map textures will make sure all resulting vector values are normalized.\n\nIt is possible to check if the image has mipmaps by calling [`has_mipmaps`][`crate::classes::Image::has_mipmaps`] or [`get_mipmap_count`][`crate::classes::Image::get_mipmap_count`]. Calling [`generate_mipmaps`][`crate::classes::Image::generate_mipmaps`] on an image that already has mipmaps will replace existing mipmaps in the image."]
        #[inline]
        pub fn generate_mipmaps_ex < 'ex > (&'ex mut self,) -> ExGenerateMipmaps < 'ex > {
            ExGenerateMipmaps::new(self,)
        }
        #[doc = "Removes the image's mipmaps."]
        pub fn clear_mipmaps(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "clear_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates an empty image of the given size and format. If `use_mipmaps` is `true`, generates mipmaps for this image (see [`generate_mipmaps`][`crate::classes::Image::generate_mipmaps`])."]
        pub fn create(width: i32, height: i32, use_mipmaps: bool, format: crate::classes::image::Format,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (i32, i32, bool, crate::classes::image::Format,);
            let args = (width, height, use_mipmaps, format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "create", None, args,)
            }
        }
        #[doc = "Creates an empty image of the given size and format. If `use_mipmaps` is `true`, generates mipmaps for this image (see [`generate_mipmaps`][`crate::classes::Image::generate_mipmaps`])."]
        pub fn create_empty(width: i32, height: i32, use_mipmaps: bool, format: crate::classes::image::Format,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (i32, i32, bool, crate::classes::image::Format,);
            let args = (width, height, use_mipmaps, format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "create_empty", None, args,)
            }
        }
        #[doc = "Creates a new image of the given size and format. Fills the image with the given raw data. If `use_mipmaps` is `true`, loads the mipmaps for this image from `data`. See [`generate_mipmaps`][`crate::classes::Image::generate_mipmaps`]."]
        pub fn create_from_data(width: i32, height: i32, use_mipmaps: bool, format: crate::classes::image::Format, data: &PackedByteArray,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams < 'a0, > = (i32, i32, bool, crate::classes::image::Format, RefArg < 'a0, PackedByteArray >,);
            let args = (width, height, use_mipmaps, format, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "create_from_data", None, args,)
            }
        }
        #[doc = "Overwrites data of an existing `Image`. Non-static equivalent of [`create_from_data`][`crate::classes::Image::create_from_data`]."]
        pub fn set_data(&mut self, width: i32, height: i32, use_mipmaps: bool, format: crate::classes::image::Format, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, bool, crate::classes::image::Format, RefArg < 'a0, PackedByteArray >,);
            let args = (width, height, use_mipmaps, format, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "set_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the image has no data."]
        pub fn is_empty(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "is_empty", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads an image from file `path`. See [Supported image formats]($DOCS_URL/tutorials/assets_pipeline/importing_images.html#supported-image-formats) for a list of supported image formats and limitations.\n\n**Warning:** This method should only be used in the editor or in cases when you need to load external images at run-time, such as images located at the `user://` directory, and may not work in exported projects.\n\nSee also [`ImageTexture`][crate::classes::ImageTexture] description for usage examples."]
        pub fn load(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11562usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new `Image` and loads data from the specified file."]
        pub fn load_from_file(path: impl AsArg < GString >,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_from_file", None, args,)
            }
        }
        #[doc = "Saves the image as a PNG file to the file at `path`."]
        pub fn save_png(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_png", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the image as a PNG file to a byte array."]
        pub fn save_png_to_buffer(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11565usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_png_to_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the image as a JPEG file to `path` with the specified `quality` between `0.01` and `1.0` (inclusive). Higher `quality` values result in better-looking output at the cost of larger file sizes. Recommended `quality` values are between `0.75` and `0.90`. Even at quality `1.00`, JPEG compression remains lossy.\n\n**Note:** JPEG does not save an alpha channel. If the `Image` contains an alpha channel, the image will still be saved, but the resulting JPEG file won't contain the alpha channel."]
        pub(crate) fn save_jpg_full(&self, path: CowArg < GString >, quality: f32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, f32,);
            let args = (path, quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_jpg", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`save_jpg_ex`][Self::save_jpg_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Saves the image as a JPEG file to `path` with the specified `quality` between `0.01` and `1.0` (inclusive). Higher `quality` values result in better-looking output at the cost of larger file sizes. Recommended `quality` values are between `0.75` and `0.90`. Even at quality `1.00`, JPEG compression remains lossy.\n\n**Note:** JPEG does not save an alpha channel. If the `Image` contains an alpha channel, the image will still be saved, but the resulting JPEG file won't contain the alpha channel."]
        #[inline]
        pub fn save_jpg(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            self.save_jpg_ex(path,) . done()
        }
        #[doc = "Saves the image as a JPEG file to `path` with the specified `quality` between `0.01` and `1.0` (inclusive). Higher `quality` values result in better-looking output at the cost of larger file sizes. Recommended `quality` values are between `0.75` and `0.90`. Even at quality `1.00`, JPEG compression remains lossy.\n\n**Note:** JPEG does not save an alpha channel. If the `Image` contains an alpha channel, the image will still be saved, but the resulting JPEG file won't contain the alpha channel."]
        #[inline]
        pub fn save_jpg_ex < 'ex > (&'ex self, path: impl AsArg < GString > + 'ex,) -> ExSaveJpg < 'ex > {
            ExSaveJpg::new(self, path,)
        }
        #[doc = "Saves the image as a JPEG file to a byte array with the specified `quality` between `0.01` and `1.0` (inclusive). Higher `quality` values result in better-looking output at the cost of larger byte array sizes (and therefore memory usage). Recommended `quality` values are between `0.75` and `0.90`. Even at quality `1.00`, JPEG compression remains lossy.\n\n**Note:** JPEG does not save an alpha channel. If the `Image` contains an alpha channel, the image will still be saved, but the resulting byte array won't contain the alpha channel."]
        pub(crate) fn save_jpg_to_buffer_full(&self, quality: f32,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (f32,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11567usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_jpg_to_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`save_jpg_to_buffer_ex`][Self::save_jpg_to_buffer_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Saves the image as a JPEG file to a byte array with the specified `quality` between `0.01` and `1.0` (inclusive). Higher `quality` values result in better-looking output at the cost of larger byte array sizes (and therefore memory usage). Recommended `quality` values are between `0.75` and `0.90`. Even at quality `1.00`, JPEG compression remains lossy.\n\n**Note:** JPEG does not save an alpha channel. If the `Image` contains an alpha channel, the image will still be saved, but the resulting byte array won't contain the alpha channel."]
        #[inline]
        pub fn save_jpg_to_buffer(&self,) -> PackedByteArray {
            self.save_jpg_to_buffer_ex() . done()
        }
        #[doc = "Saves the image as a JPEG file to a byte array with the specified `quality` between `0.01` and `1.0` (inclusive). Higher `quality` values result in better-looking output at the cost of larger byte array sizes (and therefore memory usage). Recommended `quality` values are between `0.75` and `0.90`. Even at quality `1.00`, JPEG compression remains lossy.\n\n**Note:** JPEG does not save an alpha channel. If the `Image` contains an alpha channel, the image will still be saved, but the resulting byte array won't contain the alpha channel."]
        #[inline]
        pub fn save_jpg_to_buffer_ex < 'ex > (&'ex self,) -> ExSaveJpgToBuffer < 'ex > {
            ExSaveJpgToBuffer::new(self,)
        }
        #[doc = "Saves the image as an EXR file to `path`. If `grayscale` is `true` and the image has only one channel, it will be saved explicitly as monochrome rather than one red channel. This function will return [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`] if Godot was compiled without the TinyEXR module."]
        pub(crate) fn save_exr_full(&self, path: CowArg < GString >, grayscale: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (path, grayscale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11568usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_exr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`save_exr_ex`][Self::save_exr_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Saves the image as an EXR file to `path`. If `grayscale` is `true` and the image has only one channel, it will be saved explicitly as monochrome rather than one red channel. This function will return [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`] if Godot was compiled without the TinyEXR module."]
        #[inline]
        pub fn save_exr(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            self.save_exr_ex(path,) . done()
        }
        #[doc = "Saves the image as an EXR file to `path`. If `grayscale` is `true` and the image has only one channel, it will be saved explicitly as monochrome rather than one red channel. This function will return [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`] if Godot was compiled without the TinyEXR module."]
        #[inline]
        pub fn save_exr_ex < 'ex > (&'ex self, path: impl AsArg < GString > + 'ex,) -> ExSaveExr < 'ex > {
            ExSaveExr::new(self, path,)
        }
        #[doc = "Saves the image as an EXR file to a byte array. If `grayscale` is `true` and the image has only one channel, it will be saved explicitly as monochrome rather than one red channel. This function will return an empty byte array if Godot was compiled without the TinyEXR module."]
        pub(crate) fn save_exr_to_buffer_full(&self, grayscale: bool,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (bool,);
            let args = (grayscale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11569usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_exr_to_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`save_exr_to_buffer_ex`][Self::save_exr_to_buffer_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Saves the image as an EXR file to a byte array. If `grayscale` is `true` and the image has only one channel, it will be saved explicitly as monochrome rather than one red channel. This function will return an empty byte array if Godot was compiled without the TinyEXR module."]
        #[inline]
        pub fn save_exr_to_buffer(&self,) -> PackedByteArray {
            self.save_exr_to_buffer_ex() . done()
        }
        #[doc = "Saves the image as an EXR file to a byte array. If `grayscale` is `true` and the image has only one channel, it will be saved explicitly as monochrome rather than one red channel. This function will return an empty byte array if Godot was compiled without the TinyEXR module."]
        #[inline]
        pub fn save_exr_to_buffer_ex < 'ex > (&'ex self,) -> ExSaveExrToBuffer < 'ex > {
            ExSaveExrToBuffer::new(self,)
        }
        #[doc = "Saves the image as a DDS (DirectDraw Surface) file to `path`. DDS is a container format that can store textures in various compression formats, such as DXT1, DXT5, or BC7. This function will return [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`] if Godot was compiled without the DDS module.\n\n**Note:** The DDS module may be disabled in certain builds, which means [`save_dds`][`crate::classes::Image::save_dds`] will return [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`] when it is called from an exported project."]
        pub fn save_dds(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11570usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_dds", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the image as a DDS (DirectDraw Surface) file to a byte array. DDS is a container format that can store textures in various compression formats, such as DXT1, DXT5, or BC7. This function will return an empty byte array if Godot was compiled without the DDS module.\n\n**Note:** The DDS module may be disabled in certain builds, which means [`save_dds_to_buffer`][`crate::classes::Image::save_dds_to_buffer`] will return an empty byte array when it is called from an exported project."]
        pub fn save_dds_to_buffer(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_dds_to_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the image as a WebP (Web Picture) file to the file at `path`. By default it will save lossless. If `lossy` is `true`, the image will be saved lossy, using the `quality` setting between `0.0` and `1.0` (inclusive). Lossless WebP offers more efficient compression than PNG.\n\n**Note:** The WebP format is limited to a size of 16383×16383 pixels, while PNG can save larger images."]
        pub(crate) fn save_webp_full(&self, path: CowArg < GString >, lossy: bool, quality: f32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, f32,);
            let args = (path, lossy, quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_webp", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`save_webp_ex`][Self::save_webp_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Saves the image as a WebP (Web Picture) file to the file at `path`. By default it will save lossless. If `lossy` is `true`, the image will be saved lossy, using the `quality` setting between `0.0` and `1.0` (inclusive). Lossless WebP offers more efficient compression than PNG.\n\n**Note:** The WebP format is limited to a size of 16383×16383 pixels, while PNG can save larger images."]
        #[inline]
        pub fn save_webp(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            self.save_webp_ex(path,) . done()
        }
        #[doc = "Saves the image as a WebP (Web Picture) file to the file at `path`. By default it will save lossless. If `lossy` is `true`, the image will be saved lossy, using the `quality` setting between `0.0` and `1.0` (inclusive). Lossless WebP offers more efficient compression than PNG.\n\n**Note:** The WebP format is limited to a size of 16383×16383 pixels, while PNG can save larger images."]
        #[inline]
        pub fn save_webp_ex < 'ex > (&'ex self, path: impl AsArg < GString > + 'ex,) -> ExSaveWebp < 'ex > {
            ExSaveWebp::new(self, path,)
        }
        #[doc = "Saves the image as a WebP (Web Picture) file to a byte array. By default it will save lossless. If `lossy` is `true`, the image will be saved lossy, using the `quality` setting between `0.0` and `1.0` (inclusive). Lossless WebP offers more efficient compression than PNG.\n\n**Note:** The WebP format is limited to a size of 16383×16383 pixels, while PNG can save larger images."]
        pub(crate) fn save_webp_to_buffer_full(&self, lossy: bool, quality: f32,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (bool, f32,);
            let args = (lossy, quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_webp_to_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`save_webp_to_buffer_ex`][Self::save_webp_to_buffer_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Saves the image as a WebP (Web Picture) file to a byte array. By default it will save lossless. If `lossy` is `true`, the image will be saved lossy, using the `quality` setting between `0.0` and `1.0` (inclusive). Lossless WebP offers more efficient compression than PNG.\n\n**Note:** The WebP format is limited to a size of 16383×16383 pixels, while PNG can save larger images."]
        #[inline]
        pub fn save_webp_to_buffer(&self,) -> PackedByteArray {
            self.save_webp_to_buffer_ex() . done()
        }
        #[doc = "Saves the image as a WebP (Web Picture) file to a byte array. By default it will save lossless. If `lossy` is `true`, the image will be saved lossy, using the `quality` setting between `0.0` and `1.0` (inclusive). Lossless WebP offers more efficient compression than PNG.\n\n**Note:** The WebP format is limited to a size of 16383×16383 pixels, while PNG can save larger images."]
        #[inline]
        pub fn save_webp_to_buffer_ex < 'ex > (&'ex self,) -> ExSaveWebpToBuffer < 'ex > {
            ExSaveWebpToBuffer::new(self,)
        }
        #[doc = "Returns [`AlphaMode::BLEND`][`crate::classes::image::AlphaMode::BLEND`] if the image has data for alpha values. Returns [`AlphaMode::BIT`][`crate::classes::image::AlphaMode::BIT`] if all the alpha values are stored in a single bit. Returns [`AlphaMode::NONE`][`crate::classes::image::AlphaMode::NONE`] if no data for alpha values is found."]
        pub fn detect_alpha(&self,) -> crate::classes::image::AlphaMode {
            type CallRet = crate::classes::image::AlphaMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "detect_alpha", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if all the image's pixels have an alpha value of 0. Returns `false` if any pixel has an alpha value higher than 0."]
        pub fn is_invisible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "is_invisible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the color channels used by this image. If the image is compressed, the original `source` must be specified."]
        pub(crate) fn detect_used_channels_full(&self, source: crate::classes::image::CompressSource,) -> crate::classes::image::UsedChannels {
            type CallRet = crate::classes::image::UsedChannels;
            type CallParams = (crate::classes::image::CompressSource,);
            let args = (source,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "detect_used_channels", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`detect_used_channels_ex`][Self::detect_used_channels_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the color channels used by this image. If the image is compressed, the original `source` must be specified."]
        #[inline]
        pub fn detect_used_channels(&self,) -> crate::classes::image::UsedChannels {
            self.detect_used_channels_ex() . done()
        }
        #[doc = "Returns the color channels used by this image. If the image is compressed, the original `source` must be specified."]
        #[inline]
        pub fn detect_used_channels_ex < 'ex > (&'ex self,) -> ExDetectUsedChannels < 'ex > {
            ExDetectUsedChannels::new(self,)
        }
        #[doc = "Compresses the image with a VRAM-compressed format to use less memory. Can not directly access pixel data while the image is compressed. Returns error if the chosen compression mode is not available.\n\nThe `source` parameter helps to pick the best compression method for DXT and ETC2 formats. It is ignored for ASTC compression.\n\nThe `astc_format` parameter is only taken into account when using ASTC compression; it is ignored for all other formats.\n\n**Note:** [`compress`][`crate::classes::Image::compress`] is only supported in editor builds. When run in an exported project, this method always returns [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`]."]
        pub(crate) fn compress_full(&mut self, mode: crate::classes::image::CompressMode, source: crate::classes::image::CompressSource, astc_format: crate::classes::image::AstcFormat,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (crate::classes::image::CompressMode, crate::classes::image::CompressSource, crate::classes::image::AstcFormat,);
            let args = (mode, source, astc_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "compress", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`compress_ex`][Self::compress_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Compresses the image with a VRAM-compressed format to use less memory. Can not directly access pixel data while the image is compressed. Returns error if the chosen compression mode is not available.\n\nThe `source` parameter helps to pick the best compression method for DXT and ETC2 formats. It is ignored for ASTC compression.\n\nThe `astc_format` parameter is only taken into account when using ASTC compression; it is ignored for all other formats.\n\n**Note:** [`compress`][`crate::classes::Image::compress`] is only supported in editor builds. When run in an exported project, this method always returns [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`]."]
        #[inline]
        pub fn compress(&mut self, mode: crate::classes::image::CompressMode,) -> crate::global::Error {
            self.compress_ex(mode,) . done()
        }
        #[doc = "Compresses the image with a VRAM-compressed format to use less memory. Can not directly access pixel data while the image is compressed. Returns error if the chosen compression mode is not available.\n\nThe `source` parameter helps to pick the best compression method for DXT and ETC2 formats. It is ignored for ASTC compression.\n\nThe `astc_format` parameter is only taken into account when using ASTC compression; it is ignored for all other formats.\n\n**Note:** [`compress`][`crate::classes::Image::compress`] is only supported in editor builds. When run in an exported project, this method always returns [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`]."]
        #[inline]
        pub fn compress_ex < 'ex > (&'ex mut self, mode: crate::classes::image::CompressMode,) -> ExCompress < 'ex > {
            ExCompress::new(self, mode,)
        }
        #[doc = "Compresses the image with a VRAM-compressed format to use less memory. Can not directly access pixel data while the image is compressed. Returns error if the chosen compression mode is not available.\n\nThis is an alternative to [`compress`][`crate::classes::Image::compress`] that lets the user supply the channels used in order for the compressor to pick the best DXT and ETC2 formats. For other formats (non DXT or ETC2), this argument is ignored.\n\nThe `astc_format` parameter is only taken into account when using ASTC compression; it is ignored for all other formats.\n\n**Note:** [`compress_from_channels`][`crate::classes::Image::compress_from_channels`] is only supported in editor builds. When run in an exported project, this method always returns [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`]."]
        pub(crate) fn compress_from_channels_full(&mut self, mode: crate::classes::image::CompressMode, channels: crate::classes::image::UsedChannels, astc_format: crate::classes::image::AstcFormat,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (crate::classes::image::CompressMode, crate::classes::image::UsedChannels, crate::classes::image::AstcFormat,);
            let args = (mode, channels, astc_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "compress_from_channels", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`compress_from_channels_ex`][Self::compress_from_channels_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Compresses the image with a VRAM-compressed format to use less memory. Can not directly access pixel data while the image is compressed. Returns error if the chosen compression mode is not available.\n\nThis is an alternative to [`compress`][`crate::classes::Image::compress`] that lets the user supply the channels used in order for the compressor to pick the best DXT and ETC2 formats. For other formats (non DXT or ETC2), this argument is ignored.\n\nThe `astc_format` parameter is only taken into account when using ASTC compression; it is ignored for all other formats.\n\n**Note:** [`compress_from_channels`][`crate::classes::Image::compress_from_channels`] is only supported in editor builds. When run in an exported project, this method always returns [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`]."]
        #[inline]
        pub fn compress_from_channels(&mut self, mode: crate::classes::image::CompressMode, channels: crate::classes::image::UsedChannels,) -> crate::global::Error {
            self.compress_from_channels_ex(mode, channels,) . done()
        }
        #[doc = "Compresses the image with a VRAM-compressed format to use less memory. Can not directly access pixel data while the image is compressed. Returns error if the chosen compression mode is not available.\n\nThis is an alternative to [`compress`][`crate::classes::Image::compress`] that lets the user supply the channels used in order for the compressor to pick the best DXT and ETC2 formats. For other formats (non DXT or ETC2), this argument is ignored.\n\nThe `astc_format` parameter is only taken into account when using ASTC compression; it is ignored for all other formats.\n\n**Note:** [`compress_from_channels`][`crate::classes::Image::compress_from_channels`] is only supported in editor builds. When run in an exported project, this method always returns [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`]."]
        #[inline]
        pub fn compress_from_channels_ex < 'ex > (&'ex mut self, mode: crate::classes::image::CompressMode, channels: crate::classes::image::UsedChannels,) -> ExCompressFromChannels < 'ex > {
            ExCompressFromChannels::new(self, mode, channels,)
        }
        #[doc = "Decompresses the image if it is VRAM-compressed in a supported format. This increases memory utilization, but allows modifying the image. Returns [`Error::OK`][`crate::global::Error::OK`] if the format is supported, otherwise [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`]. All VRAM-compressed formats supported by Godot can be decompressed with this method, except [`Format::ETC2_R11S`][`crate::classes::image::Format::ETC2_R11S`], [`Format::ETC2_RG11S`][`crate::classes::image::Format::ETC2_RG11S`], and [`Format::ETC2_RGB8A1`][`crate::classes::image::Format::ETC2_RGB8A1`]."]
        pub fn decompress(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "decompress", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the image is compressed."]
        pub fn is_compressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "is_compressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Rotates the image in the specified `direction` by `90` degrees. The width and height of the image must be greater than `1`. If the width and height are not equal, the image will be resized."]
        pub fn rotate_90(&mut self, direction: crate::global::ClockDirection,) {
            type CallRet = ();
            type CallParams = (crate::global::ClockDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "rotate_90", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Rotates the image by `180` degrees. The width and height of the image must be greater than `1`."]
        pub fn rotate_180(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "rotate_180", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Blends low-alpha pixels with nearby pixels."]
        pub fn fix_alpha_edges(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "fix_alpha_edges", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Multiplies color values with alpha values. Resulting color values for a pixel are `(color * alpha)/256`. See also \\[member CanvasItemMaterial.blend_mode]."]
        pub fn premultiply_alpha(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "premultiply_alpha", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the raw data from nonlinear sRGB encoding to linear encoding using a lookup table. Only works on images with [`Format::RGB8`][`crate::classes::image::Format::RGB8`] or [`Format::RGBA8`][`crate::classes::image::Format::RGBA8`] formats.\n\n**Note:** The 8-bit formats required by this method are not suitable for storing linearly encoded values; a significant amount of color information will be lost in darker values. To maintain image quality, this method should not be used."]
        pub fn srgb_to_linear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "srgb_to_linear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the entire image from linear encoding to nonlinear sRGB encoding by using a lookup table. Only works on images with [`Format::RGB8`][`crate::classes::image::Format::RGB8`] or [`Format::RGBA8`][`crate::classes::image::Format::RGBA8`] formats."]
        pub fn linear_to_srgb(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "linear_to_srgb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the image's data to represent coordinates on a 3D plane. This is used when the image represents a normal map. A normal map can add lots of detail to a 3D surface without increasing the polygon count."]
        pub fn normal_map_to_xy(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "normal_map_to_xy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts a standard linear RGBE (Red Green Blue Exponent) image to an image that uses nonlinear sRGB encoding."]
        pub fn rgbe_to_srgb(&mut self,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "rgbe_to_srgb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts a bump map to a normal map. A bump map provides a height offset per-pixel, while a normal map provides a normal direction per pixel."]
        pub(crate) fn bump_map_to_normal_map_full(&mut self, bump_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bump_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "bump_map_to_normal_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`bump_map_to_normal_map_ex`][Self::bump_map_to_normal_map_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Converts a bump map to a normal map. A bump map provides a height offset per-pixel, while a normal map provides a normal direction per pixel."]
        #[inline]
        pub fn bump_map_to_normal_map(&mut self,) {
            self.bump_map_to_normal_map_ex() . done()
        }
        #[doc = "Converts a bump map to a normal map. A bump map provides a height offset per-pixel, while a normal map provides a normal direction per pixel."]
        #[inline]
        pub fn bump_map_to_normal_map_ex < 'ex > (&'ex mut self,) -> ExBumpMapToNormalMap < 'ex > {
            ExBumpMapToNormalMap::new(self,)
        }
        #[doc = "Compute image metrics on the current image and the compared image. This can be used to calculate the similarity between two images.\n\nThe dictionary contains `max`, `mean`, `mean_squared`, `root_mean_squared` and `peak_snr`."]
        pub fn compute_image_metrics(&mut self, compared_image: impl AsArg < Option < Gd < crate::classes::Image >> >, use_luma: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, bool,);
            let args = (compared_image.into_arg(), use_luma,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "compute_image_metrics", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Copies `src_rect` from `src` image to this image at coordinates `dst`, clipped accordingly to both image bounds. This image and `src` image **must** have the same format. `src_rect` with non-positive size is treated as empty.\n\n**Note:** The alpha channel data in `src` will overwrite the corresponding data in this image at the target position. To blend alpha channels, use [`blend_rect`][`crate::classes::Image::blend_rect`] instead."]
        pub fn blit_rect(&mut self, src: impl AsArg < Option < Gd < crate::classes::Image >> >, src_rect: Rect2i, dst: Vector2i,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, Rect2i, Vector2i,);
            let args = (src.into_arg(), src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "blit_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Blits `src_rect` area from `src` image to this image at the coordinates given by `dst`, clipped accordingly to both image bounds. `src` pixel is copied onto `dst` if the corresponding `mask` pixel's alpha value is not 0. This image and `src` image **must** have the same format. `src` image and `mask` image **must** have the same size (width and height) but they can have different formats. `src_rect` with non-positive size is treated as empty."]
        pub fn blit_rect_mask(&mut self, src: impl AsArg < Option < Gd < crate::classes::Image >> >, mask: impl AsArg < Option < Gd < crate::classes::Image >> >, src_rect: Rect2i, dst: Vector2i,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, CowArg < 'a1, Option < Gd < crate::classes::Image > > >, Rect2i, Vector2i,);
            let args = (src.into_arg(), mask.into_arg(), src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "blit_rect_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Alpha-blends `src_rect` from `src` image to this image at coordinates `dst`, clipped accordingly to both image bounds. This image and `src` image **must** have the same format. `src_rect` with non-positive size is treated as empty."]
        pub fn blend_rect(&mut self, src: impl AsArg < Option < Gd < crate::classes::Image >> >, src_rect: Rect2i, dst: Vector2i,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, Rect2i, Vector2i,);
            let args = (src.into_arg(), src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "blend_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Alpha-blends `src_rect` from `src` image to this image using `mask` image at coordinates `dst`, clipped accordingly to both image bounds. Alpha channels are required for both `src` and `mask`. `dst` pixels and `src` pixels will blend if the corresponding mask pixel's alpha value is not 0. This image and `src` image **must** have the same format. `src` image and `mask` image **must** have the same size (width and height) but they can have different formats. `src_rect` with non-positive size is treated as empty."]
        pub fn blend_rect_mask(&mut self, src: impl AsArg < Option < Gd < crate::classes::Image >> >, mask: impl AsArg < Option < Gd < crate::classes::Image >> >, src_rect: Rect2i, dst: Vector2i,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, CowArg < 'a1, Option < Gd < crate::classes::Image > > >, Rect2i, Vector2i,);
            let args = (src.into_arg(), mask.into_arg(), src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "blend_rect_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Fills the image with `color`."]
        pub fn fill(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "fill", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Fills `rect` with `color`."]
        pub fn fill_rect(&mut self, rect: Rect2i, color: Color,) {
            type CallRet = ();
            type CallParams = (Rect2i, Color,);
            let args = (rect, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11596usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "fill_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Rect2i`][crate::builtin::Rect2i] enclosing the visible portion of the image, considering each pixel with a non-zero alpha channel as visible."]
        pub fn get_used_rect(&self,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11597usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_used_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a new `Image` that is a copy of this `Image`'s area specified with `region`."]
        pub fn get_region(&self, region: Rect2i,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rect2i,);
            let args = (region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11598usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Copies `src` image to this image."]
        pub fn copy_from(&mut self, src: impl AsArg < Option < Gd < crate::classes::Image >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (src.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11599usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "copy_from", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the color of the pixel at `point`.\n\nThis is the same as [`get_pixel`][`crate::classes::Image::get_pixel`], but with a [`Vector2i`][crate::builtin::Vector2i] argument instead of two integer arguments."]
        pub fn get_pixelv(&self, point: Vector2i,) -> Color {
            type CallRet = Color;
            type CallParams = (Vector2i,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11600usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_pixelv", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the color of the pixel at `(x, y)`.\n\nThis is the same as [`get_pixelv`][`crate::classes::Image::get_pixelv`], but with two integer arguments instead of a [`Vector2i`][crate::builtin::Vector2i] argument."]
        pub fn get_pixel(&self, x: i32, y: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32, i32,);
            let args = (x, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11601usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_pixel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Color`][crate::builtin::Color] of the pixel at `point` to `color`.\n\n\n```gdscript\nvar img_width = 10\nvar img_height = 5\nvar img = Image.create(img_width, img_height, false, Image.FORMAT_RGBA8)\n\nimg.set_pixelv(Vector2i(1, 2), Color.RED) # Sets the color at (1, 2) to red.\n```\n\n\nThis is the same as [`set_pixel`][`crate::classes::Image::set_pixel`], but with a [`Vector2i`][crate::builtin::Vector2i] argument instead of two integer arguments.\n\n**Note:** Depending on the image's format, the color set here may be clamped or lose precision. Do not assume the color returned by [`get_pixelv`][`crate::classes::Image::get_pixelv`] to be identical to the one set here; any comparisons will likely need to use an approximation like [`approx_eq`][`crate::builtin::math::ApproxEq::approx_eq`].\n\n**Note:** On grayscale image formats, only the red channel of `color` is used (and alpha if relevant). The green and blue channels are ignored."]
        pub fn set_pixelv(&mut self, point: Vector2i, color: Color,) {
            type CallRet = ();
            type CallParams = (Vector2i, Color,);
            let args = (point, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11602usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "set_pixelv", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Color`][crate::builtin::Color] of the pixel at `(x, y)` to `color`.\n\n\n```gdscript\nvar img_width = 10\nvar img_height = 5\nvar img = Image.create(img_width, img_height, false, Image.FORMAT_RGBA8)\n\nimg.set_pixel(1, 2, Color.RED) # Sets the color at (1, 2) to red.\n```\n\n\nThis is the same as [`set_pixelv`][`crate::classes::Image::set_pixelv`], but with a two integer arguments instead of a [`Vector2i`][crate::builtin::Vector2i] argument.\n\n**Note:** Depending on the image's format, the color set here may be clamped or lose precision. Do not assume the color returned by [`get_pixel`][`crate::classes::Image::get_pixel`] to be identical to the one set here; any comparisons will likely need to use an approximation like [`approx_eq`][`crate::builtin::math::ApproxEq::approx_eq`].\n\n**Note:** On grayscale image formats, only the red channel of `color` is used (and alpha if relevant). The green and blue channels are ignored."]
        pub fn set_pixel(&mut self, x: i32, y: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, i32, Color,);
            let args = (x, y, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11603usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "set_pixel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adjusts this image's `brightness`, `contrast`, and `saturation` by the given values. Does not work if the image is compressed (see [`is_compressed`][`crate::classes::Image::is_compressed`])."]
        pub fn adjust_bcs(&mut self, brightness: f32, contrast: f32, saturation: f32,) {
            type CallRet = ();
            type CallParams = (f32, f32, f32,);
            let args = (brightness, contrast, saturation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11604usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "adjust_bcs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads an image from the binary contents of a PNG file."]
        pub fn load_png_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_png_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads an image from the binary contents of a JPEG file."]
        pub fn load_jpg_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_jpg_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads an image from the binary contents of a WebP file."]
        pub fn load_webp_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_webp_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads an image from the binary contents of a TGA file.\n\n**Note:** This method is only available in engine builds with the TGA module enabled. By default, the TGA module is enabled, but it can be disabled at build-time using the `module_tga_enabled=no` SCons option."]
        pub fn load_tga_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_tga_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads an image from the binary contents of a BMP file.\n\n**Note:** Godot's BMP module doesn't support 16-bit per pixel images. Only 1-bit, 4-bit, 8-bit, 24-bit, and 32-bit per pixel images are supported.\n\n**Note:** This method is only available in engine builds with the BMP module enabled. By default, the BMP module is enabled, but it can be disabled at build-time using the `module_bmp_enabled=no` SCons option."]
        pub fn load_bmp_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_bmp_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads an image from the binary contents of a [KTX](https://github.com/KhronosGroup/KTX-Software) file. Unlike most image formats, KTX can store VRAM-compressed data and embed mipmaps.\n\n**Note:** Godot's libktx implementation only supports 2D images. Cubemaps, texture arrays, and de-padding are not supported.\n\n**Note:** This method is only available in engine builds with the KTX module enabled. By default, the KTX module is enabled, but it can be disabled at build-time using the `module_ktx_enabled=no` SCons option."]
        pub fn load_ktx_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_ktx_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads an image from the binary contents of a DDS file.\n\n**Note:** This method is only available in engine builds with the DDS module enabled. By default, the DDS module is enabled, but it can be disabled at build-time using the `module_dds_enabled=no` SCons option."]
        pub fn load_dds_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_dds_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads an image from the binary contents of an OpenEXR file."]
        pub fn load_exr_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_exr_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads an image from the UTF-8 binary contents of an **uncompressed** SVG file (**.svg**).\n\n**Note:** Beware when using compressed SVG files (like **.svgz**), they need to be `decompressed` before loading.\n\n**Note:** This method is only available in engine builds with the SVG module enabled. By default, the SVG module is enabled, but it can be disabled at build-time using the `module_svg_enabled=no` SCons option."]
        pub(crate) fn load_svg_from_buffer_full(&mut self, buffer: RefArg < PackedByteArray >, scale: f32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >, f32,);
            let args = (buffer, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_svg_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`load_svg_from_buffer_ex`][Self::load_svg_from_buffer_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Loads an image from the UTF-8 binary contents of an **uncompressed** SVG file (**.svg**).\n\n**Note:** Beware when using compressed SVG files (like **.svgz**), they need to be `decompressed` before loading.\n\n**Note:** This method is only available in engine builds with the SVG module enabled. By default, the SVG module is enabled, but it can be disabled at build-time using the `module_svg_enabled=no` SCons option."]
        #[inline]
        pub fn load_svg_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            self.load_svg_from_buffer_ex(buffer,) . done()
        }
        #[doc = "Loads an image from the UTF-8 binary contents of an **uncompressed** SVG file (**.svg**).\n\n**Note:** Beware when using compressed SVG files (like **.svgz**), they need to be `decompressed` before loading.\n\n**Note:** This method is only available in engine builds with the SVG module enabled. By default, the SVG module is enabled, but it can be disabled at build-time using the `module_svg_enabled=no` SCons option."]
        #[inline]
        pub fn load_svg_from_buffer_ex < 'ex > (&'ex mut self, buffer: &'ex PackedByteArray,) -> ExLoadSvgFromBuffer < 'ex > {
            ExLoadSvgFromBuffer::new(self, buffer,)
        }
        #[doc = "Loads an image from the string contents of an SVG file (**.svg**).\n\n**Note:** This method is only available in engine builds with the SVG module enabled. By default, the SVG module is enabled, but it can be disabled at build-time using the `module_svg_enabled=no` SCons option."]
        pub(crate) fn load_svg_from_string_full(&mut self, svg_str: CowArg < GString >, scale: f32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, f32,);
            let args = (svg_str, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_svg_from_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`load_svg_from_string_ex`][Self::load_svg_from_string_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Loads an image from the string contents of an SVG file (**.svg**).\n\n**Note:** This method is only available in engine builds with the SVG module enabled. By default, the SVG module is enabled, but it can be disabled at build-time using the `module_svg_enabled=no` SCons option."]
        #[inline]
        pub fn load_svg_from_string(&mut self, svg_str: impl AsArg < GString >,) -> crate::global::Error {
            self.load_svg_from_string_ex(svg_str,) . done()
        }
        #[doc = "Loads an image from the string contents of an SVG file (**.svg**).\n\n**Note:** This method is only available in engine builds with the SVG module enabled. By default, the SVG module is enabled, but it can be disabled at build-time using the `module_svg_enabled=no` SCons option."]
        #[inline]
        pub fn load_svg_from_string_ex < 'ex > (&'ex mut self, svg_str: impl AsArg < GString > + 'ex,) -> ExLoadSvgFromString < 'ex > {
            ExLoadSvgFromString::new(self, svg_str,)
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
        pub const MAX_WIDTH: i32 = 16777216i32;
        pub const MAX_HEIGHT: i32 = 16777216i32;
        
    }
    impl crate::obj::GodotClass for Image {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Image"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Image {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Image {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Image {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Image {
        
    }
    impl crate::obj::cap::GodotDefault for Image {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Image {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Image {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Image`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Image__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Image > for $Class {
                
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
#[doc = "Default-param extender for [`Image::resize_to_po2_ex`][super::Image::resize_to_po2_ex]."]
#[must_use]
pub struct ExResizeToPo2 < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Image, square: bool, interpolation: crate::classes::image::Interpolation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExResizeToPo2 < 'ex > {
    fn new(surround_object: &'ex mut re_export::Image,) -> Self {
        let square = false;
        let interpolation = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, square: square, interpolation: interpolation,
        }
    }
    #[inline]
    pub fn square(self, square: bool) -> Self {
        Self {
            square: square, .. self
        }
    }
    #[inline]
    pub fn interpolation(self, interpolation: crate::classes::image::Interpolation) -> Self {
        Self {
            interpolation: interpolation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, square, interpolation,
        }
        = self;
        re_export::Image::resize_to_po2_full(surround_object, square, interpolation,)
    }
}
#[doc = "Default-param extender for [`Image::resize_ex`][super::Image::resize_ex]."]
#[must_use]
pub struct ExResize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Image, width: i32, height: i32, interpolation: crate::classes::image::Interpolation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExResize < 'ex > {
    fn new(surround_object: &'ex mut re_export::Image, width: i32, height: i32,) -> Self {
        let interpolation = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, interpolation: interpolation,
        }
    }
    #[inline]
    pub fn interpolation(self, interpolation: crate::classes::image::Interpolation) -> Self {
        Self {
            interpolation: interpolation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, width, height, interpolation,
        }
        = self;
        re_export::Image::resize_full(surround_object, width, height, interpolation,)
    }
}
#[doc = "Default-param extender for [`Image::generate_mipmaps_ex`][super::Image::generate_mipmaps_ex]."]
#[must_use]
pub struct ExGenerateMipmaps < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Image, renormalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGenerateMipmaps < 'ex > {
    fn new(surround_object: &'ex mut re_export::Image,) -> Self {
        let renormalize = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, renormalize: renormalize,
        }
    }
    #[inline]
    pub fn renormalize(self, renormalize: bool) -> Self {
        Self {
            renormalize: renormalize, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, renormalize,
        }
        = self;
        re_export::Image::generate_mipmaps_full(surround_object, renormalize,)
    }
}
#[doc = "Default-param extender for [`Image::save_jpg_ex`][super::Image::save_jpg_ex]."]
#[must_use]
pub struct ExSaveJpg < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Image, path: CowArg < 'ex, GString >, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSaveJpg < 'ex > {
    fn new(surround_object: &'ex re_export::Image, path: impl AsArg < GString > + 'ex,) -> Self {
        let quality = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), quality: quality,
        }
    }
    #[inline]
    pub fn quality(self, quality: f32) -> Self {
        Self {
            quality: quality, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, quality,
        }
        = self;
        re_export::Image::save_jpg_full(surround_object, path, quality,)
    }
}
#[doc = "Default-param extender for [`Image::save_jpg_to_buffer_ex`][super::Image::save_jpg_to_buffer_ex]."]
#[must_use]
pub struct ExSaveJpgToBuffer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Image, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSaveJpgToBuffer < 'ex > {
    fn new(surround_object: &'ex re_export::Image,) -> Self {
        let quality = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, quality: quality,
        }
    }
    #[inline]
    pub fn quality(self, quality: f32) -> Self {
        Self {
            quality: quality, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, quality,
        }
        = self;
        re_export::Image::save_jpg_to_buffer_full(surround_object, quality,)
    }
}
#[doc = "Default-param extender for [`Image::save_exr_ex`][super::Image::save_exr_ex]."]
#[must_use]
pub struct ExSaveExr < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Image, path: CowArg < 'ex, GString >, grayscale: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSaveExr < 'ex > {
    fn new(surround_object: &'ex re_export::Image, path: impl AsArg < GString > + 'ex,) -> Self {
        let grayscale = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), grayscale: grayscale,
        }
    }
    #[inline]
    pub fn grayscale(self, grayscale: bool) -> Self {
        Self {
            grayscale: grayscale, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, grayscale,
        }
        = self;
        re_export::Image::save_exr_full(surround_object, path, grayscale,)
    }
}
#[doc = "Default-param extender for [`Image::save_exr_to_buffer_ex`][super::Image::save_exr_to_buffer_ex]."]
#[must_use]
pub struct ExSaveExrToBuffer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Image, grayscale: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSaveExrToBuffer < 'ex > {
    fn new(surround_object: &'ex re_export::Image,) -> Self {
        let grayscale = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, grayscale: grayscale,
        }
    }
    #[inline]
    pub fn grayscale(self, grayscale: bool) -> Self {
        Self {
            grayscale: grayscale, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, grayscale,
        }
        = self;
        re_export::Image::save_exr_to_buffer_full(surround_object, grayscale,)
    }
}
#[doc = "Default-param extender for [`Image::save_webp_ex`][super::Image::save_webp_ex]."]
#[must_use]
pub struct ExSaveWebp < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Image, path: CowArg < 'ex, GString >, lossy: bool, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSaveWebp < 'ex > {
    fn new(surround_object: &'ex re_export::Image, path: impl AsArg < GString > + 'ex,) -> Self {
        let lossy = false;
        let quality = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), lossy: lossy, quality: quality,
        }
    }
    #[inline]
    pub fn lossy(self, lossy: bool) -> Self {
        Self {
            lossy: lossy, .. self
        }
    }
    #[inline]
    pub fn quality(self, quality: f32) -> Self {
        Self {
            quality: quality, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, lossy, quality,
        }
        = self;
        re_export::Image::save_webp_full(surround_object, path, lossy, quality,)
    }
}
#[doc = "Default-param extender for [`Image::save_webp_to_buffer_ex`][super::Image::save_webp_to_buffer_ex]."]
#[must_use]
pub struct ExSaveWebpToBuffer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Image, lossy: bool, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSaveWebpToBuffer < 'ex > {
    fn new(surround_object: &'ex re_export::Image,) -> Self {
        let lossy = false;
        let quality = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, lossy: lossy, quality: quality,
        }
    }
    #[inline]
    pub fn lossy(self, lossy: bool) -> Self {
        Self {
            lossy: lossy, .. self
        }
    }
    #[inline]
    pub fn quality(self, quality: f32) -> Self {
        Self {
            quality: quality, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, lossy, quality,
        }
        = self;
        re_export::Image::save_webp_to_buffer_full(surround_object, lossy, quality,)
    }
}
#[doc = "Default-param extender for [`Image::detect_used_channels_ex`][super::Image::detect_used_channels_ex]."]
#[must_use]
pub struct ExDetectUsedChannels < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Image, source: crate::classes::image::CompressSource,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDetectUsedChannels < 'ex > {
    fn new(surround_object: &'ex re_export::Image,) -> Self {
        let source = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, source: source,
        }
    }
    #[inline]
    pub fn source(self, source: crate::classes::image::CompressSource) -> Self {
        Self {
            source: source, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::image::UsedChannels {
        let Self {
            _phantom, surround_object, source,
        }
        = self;
        re_export::Image::detect_used_channels_full(surround_object, source,)
    }
}
#[doc = "Default-param extender for [`Image::compress_ex`][super::Image::compress_ex]."]
#[must_use]
pub struct ExCompress < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Image, mode: crate::classes::image::CompressMode, source: crate::classes::image::CompressSource, astc_format: crate::classes::image::AstcFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCompress < 'ex > {
    fn new(surround_object: &'ex mut re_export::Image, mode: crate::classes::image::CompressMode,) -> Self {
        let source = crate::obj::EngineEnum::from_ord(0);
        let astc_format = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mode: mode, source: source, astc_format: astc_format,
        }
    }
    #[inline]
    pub fn source(self, source: crate::classes::image::CompressSource) -> Self {
        Self {
            source: source, .. self
        }
    }
    #[inline]
    pub fn astc_format(self, astc_format: crate::classes::image::AstcFormat) -> Self {
        Self {
            astc_format: astc_format, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, mode, source, astc_format,
        }
        = self;
        re_export::Image::compress_full(surround_object, mode, source, astc_format,)
    }
}
#[doc = "Default-param extender for [`Image::compress_from_channels_ex`][super::Image::compress_from_channels_ex]."]
#[must_use]
pub struct ExCompressFromChannels < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Image, mode: crate::classes::image::CompressMode, channels: crate::classes::image::UsedChannels, astc_format: crate::classes::image::AstcFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCompressFromChannels < 'ex > {
    fn new(surround_object: &'ex mut re_export::Image, mode: crate::classes::image::CompressMode, channels: crate::classes::image::UsedChannels,) -> Self {
        let astc_format = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mode: mode, channels: channels, astc_format: astc_format,
        }
    }
    #[inline]
    pub fn astc_format(self, astc_format: crate::classes::image::AstcFormat) -> Self {
        Self {
            astc_format: astc_format, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, mode, channels, astc_format,
        }
        = self;
        re_export::Image::compress_from_channels_full(surround_object, mode, channels, astc_format,)
    }
}
#[doc = "Default-param extender for [`Image::bump_map_to_normal_map_ex`][super::Image::bump_map_to_normal_map_ex]."]
#[must_use]
pub struct ExBumpMapToNormalMap < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Image, bump_scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBumpMapToNormalMap < 'ex > {
    fn new(surround_object: &'ex mut re_export::Image,) -> Self {
        let bump_scale = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bump_scale: bump_scale,
        }
    }
    #[inline]
    pub fn bump_scale(self, bump_scale: f32) -> Self {
        Self {
            bump_scale: bump_scale, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, bump_scale,
        }
        = self;
        re_export::Image::bump_map_to_normal_map_full(surround_object, bump_scale,)
    }
}
#[doc = "Default-param extender for [`Image::load_svg_from_buffer_ex`][super::Image::load_svg_from_buffer_ex]."]
#[must_use]
pub struct ExLoadSvgFromBuffer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Image, buffer: CowArg < 'ex, PackedByteArray >, scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExLoadSvgFromBuffer < 'ex > {
    fn new(surround_object: &'ex mut re_export::Image, buffer: &'ex PackedByteArray,) -> Self {
        let scale = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, buffer: CowArg::Borrowed(buffer), scale: scale,
        }
    }
    #[inline]
    pub fn scale(self, scale: f32) -> Self {
        Self {
            scale: scale, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, buffer, scale,
        }
        = self;
        re_export::Image::load_svg_from_buffer_full(surround_object, buffer.cow_as_arg(), scale,)
    }
}
#[doc = "Default-param extender for [`Image::load_svg_from_string_ex`][super::Image::load_svg_from_string_ex]."]
#[must_use]
pub struct ExLoadSvgFromString < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Image, svg_str: CowArg < 'ex, GString >, scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExLoadSvgFromString < 'ex > {
    fn new(surround_object: &'ex mut re_export::Image, svg_str: impl AsArg < GString > + 'ex,) -> Self {
        let scale = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, svg_str: svg_str.into_arg(), scale: scale,
        }
    }
    #[inline]
    pub fn scale(self, scale: f32) -> Self {
        Self {
            scale: scale, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, svg_str, scale,
        }
        = self;
        re_export::Image::load_svg_from_string_full(surround_object, svg_str, scale,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Format {
    ord: i32
}
impl Format {
    #[doc(alias = "FORMAT_L8")]
    #[doc = "Godot enumerator name: `FORMAT_L8`"]
    pub const L8: Format = Format {
        ord: 0i32
    };
    #[doc(alias = "FORMAT_LA8")]
    #[doc = "Godot enumerator name: `FORMAT_LA8`"]
    pub const LA8: Format = Format {
        ord: 1i32
    };
    #[doc(alias = "FORMAT_R8")]
    #[doc = "Godot enumerator name: `FORMAT_R8`"]
    pub const R8: Format = Format {
        ord: 2i32
    };
    #[doc(alias = "FORMAT_RG8")]
    #[doc = "Godot enumerator name: `FORMAT_RG8`"]
    pub const RG8: Format = Format {
        ord: 3i32
    };
    #[doc(alias = "FORMAT_RGB8")]
    #[doc = "Godot enumerator name: `FORMAT_RGB8`"]
    pub const RGB8: Format = Format {
        ord: 4i32
    };
    #[doc(alias = "FORMAT_RGBA8")]
    #[doc = "Godot enumerator name: `FORMAT_RGBA8`"]
    pub const RGBA8: Format = Format {
        ord: 5i32
    };
    #[doc(alias = "FORMAT_RGBA4444")]
    #[doc = "Godot enumerator name: `FORMAT_RGBA4444`"]
    pub const RGBA4444: Format = Format {
        ord: 6i32
    };
    #[doc(alias = "FORMAT_RGB565")]
    #[doc = "Godot enumerator name: `FORMAT_RGB565`"]
    pub const RGB565: Format = Format {
        ord: 7i32
    };
    #[doc(alias = "FORMAT_RF")]
    #[doc = "Godot enumerator name: `FORMAT_RF`"]
    pub const RF: Format = Format {
        ord: 8i32
    };
    #[doc(alias = "FORMAT_RGF")]
    #[doc = "Godot enumerator name: `FORMAT_RGF`"]
    pub const RGF: Format = Format {
        ord: 9i32
    };
    #[doc(alias = "FORMAT_RGBF")]
    #[doc = "Godot enumerator name: `FORMAT_RGBF`"]
    pub const RGBF: Format = Format {
        ord: 10i32
    };
    #[doc(alias = "FORMAT_RGBAF")]
    #[doc = "Godot enumerator name: `FORMAT_RGBAF`"]
    pub const RGBAF: Format = Format {
        ord: 11i32
    };
    #[doc(alias = "FORMAT_RH")]
    #[doc = "Godot enumerator name: `FORMAT_RH`"]
    pub const RH: Format = Format {
        ord: 12i32
    };
    #[doc(alias = "FORMAT_RGH")]
    #[doc = "Godot enumerator name: `FORMAT_RGH`"]
    pub const RGH: Format = Format {
        ord: 13i32
    };
    #[doc(alias = "FORMAT_RGBH")]
    #[doc = "Godot enumerator name: `FORMAT_RGBH`"]
    pub const RGBH: Format = Format {
        ord: 14i32
    };
    #[doc(alias = "FORMAT_RGBAH")]
    #[doc = "Godot enumerator name: `FORMAT_RGBAH`"]
    pub const RGBAH: Format = Format {
        ord: 15i32
    };
    #[doc(alias = "FORMAT_RGBE9995")]
    #[doc = "Godot enumerator name: `FORMAT_RGBE9995`"]
    pub const RGBE9995: Format = Format {
        ord: 16i32
    };
    #[doc(alias = "FORMAT_DXT1")]
    #[doc = "Godot enumerator name: `FORMAT_DXT1`"]
    pub const DXT1: Format = Format {
        ord: 17i32
    };
    #[doc(alias = "FORMAT_DXT3")]
    #[doc = "Godot enumerator name: `FORMAT_DXT3`"]
    pub const DXT3: Format = Format {
        ord: 18i32
    };
    #[doc(alias = "FORMAT_DXT5")]
    #[doc = "Godot enumerator name: `FORMAT_DXT5`"]
    pub const DXT5: Format = Format {
        ord: 19i32
    };
    #[doc(alias = "FORMAT_RGTC_R")]
    #[doc = "Godot enumerator name: `FORMAT_RGTC_R`"]
    pub const RGTC_R: Format = Format {
        ord: 20i32
    };
    #[doc(alias = "FORMAT_RGTC_RG")]
    #[doc = "Godot enumerator name: `FORMAT_RGTC_RG`"]
    pub const RGTC_RG: Format = Format {
        ord: 21i32
    };
    #[doc(alias = "FORMAT_BPTC_RGBA")]
    #[doc = "Godot enumerator name: `FORMAT_BPTC_RGBA`"]
    pub const BPTC_RGBA: Format = Format {
        ord: 22i32
    };
    #[doc(alias = "FORMAT_BPTC_RGBF")]
    #[doc = "Godot enumerator name: `FORMAT_BPTC_RGBF`"]
    pub const BPTC_RGBF: Format = Format {
        ord: 23i32
    };
    #[doc(alias = "FORMAT_BPTC_RGBFU")]
    #[doc = "Godot enumerator name: `FORMAT_BPTC_RGBFU`"]
    pub const BPTC_RGBFU: Format = Format {
        ord: 24i32
    };
    #[doc(alias = "FORMAT_ETC")]
    #[doc = "Godot enumerator name: `FORMAT_ETC`"]
    pub const ETC: Format = Format {
        ord: 25i32
    };
    #[doc(alias = "FORMAT_ETC2_R11")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_R11`"]
    pub const ETC2_R11: Format = Format {
        ord: 26i32
    };
    #[doc(alias = "FORMAT_ETC2_R11S")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_R11S`"]
    pub const ETC2_R11S: Format = Format {
        ord: 27i32
    };
    #[doc(alias = "FORMAT_ETC2_RG11")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RG11`"]
    pub const ETC2_RG11: Format = Format {
        ord: 28i32
    };
    #[doc(alias = "FORMAT_ETC2_RG11S")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RG11S`"]
    pub const ETC2_RG11S: Format = Format {
        ord: 29i32
    };
    #[doc(alias = "FORMAT_ETC2_RGB8")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RGB8`"]
    pub const ETC2_RGB8: Format = Format {
        ord: 30i32
    };
    #[doc(alias = "FORMAT_ETC2_RGBA8")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RGBA8`"]
    pub const ETC2_RGBA8: Format = Format {
        ord: 31i32
    };
    #[doc(alias = "FORMAT_ETC2_RGB8A1")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RGB8A1`"]
    pub const ETC2_RGB8A1: Format = Format {
        ord: 32i32
    };
    #[doc(alias = "FORMAT_ETC2_RA_AS_RG")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RA_AS_RG`"]
    pub const ETC2_RA_AS_RG: Format = Format {
        ord: 33i32
    };
    #[doc(alias = "FORMAT_DXT5_RA_AS_RG")]
    #[doc = "Godot enumerator name: `FORMAT_DXT5_RA_AS_RG`"]
    pub const DXT5_RA_AS_RG: Format = Format {
        ord: 34i32
    };
    #[doc(alias = "FORMAT_ASTC_4x4")]
    #[doc = "Godot enumerator name: `FORMAT_ASTC_4x4`"]
    pub const ASTC_4x4: Format = Format {
        ord: 35i32
    };
    #[doc(alias = "FORMAT_ASTC_4x4_HDR")]
    #[doc = "Godot enumerator name: `FORMAT_ASTC_4x4_HDR`"]
    pub const ASTC_4x4_HDR: Format = Format {
        ord: 36i32
    };
    #[doc(alias = "FORMAT_ASTC_8x8")]
    #[doc = "Godot enumerator name: `FORMAT_ASTC_8x8`"]
    pub const ASTC_8x8: Format = Format {
        ord: 37i32
    };
    #[doc(alias = "FORMAT_ASTC_8x8_HDR")]
    #[doc = "Godot enumerator name: `FORMAT_ASTC_8x8_HDR`"]
    pub const ASTC_8x8_HDR: Format = Format {
        ord: 38i32
    };
    #[doc(alias = "FORMAT_R16")]
    #[doc = "Godot enumerator name: `FORMAT_R16`"]
    pub const R16: Format = Format {
        ord: 39i32
    };
    #[doc(alias = "FORMAT_RG16")]
    #[doc = "Godot enumerator name: `FORMAT_RG16`"]
    pub const RG16: Format = Format {
        ord: 40i32
    };
    #[doc(alias = "FORMAT_RGB16")]
    #[doc = "Godot enumerator name: `FORMAT_RGB16`"]
    pub const RGB16: Format = Format {
        ord: 41i32
    };
    #[doc(alias = "FORMAT_RGBA16")]
    #[doc = "Godot enumerator name: `FORMAT_RGBA16`"]
    pub const RGBA16: Format = Format {
        ord: 42i32
    };
    #[doc(alias = "FORMAT_R16I")]
    #[doc = "Godot enumerator name: `FORMAT_R16I`"]
    pub const R16I: Format = Format {
        ord: 43i32
    };
    #[doc(alias = "FORMAT_RG16I")]
    #[doc = "Godot enumerator name: `FORMAT_RG16I`"]
    pub const RG16I: Format = Format {
        ord: 44i32
    };
    #[doc(alias = "FORMAT_RGB16I")]
    #[doc = "Godot enumerator name: `FORMAT_RGB16I`"]
    pub const RGB16I: Format = Format {
        ord: 45i32
    };
    #[doc(alias = "FORMAT_RGBA16I")]
    #[doc = "Godot enumerator name: `FORMAT_RGBA16I`"]
    pub const RGBA16I: Format = Format {
        ord: 46i32
    };
    #[doc(alias = "FORMAT_MAX")]
    #[doc = "Godot enumerator name: `FORMAT_MAX`"]
    pub const MAX: Format = Format {
        ord: 47i32
    };
    
}
impl std::fmt::Debug for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Format") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Format {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 | ord @ 40i32 | ord @ 41i32 | ord @ 42i32 | ord @ 43i32 | ord @ 44i32 | ord @ 45i32 | ord @ 46i32 | ord @ 47i32 => Some(Self {
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
            Self::L8 => "L8", Self::LA8 => "LA8", Self::R8 => "R8", Self::RG8 => "RG8", Self::RGB8 => "RGB8", Self::RGBA8 => "RGBA8", Self::RGBA4444 => "RGBA4444", Self::RGB565 => "RGB565", Self::RF => "RF", Self::RGF => "RGF", Self::RGBF => "RGBF", Self::RGBAF => "RGBAF", Self::RH => "RH", Self::RGH => "RGH", Self::RGBH => "RGBH", Self::RGBAH => "RGBAH", Self::RGBE9995 => "RGBE9995", Self::DXT1 => "DXT1", Self::DXT3 => "DXT3", Self::DXT5 => "DXT5", Self::RGTC_R => "RGTC_R", Self::RGTC_RG => "RGTC_RG", Self::BPTC_RGBA => "BPTC_RGBA", Self::BPTC_RGBF => "BPTC_RGBF", Self::BPTC_RGBFU => "BPTC_RGBFU", Self::ETC => "ETC", Self::ETC2_R11 => "ETC2_R11", Self::ETC2_R11S => "ETC2_R11S", Self::ETC2_RG11 => "ETC2_RG11", Self::ETC2_RG11S => "ETC2_RG11S", Self::ETC2_RGB8 => "ETC2_RGB8", Self::ETC2_RGBA8 => "ETC2_RGBA8", Self::ETC2_RGB8A1 => "ETC2_RGB8A1", Self::ETC2_RA_AS_RG => "ETC2_RA_AS_RG", Self::DXT5_RA_AS_RG => "DXT5_RA_AS_RG", Self::ASTC_4x4 => "ASTC_4x4", Self::ASTC_4x4_HDR => "ASTC_4x4_HDR", Self::ASTC_8x8 => "ASTC_8x8", Self::ASTC_8x8_HDR => "ASTC_8x8_HDR", Self::R16 => "R16", Self::RG16 => "RG16", Self::RGB16 => "RGB16", Self::RGBA16 => "RGBA16", Self::R16I => "R16I", Self::RG16I => "RG16I", Self::RGB16I => "RGB16I", Self::RGBA16I => "RGBA16I", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Format::L8, Format::LA8, Format::R8, Format::RG8, Format::RGB8, Format::RGBA8, Format::RGBA4444, Format::RGB565, Format::RF, Format::RGF, Format::RGBF, Format::RGBAF, Format::RH, Format::RGH, Format::RGBH, Format::RGBAH, Format::RGBE9995, Format::DXT1, Format::DXT3, Format::DXT5, Format::RGTC_R, Format::RGTC_RG, Format::BPTC_RGBA, Format::BPTC_RGBF, Format::BPTC_RGBFU, Format::ETC, Format::ETC2_R11, Format::ETC2_R11S, Format::ETC2_RG11, Format::ETC2_RG11S, Format::ETC2_RGB8, Format::ETC2_RGBA8, Format::ETC2_RGB8A1, Format::ETC2_RA_AS_RG, Format::DXT5_RA_AS_RG, Format::ASTC_4x4, Format::ASTC_4x4_HDR, Format::ASTC_8x8, Format::ASTC_8x8_HDR, Format::R16, Format::RG16, Format::RGB16, Format::RGBA16, Format::R16I, Format::RG16I, Format::RGB16I, Format::RGBA16I]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Format >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("L8", "FORMAT_L8", Format::L8), crate::meta::inspect::EnumConstant::new("LA8", "FORMAT_LA8", Format::LA8), crate::meta::inspect::EnumConstant::new("R8", "FORMAT_R8", Format::R8), crate::meta::inspect::EnumConstant::new("RG8", "FORMAT_RG8", Format::RG8), crate::meta::inspect::EnumConstant::new("RGB8", "FORMAT_RGB8", Format::RGB8), crate::meta::inspect::EnumConstant::new("RGBA8", "FORMAT_RGBA8", Format::RGBA8), crate::meta::inspect::EnumConstant::new("RGBA4444", "FORMAT_RGBA4444", Format::RGBA4444), crate::meta::inspect::EnumConstant::new("RGB565", "FORMAT_RGB565", Format::RGB565), crate::meta::inspect::EnumConstant::new("RF", "FORMAT_RF", Format::RF), crate::meta::inspect::EnumConstant::new("RGF", "FORMAT_RGF", Format::RGF), crate::meta::inspect::EnumConstant::new("RGBF", "FORMAT_RGBF", Format::RGBF), crate::meta::inspect::EnumConstant::new("RGBAF", "FORMAT_RGBAF", Format::RGBAF), crate::meta::inspect::EnumConstant::new("RH", "FORMAT_RH", Format::RH), crate::meta::inspect::EnumConstant::new("RGH", "FORMAT_RGH", Format::RGH), crate::meta::inspect::EnumConstant::new("RGBH", "FORMAT_RGBH", Format::RGBH), crate::meta::inspect::EnumConstant::new("RGBAH", "FORMAT_RGBAH", Format::RGBAH), crate::meta::inspect::EnumConstant::new("RGBE9995", "FORMAT_RGBE9995", Format::RGBE9995), crate::meta::inspect::EnumConstant::new("DXT1", "FORMAT_DXT1", Format::DXT1), crate::meta::inspect::EnumConstant::new("DXT3", "FORMAT_DXT3", Format::DXT3), crate::meta::inspect::EnumConstant::new("DXT5", "FORMAT_DXT5", Format::DXT5), crate::meta::inspect::EnumConstant::new("RGTC_R", "FORMAT_RGTC_R", Format::RGTC_R), crate::meta::inspect::EnumConstant::new("RGTC_RG", "FORMAT_RGTC_RG", Format::RGTC_RG), crate::meta::inspect::EnumConstant::new("BPTC_RGBA", "FORMAT_BPTC_RGBA", Format::BPTC_RGBA), crate::meta::inspect::EnumConstant::new("BPTC_RGBF", "FORMAT_BPTC_RGBF", Format::BPTC_RGBF), crate::meta::inspect::EnumConstant::new("BPTC_RGBFU", "FORMAT_BPTC_RGBFU", Format::BPTC_RGBFU), crate::meta::inspect::EnumConstant::new("ETC", "FORMAT_ETC", Format::ETC), crate::meta::inspect::EnumConstant::new("ETC2_R11", "FORMAT_ETC2_R11", Format::ETC2_R11), crate::meta::inspect::EnumConstant::new("ETC2_R11S", "FORMAT_ETC2_R11S", Format::ETC2_R11S), crate::meta::inspect::EnumConstant::new("ETC2_RG11", "FORMAT_ETC2_RG11", Format::ETC2_RG11), crate::meta::inspect::EnumConstant::new("ETC2_RG11S", "FORMAT_ETC2_RG11S", Format::ETC2_RG11S), crate::meta::inspect::EnumConstant::new("ETC2_RGB8", "FORMAT_ETC2_RGB8", Format::ETC2_RGB8), crate::meta::inspect::EnumConstant::new("ETC2_RGBA8", "FORMAT_ETC2_RGBA8", Format::ETC2_RGBA8), crate::meta::inspect::EnumConstant::new("ETC2_RGB8A1", "FORMAT_ETC2_RGB8A1", Format::ETC2_RGB8A1), crate::meta::inspect::EnumConstant::new("ETC2_RA_AS_RG", "FORMAT_ETC2_RA_AS_RG", Format::ETC2_RA_AS_RG), crate::meta::inspect::EnumConstant::new("DXT5_RA_AS_RG", "FORMAT_DXT5_RA_AS_RG", Format::DXT5_RA_AS_RG), crate::meta::inspect::EnumConstant::new("ASTC_4x4", "FORMAT_ASTC_4x4", Format::ASTC_4x4), crate::meta::inspect::EnumConstant::new("ASTC_4x4_HDR", "FORMAT_ASTC_4x4_HDR", Format::ASTC_4x4_HDR), crate::meta::inspect::EnumConstant::new("ASTC_8x8", "FORMAT_ASTC_8x8", Format::ASTC_8x8), crate::meta::inspect::EnumConstant::new("ASTC_8x8_HDR", "FORMAT_ASTC_8x8_HDR", Format::ASTC_8x8_HDR), crate::meta::inspect::EnumConstant::new("R16", "FORMAT_R16", Format::R16), crate::meta::inspect::EnumConstant::new("RG16", "FORMAT_RG16", Format::RG16), crate::meta::inspect::EnumConstant::new("RGB16", "FORMAT_RGB16", Format::RGB16), crate::meta::inspect::EnumConstant::new("RGBA16", "FORMAT_RGBA16", Format::RGBA16), crate::meta::inspect::EnumConstant::new("R16I", "FORMAT_R16I", Format::R16I), crate::meta::inspect::EnumConstant::new("RG16I", "FORMAT_RG16I", Format::RG16I), crate::meta::inspect::EnumConstant::new("RGB16I", "FORMAT_RGB16I", Format::RGB16I), crate::meta::inspect::EnumConstant::new("RGBA16I", "FORMAT_RGBA16I", Format::RGBA16I), crate::meta::inspect::EnumConstant::new("MAX", "FORMAT_MAX", Format::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Format {
    const ENUMERATOR_COUNT: usize = 47usize;
    
}
impl crate::meta::GodotConvert for Format {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Format L8", 0i64), EnumeratorShape::new_int("Format La8", 1i64), EnumeratorShape::new_int("Format R8", 2i64), EnumeratorShape::new_int("Format Rg8", 3i64), EnumeratorShape::new_int("Format Rgb8", 4i64), EnumeratorShape::new_int("Format Rgba8", 5i64), EnumeratorShape::new_int("Format Rgba4444", 6i64), EnumeratorShape::new_int("Format Rgb565", 7i64), EnumeratorShape::new_int("Format Rf", 8i64), EnumeratorShape::new_int("Format Rgf", 9i64), EnumeratorShape::new_int("Format Rgbf", 10i64), EnumeratorShape::new_int("Format Rgbaf", 11i64), EnumeratorShape::new_int("Format Rh", 12i64), EnumeratorShape::new_int("Format Rgh", 13i64), EnumeratorShape::new_int("Format Rgbh", 14i64), EnumeratorShape::new_int("Format Rgbah", 15i64), EnumeratorShape::new_int("Format Rgbe9995", 16i64), EnumeratorShape::new_int("Format Dxt1", 17i64), EnumeratorShape::new_int("Format Dxt3", 18i64), EnumeratorShape::new_int("Format Dxt5", 19i64), EnumeratorShape::new_int("Format Rgtc R", 20i64), EnumeratorShape::new_int("Format Rgtc Rg", 21i64), EnumeratorShape::new_int("Format Bptc Rgba", 22i64), EnumeratorShape::new_int("Format Bptc Rgbf", 23i64), EnumeratorShape::new_int("Format Bptc Rgbfu", 24i64), EnumeratorShape::new_int("Format Etc", 25i64), EnumeratorShape::new_int("Format Etc2 R11", 26i64), EnumeratorShape::new_int("Format Etc2 R11s", 27i64), EnumeratorShape::new_int("Format Etc2 Rg11", 28i64), EnumeratorShape::new_int("Format Etc2 Rg11s", 29i64), EnumeratorShape::new_int("Format Etc2 Rgb8", 30i64), EnumeratorShape::new_int("Format Etc2 Rgba8", 31i64), EnumeratorShape::new_int("Format Etc2 Rgb8a1", 32i64), EnumeratorShape::new_int("Format Etc2 Ra As Rg", 33i64), EnumeratorShape::new_int("Format Dxt5 Ra As Rg", 34i64), EnumeratorShape::new_int("Format Astc 4x4", 35i64), EnumeratorShape::new_int("Format Astc 4x4 Hdr", 36i64), EnumeratorShape::new_int("Format Astc 8x8", 37i64), EnumeratorShape::new_int("Format Astc 8x8 Hdr", 38i64), EnumeratorShape::new_int("Format R16", 39i64), EnumeratorShape::new_int("Format Rg16", 40i64), EnumeratorShape::new_int("Format Rgb16", 41i64), EnumeratorShape::new_int("Format Rgba16", 42i64), EnumeratorShape::new_int("Format R16i", 43i64), EnumeratorShape::new_int("Format Rg16i", 44i64), EnumeratorShape::new_int("Format Rgb16i", 45i64), EnumeratorShape::new_int("Format Rgba16i", 46i64), EnumeratorShape::new_int("Format Max", 47i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Image.Format")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Format {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Format {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Format {
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
impl crate::registry::property::Export for Format {
    
}
impl crate::meta::Element for Format {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Interpolation {
    ord: i32
}
impl Interpolation {
    #[doc(alias = "INTERPOLATE_NEAREST")]
    #[doc = "Godot enumerator name: `INTERPOLATE_NEAREST`"]
    pub const NEAREST: Interpolation = Interpolation {
        ord: 0i32
    };
    #[doc(alias = "INTERPOLATE_BILINEAR")]
    #[doc = "Godot enumerator name: `INTERPOLATE_BILINEAR`"]
    pub const BILINEAR: Interpolation = Interpolation {
        ord: 1i32
    };
    #[doc(alias = "INTERPOLATE_CUBIC")]
    #[doc = "Godot enumerator name: `INTERPOLATE_CUBIC`"]
    pub const CUBIC: Interpolation = Interpolation {
        ord: 2i32
    };
    #[doc(alias = "INTERPOLATE_TRILINEAR")]
    #[doc = "Godot enumerator name: `INTERPOLATE_TRILINEAR`"]
    pub const TRILINEAR: Interpolation = Interpolation {
        ord: 3i32
    };
    #[doc(alias = "INTERPOLATE_LANCZOS")]
    #[doc = "Godot enumerator name: `INTERPOLATE_LANCZOS`"]
    pub const LANCZOS: Interpolation = Interpolation {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Interpolation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Interpolation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Interpolation {
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
            Self::NEAREST => "NEAREST", Self::BILINEAR => "BILINEAR", Self::CUBIC => "CUBIC", Self::TRILINEAR => "TRILINEAR", Self::LANCZOS => "LANCZOS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Interpolation::NEAREST, Interpolation::BILINEAR, Interpolation::CUBIC, Interpolation::TRILINEAR, Interpolation::LANCZOS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Interpolation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "INTERPOLATE_NEAREST", Interpolation::NEAREST), crate::meta::inspect::EnumConstant::new("BILINEAR", "INTERPOLATE_BILINEAR", Interpolation::BILINEAR), crate::meta::inspect::EnumConstant::new("CUBIC", "INTERPOLATE_CUBIC", Interpolation::CUBIC), crate::meta::inspect::EnumConstant::new("TRILINEAR", "INTERPOLATE_TRILINEAR", Interpolation::TRILINEAR), crate::meta::inspect::EnumConstant::new("LANCZOS", "INTERPOLATE_LANCZOS", Interpolation::LANCZOS)]
        }
    }
}
impl crate::meta::GodotConvert for Interpolation {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Interpolate Nearest", 0i64), EnumeratorShape::new_int("Interpolate Bilinear", 1i64), EnumeratorShape::new_int("Interpolate Cubic", 2i64), EnumeratorShape::new_int("Interpolate Trilinear", 3i64), EnumeratorShape::new_int("Interpolate Lanczos", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Image.Interpolation")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Interpolation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Interpolation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Interpolation {
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
impl crate::registry::property::Export for Interpolation {
    
}
impl crate::meta::Element for Interpolation {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlphaMode {
    ord: i32
}
impl AlphaMode {
    #[doc(alias = "ALPHA_NONE")]
    #[doc = "Godot enumerator name: `ALPHA_NONE`"]
    pub const NONE: AlphaMode = AlphaMode {
        ord: 0i32
    };
    #[doc(alias = "ALPHA_BIT")]
    #[doc = "Godot enumerator name: `ALPHA_BIT`"]
    pub const BIT: AlphaMode = AlphaMode {
        ord: 1i32
    };
    #[doc(alias = "ALPHA_BLEND")]
    #[doc = "Godot enumerator name: `ALPHA_BLEND`"]
    pub const BLEND: AlphaMode = AlphaMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AlphaMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AlphaMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AlphaMode {
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
            Self::NONE => "NONE", Self::BIT => "BIT", Self::BLEND => "BLEND", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AlphaMode::NONE, AlphaMode::BIT, AlphaMode::BLEND]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AlphaMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "ALPHA_NONE", AlphaMode::NONE), crate::meta::inspect::EnumConstant::new("BIT", "ALPHA_BIT", AlphaMode::BIT), crate::meta::inspect::EnumConstant::new("BLEND", "ALPHA_BLEND", AlphaMode::BLEND)]
        }
    }
}
impl crate::meta::GodotConvert for AlphaMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Alpha None", 0i64), EnumeratorShape::new_int("Alpha Bit", 1i64), EnumeratorShape::new_int("Alpha Blend", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Image.AlphaMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AlphaMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AlphaMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AlphaMode {
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
impl crate::registry::property::Export for AlphaMode {
    
}
impl crate::meta::Element for AlphaMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompressMode {
    ord: i32
}
impl CompressMode {
    #[doc(alias = "COMPRESS_S3TC")]
    #[doc = "Godot enumerator name: `COMPRESS_S3TC`"]
    pub const S3TC: CompressMode = CompressMode {
        ord: 0i32
    };
    #[doc(alias = "COMPRESS_ETC")]
    #[doc = "Godot enumerator name: `COMPRESS_ETC`"]
    pub const ETC: CompressMode = CompressMode {
        ord: 1i32
    };
    #[doc(alias = "COMPRESS_ETC2")]
    #[doc = "Godot enumerator name: `COMPRESS_ETC2`"]
    pub const ETC2: CompressMode = CompressMode {
        ord: 2i32
    };
    #[doc(alias = "COMPRESS_BPTC")]
    #[doc = "Godot enumerator name: `COMPRESS_BPTC`"]
    pub const BPTC: CompressMode = CompressMode {
        ord: 3i32
    };
    #[doc(alias = "COMPRESS_ASTC")]
    #[doc = "Godot enumerator name: `COMPRESS_ASTC`"]
    pub const ASTC: CompressMode = CompressMode {
        ord: 4i32
    };
    #[doc(alias = "COMPRESS_MAX")]
    #[doc = "Godot enumerator name: `COMPRESS_MAX`"]
    pub const MAX: CompressMode = CompressMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for CompressMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompressMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompressMode {
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
            Self::S3TC => "S3TC", Self::ETC => "ETC", Self::ETC2 => "ETC2", Self::BPTC => "BPTC", Self::ASTC => "ASTC", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompressMode::S3TC, CompressMode::ETC, CompressMode::ETC2, CompressMode::BPTC, CompressMode::ASTC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompressMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("S3TC", "COMPRESS_S3TC", CompressMode::S3TC), crate::meta::inspect::EnumConstant::new("ETC", "COMPRESS_ETC", CompressMode::ETC), crate::meta::inspect::EnumConstant::new("ETC2", "COMPRESS_ETC2", CompressMode::ETC2), crate::meta::inspect::EnumConstant::new("BPTC", "COMPRESS_BPTC", CompressMode::BPTC), crate::meta::inspect::EnumConstant::new("ASTC", "COMPRESS_ASTC", CompressMode::ASTC), crate::meta::inspect::EnumConstant::new("MAX", "COMPRESS_MAX", CompressMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CompressMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for CompressMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Compress S3tc", 0i64), EnumeratorShape::new_int("Compress Etc", 1i64), EnumeratorShape::new_int("Compress Etc2", 2i64), EnumeratorShape::new_int("Compress Bptc", 3i64), EnumeratorShape::new_int("Compress Astc", 4i64), EnumeratorShape::new_int("Compress Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Image.CompressMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CompressMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompressMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CompressMode {
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
impl crate::registry::property::Export for CompressMode {
    
}
impl crate::meta::Element for CompressMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct UsedChannels {
    ord: i32
}
impl UsedChannels {
    #[doc(alias = "USED_CHANNELS_L")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_L`"]
    pub const L: UsedChannels = UsedChannels {
        ord: 0i32
    };
    #[doc(alias = "USED_CHANNELS_LA")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_LA`"]
    pub const LA: UsedChannels = UsedChannels {
        ord: 1i32
    };
    #[doc(alias = "USED_CHANNELS_R")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_R`"]
    pub const R: UsedChannels = UsedChannels {
        ord: 2i32
    };
    #[doc(alias = "USED_CHANNELS_RG")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_RG`"]
    pub const RG: UsedChannels = UsedChannels {
        ord: 3i32
    };
    #[doc(alias = "USED_CHANNELS_RGB")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_RGB`"]
    pub const RGB: UsedChannels = UsedChannels {
        ord: 4i32
    };
    #[doc(alias = "USED_CHANNELS_RGBA")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_RGBA`"]
    pub const RGBA: UsedChannels = UsedChannels {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for UsedChannels {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("UsedChannels") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for UsedChannels {
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
            Self::L => "L", Self::LA => "LA", Self::R => "R", Self::RG => "RG", Self::RGB => "RGB", Self::RGBA => "RGBA", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[UsedChannels::L, UsedChannels::LA, UsedChannels::R, UsedChannels::RG, UsedChannels::RGB, UsedChannels::RGBA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UsedChannels >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("L", "USED_CHANNELS_L", UsedChannels::L), crate::meta::inspect::EnumConstant::new("LA", "USED_CHANNELS_LA", UsedChannels::LA), crate::meta::inspect::EnumConstant::new("R", "USED_CHANNELS_R", UsedChannels::R), crate::meta::inspect::EnumConstant::new("RG", "USED_CHANNELS_RG", UsedChannels::RG), crate::meta::inspect::EnumConstant::new("RGB", "USED_CHANNELS_RGB", UsedChannels::RGB), crate::meta::inspect::EnumConstant::new("RGBA", "USED_CHANNELS_RGBA", UsedChannels::RGBA)]
        }
    }
}
impl crate::meta::GodotConvert for UsedChannels {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Used Channels L", 0i64), EnumeratorShape::new_int("Used Channels La", 1i64), EnumeratorShape::new_int("Used Channels R", 2i64), EnumeratorShape::new_int("Used Channels Rg", 3i64), EnumeratorShape::new_int("Used Channels Rgb", 4i64), EnumeratorShape::new_int("Used Channels Rgba", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Image.UsedChannels")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for UsedChannels {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UsedChannels {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for UsedChannels {
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
impl crate::registry::property::Export for UsedChannels {
    
}
impl crate::meta::Element for UsedChannels {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompressSource {
    ord: i32
}
impl CompressSource {
    #[doc(alias = "COMPRESS_SOURCE_GENERIC")]
    #[doc = "Godot enumerator name: `COMPRESS_SOURCE_GENERIC`"]
    pub const GENERIC: CompressSource = CompressSource {
        ord: 0i32
    };
    #[doc(alias = "COMPRESS_SOURCE_SRGB")]
    #[doc = "Godot enumerator name: `COMPRESS_SOURCE_SRGB`"]
    pub const SRGB: CompressSource = CompressSource {
        ord: 1i32
    };
    #[doc(alias = "COMPRESS_SOURCE_NORMAL")]
    #[doc = "Godot enumerator name: `COMPRESS_SOURCE_NORMAL`"]
    pub const NORMAL: CompressSource = CompressSource {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CompressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompressSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompressSource {
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
            Self::GENERIC => "GENERIC", Self::SRGB => "SRGB", Self::NORMAL => "NORMAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompressSource::GENERIC, CompressSource::SRGB, CompressSource::NORMAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompressSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GENERIC", "COMPRESS_SOURCE_GENERIC", CompressSource::GENERIC), crate::meta::inspect::EnumConstant::new("SRGB", "COMPRESS_SOURCE_SRGB", CompressSource::SRGB), crate::meta::inspect::EnumConstant::new("NORMAL", "COMPRESS_SOURCE_NORMAL", CompressSource::NORMAL)]
        }
    }
}
impl crate::meta::GodotConvert for CompressSource {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Compress Source Generic", 0i64), EnumeratorShape::new_int("Compress Source Srgb", 1i64), EnumeratorShape::new_int("Compress Source Normal", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Image.CompressSource")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CompressSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompressSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CompressSource {
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
impl crate::registry::property::Export for CompressSource {
    
}
impl crate::meta::Element for CompressSource {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ASTCFormat`."]
pub struct AstcFormat {
    ord: i32
}
impl AstcFormat {
    #[doc(alias = "ASTC_FORMAT_4x4")]
    #[doc = "Godot enumerator name: `ASTC_FORMAT_4x4`"]
    pub const FORMAT_4x4: AstcFormat = AstcFormat {
        ord: 0i32
    };
    #[doc(alias = "ASTC_FORMAT_8x8")]
    #[doc = "Godot enumerator name: `ASTC_FORMAT_8x8`"]
    pub const FORMAT_8x8: AstcFormat = AstcFormat {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AstcFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AstcFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AstcFormat {
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
            Self::FORMAT_4x4 => "FORMAT_4x4", Self::FORMAT_8x8 => "FORMAT_8x8", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AstcFormat::FORMAT_4x4, AstcFormat::FORMAT_8x8]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AstcFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FORMAT_4x4", "ASTC_FORMAT_4x4", AstcFormat::FORMAT_4x4), crate::meta::inspect::EnumConstant::new("FORMAT_8x8", "ASTC_FORMAT_8x8", AstcFormat::FORMAT_8x8)]
        }
    }
}
impl crate::meta::GodotConvert for AstcFormat {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Astc Format 4x4", 0i64), EnumeratorShape::new_int("Astc Format 8x8", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Image.ASTCFormat")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AstcFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AstcFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AstcFormat {
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
impl crate::registry::property::Export for AstcFormat {
    
}
impl crate::meta::Element for AstcFormat {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Image;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Image {
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