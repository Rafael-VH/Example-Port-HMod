// Not ported yet: the Java @Accessor on ParameterList.values (ParameterListAccessor.lithostitched$setValues).
// Why it's off: No Horizon function mapped for ParameterList.values (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for ParameterList.values: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ParameterList.index -> (not mapped yet)
//   ParameterList.values -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/mnbs/ParameterListAccessor.java:15:
// | @Accessor("values")
// | @Mutable
// | @Final
// | void lithostitched$setValues(List<Pair<ParameterPoint, T>> var1);
