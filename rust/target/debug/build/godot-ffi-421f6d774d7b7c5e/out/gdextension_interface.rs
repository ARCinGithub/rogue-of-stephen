#[doc = r" UTF-16 character type."]
pub type char16_t = u16;
#[doc = r" UTF-32 character type."]
pub type char32_t = u32;
#[doc = r" Wide character type."]
pub type wchar_t = std::ffi::c_int;
pub type GDExtensionVariantType = std::ffi::c_int;
pub const GDEXTENSION_VARIANT_TYPE_NIL: GDExtensionVariantType = 0;
pub const GDEXTENSION_VARIANT_TYPE_BOOL: GDExtensionVariantType = 1;
pub const GDEXTENSION_VARIANT_TYPE_INT: GDExtensionVariantType = 2;
pub const GDEXTENSION_VARIANT_TYPE_FLOAT: GDExtensionVariantType = 3;
pub const GDEXTENSION_VARIANT_TYPE_STRING: GDExtensionVariantType = 4;
pub const GDEXTENSION_VARIANT_TYPE_VECTOR2: GDExtensionVariantType = 5;
pub const GDEXTENSION_VARIANT_TYPE_VECTOR2I: GDExtensionVariantType = 6;
pub const GDEXTENSION_VARIANT_TYPE_RECT2: GDExtensionVariantType = 7;
pub const GDEXTENSION_VARIANT_TYPE_RECT2I: GDExtensionVariantType = 8;
pub const GDEXTENSION_VARIANT_TYPE_VECTOR3: GDExtensionVariantType = 9;
pub const GDEXTENSION_VARIANT_TYPE_VECTOR3I: GDExtensionVariantType = 10;
pub const GDEXTENSION_VARIANT_TYPE_TRANSFORM2D: GDExtensionVariantType = 11;
pub const GDEXTENSION_VARIANT_TYPE_VECTOR4: GDExtensionVariantType = 12;
pub const GDEXTENSION_VARIANT_TYPE_VECTOR4I: GDExtensionVariantType = 13;
pub const GDEXTENSION_VARIANT_TYPE_PLANE: GDExtensionVariantType = 14;
pub const GDEXTENSION_VARIANT_TYPE_QUATERNION: GDExtensionVariantType = 15;
pub const GDEXTENSION_VARIANT_TYPE_AABB: GDExtensionVariantType = 16;
pub const GDEXTENSION_VARIANT_TYPE_BASIS: GDExtensionVariantType = 17;
pub const GDEXTENSION_VARIANT_TYPE_TRANSFORM3D: GDExtensionVariantType = 18;
pub const GDEXTENSION_VARIANT_TYPE_PROJECTION: GDExtensionVariantType = 19;
pub const GDEXTENSION_VARIANT_TYPE_COLOR: GDExtensionVariantType = 20;
pub const GDEXTENSION_VARIANT_TYPE_STRING_NAME: GDExtensionVariantType = 21;
pub const GDEXTENSION_VARIANT_TYPE_NODE_PATH: GDExtensionVariantType = 22;
pub const GDEXTENSION_VARIANT_TYPE_RID: GDExtensionVariantType = 23;
pub const GDEXTENSION_VARIANT_TYPE_OBJECT: GDExtensionVariantType = 24;
pub const GDEXTENSION_VARIANT_TYPE_CALLABLE: GDExtensionVariantType = 25;
pub const GDEXTENSION_VARIANT_TYPE_SIGNAL: GDExtensionVariantType = 26;
pub const GDEXTENSION_VARIANT_TYPE_DICTIONARY: GDExtensionVariantType = 27;
pub const GDEXTENSION_VARIANT_TYPE_ARRAY: GDExtensionVariantType = 28;
pub const GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY: GDExtensionVariantType = 29;
pub const GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY: GDExtensionVariantType = 30;
pub const GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY: GDExtensionVariantType = 31;
pub const GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY: GDExtensionVariantType = 32;
pub const GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY: GDExtensionVariantType = 33;
pub const GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY: GDExtensionVariantType = 34;
pub const GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY: GDExtensionVariantType = 35;
pub const GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY: GDExtensionVariantType = 36;
pub const GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY: GDExtensionVariantType = 37;
pub const GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR4_ARRAY: GDExtensionVariantType = 38;
pub const GDEXTENSION_VARIANT_TYPE_VARIANT_MAX: GDExtensionVariantType = 39;
pub type GDExtensionVariantOperator = std::ffi::c_int;
pub const GDEXTENSION_VARIANT_OP_EQUAL: GDExtensionVariantOperator = 0;
pub const GDEXTENSION_VARIANT_OP_NOT_EQUAL: GDExtensionVariantOperator = 1;
pub const GDEXTENSION_VARIANT_OP_LESS: GDExtensionVariantOperator = 2;
pub const GDEXTENSION_VARIANT_OP_LESS_EQUAL: GDExtensionVariantOperator = 3;
pub const GDEXTENSION_VARIANT_OP_GREATER: GDExtensionVariantOperator = 4;
pub const GDEXTENSION_VARIANT_OP_GREATER_EQUAL: GDExtensionVariantOperator = 5;
pub const GDEXTENSION_VARIANT_OP_ADD: GDExtensionVariantOperator = 6;
pub const GDEXTENSION_VARIANT_OP_SUBTRACT: GDExtensionVariantOperator = 7;
pub const GDEXTENSION_VARIANT_OP_MULTIPLY: GDExtensionVariantOperator = 8;
pub const GDEXTENSION_VARIANT_OP_DIVIDE: GDExtensionVariantOperator = 9;
pub const GDEXTENSION_VARIANT_OP_NEGATE: GDExtensionVariantOperator = 10;
pub const GDEXTENSION_VARIANT_OP_POSITIVE: GDExtensionVariantOperator = 11;
pub const GDEXTENSION_VARIANT_OP_MODULE: GDExtensionVariantOperator = 12;
pub const GDEXTENSION_VARIANT_OP_POWER: GDExtensionVariantOperator = 13;
pub const GDEXTENSION_VARIANT_OP_SHIFT_LEFT: GDExtensionVariantOperator = 14;
pub const GDEXTENSION_VARIANT_OP_SHIFT_RIGHT: GDExtensionVariantOperator = 15;
pub const GDEXTENSION_VARIANT_OP_BIT_AND: GDExtensionVariantOperator = 16;
pub const GDEXTENSION_VARIANT_OP_BIT_OR: GDExtensionVariantOperator = 17;
pub const GDEXTENSION_VARIANT_OP_BIT_XOR: GDExtensionVariantOperator = 18;
pub const GDEXTENSION_VARIANT_OP_BIT_NEGATE: GDExtensionVariantOperator = 19;
pub const GDEXTENSION_VARIANT_OP_AND: GDExtensionVariantOperator = 20;
pub const GDEXTENSION_VARIANT_OP_OR: GDExtensionVariantOperator = 21;
pub const GDEXTENSION_VARIANT_OP_XOR: GDExtensionVariantOperator = 22;
pub const GDEXTENSION_VARIANT_OP_NOT: GDExtensionVariantOperator = 23;
pub const GDEXTENSION_VARIANT_OP_IN: GDExtensionVariantOperator = 24;
pub const GDEXTENSION_VARIANT_OP_MAX: GDExtensionVariantOperator = 25;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextVariant {
    _unused: [u8;
    0],
}
pub type GDExtensionVariantPtr = * mut __GdextVariant;
pub type GDExtensionConstVariantPtr = * const __GdextVariant;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextUninitializedVariant {
    _unused: [u8;
    0],
}
pub type GDExtensionUninitializedVariantPtr = * mut __GdextUninitializedVariant;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextStringName {
    _unused: [u8;
    0],
}
pub type GDExtensionStringNamePtr = * mut __GdextStringName;
pub type GDExtensionConstStringNamePtr = * const __GdextStringName;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextUninitializedStringName {
    _unused: [u8;
    0],
}
pub type GDExtensionUninitializedStringNamePtr = * mut __GdextUninitializedStringName;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextString {
    _unused: [u8;
    0],
}
pub type GDExtensionStringPtr = * mut __GdextString;
pub type GDExtensionConstStringPtr = * const __GdextString;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextUninitializedString {
    _unused: [u8;
    0],
}
pub type GDExtensionUninitializedStringPtr = * mut __GdextUninitializedString;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextObject {
    _unused: [u8;
    0],
}
pub type GDExtensionObjectPtr = * mut __GdextObject;
pub type GDExtensionConstObjectPtr = * const __GdextObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextUninitializedObject {
    _unused: [u8;
    0],
}
pub type GDExtensionUninitializedObjectPtr = * mut __GdextUninitializedObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextType {
    _unused: [u8;
    0],
}
pub type GDExtensionTypePtr = * mut __GdextType;
pub type GDExtensionConstTypePtr = * const __GdextType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextUninitializedType {
    _unused: [u8;
    0],
}
pub type GDExtensionUninitializedTypePtr = * mut __GdextUninitializedType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextMethodBind {
    _unused: [u8;
    0],
}
pub type GDExtensionMethodBindPtr = * const __GdextMethodBind;
pub type GDExtensionInt = i64;
pub type GDExtensionBool = u8;
pub type GDObjectInstanceID = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextRef {
    _unused: [u8;
    0],
}
pub type GDExtensionRefPtr = * mut __GdextRef;
pub type GDExtensionConstRefPtr = * const __GdextRef;
pub type GDExtensionCallErrorType = std::ffi::c_int;
pub const GDEXTENSION_CALL_OK: GDExtensionCallErrorType = 0;
pub const GDEXTENSION_CALL_ERROR_INVALID_METHOD: GDExtensionCallErrorType = 1;
pub const GDEXTENSION_CALL_ERROR_INVALID_ARGUMENT: GDExtensionCallErrorType = 2;
pub const GDEXTENSION_CALL_ERROR_TOO_MANY_ARGUMENTS: GDExtensionCallErrorType = 3;
pub const GDEXTENSION_CALL_ERROR_TOO_FEW_ARGUMENTS: GDExtensionCallErrorType = 4;
pub const GDEXTENSION_CALL_ERROR_INSTANCE_IS_NULL: GDExtensionCallErrorType = 5;
pub const GDEXTENSION_CALL_ERROR_METHOD_NOT_CONST: GDExtensionCallErrorType = 6;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionCallError {
    pub error: GDExtensionCallErrorType, pub argument: i32, pub expected: i32,
}
pub type GDExtensionVariantFromTypeConstructorFunc = Option < unsafe extern "C" fn(GDExtensionUninitializedVariantPtr, GDExtensionTypePtr) >;
pub type GDExtensionTypeFromVariantConstructorFunc = Option < unsafe extern "C" fn(GDExtensionUninitializedTypePtr, GDExtensionVariantPtr) >;
pub type GDExtensionVariantGetInternalPtrFunc = Option < unsafe extern "C" fn(GDExtensionVariantPtr) -> * mut std::ffi::c_void >;
pub type GDExtensionPtrOperatorEvaluator = Option < unsafe extern "C" fn(p_left: GDExtensionConstTypePtr, p_right: GDExtensionConstTypePtr, r_result: GDExtensionTypePtr) >;
pub type GDExtensionPtrBuiltInMethod = Option < unsafe extern "C" fn(p_base: GDExtensionTypePtr, p_args: * const GDExtensionConstTypePtr, r_return: GDExtensionTypePtr, p_argument_count: i32) >;
pub type GDExtensionPtrConstructor = Option < unsafe extern "C" fn(p_base: GDExtensionUninitializedTypePtr, p_args: * const GDExtensionConstTypePtr) >;
pub type GDExtensionPtrDestructor = Option < unsafe extern "C" fn(p_base: GDExtensionTypePtr) >;
pub type GDExtensionPtrSetter = Option < unsafe extern "C" fn(p_base: GDExtensionTypePtr, p_value: GDExtensionConstTypePtr) >;
pub type GDExtensionPtrGetter = Option < unsafe extern "C" fn(p_base: GDExtensionConstTypePtr, r_value: GDExtensionTypePtr) >;
pub type GDExtensionPtrIndexedSetter = Option < unsafe extern "C" fn(p_base: GDExtensionTypePtr, p_index: GDExtensionInt, p_value: GDExtensionConstTypePtr) >;
pub type GDExtensionPtrIndexedGetter = Option < unsafe extern "C" fn(p_base: GDExtensionConstTypePtr, p_index: GDExtensionInt, r_value: GDExtensionTypePtr) >;
pub type GDExtensionPtrKeyedSetter = Option < unsafe extern "C" fn(p_base: GDExtensionTypePtr, p_key: GDExtensionConstTypePtr, p_value: GDExtensionConstTypePtr) >;
pub type GDExtensionPtrKeyedGetter = Option < unsafe extern "C" fn(p_base: GDExtensionConstTypePtr, p_key: GDExtensionConstTypePtr, r_value: GDExtensionTypePtr) >;
pub type GDExtensionPtrKeyedChecker = Option < unsafe extern "C" fn(p_base: GDExtensionConstVariantPtr, p_key: GDExtensionConstVariantPtr) -> u32 >;
pub type GDExtensionPtrUtilityFunction = Option < unsafe extern "C" fn(r_return: GDExtensionTypePtr, p_args: * const GDExtensionConstTypePtr, p_argument_count: i32) >;
pub type GDExtensionClassConstructor = Option < unsafe extern "C" fn() -> GDExtensionObjectPtr >;
pub type GDExtensionInstanceBindingCreateCallback = Option < unsafe extern "C" fn(p_token: * mut std::ffi::c_void, p_instance: * mut std::ffi::c_void) -> * mut std::ffi::c_void >;
pub type GDExtensionInstanceBindingFreeCallback = Option < unsafe extern "C" fn(p_token: * mut std::ffi::c_void, p_instance: * mut std::ffi::c_void, p_binding: * mut std::ffi::c_void) >;
pub type GDExtensionInstanceBindingReferenceCallback = Option < unsafe extern "C" fn(p_token: * mut std::ffi::c_void, p_binding: * mut std::ffi::c_void, p_reference: GDExtensionBool) -> GDExtensionBool >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionInstanceBindingCallbacks {
    pub create_callback: GDExtensionInstanceBindingCreateCallback, pub free_callback: GDExtensionInstanceBindingFreeCallback, pub reference_callback: GDExtensionInstanceBindingReferenceCallback,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextClassInstance {
    _unused: [u8;
    0],
}
pub type GDExtensionClassInstancePtr = * mut __GdextClassInstance;
pub type GDExtensionClassSet = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, p_name: GDExtensionConstStringNamePtr, p_value: GDExtensionConstVariantPtr) -> GDExtensionBool >;
pub type GDExtensionClassGet = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, p_name: GDExtensionConstStringNamePtr, r_ret: GDExtensionVariantPtr) -> GDExtensionBool >;
pub type GDExtensionClassGetRID = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr) -> u64 >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionPropertyInfo {
    pub type_: GDExtensionVariantType, pub name: GDExtensionStringNamePtr, pub class_name: GDExtensionStringNamePtr, pub hint: u32, pub hint_string: GDExtensionStringPtr, pub usage: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionMethodInfo {
    pub name: GDExtensionStringNamePtr, pub return_value: GDExtensionPropertyInfo, pub flags: u32, pub id: i32, pub argument_count: u32, pub arguments: * mut GDExtensionPropertyInfo, pub default_argument_count: u32, pub default_arguments: * mut GDExtensionVariantPtr,
}
pub type GDExtensionClassGetPropertyList = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, r_count: * mut u32) -> * const GDExtensionPropertyInfo >;
pub type GDExtensionClassFreePropertyList = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, p_list: * const GDExtensionPropertyInfo) >;
pub type GDExtensionClassFreePropertyList2 = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, p_list: * const GDExtensionPropertyInfo, p_count: u32) >;
pub type GDExtensionClassPropertyCanRevert = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, p_name: GDExtensionConstStringNamePtr) -> GDExtensionBool >;
pub type GDExtensionClassPropertyGetRevert = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, p_name: GDExtensionConstStringNamePtr, r_ret: GDExtensionVariantPtr) -> GDExtensionBool >;
pub type GDExtensionClassValidateProperty = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, p_property: * mut GDExtensionPropertyInfo) -> GDExtensionBool >;
pub type GDExtensionClassNotification = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, p_what: i32) >;
pub type GDExtensionClassNotification2 = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, p_what: i32, p_reversed: GDExtensionBool) >;
pub type GDExtensionClassToString = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, r_is_valid: * mut GDExtensionBool, p_out: GDExtensionStringPtr) >;
pub type GDExtensionClassReference = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr) >;
pub type GDExtensionClassUnreference = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr) >;
pub type GDExtensionClassCallVirtual = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, p_args: * const GDExtensionConstTypePtr, r_ret: GDExtensionTypePtr) >;
pub type GDExtensionClassCreateInstance = Option < unsafe extern "C" fn(p_class_userdata: * mut std::ffi::c_void) -> GDExtensionObjectPtr >;
pub type GDExtensionClassCreateInstance2 = Option < unsafe extern "C" fn(p_class_userdata: * mut std::ffi::c_void, p_notify_postinitialize: GDExtensionBool) -> GDExtensionObjectPtr >;
pub type GDExtensionClassCreateInstance3 = Option < unsafe extern "C" fn(p_class_userdata: * mut std::ffi::c_void, p_notify_postinitialize: GDExtensionBool) -> GDExtensionObjectPtr >;
pub type GDExtensionClassFreeInstance = Option < unsafe extern "C" fn(p_class_userdata: * mut std::ffi::c_void, p_instance: GDExtensionClassInstancePtr) >;
pub type GDExtensionClassRecreateInstance = Option < unsafe extern "C" fn(p_class_userdata: * mut std::ffi::c_void, p_object: GDExtensionObjectPtr) -> GDExtensionClassInstancePtr >;
pub type GDExtensionClassGetVirtual = Option < unsafe extern "C" fn(p_class_userdata: * mut std::ffi::c_void, p_name: GDExtensionConstStringNamePtr) -> GDExtensionClassCallVirtual >;
pub type GDExtensionClassGetVirtual2 = Option < unsafe extern "C" fn(p_class_userdata: * mut std::ffi::c_void, p_name: GDExtensionConstStringNamePtr, p_hash: u32) -> GDExtensionClassCallVirtual >;
pub type GDExtensionClassGetVirtualCallData = Option < unsafe extern "C" fn(p_class_userdata: * mut std::ffi::c_void, p_name: GDExtensionConstStringNamePtr) -> * mut std::ffi::c_void >;
pub type GDExtensionClassGetVirtualCallData2 = Option < unsafe extern "C" fn(p_class_userdata: * mut std::ffi::c_void, p_name: GDExtensionConstStringNamePtr, p_hash: u32) -> * mut std::ffi::c_void >;
pub type GDExtensionClassCallVirtualWithData = Option < unsafe extern "C" fn(p_instance: GDExtensionClassInstancePtr, p_name: GDExtensionConstStringNamePtr, p_virtual_call_userdata: * mut std::ffi::c_void, p_args: * const GDExtensionConstTypePtr, r_ret: GDExtensionTypePtr) >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionClassCreationInfo {
    pub is_virtual: GDExtensionBool, pub is_abstract: GDExtensionBool, pub set_func: GDExtensionClassSet, pub get_func: GDExtensionClassGet, pub get_property_list_func: GDExtensionClassGetPropertyList, pub free_property_list_func: GDExtensionClassFreePropertyList, pub property_can_revert_func: GDExtensionClassPropertyCanRevert, pub property_get_revert_func: GDExtensionClassPropertyGetRevert, pub notification_func: GDExtensionClassNotification, pub to_string_func: GDExtensionClassToString, pub reference_func: GDExtensionClassReference, pub unreference_func: GDExtensionClassUnreference, pub create_instance_func: GDExtensionClassCreateInstance, pub free_instance_func: GDExtensionClassFreeInstance, pub get_virtual_func: GDExtensionClassGetVirtual, pub get_rid_func: GDExtensionClassGetRID, pub class_userdata: * mut std::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionClassCreationInfo2 {
    pub is_virtual: GDExtensionBool, pub is_abstract: GDExtensionBool, pub is_exposed: GDExtensionBool, pub set_func: GDExtensionClassSet, pub get_func: GDExtensionClassGet, pub get_property_list_func: GDExtensionClassGetPropertyList, pub free_property_list_func: GDExtensionClassFreePropertyList, pub property_can_revert_func: GDExtensionClassPropertyCanRevert, pub property_get_revert_func: GDExtensionClassPropertyGetRevert, pub validate_property_func: GDExtensionClassValidateProperty, pub notification_func: GDExtensionClassNotification2, pub to_string_func: GDExtensionClassToString, pub reference_func: GDExtensionClassReference, pub unreference_func: GDExtensionClassUnreference, pub create_instance_func: GDExtensionClassCreateInstance, pub free_instance_func: GDExtensionClassFreeInstance, pub recreate_instance_func: GDExtensionClassRecreateInstance, pub get_virtual_func: GDExtensionClassGetVirtual, pub get_virtual_call_data_func: GDExtensionClassGetVirtualCallData, pub call_virtual_with_data_func: GDExtensionClassCallVirtualWithData, pub get_rid_func: GDExtensionClassGetRID, pub class_userdata: * mut std::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionClassCreationInfo3 {
    pub is_virtual: GDExtensionBool, pub is_abstract: GDExtensionBool, pub is_exposed: GDExtensionBool, pub is_runtime: GDExtensionBool, pub set_func: GDExtensionClassSet, pub get_func: GDExtensionClassGet, pub get_property_list_func: GDExtensionClassGetPropertyList, pub free_property_list_func: GDExtensionClassFreePropertyList2, pub property_can_revert_func: GDExtensionClassPropertyCanRevert, pub property_get_revert_func: GDExtensionClassPropertyGetRevert, pub validate_property_func: GDExtensionClassValidateProperty, pub notification_func: GDExtensionClassNotification2, pub to_string_func: GDExtensionClassToString, pub reference_func: GDExtensionClassReference, pub unreference_func: GDExtensionClassUnreference, pub create_instance_func: GDExtensionClassCreateInstance, pub free_instance_func: GDExtensionClassFreeInstance, pub recreate_instance_func: GDExtensionClassRecreateInstance, pub get_virtual_func: GDExtensionClassGetVirtual, pub get_virtual_call_data_func: GDExtensionClassGetVirtualCallData, pub call_virtual_with_data_func: GDExtensionClassCallVirtualWithData, pub get_rid_func: GDExtensionClassGetRID, pub class_userdata: * mut std::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionClassCreationInfo4 {
    pub is_virtual: GDExtensionBool, pub is_abstract: GDExtensionBool, pub is_exposed: GDExtensionBool, pub is_runtime: GDExtensionBool, pub icon_path: GDExtensionConstStringPtr, pub set_func: GDExtensionClassSet, pub get_func: GDExtensionClassGet, pub get_property_list_func: GDExtensionClassGetPropertyList, pub free_property_list_func: GDExtensionClassFreePropertyList2, pub property_can_revert_func: GDExtensionClassPropertyCanRevert, pub property_get_revert_func: GDExtensionClassPropertyGetRevert, pub validate_property_func: GDExtensionClassValidateProperty, pub notification_func: GDExtensionClassNotification2, pub to_string_func: GDExtensionClassToString, pub reference_func: GDExtensionClassReference, pub unreference_func: GDExtensionClassUnreference, pub create_instance_func: GDExtensionClassCreateInstance2, pub free_instance_func: GDExtensionClassFreeInstance, pub recreate_instance_func: GDExtensionClassRecreateInstance, pub get_virtual_func: GDExtensionClassGetVirtual2, pub get_virtual_call_data_func: GDExtensionClassGetVirtualCallData2, pub call_virtual_with_data_func: GDExtensionClassCallVirtualWithData, pub class_userdata: * mut std::ffi::c_void,
}
pub type GDExtensionClassCreationInfo5 = GDExtensionClassCreationInfo4;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionClassCreationInfo6 {
    pub is_virtual: GDExtensionBool, pub is_abstract: GDExtensionBool, pub is_exposed: GDExtensionBool, pub is_runtime: GDExtensionBool, pub icon_path: GDExtensionConstStringPtr, pub set_func: GDExtensionClassSet, pub get_func: GDExtensionClassGet, pub get_property_list_func: GDExtensionClassGetPropertyList, pub free_property_list_func: GDExtensionClassFreePropertyList2, pub property_can_revert_func: GDExtensionClassPropertyCanRevert, pub property_get_revert_func: GDExtensionClassPropertyGetRevert, pub validate_property_func: GDExtensionClassValidateProperty, pub notification_func: GDExtensionClassNotification2, pub to_string_func: GDExtensionClassToString, pub reference_func: GDExtensionClassReference, pub unreference_func: GDExtensionClassUnreference, pub create_instance_func: GDExtensionClassCreateInstance3, pub free_instance_func: GDExtensionClassFreeInstance, pub recreate_instance_func: GDExtensionClassRecreateInstance, pub get_virtual_func: GDExtensionClassGetVirtual2, pub get_virtual_call_data_func: GDExtensionClassGetVirtualCallData2, pub call_virtual_with_data_func: GDExtensionClassCallVirtualWithData, pub class_userdata: * mut std::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextClassLibrary {
    _unused: [u8;
    0],
}
pub type GDExtensionClassLibraryPtr = * mut __GdextClassLibrary;
pub type GDExtensionEditorGetClassesUsedCallback = Option < unsafe extern "C" fn(p_packed_string_array: GDExtensionTypePtr) >;
pub type GDExtensionClassMethodFlags = std::ffi::c_int;
pub const GDEXTENSION_METHOD_FLAG_NORMAL: GDExtensionClassMethodFlags = 1;
pub const GDEXTENSION_METHOD_FLAG_EDITOR: GDExtensionClassMethodFlags = 2;
pub const GDEXTENSION_METHOD_FLAG_CONST: GDExtensionClassMethodFlags = 4;
pub const GDEXTENSION_METHOD_FLAG_VIRTUAL: GDExtensionClassMethodFlags = 8;
pub const GDEXTENSION_METHOD_FLAG_VARARG: GDExtensionClassMethodFlags = 16;
pub const GDEXTENSION_METHOD_FLAG_STATIC: GDExtensionClassMethodFlags = 32;
pub const GDEXTENSION_METHOD_FLAG_VIRTUAL_REQUIRED: GDExtensionClassMethodFlags = 128;
pub const GDEXTENSION_METHOD_FLAGS_DEFAULT: GDExtensionClassMethodFlags = 1;
pub type GDExtensionClassMethodArgumentMetadata = std::ffi::c_int;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_NONE: GDExtensionClassMethodArgumentMetadata = 0;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_INT8: GDExtensionClassMethodArgumentMetadata = 1;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_INT16: GDExtensionClassMethodArgumentMetadata = 2;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_INT32: GDExtensionClassMethodArgumentMetadata = 3;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_INT64: GDExtensionClassMethodArgumentMetadata = 4;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_UINT8: GDExtensionClassMethodArgumentMetadata = 5;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_UINT16: GDExtensionClassMethodArgumentMetadata = 6;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_UINT32: GDExtensionClassMethodArgumentMetadata = 7;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_UINT64: GDExtensionClassMethodArgumentMetadata = 8;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_REAL_IS_FLOAT: GDExtensionClassMethodArgumentMetadata = 9;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_REAL_IS_DOUBLE: GDExtensionClassMethodArgumentMetadata = 10;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_CHAR16: GDExtensionClassMethodArgumentMetadata = 11;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_INT_IS_CHAR32: GDExtensionClassMethodArgumentMetadata = 12;
pub const GDEXTENSION_METHOD_ARGUMENT_METADATA_OBJECT_IS_REQUIRED: GDExtensionClassMethodArgumentMetadata = 13;
pub type GDExtensionClassMethodCall = Option < unsafe extern "C" fn(method_userdata: * mut std::ffi::c_void, p_instance: GDExtensionClassInstancePtr, p_args: * const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionVariantPtr, r_error: * mut GDExtensionCallError) >;
pub type GDExtensionClassMethodValidatedCall = Option < unsafe extern "C" fn(method_userdata: * mut std::ffi::c_void, p_instance: GDExtensionClassInstancePtr, p_args: * const GDExtensionConstVariantPtr, r_return: GDExtensionVariantPtr) >;
pub type GDExtensionClassMethodPtrCall = Option < unsafe extern "C" fn(method_userdata: * mut std::ffi::c_void, p_instance: GDExtensionClassInstancePtr, p_args: * const GDExtensionConstTypePtr, r_ret: GDExtensionTypePtr) >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionClassMethodInfo {
    pub name: GDExtensionStringNamePtr, pub method_userdata: * mut std::ffi::c_void, pub call_func: GDExtensionClassMethodCall, pub ptrcall_func: GDExtensionClassMethodPtrCall, pub method_flags: u32, pub has_return_value: GDExtensionBool, pub return_value_info: * mut GDExtensionPropertyInfo, pub return_value_metadata: GDExtensionClassMethodArgumentMetadata, pub argument_count: u32, pub arguments_info: * mut GDExtensionPropertyInfo, pub arguments_metadata: * mut GDExtensionClassMethodArgumentMetadata, pub default_argument_count: u32, pub default_arguments: * mut GDExtensionVariantPtr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionClassVirtualMethodInfo {
    pub name: GDExtensionStringNamePtr, pub method_flags: u32, pub return_value: GDExtensionPropertyInfo, pub return_value_metadata: GDExtensionClassMethodArgumentMetadata, pub argument_count: u32, pub arguments: * mut GDExtensionPropertyInfo, pub arguments_metadata: * mut GDExtensionClassMethodArgumentMetadata,
}
pub type GDExtensionCallableCustomCall = Option < unsafe extern "C" fn(callable_userdata: * mut std::ffi::c_void, p_args: * const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionVariantPtr, r_error: * mut GDExtensionCallError) >;
pub type GDExtensionCallableCustomIsValid = Option < unsafe extern "C" fn(callable_userdata: * mut std::ffi::c_void) -> GDExtensionBool >;
pub type GDExtensionCallableCustomFree = Option < unsafe extern "C" fn(callable_userdata: * mut std::ffi::c_void) >;
pub type GDExtensionCallableCustomHash = Option < unsafe extern "C" fn(callable_userdata: * mut std::ffi::c_void) -> u32 >;
pub type GDExtensionCallableCustomEqual = Option < unsafe extern "C" fn(callable_userdata_a: * mut std::ffi::c_void, callable_userdata_b: * mut std::ffi::c_void) -> GDExtensionBool >;
pub type GDExtensionCallableCustomLessThan = Option < unsafe extern "C" fn(callable_userdata_a: * mut std::ffi::c_void, callable_userdata_b: * mut std::ffi::c_void) -> GDExtensionBool >;
pub type GDExtensionCallableCustomToString = Option < unsafe extern "C" fn(callable_userdata: * mut std::ffi::c_void, r_is_valid: * mut GDExtensionBool, r_out: GDExtensionStringPtr) >;
pub type GDExtensionCallableCustomGetArgumentCount = Option < unsafe extern "C" fn(callable_userdata: * mut std::ffi::c_void, r_is_valid: * mut GDExtensionBool) -> GDExtensionInt >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionCallableCustomInfo {
    pub callable_userdata: * mut std::ffi::c_void, pub token: * mut std::ffi::c_void, pub object_id: GDObjectInstanceID, pub call_func: GDExtensionCallableCustomCall, pub is_valid_func: GDExtensionCallableCustomIsValid, pub free_func: GDExtensionCallableCustomFree, pub hash_func: GDExtensionCallableCustomHash, pub equal_func: GDExtensionCallableCustomEqual, pub less_than_func: GDExtensionCallableCustomLessThan, pub to_string_func: GDExtensionCallableCustomToString,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionCallableCustomInfo2 {
    pub callable_userdata: * mut std::ffi::c_void, pub token: * mut std::ffi::c_void, pub object_id: GDObjectInstanceID, pub call_func: GDExtensionCallableCustomCall, pub is_valid_func: GDExtensionCallableCustomIsValid, pub free_func: GDExtensionCallableCustomFree, pub hash_func: GDExtensionCallableCustomHash, pub equal_func: GDExtensionCallableCustomEqual, pub less_than_func: GDExtensionCallableCustomLessThan, pub to_string_func: GDExtensionCallableCustomToString, pub get_argument_count_func: GDExtensionCallableCustomGetArgumentCount,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextScriptInstanceData {
    _unused: [u8;
    0],
}
pub type GDExtensionScriptInstanceDataPtr = * mut __GdextScriptInstanceData;
pub type GDExtensionScriptInstanceSet = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr, p_value: GDExtensionConstVariantPtr) -> GDExtensionBool >;
pub type GDExtensionScriptInstanceGet = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr, r_ret: GDExtensionVariantPtr) -> GDExtensionBool >;
pub type GDExtensionScriptInstanceGetPropertyList = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, r_count: * mut u32) -> * const GDExtensionPropertyInfo >;
pub type GDExtensionScriptInstanceFreePropertyList = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_list: * const GDExtensionPropertyInfo) >;
pub type GDExtensionScriptInstanceFreePropertyList2 = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_list: * const GDExtensionPropertyInfo, p_count: u32) >;
pub type GDExtensionScriptInstanceGetClassCategory = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_class_category: * mut GDExtensionPropertyInfo) -> GDExtensionBool >;
pub type GDExtensionScriptInstanceGetPropertyType = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr, r_is_valid: * mut GDExtensionBool) -> GDExtensionVariantType >;
pub type GDExtensionScriptInstanceValidateProperty = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_property: * mut GDExtensionPropertyInfo) -> GDExtensionBool >;
pub type GDExtensionScriptInstancePropertyCanRevert = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr) -> GDExtensionBool >;
pub type GDExtensionScriptInstancePropertyGetRevert = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr, r_ret: GDExtensionVariantPtr) -> GDExtensionBool >;
pub type GDExtensionScriptInstanceGetOwner = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr) -> GDExtensionObjectPtr >;
pub type GDExtensionScriptInstancePropertyStateAdd = Option < unsafe extern "C" fn(p_name: GDExtensionConstStringNamePtr, p_value: GDExtensionConstVariantPtr, p_userdata: * mut std::ffi::c_void) >;
pub type GDExtensionScriptInstanceGetPropertyState = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_add_func: GDExtensionScriptInstancePropertyStateAdd, p_userdata: * mut std::ffi::c_void) >;
pub type GDExtensionScriptInstanceGetMethodList = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, r_count: * mut u32) -> * const GDExtensionMethodInfo >;
pub type GDExtensionScriptInstanceFreeMethodList = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_list: * const GDExtensionMethodInfo) >;
pub type GDExtensionScriptInstanceFreeMethodList2 = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_list: * const GDExtensionMethodInfo, p_count: u32) >;
pub type GDExtensionScriptInstanceHasMethod = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr) -> GDExtensionBool >;
pub type GDExtensionScriptInstanceGetMethodArgumentCount = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_name: GDExtensionConstStringNamePtr, r_is_valid: * mut GDExtensionBool) -> GDExtensionInt >;
pub type GDExtensionScriptInstanceCall = Option < unsafe extern "C" fn(p_self: GDExtensionScriptInstanceDataPtr, p_method: GDExtensionConstStringNamePtr, p_args: * const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionVariantPtr, r_error: * mut GDExtensionCallError) >;
pub type GDExtensionScriptInstanceNotification = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_what: i32) >;
pub type GDExtensionScriptInstanceNotification2 = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, p_what: i32, p_reversed: GDExtensionBool) >;
pub type GDExtensionScriptInstanceToString = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr, r_is_valid: * mut GDExtensionBool, r_out: GDExtensionStringPtr) >;
pub type GDExtensionScriptInstanceRefCountIncremented = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr) >;
pub type GDExtensionScriptInstanceRefCountDecremented = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr) -> GDExtensionBool >;
pub type GDExtensionScriptInstanceGetScript = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr) -> GDExtensionObjectPtr >;
pub type GDExtensionScriptInstanceIsPlaceholder = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr) -> GDExtensionBool >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextScriptLanguage {
    _unused: [u8;
    0],
}
pub type GDExtensionScriptLanguagePtr = * mut __GdextScriptLanguage;
pub type GDExtensionScriptInstanceGetLanguage = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr) -> GDExtensionScriptLanguagePtr >;
pub type GDExtensionScriptInstanceFree = Option < unsafe extern "C" fn(p_instance: GDExtensionScriptInstanceDataPtr) >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GdextScriptInstance {
    _unused: [u8;
    0],
}
pub type GDExtensionScriptInstancePtr = * mut __GdextScriptInstance;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionScriptInstanceInfo {
    pub set_func: GDExtensionScriptInstanceSet, pub get_func: GDExtensionScriptInstanceGet, pub get_property_list_func: GDExtensionScriptInstanceGetPropertyList, pub free_property_list_func: GDExtensionScriptInstanceFreePropertyList, pub property_can_revert_func: GDExtensionScriptInstancePropertyCanRevert, pub property_get_revert_func: GDExtensionScriptInstancePropertyGetRevert, pub get_owner_func: GDExtensionScriptInstanceGetOwner, pub get_property_state_func: GDExtensionScriptInstanceGetPropertyState, pub get_method_list_func: GDExtensionScriptInstanceGetMethodList, pub free_method_list_func: GDExtensionScriptInstanceFreeMethodList, pub get_property_type_func: GDExtensionScriptInstanceGetPropertyType, pub has_method_func: GDExtensionScriptInstanceHasMethod, pub call_func: GDExtensionScriptInstanceCall, pub notification_func: GDExtensionScriptInstanceNotification, pub to_string_func: GDExtensionScriptInstanceToString, pub refcount_incremented_func: GDExtensionScriptInstanceRefCountIncremented, pub refcount_decremented_func: GDExtensionScriptInstanceRefCountDecremented, pub get_script_func: GDExtensionScriptInstanceGetScript, pub is_placeholder_func: GDExtensionScriptInstanceIsPlaceholder, pub set_fallback_func: GDExtensionScriptInstanceSet, pub get_fallback_func: GDExtensionScriptInstanceGet, pub get_language_func: GDExtensionScriptInstanceGetLanguage, pub free_func: GDExtensionScriptInstanceFree,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionScriptInstanceInfo2 {
    pub set_func: GDExtensionScriptInstanceSet, pub get_func: GDExtensionScriptInstanceGet, pub get_property_list_func: GDExtensionScriptInstanceGetPropertyList, pub free_property_list_func: GDExtensionScriptInstanceFreePropertyList, pub get_class_category_func: GDExtensionScriptInstanceGetClassCategory, pub property_can_revert_func: GDExtensionScriptInstancePropertyCanRevert, pub property_get_revert_func: GDExtensionScriptInstancePropertyGetRevert, pub get_owner_func: GDExtensionScriptInstanceGetOwner, pub get_property_state_func: GDExtensionScriptInstanceGetPropertyState, pub get_method_list_func: GDExtensionScriptInstanceGetMethodList, pub free_method_list_func: GDExtensionScriptInstanceFreeMethodList, pub get_property_type_func: GDExtensionScriptInstanceGetPropertyType, pub validate_property_func: GDExtensionScriptInstanceValidateProperty, pub has_method_func: GDExtensionScriptInstanceHasMethod, pub call_func: GDExtensionScriptInstanceCall, pub notification_func: GDExtensionScriptInstanceNotification2, pub to_string_func: GDExtensionScriptInstanceToString, pub refcount_incremented_func: GDExtensionScriptInstanceRefCountIncremented, pub refcount_decremented_func: GDExtensionScriptInstanceRefCountDecremented, pub get_script_func: GDExtensionScriptInstanceGetScript, pub is_placeholder_func: GDExtensionScriptInstanceIsPlaceholder, pub set_fallback_func: GDExtensionScriptInstanceSet, pub get_fallback_func: GDExtensionScriptInstanceGet, pub get_language_func: GDExtensionScriptInstanceGetLanguage, pub free_func: GDExtensionScriptInstanceFree,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionScriptInstanceInfo3 {
    pub set_func: GDExtensionScriptInstanceSet, pub get_func: GDExtensionScriptInstanceGet, pub get_property_list_func: GDExtensionScriptInstanceGetPropertyList, pub free_property_list_func: GDExtensionScriptInstanceFreePropertyList2, pub get_class_category_func: GDExtensionScriptInstanceGetClassCategory, pub property_can_revert_func: GDExtensionScriptInstancePropertyCanRevert, pub property_get_revert_func: GDExtensionScriptInstancePropertyGetRevert, pub get_owner_func: GDExtensionScriptInstanceGetOwner, pub get_property_state_func: GDExtensionScriptInstanceGetPropertyState, pub get_method_list_func: GDExtensionScriptInstanceGetMethodList, pub free_method_list_func: GDExtensionScriptInstanceFreeMethodList2, pub get_property_type_func: GDExtensionScriptInstanceGetPropertyType, pub validate_property_func: GDExtensionScriptInstanceValidateProperty, pub has_method_func: GDExtensionScriptInstanceHasMethod, pub get_method_argument_count_func: GDExtensionScriptInstanceGetMethodArgumentCount, pub call_func: GDExtensionScriptInstanceCall, pub notification_func: GDExtensionScriptInstanceNotification2, pub to_string_func: GDExtensionScriptInstanceToString, pub refcount_incremented_func: GDExtensionScriptInstanceRefCountIncremented, pub refcount_decremented_func: GDExtensionScriptInstanceRefCountDecremented, pub get_script_func: GDExtensionScriptInstanceGetScript, pub is_placeholder_func: GDExtensionScriptInstanceIsPlaceholder, pub set_fallback_func: GDExtensionScriptInstanceSet, pub get_fallback_func: GDExtensionScriptInstanceGet, pub get_language_func: GDExtensionScriptInstanceGetLanguage, pub free_func: GDExtensionScriptInstanceFree,
}
pub type GDExtensionWorkerThreadPoolGroupTask = Option < unsafe extern "C" fn(* mut std::ffi::c_void, u32) >;
pub type GDExtensionWorkerThreadPoolTask = Option < unsafe extern "C" fn(* mut std::ffi::c_void) >;
pub type GDExtensionInitializationLevel = std::ffi::c_int;
pub const GDEXTENSION_INITIALIZATION_CORE: GDExtensionInitializationLevel = 0;
pub const GDEXTENSION_INITIALIZATION_SERVERS: GDExtensionInitializationLevel = 1;
pub const GDEXTENSION_INITIALIZATION_SCENE: GDExtensionInitializationLevel = 2;
pub const GDEXTENSION_INITIALIZATION_EDITOR: GDExtensionInitializationLevel = 3;
pub const GDEXTENSION_MAX_INITIALIZATION_LEVEL: GDExtensionInitializationLevel = 4;
pub type GDExtensionInitializeCallback = Option < unsafe extern "C" fn(p_userdata: * mut std::ffi::c_void, p_level: GDExtensionInitializationLevel) >;
pub type GDExtensionDeinitializeCallback = Option < unsafe extern "C" fn(p_userdata: * mut std::ffi::c_void, p_level: GDExtensionInitializationLevel) >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionInitialization {
    pub minimum_initialization_level: GDExtensionInitializationLevel, pub userdata: * mut std::ffi::c_void, pub initialize: GDExtensionInitializeCallback, pub deinitialize: GDExtensionDeinitializeCallback,
}
pub type GDExtensionInterfaceFunctionPtr = Option < unsafe extern "C" fn() >;
pub type GDExtensionInterfaceGetProcAddress = Option < unsafe extern "C" fn(p_function_name: * const std::ffi::c_char) -> GDExtensionInterfaceFunctionPtr >;
pub type GDExtensionInitializationFunction = Option < unsafe extern "C" fn(p_get_proc_address: GDExtensionInterfaceGetProcAddress, p_library: GDExtensionClassLibraryPtr, r_initialization: * mut GDExtensionInitialization) -> GDExtensionBool >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionGodotVersion {
    pub major: u32, pub minor: u32, pub patch: u32, pub string: * const std::ffi::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionGodotVersion2 {
    pub major: u32, pub minor: u32, pub patch: u32, pub hex: u32, pub status: * const std::ffi::c_char, pub build: * const std::ffi::c_char, pub hash: * const std::ffi::c_char, pub timestamp: u64, pub string: * const std::ffi::c_char,
}
pub type GDExtensionMainLoopStartupCallback = Option < unsafe extern "C" fn() >;
pub type GDExtensionMainLoopShutdownCallback = Option < unsafe extern "C" fn() >;
pub type GDExtensionMainLoopFrameCallback = Option < unsafe extern "C" fn() >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GDExtensionMainLoopCallbacks {
    pub startup_func: GDExtensionMainLoopStartupCallback, pub shutdown_func: GDExtensionMainLoopShutdownCallback, pub frame_func: GDExtensionMainLoopFrameCallback,
}
pub type GDExtensionInterfaceGetGodotVersion = unsafe extern "C" fn(r_godot_version: * mut GDExtensionGodotVersion);
#[cfg(since_api = "4.5")]
pub type GDExtensionInterfaceGetGodotVersion2 = unsafe extern "C" fn(r_godot_version: * mut GDExtensionGodotVersion2);
pub type GDExtensionInterfaceMemAlloc = unsafe extern "C" fn(p_bytes: usize) -> * mut std::ffi::c_void;
pub type GDExtensionInterfaceMemRealloc = unsafe extern "C" fn(p_ptr: * mut std::ffi::c_void, p_bytes: usize) -> * mut std::ffi::c_void;
pub type GDExtensionInterfaceMemFree = unsafe extern "C" fn(p_ptr: * mut std::ffi::c_void);
#[cfg(since_api = "4.6")]
pub type GDExtensionInterfaceMemAlloc2 = unsafe extern "C" fn(p_bytes: usize, p_pad_align: GDExtensionBool) -> * mut std::ffi::c_void;
#[cfg(since_api = "4.6")]
pub type GDExtensionInterfaceMemRealloc2 = unsafe extern "C" fn(p_ptr: * mut std::ffi::c_void, p_bytes: usize, p_pad_align: GDExtensionBool) -> * mut std::ffi::c_void;
#[cfg(since_api = "4.6")]
pub type GDExtensionInterfaceMemFree2 = unsafe extern "C" fn(p_ptr: * mut std::ffi::c_void, p_pad_align: GDExtensionBool);
pub type GDExtensionInterfacePrintError = unsafe extern "C" fn(p_description: * const std::ffi::c_char, p_function: * const std::ffi::c_char, p_file: * const std::ffi::c_char, p_line: i32, p_editor_notify: GDExtensionBool);
pub type GDExtensionInterfacePrintErrorWithMessage = unsafe extern "C" fn(p_description: * const std::ffi::c_char, p_message: * const std::ffi::c_char, p_function: * const std::ffi::c_char, p_file: * const std::ffi::c_char, p_line: i32, p_editor_notify: GDExtensionBool);
pub type GDExtensionInterfacePrintWarning = unsafe extern "C" fn(p_description: * const std::ffi::c_char, p_function: * const std::ffi::c_char, p_file: * const std::ffi::c_char, p_line: i32, p_editor_notify: GDExtensionBool);
pub type GDExtensionInterfacePrintWarningWithMessage = unsafe extern "C" fn(p_description: * const std::ffi::c_char, p_message: * const std::ffi::c_char, p_function: * const std::ffi::c_char, p_file: * const std::ffi::c_char, p_line: i32, p_editor_notify: GDExtensionBool);
pub type GDExtensionInterfacePrintScriptError = unsafe extern "C" fn(p_description: * const std::ffi::c_char, p_function: * const std::ffi::c_char, p_file: * const std::ffi::c_char, p_line: i32, p_editor_notify: GDExtensionBool);
pub type GDExtensionInterfacePrintScriptErrorWithMessage = unsafe extern "C" fn(p_description: * const std::ffi::c_char, p_message: * const std::ffi::c_char, p_function: * const std::ffi::c_char, p_file: * const std::ffi::c_char, p_line: i32, p_editor_notify: GDExtensionBool);
pub type GDExtensionInterfaceGetNativeStructSize = unsafe extern "C" fn(p_name: GDExtensionConstStringNamePtr) -> u64;
pub type GDExtensionInterfaceVariantNewCopy = unsafe extern "C" fn(r_dest: GDExtensionUninitializedVariantPtr, p_src: GDExtensionConstVariantPtr);
pub type GDExtensionInterfaceVariantNewNil = unsafe extern "C" fn(r_dest: GDExtensionUninitializedVariantPtr);
pub type GDExtensionInterfaceVariantDestroy = unsafe extern "C" fn(p_self: GDExtensionVariantPtr);
pub type GDExtensionInterfaceVariantCall = unsafe extern "C" fn(p_self: GDExtensionVariantPtr, p_method: GDExtensionConstStringNamePtr, p_args: * const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionUninitializedVariantPtr, r_error: * mut GDExtensionCallError);
pub type GDExtensionInterfaceVariantCallStatic = unsafe extern "C" fn(p_type: GDExtensionVariantType, p_method: GDExtensionConstStringNamePtr, p_args: * const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionUninitializedVariantPtr, r_error: * mut GDExtensionCallError);
pub type GDExtensionInterfaceVariantEvaluate = unsafe extern "C" fn(p_op: GDExtensionVariantOperator, p_a: GDExtensionConstVariantPtr, p_b: GDExtensionConstVariantPtr, r_return: GDExtensionUninitializedVariantPtr, r_valid: * mut GDExtensionBool);
pub type GDExtensionInterfaceVariantSet = unsafe extern "C" fn(p_self: GDExtensionVariantPtr, p_key: GDExtensionConstVariantPtr, p_value: GDExtensionConstVariantPtr, r_valid: * mut GDExtensionBool);
pub type GDExtensionInterfaceVariantSetNamed = unsafe extern "C" fn(p_self: GDExtensionVariantPtr, p_key: GDExtensionConstStringNamePtr, p_value: GDExtensionConstVariantPtr, r_valid: * mut GDExtensionBool);
pub type GDExtensionInterfaceVariantSetKeyed = unsafe extern "C" fn(p_self: GDExtensionVariantPtr, p_key: GDExtensionConstVariantPtr, p_value: GDExtensionConstVariantPtr, r_valid: * mut GDExtensionBool);
pub type GDExtensionInterfaceVariantSetIndexed = unsafe extern "C" fn(p_self: GDExtensionVariantPtr, p_index: GDExtensionInt, p_value: GDExtensionConstVariantPtr, r_valid: * mut GDExtensionBool, r_oob: * mut GDExtensionBool);
pub type GDExtensionInterfaceVariantGet = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, p_key: GDExtensionConstVariantPtr, r_ret: GDExtensionUninitializedVariantPtr, r_valid: * mut GDExtensionBool);
pub type GDExtensionInterfaceVariantGetNamed = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, p_key: GDExtensionConstStringNamePtr, r_ret: GDExtensionUninitializedVariantPtr, r_valid: * mut GDExtensionBool);
pub type GDExtensionInterfaceVariantGetKeyed = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, p_key: GDExtensionConstVariantPtr, r_ret: GDExtensionUninitializedVariantPtr, r_valid: * mut GDExtensionBool);
pub type GDExtensionInterfaceVariantGetIndexed = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, p_index: GDExtensionInt, r_ret: GDExtensionUninitializedVariantPtr, r_valid: * mut GDExtensionBool, r_oob: * mut GDExtensionBool);
pub type GDExtensionInterfaceVariantIterInit = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, r_iter: GDExtensionUninitializedVariantPtr, r_valid: * mut GDExtensionBool) -> GDExtensionBool;
pub type GDExtensionInterfaceVariantIterNext = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, r_iter: GDExtensionVariantPtr, r_valid: * mut GDExtensionBool) -> GDExtensionBool;
pub type GDExtensionInterfaceVariantIterGet = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, r_iter: GDExtensionVariantPtr, r_ret: GDExtensionUninitializedVariantPtr, r_valid: * mut GDExtensionBool);
pub type GDExtensionInterfaceVariantHash = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr) -> GDExtensionInt;
pub type GDExtensionInterfaceVariantRecursiveHash = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, p_recursion_count: GDExtensionInt) -> GDExtensionInt;
pub type GDExtensionInterfaceVariantHashCompare = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, p_other: GDExtensionConstVariantPtr) -> GDExtensionBool;
pub type GDExtensionInterfaceVariantBooleanize = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr) -> GDExtensionBool;
pub type GDExtensionInterfaceVariantDuplicate = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, r_ret: GDExtensionVariantPtr, p_deep: GDExtensionBool);
pub type GDExtensionInterfaceVariantStringify = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, r_ret: GDExtensionStringPtr);
pub type GDExtensionInterfaceVariantGetType = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr) -> GDExtensionVariantType;
pub type GDExtensionInterfaceVariantHasMethod = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, p_method: GDExtensionConstStringNamePtr) -> GDExtensionBool;
pub type GDExtensionInterfaceVariantHasMember = unsafe extern "C" fn(p_type: GDExtensionVariantType, p_member: GDExtensionConstStringNamePtr) -> GDExtensionBool;
pub type GDExtensionInterfaceVariantHasKey = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr, p_key: GDExtensionConstVariantPtr, r_valid: * mut GDExtensionBool) -> GDExtensionBool;
#[cfg(since_api = "4.4")]
pub type GDExtensionInterfaceVariantGetObjectInstanceId = unsafe extern "C" fn(p_self: GDExtensionConstVariantPtr) -> GDObjectInstanceID;
pub type GDExtensionInterfaceVariantGetTypeName = unsafe extern "C" fn(p_type: GDExtensionVariantType, r_name: GDExtensionUninitializedStringPtr);
#[cfg(since_api = "4.7")]
pub type GDExtensionInterfaceVariantGetTypeByName = unsafe extern "C" fn(p_type_name: GDExtensionConstStringPtr) -> GDExtensionVariantType;
pub type GDExtensionInterfaceVariantCanConvert = unsafe extern "C" fn(p_from: GDExtensionVariantType, p_to: GDExtensionVariantType) -> GDExtensionBool;
pub type GDExtensionInterfaceVariantCanConvertStrict = unsafe extern "C" fn(p_from: GDExtensionVariantType, p_to: GDExtensionVariantType) -> GDExtensionBool;
pub type GDExtensionInterfaceGetVariantFromTypeConstructor = unsafe extern "C" fn(p_type: GDExtensionVariantType) -> GDExtensionVariantFromTypeConstructorFunc;
pub type GDExtensionInterfaceGetVariantToTypeConstructor = unsafe extern "C" fn(p_type: GDExtensionVariantType) -> GDExtensionTypeFromVariantConstructorFunc;
#[cfg(since_api = "4.4")]
pub type GDExtensionInterfaceVariantGetPtrInternalGetter = unsafe extern "C" fn(p_type: GDExtensionVariantType) -> GDExtensionVariantGetInternalPtrFunc;
pub type GDExtensionInterfaceVariantGetPtrOperatorEvaluator = unsafe extern "C" fn(p_operator: GDExtensionVariantOperator, p_type_a: GDExtensionVariantType, p_type_b: GDExtensionVariantType) -> GDExtensionPtrOperatorEvaluator;
pub type GDExtensionInterfaceVariantGetPtrBuiltinMethod = unsafe extern "C" fn(p_type: GDExtensionVariantType, p_method: GDExtensionConstStringNamePtr, p_hash: GDExtensionInt) -> GDExtensionPtrBuiltInMethod;
pub type GDExtensionInterfaceVariantGetPtrConstructor = unsafe extern "C" fn(p_type: GDExtensionVariantType, p_constructor: i32) -> GDExtensionPtrConstructor;
pub type GDExtensionInterfaceVariantGetPtrDestructor = unsafe extern "C" fn(p_type: GDExtensionVariantType) -> GDExtensionPtrDestructor;
pub type GDExtensionInterfaceVariantConstruct = unsafe extern "C" fn(p_type: GDExtensionVariantType, r_base: GDExtensionUninitializedVariantPtr, p_args: * const GDExtensionConstVariantPtr, p_argument_count: i32, r_error: * mut GDExtensionCallError);
pub type GDExtensionInterfaceVariantGetPtrSetter = unsafe extern "C" fn(p_type: GDExtensionVariantType, p_member: GDExtensionConstStringNamePtr) -> GDExtensionPtrSetter;
pub type GDExtensionInterfaceVariantGetPtrGetter = unsafe extern "C" fn(p_type: GDExtensionVariantType, p_member: GDExtensionConstStringNamePtr) -> GDExtensionPtrGetter;
pub type GDExtensionInterfaceVariantGetPtrIndexedSetter = unsafe extern "C" fn(p_type: GDExtensionVariantType) -> GDExtensionPtrIndexedSetter;
pub type GDExtensionInterfaceVariantGetPtrIndexedGetter = unsafe extern "C" fn(p_type: GDExtensionVariantType) -> GDExtensionPtrIndexedGetter;
pub type GDExtensionInterfaceVariantGetPtrKeyedSetter = unsafe extern "C" fn(p_type: GDExtensionVariantType) -> GDExtensionPtrKeyedSetter;
pub type GDExtensionInterfaceVariantGetPtrKeyedGetter = unsafe extern "C" fn(p_type: GDExtensionVariantType) -> GDExtensionPtrKeyedGetter;
pub type GDExtensionInterfaceVariantGetPtrKeyedChecker = unsafe extern "C" fn(p_type: GDExtensionVariantType) -> GDExtensionPtrKeyedChecker;
pub type GDExtensionInterfaceVariantGetConstantValue = unsafe extern "C" fn(p_type: GDExtensionVariantType, p_constant: GDExtensionConstStringNamePtr, r_ret: GDExtensionUninitializedVariantPtr);
pub type GDExtensionInterfaceVariantGetPtrUtilityFunction = unsafe extern "C" fn(p_function: GDExtensionConstStringNamePtr, p_hash: GDExtensionInt) -> GDExtensionPtrUtilityFunction;
pub type GDExtensionInterfaceStringNewWithLatin1Chars = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const std::ffi::c_char);
pub type GDExtensionInterfaceStringNewWithUtf8Chars = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const std::ffi::c_char);
pub type GDExtensionInterfaceStringNewWithUtf16Chars = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const char16_t);
pub type GDExtensionInterfaceStringNewWithUtf32Chars = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const char32_t);
pub type GDExtensionInterfaceStringNewWithWideChars = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const wchar_t);
pub type GDExtensionInterfaceStringNewWithLatin1CharsAndLen = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const std::ffi::c_char, p_size: GDExtensionInt);
pub type GDExtensionInterfaceStringNewWithUtf8CharsAndLen = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const std::ffi::c_char, p_size: GDExtensionInt);
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceStringNewWithUtf8CharsAndLen2 = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const std::ffi::c_char, p_size: GDExtensionInt) -> GDExtensionInt;
pub type GDExtensionInterfaceStringNewWithUtf16CharsAndLen = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const char16_t, p_char_count: GDExtensionInt);
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceStringNewWithUtf16CharsAndLen2 = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const char16_t, p_char_count: GDExtensionInt, p_default_little_endian: GDExtensionBool) -> GDExtensionInt;
pub type GDExtensionInterfaceStringNewWithUtf32CharsAndLen = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const char32_t, p_char_count: GDExtensionInt);
pub type GDExtensionInterfaceStringNewWithWideCharsAndLen = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringPtr, p_contents: * const wchar_t, p_char_count: GDExtensionInt);
pub type GDExtensionInterfaceStringToLatin1Chars = unsafe extern "C" fn(p_self: GDExtensionConstStringPtr, r_text: * mut std::ffi::c_char, p_max_write_length: GDExtensionInt) -> GDExtensionInt;
pub type GDExtensionInterfaceStringToUtf8Chars = unsafe extern "C" fn(p_self: GDExtensionConstStringPtr, r_text: * mut std::ffi::c_char, p_max_write_length: GDExtensionInt) -> GDExtensionInt;
pub type GDExtensionInterfaceStringToUtf16Chars = unsafe extern "C" fn(p_self: GDExtensionConstStringPtr, r_text: * mut char16_t, p_max_write_length: GDExtensionInt) -> GDExtensionInt;
pub type GDExtensionInterfaceStringToUtf32Chars = unsafe extern "C" fn(p_self: GDExtensionConstStringPtr, r_text: * mut char32_t, p_max_write_length: GDExtensionInt) -> GDExtensionInt;
pub type GDExtensionInterfaceStringToWideChars = unsafe extern "C" fn(p_self: GDExtensionConstStringPtr, r_text: * mut wchar_t, p_max_write_length: GDExtensionInt) -> GDExtensionInt;
pub type GDExtensionInterfaceStringOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionStringPtr, p_index: GDExtensionInt) -> * mut char32_t;
pub type GDExtensionInterfaceStringOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstStringPtr, p_index: GDExtensionInt) -> * const char32_t;
pub type GDExtensionInterfaceStringOperatorPlusEqString = unsafe extern "C" fn(p_self: GDExtensionStringPtr, p_b: GDExtensionConstStringPtr);
pub type GDExtensionInterfaceStringOperatorPlusEqChar = unsafe extern "C" fn(p_self: GDExtensionStringPtr, p_b: char32_t);
pub type GDExtensionInterfaceStringOperatorPlusEqCstr = unsafe extern "C" fn(p_self: GDExtensionStringPtr, p_b: * const std::ffi::c_char);
pub type GDExtensionInterfaceStringOperatorPlusEqWcstr = unsafe extern "C" fn(p_self: GDExtensionStringPtr, p_b: * const wchar_t);
pub type GDExtensionInterfaceStringOperatorPlusEqC32str = unsafe extern "C" fn(p_self: GDExtensionStringPtr, p_b: * const char32_t);
pub type GDExtensionInterfaceStringResize = unsafe extern "C" fn(p_self: GDExtensionStringPtr, p_resize: GDExtensionInt) -> GDExtensionInt;
pub type GDExtensionInterfaceStringNameNewWithLatin1Chars = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringNamePtr, p_contents: * const std::ffi::c_char, p_is_static: GDExtensionBool);
pub type GDExtensionInterfaceStringNameNewWithUtf8Chars = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringNamePtr, p_contents: * const std::ffi::c_char);
pub type GDExtensionInterfaceStringNameNewWithUtf8CharsAndLen = unsafe extern "C" fn(r_dest: GDExtensionUninitializedStringNamePtr, p_contents: * const std::ffi::c_char, p_size: GDExtensionInt);
pub type GDExtensionInterfaceXmlParserOpenBuffer = unsafe extern "C" fn(p_instance: GDExtensionObjectPtr, p_buffer: * const u8, p_size: usize) -> GDExtensionInt;
pub type GDExtensionInterfaceFileAccessStoreBuffer = unsafe extern "C" fn(p_instance: GDExtensionObjectPtr, p_src: * const u8, p_length: u64);
pub type GDExtensionInterfaceFileAccessGetBuffer = unsafe extern "C" fn(p_instance: GDExtensionConstObjectPtr, p_dst: * mut u8, p_length: u64) -> u64;
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceImagePtrw = unsafe extern "C" fn(p_instance: GDExtensionObjectPtr) -> * mut u8;
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceImagePtr = unsafe extern "C" fn(p_instance: GDExtensionObjectPtr) -> * const u8;
pub type GDExtensionInterfaceWorkerThreadPoolAddNativeGroupTask = unsafe extern "C" fn(p_instance: GDExtensionObjectPtr, p_func: GDExtensionWorkerThreadPoolGroupTask, p_userdata: * mut std::ffi::c_void, p_elements: i32, p_tasks: i32, p_high_priority: GDExtensionBool, p_description: GDExtensionConstStringPtr) -> i64;
pub type GDExtensionInterfaceWorkerThreadPoolAddNativeTask = unsafe extern "C" fn(p_instance: GDExtensionObjectPtr, p_func: GDExtensionWorkerThreadPoolTask, p_userdata: * mut std::ffi::c_void, p_high_priority: GDExtensionBool, p_description: GDExtensionConstStringPtr) -> i64;
pub type GDExtensionInterfacePackedByteArrayOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> * mut u8;
pub type GDExtensionInterfacePackedByteArrayOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> * const u8;
pub type GDExtensionInterfacePackedFloat32ArrayOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> * mut f32;
pub type GDExtensionInterfacePackedFloat32ArrayOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> * const f32;
pub type GDExtensionInterfacePackedFloat64ArrayOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> * mut f64;
pub type GDExtensionInterfacePackedFloat64ArrayOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> * const f64;
pub type GDExtensionInterfacePackedInt32ArrayOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> * mut i32;
pub type GDExtensionInterfacePackedInt32ArrayOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> * const i32;
pub type GDExtensionInterfacePackedInt64ArrayOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> * mut i64;
pub type GDExtensionInterfacePackedInt64ArrayOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> * const i64;
pub type GDExtensionInterfacePackedStringArrayOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionStringPtr;
pub type GDExtensionInterfacePackedStringArrayOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionStringPtr;
pub type GDExtensionInterfacePackedVector2ArrayOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;
pub type GDExtensionInterfacePackedVector2ArrayOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;
pub type GDExtensionInterfacePackedVector3ArrayOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;
pub type GDExtensionInterfacePackedVector3ArrayOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfacePackedVector4ArrayOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfacePackedVector4ArrayOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;
pub type GDExtensionInterfacePackedColorArrayOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;
pub type GDExtensionInterfacePackedColorArrayOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionTypePtr;
pub type GDExtensionInterfaceArrayOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_index: GDExtensionInt) -> GDExtensionVariantPtr;
pub type GDExtensionInterfaceArrayOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_index: GDExtensionInt) -> GDExtensionVariantPtr;
pub type GDExtensionInterfaceArrayRef = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_from: GDExtensionConstTypePtr);
pub type GDExtensionInterfaceArraySetTyped = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_type: GDExtensionVariantType, p_class_name: GDExtensionConstStringNamePtr, p_script: GDExtensionConstVariantPtr);
pub type GDExtensionInterfaceDictionaryOperatorIndex = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_key: GDExtensionConstVariantPtr) -> GDExtensionVariantPtr;
pub type GDExtensionInterfaceDictionaryOperatorIndexConst = unsafe extern "C" fn(p_self: GDExtensionConstTypePtr, p_key: GDExtensionConstVariantPtr) -> GDExtensionVariantPtr;
#[cfg(since_api = "4.4")]
pub type GDExtensionInterfaceDictionarySetTyped = unsafe extern "C" fn(p_self: GDExtensionTypePtr, p_key_type: GDExtensionVariantType, p_key_class_name: GDExtensionConstStringNamePtr, p_key_script: GDExtensionConstVariantPtr, p_value_type: GDExtensionVariantType, p_value_class_name: GDExtensionConstStringNamePtr, p_value_script: GDExtensionConstVariantPtr);
pub type GDExtensionInterfaceObjectMethodBindCall = unsafe extern "C" fn(p_method_bind: GDExtensionMethodBindPtr, p_instance: GDExtensionObjectPtr, p_args: * const GDExtensionConstVariantPtr, p_arg_count: GDExtensionInt, r_ret: GDExtensionUninitializedVariantPtr, r_error: * mut GDExtensionCallError);
pub type GDExtensionInterfaceObjectMethodBindPtrcall = unsafe extern "C" fn(p_method_bind: GDExtensionMethodBindPtr, p_instance: GDExtensionObjectPtr, p_args: * const GDExtensionConstTypePtr, r_ret: GDExtensionTypePtr);
pub type GDExtensionInterfaceObjectDestroy = unsafe extern "C" fn(p_o: GDExtensionObjectPtr);
pub type GDExtensionInterfaceGlobalGetSingleton = unsafe extern "C" fn(p_name: GDExtensionConstStringNamePtr) -> GDExtensionObjectPtr;
pub type GDExtensionInterfaceObjectGetInstanceBinding = unsafe extern "C" fn(p_o: GDExtensionObjectPtr, p_token: * mut std::ffi::c_void, p_callbacks: * const GDExtensionInstanceBindingCallbacks) -> * mut std::ffi::c_void;
pub type GDExtensionInterfaceObjectSetInstanceBinding = unsafe extern "C" fn(p_o: GDExtensionObjectPtr, p_token: * mut std::ffi::c_void, p_binding: * mut std::ffi::c_void, p_callbacks: * const GDExtensionInstanceBindingCallbacks);
pub type GDExtensionInterfaceObjectFreeInstanceBinding = unsafe extern "C" fn(p_o: GDExtensionObjectPtr, p_token: * mut std::ffi::c_void);
pub type GDExtensionInterfaceObjectSetInstance = unsafe extern "C" fn(p_o: GDExtensionObjectPtr, p_classname: GDExtensionConstStringNamePtr, p_instance: GDExtensionClassInstancePtr);
pub type GDExtensionInterfaceObjectGetClassName = unsafe extern "C" fn(p_object: GDExtensionConstObjectPtr, p_library: GDExtensionClassLibraryPtr, r_class_name: GDExtensionUninitializedStringNamePtr) -> GDExtensionBool;
pub type GDExtensionInterfaceObjectCastTo = unsafe extern "C" fn(p_object: GDExtensionConstObjectPtr, p_class_tag: * mut std::ffi::c_void) -> GDExtensionObjectPtr;
pub type GDExtensionInterfaceObjectGetInstanceFromId = unsafe extern "C" fn(p_instance_id: GDObjectInstanceID) -> GDExtensionObjectPtr;
pub type GDExtensionInterfaceObjectGetInstanceId = unsafe extern "C" fn(p_object: GDExtensionConstObjectPtr) -> GDObjectInstanceID;
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceObjectHasScriptMethod = unsafe extern "C" fn(p_object: GDExtensionConstObjectPtr, p_method: GDExtensionConstStringNamePtr) -> GDExtensionBool;
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceObjectCallScriptMethod = unsafe extern "C" fn(p_object: GDExtensionObjectPtr, p_method: GDExtensionConstStringNamePtr, p_args: * const GDExtensionConstVariantPtr, p_argument_count: GDExtensionInt, r_return: GDExtensionUninitializedVariantPtr, r_error: * mut GDExtensionCallError);
pub type GDExtensionInterfaceRefGetObject = unsafe extern "C" fn(p_ref: GDExtensionConstRefPtr) -> GDExtensionObjectPtr;
pub type GDExtensionInterfaceRefSetObject = unsafe extern "C" fn(p_ref: GDExtensionRefPtr, p_object: GDExtensionObjectPtr);
pub type GDExtensionInterfaceScriptInstanceCreate = unsafe extern "C" fn(p_info: * const GDExtensionScriptInstanceInfo, p_instance_data: GDExtensionScriptInstanceDataPtr) -> GDExtensionScriptInstancePtr;
pub type GDExtensionInterfaceScriptInstanceCreate2 = unsafe extern "C" fn(p_info: * const GDExtensionScriptInstanceInfo2, p_instance_data: GDExtensionScriptInstanceDataPtr) -> GDExtensionScriptInstancePtr;
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceScriptInstanceCreate3 = unsafe extern "C" fn(p_info: * const GDExtensionScriptInstanceInfo3, p_instance_data: GDExtensionScriptInstanceDataPtr) -> GDExtensionScriptInstancePtr;
pub type GDExtensionInterfacePlaceholderScriptInstanceCreate = unsafe extern "C" fn(p_language: GDExtensionObjectPtr, p_script: GDExtensionObjectPtr, p_owner: GDExtensionObjectPtr) -> GDExtensionScriptInstancePtr;
pub type GDExtensionInterfacePlaceholderScriptInstanceUpdate = unsafe extern "C" fn(p_placeholder: GDExtensionScriptInstancePtr, p_properties: GDExtensionConstTypePtr, p_values: GDExtensionConstTypePtr);
pub type GDExtensionInterfaceObjectGetScriptInstance = unsafe extern "C" fn(p_object: GDExtensionConstObjectPtr, p_language: GDExtensionObjectPtr) -> GDExtensionScriptInstanceDataPtr;
#[cfg(since_api = "4.5")]
pub type GDExtensionInterfaceObjectSetScriptInstance = unsafe extern "C" fn(p_object: GDExtensionObjectPtr, p_script_instance: GDExtensionScriptInstanceDataPtr);
pub type GDExtensionInterfaceCallableCustomCreate = unsafe extern "C" fn(r_callable: GDExtensionUninitializedTypePtr, p_callable_custom_info: * mut GDExtensionCallableCustomInfo);
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceCallableCustomCreate2 = unsafe extern "C" fn(r_callable: GDExtensionUninitializedTypePtr, p_callable_custom_info: * mut GDExtensionCallableCustomInfo2);
pub type GDExtensionInterfaceCallableCustomGetUserdata = unsafe extern "C" fn(p_callable: GDExtensionConstTypePtr, p_token: * mut std::ffi::c_void) -> * mut std::ffi::c_void;
pub type GDExtensionInterfaceClassdbConstructObject = unsafe extern "C" fn(p_classname: GDExtensionConstStringNamePtr) -> GDExtensionObjectPtr;
#[cfg(since_api = "4.4")]
pub type GDExtensionInterfaceClassdbConstructObject2 = unsafe extern "C" fn(p_classname: GDExtensionConstStringNamePtr) -> GDExtensionObjectPtr;
#[cfg(since_api = "4.7")]
pub type GDExtensionInterfaceClassdbConstructObject3 = unsafe extern "C" fn(p_classname: GDExtensionConstStringNamePtr) -> GDExtensionObjectPtr;
pub type GDExtensionInterfaceClassdbGetMethodBind = unsafe extern "C" fn(p_classname: GDExtensionConstStringNamePtr, p_methodname: GDExtensionConstStringNamePtr, p_hash: GDExtensionInt) -> GDExtensionMethodBindPtr;
pub type GDExtensionInterfaceClassdbGetClassTag = unsafe extern "C" fn(p_classname: GDExtensionConstStringNamePtr) -> * mut std::ffi::c_void;
pub type GDExtensionInterfaceClassdbRegisterExtensionClass = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_parent_class_name: GDExtensionConstStringNamePtr, p_extension_funcs: * const GDExtensionClassCreationInfo);
pub type GDExtensionInterfaceClassdbRegisterExtensionClass2 = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_parent_class_name: GDExtensionConstStringNamePtr, p_extension_funcs: * const GDExtensionClassCreationInfo2);
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceClassdbRegisterExtensionClass3 = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_parent_class_name: GDExtensionConstStringNamePtr, p_extension_funcs: * const GDExtensionClassCreationInfo3);
#[cfg(since_api = "4.4")]
pub type GDExtensionInterfaceClassdbRegisterExtensionClass4 = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_parent_class_name: GDExtensionConstStringNamePtr, p_extension_funcs: * const GDExtensionClassCreationInfo4);
#[cfg(since_api = "4.5")]
pub type GDExtensionInterfaceClassdbRegisterExtensionClass5 = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_parent_class_name: GDExtensionConstStringNamePtr, p_extension_funcs: * const GDExtensionClassCreationInfo5);
#[cfg(since_api = "4.7")]
pub type GDExtensionInterfaceClassdbRegisterExtensionClass6 = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_parent_class_name: GDExtensionConstStringNamePtr, p_extension_funcs: * const GDExtensionClassCreationInfo6);
pub type GDExtensionInterfaceClassdbRegisterExtensionClassMethod = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_method_info: * const GDExtensionClassMethodInfo);
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceClassdbRegisterExtensionClassVirtualMethod = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_method_info: * const GDExtensionClassVirtualMethodInfo);
pub type GDExtensionInterfaceClassdbRegisterExtensionClassIntegerConstant = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_enum_name: GDExtensionConstStringNamePtr, p_constant_name: GDExtensionConstStringNamePtr, p_constant_value: GDExtensionInt, p_is_bitfield: GDExtensionBool);
pub type GDExtensionInterfaceClassdbRegisterExtensionClassProperty = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_info: * const GDExtensionPropertyInfo, p_setter: GDExtensionConstStringNamePtr, p_getter: GDExtensionConstStringNamePtr);
pub type GDExtensionInterfaceClassdbRegisterExtensionClassPropertyIndexed = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_info: * const GDExtensionPropertyInfo, p_setter: GDExtensionConstStringNamePtr, p_getter: GDExtensionConstStringNamePtr, p_index: GDExtensionInt);
pub type GDExtensionInterfaceClassdbRegisterExtensionClassPropertyGroup = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_group_name: GDExtensionConstStringPtr, p_prefix: GDExtensionConstStringPtr);
pub type GDExtensionInterfaceClassdbRegisterExtensionClassPropertySubgroup = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_subgroup_name: GDExtensionConstStringPtr, p_prefix: GDExtensionConstStringPtr);
pub type GDExtensionInterfaceClassdbRegisterExtensionClassSignal = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr, p_signal_name: GDExtensionConstStringNamePtr, p_argument_info: * const GDExtensionPropertyInfo, p_argument_count: GDExtensionInt);
pub type GDExtensionInterfaceClassdbUnregisterExtensionClass = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_class_name: GDExtensionConstStringNamePtr);
pub type GDExtensionInterfaceGetLibraryPath = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, r_path: GDExtensionUninitializedStringPtr);
pub type GDExtensionInterfaceEditorAddPlugin = unsafe extern "C" fn(p_class_name: GDExtensionConstStringNamePtr);
pub type GDExtensionInterfaceEditorRemovePlugin = unsafe extern "C" fn(p_class_name: GDExtensionConstStringNamePtr);
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceEditorHelpLoadXmlFromUtf8Chars = unsafe extern "C" fn(p_data: * const std::ffi::c_char);
#[cfg(since_api = "4.3")]
pub type GDExtensionInterfaceEditorHelpLoadXmlFromUtf8CharsAndLen = unsafe extern "C" fn(p_data: * const std::ffi::c_char, p_size: GDExtensionInt);
#[cfg(since_api = "4.5")]
pub type GDExtensionInterfaceEditorRegisterGetClassesUsedCallback = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_callback: GDExtensionEditorGetClassesUsedCallback);
#[cfg(since_api = "4.5")]
pub type GDExtensionInterfaceRegisterMainLoopCallbacks = unsafe extern "C" fn(p_library: GDExtensionClassLibraryPtr, p_callbacks: * const GDExtensionMainLoopCallbacks);
pub struct GDExtensionInterface {
    #[doc = "Gets the Godot version that the GDExtension was loaded into.\n\n## Parameters\n- `r_godot_version` - A pointer to the structure to write the version information into."]
    pub get_godot_version: GDExtensionInterfaceGetGodotVersion, #[cfg(since_api = "4.5")]
    #[doc = "Gets the Godot version that the GDExtension was loaded into.\n\n## Parameters\n- `r_godot_version` - A pointer to the structure to write the version information into."]
    pub get_godot_version2: GDExtensionInterfaceGetGodotVersion2, #[doc = "Allocates memory.\n\n## Parameters\n- `p_bytes` - The amount of memory to allocate in bytes.\n\n## Return value\nA pointer to the allocated memory, or NULL if unsuccessful."]
    pub mem_alloc: GDExtensionInterfaceMemAlloc, #[doc = "Reallocates memory.\n\n## Parameters\n- `p_ptr` - A pointer to the previously allocated memory.\n- `p_bytes` - The number of bytes to resize the memory block to.\n\n## Return value\nA pointer to the allocated memory, or NULL if unsuccessful."]
    pub mem_realloc: GDExtensionInterfaceMemRealloc, #[doc = "Frees memory.\n\n## Parameters\n- `p_ptr` - A pointer to the previously allocated memory."]
    pub mem_free: GDExtensionInterfaceMemFree, #[cfg(since_api = "4.6")]
    #[doc = "Allocates memory.\n\n## Parameters\n- `p_bytes` - The amount of memory to allocate in bytes.\n- `p_pad_align` - If true, the returned memory will have prepadding of at least 8 bytes.\n\n## Return value\nA pointer to the allocated memory, or NULL if unsuccessful."]
    pub mem_alloc2: GDExtensionInterfaceMemAlloc2, #[cfg(since_api = "4.6")]
    #[doc = "Reallocates memory.\n\n## Parameters\n- `p_ptr` - A pointer to the previously allocated memory.\n- `p_bytes` - The number of bytes to resize the memory block to.\n- `p_pad_align` - If true, the returned memory will have prepadding of at least 8 bytes.\n\n## Return value\nA pointer to the allocated memory, or NULL if unsuccessful."]
    pub mem_realloc2: GDExtensionInterfaceMemRealloc2, #[cfg(since_api = "4.6")]
    #[doc = "Frees memory.\n\n## Parameters\n- `p_ptr` - A pointer to the previously allocated memory.\n- `p_pad_align` - If true, the given memory was allocated with prepadding."]
    pub mem_free2: GDExtensionInterfaceMemFree2, #[doc = "Logs an error to Godot's built-in debugger and to the OS terminal.\n\n## Parameters\n- `p_description` - The code triggering the error.\n- `p_function` - The function name where the error occurred.\n- `p_file` - The file where the error occurred.\n- `p_line` - The line where the error occurred.\n- `p_editor_notify` - Whether or not to notify the editor."]
    pub print_error: GDExtensionInterfacePrintError, #[doc = "Logs an error with a message to Godot's built-in debugger and to the OS terminal.\n\n## Parameters\n- `p_description` - The code triggering the error.\n- `p_message` - The message to show along with the error.\n- `p_function` - The function name where the error occurred.\n- `p_file` - The file where the error occurred.\n- `p_line` - The line where the error occurred.\n- `p_editor_notify` - Whether or not to notify the editor."]
    pub print_error_with_message: GDExtensionInterfacePrintErrorWithMessage, #[doc = "Logs a warning to Godot's built-in debugger and to the OS terminal.\n\n## Parameters\n- `p_description` - The code triggering the warning.\n- `p_function` - The function name where the warning occurred.\n- `p_file` - The file where the warning occurred.\n- `p_line` - The line where the warning occurred.\n- `p_editor_notify` - Whether or not to notify the editor."]
    pub print_warning: GDExtensionInterfacePrintWarning, #[doc = "Logs a warning with a message to Godot's built-in debugger and to the OS terminal.\n\n## Parameters\n- `p_description` - The code triggering the warning.\n- `p_message` - The message to show along with the warning.\n- `p_function` - The function name where the warning occurred.\n- `p_file` - The file where the warning occurred.\n- `p_line` - The line where the warning occurred.\n- `p_editor_notify` - Whether or not to notify the editor."]
    pub print_warning_with_message: GDExtensionInterfacePrintWarningWithMessage, #[doc = "Logs a script error to Godot's built-in debugger and to the OS terminal.\n\n## Parameters\n- `p_description` - The code triggering the error.\n- `p_function` - The function name where the error occurred.\n- `p_file` - The file where the error occurred.\n- `p_line` - The line where the error occurred.\n- `p_editor_notify` - Whether or not to notify the editor."]
    pub print_script_error: GDExtensionInterfacePrintScriptError, #[doc = "Logs a script error with a message to Godot's built-in debugger and to the OS terminal.\n\n## Parameters\n- `p_description` - The code triggering the error.\n- `p_message` - The message to show along with the error.\n- `p_function` - The function name where the error occurred.\n- `p_file` - The file where the error occurred.\n- `p_line` - The line where the error occurred.\n- `p_editor_notify` - Whether or not to notify the editor."]
    pub print_script_error_with_message: GDExtensionInterfacePrintScriptErrorWithMessage, #[doc = "Gets the size of a native struct (ex. ObjectID) in bytes.\n\n## Parameters\n- `p_name` - A pointer to a StringName identifying the struct name.\n\n## Return value\nThe size in bytes."]
    pub get_native_struct_size: GDExtensionInterfaceGetNativeStructSize, #[doc = "Copies one Variant into a another.\n\n## Parameters\n- `r_dest` - A pointer to the destination Variant.\n- `p_src` - A pointer to the source Variant."]
    pub variant_new_copy: GDExtensionInterfaceVariantNewCopy, #[doc = "Creates a new Variant containing nil.\n\n## Parameters\n- `r_dest` - A pointer to the destination Variant."]
    pub variant_new_nil: GDExtensionInterfaceVariantNewNil, #[doc = "Destroys a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant to destroy."]
    pub variant_destroy: GDExtensionInterfaceVariantDestroy, #[doc = "Calls a method on a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_method` - A pointer to a StringName identifying the method.\n- `p_args` - A pointer to a C array of Variant.\n- `p_argument_count` - The number of arguments.\n- `r_return` - A pointer a Variant which will be assigned the return value.\n- `r_error` - A pointer the structure which will hold error information."]
    pub variant_call: GDExtensionInterfaceVariantCall, #[doc = "Calls a static method on a Variant.\n\n## Parameters\n- `p_type` - The variant type.\n- `p_method` - A pointer to a StringName identifying the method.\n- `p_args` - A pointer to a C array of Variant.\n- `p_argument_count` - The number of arguments.\n- `r_return` - A pointer a Variant which will be assigned the return value.\n- `r_error` - A pointer the structure which will be updated with error information."]
    pub variant_call_static: GDExtensionInterfaceVariantCallStatic, #[doc = "Evaluate an operator on two Variants.\n\n## Parameters\n- `p_op` - The operator to evaluate.\n- `p_a` - The first Variant.\n- `p_b` - The second Variant.\n- `r_return` - A pointer a Variant which will be assigned the return value.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid."]
    pub variant_evaluate: GDExtensionInterfaceVariantEvaluate, #[doc = "Sets a key on a Variant to a value.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_key` - A pointer to a Variant representing the key.\n- `p_value` - A pointer to a Variant representing the value.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid."]
    pub variant_set: GDExtensionInterfaceVariantSet, #[doc = "Sets a named key on a Variant to a value.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_key` - A pointer to a StringName representing the key.\n- `p_value` - A pointer to a Variant representing the value.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid."]
    pub variant_set_named: GDExtensionInterfaceVariantSetNamed, #[doc = "Sets a keyed property on a Variant to a value.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_key` - A pointer to a Variant representing the key.\n- `p_value` - A pointer to a Variant representing the value.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid."]
    pub variant_set_keyed: GDExtensionInterfaceVariantSetKeyed, #[doc = "Sets an index on a Variant to a value.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_index` - The index.\n- `p_value` - A pointer to a Variant representing the value.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid.\n- `r_oob` - A pointer to a boolean which will be set to true if the index is out of bounds."]
    pub variant_set_indexed: GDExtensionInterfaceVariantSetIndexed, #[doc = "Gets the value of a key from a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_key` - A pointer to a Variant representing the key.\n- `r_ret` - A pointer to a Variant which will be assigned the value.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid."]
    pub variant_get: GDExtensionInterfaceVariantGet, #[doc = "Gets the value of a named key from a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_key` - A pointer to a StringName representing the key.\n- `r_ret` - A pointer to a Variant which will be assigned the value.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid."]
    pub variant_get_named: GDExtensionInterfaceVariantGetNamed, #[doc = "Gets the value of a keyed property from a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_key` - A pointer to a Variant representing the key.\n- `r_ret` - A pointer to a Variant which will be assigned the value.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid."]
    pub variant_get_keyed: GDExtensionInterfaceVariantGetKeyed, #[doc = "Gets the value of an index from a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_index` - The index.\n- `r_ret` - A pointer to a Variant which will be assigned the value.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid.\n- `r_oob` - A pointer to a boolean which will be set to true if the index is out of bounds."]
    pub variant_get_indexed: GDExtensionInterfaceVariantGetIndexed, #[doc = "Initializes an iterator over a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `r_iter` - A pointer to a Variant which will be assigned the iterator.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid.\n\n## Return value\ntrue if the operation is valid; otherwise false."]
    pub variant_iter_init: GDExtensionInterfaceVariantIterInit, #[doc = "Gets the next value for an iterator over a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `r_iter` - A pointer to a Variant which will be assigned the iterator.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid.\n\n## Return value\ntrue if the operation is valid; otherwise false."]
    pub variant_iter_next: GDExtensionInterfaceVariantIterNext, #[doc = "Gets the next value for an iterator over a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `r_iter` - A pointer to a Variant which will be assigned the iterator.\n- `r_ret` - A pointer to a Variant which will be assigned false if the operation is invalid.\n- `r_valid` - A pointer to a boolean which will be set to false if the operation is invalid."]
    pub variant_iter_get: GDExtensionInterfaceVariantIterGet, #[doc = "Gets the hash of a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n\n## Return value\nThe hash value."]
    pub variant_hash: GDExtensionInterfaceVariantHash, #[doc = "Gets the recursive hash of a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_recursion_count` - The number of recursive loops so far.\n\n## Return value\nThe hash value."]
    pub variant_recursive_hash: GDExtensionInterfaceVariantRecursiveHash, #[doc = "Compares two Variants by their hash.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_other` - A pointer to the other Variant to compare it to.\n\n## Return value\nThe hash value."]
    pub variant_hash_compare: GDExtensionInterfaceVariantHashCompare, #[doc = "Converts a Variant to a boolean.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n\n## Return value\nThe boolean value of the Variant."]
    pub variant_booleanize: GDExtensionInterfaceVariantBooleanize, #[doc = "Duplicates a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `r_ret` - A pointer to a Variant to store the duplicated value.\n- `p_deep` - Whether or not to duplicate deeply (when supported by the Variant type)."]
    pub variant_duplicate: GDExtensionInterfaceVariantDuplicate, #[doc = "Converts a Variant to a string.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `r_ret` - A pointer to a String to store the resulting value."]
    pub variant_stringify: GDExtensionInterfaceVariantStringify, #[doc = "Gets the type of a Variant.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n\n## Return value\nThe variant type."]
    pub variant_get_type: GDExtensionInterfaceVariantGetType, #[doc = "Checks if a Variant has the given method.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_method` - A pointer to a StringName with the method name.\n\n## Return value\ntrue if the variant has the given method; otherwise false."]
    pub variant_has_method: GDExtensionInterfaceVariantHasMethod, #[doc = "Checks if a type of Variant has the given member.\n\n## Parameters\n- `p_type` - The Variant type.\n- `p_member` - A pointer to a StringName with the member name.\n\n## Return value\ntrue if the variant has the given method; otherwise false."]
    pub variant_has_member: GDExtensionInterfaceVariantHasMember, #[doc = "Checks if a Variant has a key.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n- `p_key` - A pointer to a Variant representing the key.\n- `r_valid` - A pointer to a boolean which will be set to false if the key doesn't exist.\n\n## Return value\ntrue if the key exists; otherwise false."]
    pub variant_has_key: GDExtensionInterfaceVariantHasKey, #[cfg(since_api = "4.4")]
    #[doc = "Gets the object instance ID from a variant of type GDEXTENSION_VARIANT_TYPE_OBJECT.\nIf the variant isn't of type GDEXTENSION_VARIANT_TYPE_OBJECT, then zero will be returned.\nThe instance ID will be returned even if the object is no longer valid - use `object_get_instance_by_id()` to check if the object is still valid.\n\n## Parameters\n- `p_self` - A pointer to the Variant.\n\n## Return value\nThe instance ID for the contained object."]
    pub variant_get_object_instance_id: GDExtensionInterfaceVariantGetObjectInstanceId, #[doc = "Gets the name of a Variant type.\n\n## Parameters\n- `p_type` - The Variant type.\n- `r_name` - A pointer to a String to store the Variant type name."]
    pub variant_get_type_name: GDExtensionInterfaceVariantGetTypeName, #[cfg(since_api = "4.7")]
    #[doc = "Gets the Variant type by name.\n\n## Parameters\n- `p_type_name` - The variant type name.\n\n## Return value\nThe variant type for the given name; otherwise VARIANT_MAX if name is invalid."]
    pub variant_get_type_by_name: GDExtensionInterfaceVariantGetTypeByName, #[doc = "Checks if Variants can be converted from one type to another.\n\n## Parameters\n- `p_from` - The Variant type to convert from.\n- `p_to` - The Variant type to convert to.\n\n## Return value\ntrue if the conversion is possible; otherwise false."]
    pub variant_can_convert: GDExtensionInterfaceVariantCanConvert, #[doc = "Checks if Variant can be converted from one type to another using stricter rules.\n\n## Parameters\n- `p_from` - The Variant type to convert from.\n- `p_to` - The Variant type to convert to.\n\n## Return value\ntrue if the conversion is possible; otherwise false."]
    pub variant_can_convert_strict: GDExtensionInterfaceVariantCanConvertStrict, #[doc = "Gets a pointer to a function that can create a Variant of the given type from a raw value.\n\n## Parameters\n- `p_type` - The Variant type.\n\n## Return value\nA pointer to a function that can create a Variant of the given type from a raw value."]
    pub get_variant_from_type_constructor: GDExtensionInterfaceGetVariantFromTypeConstructor, #[doc = "Gets a pointer to a function that can get the raw value from a Variant of the given type.\n\n## Parameters\n- `p_type` - The Variant type.\n\n## Return value\nA pointer to a function that can get the raw value from a Variant of the given type."]
    pub get_variant_to_type_constructor: GDExtensionInterfaceGetVariantToTypeConstructor, #[cfg(since_api = "4.4")]
    #[doc = "Provides a function pointer for retrieving a pointer to a variant's internal value.\nAccess to a variant's internal value can be used to modify it in-place, or to retrieve its value without the overhead of variant conversion functions.\nIt is recommended to cache the getter for all variant types in a function table to avoid retrieval overhead upon use.\n\nEach function assumes the variant's type has already been determined and matches the function.\nInvoking the function with a variant of a mismatched type has undefined behavior, and may lead to a segmentation fault.\n\n## Parameters\n- `p_type` - The Variant type.\n\n## Return value\nA pointer to a type-specific function that returns a pointer to the internal value of a variant. Check the implementation of this function (gdextension_variant_get_ptr_internal_getter) for pointee type info of each variant type."]
    pub variant_get_ptr_internal_getter: GDExtensionInterfaceVariantGetPtrInternalGetter, #[doc = "Gets a pointer to a function that can evaluate the given Variant operator on the given Variant types.\n\n## Parameters\n- `p_operator` - The variant operator.\n- `p_type_a` - The type of the first Variant.\n- `p_type_b` - The type of the second Variant.\n\n## Return value\nA pointer to a function that can evaluate the given Variant operator on the given Variant types."]
    pub variant_get_ptr_operator_evaluator: GDExtensionInterfaceVariantGetPtrOperatorEvaluator, #[doc = "Gets a pointer to a function that can call a builtin method on a type of Variant.\n\n## Parameters\n- `p_type` - The Variant type.\n- `p_method` - A pointer to a StringName with the method name.\n- `p_hash` - A hash representing the method signature.\n\n## Return value\nA pointer to a function that can call a builtin method on a type of Variant."]
    pub variant_get_ptr_builtin_method: GDExtensionInterfaceVariantGetPtrBuiltinMethod, #[doc = "Gets a pointer to a function that can call one of the constructors for a type of Variant.\n\n## Parameters\n- `p_type` - The Variant type.\n- `p_constructor` - The index of the constructor.\n\n## Return value\nA pointer to a function that can call one of the constructors for a type of Variant."]
    pub variant_get_ptr_constructor: GDExtensionInterfaceVariantGetPtrConstructor, #[doc = "Gets a pointer to a function than can call the destructor for a type of Variant.\n\n## Parameters\n- `p_type` - The Variant type.\n\n## Return value\nA pointer to a function than can call the destructor for a type of Variant."]
    pub variant_get_ptr_destructor: GDExtensionInterfaceVariantGetPtrDestructor, #[doc = "Constructs a Variant of the given type, using the first constructor that matches the given arguments.\n\n## Parameters\n- `p_type` - The Variant type.\n- `r_base` - A pointer to a Variant to store the constructed value.\n- `p_args` - A pointer to a C array of Variant pointers representing the arguments for the constructor.\n- `p_argument_count` - The number of arguments to pass to the constructor.\n- `r_error` - A pointer the structure which will be updated with error information."]
    pub variant_construct: GDExtensionInterfaceVariantConstruct, #[doc = "Gets a pointer to a function that can call a member's setter on the given Variant type.\n\n## Parameters\n- `p_type` - The Variant type.\n- `p_member` - A pointer to a StringName with the member name.\n\n## Return value\nA pointer to a function that can call a member's setter on the given Variant type."]
    pub variant_get_ptr_setter: GDExtensionInterfaceVariantGetPtrSetter, #[doc = "Gets a pointer to a function that can call a member's getter on the given Variant type.\n\n## Parameters\n- `p_type` - The Variant type.\n- `p_member` - A pointer to a StringName with the member name.\n\n## Return value\nA pointer to a function that can call a member's getter on the given Variant type."]
    pub variant_get_ptr_getter: GDExtensionInterfaceVariantGetPtrGetter, #[doc = "Gets a pointer to a function that can set an index on the given Variant type.\n\n## Parameters\n- `p_type` - The Variant type.\n\n## Return value\nA pointer to a function that can set an index on the given Variant type."]
    pub variant_get_ptr_indexed_setter: GDExtensionInterfaceVariantGetPtrIndexedSetter, #[doc = "Gets a pointer to a function that can get an index on the given Variant type.\n\n## Parameters\n- `p_type` - The Variant type.\n\n## Return value\nA pointer to a function that can get an index on the given Variant type."]
    pub variant_get_ptr_indexed_getter: GDExtensionInterfaceVariantGetPtrIndexedGetter, #[doc = "Gets a pointer to a function that can set a key on the given Variant type.\n\n## Parameters\n- `p_type` - The Variant type.\n\n## Return value\nA pointer to a function that can set a key on the given Variant type."]
    pub variant_get_ptr_keyed_setter: GDExtensionInterfaceVariantGetPtrKeyedSetter, #[doc = "Gets a pointer to a function that can get a key on the given Variant type.\n\n## Parameters\n- `p_type` - The Variant type.\n\n## Return value\nA pointer to a function that can get a key on the given Variant type."]
    pub variant_get_ptr_keyed_getter: GDExtensionInterfaceVariantGetPtrKeyedGetter, #[doc = "Gets a pointer to a function that can check a key on the given Variant type.\n\n## Parameters\n- `p_type` - The Variant type.\n\n## Return value\nA pointer to a function that can check a key on the given Variant type."]
    pub variant_get_ptr_keyed_checker: GDExtensionInterfaceVariantGetPtrKeyedChecker, #[doc = "Gets the value of a constant from the given Variant type.\n\n## Parameters\n- `p_type` - The Variant type.\n- `p_constant` - A pointer to a StringName with the constant name.\n- `r_ret` - A pointer to a Variant to store the value."]
    pub variant_get_constant_value: GDExtensionInterfaceVariantGetConstantValue, #[doc = "Gets a pointer to a function that can call a Variant utility function.\n\n## Parameters\n- `p_function` - A pointer to a StringName with the function name.\n- `p_hash` - A hash representing the function signature.\n\n## Return value\nA pointer to a function that can call a Variant utility function."]
    pub variant_get_ptr_utility_function: GDExtensionInterfaceVariantGetPtrUtilityFunction, #[doc = "Creates a String from a Latin-1 encoded C string.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a Latin-1 encoded C string (null terminated)."]
    pub string_new_with_latin1_chars: GDExtensionInterfaceStringNewWithLatin1Chars, #[doc = "Creates a String from a UTF-8 encoded C string.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a UTF-8 encoded C string (null terminated)."]
    pub string_new_with_utf8_chars: GDExtensionInterfaceStringNewWithUtf8Chars, #[doc = "Creates a String from a UTF-16 encoded C string.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a UTF-16 encoded C string (null terminated)."]
    pub string_new_with_utf16_chars: GDExtensionInterfaceStringNewWithUtf16Chars, #[doc = "Creates a String from a UTF-32 encoded C string.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a UTF-32 encoded C string (null terminated)."]
    pub string_new_with_utf32_chars: GDExtensionInterfaceStringNewWithUtf32Chars, #[doc = "Creates a String from a wide C string.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a wide C string (null terminated)."]
    pub string_new_with_wide_chars: GDExtensionInterfaceStringNewWithWideChars, #[doc = "Creates a String from a Latin-1 encoded C string with the given length.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a Latin-1 encoded C string.\n- `p_size` - The number of characters (= number of bytes)."]
    pub string_new_with_latin1_chars_and_len: GDExtensionInterfaceStringNewWithLatin1CharsAndLen, #[doc = "Creates a String from a UTF-8 encoded C string with the given length.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a UTF-8 encoded C string.\n- `p_size` - The number of bytes (not code units)."]
    pub string_new_with_utf8_chars_and_len: GDExtensionInterfaceStringNewWithUtf8CharsAndLen, #[cfg(since_api = "4.3")]
    #[doc = "Creates a String from a UTF-8 encoded C string with the given length.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a UTF-8 encoded C string.\n- `p_size` - The number of bytes (not code units).\n\n## Return value\nError code signifying if the operation successful."]
    pub string_new_with_utf8_chars_and_len2: GDExtensionInterfaceStringNewWithUtf8CharsAndLen2, #[doc = "Creates a String from a UTF-16 encoded C string with the given length.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a UTF-16 encoded C string.\n- `p_char_count` - The number of characters (not bytes)."]
    pub string_new_with_utf16_chars_and_len: GDExtensionInterfaceStringNewWithUtf16CharsAndLen, #[cfg(since_api = "4.3")]
    #[doc = "Creates a String from a UTF-16 encoded C string with the given length.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a UTF-16 encoded C string.\n- `p_char_count` - The number of characters (not bytes).\n- `p_default_little_endian` - If true, UTF-16 use little endian.\n\n## Return value\nError code signifying if the operation successful."]
    pub string_new_with_utf16_chars_and_len2: GDExtensionInterfaceStringNewWithUtf16CharsAndLen2, #[doc = "Creates a String from a UTF-32 encoded C string with the given length.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a UTF-32 encoded C string.\n- `p_char_count` - The number of characters (not bytes)."]
    pub string_new_with_utf32_chars_and_len: GDExtensionInterfaceStringNewWithUtf32CharsAndLen, #[doc = "Creates a String from a wide C string with the given length.\n\n## Parameters\n- `r_dest` - A pointer to a Variant to hold the newly created String.\n- `p_contents` - A pointer to a wide C string.\n- `p_char_count` - The number of characters (not bytes)."]
    pub string_new_with_wide_chars_and_len: GDExtensionInterfaceStringNewWithWideCharsAndLen, #[doc = "Converts a String to a Latin-1 encoded C string.\nIt doesn't write a null terminator.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `r_text` - A pointer to the buffer to hold the resulting data. If NULL is passed in, only the length will be computed.\n- `p_max_write_length` - The maximum number of characters that can be written to r_text. It has no affect on the return value.\n\n## Return value\nThe resulting encoded string length in characters, not including a null terminator. Characters that cannot be converted to Latin-1 are replaced with a space."]
    pub string_to_latin1_chars: GDExtensionInterfaceStringToLatin1Chars, #[doc = "Converts a String to a UTF-8 encoded C string.\nIt doesn't write a null terminator.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `r_text` - A pointer to the buffer to hold the resulting data. If NULL is passed in, only the length will be computed.\n- `p_max_write_length` - The maximum number of characters that can be written to r_text. It has no affect on the return value.\n\n## Return value\nThe resulting encoded string length in bytes (not characters), not including a null terminator."]
    pub string_to_utf8_chars: GDExtensionInterfaceStringToUtf8Chars, #[doc = "Converts a String to a UTF-16 encoded C string.\nIt doesn't write a null terminator.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `r_text` - A pointer to the buffer to hold the resulting data. If NULL is passed in, only the length will be computed.\n- `p_max_write_length` - The maximum number of characters that can be written to r_text. It has no affect on the return value.\n\n## Return value\nThe resulting encoded string length in 16-bit code units (not bytes or characters), not including a null terminator."]
    pub string_to_utf16_chars: GDExtensionInterfaceStringToUtf16Chars, #[doc = "Converts a String to a UTF-32 encoded C string.\nIt doesn't write a null terminator.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `r_text` - A pointer to the buffer to hold the resulting data. If NULL is passed in, only the length will be computed.\n- `p_max_write_length` - The maximum number of characters that can be written to r_text. It has no affect on the return value.\n\n## Return value\nThe resulting encoded string length in characters (not bytes), not including a null terminator."]
    pub string_to_utf32_chars: GDExtensionInterfaceStringToUtf32Chars, #[doc = "Converts a String to a wide C string.\nIt doesn't write a null terminator.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `r_text` - A pointer to the buffer to hold the resulting data. If NULL is passed in, only the length will be computed.\n- `p_max_write_length` - The maximum number of characters that can be written to r_text. It has no affect on the return value.\n\n## Return value\nThe resulting encoded string length in characters (for UTF-32) or 16-bit code units (for UTF-16), depending on the wchar_t representation. Does not include a null terminator."]
    pub string_to_wide_chars: GDExtensionInterfaceStringToWideChars, #[doc = "Gets a pointer to the character at the given index from a String.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `p_index` - The index.\n\n## Return value\nA pointer to the requested character."]
    pub string_operator_index: GDExtensionInterfaceStringOperatorIndex, #[doc = "Gets a const pointer to the character at the given index from a String.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `p_index` - The index.\n\n## Return value\nA const pointer to the requested character."]
    pub string_operator_index_const: GDExtensionInterfaceStringOperatorIndexConst, #[doc = "Appends another String to a String.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `p_b` - A pointer to the other String to append."]
    pub string_operator_plus_eq_string: GDExtensionInterfaceStringOperatorPlusEqString, #[doc = "Appends a character to a String.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `p_b` - A pointer to the character to append."]
    pub string_operator_plus_eq_char: GDExtensionInterfaceStringOperatorPlusEqChar, #[doc = "Appends a Latin-1 encoded C string to a String.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `p_b` - A pointer to a Latin-1 encoded C string (null terminated)."]
    pub string_operator_plus_eq_cstr: GDExtensionInterfaceStringOperatorPlusEqCstr, #[doc = "Appends a wide C string to a String.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `p_b` - A pointer to a wide C string (null terminated)."]
    pub string_operator_plus_eq_wcstr: GDExtensionInterfaceStringOperatorPlusEqWcstr, #[doc = "Appends a UTF-32 encoded C string to a String.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `p_b` - A pointer to a UTF-32 encoded C string (null terminated)."]
    pub string_operator_plus_eq_c32str: GDExtensionInterfaceStringOperatorPlusEqC32str, #[doc = "Resizes the underlying string data to the given number of characters.\nSpace needs to be allocated for the null terminating character ('\\0') which\nalso must be added manually, in order for all string functions to work correctly.\n\nWarning: This is an error-prone operation - only use it if there's no other\nefficient way to accomplish your goal.\n\n## Parameters\n- `p_self` - A pointer to the String.\n- `p_resize` - The new length for the String.\n\n## Return value\nError code signifying if the operation successful."]
    pub string_resize: GDExtensionInterfaceStringResize, #[doc = "Creates a StringName from a Latin-1 encoded C string.\nIf `p_is_static` is true, then:\n- The StringName will reuse the `p_contents` buffer instead of copying it.\n- You must guarantee that the buffer remains valid for the duration of the application (e.g. string literal).\n- You must not call a destructor for this StringName. Incrementing the initial reference once should achieve this.\n\n`p_is_static` is purely an optimization and can easily introduce undefined behavior if used wrong. In case of doubt, set it to false.\n\n## Parameters\n- `r_dest` - A pointer to uninitialized storage, into which the newly created StringName is constructed.\n- `p_contents` - A pointer to a C string (null terminated and Latin-1 or ASCII encoded).\n- `p_is_static` - Whether the StringName reuses the buffer directly (see above)."]
    pub string_name_new_with_latin1_chars: GDExtensionInterfaceStringNameNewWithLatin1Chars, #[doc = "Creates a StringName from a UTF-8 encoded C string.\n\n## Parameters\n- `r_dest` - A pointer to uninitialized storage, into which the newly created StringName is constructed.\n- `p_contents` - A pointer to a C string (null terminated and UTF-8 encoded)."]
    pub string_name_new_with_utf8_chars: GDExtensionInterfaceStringNameNewWithUtf8Chars, #[doc = "Creates a StringName from a UTF-8 encoded string with a given number of characters.\n\n## Parameters\n- `r_dest` - A pointer to uninitialized storage, into which the newly created StringName is constructed.\n- `p_contents` - A pointer to a C string (null terminated and UTF-8 encoded).\n- `p_size` - The number of bytes (not UTF-8 code points)."]
    pub string_name_new_with_utf8_chars_and_len: GDExtensionInterfaceStringNameNewWithUtf8CharsAndLen, #[doc = "Opens a raw XML buffer on an XMLParser instance.\n\n## Parameters\n- `p_instance` - A pointer to an XMLParser object.\n- `p_buffer` - A pointer to the buffer.\n- `p_size` - The size of the buffer.\n\n## Return value\nA Godot error code (ex. OK, ERR_INVALID_DATA, etc)."]
    pub xml_parser_open_buffer: GDExtensionInterfaceXmlParserOpenBuffer, #[doc = "Stores the given buffer using an instance of FileAccess.\n\n## Parameters\n- `p_instance` - A pointer to a FileAccess object.\n- `p_src` - A pointer to the buffer.\n- `p_length` - The size of the buffer."]
    pub file_access_store_buffer: GDExtensionInterfaceFileAccessStoreBuffer, #[doc = "Reads the next p_length bytes into the given buffer using an instance of FileAccess.\n\n## Parameters\n- `p_instance` - A pointer to a FileAccess object.\n- `p_dst` - A pointer to the buffer to store the data.\n- `p_length` - The requested number of bytes to read.\n\n## Return value\nThe actual number of bytes read (may be less than requested)."]
    pub file_access_get_buffer: GDExtensionInterfaceFileAccessGetBuffer, #[cfg(since_api = "4.3")]
    #[doc = "Returns writable pointer to internal Image buffer.\n\n## Parameters\n- `p_instance` - A pointer to a Image object.\n\n## Return value\nPointer to internal Image buffer."]
    pub image_ptrw: GDExtensionInterfaceImagePtrw, #[cfg(since_api = "4.3")]
    #[doc = "Returns read only pointer to internal Image buffer.\n\n## Parameters\n- `p_instance` - A pointer to a Image object.\n\n## Return value\nPointer to internal Image buffer."]
    pub image_ptr: GDExtensionInterfaceImagePtr, #[doc = "Adds a group task to an instance of WorkerThreadPool.\n\n## Parameters\n- `p_instance` - A pointer to a WorkerThreadPool object.\n- `p_func` - A pointer to a function to run in the thread pool.\n- `p_userdata` - A pointer to arbitrary data which will be passed to p_func.\n- `p_elements` - The number of element needed in the group.\n- `p_tasks` - The number of tasks needed in the group.\n- `p_high_priority` - Whether or not this is a high priority task.\n- `p_description` - A pointer to a String with the task description.\n\n## Return value\nThe task group ID."]
    pub worker_thread_pool_add_native_group_task: GDExtensionInterfaceWorkerThreadPoolAddNativeGroupTask, #[doc = "Adds a task to an instance of WorkerThreadPool.\n\n## Parameters\n- `p_instance` - A pointer to a WorkerThreadPool object.\n- `p_func` - A pointer to a function to run in the thread pool.\n- `p_userdata` - A pointer to arbitrary data which will be passed to p_func.\n- `p_high_priority` - Whether or not this is a high priority task.\n- `p_description` - A pointer to a String with the task description.\n\n## Return value\nThe task ID."]
    pub worker_thread_pool_add_native_task: GDExtensionInterfaceWorkerThreadPoolAddNativeTask, #[doc = "Gets a pointer to a byte in a PackedByteArray.\n\n## Parameters\n- `p_self` - A pointer to a PackedByteArray object.\n- `p_index` - The index of the byte to get.\n\n## Return value\nA pointer to the requested byte."]
    pub packed_byte_array_operator_index: GDExtensionInterfacePackedByteArrayOperatorIndex, #[doc = "Gets a const pointer to a byte in a PackedByteArray.\n\n## Parameters\n- `p_self` - A const pointer to a PackedByteArray object.\n- `p_index` - The index of the byte to get.\n\n## Return value\nA const pointer to the requested byte."]
    pub packed_byte_array_operator_index_const: GDExtensionInterfacePackedByteArrayOperatorIndexConst, #[doc = "Gets a pointer to a 32-bit float in a PackedFloat32Array.\n\n## Parameters\n- `p_self` - A pointer to a PackedFloat32Array object.\n- `p_index` - The index of the float to get.\n\n## Return value\nA pointer to the requested 32-bit float."]
    pub packed_float32_array_operator_index: GDExtensionInterfacePackedFloat32ArrayOperatorIndex, #[doc = "Gets a const pointer to a 32-bit float in a PackedFloat32Array.\n\n## Parameters\n- `p_self` - A const pointer to a PackedFloat32Array object.\n- `p_index` - The index of the float to get.\n\n## Return value\nA const pointer to the requested 32-bit float."]
    pub packed_float32_array_operator_index_const: GDExtensionInterfacePackedFloat32ArrayOperatorIndexConst, #[doc = "Gets a pointer to a 64-bit float in a PackedFloat64Array.\n\n## Parameters\n- `p_self` - A pointer to a PackedFloat64Array object.\n- `p_index` - The index of the float to get.\n\n## Return value\nA pointer to the requested 64-bit float."]
    pub packed_float64_array_operator_index: GDExtensionInterfacePackedFloat64ArrayOperatorIndex, #[doc = "Gets a const pointer to a 64-bit float in a PackedFloat64Array.\n\n## Parameters\n- `p_self` - A const pointer to a PackedFloat64Array object.\n- `p_index` - The index of the float to get.\n\n## Return value\nA const pointer to the requested 64-bit float."]
    pub packed_float64_array_operator_index_const: GDExtensionInterfacePackedFloat64ArrayOperatorIndexConst, #[doc = "Gets a pointer to a 32-bit integer in a PackedInt32Array.\n\n## Parameters\n- `p_self` - A pointer to a PackedInt32Array object.\n- `p_index` - The index of the integer to get.\n\n## Return value\nA pointer to the requested 32-bit integer."]
    pub packed_int32_array_operator_index: GDExtensionInterfacePackedInt32ArrayOperatorIndex, #[doc = "Gets a const pointer to a 32-bit integer in a PackedInt32Array.\n\n## Parameters\n- `p_self` - A const pointer to a PackedInt32Array object.\n- `p_index` - The index of the integer to get.\n\n## Return value\nA const pointer to the requested 32-bit integer."]
    pub packed_int32_array_operator_index_const: GDExtensionInterfacePackedInt32ArrayOperatorIndexConst, #[doc = "Gets a pointer to a 64-bit integer in a PackedInt64Array.\n\n## Parameters\n- `p_self` - A pointer to a PackedInt64Array object.\n- `p_index` - The index of the integer to get.\n\n## Return value\nA pointer to the requested 64-bit integer."]
    pub packed_int64_array_operator_index: GDExtensionInterfacePackedInt64ArrayOperatorIndex, #[doc = "Gets a const pointer to a 64-bit integer in a PackedInt64Array.\n\n## Parameters\n- `p_self` - A const pointer to a PackedInt64Array object.\n- `p_index` - The index of the integer to get.\n\n## Return value\nA const pointer to the requested 64-bit integer."]
    pub packed_int64_array_operator_index_const: GDExtensionInterfacePackedInt64ArrayOperatorIndexConst, #[doc = "Gets a pointer to a string in a PackedStringArray.\n\n## Parameters\n- `p_self` - A pointer to a PackedStringArray object.\n- `p_index` - The index of the String to get.\n\n## Return value\nA pointer to the requested String."]
    pub packed_string_array_operator_index: GDExtensionInterfacePackedStringArrayOperatorIndex, #[doc = "Gets a const pointer to a string in a PackedStringArray.\n\n## Parameters\n- `p_self` - A const pointer to a PackedStringArray object.\n- `p_index` - The index of the String to get.\n\n## Return value\nA const pointer to the requested String."]
    pub packed_string_array_operator_index_const: GDExtensionInterfacePackedStringArrayOperatorIndexConst, #[doc = "Gets a pointer to a Vector2 in a PackedVector2Array.\n\n## Parameters\n- `p_self` - A pointer to a PackedVector2Array object.\n- `p_index` - The index of the Vector2 to get.\n\n## Return value\nA pointer to the requested Vector2."]
    pub packed_vector2_array_operator_index: GDExtensionInterfacePackedVector2ArrayOperatorIndex, #[doc = "Gets a const pointer to a Vector2 in a PackedVector2Array.\n\n## Parameters\n- `p_self` - A const pointer to a PackedVector2Array object.\n- `p_index` - The index of the Vector2 to get.\n\n## Return value\nA const pointer to the requested Vector2."]
    pub packed_vector2_array_operator_index_const: GDExtensionInterfacePackedVector2ArrayOperatorIndexConst, #[doc = "Gets a pointer to a Vector3 in a PackedVector3Array.\n\n## Parameters\n- `p_self` - A pointer to a PackedVector3Array object.\n- `p_index` - The index of the Vector3 to get.\n\n## Return value\nA pointer to the requested Vector3."]
    pub packed_vector3_array_operator_index: GDExtensionInterfacePackedVector3ArrayOperatorIndex, #[doc = "Gets a const pointer to a Vector3 in a PackedVector3Array.\n\n## Parameters\n- `p_self` - A const pointer to a PackedVector3Array object.\n- `p_index` - The index of the Vector3 to get.\n\n## Return value\nA const pointer to the requested Vector3."]
    pub packed_vector3_array_operator_index_const: GDExtensionInterfacePackedVector3ArrayOperatorIndexConst, #[cfg(since_api = "4.3")]
    #[doc = "Gets a pointer to a Vector4 in a PackedVector4Array.\n\n## Parameters\n- `p_self` - A pointer to a PackedVector4Array object.\n- `p_index` - The index of the Vector4 to get.\n\n## Return value\nA pointer to the requested Vector4."]
    pub packed_vector4_array_operator_index: GDExtensionInterfacePackedVector4ArrayOperatorIndex, #[cfg(since_api = "4.3")]
    #[doc = "Gets a const pointer to a Vector4 in a PackedVector4Array.\n\n## Parameters\n- `p_self` - A const pointer to a PackedVector4Array object.\n- `p_index` - The index of the Vector4 to get.\n\n## Return value\nA const pointer to the requested Vector4."]
    pub packed_vector4_array_operator_index_const: GDExtensionInterfacePackedVector4ArrayOperatorIndexConst, #[doc = "Gets a pointer to a color in a PackedColorArray.\n\n## Parameters\n- `p_self` - A pointer to a PackedColorArray object.\n- `p_index` - The index of the Color to get.\n\n## Return value\nA pointer to the requested Color."]
    pub packed_color_array_operator_index: GDExtensionInterfacePackedColorArrayOperatorIndex, #[doc = "Gets a const pointer to a color in a PackedColorArray.\n\n## Parameters\n- `p_self` - A const pointer to a PackedColorArray object.\n- `p_index` - The index of the Color to get.\n\n## Return value\nA const pointer to the requested Color."]
    pub packed_color_array_operator_index_const: GDExtensionInterfacePackedColorArrayOperatorIndexConst, #[doc = "Gets a pointer to a Variant in an Array.\n\n## Parameters\n- `p_self` - A pointer to an Array object.\n- `p_index` - The index of the Variant to get.\n\n## Return value\nA pointer to the requested Variant."]
    pub array_operator_index: GDExtensionInterfaceArrayOperatorIndex, #[doc = "Gets a const pointer to a Variant in an Array.\n\n## Parameters\n- `p_self` - A const pointer to an Array object.\n- `p_index` - The index of the Variant to get.\n\n## Return value\nA const pointer to the requested Variant."]
    pub array_operator_index_const: GDExtensionInterfaceArrayOperatorIndexConst, #[doc = "Sets an Array to be a reference to another Array object.\n\n## Parameters\n- `p_self` - A pointer to the Array object to update.\n- `p_from` - A pointer to the Array object to reference."]
    pub array_ref: GDExtensionInterfaceArrayRef, #[doc = "Makes an Array into a typed Array.\n\n## Parameters\n- `p_self` - A pointer to the Array.\n- `p_type` - The type of Variant the Array will store.\n- `p_class_name` - A pointer to a StringName with the name of the object (if p_type is GDEXTENSION_VARIANT_TYPE_OBJECT).\n- `p_script` - A pointer to a Script object (if p_type is GDEXTENSION_VARIANT_TYPE_OBJECT and the base class is extended by a script)."]
    pub array_set_typed: GDExtensionInterfaceArraySetTyped, #[doc = "Gets a pointer to a Variant in a Dictionary with the given key.\n\n## Parameters\n- `p_self` - A pointer to a Dictionary object.\n- `p_key` - A pointer to a Variant representing the key.\n\n## Return value\nA pointer to a Variant representing the value at the given key."]
    pub dictionary_operator_index: GDExtensionInterfaceDictionaryOperatorIndex, #[doc = "Gets a const pointer to a Variant in a Dictionary with the given key.\n\n## Parameters\n- `p_self` - A const pointer to a Dictionary object.\n- `p_key` - A pointer to a Variant representing the key.\n\n## Return value\nA const pointer to a Variant representing the value at the given key."]
    pub dictionary_operator_index_const: GDExtensionInterfaceDictionaryOperatorIndexConst, #[cfg(since_api = "4.4")]
    #[doc = "Makes a Dictionary into a typed Dictionary.\n\n## Parameters\n- `p_self` - A pointer to the Dictionary.\n- `p_key_type` - The type of Variant the Dictionary key will store.\n- `p_key_class_name` - A pointer to a StringName with the name of the object (if p_key_type is GDEXTENSION_VARIANT_TYPE_OBJECT).\n- `p_key_script` - A pointer to a Script object (if p_key_type is GDEXTENSION_VARIANT_TYPE_OBJECT and the base class is extended by a script).\n- `p_value_type` - The type of Variant the Dictionary value will store.\n- `p_value_class_name` - A pointer to a StringName with the name of the object (if p_value_type is GDEXTENSION_VARIANT_TYPE_OBJECT).\n- `p_value_script` - A pointer to a Script object (if p_value_type is GDEXTENSION_VARIANT_TYPE_OBJECT and the base class is extended by a script)."]
    pub dictionary_set_typed: GDExtensionInterfaceDictionarySetTyped, #[doc = "Calls a method on an Object.\n\n## Parameters\n- `p_method_bind` - A pointer to the MethodBind representing the method on the Object's class.\n- `p_instance` - A pointer to the Object.\n- `p_args` - A pointer to a C array of Variants representing the arguments.\n- `p_arg_count` - The number of arguments.\n- `r_ret` - A pointer to Variant which will receive the return value.\n- `r_error` - A pointer to a GDExtensionCallError struct that will receive error information."]
    pub object_method_bind_call: GDExtensionInterfaceObjectMethodBindCall, #[doc = "Calls a method on an Object (using a \"ptrcall\").\n\n## Parameters\n- `p_method_bind` - A pointer to the MethodBind representing the method on the Object's class.\n- `p_instance` - A pointer to the Object.\n- `p_args` - A pointer to a C array representing the arguments.\n- `r_ret` - A pointer to the Object that will receive the return value."]
    pub object_method_bind_ptrcall: GDExtensionInterfaceObjectMethodBindPtrcall, #[doc = "Destroys an Object.\n\n## Parameters\n- `p_o` - A pointer to the Object."]
    pub object_destroy: GDExtensionInterfaceObjectDestroy, #[doc = "Gets a global singleton by name.\n\n## Parameters\n- `p_name` - A pointer to a StringName with the singleton name.\n\n## Return value\nA pointer to the singleton Object."]
    pub global_get_singleton: GDExtensionInterfaceGlobalGetSingleton, #[doc = "Gets a pointer representing an Object's instance binding.\n\n## Parameters\n- `p_o` - A pointer to the Object.\n- `p_token` - A token the library received by the GDExtension's entry point function.\n- `p_callbacks` - A pointer to a GDExtensionInstanceBindingCallbacks struct.\n\n## Return value\nA pointer to the instance binding."]
    pub object_get_instance_binding: GDExtensionInterfaceObjectGetInstanceBinding, #[doc = "Sets an Object's instance binding.\n\n## Parameters\n- `p_o` - A pointer to the Object.\n- `p_token` - A token the library received by the GDExtension's entry point function.\n- `p_binding` - A pointer to the instance binding.\n- `p_callbacks` - A pointer to a GDExtensionInstanceBindingCallbacks struct."]
    pub object_set_instance_binding: GDExtensionInterfaceObjectSetInstanceBinding, #[doc = "Free an Object's instance binding.\n\n## Parameters\n- `p_o` - A pointer to the Object.\n- `p_token` - A token the library received by the GDExtension's entry point function."]
    pub object_free_instance_binding: GDExtensionInterfaceObjectFreeInstanceBinding, #[doc = "Sets an extension class instance on a Object.\n`p_classname` should be a registered extension class and should extend the `p_o` Object's class.\n\n## Parameters\n- `p_o` - A pointer to the Object.\n- `p_classname` - A pointer to a StringName with the registered extension class's name.\n- `p_instance` - A pointer to the extension class instance."]
    pub object_set_instance: GDExtensionInterfaceObjectSetInstance, #[doc = "Gets the class name of an Object.\nIf the GDExtension wraps the Godot object in an abstraction specific to its class, this is the\nfunction that should be used to determine which wrapper to use.\n\n## Parameters\n- `p_object` - A pointer to the Object.\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `r_class_name` - A pointer to a String to receive the class name.\n\n## Return value\ntrue if successful in getting the class name; otherwise false."]
    pub object_get_class_name: GDExtensionInterfaceObjectGetClassName, #[doc = "Casts an Object to a different type.\n\n## Parameters\n- `p_object` - A pointer to the Object.\n- `p_class_tag` - A pointer uniquely identifying a built-in class in the ClassDB.\n\n## Return value\nReturns a pointer to the Object, or NULL if it can't be cast to the requested type."]
    pub object_cast_to: GDExtensionInterfaceObjectCastTo, #[doc = "Gets an Object by its instance ID.\n\n## Parameters\n- `p_instance_id` - The instance ID.\n\n## Return value\nA pointer to the Object."]
    pub object_get_instance_from_id: GDExtensionInterfaceObjectGetInstanceFromId, #[doc = "Gets the instance ID from an Object.\n\n## Parameters\n- `p_object` - A pointer to the Object.\n\n## Return value\nThe instance ID."]
    pub object_get_instance_id: GDExtensionInterfaceObjectGetInstanceId, #[cfg(since_api = "4.3")]
    #[doc = "Checks if this object has a script with the given method.\n\n## Parameters\n- `p_object` - A pointer to the Object.\n- `p_method` - A pointer to a StringName identifying the method.\n\n## Return value\ntrue if the object has a script and that script has a method with the given name. Returns false if the object has no script."]
    pub object_has_script_method: GDExtensionInterfaceObjectHasScriptMethod, #[cfg(since_api = "4.3")]
    #[doc = "Call the given script method on this object.\n\n## Parameters\n- `p_object` - A pointer to the Object.\n- `p_method` - A pointer to a StringName identifying the method.\n- `p_args` - A pointer to a C array of Variant.\n- `p_argument_count` - The number of arguments.\n- `r_return` - A pointer a Variant which will be assigned the return value.\n- `r_error` - A pointer the structure which will hold error information."]
    pub object_call_script_method: GDExtensionInterfaceObjectCallScriptMethod, #[doc = "Gets the Object from a reference.\n\n## Parameters\n- `p_ref` - A pointer to the reference.\n\n## Return value\nA pointer to the Object from the reference or NULL."]
    pub ref_get_object: GDExtensionInterfaceRefGetObject, #[doc = "Sets the Object referred to by a reference.\n\n## Parameters\n- `p_ref` - A pointer to the reference.\n- `p_object` - A pointer to the Object to refer to."]
    pub ref_set_object: GDExtensionInterfaceRefSetObject, #[doc = "Creates a script instance that contains the given info and instance data.\n\n## Parameters\n- `p_info` - A pointer to a GDExtensionScriptInstanceInfo struct.\n- `p_instance_data` - A pointer to a data representing the script instance in the GDExtension. This will be passed to all the function pointers on p_info.\n\n## Return value\nA pointer to a ScriptInstanceExtension object."]
    pub script_instance_create: GDExtensionInterfaceScriptInstanceCreate, #[doc = "Creates a script instance that contains the given info and instance data.\n\n## Parameters\n- `p_info` - A pointer to a GDExtensionScriptInstanceInfo2 struct.\n- `p_instance_data` - A pointer to a data representing the script instance in the GDExtension. This will be passed to all the function pointers on p_info.\n\n## Return value\nA pointer to a ScriptInstanceExtension object."]
    pub script_instance_create2: GDExtensionInterfaceScriptInstanceCreate2, #[cfg(since_api = "4.3")]
    #[doc = "Creates a script instance that contains the given info and instance data.\n\n## Parameters\n- `p_info` - A pointer to a GDExtensionScriptInstanceInfo3 struct.\n- `p_instance_data` - A pointer to a data representing the script instance in the GDExtension. This will be passed to all the function pointers on p_info.\n\n## Return value\nA pointer to a ScriptInstanceExtension object."]
    pub script_instance_create3: GDExtensionInterfaceScriptInstanceCreate3, #[doc = "Creates a placeholder script instance for a given script and instance.\nThis interface is optional as a custom placeholder could also be created with script_instance_create().\n\n## Parameters\n- `p_language` - A pointer to a ScriptLanguage.\n- `p_script` - A pointer to a Script.\n- `p_owner` - A pointer to an Object.\n\n## Return value\nA pointer to a PlaceHolderScriptInstance object."]
    pub placeholder_script_instance_create: GDExtensionInterfacePlaceholderScriptInstanceCreate, #[doc = "Updates a placeholder script instance with the given properties and values.\nThe passed in placeholder must be an instance of PlaceHolderScriptInstance\nsuch as the one returned by placeholder_script_instance_create().\n\n## Parameters\n- `p_placeholder` - A pointer to a PlaceHolderScriptInstance.\n- `p_properties` - A pointer to an Array of Dictionary representing PropertyInfo.\n- `p_values` - A pointer to a Dictionary mapping StringName to Variant values."]
    pub placeholder_script_instance_update: GDExtensionInterfacePlaceholderScriptInstanceUpdate, #[doc = "Get the script instance data attached to this object.\n\n## Parameters\n- `p_object` - A pointer to the Object.\n- `p_language` - A pointer to the language expected for this script instance.\n\n## Return value\nA GDExtensionScriptInstanceDataPtr that was attached to this object as part of script_instance_create."]
    pub object_get_script_instance: GDExtensionInterfaceObjectGetScriptInstance, #[cfg(since_api = "4.5")]
    #[doc = "Set the script instance data attached to this object.\n\n## Parameters\n- `p_object` - A pointer to the Object.\n- `p_script_instance` - A pointer to the script instance data to attach to this object."]
    pub object_set_script_instance: GDExtensionInterfaceObjectSetScriptInstance, #[doc = "Creates a custom Callable object from a function pointer.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `r_callable` - A pointer that will receive the new Callable.\n- `p_callable_custom_info` - The info required to construct a Callable."]
    pub callable_custom_create: GDExtensionInterfaceCallableCustomCreate, #[cfg(since_api = "4.3")]
    #[doc = "Creates a custom Callable object from a function pointer.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `r_callable` - A pointer that will receive the new Callable.\n- `p_callable_custom_info` - The info required to construct a Callable."]
    pub callable_custom_create2: GDExtensionInterfaceCallableCustomCreate2, #[doc = "Retrieves the userdata pointer from a custom Callable.\nIf the Callable is not a custom Callable or the token does not match the one provided to callable_custom_create() via GDExtensionCallableCustomInfo then NULL will be returned.\n\n## Parameters\n- `p_callable` - A pointer to a Callable.\n- `p_token` - A pointer to an address that uniquely identifies the GDExtension.\n\n## Return value\nThe userdata pointer given when creating this custom Callable."]
    pub callable_custom_get_userdata: GDExtensionInterfaceCallableCustomGetUserdata, #[doc = "Constructs an Object of the requested class.\nThe passed class must be a built-in godot class, or an already-registered extension class. In both cases, object_set_instance() should be called to fully initialize the object.\n\n## Parameters\n- `p_classname` - A pointer to a StringName with the class name.\n\n## Return value\nA pointer to the newly created Object."]
    pub classdb_construct_object: GDExtensionInterfaceClassdbConstructObject, #[cfg(since_api = "4.4")]
    #[doc = "Constructs an Object of the requested class.\nThe passed class must be a built-in godot class, or an already-registered extension class. In both cases, object_set_instance() should be called to fully initialize the object.\n\n\"NOTIFICATION_POSTINITIALIZE\" must be sent after construction.\n\n## Parameters\n- `p_classname` - A pointer to a StringName with the class name.\n\n## Return value\nA pointer to the newly created Object."]
    pub classdb_construct_object2: GDExtensionInterfaceClassdbConstructObject2, #[cfg(since_api = "4.7")]
    #[doc = "Constructs an Object of the requested class.\nThe passed class must be a built-in godot class, or an already-registered extension class. In both cases, object_set_instance() should be called to fully initialize the object.\nIf the type is a subtype of RefCounted, it already has a refcount of 1. The caller must take ownership the refcount and is responsible for decrementing it again when the object is no longer needed.\n\n\"NOTIFICATION_POSTINITIALIZE\" must be sent after construction.\n\n## Parameters\n- `p_classname` - A pointer to a StringName with the class name.\n\n## Return value\nA pointer to the newly created Object."]
    pub classdb_construct_object3: GDExtensionInterfaceClassdbConstructObject3, #[doc = "Gets a pointer to the MethodBind in ClassDB for the given class, method and hash.\n\n## Parameters\n- `p_classname` - A pointer to a StringName with the class name.\n- `p_methodname` - A pointer to a StringName with the method name.\n- `p_hash` - A hash representing the function signature.\n\n## Return value\nA pointer to the MethodBind from ClassDB."]
    pub classdb_get_method_bind: GDExtensionInterfaceClassdbGetMethodBind, #[doc = "Gets a pointer uniquely identifying the given built-in class in the ClassDB.\n\n## Parameters\n- `p_classname` - A pointer to a StringName with the class name.\n\n## Return value\nA pointer uniquely identifying the built-in class in the ClassDB."]
    pub classdb_get_class_tag: GDExtensionInterfaceClassdbGetClassTag, #[doc = "Registers an extension class in the ClassDB.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_parent_class_name` - A pointer to a StringName with the parent class name.\n- `p_extension_funcs` - A pointer to a GDExtensionClassCreationInfo struct."]
    pub classdb_register_extension_class: GDExtensionInterfaceClassdbRegisterExtensionClass, #[doc = "Registers an extension class in the ClassDB.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_parent_class_name` - A pointer to a StringName with the parent class name.\n- `p_extension_funcs` - A pointer to a GDExtensionClassCreationInfo2 struct."]
    pub classdb_register_extension_class2: GDExtensionInterfaceClassdbRegisterExtensionClass2, #[cfg(since_api = "4.3")]
    #[doc = "Registers an extension class in the ClassDB.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_parent_class_name` - A pointer to a StringName with the parent class name.\n- `p_extension_funcs` - A pointer to a GDExtensionClassCreationInfo3 struct."]
    pub classdb_register_extension_class3: GDExtensionInterfaceClassdbRegisterExtensionClass3, #[cfg(since_api = "4.4")]
    #[doc = "Registers an extension class in the ClassDB.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_parent_class_name` - A pointer to a StringName with the parent class name.\n- `p_extension_funcs` - A pointer to a GDExtensionClassCreationInfo4 struct."]
    pub classdb_register_extension_class4: GDExtensionInterfaceClassdbRegisterExtensionClass4, #[cfg(since_api = "4.5")]
    #[doc = "Registers an extension class in the ClassDB.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_parent_class_name` - A pointer to a StringName with the parent class name.\n- `p_extension_funcs` - A pointer to a GDExtensionClassCreationInfo5 struct."]
    pub classdb_register_extension_class5: GDExtensionInterfaceClassdbRegisterExtensionClass5, #[cfg(since_api = "4.7")]
    #[doc = "Registers an extension class in the ClassDB.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_parent_class_name` - A pointer to a StringName with the parent class name.\n- `p_extension_funcs` - A pointer to a GDExtensionClassCreationInfo6 struct."]
    pub classdb_register_extension_class6: GDExtensionInterfaceClassdbRegisterExtensionClass6, #[doc = "Registers a method on an extension class in the ClassDB.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_method_info` - A pointer to a GDExtensionClassMethodInfo struct."]
    pub classdb_register_extension_class_method: GDExtensionInterfaceClassdbRegisterExtensionClassMethod, #[cfg(since_api = "4.3")]
    #[doc = "Registers a virtual method on an extension class in ClassDB, that can be implemented by scripts or other extensions.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_method_info` - A pointer to a GDExtensionClassMethodInfo struct."]
    pub classdb_register_extension_class_virtual_method: GDExtensionInterfaceClassdbRegisterExtensionClassVirtualMethod, #[doc = "Registers an integer constant on an extension class in the ClassDB.\nNote about registering bitfield values (if p_is_bitfield is true): even though p_constant_value is signed, language bindings are\nadvised to treat bitfields as uint64_t, since this is generally clearer and can prevent mistakes like using -1 for setting all bits.\nLanguage APIs should thus provide an abstraction that registers bitfields (uint64_t) separately from regular constants (int64_t).\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_enum_name` - A pointer to a StringName with the enum name.\n- `p_constant_name` - A pointer to a StringName with the constant name.\n- `p_constant_value` - The constant value.\n- `p_is_bitfield` - Whether or not this constant is part of a bitfield."]
    pub classdb_register_extension_class_integer_constant: GDExtensionInterfaceClassdbRegisterExtensionClassIntegerConstant, #[doc = "Registers a property on an extension class in the ClassDB.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_info` - A pointer to a GDExtensionPropertyInfo struct.\n- `p_setter` - A pointer to a StringName with the name of the setter method.\n- `p_getter` - A pointer to a StringName with the name of the getter method."]
    pub classdb_register_extension_class_property: GDExtensionInterfaceClassdbRegisterExtensionClassProperty, #[doc = "Registers an indexed property on an extension class in the ClassDB.\nProvided struct can be safely freed once the function returns.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_info` - A pointer to a GDExtensionPropertyInfo struct.\n- `p_setter` - A pointer to a StringName with the name of the setter method.\n- `p_getter` - A pointer to a StringName with the name of the getter method.\n- `p_index` - The index to pass as the first argument to the getter and setter methods."]
    pub classdb_register_extension_class_property_indexed: GDExtensionInterfaceClassdbRegisterExtensionClassPropertyIndexed, #[doc = "Registers a property group on an extension class in the ClassDB.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_group_name` - A pointer to a String with the group name.\n- `p_prefix` - A pointer to a String with the prefix used by properties in this group."]
    pub classdb_register_extension_class_property_group: GDExtensionInterfaceClassdbRegisterExtensionClassPropertyGroup, #[doc = "Registers a property subgroup on an extension class in the ClassDB.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_subgroup_name` - A pointer to a String with the subgroup name.\n- `p_prefix` - A pointer to a String with the prefix used by properties in this subgroup."]
    pub classdb_register_extension_class_property_subgroup: GDExtensionInterfaceClassdbRegisterExtensionClassPropertySubgroup, #[doc = "Registers a signal on an extension class in the ClassDB.\nProvided structs can be safely freed once the function returns.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name.\n- `p_signal_name` - A pointer to a StringName with the signal name.\n- `p_argument_info` - A pointer to a GDExtensionPropertyInfo struct.\n- `p_argument_count` - The number of arguments the signal receives."]
    pub classdb_register_extension_class_signal: GDExtensionInterfaceClassdbRegisterExtensionClassSignal, #[doc = "Unregisters an extension class in the ClassDB.\nUnregistering a parent class before a class that inherits it will result in failure. Inheritors must be unregistered first.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_class_name` - A pointer to a StringName with the class name."]
    pub classdb_unregister_extension_class: GDExtensionInterfaceClassdbUnregisterExtensionClass, #[doc = "Gets the path to the current GDExtension library.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `r_path` - A pointer to a String which will receive the path."]
    pub get_library_path: GDExtensionInterfaceGetLibraryPath, #[doc = "Adds an editor plugin.\nIt's safe to call during initialization.\n\n## Parameters\n- `p_class_name` - A pointer to a StringName with the name of a class (descending from EditorPlugin) which is already registered with ClassDB."]
    pub editor_add_plugin: GDExtensionInterfaceEditorAddPlugin, #[doc = "Removes an editor plugin.\n\n## Parameters\n- `p_class_name` - A pointer to a StringName with the name of a class that was previously added as an editor plugin."]
    pub editor_remove_plugin: GDExtensionInterfaceEditorRemovePlugin, #[cfg(since_api = "4.3")]
    #[doc = "Loads new XML-formatted documentation data in the editor.\nThe provided pointer can be immediately freed once the function returns.\n\n## Parameters\n- `p_data` - A pointer to a UTF-8 encoded C string (null terminated)."]
    pub editor_help_load_xml_from_utf8_chars: GDExtensionInterfaceEditorHelpLoadXmlFromUtf8Chars, #[cfg(since_api = "4.3")]
    #[doc = "Loads new XML-formatted documentation data in the editor.\nThe provided pointer can be immediately freed once the function returns.\n\n## Parameters\n- `p_data` - A pointer to a UTF-8 encoded C string.\n- `p_size` - The number of bytes (not code units)."]
    pub editor_help_load_xml_from_utf8_chars_and_len: GDExtensionInterfaceEditorHelpLoadXmlFromUtf8CharsAndLen, #[cfg(since_api = "4.5")]
    #[doc = "Registers a callback that Godot can call to get the list of all classes (from ClassDB) that may be used by the calling GDExtension.\nThis is used by the editor to generate a build profile (in \"Tools\" > \"Engine Compilation Configuration Editor...\" > \"Detect from project\"),\nin order to recompile Godot with only the classes used.\nIn the provided callback, the GDExtension should provide the list of classes that _may_ be used statically, thus the time of invocation shouldn't matter.\nIf a GDExtension doesn't register a callback, Godot will assume that it could be using any classes.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_callback` - The callback to retrieve the list of classes used."]
    pub editor_register_get_classes_used_callback: GDExtensionInterfaceEditorRegisterGetClassesUsedCallback, #[cfg(since_api = "4.5")]
    #[doc = "Registers callbacks to be called at different phases of the main loop.\n\n## Parameters\n- `p_library` - A pointer the library received by the GDExtension's entry point function.\n- `p_callbacks` - A pointer to the structure that contains the callbacks."]
    pub register_main_loop_callbacks: GDExtensionInterfaceRegisterMainLoopCallbacks,
}
impl GDExtensionInterface {
    #[doc = r" # Safety"]
    #[doc = r" - Must be called exactly once during library initialization."]
    #[doc = r" - All parameters (dependencies) must have been initialized and valid."]
    pub(crate) unsafe fn load(get_proc_address: crate::GDExtensionInterfaceGetProcAddress,) -> Self {
        let get_proc_address = get_proc_address.expect("invalid get_proc_address function pointer");
        Self {
            get_godot_version: {
                let fptr = get_proc_address(c"get_godot_version" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "get_godot_version"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceGetGodotVersion > (fptr)
                }
            },
            #[cfg(since_api = "4.5")]
            get_godot_version2: {
                let fptr = get_proc_address(c"get_godot_version2" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "get_godot_version2"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceGetGodotVersion2 > (fptr)
                }
            },
            mem_alloc: {
                let fptr = get_proc_address(c"mem_alloc" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "mem_alloc"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceMemAlloc > (fptr)
                }
            },
            mem_realloc: {
                let fptr = get_proc_address(c"mem_realloc" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "mem_realloc"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceMemRealloc > (fptr)
                }
            },
            mem_free: {
                let fptr = get_proc_address(c"mem_free" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "mem_free"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceMemFree > (fptr)
                }
            },
            #[cfg(since_api = "4.6")]
            mem_alloc2: {
                let fptr = get_proc_address(c"mem_alloc2" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "mem_alloc2"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceMemAlloc2 > (fptr)
                }
            },
            #[cfg(since_api = "4.6")]
            mem_realloc2: {
                let fptr = get_proc_address(c"mem_realloc2" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "mem_realloc2"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceMemRealloc2 > (fptr)
                }
            },
            #[cfg(since_api = "4.6")]
            mem_free2: {
                let fptr = get_proc_address(c"mem_free2" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "mem_free2"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceMemFree2 > (fptr)
                }
            },
            print_error: {
                let fptr = get_proc_address(c"print_error" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "print_error"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePrintError > (fptr)
                }
            },
            print_error_with_message: {
                let fptr = get_proc_address(c"print_error_with_message" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "print_error_with_message"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePrintErrorWithMessage > (fptr)
                }
            },
            print_warning: {
                let fptr = get_proc_address(c"print_warning" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "print_warning"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePrintWarning > (fptr)
                }
            },
            print_warning_with_message: {
                let fptr = get_proc_address(c"print_warning_with_message" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "print_warning_with_message"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePrintWarningWithMessage > (fptr)
                }
            },
            print_script_error: {
                let fptr = get_proc_address(c"print_script_error" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "print_script_error"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePrintScriptError > (fptr)
                }
            },
            print_script_error_with_message: {
                let fptr = get_proc_address(c"print_script_error_with_message" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "print_script_error_with_message"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePrintScriptErrorWithMessage > (fptr)
                }
            },
            get_native_struct_size: {
                let fptr = get_proc_address(c"get_native_struct_size" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "get_native_struct_size"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceGetNativeStructSize > (fptr)
                }
            },
            variant_new_copy: {
                let fptr = get_proc_address(c"variant_new_copy" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_new_copy"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantNewCopy > (fptr)
                }
            },
            variant_new_nil: {
                let fptr = get_proc_address(c"variant_new_nil" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_new_nil"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantNewNil > (fptr)
                }
            },
            variant_destroy: {
                let fptr = get_proc_address(c"variant_destroy" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_destroy"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantDestroy > (fptr)
                }
            },
            variant_call: {
                let fptr = get_proc_address(c"variant_call" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_call"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantCall > (fptr)
                }
            },
            variant_call_static: {
                let fptr = get_proc_address(c"variant_call_static" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_call_static"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantCallStatic > (fptr)
                }
            },
            variant_evaluate: {
                let fptr = get_proc_address(c"variant_evaluate" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_evaluate"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantEvaluate > (fptr)
                }
            },
            variant_set: {
                let fptr = get_proc_address(c"variant_set" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_set"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantSet > (fptr)
                }
            },
            variant_set_named: {
                let fptr = get_proc_address(c"variant_set_named" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_set_named"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantSetNamed > (fptr)
                }
            },
            variant_set_keyed: {
                let fptr = get_proc_address(c"variant_set_keyed" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_set_keyed"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantSetKeyed > (fptr)
                }
            },
            variant_set_indexed: {
                let fptr = get_proc_address(c"variant_set_indexed" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_set_indexed"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantSetIndexed > (fptr)
                }
            },
            variant_get: {
                let fptr = get_proc_address(c"variant_get" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGet > (fptr)
                }
            },
            variant_get_named: {
                let fptr = get_proc_address(c"variant_get_named" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_named"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetNamed > (fptr)
                }
            },
            variant_get_keyed: {
                let fptr = get_proc_address(c"variant_get_keyed" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_keyed"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetKeyed > (fptr)
                }
            },
            variant_get_indexed: {
                let fptr = get_proc_address(c"variant_get_indexed" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_indexed"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetIndexed > (fptr)
                }
            },
            variant_iter_init: {
                let fptr = get_proc_address(c"variant_iter_init" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_iter_init"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantIterInit > (fptr)
                }
            },
            variant_iter_next: {
                let fptr = get_proc_address(c"variant_iter_next" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_iter_next"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantIterNext > (fptr)
                }
            },
            variant_iter_get: {
                let fptr = get_proc_address(c"variant_iter_get" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_iter_get"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantIterGet > (fptr)
                }
            },
            variant_hash: {
                let fptr = get_proc_address(c"variant_hash" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_hash"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantHash > (fptr)
                }
            },
            variant_recursive_hash: {
                let fptr = get_proc_address(c"variant_recursive_hash" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_recursive_hash"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantRecursiveHash > (fptr)
                }
            },
            variant_hash_compare: {
                let fptr = get_proc_address(c"variant_hash_compare" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_hash_compare"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantHashCompare > (fptr)
                }
            },
            variant_booleanize: {
                let fptr = get_proc_address(c"variant_booleanize" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_booleanize"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantBooleanize > (fptr)
                }
            },
            variant_duplicate: {
                let fptr = get_proc_address(c"variant_duplicate" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_duplicate"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantDuplicate > (fptr)
                }
            },
            variant_stringify: {
                let fptr = get_proc_address(c"variant_stringify" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_stringify"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantStringify > (fptr)
                }
            },
            variant_get_type: {
                let fptr = get_proc_address(c"variant_get_type" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_type"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetType > (fptr)
                }
            },
            variant_has_method: {
                let fptr = get_proc_address(c"variant_has_method" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_has_method"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantHasMethod > (fptr)
                }
            },
            variant_has_member: {
                let fptr = get_proc_address(c"variant_has_member" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_has_member"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantHasMember > (fptr)
                }
            },
            variant_has_key: {
                let fptr = get_proc_address(c"variant_has_key" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_has_key"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantHasKey > (fptr)
                }
            },
            #[cfg(since_api = "4.4")]
            variant_get_object_instance_id: {
                let fptr = get_proc_address(c"variant_get_object_instance_id" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_object_instance_id"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetObjectInstanceId > (fptr)
                }
            },
            variant_get_type_name: {
                let fptr = get_proc_address(c"variant_get_type_name" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_type_name"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetTypeName > (fptr)
                }
            },
            #[cfg(since_api = "4.7")]
            variant_get_type_by_name: {
                let fptr = get_proc_address(c"variant_get_type_by_name" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_type_by_name"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetTypeByName > (fptr)
                }
            },
            variant_can_convert: {
                let fptr = get_proc_address(c"variant_can_convert" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_can_convert"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantCanConvert > (fptr)
                }
            },
            variant_can_convert_strict: {
                let fptr = get_proc_address(c"variant_can_convert_strict" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_can_convert_strict"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantCanConvertStrict > (fptr)
                }
            },
            get_variant_from_type_constructor: {
                let fptr = get_proc_address(c"get_variant_from_type_constructor" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "get_variant_from_type_constructor"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceGetVariantFromTypeConstructor > (fptr)
                }
            },
            get_variant_to_type_constructor: {
                let fptr = get_proc_address(c"get_variant_to_type_constructor" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "get_variant_to_type_constructor"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceGetVariantToTypeConstructor > (fptr)
                }
            },
            #[cfg(since_api = "4.4")]
            variant_get_ptr_internal_getter: {
                let fptr = get_proc_address(c"variant_get_ptr_internal_getter" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_internal_getter"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrInternalGetter > (fptr)
                }
            },
            variant_get_ptr_operator_evaluator: {
                let fptr = get_proc_address(c"variant_get_ptr_operator_evaluator" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_operator_evaluator"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrOperatorEvaluator > (fptr)
                }
            },
            variant_get_ptr_builtin_method: {
                let fptr = get_proc_address(c"variant_get_ptr_builtin_method" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_builtin_method"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrBuiltinMethod > (fptr)
                }
            },
            variant_get_ptr_constructor: {
                let fptr = get_proc_address(c"variant_get_ptr_constructor" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_constructor"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrConstructor > (fptr)
                }
            },
            variant_get_ptr_destructor: {
                let fptr = get_proc_address(c"variant_get_ptr_destructor" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_destructor"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrDestructor > (fptr)
                }
            },
            variant_construct: {
                let fptr = get_proc_address(c"variant_construct" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_construct"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantConstruct > (fptr)
                }
            },
            variant_get_ptr_setter: {
                let fptr = get_proc_address(c"variant_get_ptr_setter" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_setter"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrSetter > (fptr)
                }
            },
            variant_get_ptr_getter: {
                let fptr = get_proc_address(c"variant_get_ptr_getter" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_getter"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrGetter > (fptr)
                }
            },
            variant_get_ptr_indexed_setter: {
                let fptr = get_proc_address(c"variant_get_ptr_indexed_setter" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_indexed_setter"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrIndexedSetter > (fptr)
                }
            },
            variant_get_ptr_indexed_getter: {
                let fptr = get_proc_address(c"variant_get_ptr_indexed_getter" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_indexed_getter"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrIndexedGetter > (fptr)
                }
            },
            variant_get_ptr_keyed_setter: {
                let fptr = get_proc_address(c"variant_get_ptr_keyed_setter" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_keyed_setter"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrKeyedSetter > (fptr)
                }
            },
            variant_get_ptr_keyed_getter: {
                let fptr = get_proc_address(c"variant_get_ptr_keyed_getter" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_keyed_getter"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrKeyedGetter > (fptr)
                }
            },
            variant_get_ptr_keyed_checker: {
                let fptr = get_proc_address(c"variant_get_ptr_keyed_checker" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_keyed_checker"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrKeyedChecker > (fptr)
                }
            },
            variant_get_constant_value: {
                let fptr = get_proc_address(c"variant_get_constant_value" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_constant_value"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetConstantValue > (fptr)
                }
            },
            variant_get_ptr_utility_function: {
                let fptr = get_proc_address(c"variant_get_ptr_utility_function" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "variant_get_ptr_utility_function"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceVariantGetPtrUtilityFunction > (fptr)
                }
            },
            string_new_with_latin1_chars: {
                let fptr = get_proc_address(c"string_new_with_latin1_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_latin1_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithLatin1Chars > (fptr)
                }
            },
            string_new_with_utf8_chars: {
                let fptr = get_proc_address(c"string_new_with_utf8_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_utf8_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithUtf8Chars > (fptr)
                }
            },
            string_new_with_utf16_chars: {
                let fptr = get_proc_address(c"string_new_with_utf16_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_utf16_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithUtf16Chars > (fptr)
                }
            },
            string_new_with_utf32_chars: {
                let fptr = get_proc_address(c"string_new_with_utf32_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_utf32_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithUtf32Chars > (fptr)
                }
            },
            string_new_with_wide_chars: {
                let fptr = get_proc_address(c"string_new_with_wide_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_wide_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithWideChars > (fptr)
                }
            },
            string_new_with_latin1_chars_and_len: {
                let fptr = get_proc_address(c"string_new_with_latin1_chars_and_len" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_latin1_chars_and_len"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithLatin1CharsAndLen > (fptr)
                }
            },
            string_new_with_utf8_chars_and_len: {
                let fptr = get_proc_address(c"string_new_with_utf8_chars_and_len" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_utf8_chars_and_len"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithUtf8CharsAndLen > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            string_new_with_utf8_chars_and_len2: {
                let fptr = get_proc_address(c"string_new_with_utf8_chars_and_len2" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_utf8_chars_and_len2"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithUtf8CharsAndLen2 > (fptr)
                }
            },
            string_new_with_utf16_chars_and_len: {
                let fptr = get_proc_address(c"string_new_with_utf16_chars_and_len" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_utf16_chars_and_len"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithUtf16CharsAndLen > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            string_new_with_utf16_chars_and_len2: {
                let fptr = get_proc_address(c"string_new_with_utf16_chars_and_len2" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_utf16_chars_and_len2"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithUtf16CharsAndLen2 > (fptr)
                }
            },
            string_new_with_utf32_chars_and_len: {
                let fptr = get_proc_address(c"string_new_with_utf32_chars_and_len" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_utf32_chars_and_len"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithUtf32CharsAndLen > (fptr)
                }
            },
            string_new_with_wide_chars_and_len: {
                let fptr = get_proc_address(c"string_new_with_wide_chars_and_len" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_new_with_wide_chars_and_len"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNewWithWideCharsAndLen > (fptr)
                }
            },
            string_to_latin1_chars: {
                let fptr = get_proc_address(c"string_to_latin1_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_to_latin1_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringToLatin1Chars > (fptr)
                }
            },
            string_to_utf8_chars: {
                let fptr = get_proc_address(c"string_to_utf8_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_to_utf8_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringToUtf8Chars > (fptr)
                }
            },
            string_to_utf16_chars: {
                let fptr = get_proc_address(c"string_to_utf16_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_to_utf16_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringToUtf16Chars > (fptr)
                }
            },
            string_to_utf32_chars: {
                let fptr = get_proc_address(c"string_to_utf32_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_to_utf32_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringToUtf32Chars > (fptr)
                }
            },
            string_to_wide_chars: {
                let fptr = get_proc_address(c"string_to_wide_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_to_wide_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringToWideChars > (fptr)
                }
            },
            string_operator_index: {
                let fptr = get_proc_address(c"string_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringOperatorIndex > (fptr)
                }
            },
            string_operator_index_const: {
                let fptr = get_proc_address(c"string_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringOperatorIndexConst > (fptr)
                }
            },
            string_operator_plus_eq_string: {
                let fptr = get_proc_address(c"string_operator_plus_eq_string" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_operator_plus_eq_string"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringOperatorPlusEqString > (fptr)
                }
            },
            string_operator_plus_eq_char: {
                let fptr = get_proc_address(c"string_operator_plus_eq_char" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_operator_plus_eq_char"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringOperatorPlusEqChar > (fptr)
                }
            },
            string_operator_plus_eq_cstr: {
                let fptr = get_proc_address(c"string_operator_plus_eq_cstr" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_operator_plus_eq_cstr"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringOperatorPlusEqCstr > (fptr)
                }
            },
            string_operator_plus_eq_wcstr: {
                let fptr = get_proc_address(c"string_operator_plus_eq_wcstr" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_operator_plus_eq_wcstr"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringOperatorPlusEqWcstr > (fptr)
                }
            },
            string_operator_plus_eq_c32str: {
                let fptr = get_proc_address(c"string_operator_plus_eq_c32str" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_operator_plus_eq_c32str"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringOperatorPlusEqC32str > (fptr)
                }
            },
            string_resize: {
                let fptr = get_proc_address(c"string_resize" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_resize"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringResize > (fptr)
                }
            },
            string_name_new_with_latin1_chars: {
                let fptr = get_proc_address(c"string_name_new_with_latin1_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_name_new_with_latin1_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNameNewWithLatin1Chars > (fptr)
                }
            },
            string_name_new_with_utf8_chars: {
                let fptr = get_proc_address(c"string_name_new_with_utf8_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_name_new_with_utf8_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNameNewWithUtf8Chars > (fptr)
                }
            },
            string_name_new_with_utf8_chars_and_len: {
                let fptr = get_proc_address(c"string_name_new_with_utf8_chars_and_len" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "string_name_new_with_utf8_chars_and_len"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceStringNameNewWithUtf8CharsAndLen > (fptr)
                }
            },
            xml_parser_open_buffer: {
                let fptr = get_proc_address(c"xml_parser_open_buffer" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "xml_parser_open_buffer"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceXmlParserOpenBuffer > (fptr)
                }
            },
            file_access_store_buffer: {
                let fptr = get_proc_address(c"file_access_store_buffer" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "file_access_store_buffer"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceFileAccessStoreBuffer > (fptr)
                }
            },
            file_access_get_buffer: {
                let fptr = get_proc_address(c"file_access_get_buffer" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "file_access_get_buffer"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceFileAccessGetBuffer > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            image_ptrw: {
                let fptr = get_proc_address(c"image_ptrw" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "image_ptrw"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceImagePtrw > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            image_ptr: {
                let fptr = get_proc_address(c"image_ptr" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "image_ptr"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceImagePtr > (fptr)
                }
            },
            worker_thread_pool_add_native_group_task: {
                let fptr = get_proc_address(c"worker_thread_pool_add_native_group_task" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "worker_thread_pool_add_native_group_task"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceWorkerThreadPoolAddNativeGroupTask > (fptr)
                }
            },
            worker_thread_pool_add_native_task: {
                let fptr = get_proc_address(c"worker_thread_pool_add_native_task" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "worker_thread_pool_add_native_task"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceWorkerThreadPoolAddNativeTask > (fptr)
                }
            },
            packed_byte_array_operator_index: {
                let fptr = get_proc_address(c"packed_byte_array_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_byte_array_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedByteArrayOperatorIndex > (fptr)
                }
            },
            packed_byte_array_operator_index_const: {
                let fptr = get_proc_address(c"packed_byte_array_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_byte_array_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedByteArrayOperatorIndexConst > (fptr)
                }
            },
            packed_float32_array_operator_index: {
                let fptr = get_proc_address(c"packed_float32_array_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_float32_array_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedFloat32ArrayOperatorIndex > (fptr)
                }
            },
            packed_float32_array_operator_index_const: {
                let fptr = get_proc_address(c"packed_float32_array_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_float32_array_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedFloat32ArrayOperatorIndexConst > (fptr)
                }
            },
            packed_float64_array_operator_index: {
                let fptr = get_proc_address(c"packed_float64_array_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_float64_array_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedFloat64ArrayOperatorIndex > (fptr)
                }
            },
            packed_float64_array_operator_index_const: {
                let fptr = get_proc_address(c"packed_float64_array_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_float64_array_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedFloat64ArrayOperatorIndexConst > (fptr)
                }
            },
            packed_int32_array_operator_index: {
                let fptr = get_proc_address(c"packed_int32_array_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_int32_array_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedInt32ArrayOperatorIndex > (fptr)
                }
            },
            packed_int32_array_operator_index_const: {
                let fptr = get_proc_address(c"packed_int32_array_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_int32_array_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedInt32ArrayOperatorIndexConst > (fptr)
                }
            },
            packed_int64_array_operator_index: {
                let fptr = get_proc_address(c"packed_int64_array_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_int64_array_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedInt64ArrayOperatorIndex > (fptr)
                }
            },
            packed_int64_array_operator_index_const: {
                let fptr = get_proc_address(c"packed_int64_array_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_int64_array_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedInt64ArrayOperatorIndexConst > (fptr)
                }
            },
            packed_string_array_operator_index: {
                let fptr = get_proc_address(c"packed_string_array_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_string_array_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedStringArrayOperatorIndex > (fptr)
                }
            },
            packed_string_array_operator_index_const: {
                let fptr = get_proc_address(c"packed_string_array_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_string_array_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedStringArrayOperatorIndexConst > (fptr)
                }
            },
            packed_vector2_array_operator_index: {
                let fptr = get_proc_address(c"packed_vector2_array_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_vector2_array_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedVector2ArrayOperatorIndex > (fptr)
                }
            },
            packed_vector2_array_operator_index_const: {
                let fptr = get_proc_address(c"packed_vector2_array_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_vector2_array_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedVector2ArrayOperatorIndexConst > (fptr)
                }
            },
            packed_vector3_array_operator_index: {
                let fptr = get_proc_address(c"packed_vector3_array_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_vector3_array_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedVector3ArrayOperatorIndex > (fptr)
                }
            },
            packed_vector3_array_operator_index_const: {
                let fptr = get_proc_address(c"packed_vector3_array_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_vector3_array_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedVector3ArrayOperatorIndexConst > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            packed_vector4_array_operator_index: {
                let fptr = get_proc_address(c"packed_vector4_array_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_vector4_array_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedVector4ArrayOperatorIndex > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            packed_vector4_array_operator_index_const: {
                let fptr = get_proc_address(c"packed_vector4_array_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_vector4_array_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedVector4ArrayOperatorIndexConst > (fptr)
                }
            },
            packed_color_array_operator_index: {
                let fptr = get_proc_address(c"packed_color_array_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_color_array_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedColorArrayOperatorIndex > (fptr)
                }
            },
            packed_color_array_operator_index_const: {
                let fptr = get_proc_address(c"packed_color_array_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "packed_color_array_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePackedColorArrayOperatorIndexConst > (fptr)
                }
            },
            array_operator_index: {
                let fptr = get_proc_address(c"array_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "array_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceArrayOperatorIndex > (fptr)
                }
            },
            array_operator_index_const: {
                let fptr = get_proc_address(c"array_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "array_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceArrayOperatorIndexConst > (fptr)
                }
            },
            array_ref: {
                let fptr = get_proc_address(c"array_ref" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "array_ref"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceArrayRef > (fptr)
                }
            },
            array_set_typed: {
                let fptr = get_proc_address(c"array_set_typed" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "array_set_typed"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceArraySetTyped > (fptr)
                }
            },
            dictionary_operator_index: {
                let fptr = get_proc_address(c"dictionary_operator_index" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "dictionary_operator_index"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceDictionaryOperatorIndex > (fptr)
                }
            },
            dictionary_operator_index_const: {
                let fptr = get_proc_address(c"dictionary_operator_index_const" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "dictionary_operator_index_const"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceDictionaryOperatorIndexConst > (fptr)
                }
            },
            #[cfg(since_api = "4.4")]
            dictionary_set_typed: {
                let fptr = get_proc_address(c"dictionary_set_typed" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "dictionary_set_typed"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceDictionarySetTyped > (fptr)
                }
            },
            object_method_bind_call: {
                let fptr = get_proc_address(c"object_method_bind_call" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_method_bind_call"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectMethodBindCall > (fptr)
                }
            },
            object_method_bind_ptrcall: {
                let fptr = get_proc_address(c"object_method_bind_ptrcall" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_method_bind_ptrcall"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectMethodBindPtrcall > (fptr)
                }
            },
            object_destroy: {
                let fptr = get_proc_address(c"object_destroy" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_destroy"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectDestroy > (fptr)
                }
            },
            global_get_singleton: {
                let fptr = get_proc_address(c"global_get_singleton" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "global_get_singleton"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceGlobalGetSingleton > (fptr)
                }
            },
            object_get_instance_binding: {
                let fptr = get_proc_address(c"object_get_instance_binding" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_get_instance_binding"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectGetInstanceBinding > (fptr)
                }
            },
            object_set_instance_binding: {
                let fptr = get_proc_address(c"object_set_instance_binding" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_set_instance_binding"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectSetInstanceBinding > (fptr)
                }
            },
            object_free_instance_binding: {
                let fptr = get_proc_address(c"object_free_instance_binding" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_free_instance_binding"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectFreeInstanceBinding > (fptr)
                }
            },
            object_set_instance: {
                let fptr = get_proc_address(c"object_set_instance" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_set_instance"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectSetInstance > (fptr)
                }
            },
            object_get_class_name: {
                let fptr = get_proc_address(c"object_get_class_name" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_get_class_name"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectGetClassName > (fptr)
                }
            },
            object_cast_to: {
                let fptr = get_proc_address(c"object_cast_to" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_cast_to"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectCastTo > (fptr)
                }
            },
            object_get_instance_from_id: {
                let fptr = get_proc_address(c"object_get_instance_from_id" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_get_instance_from_id"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectGetInstanceFromId > (fptr)
                }
            },
            object_get_instance_id: {
                let fptr = get_proc_address(c"object_get_instance_id" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_get_instance_id"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectGetInstanceId > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            object_has_script_method: {
                let fptr = get_proc_address(c"object_has_script_method" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_has_script_method"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectHasScriptMethod > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            object_call_script_method: {
                let fptr = get_proc_address(c"object_call_script_method" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_call_script_method"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectCallScriptMethod > (fptr)
                }
            },
            ref_get_object: {
                let fptr = get_proc_address(c"ref_get_object" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "ref_get_object"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceRefGetObject > (fptr)
                }
            },
            ref_set_object: {
                let fptr = get_proc_address(c"ref_set_object" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "ref_set_object"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceRefSetObject > (fptr)
                }
            },
            script_instance_create: {
                let fptr = get_proc_address(c"script_instance_create" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "script_instance_create"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceScriptInstanceCreate > (fptr)
                }
            },
            script_instance_create2: {
                let fptr = get_proc_address(c"script_instance_create2" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "script_instance_create2"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceScriptInstanceCreate2 > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            script_instance_create3: {
                let fptr = get_proc_address(c"script_instance_create3" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "script_instance_create3"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceScriptInstanceCreate3 > (fptr)
                }
            },
            placeholder_script_instance_create: {
                let fptr = get_proc_address(c"placeholder_script_instance_create" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "placeholder_script_instance_create"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePlaceholderScriptInstanceCreate > (fptr)
                }
            },
            placeholder_script_instance_update: {
                let fptr = get_proc_address(c"placeholder_script_instance_update" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "placeholder_script_instance_update"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfacePlaceholderScriptInstanceUpdate > (fptr)
                }
            },
            object_get_script_instance: {
                let fptr = get_proc_address(c"object_get_script_instance" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_get_script_instance"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectGetScriptInstance > (fptr)
                }
            },
            #[cfg(since_api = "4.5")]
            object_set_script_instance: {
                let fptr = get_proc_address(c"object_set_script_instance" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "object_set_script_instance"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceObjectSetScriptInstance > (fptr)
                }
            },
            callable_custom_create: {
                let fptr = get_proc_address(c"callable_custom_create" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "callable_custom_create"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceCallableCustomCreate > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            callable_custom_create2: {
                let fptr = get_proc_address(c"callable_custom_create2" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "callable_custom_create2"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceCallableCustomCreate2 > (fptr)
                }
            },
            callable_custom_get_userdata: {
                let fptr = get_proc_address(c"callable_custom_get_userdata" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "callable_custom_get_userdata"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceCallableCustomGetUserdata > (fptr)
                }
            },
            classdb_construct_object: {
                let fptr = get_proc_address(c"classdb_construct_object" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_construct_object"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbConstructObject > (fptr)
                }
            },
            #[cfg(since_api = "4.4")]
            classdb_construct_object2: {
                let fptr = get_proc_address(c"classdb_construct_object2" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_construct_object2"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbConstructObject2 > (fptr)
                }
            },
            #[cfg(since_api = "4.7")]
            classdb_construct_object3: {
                let fptr = get_proc_address(c"classdb_construct_object3" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_construct_object3"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbConstructObject3 > (fptr)
                }
            },
            classdb_get_method_bind: {
                let fptr = get_proc_address(c"classdb_get_method_bind" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_get_method_bind"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbGetMethodBind > (fptr)
                }
            },
            classdb_get_class_tag: {
                let fptr = get_proc_address(c"classdb_get_class_tag" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_get_class_tag"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbGetClassTag > (fptr)
                }
            },
            classdb_register_extension_class: {
                let fptr = get_proc_address(c"classdb_register_extension_class" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClass > (fptr)
                }
            },
            classdb_register_extension_class2: {
                let fptr = get_proc_address(c"classdb_register_extension_class2" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class2"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClass2 > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            classdb_register_extension_class3: {
                let fptr = get_proc_address(c"classdb_register_extension_class3" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class3"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClass3 > (fptr)
                }
            },
            #[cfg(since_api = "4.4")]
            classdb_register_extension_class4: {
                let fptr = get_proc_address(c"classdb_register_extension_class4" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class4"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClass4 > (fptr)
                }
            },
            #[cfg(since_api = "4.5")]
            classdb_register_extension_class5: {
                let fptr = get_proc_address(c"classdb_register_extension_class5" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class5"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClass5 > (fptr)
                }
            },
            #[cfg(since_api = "4.7")]
            classdb_register_extension_class6: {
                let fptr = get_proc_address(c"classdb_register_extension_class6" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class6"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClass6 > (fptr)
                }
            },
            classdb_register_extension_class_method: {
                let fptr = get_proc_address(c"classdb_register_extension_class_method" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class_method"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClassMethod > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            classdb_register_extension_class_virtual_method: {
                let fptr = get_proc_address(c"classdb_register_extension_class_virtual_method" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class_virtual_method"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClassVirtualMethod > (fptr)
                }
            },
            classdb_register_extension_class_integer_constant: {
                let fptr = get_proc_address(c"classdb_register_extension_class_integer_constant" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class_integer_constant"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClassIntegerConstant > (fptr)
                }
            },
            classdb_register_extension_class_property: {
                let fptr = get_proc_address(c"classdb_register_extension_class_property" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class_property"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClassProperty > (fptr)
                }
            },
            classdb_register_extension_class_property_indexed: {
                let fptr = get_proc_address(c"classdb_register_extension_class_property_indexed" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class_property_indexed"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClassPropertyIndexed > (fptr)
                }
            },
            classdb_register_extension_class_property_group: {
                let fptr = get_proc_address(c"classdb_register_extension_class_property_group" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class_property_group"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClassPropertyGroup > (fptr)
                }
            },
            classdb_register_extension_class_property_subgroup: {
                let fptr = get_proc_address(c"classdb_register_extension_class_property_subgroup" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class_property_subgroup"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClassPropertySubgroup > (fptr)
                }
            },
            classdb_register_extension_class_signal: {
                let fptr = get_proc_address(c"classdb_register_extension_class_signal" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_register_extension_class_signal"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbRegisterExtensionClassSignal > (fptr)
                }
            },
            classdb_unregister_extension_class: {
                let fptr = get_proc_address(c"classdb_unregister_extension_class" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "classdb_unregister_extension_class"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceClassdbUnregisterExtensionClass > (fptr)
                }
            },
            get_library_path: {
                let fptr = get_proc_address(c"get_library_path" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "get_library_path"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceGetLibraryPath > (fptr)
                }
            },
            editor_add_plugin: {
                let fptr = get_proc_address(c"editor_add_plugin" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "editor_add_plugin"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceEditorAddPlugin > (fptr)
                }
            },
            editor_remove_plugin: {
                let fptr = get_proc_address(c"editor_remove_plugin" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "editor_remove_plugin"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceEditorRemovePlugin > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            editor_help_load_xml_from_utf8_chars: {
                let fptr = get_proc_address(c"editor_help_load_xml_from_utf8_chars" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "editor_help_load_xml_from_utf8_chars"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceEditorHelpLoadXmlFromUtf8Chars > (fptr)
                }
            },
            #[cfg(since_api = "4.3")]
            editor_help_load_xml_from_utf8_chars_and_len: {
                let fptr = get_proc_address(c"editor_help_load_xml_from_utf8_chars_and_len" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "editor_help_load_xml_from_utf8_chars_and_len"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceEditorHelpLoadXmlFromUtf8CharsAndLen > (fptr)
                }
            },
            #[cfg(since_api = "4.5")]
            editor_register_get_classes_used_callback: {
                let fptr = get_proc_address(c"editor_register_get_classes_used_callback" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "editor_register_get_classes_used_callback"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceEditorRegisterGetClassesUsedCallback > (fptr)
                }
            },
            #[cfg(since_api = "4.5")]
            register_main_loop_callbacks: {
                let fptr = get_proc_address(c"register_main_loop_callbacks" . as_ptr()) . unwrap_or_else(|| panic !("failed to load `{}`", "register_main_loop_callbacks"));
                unsafe {
                    std::mem::transmute::< unsafe extern "C" fn(), GDExtensionInterfaceRegisterMainLoopCallbacks > (fptr)
                }
            },
            
        }
    }
}