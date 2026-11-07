// Not ported yet: the Java @Accessor on Mob.goalSelector (MobEntityAccessor.graveyard$getGoalSelector).
// Why it's off: No Horizon function mapped for Mob.goalSelector (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Mob.goalSelector: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Mob.goalSelector -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/MobEntityAccessor.java:10:
// | @Accessor("goalSelector")
// | GoalSelector graveyard$getGoalSelector();
