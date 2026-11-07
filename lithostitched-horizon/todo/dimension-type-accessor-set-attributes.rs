// Not ported yet: the Java @Accessor on DimensionType.attributes (DimensionTypeAccessor.setAttributes).
// Why it's off: No Horizon function mapped for DimensionType.attributes (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for DimensionType.attributes: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DimensionType.attributes -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/DimensionTypeAccessor.java:11:
// | @Accessor("attributes")
// | @Mutable
// | void setAttributes(EnvironmentAttributeMap var1);
