// Not ported yet: the Java @Accessor on CuboidItemModelWrapper.properties (CuboidItemModelWrapperAccessor.fdrf$setProperties).
// Why it's off: No Horizon function mapped for CuboidItemModelWrapper.properties (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for CuboidItemModelWrapper.properties: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CuboidItemModelWrapper.properties -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/CuboidItemModelWrapperAccessor.java:14:
// | @Accessor("properties")
// | @Mutable
// | void fdrf$setProperties(ModelRenderProperties var1);
