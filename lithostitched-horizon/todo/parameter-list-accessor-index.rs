// Not ported yet: the Java @Accessor on ParameterList.index (ParameterListAccessor.lithostitched$index).
// Why it's off: No Horizon function mapped for ParameterList.index (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for ParameterList.index: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ParameterList.index -> (not mapped yet)
//   ParameterList.values -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/mnbs/ParameterListAccessor.java:20:
// | @Accessor("index")
// | @Mutable
// | @Final
// | void lithostitched$index(RTree<T> var1);
