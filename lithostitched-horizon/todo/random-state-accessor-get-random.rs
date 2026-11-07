// Not ported yet: the Java @Accessor on RandomState.random (RandomStateAccessor.getRandom).
// Why it's off: No Horizon function mapped for RandomState.random (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for RandomState.random: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RandomState.random -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/RandomStateAccessor.java:11:
// | @Accessor("random")
// | @Mutable
// | PositionalRandomFactory getRandom();
