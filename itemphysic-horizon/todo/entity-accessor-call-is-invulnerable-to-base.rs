// Not ported yet: the Java @Invoker on Entity.callIsInvulnerableToBase (EntityAccessor.callIsInvulnerableToBase).
// Why it's off: No Horizon function mapped for Entity.callIsInvulnerableToBase (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for Entity.callIsInvulnerableToBase: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.callGetBlockPosBelowThatAffectsMyMovement -> (not mapped yet)
//   Entity.callIsInvulnerableToBase -> (not mapped yet)
//   Entity.stuckSpeedMultiplier -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/EntityAccessor.java:19:
// | @Invoker
// | boolean callIsInvulnerableToBase(DamageSource var1);
