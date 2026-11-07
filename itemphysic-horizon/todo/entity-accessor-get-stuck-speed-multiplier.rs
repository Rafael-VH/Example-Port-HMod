// Not ported yet: the Java @Accessor on Entity.stuckSpeedMultiplier (EntityAccessor.getStuckSpeedMultiplier).
// Why it's off: No Horizon function mapped for Entity.stuckSpeedMultiplier (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Entity.stuckSpeedMultiplier: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.callGetBlockPosBelowThatAffectsMyMovement -> (not mapped yet)
//   Entity.callIsInvulnerableToBase -> (not mapped yet)
//   Entity.stuckSpeedMultiplier -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/EntityAccessor.java:13:
// | @Accessor("stuckSpeedMultiplier")
// | Vec3 getStuckSpeedMultiplier();
