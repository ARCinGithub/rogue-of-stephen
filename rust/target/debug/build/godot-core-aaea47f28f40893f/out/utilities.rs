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
#[doc = "Returns the sine of angle `angle_rad` in radians.\n\n```gdscript\nsin(0.523599)       # Returns 0.5\nsin(deg_to_rad(90)) # Returns 1.0\n```"]
pub fn sin(angle_rad: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (angle_rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . sin;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "sin", args)
    }
}
#[doc = "Returns the cosine of angle `angle_rad` in radians.\n\n```gdscript\ncos(PI * 2)         # Returns 1.0\ncos(PI)             # Returns -1.0\ncos(deg_to_rad(90)) # Returns 0.0\n```"]
pub fn cos(angle_rad: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (angle_rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cos;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cos", args)
    }
}
#[doc = "Returns the tangent of angle `angle_rad` in radians.\n\n```gdscript\ntan(deg_to_rad(45)) # Returns 1\n```"]
pub fn tan(angle_rad: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (angle_rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . tan;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "tan", args)
    }
}
#[doc = "Returns the hyperbolic sine of `x`.\n\n```gdscript\nvar a = log(2.0) # Returns 0.693147\nsinh(a) # Returns 0.75\n```"]
pub fn sinh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . sinh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "sinh", args)
    }
}
#[doc = "Returns the hyperbolic cosine of `x` in radians.\n\n```gdscript\nprint(cosh(1)) # Prints 1.543081\n```"]
pub fn cosh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cosh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cosh", args)
    }
}
#[doc = "Returns the hyperbolic tangent of `x`.\n\n```gdscript\nvar a = log(2.0) # Returns 0.693147\ntanh(a)          # Returns 0.6\n```"]
pub fn tanh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . tanh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "tanh", args)
    }
}
#[doc = "Returns the arc sine of `x` in radians. Use to get the angle of sine `x`. `x` will be clamped between `-1.0` and `1.0` (inclusive), in order to prevent \\[method asin] from returning `@GDScript.NAN`.\n\n```gdscript\n# s is 0.523599 or 30 degrees if converted with rad_to_deg(s)\nvar s = asin(0.5)\n```"]
pub fn asin(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . asin;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "asin", args)
    }
}
#[doc = "Returns the arc cosine of `x` in radians. Use to get the angle of cosine `x`. `x` will be clamped between `-1.0` and `1.0` (inclusive), in order to prevent \\[method acos] from returning `@GDScript.NAN`.\n\n```gdscript\n# c is 0.523599 or 30 degrees if converted with rad_to_deg(c)\nvar c = acos(0.866025)\n```"]
pub fn acos(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . acos;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "acos", args)
    }
}
#[doc = "Returns the arc tangent of `x` in radians. Use it to get the angle from an angle's tangent in trigonometry.\n\nThe method cannot know in which quadrant the angle should fall. See \\[method atan2] if you have both `y` and `x`.\n\n```gdscript\nvar a = atan(0.5) # a is 0.463648\n```\n\nIf `x` is between `-PI / 2` and `PI / 2` (inclusive), `atan(tan(x))` is equal to `x`."]
pub fn atan(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . atan;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "atan", args)
    }
}
#[doc = "Returns the arc tangent of `y/x` in radians. Use to get the angle of tangent `y/x`. To compute the value, the method takes into account the sign of both arguments in order to determine the quadrant.\n\nImportant note: The Y coordinate comes first, by convention.\n\n```gdscript\nvar a = atan2(0, -1) # a is 3.141593\n```"]
pub fn atan2(y: f64, x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (y, x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . atan2;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "atan2", args)
    }
}
#[doc = "Returns the hyperbolic arc (also called inverse) sine of `x`, returning a value in radians. Use it to get the angle from an angle's sine in hyperbolic space.\n\n```gdscript\nvar a = asinh(0.9) # Returns 0.8088669356527824\nsinh(a) # Returns 0.9\n```"]
pub fn asinh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . asinh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "asinh", args)
    }
}
#[doc = "Returns the hyperbolic arc (also called inverse) cosine of `x`, returning a value in radians. Use it to get the angle from an angle's cosine in hyperbolic space if `x` is larger or equal to 1. For values of `x` lower than 1, it will return 0, in order to prevent \\[method acosh] from returning `@GDScript.NAN`.\n\n```gdscript\nvar a = acosh(2) # Returns 1.31695789692482\ncosh(a) # Returns 2\n\nvar b = acosh(-1) # Returns 0\n```"]
pub fn acosh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . acosh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "acosh", args)
    }
}
#[doc = "Returns the hyperbolic arc (also called inverse) tangent of `x`, returning a value in radians. Use it to get the angle from an angle's tangent in hyperbolic space if `x` is between -1 and 1 (non-inclusive).\n\nIn mathematics, the inverse hyperbolic tangent is only defined for -1 < `x` < 1 in the real set, so values equal or lower to -1 for `x` return negative `@GDScript.INF` and values equal or higher than 1 return positive `@GDScript.INF` in order to prevent \\[method atanh] from returning `@GDScript.NAN`.\n\n```gdscript\nvar a = atanh(0.9) # Returns 1.47221948958322\ntanh(a) # Returns 0.9\n\nvar b = atanh(-2) # Returns -inf\ntanh(b) # Returns -1\n```"]
pub fn atanh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . atanh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "atanh", args)
    }
}
#[doc = "Returns the square root of `x`, where `x` is a non-negative number.\n\n```gdscript\nsqrt(9)     # Returns 3\nsqrt(10.24) # Returns 3.2\nsqrt(-1)    # Returns NaN\n```\n\n**Note:** Negative values of `x` return NaN (\"Not a Number\"). In C#, if you need negative inputs, use `System.Numerics.Complex`."]
pub fn sqrt(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . sqrt;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "sqrt", args)
    }
}
#[doc = "Returns the floating-point remainder of `x` divided by `y`, keeping the sign of `x`.\n\n```gdscript\nvar remainder = fmod(7, 5.5) # remainder is 1.5\n```\n\nFor the integer remainder operation, use the `%` operator."]
pub fn fmod(x: f64, y: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (x, y,);
    unsafe {
        let utility_fn = sys::utility_function_table() . fmod;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "fmod", args)
    }
}
#[doc = "Returns the floating-point modulus of `x` divided by `y`, wrapping equally in positive and negative.\n\n```gdscript\nprint(\" (x)  (fmod(x, 1.5))   (fposmod(x, 1.5))\")\nfor i in 7:\n\tvar x = i * 0.5 - 1.5\n\tprint(\"%4.1f           %4.1f  | %4.1f\" % [x, fmod(x, 1.5), fposmod(x, 1.5)])\n```\n\nPrints:\n\n```text\n (x)  (fmod(x, 1.5))   (fposmod(x, 1.5))\n-1.5           -0.0  |  0.0\n-1.0           -1.0  |  0.5\n-0.5           -0.5  |  1.0\n 0.0            0.0  |  0.0\n 0.5            0.5  |  0.5\n 1.0            1.0  |  1.0\n 1.5            0.0  |  0.0\n```"]
pub fn fposmod(x: f64, y: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (x, y,);
    unsafe {
        let utility_fn = sys::utility_function_table() . fposmod;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "fposmod", args)
    }
}
#[doc = "Returns the integer modulus of `x` divided by `y` that wraps equally in positive and negative.\n\n```gdscript\nprint(\"#(i)  (i % 3)   (posmod(i, 3))\")\nfor i in range(-3, 4):\n\tprint(\"%2d       %2d  | %2d\" % [i, i % 3, posmod(i, 3)])\n```\n\nPrints:\n\n```text\n(i)  (i % 3)   (posmod(i, 3))\n-3        0  |  0\n-2       -2  |  1\n-1       -1  |  2\n 0        0  |  0\n 1        1  |  1\n 2        2  |  2\n 3        0  |  0\n```"]
pub fn posmod(x: i64, y: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64,);
    let args = (x, y,);
    unsafe {
        let utility_fn = sys::utility_function_table() . posmod;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "posmod", args)
    }
}
#[doc = "Rounds `x` downward (towards negative infinity), returning the largest whole number that is not more than `x`. Supported types: `int`, `float`, [`Vector2`][crate::builtin::Vector2], [`Vector2i`][crate::builtin::Vector2i], [`Vector3`][crate::builtin::Vector3], [`Vector3i`][crate::builtin::Vector3i], [`Vector4`][crate::builtin::Vector4], [`Vector4i`][crate::builtin::Vector4i].\n\n```gdscript\nvar a = floor(2.99) # a is 2.0\na = floor(-2.99)    # a is -3.0\n```\n\nSee also \\[method ceil], \\[method round], and \\[method snapped].\n\n**Note:** For better type safety, use \\[method floorf], \\[method floori], [`floor`][`crate::builtin::Vector2::floor`], [`floor`][`crate::builtin::Vector3::floor`], or [`floor`][`crate::builtin::Vector4::floor`]."]
pub fn floor(x: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(x),);
    unsafe {
        let utility_fn = sys::utility_function_table() . floor;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "floor", args)
    }
}
#[doc = "Rounds `x` downward (towards negative infinity), returning the largest whole number that is not more than `x`.\n\nA type-safe version of \\[method floor], returning a `float`."]
pub fn floorf(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . floorf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "floorf", args)
    }
}
#[doc = "Rounds `x` downward (towards negative infinity), returning the largest whole number that is not more than `x`.\n\nA type-safe version of \\[method floor], returning an `int`.\n\n**Note:** This function is _not_ the same as `int(x)`, which rounds towards 0."]
pub fn floori(x: f64,) -> i64 {
    type CallRet = i64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . floori;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "floori", args)
    }
}
#[doc = "Rounds `x` upward (towards positive infinity), returning the smallest whole number that is not less than `x`. Supported types: `int`, `float`, [`Vector2`][crate::builtin::Vector2], [`Vector2i`][crate::builtin::Vector2i], [`Vector3`][crate::builtin::Vector3], [`Vector3i`][crate::builtin::Vector3i], [`Vector4`][crate::builtin::Vector4], [`Vector4i`][crate::builtin::Vector4i].\n\n```gdscript\nvar i = ceil(1.45) # i is 2.0\ni = ceil(1.001)    # i is 2.0\n```\n\nSee also \\[method floor], \\[method round], and \\[method snapped].\n\n**Note:** For better type safety, use \\[method ceilf], \\[method ceili], [`ceil`][`crate::builtin::Vector2::ceil`], [`ceil`][`crate::builtin::Vector3::ceil`], or [`ceil`][`crate::builtin::Vector4::ceil`]."]
pub fn ceil(x: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(x),);
    unsafe {
        let utility_fn = sys::utility_function_table() . ceil;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "ceil", args)
    }
}
#[doc = "Rounds `x` upward (towards positive infinity), returning the smallest whole number that is not less than `x`.\n\nA type-safe version of \\[method ceil], returning a `float`."]
pub fn ceilf(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . ceilf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "ceilf", args)
    }
}
#[doc = "Rounds `x` upward (towards positive infinity), returning the smallest whole number that is not less than `x`.\n\nA type-safe version of \\[method ceil], returning an `int`."]
pub fn ceili(x: f64,) -> i64 {
    type CallRet = i64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . ceili;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "ceili", args)
    }
}
#[doc = "Rounds `x` to the nearest whole number, with halfway cases rounded away from 0. Supported types: `int`, `float`, [`Vector2`][crate::builtin::Vector2], [`Vector2i`][crate::builtin::Vector2i], [`Vector3`][crate::builtin::Vector3], [`Vector3i`][crate::builtin::Vector3i], [`Vector4`][crate::builtin::Vector4], [`Vector4i`][crate::builtin::Vector4i].\n\n```gdscript\nround(2.4) # Returns 2\nround(2.5) # Returns 3\nround(2.6) # Returns 3\n```\n\nSee also \\[method floor], \\[method ceil], and \\[method snapped].\n\n**Note:** For better type safety, use \\[method roundf], \\[method roundi], [`round`][`crate::builtin::Vector2::round`], [`round`][`crate::builtin::Vector3::round`], or [`round`][`crate::builtin::Vector4::round`]."]
pub fn round(x: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(x),);
    unsafe {
        let utility_fn = sys::utility_function_table() . round;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "round", args)
    }
}
#[doc = "Rounds `x` to the nearest whole number, with halfway cases rounded away from 0.\n\nA type-safe version of \\[method round], returning a `float`."]
pub fn roundf(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . roundf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "roundf", args)
    }
}
#[doc = "Rounds `x` to the nearest whole number, with halfway cases rounded away from 0.\n\nA type-safe version of \\[method round], returning an `int`."]
pub fn roundi(x: f64,) -> i64 {
    type CallRet = i64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . roundi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "roundi", args)
    }
}
#[doc = "Returns the absolute value of a [`Variant`][crate::builtin::Variant] parameter `x` (i.e. non-negative value). Supported types: `int`, `float`, [`Vector2`][crate::builtin::Vector2], [`Vector2i`][crate::builtin::Vector2i], [`Vector3`][crate::builtin::Vector3], [`Vector3i`][crate::builtin::Vector3i], [`Vector4`][crate::builtin::Vector4], [`Vector4i`][crate::builtin::Vector4i].\n\n```gdscript\nvar a = abs(-1)\n# a is 1\n\nvar b = abs(-1.2)\n# b is 1.2\n\nvar c = abs(Vector2(-3.5, -4))\n# c is (3.5, 4)\n\nvar d = abs(Vector2i(-5, -6))\n# d is (5, 6)\n\nvar e = abs(Vector3(-7, 8.5, -3.8))\n# e is (7, 8.5, 3.8)\n\nvar f = abs(Vector3i(-7, -8, -9))\n# f is (7, 8, 9)\n```\n\n**Note:** For better type safety, use \\[method absf], \\[method absi], [`abs`][`crate::builtin::Vector2::abs`], [`abs`][`crate::builtin::Vector2i::abs`], [`abs`][`crate::builtin::Vector3::abs`], [`abs`][`crate::builtin::Vector3i::abs`], [`abs`][`crate::builtin::Vector4::abs`], or [`abs`][`crate::builtin::Vector4i::abs`]."]
pub fn abs(x: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(x),);
    unsafe {
        let utility_fn = sys::utility_function_table() . abs;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "abs", args)
    }
}
#[doc = "Returns the absolute value of float parameter `x` (i.e. positive value).\n\n```gdscript\n# a is 1.2\nvar a = absf(-1.2)\n```"]
pub fn absf(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . absf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "absf", args)
    }
}
#[doc = "Returns the absolute value of int parameter `x` (i.e. positive value).\n\n```gdscript\n# a is 1\nvar a = absi(-1)\n```"]
pub fn absi(x: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . absi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "absi", args)
    }
}
#[doc = "Returns the same type of [`Variant`][crate::builtin::Variant] as `x`, with `-1` for negative values, `1` for positive values, and `0` for zeros. For `nan` values it returns 0.\n\nSupported types: `int`, `float`, [`Vector2`][crate::builtin::Vector2], [`Vector2i`][crate::builtin::Vector2i], [`Vector3`][crate::builtin::Vector3], [`Vector3i`][crate::builtin::Vector3i], [`Vector4`][crate::builtin::Vector4], [`Vector4i`][crate::builtin::Vector4i].\n\n```gdscript\nsign(-6.0) # Returns -1\nsign(0.0)  # Returns 0\nsign(6.0)  # Returns 1\nsign(NAN)  # Returns 0\n\nsign(Vector3(-6.0, 0.0, 6.0)) # Returns (-1, 0, 1)\n```\n\n**Note:** For better type safety, use \\[method signf], \\[method signi], [`sign`][`crate::builtin::Vector2::sign`], [`sign`][`crate::builtin::Vector2i::sign`], [`sign`][`crate::builtin::Vector3::sign`], [`sign`][`crate::builtin::Vector3i::sign`], [`sign`][`crate::builtin::Vector4::sign`], or [`sign`][`crate::builtin::Vector4i::sign`]."]
pub fn sign(x: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(x),);
    unsafe {
        let utility_fn = sys::utility_function_table() . sign;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "sign", args)
    }
}
#[doc = "Returns `-1.0` if `x` is negative, `1.0` if `x` is positive, and `0.0` if `x` is zero. For `nan` values of `x` it returns 0.0.\n\n```gdscript\nsignf(-6.5) # Returns -1.0\nsignf(0.0)  # Returns 0.0\nsignf(6.5)  # Returns 1.0\nsignf(NAN)  # Returns 0.0\n```"]
pub fn signf(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . signf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "signf", args)
    }
}
#[doc = "Returns `-1` if `x` is negative, `1` if `x` is positive, and `0` if `x` is zero.\n\n```gdscript\nsigni(-6) # Returns -1\nsigni(0)  # Returns 0\nsigni(6)  # Returns 1\n```"]
pub fn signi(x: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . signi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "signi", args)
    }
}
#[doc = "Returns the multiple of `step` that is the closest to `x`. This can also be used to round a floating-point number to an arbitrary number of decimals.\n\nThe returned value is the same type of [`Variant`][crate::builtin::Variant] as `step`. Supported types: `int`, `float`, [`Vector2`][crate::builtin::Vector2], [`Vector2i`][crate::builtin::Vector2i], [`Vector3`][crate::builtin::Vector3], [`Vector3i`][crate::builtin::Vector3i], [`Vector4`][crate::builtin::Vector4], [`Vector4i`][crate::builtin::Vector4i].\n\n```gdscript\nsnapped(100, 32)  # Returns 96\nsnapped(3.14159, 0.01)  # Returns 3.14\n\nsnapped(Vector2(34, 70), Vector2(8, 8))  # Returns (32, 72)\n```\n\nSee also \\[method ceil], \\[method floor], and \\[method round].\n\n**Note:** For better type safety, use \\[method snappedf], \\[method snappedi], [`snapped`][`crate::builtin::Vector2::snapped`], [`snapped`][`crate::builtin::Vector2i::snapped`], [`snapped`][`crate::builtin::Vector3::snapped`], [`snapped`][`crate::builtin::Vector3i::snapped`], [`snapped`][`crate::builtin::Vector4::snapped`], or [`snapped`][`crate::builtin::Vector4i::snapped`]."]
pub fn snapped(x: &Variant, step: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
    let args = (RefArg::new(x), RefArg::new(step),);
    unsafe {
        let utility_fn = sys::utility_function_table() . snapped;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "snapped", args)
    }
}
#[doc = "Returns the multiple of `step` that is the closest to `x`. This can also be used to round a floating-point number to an arbitrary number of decimals.\n\nA type-safe version of \\[method snapped], returning a `float`.\n\n```gdscript\nsnappedf(32.0, 2.5)  # Returns 32.5\nsnappedf(3.14159, 0.01)  # Returns 3.14\n```"]
pub fn snappedf(x: f64, step: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (x, step,);
    unsafe {
        let utility_fn = sys::utility_function_table() . snappedf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "snappedf", args)
    }
}
#[doc = "Returns the multiple of `step` that is the closest to `x`.\n\nA type-safe version of \\[method snapped], returning an `int`.\n\n```gdscript\nsnappedi(53, 16)  # Returns 48\nsnappedi(4096, 100)  # Returns 4100\n```"]
pub fn snappedi(x: f64, step: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (f64, i64,);
    let args = (x, step,);
    unsafe {
        let utility_fn = sys::utility_function_table() . snappedi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "snappedi", args)
    }
}
#[doc = "Returns the result of `base` raised to the power of `exp`.\n\nIn GDScript, this is the equivalent of the `**` operator.\n\n```gdscript\npow(2, 5)   # Returns 32.0\npow(4, 1.5) # Returns 8.0\n```"]
pub fn pow(base: f64, exp: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (base, exp,);
    unsafe {
        let utility_fn = sys::utility_function_table() . pow;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "pow", args)
    }
}
#[doc = "Returns the [natural logarithm](https://en.wikipedia.org/wiki/Natural_logarithm) of `x` (base [_e_](https://en.wikipedia.org/wiki/E_(mathematical_constant)), with _e_ being approximately 2.71828). This is the amount of time needed to reach a certain level of continuous growth.\n\n**Note:** This is not the same as the \"log\" function on most calculators, which uses a base 10 logarithm. To use base 10 logarithm, use `log(x) / log(10)`.\n\n```gdscript\nlog(10) # Returns 2.302585\n```\n\n**Note:** The logarithm of `0` returns `-inf`, while negative values return `-nan`."]
pub fn log(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . log;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "log", args)
    }
}
#[doc = "The natural exponential function. It raises the mathematical constant _e_ to the power of `x` and returns it.\n\n_e_ has an approximate value of 2.71828, and can be obtained with `exp(1)`.\n\nFor exponents to other bases use the method \\[method pow].\n\n```gdscript\nvar a = exp(2) # Approximately 7.39\n```"]
pub fn exp(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . exp;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "exp", args)
    }
}
#[doc = "Returns `true` if `x` is a NaN (\"Not a Number\" or invalid) value. This method is needed as `@GDScript.NAN` is not equal to itself, which means `x == NAN` can't be used to check whether a value is a NaN."]
pub fn is_nan(x: f64,) -> bool {
    type CallRet = bool;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_nan;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_nan", args)
    }
}
#[doc = "Returns `true` if `x` is either positive infinity or negative infinity. See also \\[method is_finite] and \\[method is_nan]."]
pub fn is_inf(x: f64,) -> bool {
    type CallRet = bool;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_inf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_inf", args)
    }
}
#[doc = "Returns `true` if `a` and `b` are approximately equal to each other.\n\nHere, \"approximately equal\" means that `a` and `b` are within a small internal epsilon of each other, which scales with the magnitude of the numbers.\n\nInfinity values of the same sign are considered equal."]
pub fn is_equal_approx(a: f64, b: f64,) -> bool {
    type CallRet = bool;
    type CallParams = (f64, f64,);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_equal_approx;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_equal_approx", args)
    }
}
#[doc = "Returns `true` if `x` is zero or almost zero. The comparison is done using a tolerance calculation with a small internal epsilon.\n\nThis function is faster than using \\[method is_equal_approx] with one value as zero."]
pub fn is_zero_approx(x: f64,) -> bool {
    type CallRet = bool;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_zero_approx;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_zero_approx", args)
    }
}
#[doc = "Returns whether `x` is a finite value, i.e. it is not `@GDScript.NAN`, positive infinity, or negative infinity. See also \\[method is_inf] and \\[method is_nan]."]
pub fn is_finite(x: f64,) -> bool {
    type CallRet = bool;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_finite;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_finite", args)
    }
}
#[doc = "Returns an \"eased\" value of `x` based on an easing function defined with `curve`. This easing function is based on an exponent. The `curve` can be any floating-point number, with specific values leading to the following behaviors:\n\n```text\n- Lower than -1.0 (exclusive): Ease in-out\n- -1.0: Linear\n- Between -1.0 and 0.0 (exclusive): Ease out-in\n- 0.0: Constant\n- Between 0.0 to 1.0 (exclusive): Ease out\n- 1.0: Linear\n- Greater than 1.0 (exclusive): Ease in\n```\n\n[ease() curve values cheatsheet](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/ease_cheatsheet.png)\n\nSee also \\[method smoothstep]. If you need to perform more advanced transitions, use [`interpolate_value`][`crate::classes::Tween::interpolate_value`]."]
pub fn ease(x: f64, curve: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (x, curve,);
    unsafe {
        let utility_fn = sys::utility_function_table() . ease;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "ease", args)
    }
}
#[doc = "Returns the position of the first non-zero digit, after the decimal point. Note that the maximum return value is 10, which is a design decision in the implementation.\n\n```gdscript\nvar n = step_decimals(5)       # n is 0\nn = step_decimals(1.0005)      # n is 4\nn = step_decimals(0.000000005) # n is 9\n```"]
pub fn step_decimals(x: f64,) -> i64 {
    type CallRet = i64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . step_decimals;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "step_decimals", args)
    }
}
#[doc = "Linearly interpolates between two values by the factor defined in `weight`. To perform interpolation, `weight` should be between `0.0` and `1.0` (inclusive). However, values outside this range are allowed and can be used to perform _extrapolation_. If this is not desired, use \\[method clampf] to limit `weight`.\n\nBoth `from` and `to` must be the same type. Supported types: `int`, `float`, [`Vector2`][crate::builtin::Vector2], [`Vector3`][crate::builtin::Vector3], [`Vector4`][crate::builtin::Vector4], [`Color`][crate::builtin::Color], [`Quaternion`][crate::builtin::Quaternion], [`Basis`][crate::builtin::Basis], [`Transform2D`][crate::builtin::Transform2D], [`Transform3D`][crate::builtin::Transform3D].\n\n```gdscript\nlerp(0, 4, 0.75) # Returns 3.0\n```\n\nSee also \\[method inverse_lerp] which performs the reverse of this operation. To perform eased interpolation with \\[method lerp], combine it with \\[method ease] or \\[method smoothstep]. See also \\[method remap] to map a continuous series of values to another.\n\n**Note:** For better type safety, use \\[method lerpf], [`lerp`][`crate::builtin::Vector2::lerp`], [`lerp`][`crate::builtin::Vector3::lerp`], [`lerp`][`crate::builtin::Vector4::lerp`], [`lerp`][`crate::builtin::Color::lerp`], [`slerp`][`crate::builtin::Quaternion::slerp`], [`slerp`][`crate::builtin::Basis::slerp`], [`interpolate_with`][`crate::builtin::Transform2D::interpolate_with`], or [`interpolate_with`][`crate::builtin::Transform3D::interpolate_with`]."]
pub fn lerp(from: &Variant, to: &Variant, weight: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >, RefArg < 'a2, Variant >,);
    let args = (RefArg::new(from), RefArg::new(to), RefArg::new(weight),);
    unsafe {
        let utility_fn = sys::utility_function_table() . lerp;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "lerp", args)
    }
}
#[doc = "Linearly interpolates between two values by the factor defined in `weight`. To perform interpolation, `weight` should be between `0.0` and `1.0` (inclusive). However, values outside this range are allowed and can be used to perform _extrapolation_. If this is not desired, use \\[method clampf] on the result of this function.\n\n```gdscript\nlerpf(0, 4, 0.75) # Returns 3.0\n```\n\nSee also \\[method inverse_lerp] which performs the reverse of this operation. To perform eased interpolation with \\[method lerp], combine it with \\[method ease] or \\[method smoothstep]."]
pub fn lerpf(from: f64, to: f64, weight: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . lerpf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "lerpf", args)
    }
}
#[doc = "Cubic interpolates between two values by the factor defined in `weight` with `pre` and `post` values."]
pub fn cubic_interpolate(from: f64, to: f64, pre: f64, post: f64, weight: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64,);
    let args = (from, to, pre, post, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cubic_interpolate", args)
    }
}
#[doc = "Cubic interpolates between two rotation values with shortest path by the factor defined in `weight` with `pre` and `post` values. See also \\[method lerp_angle]."]
pub fn cubic_interpolate_angle(from: f64, to: f64, pre: f64, post: f64, weight: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64,);
    let args = (from, to, pre, post, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate_angle;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cubic_interpolate_angle", args)
    }
}
#[doc = "Cubic interpolates between two values by the factor defined in `weight` with `pre` and `post` values.\n\nIt can perform smoother interpolation than \\[method cubic_interpolate] by the time values."]
pub fn cubic_interpolate_in_time(from: f64, to: f64, pre: f64, post: f64, weight: f64, to_t: f64, pre_t: f64, post_t: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64, f64, f64, f64,);
    let args = (from, to, pre, post, weight, to_t, pre_t, post_t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate_in_time;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cubic_interpolate_in_time", args)
    }
}
#[doc = "Cubic interpolates between two rotation values with shortest path by the factor defined in `weight` with `pre` and `post` values. See also \\[method lerp_angle].\n\nIt can perform smoother interpolation than \\[method cubic_interpolate] by the time values."]
pub fn cubic_interpolate_angle_in_time(from: f64, to: f64, pre: f64, post: f64, weight: f64, to_t: f64, pre_t: f64, post_t: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64, f64, f64, f64,);
    let args = (from, to, pre, post, weight, to_t, pre_t, post_t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate_angle_in_time;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cubic_interpolate_angle_in_time", args)
    }
}
#[doc = "Returns the point at the given `t` on a one-dimensional [Bézier curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) defined by the given `control_1`, `control_2`, and `end` points."]
pub fn bezier_interpolate(start: f64, control_1: f64, control_2: f64, end: f64, t: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64,);
    let args = (start, control_1, control_2, end, t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . bezier_interpolate;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "bezier_interpolate", args)
    }
}
#[doc = "Returns the derivative at the given `t` on a one-dimensional [Bézier curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) defined by the given `control_1`, `control_2`, and `end` points."]
pub fn bezier_derivative(start: f64, control_1: f64, control_2: f64, end: f64, t: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64,);
    let args = (start, control_1, control_2, end, t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . bezier_derivative;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "bezier_derivative", args)
    }
}
#[doc = "Returns the difference between the two angles (in radians), in the range of `[-PI, +PI]`. When `from` and `to` are opposite, returns `-PI` if `from` is smaller than `to`, or `PI` otherwise."]
pub fn angle_difference(from: f64, to: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (from, to,);
    unsafe {
        let utility_fn = sys::utility_function_table() . angle_difference;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "angle_difference", args)
    }
}
#[doc = "Linearly interpolates between two angles (in radians) by a `weight` value between 0.0 and 1.0.\n\nSimilar to \\[method lerp], but interpolates correctly when the angles wrap around `@GDScript.TAU`. To perform eased interpolation with \\[method lerp_angle], combine it with \\[method ease] or \\[method smoothstep].\n\n```gdscript\nextends Sprite\nvar elapsed = 0.0\nfunc _process(delta):\n\tvar min_angle = deg_to_rad(0.0)\n\tvar max_angle = deg_to_rad(90.0)\n\trotation = lerp_angle(min_angle, max_angle, elapsed)\n\telapsed += delta\n```\n\n**Note:** This function lerps through the shortest path between `from` and `to`. However, when these two angles are approximately `PI + k * TAU` apart for any integer `k`, it's not obvious which way they lerp due to floating-point precision errors. For example, `lerp_angle(0, PI, weight)` lerps counter-clockwise, while `lerp_angle(0, PI + 5 * TAU, weight)` lerps clockwise."]
pub fn lerp_angle(from: f64, to: f64, weight: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . lerp_angle;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "lerp_angle", args)
    }
}
#[doc = "Returns an interpolation or extrapolation factor considering the range specified in `from` and `to`, and the interpolated value specified in `weight`. The returned value will be between `0.0` and `1.0` if `weight` is between `from` and `to` (inclusive). If `weight` is located outside this range, then an extrapolation factor will be returned (return value lower than `0.0` or greater than `1.0`). Use \\[method clamp] on the result of \\[method inverse_lerp] if this is not desired.\n\n```gdscript\n# The interpolation ratio in the `lerp()` call below is 0.75.\nvar middle = lerp(20, 30, 0.75)\n# middle is now 27.5.\n\n# Now, we pretend to have forgotten the original ratio and want to get it back.\nvar ratio = inverse_lerp(20, 30, 27.5)\n# ratio is now 0.75.\n```\n\nSee also \\[method lerp], which performs the reverse of this operation, and \\[method remap] to map a continuous series of values to another."]
pub fn inverse_lerp(from: f64, to: f64, weight: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . inverse_lerp;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "inverse_lerp", args)
    }
}
#[doc = "Maps a `value` from range `[istart, istop]` to `[ostart, ostop]`. See also \\[method lerp] and \\[method inverse_lerp]. If `value` is outside `[istart, istop]`, then the resulting value will also be outside `[ostart, ostop]`. If this is not desired, use \\[method clamp] on the result of this function.\n\n```gdscript\nremap(75, 0, 100, -1, 1) # Returns 0.5\n```\n\nFor complex use cases where multiple ranges are needed, consider using [`Curve`][crate::classes::Curve] or [`Gradient`][crate::classes::Gradient] instead.\n\n**Note:** If `istart == istop`, the return value is undefined (most likely NaN, INF, or -INF)."]
pub fn remap(value: f64, istart: f64, istop: f64, ostart: f64, ostop: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64,);
    let args = (value, istart, istop, ostart, ostop,);
    unsafe {
        let utility_fn = sys::utility_function_table() . remap;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "remap", args)
    }
}
#[doc = "Returns a smooth cubic Hermite interpolation between `0` and `1`.\n\nFor positive ranges (when `from <= to`) the return value is `0` when `x <= from`, and `1` when `x >= to`. If `x` lies between `from` and `to`, the return value follows an S-shaped curve that smoothly transitions from `0` to `1`.\n\nFor negative ranges (when `from > to`) the function is mirrored and returns `1` when `x <= to` and `0` when `x >= from`.\n\nThis S-shaped curve is the cubic Hermite interpolator, given by `f(y) = 3*y^2 - 2*y^3` where `y = (x-from) / (to-from)`.\n\n```gdscript\nsmoothstep(0, 2, -5.0) # Returns 0.0\nsmoothstep(0, 2, 0.5) # Returns 0.15625\nsmoothstep(0, 2, 1.0) # Returns 0.5\nsmoothstep(0, 2, 2.0) # Returns 1.0\n```\n\nCompared to \\[method ease] with a curve value of `-1.6521`, \\[method smoothstep] returns the smoothest possible curve with no sudden changes in the derivative. If you need to perform more advanced transitions, use [`Tween`][crate::classes::Tween] or [`AnimationPlayer`][crate::classes::AnimationPlayer].\n\n[Comparison between smoothstep() and ease(x, -1.6521) return values](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/smoothstep_ease_comparison.png)\n\n[Smoothstep() return values with positive, zero, and negative ranges](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/smoothstep_range.webp)"]
pub fn smoothstep(from: f64, to: f64, x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . smoothstep;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "smoothstep", args)
    }
}
#[doc = "Moves `from` toward `to` by the `delta` amount. Will not go past `to`.\n\nUse a negative `delta` value to move away.\n\n```gdscript\nmove_toward(5, 10, 4)    # Returns 9\nmove_toward(10, 5, 4)    # Returns 6\nmove_toward(5, 10, 9)    # Returns 10\nmove_toward(10, 5, -1.5) # Returns 11.5\n```"]
pub fn move_toward(from: f64, to: f64, delta: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, delta,);
    unsafe {
        let utility_fn = sys::utility_function_table() . move_toward;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "move_toward", args)
    }
}
#[doc = "Rotates `from` toward `to` by the `delta` amount. Will not go past `to`.\n\nSimilar to \\[method move_toward], but interpolates correctly when the angles wrap around `@GDScript.TAU`.\n\nIf `delta` is negative, this function will rotate away from `to`, toward the opposite angle, and will not go past the opposite angle."]
pub fn rotate_toward(from: f64, to: f64, delta: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, delta,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rotate_toward;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "rotate_toward", args)
    }
}
#[doc = "Converts an angle expressed in degrees to radians.\n\n```gdscript\nvar r = deg_to_rad(180) # r is 3.141593\n```"]
pub fn deg_to_rad(deg: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (deg,);
    unsafe {
        let utility_fn = sys::utility_function_table() . deg_to_rad;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "deg_to_rad", args)
    }
}
#[doc = "Converts an angle expressed in radians to degrees.\n\n```gdscript\nrad_to_deg(0.523599) # Returns 30\nrad_to_deg(PI)       # Returns 180\nrad_to_deg(PI * 2)   # Returns 360\n```"]
pub fn rad_to_deg(rad: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rad_to_deg;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "rad_to_deg", args)
    }
}
#[doc = "Converts from linear energy to decibels (audio). Since volume is not normally linear, this can be used to implement volume sliders that behave as expected.\n\n**Example:** Change the Master bus's volume through a [`Slider`][crate::classes::Slider] node, which ranges from `0.0` to `1.0`:\n\n```gdscript\nAudioServer.set_bus_volume_db(AudioServer.get_bus_index(\"Master\"), linear_to_db($Slider.value))\n```"]
pub fn linear_to_db(lin: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (lin,);
    unsafe {
        let utility_fn = sys::utility_function_table() . linear_to_db;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "linear_to_db", args)
    }
}
#[doc = "Converts from decibels to linear energy (audio)."]
pub fn db_to_linear(db: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (db,);
    unsafe {
        let utility_fn = sys::utility_function_table() . db_to_linear;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "db_to_linear", args)
    }
}
#[doc = "Wraps the [`Variant`][crate::builtin::Variant] `value` between `min` and `max`. `min` is _inclusive_ while `max` is _exclusive_. This can be used for creating loop-like behavior or infinite surfaces.\n\nVariant types `int` and `float` are supported. If any of the arguments is `float`, this function returns a `float`, otherwise it returns an `int`.\n\n```gdscript\nvar a = wrap(4, 5, 10)\n# a is 9 (int)\n\nvar a = wrap(7, 5, 10)\n# a is 7 (int)\n\nvar a = wrap(10.5, 5, 10)\n# a is 5.5 (float)\n```"]
pub fn wrap(value: &Variant, min: &Variant, max: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >, RefArg < 'a2, Variant >,);
    let args = (RefArg::new(value), RefArg::new(min), RefArg::new(max),);
    unsafe {
        let utility_fn = sys::utility_function_table() . wrap;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "wrap", args)
    }
}
#[doc = "Wraps the integer `value` between `min` and `max`. `min` is _inclusive_ while `max` is _exclusive_. This can be used for creating loop-like behavior or infinite surfaces.\n\n```gdscript\n# Infinite loop between 5 and 9\nframe = wrapi(frame + 1, 5, 10)\n```\n\n```gdscript\n# result is -2\nvar result = wrapi(-6, -5, -1)\n```"]
pub fn wrapi(value: i64, min: i64, max: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64, i64,);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . wrapi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "wrapi", args)
    }
}
#[doc = "Wraps the float `value` between `min` and `max`. `min` is _inclusive_ while `max` is _exclusive_. This can be used for creating loop-like behavior or infinite surfaces.\n\n```gdscript\n# Infinite loop between 5.0 and 9.9\nvalue = wrapf(value + 0.1, 5.0, 10.0)\n```\n\n```gdscript\n# Infinite rotation (in radians)\nangle = wrapf(angle + 0.1, 0.0, TAU)\n```\n\n```gdscript\n# Infinite rotation (in radians)\nangle = wrapf(angle + 0.1, -PI, PI)\n```\n\n**Note:** If `min` is `0`, this is equivalent to \\[method fposmod], so prefer using that instead. \\[method wrapf] is more flexible than using the \\[method fposmod] approach by giving the user control over the minimum value."]
pub fn wrapf(value: f64, min: f64, max: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . wrapf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "wrapf", args)
    }
}
#[doc = "Returns the maximum of the given numeric values. This function can take any number of arguments.\n\n```gdscript\nmax(1, 7, 3, -6, 5) # Returns 7\n```\n\n**Note:** When using this on vectors it will _not_ perform component-wise maximum, and will pick the largest value when compared using `x < y`. To perform component-wise maximum, use [`coord_max`][`crate::builtin::Vector2::coord_max`], [`max`][`crate::builtin::Vector2i::max`], [`coord_max`][`crate::builtin::Vector3::coord_max`], [`max`][`crate::builtin::Vector3i::max`], [`coord_max`][`crate::builtin::Vector4::coord_max`], and [`max`][`crate::builtin::Vector4i::max`]."]
pub fn max(arg1: &Variant, arg2: &Variant, varargs: &[Variant]) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
    let args = (RefArg::new(arg1), RefArg::new(arg2),);
    unsafe {
        let utility_fn = sys::utility_function_table() . max;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "max", args, varargs)
    }
}
#[doc = "Returns the maximum of two `int` values.\n\n```gdscript\nmaxi(1, 2)   # Returns 2\nmaxi(-3, -4) # Returns -3\n```"]
pub fn maxi(a: i64, b: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64,);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . maxi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "maxi", args)
    }
}
#[doc = "Returns the maximum of two `float` values.\n\n```gdscript\nmaxf(3.6, 24)   # Returns 24.0\nmaxf(-3.99, -4) # Returns -3.99\n```"]
pub fn maxf(a: f64, b: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . maxf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "maxf", args)
    }
}
#[doc = "Returns the minimum of the given numeric values. This function can take any number of arguments.\n\n```gdscript\nmin(1, 7, 3, -6, 5) # Returns -6\n```\n\n**Note:** When using this on vectors it will _not_ perform component-wise minimum, and will pick the smallest value when compared using `x < y`. To perform component-wise minimum, use [`coord_min`][`crate::builtin::Vector2::coord_min`], [`min`][`crate::builtin::Vector2i::min`], [`coord_min`][`crate::builtin::Vector3::coord_min`], [`min`][`crate::builtin::Vector3i::min`], [`coord_min`][`crate::builtin::Vector4::coord_min`], and [`min`][`crate::builtin::Vector4i::min`]."]
pub fn min(arg1: &Variant, arg2: &Variant, varargs: &[Variant]) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
    let args = (RefArg::new(arg1), RefArg::new(arg2),);
    unsafe {
        let utility_fn = sys::utility_function_table() . min;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "min", args, varargs)
    }
}
#[doc = "Returns the minimum of two `int` values.\n\n```gdscript\nmini(1, 2)   # Returns 1\nmini(-3, -4) # Returns -4\n```"]
pub fn mini(a: i64, b: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64,);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . mini;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "mini", args)
    }
}
#[doc = "Returns the minimum of two `float` values.\n\n```gdscript\nminf(3.6, 24)   # Returns 3.6\nminf(-3.99, -4) # Returns -4.0\n```"]
pub fn minf(a: f64, b: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . minf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "minf", args)
    }
}
#[doc = "Clamps the `value`, returning a [`Variant`][crate::builtin::Variant] not less than `min` and not more than `max`. Any values that can be compared with the less than and greater than operators will work.\n\n```gdscript\nvar a = clamp(-10, -1, 5)\n# a is -1\n\nvar b = clamp(8.1, 0.9, 5.5)\n# b is 5.5\n```\n\n**Note:** For better type safety, use \\[method clampf], \\[method clampi], [`clamp`][`crate::builtin::Vector2::clamp`], [`clamp`][`crate::builtin::Vector2i::clamp`], [`clamp`][`crate::builtin::Vector3::clamp`], [`clamp`][`crate::builtin::Vector3i::clamp`], [`clamp`][`crate::builtin::Vector4::clamp`], [`clamp`][`crate::builtin::Vector4i::clamp`], or [`clamp`][`crate::builtin::Color::clamp`] (not currently supported by this method).\n\n**Note:** When using this on vectors it will _not_ perform component-wise clamping, and will pick `min` if `value < min` or `max` if `value > max`. To perform component-wise clamping use the methods listed above."]
pub fn clamp(value: &Variant, min: &Variant, max: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >, RefArg < 'a2, Variant >,);
    let args = (RefArg::new(value), RefArg::new(min), RefArg::new(max),);
    unsafe {
        let utility_fn = sys::utility_function_table() . clamp;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "clamp", args)
    }
}
#[doc = "Clamps the `value`, returning an `int` not less than `min` and not more than `max`.\n\n```gdscript\nvar speed = 42\nvar a = clampi(speed, 1, 20) # a is 20\n\nspeed = -10\nvar b = clampi(speed, -1, 1) # b is -1\n```"]
pub fn clampi(value: i64, min: i64, max: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64, i64,);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . clampi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "clampi", args)
    }
}
#[doc = "Clamps the `value`, returning a `float` not less than `min` and not more than `max`.\n\n```gdscript\nvar speed = 42.1\nvar a = clampf(speed, 1.0, 20.5) # a is 20.5\n\nspeed = -10.0\nvar b = clampf(speed, -1.0, 1.0) # b is -1.0\n```"]
pub fn clampf(value: f64, min: f64, max: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . clampf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "clampf", args)
    }
}
#[doc = "Returns the smallest integer power of 2 that is greater than or equal to `value`.\n\n```gdscript\nnearest_po2(3) # Returns 4\nnearest_po2(4) # Returns 4\nnearest_po2(5) # Returns 8\n\nnearest_po2(0)  # Returns 0 (this may not be expected)\nnearest_po2(-1) # Returns 0 (this may not be expected)\n```\n\n**Warning:** Due to its implementation, this method returns `0` rather than `1` for values less than or equal to `0`, with an exception for `value` being the smallest negative 64-bit integer (`-9223372036854775808`) in which case the `value` is returned unchanged."]
pub fn nearest_po2(value: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64,);
    let args = (value,);
    unsafe {
        let utility_fn = sys::utility_function_table() . nearest_po2;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "nearest_po2", args)
    }
}
#[doc = "Wraps `value` between `0` and the `length`. If the limit is reached, the next value the function returns is decreased to the `0` side or increased to the `length` side (like a triangle wave). If `length` is less than zero, it becomes positive.\n\n```gdscript\npingpong(-3.0, 3.0) # Returns 3.0\npingpong(-2.0, 3.0) # Returns 2.0\npingpong(-1.0, 3.0) # Returns 1.0\npingpong(0.0, 3.0)  # Returns 0.0\npingpong(1.0, 3.0)  # Returns 1.0\npingpong(2.0, 3.0)  # Returns 2.0\npingpong(3.0, 3.0)  # Returns 3.0\npingpong(4.0, 3.0)  # Returns 2.0\npingpong(5.0, 3.0)  # Returns 1.0\npingpong(6.0, 3.0)  # Returns 0.0\n```"]
pub fn pingpong(value: f64, length: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (value, length,);
    unsafe {
        let utility_fn = sys::utility_function_table() . pingpong;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "pingpong", args)
    }
}
#[doc = "Randomizes the seed (or the internal state) of the random number generator. The current implementation uses a number based on the device's time.\n\n**Note:** This function is called automatically when the project is run. If you need to fix the seed to have consistent, reproducible results, use \\[method seed] to initialize the random number generator."]
pub fn randomize() {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . randomize;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randomize", args)
    }
}
#[doc = "Returns a random unsigned 32-bit integer. Use remainder to obtain a random value in the interval `[0, N - 1]` (where N is smaller than 2^32).\n\n\n```gdscript\nrandi()           # Returns random integer between 0 and 2^32 - 1\nrandi() % 20      # Returns random integer between 0 and 19\nrandi() % 100     # Returns random integer between 0 and 99\nrandi() % 100 + 1 # Returns random integer between 1 and 100\n```\n"]
pub fn randi() -> i64 {
    type CallRet = i64;
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . randi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randi", args)
    }
}
#[doc = "Returns a random floating-point value between `0.0` and `1.0` (inclusive).\n\n\n```gdscript\nrandf() # Returns e.g. 0.375671\n```\n"]
pub fn randf() -> f64 {
    type CallRet = f64;
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . randf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randf", args)
    }
}
#[doc = "Returns a random signed 32-bit integer between `from` and `to` (inclusive). If `to` is lesser than `from`, they are swapped.\n\n\n```gdscript\nrandi_range(0, 1)      # Returns either 0 or 1\nrandi_range(-10, 1000) # Returns random integer between -10 and 1000\n```\n"]
pub fn randi_range(from: i64, to: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64,);
    let args = (from, to,);
    unsafe {
        let utility_fn = sys::utility_function_table() . randi_range;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randi_range", args)
    }
}
#[doc = "Returns a random floating-point value between `from` and `to` (inclusive).\n\n\n```gdscript\nrandf_range(0, 20.5) # Returns e.g. 7.45315\nrandf_range(-10, 10) # Returns e.g. -3.844535\n```\n"]
pub fn randf_range(from: f64, to: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (from, to,);
    unsafe {
        let utility_fn = sys::utility_function_table() . randf_range;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randf_range", args)
    }
}
#[doc = "Returns a [normally-distributed](https://en.wikipedia.org/wiki/Normal_distribution), pseudo-random floating-point value from the specified `mean` and a standard `deviation`. This is also known as a Gaussian distribution.\n\n**Note:** This method uses the [Box-Muller transform](https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform) algorithm."]
pub fn randfn(mean: f64, deviation: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (mean, deviation,);
    unsafe {
        let utility_fn = sys::utility_function_table() . randfn;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randfn", args)
    }
}
#[doc = "Sets the seed for the random number generator to `base`. Setting the seed manually can ensure consistent, repeatable results for most random functions.\n\n\n```gdscript\nvar my_seed = \"Godot Rocks\".hash()\nseed(my_seed)\nvar a = randf() + randi()\nseed(my_seed)\nvar b = randf() + randi()\n# a and b are now identical\n```\n"]
pub fn seed(base: i64,) {
    type CallRet = ();
    type CallParams = (i64,);
    let args = (base,);
    unsafe {
        let utility_fn = sys::utility_function_table() . seed;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "seed", args)
    }
}
#[doc = "Given a `seed`, returns a [`PackedInt64Array`][crate::builtin::PackedInt64Array] of size `2`, where its first element is the randomized `int` value, and the second element is the same as `seed`. Passing the same `seed` consistently returns the same array.\n\n**Note:** \"Seed\" here refers to the internal state of the pseudo random number generator, currently implemented as a 64 bit integer.\n\n```gdscript\nvar a = rand_from_seed(4)\n\nprint(a[0]) # Prints 2879024997\nprint(a[1]) # Prints 4\n```"]
pub fn rand_from_seed(seed: i64,) -> PackedInt64Array {
    type CallRet = PackedInt64Array;
    type CallParams = (i64,);
    let args = (seed,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rand_from_seed;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "rand_from_seed", args)
    }
}
#[doc = "\n\n# Specific notes for this function\n\nUse of `weakref()` is discouraged. See [`WeakRef`][crate::classes::WeakRef] for a detailed explanation and better alternatives."]
#[doc = "\n# Godot docs\nReturns a [`WeakRef`][crate::classes::WeakRef] instance holding a weak reference to `obj`. Returns an empty [`WeakRef`][crate::classes::WeakRef] instance if `obj` is `null`. Prints an error and returns `null` if `obj` is neither [`Object`][crate::classes::Object]-derived nor `null`.\n\nA weak reference to an object is not enough to keep the object alive: when the only remaining references to a referent are weak references, garbage collection is free to destroy the referent and reuse its memory for something else. However, until the object is actually destroyed the weak reference may return the object even if there are no strong references to it."]
pub fn weakref(obj: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(obj),);
    unsafe {
        let utility_fn = sys::utility_function_table() . weakref;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "weakref", args)
    }
}
#[doc = "Returns the internal type of the given `variable`, using the \\[enum Variant.Type] values.\n\n```gdscript\nvar json = JSON.new()\njson.parse('[\"a\", \"b\", \"c\"]')\nvar result = json.get_data()\nif typeof(result) == TYPE_ARRAY:\n\tprint(result[0]) # Prints \"a\"\nelse:\n\tprint(\"Unexpected result!\")\n```\n\nSee also \\[method type_string]."]
pub fn typeof_(variable: &Variant,) -> i64 {
    type CallRet = i64;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(variable),);
    unsafe {
        let utility_fn = sys::utility_function_table() . typeof_;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "typeof", args)
    }
}
#[doc = "Converts the given `variant` to the given `type`, using the \\[enum Variant.Type] values. This method is generous with how it handles types, it can automatically convert between array types, convert numeric [`String`][crate::builtin::GString]s to `int`, and converting most things to [`String`][crate::builtin::GString].\n\nIf the type conversion cannot be done, this method will return the default value for that type, for example converting [`Rect2`][crate::builtin::Rect2] to [`Vector2`][crate::builtin::Vector2] will always return `Vector2.ZERO`. This method will never show error messages as long as `type` is a valid Variant type.\n\nThe returned value is a [`Variant`][crate::builtin::Variant], but the data inside and its type will be the same as the requested type.\n\n```gdscript\ntype_convert(\"Hi!\", TYPE_INT) # Returns 0\ntype_convert(\"123\", TYPE_INT) # Returns 123\ntype_convert(123.4, TYPE_INT) # Returns 123\ntype_convert(5, TYPE_VECTOR2) # Returns (0, 0)\ntype_convert(\"Hi!\", TYPE_NIL) # Returns null\n```"]
pub fn type_convert(variant: &Variant, type_: i64,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >, i64,);
    let args = (RefArg::new(variant), type_,);
    unsafe {
        let utility_fn = sys::utility_function_table() . type_convert;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "type_convert", args)
    }
}
#[doc = "Converts one or more arguments of any [`Variant`][crate::builtin::Variant] type to a [`String`][crate::builtin::GString] in the best way possible.\n\n```gdscript\nvar a = [10, 20, 30]\nvar b = str(a)\nprint(len(a)) # Prints 3 (the number of elements in the array).\nprint(len(b)) # Prints 12 (the length of the string \"[10, 20, 30]\").\n```"]
pub fn str(varargs: &[Variant]) -> GString {
    type CallRet = GString;
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table_thread_safe() . str;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "str", args, varargs)
    }
}
#[doc = "Returns a human-readable name for the given \\[enum Error] code.\n\n```gdscript\nprint(OK)                              # Prints 0\nprint(error_string(OK))                # Prints \"OK\"\nprint(error_string(ERR_BUSY))          # Prints \"Busy\"\nprint(error_string(ERR_OUT_OF_MEMORY)) # Prints \"Out of memory\"\n```"]
pub fn error_string(error: i64,) -> GString {
    type CallRet = GString;
    type CallParams = (i64,);
    let args = (error,);
    unsafe {
        let utility_fn = sys::utility_function_table() . error_string;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "error_string", args)
    }
}
#[doc = "Returns a human-readable name of the given `type`, using the \\[enum Variant.Type] values.\n\n```gdscript\nprint(TYPE_INT) # Prints 2\nprint(type_string(TYPE_INT)) # Prints \"int\"\nprint(type_string(TYPE_STRING)) # Prints \"String\"\n```\n\nSee also \\[method typeof]."]
pub fn type_string(type_: i64,) -> GString {
    type CallRet = GString;
    type CallParams = (i64,);
    let args = (type_,);
    unsafe {
        let utility_fn = sys::utility_function_table() . type_string;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "type_string", args)
    }
}
#[doc = "Converts one or more arguments of any type to string in the best way possible and prints them to the console.\n\n\n```gdscript\nvar a = [1, 2, 3]\nprint(\"a\", \"b\", a) # Prints \"ab[1, 2, 3]\"\n```\n\n\n**Note:** Consider using \\[method push_error] and \\[method push_warning] to print error and warning messages instead of \\[method print] or \\[method print_rich]. This distinguishes them from print messages used for debugging purposes, while also displaying a stack trace when an error or warning is printed. See also \\[member Engine.print_to_stdout] and \\[member ProjectSettings.application/run/disable_stdout]."]
pub fn print(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table_thread_safe() . print;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "print", args, varargs)
    }
}
#[doc = "Converts one or more arguments of any type to string in the best way possible and prints them to the console.\n\nThe following BBCode tags are supported: `b`, `i`, `u`, `s`, `indent`, `code`, `url`, `center`, `right`, `color`, `bgcolor`, `fgcolor`.\n\nURL tags only support URLs wrapped by a URL tag, not URLs with a different title.\n\nWhen printing to standard output, the supported subset of BBCode is converted to ANSI escape codes for the terminal emulator to display. Support for ANSI escape codes varies across terminal emulators, especially for italic and strikethrough. In standard output, `code` is represented with faint text but without any font change. Unsupported tags are left as-is in standard output.\n\n\n[gdscript skip-lint]\nprint_rich(\"[color=green]**Hello world!**[/color]\") # Prints \"Hello world!\", in green with a bold font.\n[/gdscript]\n[csharp skip-lint]\nGD.PrintRich(\"[color=green]**Hello world!**[/color]\"); // Prints \"Hello world!\", in green with a bold font.\n[/csharp]\n\n\n**Note:** Consider using \\[method push_error] and \\[method push_warning] to print error and warning messages instead of \\[method print] or \\[method print_rich]. This distinguishes them from print messages used for debugging purposes, while also displaying a stack trace when an error or warning is printed.\n\n**Note:** Output displayed in the editor supports clickable `[url=address]text[/url]` tags. The `[url]` tag's `address` value is handled by [`shell_open`][`crate::classes::Os::shell_open`] when clicked."]
pub fn print_rich(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table_thread_safe() . print_rich;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "print_rich", args, varargs)
    }
}
#[doc = "Prints one or more arguments to strings in the best way possible to standard error line.\n\n\n```gdscript\nprinterr(\"prints to stderr\")\n```\n"]
pub fn printerr(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table_thread_safe() . printerr;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "printerr", args, varargs)
    }
}
#[doc = "Prints one or more arguments to the console with a tab between each argument.\n\n\n```gdscript\nprintt(\"A\", \"B\", \"C\") # Prints \"A       B       C\"\n```\n"]
pub fn printt(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table_thread_safe() . printt;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "printt", args, varargs)
    }
}
#[doc = "Prints one or more arguments to the console with a space between each argument.\n\n\n```gdscript\nprints(\"A\", \"B\", \"C\") # Prints \"A B C\"\n```\n"]
pub fn prints(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table_thread_safe() . prints;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "prints", args, varargs)
    }
}
#[doc = "Prints one or more arguments to strings in the best way possible to the OS terminal. Unlike \\[method print], no newline is automatically added at the end.\n\n**Note:** The OS terminal is _not_ the same as the editor's Output dock. The output sent to the OS terminal can be seen when running Godot from a terminal. On Windows, this requires using the `console.exe` executable.\n\n\n```gdscript\n# Prints \"ABC\" to terminal.\nprintraw(\"A\")\nprintraw(\"B\")\nprintraw(\"C\")\n```\n"]
pub fn printraw(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table_thread_safe() . printraw;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "printraw", args, varargs)
    }
}
#[doc = "If verbose mode is enabled ([`is_stdout_verbose`][`crate::classes::Os::is_stdout_verbose`] returning `true`), converts one or more arguments of any type to string in the best way possible and prints them to the console."]
pub fn print_verbose(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table_thread_safe() . print_verbose;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "print_verbose", args, varargs)
    }
}
#[doc = "Pushes an error message to Godot's built-in debugger and to the OS terminal.\n\n\n```gdscript\npush_error(\"test error\") # Prints \"test error\" to debugger and terminal as an error.\n```\n\n\n**Note:** This function does not pause project execution. To print an error message and pause project execution in debug builds, use `assert(false, \"test error\")` instead."]
pub fn push_error(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table_thread_safe() . push_error;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "push_error", args, varargs)
    }
}
#[doc = "Pushes a warning message to Godot's built-in debugger and to the OS terminal.\n\n\n```gdscript\npush_warning(\"test warning\") # Prints \"test warning\" to debugger and terminal as a warning.\n```\n"]
pub fn push_warning(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table_thread_safe() . push_warning;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "push_warning", args, varargs)
    }
}
#[doc = "Converts a [`Variant`][crate::builtin::Variant] `variable` to a formatted [`String`][crate::builtin::GString] that can then be parsed using \\[method str_to_var].\n\n\n```gdscript\nvar a = { \"a\": 1, \"b\": 2 }\nprint(var_to_str(a))\n```\n\n\nPrints:\n\n```text\n{\n\t\"a\": 1,\n\t\"b\": 2\n}\n```\n\n**Note:** Converting [`Signal`][crate::builtin::Signal] or [`Callable`][crate::builtin::Callable] is not supported and will result in an empty value for these types, regardless of their data."]
pub fn var_to_str(variable: &Variant,) -> GString {
    type CallRet = GString;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(variable),);
    unsafe {
        let utility_fn = sys::utility_function_table() . var_to_str;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "var_to_str", args)
    }
}
#[doc = "Converts a formatted `string` that was returned by \\[method var_to_str] to the original [`Variant`][crate::builtin::Variant].\n\n\n```gdscript\nvar data = '{ \"a\": 1, \"b\": 2 }' # data is a String\nvar dict = str_to_var(data)     # dict is a Dictionary\nprint(dict[\"a\"])                # Prints 1\n```\n"]
pub fn str_to_var(string: impl AsArg < GString >,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
    let args = (string.into_arg(),);
    unsafe {
        let utility_fn = sys::utility_function_table() . str_to_var;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "str_to_var", args)
    }
}
#[doc = "Encodes a [`Variant`][crate::builtin::Variant] value to a byte array, without encoding objects. Deserialization can be done with \\[method bytes_to_var].\n\n**Note:** If you need object serialization, see \\[method var_to_bytes_with_objects].\n\n**Note:** Encoding [`Callable`][crate::builtin::Callable] is not supported and will result in an empty value, regardless of the data."]
pub fn var_to_bytes(variable: &Variant,) -> PackedByteArray {
    type CallRet = PackedByteArray;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(variable),);
    unsafe {
        let utility_fn = sys::utility_function_table() . var_to_bytes;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "var_to_bytes", args)
    }
}
#[doc = "Decodes a byte array back to a [`Variant`][crate::builtin::Variant] value, without decoding objects.\n\n**Note:** If you need object deserialization, see \\[method bytes_to_var_with_objects]."]
pub fn bytes_to_var(bytes: &PackedByteArray,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
    let args = (RefArg::new(bytes),);
    unsafe {
        let utility_fn = sys::utility_function_table() . bytes_to_var;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "bytes_to_var", args)
    }
}
#[doc = "Encodes a [`Variant`][crate::builtin::Variant] value to a byte array. Encoding objects is allowed (and can potentially include executable code). Deserialization can be done with \\[method bytes_to_var_with_objects].\n\n**Note:** Encoding [`Callable`][crate::builtin::Callable] is not supported and will result in an empty value, regardless of the data."]
pub fn var_to_bytes_with_objects(variable: &Variant,) -> PackedByteArray {
    type CallRet = PackedByteArray;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(variable),);
    unsafe {
        let utility_fn = sys::utility_function_table() . var_to_bytes_with_objects;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "var_to_bytes_with_objects", args)
    }
}
#[doc = "Decodes a byte array back to a [`Variant`][crate::builtin::Variant] value. Decoding objects is allowed.\n\n**Warning:** Deserialized object can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats (remote code execution)."]
pub fn bytes_to_var_with_objects(bytes: &PackedByteArray,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
    let args = (RefArg::new(bytes),);
    unsafe {
        let utility_fn = sys::utility_function_table() . bytes_to_var_with_objects;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "bytes_to_var_with_objects", args)
    }
}
#[doc = "Returns the integer hash of the passed `variable`.\n\n\n```gdscript\nprint(hash(\"a\")) # Prints 177670\n```\n"]
pub fn hash(variable: &Variant,) -> i64 {
    type CallRet = i64;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(variable),);
    unsafe {
        let utility_fn = sys::utility_function_table() . hash;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "hash", args)
    }
}
#[doc = "Returns `true` if the Object that corresponds to `id` is a valid object (e.g. has not been deleted from memory). All Objects have a unique instance ID."]
pub(crate) fn is_instance_id_valid(id: i64,) -> bool {
    type CallRet = bool;
    type CallParams = (i64,);
    let args = (id,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_instance_id_valid;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_instance_id_valid", args)
    }
}
#[doc = "Returns `true` if `instance` is a valid Object (e.g. has not been deleted from memory)."]
pub(crate) fn is_instance_valid(instance: &Variant,) -> bool {
    type CallRet = bool;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(instance),);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_instance_valid;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_instance_valid", args)
    }
}
#[doc = "Allocates a unique ID which can be used by the implementation to construct an RID. This is used mainly from native extensions to implement servers."]
pub fn rid_allocate_id() -> i64 {
    type CallRet = i64;
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . rid_allocate_id;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "rid_allocate_id", args)
    }
}
#[doc = "Creates an RID from a `base`. This is used mainly from native extensions to build servers."]
pub fn rid_from_int64(base: i64,) -> Rid {
    type CallRet = Rid;
    type CallParams = (i64,);
    let args = (base,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rid_from_int64;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "rid_from_int64", args)
    }
}
#[doc = "Returns `true`, for value types, if `a` and `b` share the same value. Returns `true`, for reference types, if the references of `a` and `b` are the same.\n\n```gdscript\n# Vector2 is a value type\nvar vec2_a = Vector2(0, 0)\nvar vec2_b = Vector2(0, 0)\nvar vec2_c = Vector2(1, 1)\nis_same(vec2_a, vec2_a)  # true\nis_same(vec2_a, vec2_b)  # true\nis_same(vec2_a, vec2_c)  # false\n\n# Array is a reference type\nvar arr_a = []\nvar arr_b = []\nis_same(arr_a, arr_a)  # true\nis_same(arr_a, arr_b)  # false\n```\n\nThese are [`Variant`][crate::builtin::Variant] value types: `null`, `bool`, `int`, `float`, [`String`][crate::builtin::GString], [`StringName`][crate::builtin::StringName], [`Vector2`][crate::builtin::Vector2], [`Vector2i`][crate::builtin::Vector2i], [`Vector3`][crate::builtin::Vector3], [`Vector3i`][crate::builtin::Vector3i], [`Vector4`][crate::builtin::Vector4], [`Vector4i`][crate::builtin::Vector4i], [`Rect2`][crate::builtin::Rect2], [`Rect2i`][crate::builtin::Rect2i], [`Transform2D`][crate::builtin::Transform2D], [`Transform3D`][crate::builtin::Transform3D], [`Plane`][crate::builtin::Plane], [`Quaternion`][crate::builtin::Quaternion], [`AABB`][crate::builtin::Aabb], [`Basis`][crate::builtin::Basis], [`Projection`][crate::builtin::Projection], [`Color`][crate::builtin::Color], [`NodePath`][crate::builtin::NodePath], [`RID`][crate::builtin::Rid], [`Callable`][crate::builtin::Callable] and [`Signal`][crate::builtin::Signal].\n\nThese are [`Variant`][crate::builtin::Variant] reference types: [`Object`][crate::classes::Object], [`Dictionary`][crate::builtin::Dictionary], [`Array`][crate::builtin::Array], [`PackedByteArray`][crate::builtin::PackedByteArray], [`PackedInt32Array`][crate::builtin::PackedInt32Array], [`PackedInt64Array`][crate::builtin::PackedInt64Array], [`PackedFloat32Array`][crate::builtin::PackedFloat32Array], [`PackedFloat64Array`][crate::builtin::PackedFloat64Array], [`PackedStringArray`][crate::builtin::PackedStringArray], [`PackedVector2Array`][crate::builtin::PackedVector2Array], [`PackedVector3Array`][crate::builtin::PackedVector3Array], [`PackedVector4Array`][crate::builtin::PackedVector4Array], and [`PackedColorArray`][crate::builtin::PackedColorArray]."]
pub fn is_same(a: &Variant, b: &Variant,) -> bool {
    type CallRet = bool;
    type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
    let args = (RefArg::new(a), RefArg::new(b),);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_same;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_same", args)
    }
}