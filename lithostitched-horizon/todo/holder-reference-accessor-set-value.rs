// Not ported yet: the Java @Accessor on Reference.value (HolderReferenceAccessor.setValue).
// Why it's off: No Horizon function mapped for Reference.value (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Reference.value: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Reference.value -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/HolderReferenceAccessor.java:9:
// | @Accessor("value")
// | void setValue(T var1);
