// Not ported yet: the Java @Accessor on SoundManager.soundEngine (SoundManagerAccessor.getSoundEngine).
// Why it's off: No Horizon function mapped for SoundManager.soundEngine (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for SoundManager.soundEngine: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SoundManager.soundEngine -> (not mapped yet)
//
// Decompiled Java, team/creative/enhancedvisuals/mixin/SoundManagerAccessor.java:10:
// | @Accessor
// | SoundEngine getSoundEngine();
