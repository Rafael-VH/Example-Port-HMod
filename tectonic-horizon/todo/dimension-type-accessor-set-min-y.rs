// Not ported yet: the Java @Accessor on DimensionType.minY (DimensionTypeAccessor.setMinY).
// Why it's off: No Horizon function mapped for DimensionType.minY (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for DimensionType.minY: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DimensionType.height -> (not mapped yet)
//   DimensionType.logicalHeight -> horizon_client::game::Dim::logical_height (suggested, high)
//   DimensionType.minY -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/DimensionTypeAccessor.java:10:
// | @Accessor("minY")
// | @Mutable
// | void setMinY(int var1);
