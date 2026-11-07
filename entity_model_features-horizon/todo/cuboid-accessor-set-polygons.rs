// Not ported yet: the Java @Accessor on Cube.polygons (CuboidAccessor.setPolygons).
// Why it's off: No Horizon function mapped for Cube.polygons (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Cube.polygons: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Cube.maxX -> (not mapped yet)
//   Cube.maxY -> (not mapped yet)
//   Cube.maxZ -> (not mapped yet)
//   Cube.minX -> (not mapped yet)
//   Cube.minY -> (not mapped yet)
//   Cube.minZ -> (not mapped yet)
//   Cube.polygons -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/accessor/CuboidAccessor.java:11:
// | @Mutable
// | @Accessor
// | void setPolygons(Polygon[] var1);
