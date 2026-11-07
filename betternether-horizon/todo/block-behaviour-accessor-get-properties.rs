// Not ported yet: the Java @Accessor on BlockBehaviour.properties (BlockBehaviourAccessor.getProperties).
// Why it's off: No Horizon function mapped for BlockBehaviour.properties (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for BlockBehaviour.properties: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockBehaviour.properties -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/BlockBehaviourAccessor.java:10:
// | @Accessor("properties")
// | Properties getProperties();
