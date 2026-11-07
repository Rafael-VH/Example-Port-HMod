// Not ported yet: the Java @Accessor on Properties.offsetFunction (BlockBehaviourPropertiesAccessor.betternether$getOffsetFunction).
// Why it's off: No Horizon function mapped for Properties.offsetFunction (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Properties.offsetFunction: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Properties.offsetFunction -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/BlockBehaviourPropertiesAccessor.java:10:
// | @Accessor("offsetFunction")
// | OffsetFunction betternether$getOffsetFunction();
