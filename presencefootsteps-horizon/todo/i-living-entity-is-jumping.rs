// Not ported yet: the Java @Accessor on LivingEntity.jumping (ILivingEntity.isJumping).
// Why it's off: No Horizon function mapped for LivingEntity.jumping (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for LivingEntity.jumping: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LivingEntity.jumping -> (not mapped yet)
//
// Decompiled Java, eu/ha3/presencefootsteps/mixins/ILivingEntity.java:9:
// | @Accessor("jumping")
// | boolean isJumping();
