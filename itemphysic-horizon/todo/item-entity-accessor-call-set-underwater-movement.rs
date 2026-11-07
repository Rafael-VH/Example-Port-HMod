// Not ported yet: the Java @Invoker on ItemEntity.callSetUnderwaterMovement (ItemEntityAccessor.callSetUnderwaterMovement).
// Why it's off: No Horizon function mapped for ItemEntity.callSetUnderwaterMovement (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for ItemEntity.callSetUnderwaterMovement: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ItemEntity.callSetUnderLavaMovement -> (not mapped yet)
//   ItemEntity.callSetUnderwaterMovement -> (not mapped yet)
//   ItemEntity.target -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/ItemEntityAccessor.java:14:
// | @Invoker
// | void callSetUnderwaterMovement();
