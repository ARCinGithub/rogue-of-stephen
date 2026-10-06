#![doc = "Sidecar module for class [`Geometry2D`][crate::classes::Geometry2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Geometry2D` enums](https://docs.godotengine.org/en/stable/classes/class_geometry2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Geometry2D`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`geometry_2d`][crate::classes::geometry_2d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Geometry2D`](https://docs.godotengine.org/en/stable/classes/class_geometry2d.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nProvides a set of helper functions to create geometric shapes, compute intersections between shapes, and process various other geometric operations in 2D."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Geometry2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Geometry2D {
        #[doc = "Returns `true` if `point` is inside the circle or if it's located exactly _on_ the circle's boundary, otherwise returns `false`."]
        pub fn is_point_in_circle(&self, point: Vector2, circle_position: Vector2, circle_radius: f32,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2, Vector2, f32,);
            let args = (point, circle_position, circle_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "is_point_in_circle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Given the 2D segment (`segment_from`, `segment_to`), returns the position on the segment (as a number between 0 and 1) at which the segment hits the circle that is located at position `circle_position` and has radius `circle_radius`. If the segment does not intersect the circle, -1 is returned (this is also the case if the line extending the segment would intersect the circle, but the segment does not)."]
        pub fn segment_intersects_circle(&mut self, segment_from: Vector2, segment_to: Vector2, circle_position: Vector2, circle_radius: f32,) -> f32 {
            type CallRet = f32;
            type CallParams = (Vector2, Vector2, Vector2, f32,);
            let args = (segment_from, segment_to, circle_position, circle_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "segment_intersects_circle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Checks if the two segments (`from_a`, `to_a`) and (`from_b`, `to_b`) intersect. If yes, return the point of intersection as [`Vector2`][crate::builtin::Vector2]. If no intersection takes place, returns `null`."]
        pub fn segment_intersects_segment(&mut self, from_a: Vector2, to_a: Vector2, from_b: Vector2, to_b: Vector2,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Vector2, Vector2, Vector2, Vector2,);
            let args = (from_a, to_a, from_b, to_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "segment_intersects_segment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the point of intersection between the two lines (`from_a`, `dir_a`) and (`from_b`, `dir_b`). Returns a [`Vector2`][crate::builtin::Vector2], or `null` if the lines are parallel.\n\n`from` and `dir` are _not_ endpoints of a line segment or ray but the slope (`dir`) and a known point (`from`) on that line.\n\n\n```gdscript\nvar from_a = Vector2.ZERO\nvar dir_a = Vector2.RIGHT\nvar from_b = Vector2.DOWN\n\n# Returns Vector2(1, 0)\nGeometry2D.line_intersects_line(from_a, dir_a, from_b, Vector2(1, -1))\n# Returns Vector2(-1, 0)\nGeometry2D.line_intersects_line(from_a, dir_a, from_b, Vector2(-1, -1))\n# Returns null\nGeometry2D.line_intersects_line(from_a, dir_a, from_b, Vector2.RIGHT)\n```\n"]
        pub fn line_intersects_line(&mut self, from_a: Vector2, dir_a: Vector2, from_b: Vector2, dir_b: Vector2,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Vector2, Vector2, Vector2, Vector2,);
            let args = (from_a, dir_a, from_b, dir_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "line_intersects_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Given the two 2D segments (`p1`, `q1`) and (`p2`, `q2`), finds those two points on the two segments that are closest to each other. Returns a [`PackedVector2Array`][crate::builtin::PackedVector2Array] that contains this point on (`p1`, `q1`) as well the accompanying point on (`p2`, `q2`)."]
        pub fn get_closest_points_between_segments(&self, p1: Vector2, q1: Vector2, p2: Vector2, q2: Vector2,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (Vector2, Vector2, Vector2, Vector2,);
            let args = (p1, q1, p2, q2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "get_closest_points_between_segments", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the 2D point on the 2D segment (`s1`, `s2`) that is closest to `point`. The returned point will always be inside the specified segment."]
        pub fn get_closest_point_to_segment(&self, point: Vector2, s1: Vector2, s2: Vector2,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2, Vector2, Vector2,);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "get_closest_point_to_segment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the 2D point on the 2D line defined by (`s1`, `s2`) that is closest to `point`. The returned point can be inside the segment (`s1`, `s2`) or outside of it, i.e. somewhere on the line extending from the segment."]
        pub fn get_closest_point_to_segment_uncapped(&self, point: Vector2, s1: Vector2, s2: Vector2,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2, Vector2, Vector2,);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "get_closest_point_to_segment_uncapped", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if `point` is inside the triangle specified by `a`, `b` and `c`."]
        pub fn point_is_inside_triangle(&self, point: Vector2, a: Vector2, b: Vector2, c: Vector2,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2, Vector2, Vector2, Vector2,);
            let args = (point, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "point_is_inside_triangle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if `polygon`'s vertices are ordered in clockwise order, otherwise returns `false`.\n\n**Note:** Assumes a Cartesian coordinate system where `+x` is right and `+y` is up. If using screen coordinates (`+y` is down), the result will need to be flipped (i.e. a `true` result will indicate counter-clockwise)."]
        pub fn is_polygon_clockwise(&self, polygon: &PackedVector2Array,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "is_polygon_clockwise", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if `point` is inside `polygon` or if it's located exactly _on_ polygon's boundary, otherwise returns `false`."]
        pub fn is_point_in_polygon(&self, point: Vector2, polygon: &PackedVector2Array,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Vector2, RefArg < 'a0, PackedVector2Array >,);
            let args = (point, RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "is_point_in_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Triangulates the polygon specified by the points in `polygon`. Returns a [`PackedInt32Array`][crate::builtin::PackedInt32Array] where each triangle consists of three consecutive point indices into `polygon` (i.e. the returned array will have `n * 3` elements, with `n` being the number of found triangles). Output triangles will always be counter clockwise, and the contour will be flipped if it's clockwise. If the triangulation did not succeed, an empty [`PackedInt32Array`][crate::builtin::PackedInt32Array] is returned."]
        pub fn triangulate_polygon(&mut self, polygon: &PackedVector2Array,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "triangulate_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Triangulates the area specified by discrete set of `points` such that no point is inside the circumcircle of any resulting triangle. Returns a [`PackedInt32Array`][crate::builtin::PackedInt32Array] where each triangle consists of three consecutive point indices into `points` (i.e. the returned array will have `n * 3` elements, with `n` being the number of found triangles). If the triangulation did not succeed, an empty [`PackedInt32Array`][crate::builtin::PackedInt32Array] is returned."]
        pub fn triangulate_delaunay(&mut self, points: &PackedVector2Array,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(points),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "triangulate_delaunay", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Given an array of [`Vector2`][crate::builtin::Vector2]s, returns the convex hull as a list of points in counterclockwise order. The last point is the same as the first one."]
        pub fn convex_hull(&mut self, points: &PackedVector2Array,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(points),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "convex_hull", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Decomposes the `polygon` into multiple convex hulls and returns an array of [`PackedVector2Array`][crate::builtin::PackedVector2Array]."]
        pub fn decompose_polygon_in_convex(&mut self, polygon: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallRet = Array < PackedVector2Array >;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "decompose_polygon_in_convex", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Merges (combines) `polygon_a` and `polygon_b` and returns an array of merged polygons. This performs [`PolyBooleanOperation::UNION`][`crate::classes::geometry_2d::PolyBooleanOperation::UNION`] between polygons.\n\nThe operation may result in an outer polygon (boundary) and multiple inner polygons (holes) produced which could be distinguished by calling [`is_polygon_clockwise`][`crate::classes::Geometry2D::is_polygon_clockwise`]."]
        pub fn merge_polygons(&mut self, polygon_a: &PackedVector2Array, polygon_b: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallRet = Array < PackedVector2Array >;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >,);
            let args = (RefArg::new(polygon_a), RefArg::new(polygon_b),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "merge_polygons", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clips `polygon_a` against `polygon_b` and returns an array of clipped polygons. This performs [`PolyBooleanOperation::DIFFERENCE`][`crate::classes::geometry_2d::PolyBooleanOperation::DIFFERENCE`] between polygons. Returns an empty array if `polygon_b` completely overlaps `polygon_a`.\n\nIf `polygon_b` is enclosed by `polygon_a`, returns an outer polygon (boundary) and inner polygon (hole) which could be distinguished by calling [`is_polygon_clockwise`][`crate::classes::Geometry2D::is_polygon_clockwise`]."]
        pub fn clip_polygons(&mut self, polygon_a: &PackedVector2Array, polygon_b: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallRet = Array < PackedVector2Array >;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >,);
            let args = (RefArg::new(polygon_a), RefArg::new(polygon_b),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "clip_polygons", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Intersects `polygon_a` with `polygon_b` and returns an array of intersected polygons. This performs [`PolyBooleanOperation::INTERSECTION`][`crate::classes::geometry_2d::PolyBooleanOperation::INTERSECTION`] between polygons. In other words, returns common area shared by polygons. Returns an empty array if no intersection occurs.\n\nThe operation may result in an outer polygon (boundary) and inner polygon (hole) produced which could be distinguished by calling [`is_polygon_clockwise`][`crate::classes::Geometry2D::is_polygon_clockwise`]."]
        pub fn intersect_polygons(&mut self, polygon_a: &PackedVector2Array, polygon_b: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallRet = Array < PackedVector2Array >;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >,);
            let args = (RefArg::new(polygon_a), RefArg::new(polygon_b),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "intersect_polygons", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Mutually excludes common area defined by intersection of `polygon_a` and `polygon_b` (see [`intersect_polygons`][`crate::classes::Geometry2D::intersect_polygons`]) and returns an array of excluded polygons. This performs [`PolyBooleanOperation::XOR`][`crate::classes::geometry_2d::PolyBooleanOperation::XOR`] between polygons. In other words, returns all but common area between polygons.\n\nThe operation may result in an outer polygon (boundary) and inner polygon (hole) produced which could be distinguished by calling [`is_polygon_clockwise`][`crate::classes::Geometry2D::is_polygon_clockwise`]."]
        pub fn exclude_polygons(&mut self, polygon_a: &PackedVector2Array, polygon_b: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallRet = Array < PackedVector2Array >;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >,);
            let args = (RefArg::new(polygon_a), RefArg::new(polygon_b),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "exclude_polygons", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clips `polyline` against `polygon` and returns an array of clipped polylines. This performs [`PolyBooleanOperation::DIFFERENCE`][`crate::classes::geometry_2d::PolyBooleanOperation::DIFFERENCE`] between the polyline and the polygon. This operation can be thought of as cutting a line with a closed shape."]
        pub fn clip_polyline_with_polygon(&mut self, polyline: &PackedVector2Array, polygon: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallRet = Array < PackedVector2Array >;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >,);
            let args = (RefArg::new(polyline), RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "clip_polyline_with_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Intersects `polyline` with `polygon` and returns an array of intersected polylines. This performs [`PolyBooleanOperation::INTERSECTION`][`crate::classes::geometry_2d::PolyBooleanOperation::INTERSECTION`] between the polyline and the polygon. This operation can be thought of as chopping a line with a closed shape."]
        pub fn intersect_polyline_with_polygon(&mut self, polyline: &PackedVector2Array, polygon: &PackedVector2Array,) -> Array < PackedVector2Array > {
            type CallRet = Array < PackedVector2Array >;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedVector2Array >,);
            let args = (RefArg::new(polyline), RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "intersect_polyline_with_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Inflates or deflates `polygon` by `delta` units (pixels). If `delta` is positive, makes the polygon grow outward. If `delta` is negative, shrinks the polygon inward. Returns an array of polygons because inflating/deflating may result in multiple discrete polygons. Returns an empty array if `delta` is negative and the absolute value of it approximately exceeds the minimum bounding rectangle dimensions of the polygon.\n\nEach polygon's vertices will be rounded as determined by `join_type`.\n\nThe operation may result in an outer polygon (boundary) and inner polygon (hole) produced which could be distinguished by calling [`is_polygon_clockwise`][`crate::classes::Geometry2D::is_polygon_clockwise`].\n\n**Note:** To translate the polygon's vertices specifically, multiply them to a [`Transform2D`][crate::builtin::Transform2D]:\n\n\n```gdscript\nvar polygon = PackedVector2Array([Vector2(0, 0), Vector2(100, 0), Vector2(100, 100), Vector2(0, 100)])\nvar offset = Vector2(50, 50)\npolygon = Transform2D(0, offset) * polygon\nprint(polygon) # Prints [(50.0, 50.0), (150.0, 50.0), (150.0, 150.0), (50.0, 150.0)]\n```\n"]
        pub(crate) fn offset_polygon_full(&mut self, polygon: RefArg < PackedVector2Array >, delta: f32, join_type: crate::classes::geometry_2d::PolyJoinType,) -> Array < PackedVector2Array > {
            type CallRet = Array < PackedVector2Array >;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >, f32, crate::classes::geometry_2d::PolyJoinType,);
            let args = (polygon, delta, join_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "offset_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`offset_polygon_ex`][Self::offset_polygon_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Inflates or deflates `polygon` by `delta` units (pixels). If `delta` is positive, makes the polygon grow outward. If `delta` is negative, shrinks the polygon inward. Returns an array of polygons because inflating/deflating may result in multiple discrete polygons. Returns an empty array if `delta` is negative and the absolute value of it approximately exceeds the minimum bounding rectangle dimensions of the polygon.\n\nEach polygon's vertices will be rounded as determined by `join_type`.\n\nThe operation may result in an outer polygon (boundary) and inner polygon (hole) produced which could be distinguished by calling [`is_polygon_clockwise`][`crate::classes::Geometry2D::is_polygon_clockwise`].\n\n**Note:** To translate the polygon's vertices specifically, multiply them to a [`Transform2D`][crate::builtin::Transform2D]:\n\n\n```gdscript\nvar polygon = PackedVector2Array([Vector2(0, 0), Vector2(100, 0), Vector2(100, 100), Vector2(0, 100)])\nvar offset = Vector2(50, 50)\npolygon = Transform2D(0, offset) * polygon\nprint(polygon) # Prints [(50.0, 50.0), (150.0, 50.0), (150.0, 150.0), (50.0, 150.0)]\n```\n"]
        #[inline]
        pub fn offset_polygon(&mut self, polygon: &PackedVector2Array, delta: f32,) -> Array < PackedVector2Array > {
            self.offset_polygon_ex(polygon, delta,) . done()
        }
        #[doc = "Inflates or deflates `polygon` by `delta` units (pixels). If `delta` is positive, makes the polygon grow outward. If `delta` is negative, shrinks the polygon inward. Returns an array of polygons because inflating/deflating may result in multiple discrete polygons. Returns an empty array if `delta` is negative and the absolute value of it approximately exceeds the minimum bounding rectangle dimensions of the polygon.\n\nEach polygon's vertices will be rounded as determined by `join_type`.\n\nThe operation may result in an outer polygon (boundary) and inner polygon (hole) produced which could be distinguished by calling [`is_polygon_clockwise`][`crate::classes::Geometry2D::is_polygon_clockwise`].\n\n**Note:** To translate the polygon's vertices specifically, multiply them to a [`Transform2D`][crate::builtin::Transform2D]:\n\n\n```gdscript\nvar polygon = PackedVector2Array([Vector2(0, 0), Vector2(100, 0), Vector2(100, 100), Vector2(0, 100)])\nvar offset = Vector2(50, 50)\npolygon = Transform2D(0, offset) * polygon\nprint(polygon) # Prints [(50.0, 50.0), (150.0, 50.0), (150.0, 150.0), (50.0, 150.0)]\n```\n"]
        #[inline]
        pub fn offset_polygon_ex < 'ex > (&'ex mut self, polygon: &'ex PackedVector2Array, delta: f32,) -> ExOffsetPolygon < 'ex > {
            ExOffsetPolygon::new(self, polygon, delta,)
        }
        #[doc = "Inflates or deflates `polyline` by `delta` units (pixels), producing polygons. If `delta` is positive, makes the polyline grow outward. Returns an array of polygons because inflating/deflating may result in multiple discrete polygons. If `delta` is negative, returns an empty array.\n\nEach polygon's vertices will be rounded as determined by `join_type`.\n\nEach polygon's endpoints will be rounded as determined by `end_type`.\n\nThe operation may result in an outer polygon (boundary) and inner polygon (hole) produced which could be distinguished by calling [`is_polygon_clockwise`][`crate::classes::Geometry2D::is_polygon_clockwise`]."]
        pub(crate) fn offset_polyline_full(&mut self, polyline: RefArg < PackedVector2Array >, delta: f32, join_type: crate::classes::geometry_2d::PolyJoinType, end_type: crate::classes::geometry_2d::PolyEndType,) -> Array < PackedVector2Array > {
            type CallRet = Array < PackedVector2Array >;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >, f32, crate::classes::geometry_2d::PolyJoinType, crate::classes::geometry_2d::PolyEndType,);
            let args = (polyline, delta, join_type, end_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "offset_polyline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`offset_polyline_ex`][Self::offset_polyline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Inflates or deflates `polyline` by `delta` units (pixels), producing polygons. If `delta` is positive, makes the polyline grow outward. Returns an array of polygons because inflating/deflating may result in multiple discrete polygons. If `delta` is negative, returns an empty array.\n\nEach polygon's vertices will be rounded as determined by `join_type`.\n\nEach polygon's endpoints will be rounded as determined by `end_type`.\n\nThe operation may result in an outer polygon (boundary) and inner polygon (hole) produced which could be distinguished by calling [`is_polygon_clockwise`][`crate::classes::Geometry2D::is_polygon_clockwise`]."]
        #[inline]
        pub fn offset_polyline(&mut self, polyline: &PackedVector2Array, delta: f32,) -> Array < PackedVector2Array > {
            self.offset_polyline_ex(polyline, delta,) . done()
        }
        #[doc = "Inflates or deflates `polyline` by `delta` units (pixels), producing polygons. If `delta` is positive, makes the polyline grow outward. Returns an array of polygons because inflating/deflating may result in multiple discrete polygons. If `delta` is negative, returns an empty array.\n\nEach polygon's vertices will be rounded as determined by `join_type`.\n\nEach polygon's endpoints will be rounded as determined by `end_type`.\n\nThe operation may result in an outer polygon (boundary) and inner polygon (hole) produced which could be distinguished by calling [`is_polygon_clockwise`][`crate::classes::Geometry2D::is_polygon_clockwise`]."]
        #[inline]
        pub fn offset_polyline_ex < 'ex > (&'ex mut self, polyline: &'ex PackedVector2Array, delta: f32,) -> ExOffsetPolyline < 'ex > {
            ExOffsetPolyline::new(self, polyline, delta,)
        }
        #[doc = "Given an array of [`Vector2`][crate::builtin::Vector2]s representing tiles, builds an atlas. The returned dictionary has two keys: `points` is a [`PackedVector2Array`][crate::builtin::PackedVector2Array] that specifies the positions of each tile, `size` contains the overall size of the whole atlas as [`Vector2i`][crate::builtin::Vector2i]."]
        pub fn make_atlas(&mut self, sizes: &PackedVector2Array,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(sizes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "make_atlas", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [Bresenham line](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm) between the `from` and `to` points. A Bresenham line is a series of pixels that draws a line and is always 1-pixel thick on every row and column of the drawing (never more, never less).\n\nExample code to draw a line between two [`Marker2D`][crate::classes::Marker2D] nodes using a series of [`draw_rect`][`crate::classes::CanvasItem::draw_rect`] calls:\n\n```gdscript\nfunc _draw():\n\tfor pixel in Geometry2D.bresenham_line($MarkerA.position, $MarkerB.position):\n\t\tdraw_rect(Rect2(pixel, Vector2.ONE), Color.WHITE)\n```"]
        pub fn bresenham_line(&mut self, from: Vector2i, to: Vector2i,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (Vector2i, Vector2i,);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry2D", "bresenham_line", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Geometry2D {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Geometry2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Geometry2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Geometry2D {
        
    }
    impl crate::obj::Singleton for Geometry2D {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"Geometry2D"))
            }
        }
    }
    impl std::ops::Deref for Geometry2D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Geometry2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Geometry2D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Geometry2D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Geometry2D::offset_polygon_ex`][super::Geometry2D::offset_polygon_ex]."]
#[must_use]
pub struct ExOffsetPolygon < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Geometry2D, polygon: CowArg < 'ex, PackedVector2Array >, delta: f32, join_type: crate::classes::geometry_2d::PolyJoinType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExOffsetPolygon < 'ex > {
    fn new(surround_object: &'ex mut re_export::Geometry2D, polygon: &'ex PackedVector2Array, delta: f32,) -> Self {
        let join_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, polygon: CowArg::Borrowed(polygon), delta: delta, join_type: join_type,
        }
    }
    #[inline]
    pub fn join_type(self, join_type: crate::classes::geometry_2d::PolyJoinType) -> Self {
        Self {
            join_type: join_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < PackedVector2Array > {
        let Self {
            _phantom, surround_object, polygon, delta, join_type,
        }
        = self;
        re_export::Geometry2D::offset_polygon_full(surround_object, polygon.cow_as_arg(), delta, join_type,)
    }
}
#[doc = "Default-param extender for [`Geometry2D::offset_polyline_ex`][super::Geometry2D::offset_polyline_ex]."]
#[must_use]
pub struct ExOffsetPolyline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Geometry2D, polyline: CowArg < 'ex, PackedVector2Array >, delta: f32, join_type: crate::classes::geometry_2d::PolyJoinType, end_type: crate::classes::geometry_2d::PolyEndType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExOffsetPolyline < 'ex > {
    fn new(surround_object: &'ex mut re_export::Geometry2D, polyline: &'ex PackedVector2Array, delta: f32,) -> Self {
        let join_type = crate::obj::EngineEnum::from_ord(0);
        let end_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, polyline: CowArg::Borrowed(polyline), delta: delta, join_type: join_type, end_type: end_type,
        }
    }
    #[inline]
    pub fn join_type(self, join_type: crate::classes::geometry_2d::PolyJoinType) -> Self {
        Self {
            join_type: join_type, .. self
        }
    }
    #[inline]
    pub fn end_type(self, end_type: crate::classes::geometry_2d::PolyEndType) -> Self {
        Self {
            end_type: end_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < PackedVector2Array > {
        let Self {
            _phantom, surround_object, polyline, delta, join_type, end_type,
        }
        = self;
        re_export::Geometry2D::offset_polyline_full(surround_object, polyline.cow_as_arg(), delta, join_type, end_type,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PolyBooleanOperation {
    ord: i32
}
impl PolyBooleanOperation {
    #[doc(alias = "OPERATION_UNION")]
    #[doc = "Godot enumerator name: `OPERATION_UNION`"]
    pub const UNION: PolyBooleanOperation = PolyBooleanOperation {
        ord: 0i32
    };
    #[doc(alias = "OPERATION_DIFFERENCE")]
    #[doc = "Godot enumerator name: `OPERATION_DIFFERENCE`"]
    pub const DIFFERENCE: PolyBooleanOperation = PolyBooleanOperation {
        ord: 1i32
    };
    #[doc(alias = "OPERATION_INTERSECTION")]
    #[doc = "Godot enumerator name: `OPERATION_INTERSECTION`"]
    pub const INTERSECTION: PolyBooleanOperation = PolyBooleanOperation {
        ord: 2i32
    };
    #[doc(alias = "OPERATION_XOR")]
    #[doc = "Godot enumerator name: `OPERATION_XOR`"]
    pub const XOR: PolyBooleanOperation = PolyBooleanOperation {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for PolyBooleanOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PolyBooleanOperation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PolyBooleanOperation {
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
            Self::UNION => "UNION", Self::DIFFERENCE => "DIFFERENCE", Self::INTERSECTION => "INTERSECTION", Self::XOR => "XOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PolyBooleanOperation::UNION, PolyBooleanOperation::DIFFERENCE, PolyBooleanOperation::INTERSECTION, PolyBooleanOperation::XOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PolyBooleanOperation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNION", "OPERATION_UNION", PolyBooleanOperation::UNION), crate::meta::inspect::EnumConstant::new("DIFFERENCE", "OPERATION_DIFFERENCE", PolyBooleanOperation::DIFFERENCE), crate::meta::inspect::EnumConstant::new("INTERSECTION", "OPERATION_INTERSECTION", PolyBooleanOperation::INTERSECTION), crate::meta::inspect::EnumConstant::new("XOR", "OPERATION_XOR", PolyBooleanOperation::XOR)]
        }
    }
}
impl crate::meta::GodotConvert for PolyBooleanOperation {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Operation Union", 0i64), EnumeratorShape::new_int("Operation Difference", 1i64), EnumeratorShape::new_int("Operation Intersection", 2i64), EnumeratorShape::new_int("Operation Xor", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Geometry2D.PolyBooleanOperation")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PolyBooleanOperation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PolyBooleanOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PolyBooleanOperation {
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
impl crate::registry::property::Export for PolyBooleanOperation {
    
}
impl crate::meta::Element for PolyBooleanOperation {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PolyJoinType {
    ord: i32
}
impl PolyJoinType {
    #[doc(alias = "JOIN_SQUARE")]
    #[doc = "Godot enumerator name: `JOIN_SQUARE`"]
    pub const SQUARE: PolyJoinType = PolyJoinType {
        ord: 0i32
    };
    #[doc(alias = "JOIN_ROUND")]
    #[doc = "Godot enumerator name: `JOIN_ROUND`"]
    pub const ROUND: PolyJoinType = PolyJoinType {
        ord: 1i32
    };
    #[doc(alias = "JOIN_MITER")]
    #[doc = "Godot enumerator name: `JOIN_MITER`"]
    pub const MITER: PolyJoinType = PolyJoinType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PolyJoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PolyJoinType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PolyJoinType {
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
            Self::SQUARE => "SQUARE", Self::ROUND => "ROUND", Self::MITER => "MITER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PolyJoinType::SQUARE, PolyJoinType::ROUND, PolyJoinType::MITER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PolyJoinType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SQUARE", "JOIN_SQUARE", PolyJoinType::SQUARE), crate::meta::inspect::EnumConstant::new("ROUND", "JOIN_ROUND", PolyJoinType::ROUND), crate::meta::inspect::EnumConstant::new("MITER", "JOIN_MITER", PolyJoinType::MITER)]
        }
    }
}
impl crate::meta::GodotConvert for PolyJoinType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Join Square", 0i64), EnumeratorShape::new_int("Join Round", 1i64), EnumeratorShape::new_int("Join Miter", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Geometry2D.PolyJoinType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PolyJoinType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PolyJoinType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PolyJoinType {
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
impl crate::registry::property::Export for PolyJoinType {
    
}
impl crate::meta::Element for PolyJoinType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PolyEndType {
    ord: i32
}
impl PolyEndType {
    #[doc(alias = "END_POLYGON")]
    #[doc = "Godot enumerator name: `END_POLYGON`"]
    pub const POLYGON: PolyEndType = PolyEndType {
        ord: 0i32
    };
    #[doc(alias = "END_JOINED")]
    #[doc = "Godot enumerator name: `END_JOINED`"]
    pub const JOINED: PolyEndType = PolyEndType {
        ord: 1i32
    };
    #[doc(alias = "END_BUTT")]
    #[doc = "Godot enumerator name: `END_BUTT`"]
    pub const BUTT: PolyEndType = PolyEndType {
        ord: 2i32
    };
    #[doc(alias = "END_SQUARE")]
    #[doc = "Godot enumerator name: `END_SQUARE`"]
    pub const SQUARE: PolyEndType = PolyEndType {
        ord: 3i32
    };
    #[doc(alias = "END_ROUND")]
    #[doc = "Godot enumerator name: `END_ROUND`"]
    pub const ROUND: PolyEndType = PolyEndType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for PolyEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PolyEndType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PolyEndType {
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
            Self::POLYGON => "POLYGON", Self::JOINED => "JOINED", Self::BUTT => "BUTT", Self::SQUARE => "SQUARE", Self::ROUND => "ROUND", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PolyEndType::POLYGON, PolyEndType::JOINED, PolyEndType::BUTT, PolyEndType::SQUARE, PolyEndType::ROUND]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PolyEndType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POLYGON", "END_POLYGON", PolyEndType::POLYGON), crate::meta::inspect::EnumConstant::new("JOINED", "END_JOINED", PolyEndType::JOINED), crate::meta::inspect::EnumConstant::new("BUTT", "END_BUTT", PolyEndType::BUTT), crate::meta::inspect::EnumConstant::new("SQUARE", "END_SQUARE", PolyEndType::SQUARE), crate::meta::inspect::EnumConstant::new("ROUND", "END_ROUND", PolyEndType::ROUND)]
        }
    }
}
impl crate::meta::GodotConvert for PolyEndType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("End Polygon", 0i64), EnumeratorShape::new_int("End Joined", 1i64), EnumeratorShape::new_int("End Butt", 2i64), EnumeratorShape::new_int("End Square", 3i64), EnumeratorShape::new_int("End Round", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Geometry2D.PolyEndType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PolyEndType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PolyEndType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PolyEndType {
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
impl crate::registry::property::Export for PolyEndType {
    
}
impl crate::meta::Element for PolyEndType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Geometry2D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Geometry2D {
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