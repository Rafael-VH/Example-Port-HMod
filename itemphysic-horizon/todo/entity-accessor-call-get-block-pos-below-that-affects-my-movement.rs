// Not ported yet: the Java @Invoker on Entity.callGetBlockPosBelowThatAffectsMyMovement (EntityAccessor.callGetBlockPosBelowThatAffectsMyMovement).
// Why it's off: No Horizon function mapped for Entity.callGetBlockPosBelowThatAffectsMyMovement (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for Entity.callGetBlockPosBelowThatAffectsMyMovement: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.callGetBlockPosBelowThatAffectsMyMovement -> (not mapped yet)
//   Entity.callIsInvulnerableToBase -> (not mapped yet)
//   Entity.stuckSpeedMultiplier -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/EntityAccessor.java:16:
// | @Invoker
// | BlockPos callGetBlockPosBelowThatAffectsMyMovement();
