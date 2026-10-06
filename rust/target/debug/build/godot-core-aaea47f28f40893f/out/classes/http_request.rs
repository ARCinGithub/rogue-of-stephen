#![doc = "Sidecar module for class [`HttpRequest`][crate::classes::HttpRequest].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `HTTPRequest` enums](https://docs.godotengine.org/en/stable/classes/class_httprequest.html#enumerations).\n\n"]
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
    #[doc = "Godot class `HTTPRequest`.\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`http_request`][crate::classes::http_request]: sidecar module with related enum/flag types\n* [`IHttpRequest`][crate::classes::IHttpRequest]: virtual methods\n* [`SignalsOfHttpRequest`][crate::classes::http_request::SignalsOfHttpRequest]: signal collection\n\n\nSee also [Godot docs for `HTTPRequest`](https://docs.godotengine.org/en/stable/classes/class_httprequest.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`HttpRequest::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nA node with the ability to send HTTP requests. Uses [`HTTPClient`][crate::classes::HttpClient] internally.\n\nCan be used to make HTTP requests, i.e. download or upload files or web content via HTTP.\n\n**Warning:** See the notes and warnings on [`HTTPClient`][crate::classes::HttpClient] for limitations, especially regarding TLS security.\n\n**Note:** When exporting to Android, make sure to enable the `INTERNET` permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android.\n\n**Example:** Contact a REST API and print one of its returned fields:\n\n\n```gdscript\nfunc _ready():\n\t# Create an HTTP request node and connect its completion signal.\n\tvar http_request = HTTPRequest.new()\n\tadd_child(http_request)\n\thttp_request.request_completed.connect(self._http_request_completed)\n\n\t# Perform a GET request. The URL below returns JSON as of writing.\n\tvar error = http_request.request(\"https://httpbin.org/get\")\n\tif error != OK:\n\t\tpush_error(\"An error occurred in the HTTP request.\")\n\n\t# Perform a POST request. The URL below returns JSON as of writing.\n\t# Note: Don't make simultaneous requests using a single HTTPRequest node.\n\t# The snippet below is provided for reference only.\n\tvar body = JSON.new().stringify({\"name\": \"Godette\"})\n\terror = http_request.request(\"https://httpbin.org/post\", [], HTTPClient.METHOD_POST, body)\n\tif error != OK:\n\t\tpush_error(\"An error occurred in the HTTP request.\")\n\n# Called when the HTTP request is completed.\nfunc _http_request_completed(result, response_code, headers, body):\n\tvar json = JSON.new()\n\tjson.parse(body.get_string_from_utf8())\n\tvar response = json.get_data()\n\n\t# Will print the user agent string used by the HTTPRequest node (as recognized by httpbin.org).\n\tprint(response.headers[\"User-Agent\"])\n```\n\n\n**Example:** Load an image using `HTTPRequest` and display it:\n\n\n```gdscript\nfunc _ready():\n\t# Create an HTTP request node and connect its completion signal.\n\tvar http_request = HTTPRequest.new()\n\tadd_child(http_request)\n\thttp_request.request_completed.connect(self._http_request_completed)\n\n\t# Perform the HTTP request. The URL below returns a PNG image as of writing.\n\tvar error = http_request.request(\"https://placehold.co/512.png\")\n\tif error != OK:\n\t\tpush_error(\"An error occurred in the HTTP request.\")\n\n# Called when the HTTP request is completed.\nfunc _http_request_completed(result, response_code, headers, body):\n\tif result != HTTPRequest.RESULT_SUCCESS:\n\t\tpush_error(\"Image couldn't be downloaded. Try a different image.\")\n\n\tvar image = Image.new()\n\tvar error = image.load_png_from_buffer(body)\n\tif error != OK:\n\t\tpush_error(\"Couldn't load the image.\")\n\n\tvar texture = ImageTexture.create_from_image(image)\n\n\t# Display the image in a TextureRect node.\n\tvar texture_rect = TextureRect.new()\n\tadd_child(texture_rect)\n\ttexture_rect.texture = texture\n```\n\n\n**Note:** `HTTPRequest` nodes will automatically handle decompression of response bodies. An `Accept-Encoding` header will be automatically added to each of your requests, unless one is already specified. Any response with a `Content-Encoding: gzip` header will automatically be decompressed and delivered to you as uncompressed bytes."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct HttpRequest {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`HttpRequest`][crate::classes::HttpRequest].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `HTTPRequest` methods](https://docs.godotengine.org/en/stable/classes/class_httprequest.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IHttpRequest: crate::obj::GodotClass < Base = HttpRequest > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Called when the node enters the [`SceneTree`][crate::classes::SceneTree] (e.g. upon instantiating, scene changing, or after calling [`add_child`][`crate::classes::Node::add_child`] in a script). If the node has children, its [`enter_tree`][`crate::classes::INode::enter_tree`] callback will be called first, and then that of the children.\n\nCorresponds to the [`NodeNotification::ENTER_TREE`][`crate::classes::notify::NodeNotification::ENTER_TREE`] notification in [`on_notification`][`crate::classes::IObject::on_notification`]."]
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the node is \"ready\", i.e. when both the node and its children have entered the scene tree. If the node has children, their [`ready`][`crate::classes::INode::ready`] callbacks get triggered first, and the parent node will receive the ready notification afterwards.\n\nCorresponds to the [`NodeNotification::READY`][`crate::classes::notify::NodeNotification::READY`] notification in [`on_notification`][`crate::classes::IObject::on_notification`]. See also the `@onready` annotation for variables.\n\nUsually used for initialization. For even earlier initialization, [`init`][`crate::classes::IObject::init`] may be used. See also [`enter_tree`][`crate::classes::INode::enter_tree`].\n\n**Note:** This method may be called only once for each node. After removing a node from the scene tree and adding it again, [`ready`][`crate::classes::INode::ready`] will **not** be called a second time. This can be bypassed by requesting another call with [`request_ready`][`crate::classes::Node::request_ready`], which may be called anywhere before adding the node again."]
        fn ready(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the node is about to leave the [`SceneTree`][crate::classes::SceneTree] (e.g. upon freeing, scene changing, or after calling [`remove_child`][`crate::classes::Node::remove_child`] in a script). If the node has children, its [`exit_tree`][`crate::classes::INode::exit_tree`] callback will be called last, after all its children have left the tree.\n\nCorresponds to the [`NodeNotification::EXIT_TREE`][`crate::classes::notify::NodeNotification::EXIT_TREE`] notification in [`on_notification`][`crate::classes::IObject::on_notification`] and signal `tree_exiting`. To get notified when the node has already left the active tree, connect to the `tree_exited`."]
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called on each idle frame, prior to rendering, and after physics ticks have been processed. `delta` is the time between frames in seconds.\n\nIt is only called if processing is enabled for this Node, which is done automatically if this method is overridden, and can be toggled with [`set_process`][`crate::classes::Node::set_process`].\n\nProcessing happens in order of \\[member process_priority], lower priority values are called first. Nodes with the same priority are processed in tree order, or top to bottom as seen in the editor (also known as pre-order traversal).\n\nCorresponds to the [`NodeNotification::PROCESS`][`crate::classes::notify::NodeNotification::PROCESS`] notification in [`on_notification`][`crate::classes::IObject::on_notification`].\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan).\n\n**Note:** When the engine is struggling and the frame rate is lowered, `delta` will increase. When `delta` is increased, it's capped at a maximum of \\[member Engine.time_scale] * \\[member Engine.max_physics_steps_per_frame] / \\[member Engine.physics_ticks_per_second]. As a result, accumulated `delta` may not represent real world time.\n\n**Note:** When `--fixed-fps` is enabled or the engine is running in Movie Maker mode (see [`MovieWriter`][crate::classes::MovieWriter]), process `delta` will always be the same for every frame, regardless of how much time the frame took to render.\n\n**Note:** Frame delta may be post-processed by \\[member OS.delta_smoothing] if this is enabled for the project."]
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        #[doc = "Called once on each physics tick, and allows Nodes to synchronize their logic with physics ticks. `delta` is the logical time between physics ticks in seconds and is equal to \\[member Engine.time_scale] / \\[member Engine.physics_ticks_per_second].\n\nIt is only called if physics processing is enabled for this Node, which is done automatically if this method is overridden, and can be toggled with [`set_physics_process`][`crate::classes::Node::set_physics_process`].\n\nProcessing happens in order of \\[member process_physics_priority], lower priority values are called first. Nodes with the same priority are processed in tree order, or top to bottom as seen in the editor (also known as pre-order traversal).\n\nCorresponds to the [`NodeNotification::PHYSICS_PROCESS`][`crate::classes::notify::NodeNotification::PHYSICS_PROCESS`] notification in [`on_notification`][`crate::classes::IObject::on_notification`].\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan).\n\n**Note:** Accumulated `delta` may diverge from real world seconds."]
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        #[doc = "Called when there is an input event. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if input processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_input`][`crate::classes::Node::set_process_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nFor gameplay input, [`unhandled_input`][`crate::classes::INode::unhandled_input`] and [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`] are usually a better fit as they allow the GUI to intercept the events first.\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan)."]
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = "Called when an [`InputEventKey`][crate::classes::InputEventKey], [`InputEventShortcut`][crate::classes::InputEventShortcut], or [`InputEventJoypadButton`][crate::classes::InputEventJoypadButton] hasn't been consumed by [`input`][`crate::classes::INode::input`] or any GUI [`Control`][crate::classes::Control] item. It is called before [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`] and [`unhandled_input`][`crate::classes::INode::unhandled_input`]. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if shortcut processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_shortcut_input`][`crate::classes::Node::set_process_shortcut_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nThis method can be used to handle shortcuts. For generic GUI events, use [`input`][`crate::classes::INode::input`] instead. Gameplay events should usually be handled with either [`unhandled_input`][`crate::classes::INode::unhandled_input`] or [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`].\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not orphan)."]
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = "Called when an [`InputEventKey`][crate::classes::InputEventKey] hasn't been consumed by [`input`][`crate::classes::INode::input`] or any GUI [`Control`][crate::classes::Control] item. It is called after [`shortcut_input`][`crate::classes::INode::shortcut_input`] but before [`unhandled_input`][`crate::classes::INode::unhandled_input`]. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if unhandled key input processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_unhandled_key_input`][`crate::classes::Node::set_process_unhandled_key_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nThis method can be used to handle Unicode character input with `Alt`, `Alt + Ctrl`, and `Alt + Shift` modifiers, after shortcuts were handled.\n\nFor gameplay input, this and [`unhandled_input`][`crate::classes::INode::unhandled_input`] are usually a better fit than [`input`][`crate::classes::INode::input`], as GUI events should be handled first. This method also performs better than [`unhandled_input`][`crate::classes::INode::unhandled_input`], since unrelated events such as [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] are automatically filtered. For shortcuts, consider using [`shortcut_input`][`crate::classes::INode::shortcut_input`] instead.\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan)."]
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = "Called when an [`InputEvent`][crate::classes::InputEvent] hasn't been consumed by [`input`][`crate::classes::INode::input`] or any GUI [`Control`][crate::classes::Control] item. It is called after [`shortcut_input`][`crate::classes::INode::shortcut_input`] and after [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`]. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if unhandled input processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_unhandled_input`][`crate::classes::Node::set_process_unhandled_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nFor gameplay input, this method is usually a better fit than [`input`][`crate::classes::INode::input`], as GUI events need a higher priority. For keyboard shortcuts, consider using [`shortcut_input`][`crate::classes::INode::shortcut_input`] instead, as it is called before this method. Finally, to handle keyboard events, consider using [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`] for performance reasons.\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan)."]
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
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
        fn on_notification(&mut self, what: NodeNotification) {
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
        #[doc = "The elements in the array returned from this method are displayed as warnings in the Scene dock if the script that overrides it is a `tool` script.\n\nReturning an empty array produces no warnings.\n\nCall [`update_configuration_warnings`][`crate::classes::Node::update_configuration_warnings`] when the warnings need to be updated for this node.\n\n```gdscript\n@export var energy = 0:\n\tset(value):\n\t\tenergy = value\n\t\tupdate_configuration_warnings()\n\nfunc _get_configuration_warnings():\n\tif energy < 0:\n\t\treturn [\"Energy must be 0 or greater.\"]\n\telse:\n\t\treturn []\n```"]
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "The elements in the array returned from this method are displayed as warnings in the Scene dock if the script that overrides it is a `tool` script, and accessibility warnings are enabled in the editor settings.\n\nReturning an empty array produces no warnings."]
        fn get_accessibility_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Called during accessibility information updates to determine the currently focused sub-element, should return a sub-element RID or the value returned by [`get_accessibility_element`][`crate::classes::Node::get_accessibility_element`]."]
        fn get_focused_accessibility_element(&self,) -> Rid {
            unimplemented !()
        }
    }
    impl HttpRequest {
        #[doc = "Creates request on the underlying [`HTTPClient`][crate::classes::HttpClient]. If there is no configuration errors, it tries to connect using [`connect_to_host`][`crate::classes::HttpClient::connect_to_host`] and passes parameters onto [`request`][`crate::classes::HttpClient::request`].\n\nReturns [`Error::OK`][`crate::global::Error::OK`] if request is successfully created. (Does not imply that the server has responded), [`Error::ERR_UNCONFIGURED`][`crate::global::Error::ERR_UNCONFIGURED`] if not in the tree, [`Error::ERR_BUSY`][`crate::global::Error::ERR_BUSY`] if still processing previous request, [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if given string is not a valid URL format, or [`Error::ERR_CANT_CONNECT`][`crate::global::Error::ERR_CANT_CONNECT`] if not using thread and the [`HTTPClient`][crate::classes::HttpClient] cannot connect to host.\n\n**Note:** When `method` is [`Method::GET`][`crate::classes::http_client::Method::GET`], the payload sent via `request_data` might be ignored by the server or even cause the server to reject the request (check [RFC 7231 section 4.3.1](https://datatracker.ietf.org/doc/html/rfc7231#section-4.3.1) for more details). As a workaround, you can send data as a query string in the URL (see [`uri_encode`][`crate::builtin::GString::uri_encode`] for an example).\n\n**Note:** It's recommended to use transport encryption (TLS) and to avoid sending sensitive information (such as login credentials) in HTTP GET URL parameters. Consider using HTTP POST requests or HTTP headers for such information instead."]
        pub(crate) fn request_full(&mut self, url: CowArg < GString >, custom_headers: RefArg < PackedStringArray >, method: crate::classes::http_client::Method, request_data: CowArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, crate::classes::http_client::Method, CowArg < 'a2, GString >,);
            let args = (url, custom_headers, method, request_data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "request", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`request_ex`][Self::request_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates request on the underlying [`HTTPClient`][crate::classes::HttpClient]. If there is no configuration errors, it tries to connect using [`connect_to_host`][`crate::classes::HttpClient::connect_to_host`] and passes parameters onto [`request`][`crate::classes::HttpClient::request`].\n\nReturns [`Error::OK`][`crate::global::Error::OK`] if request is successfully created. (Does not imply that the server has responded), [`Error::ERR_UNCONFIGURED`][`crate::global::Error::ERR_UNCONFIGURED`] if not in the tree, [`Error::ERR_BUSY`][`crate::global::Error::ERR_BUSY`] if still processing previous request, [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if given string is not a valid URL format, or [`Error::ERR_CANT_CONNECT`][`crate::global::Error::ERR_CANT_CONNECT`] if not using thread and the [`HTTPClient`][crate::classes::HttpClient] cannot connect to host.\n\n**Note:** When `method` is [`Method::GET`][`crate::classes::http_client::Method::GET`], the payload sent via `request_data` might be ignored by the server or even cause the server to reject the request (check [RFC 7231 section 4.3.1](https://datatracker.ietf.org/doc/html/rfc7231#section-4.3.1) for more details). As a workaround, you can send data as a query string in the URL (see [`uri_encode`][`crate::builtin::GString::uri_encode`] for an example).\n\n**Note:** It's recommended to use transport encryption (TLS) and to avoid sending sensitive information (such as login credentials) in HTTP GET URL parameters. Consider using HTTP POST requests or HTTP headers for such information instead."]
        #[inline]
        pub fn request(&mut self, url: impl AsArg < GString >,) -> crate::global::Error {
            self.request_ex(url,) . done()
        }
        #[doc = "Creates request on the underlying [`HTTPClient`][crate::classes::HttpClient]. If there is no configuration errors, it tries to connect using [`connect_to_host`][`crate::classes::HttpClient::connect_to_host`] and passes parameters onto [`request`][`crate::classes::HttpClient::request`].\n\nReturns [`Error::OK`][`crate::global::Error::OK`] if request is successfully created. (Does not imply that the server has responded), [`Error::ERR_UNCONFIGURED`][`crate::global::Error::ERR_UNCONFIGURED`] if not in the tree, [`Error::ERR_BUSY`][`crate::global::Error::ERR_BUSY`] if still processing previous request, [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if given string is not a valid URL format, or [`Error::ERR_CANT_CONNECT`][`crate::global::Error::ERR_CANT_CONNECT`] if not using thread and the [`HTTPClient`][crate::classes::HttpClient] cannot connect to host.\n\n**Note:** When `method` is [`Method::GET`][`crate::classes::http_client::Method::GET`], the payload sent via `request_data` might be ignored by the server or even cause the server to reject the request (check [RFC 7231 section 4.3.1](https://datatracker.ietf.org/doc/html/rfc7231#section-4.3.1) for more details). As a workaround, you can send data as a query string in the URL (see [`uri_encode`][`crate::builtin::GString::uri_encode`] for an example).\n\n**Note:** It's recommended to use transport encryption (TLS) and to avoid sending sensitive information (such as login credentials) in HTTP GET URL parameters. Consider using HTTP POST requests or HTTP headers for such information instead."]
        #[inline]
        pub fn request_ex < 'ex > (&'ex mut self, url: impl AsArg < GString > + 'ex,) -> ExRequest < 'ex > {
            ExRequest::new(self, url,)
        }
        #[doc = "Creates request on the underlying [`HTTPClient`][crate::classes::HttpClient] using a raw array of bytes for the request body. If there is no configuration errors, it tries to connect using [`connect_to_host`][`crate::classes::HttpClient::connect_to_host`] and passes parameters onto [`request`][`crate::classes::HttpClient::request`].\n\nReturns [`Error::OK`][`crate::global::Error::OK`] if request is successfully created. (Does not imply that the server has responded), [`Error::ERR_UNCONFIGURED`][`crate::global::Error::ERR_UNCONFIGURED`] if not in the tree, [`Error::ERR_BUSY`][`crate::global::Error::ERR_BUSY`] if still processing previous request, [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if given string is not a valid URL format, or [`Error::ERR_CANT_CONNECT`][`crate::global::Error::ERR_CANT_CONNECT`] if not using thread and the [`HTTPClient`][crate::classes::HttpClient] cannot connect to host."]
        pub(crate) fn request_raw_full(&mut self, url: CowArg < GString >, custom_headers: RefArg < PackedStringArray >, method: crate::classes::http_client::Method, request_data_raw: RefArg < PackedByteArray >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, crate::classes::http_client::Method, RefArg < 'a2, PackedByteArray >,);
            let args = (url, custom_headers, method, request_data_raw,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "request_raw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`request_raw_ex`][Self::request_raw_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates request on the underlying [`HTTPClient`][crate::classes::HttpClient] using a raw array of bytes for the request body. If there is no configuration errors, it tries to connect using [`connect_to_host`][`crate::classes::HttpClient::connect_to_host`] and passes parameters onto [`request`][`crate::classes::HttpClient::request`].\n\nReturns [`Error::OK`][`crate::global::Error::OK`] if request is successfully created. (Does not imply that the server has responded), [`Error::ERR_UNCONFIGURED`][`crate::global::Error::ERR_UNCONFIGURED`] if not in the tree, [`Error::ERR_BUSY`][`crate::global::Error::ERR_BUSY`] if still processing previous request, [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if given string is not a valid URL format, or [`Error::ERR_CANT_CONNECT`][`crate::global::Error::ERR_CANT_CONNECT`] if not using thread and the [`HTTPClient`][crate::classes::HttpClient] cannot connect to host."]
        #[inline]
        pub fn request_raw(&mut self, url: impl AsArg < GString >,) -> crate::global::Error {
            self.request_raw_ex(url,) . done()
        }
        #[doc = "Creates request on the underlying [`HTTPClient`][crate::classes::HttpClient] using a raw array of bytes for the request body. If there is no configuration errors, it tries to connect using [`connect_to_host`][`crate::classes::HttpClient::connect_to_host`] and passes parameters onto [`request`][`crate::classes::HttpClient::request`].\n\nReturns [`Error::OK`][`crate::global::Error::OK`] if request is successfully created. (Does not imply that the server has responded), [`Error::ERR_UNCONFIGURED`][`crate::global::Error::ERR_UNCONFIGURED`] if not in the tree, [`Error::ERR_BUSY`][`crate::global::Error::ERR_BUSY`] if still processing previous request, [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if given string is not a valid URL format, or [`Error::ERR_CANT_CONNECT`][`crate::global::Error::ERR_CANT_CONNECT`] if not using thread and the [`HTTPClient`][crate::classes::HttpClient] cannot connect to host."]
        #[inline]
        pub fn request_raw_ex < 'ex > (&'ex mut self, url: impl AsArg < GString > + 'ex,) -> ExRequestRaw < 'ex > {
            ExRequestRaw::new(self, url,)
        }
        #[doc = "Cancels the current request."]
        pub fn cancel_request(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "cancel_request", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`TLSOptions`][crate::classes::TlsOptions] to be used when connecting to an HTTPS server. See [`client`][`crate::classes::TlsOptions::client`]."]
        pub fn set_tls_options(&mut self, client_options: impl AsArg < Option < Gd < crate::classes::TlsOptions >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TlsOptions > > >,);
            let args = (client_options.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "set_tls_options", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current status of the underlying [`HTTPClient`][crate::classes::HttpClient]."]
        pub fn get_http_client_status(&self,) -> crate::classes::http_client::Status {
            type CallRet = crate::classes::http_client::Status;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "get_http_client_status", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_threads(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "set_use_threads", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_threads(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "is_using_threads", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accept_gzip(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "set_accept_gzip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_accepting_gzip(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "is_accepting_gzip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_body_size_limit(&mut self, bytes: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "set_body_size_limit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_body_size_limit(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "get_body_size_limit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_redirects(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "set_max_redirects", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_redirects(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "get_max_redirects", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_download_file(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "set_download_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_download_file(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "get_download_file", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of bytes this HTTPRequest downloaded."]
        pub fn get_downloaded_bytes(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "get_downloaded_bytes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the response body length.\n\n**Note:** Some Web servers may not send a body length. In this case, the value returned will be `-1`. If using chunked transfer encoding, the body length will also be `-1`."]
        pub fn get_body_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "get_body_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_timeout(&mut self, timeout: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (timeout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "set_timeout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_timeout(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "get_timeout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_download_chunk_size(&mut self, chunk_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (chunk_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "set_download_chunk_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_download_chunk_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "get_download_chunk_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the proxy server for HTTP requests.\n\nThe proxy server is unset if `host` is empty or `port` is -1."]
        pub fn set_http_proxy(&mut self, host: impl AsArg < GString >, port: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (host.into_arg(), port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "set_http_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the proxy server for HTTPS requests.\n\nThe proxy server is unset if `host` is empty or `port` is -1."]
        pub fn set_https_proxy(&mut self, host: impl AsArg < GString >, port: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (host.into_arg(), port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HttpRequest", "set_https_proxy", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for HttpRequest {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("HTTPRequest"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for HttpRequest {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for HttpRequest {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for HttpRequest {
        
    }
    impl crate::obj::cap::GodotDefault for HttpRequest {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for HttpRequest {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for HttpRequest {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`HttpRequest`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_HttpRequest__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::HttpRequest > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`HttpRequest::request_ex`][super::HttpRequest::request_ex]."]
#[must_use]
pub struct ExRequest < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::HttpRequest, url: CowArg < 'ex, GString >, custom_headers: CowArg < 'ex, PackedStringArray >, method: crate::classes::http_client::Method, request_data: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExRequest < 'ex > {
    fn new(surround_object: &'ex mut re_export::HttpRequest, url: impl AsArg < GString > + 'ex,) -> Self {
        let custom_headers = PackedStringArray::new();
        let method = crate::obj::EngineEnum::from_ord(0);
        let request_data = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, url: url.into_arg(), custom_headers: CowArg::Owned(custom_headers), method: method, request_data: CowArg::Owned(request_data),
        }
    }
    #[inline]
    pub fn custom_headers(self, custom_headers: &'ex PackedStringArray) -> Self {
        Self {
            custom_headers: CowArg::Borrowed(custom_headers), .. self
        }
    }
    #[inline]
    pub fn method(self, method: crate::classes::http_client::Method) -> Self {
        Self {
            method: method, .. self
        }
    }
    #[inline]
    pub fn request_data(self, request_data: impl AsArg < GString > + 'ex) -> Self {
        Self {
            request_data: request_data.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, url, custom_headers, method, request_data,
        }
        = self;
        re_export::HttpRequest::request_full(surround_object, url, custom_headers.cow_as_arg(), method, request_data,)
    }
}
#[doc = "Default-param extender for [`HttpRequest::request_raw_ex`][super::HttpRequest::request_raw_ex]."]
#[must_use]
pub struct ExRequestRaw < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::HttpRequest, url: CowArg < 'ex, GString >, custom_headers: CowArg < 'ex, PackedStringArray >, method: crate::classes::http_client::Method, request_data_raw: CowArg < 'ex, PackedByteArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExRequestRaw < 'ex > {
    fn new(surround_object: &'ex mut re_export::HttpRequest, url: impl AsArg < GString > + 'ex,) -> Self {
        let custom_headers = PackedStringArray::new();
        let method = crate::obj::EngineEnum::from_ord(0);
        let request_data_raw = PackedByteArray::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, url: url.into_arg(), custom_headers: CowArg::Owned(custom_headers), method: method, request_data_raw: CowArg::Owned(request_data_raw),
        }
    }
    #[inline]
    pub fn custom_headers(self, custom_headers: &'ex PackedStringArray) -> Self {
        Self {
            custom_headers: CowArg::Borrowed(custom_headers), .. self
        }
    }
    #[inline]
    pub fn method(self, method: crate::classes::http_client::Method) -> Self {
        Self {
            method: method, .. self
        }
    }
    #[inline]
    pub fn request_data_raw(self, request_data_raw: &'ex PackedByteArray) -> Self {
        Self {
            request_data_raw: CowArg::Borrowed(request_data_raw), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, url, custom_headers, method, request_data_raw,
        }
        = self;
        re_export::HttpRequest::request_raw_full(surround_object, url, custom_headers.cow_as_arg(), method, request_data_raw.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Result {
    ord: i32
}
impl Result {
    #[doc(alias = "RESULT_SUCCESS")]
    #[doc = "Godot enumerator name: `RESULT_SUCCESS`"]
    pub const SUCCESS: Result = Result {
        ord: 0i32
    };
    #[doc(alias = "RESULT_CHUNKED_BODY_SIZE_MISMATCH")]
    #[doc = "Godot enumerator name: `RESULT_CHUNKED_BODY_SIZE_MISMATCH`"]
    pub const CHUNKED_BODY_SIZE_MISMATCH: Result = Result {
        ord: 1i32
    };
    #[doc(alias = "RESULT_CANT_CONNECT")]
    #[doc = "Godot enumerator name: `RESULT_CANT_CONNECT`"]
    pub const CANT_CONNECT: Result = Result {
        ord: 2i32
    };
    #[doc(alias = "RESULT_CANT_RESOLVE")]
    #[doc = "Godot enumerator name: `RESULT_CANT_RESOLVE`"]
    pub const CANT_RESOLVE: Result = Result {
        ord: 3i32
    };
    #[doc(alias = "RESULT_CONNECTION_ERROR")]
    #[doc = "Godot enumerator name: `RESULT_CONNECTION_ERROR`"]
    pub const CONNECTION_ERROR: Result = Result {
        ord: 4i32
    };
    #[doc(alias = "RESULT_TLS_HANDSHAKE_ERROR")]
    #[doc = "Godot enumerator name: `RESULT_TLS_HANDSHAKE_ERROR`"]
    pub const TLS_HANDSHAKE_ERROR: Result = Result {
        ord: 5i32
    };
    #[doc(alias = "RESULT_NO_RESPONSE")]
    #[doc = "Godot enumerator name: `RESULT_NO_RESPONSE`"]
    pub const NO_RESPONSE: Result = Result {
        ord: 6i32
    };
    #[doc(alias = "RESULT_BODY_SIZE_LIMIT_EXCEEDED")]
    #[doc = "Godot enumerator name: `RESULT_BODY_SIZE_LIMIT_EXCEEDED`"]
    pub const BODY_SIZE_LIMIT_EXCEEDED: Result = Result {
        ord: 7i32
    };
    #[doc(alias = "RESULT_BODY_DECOMPRESS_FAILED")]
    #[doc = "Godot enumerator name: `RESULT_BODY_DECOMPRESS_FAILED`"]
    pub const BODY_DECOMPRESS_FAILED: Result = Result {
        ord: 8i32
    };
    #[doc(alias = "RESULT_REQUEST_FAILED")]
    #[doc = "Godot enumerator name: `RESULT_REQUEST_FAILED`"]
    pub const REQUEST_FAILED: Result = Result {
        ord: 9i32
    };
    #[doc(alias = "RESULT_DOWNLOAD_FILE_CANT_OPEN")]
    #[doc = "Godot enumerator name: `RESULT_DOWNLOAD_FILE_CANT_OPEN`"]
    pub const DOWNLOAD_FILE_CANT_OPEN: Result = Result {
        ord: 10i32
    };
    #[doc(alias = "RESULT_DOWNLOAD_FILE_WRITE_ERROR")]
    #[doc = "Godot enumerator name: `RESULT_DOWNLOAD_FILE_WRITE_ERROR`"]
    pub const DOWNLOAD_FILE_WRITE_ERROR: Result = Result {
        ord: 11i32
    };
    #[doc(alias = "RESULT_REDIRECT_LIMIT_REACHED")]
    #[doc = "Godot enumerator name: `RESULT_REDIRECT_LIMIT_REACHED`"]
    pub const REDIRECT_LIMIT_REACHED: Result = Result {
        ord: 12i32
    };
    #[doc(alias = "RESULT_TIMEOUT")]
    #[doc = "Godot enumerator name: `RESULT_TIMEOUT`"]
    pub const TIMEOUT: Result = Result {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Result") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Result {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::SUCCESS => "SUCCESS", Self::CHUNKED_BODY_SIZE_MISMATCH => "CHUNKED_BODY_SIZE_MISMATCH", Self::CANT_CONNECT => "CANT_CONNECT", Self::CANT_RESOLVE => "CANT_RESOLVE", Self::CONNECTION_ERROR => "CONNECTION_ERROR", Self::TLS_HANDSHAKE_ERROR => "TLS_HANDSHAKE_ERROR", Self::NO_RESPONSE => "NO_RESPONSE", Self::BODY_SIZE_LIMIT_EXCEEDED => "BODY_SIZE_LIMIT_EXCEEDED", Self::BODY_DECOMPRESS_FAILED => "BODY_DECOMPRESS_FAILED", Self::REQUEST_FAILED => "REQUEST_FAILED", Self::DOWNLOAD_FILE_CANT_OPEN => "DOWNLOAD_FILE_CANT_OPEN", Self::DOWNLOAD_FILE_WRITE_ERROR => "DOWNLOAD_FILE_WRITE_ERROR", Self::REDIRECT_LIMIT_REACHED => "REDIRECT_LIMIT_REACHED", Self::TIMEOUT => "TIMEOUT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Result::SUCCESS, Result::CHUNKED_BODY_SIZE_MISMATCH, Result::CANT_CONNECT, Result::CANT_RESOLVE, Result::CONNECTION_ERROR, Result::TLS_HANDSHAKE_ERROR, Result::NO_RESPONSE, Result::BODY_SIZE_LIMIT_EXCEEDED, Result::BODY_DECOMPRESS_FAILED, Result::REQUEST_FAILED, Result::DOWNLOAD_FILE_CANT_OPEN, Result::DOWNLOAD_FILE_WRITE_ERROR, Result::REDIRECT_LIMIT_REACHED, Result::TIMEOUT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Result >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SUCCESS", "RESULT_SUCCESS", Result::SUCCESS), crate::meta::inspect::EnumConstant::new("CHUNKED_BODY_SIZE_MISMATCH", "RESULT_CHUNKED_BODY_SIZE_MISMATCH", Result::CHUNKED_BODY_SIZE_MISMATCH), crate::meta::inspect::EnumConstant::new("CANT_CONNECT", "RESULT_CANT_CONNECT", Result::CANT_CONNECT), crate::meta::inspect::EnumConstant::new("CANT_RESOLVE", "RESULT_CANT_RESOLVE", Result::CANT_RESOLVE), crate::meta::inspect::EnumConstant::new("CONNECTION_ERROR", "RESULT_CONNECTION_ERROR", Result::CONNECTION_ERROR), crate::meta::inspect::EnumConstant::new("TLS_HANDSHAKE_ERROR", "RESULT_TLS_HANDSHAKE_ERROR", Result::TLS_HANDSHAKE_ERROR), crate::meta::inspect::EnumConstant::new("NO_RESPONSE", "RESULT_NO_RESPONSE", Result::NO_RESPONSE), crate::meta::inspect::EnumConstant::new("BODY_SIZE_LIMIT_EXCEEDED", "RESULT_BODY_SIZE_LIMIT_EXCEEDED", Result::BODY_SIZE_LIMIT_EXCEEDED), crate::meta::inspect::EnumConstant::new("BODY_DECOMPRESS_FAILED", "RESULT_BODY_DECOMPRESS_FAILED", Result::BODY_DECOMPRESS_FAILED), crate::meta::inspect::EnumConstant::new("REQUEST_FAILED", "RESULT_REQUEST_FAILED", Result::REQUEST_FAILED), crate::meta::inspect::EnumConstant::new("DOWNLOAD_FILE_CANT_OPEN", "RESULT_DOWNLOAD_FILE_CANT_OPEN", Result::DOWNLOAD_FILE_CANT_OPEN), crate::meta::inspect::EnumConstant::new("DOWNLOAD_FILE_WRITE_ERROR", "RESULT_DOWNLOAD_FILE_WRITE_ERROR", Result::DOWNLOAD_FILE_WRITE_ERROR), crate::meta::inspect::EnumConstant::new("REDIRECT_LIMIT_REACHED", "RESULT_REDIRECT_LIMIT_REACHED", Result::REDIRECT_LIMIT_REACHED), crate::meta::inspect::EnumConstant::new("TIMEOUT", "RESULT_TIMEOUT", Result::TIMEOUT)]
        }
    }
}
impl crate::meta::GodotConvert for Result {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Result Success", 0i64), EnumeratorShape::new_int("Result Chunked Body Size Mismatch", 1i64), EnumeratorShape::new_int("Result Cant Connect", 2i64), EnumeratorShape::new_int("Result Cant Resolve", 3i64), EnumeratorShape::new_int("Result Connection Error", 4i64), EnumeratorShape::new_int("Result Tls Handshake Error", 5i64), EnumeratorShape::new_int("Result No Response", 6i64), EnumeratorShape::new_int("Result Body Size Limit Exceeded", 7i64), EnumeratorShape::new_int("Result Body Decompress Failed", 8i64), EnumeratorShape::new_int("Result Request Failed", 9i64), EnumeratorShape::new_int("Result Download File Cant Open", 10i64), EnumeratorShape::new_int("Result Download File Write Error", 11i64), EnumeratorShape::new_int("Result Redirect Limit Reached", 12i64), EnumeratorShape::new_int("Result Timeout", 13i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("HTTPRequest.Result")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Result {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Result {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Result {
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
impl crate::registry::property::Export for Result {
    
}
impl crate::meta::Element for Result {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::HttpRequest;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`HttpRequest`][crate::classes::HttpRequest] class."]
    pub struct SignalsOfHttpRequest < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfHttpRequest < 'c, C > {
        #[doc = "Signature: `(result: i64, response_code: i64, headers: PackedStringArray, body: PackedByteArray)`"]
        pub fn request_completed(&mut self) -> SigRequestCompleted < 'c, C > {
            SigRequestCompleted {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "request_completed")
            }
        }
    }
    type TypedSigRequestCompleted < 'c, C > = TypedSignal < 'c, C, (i64, i64, PackedStringArray, PackedByteArray,) >;
    pub struct SigRequestCompleted < 'c, C: WithSignals > {
        typed: TypedSigRequestCompleted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRequestCompleted < 'c, C > {
        pub fn emit(&mut self, result: i64, response_code: i64, headers: PackedStringArray, body: PackedByteArray,) {
            self.typed.emit_tuple((result, response_code, headers, body,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRequestCompleted < 'c, C > {
        type Target = TypedSigRequestCompleted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRequestCompleted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for HttpRequest {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfHttpRequest < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfHttpRequest < 'c, C > {
        type Target = < < HttpRequest as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = HttpRequest;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfHttpRequest < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = HttpRequest;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}