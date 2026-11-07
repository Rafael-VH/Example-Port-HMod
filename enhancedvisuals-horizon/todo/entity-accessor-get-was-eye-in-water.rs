// Not ported yet: the Java @Accessor on Entity.wasEyeInWater (EntityAccessor.getWasEyeInWater).
// Why it's off: No Horizon function mapped for Entity.wasEyeInWater (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Entity.wasEyeInWater: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.wasEyeInWater -> (not mapped yet)
//
// Decompiled Java, team/creative/enhancedvisuals/mixin/EntityAccessor.java:9:
// | @Accessor
// | boolean getWasEyeInWater();
