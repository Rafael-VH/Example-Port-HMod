// Not ported yet: the Java @Accessor on ItemEntity.target (ItemEntityAccessor.getTarget).
// Why it's off: No Horizon function mapped for ItemEntity.target (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for ItemEntity.target: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ItemEntity.callSetUnderLavaMovement -> (not mapped yet)
//   ItemEntity.callSetUnderwaterMovement -> (not mapped yet)
//   ItemEntity.target -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/ItemEntityAccessor.java:11:
// | @Accessor
// | UUID getTarget();
