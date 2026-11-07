// Not ported yet: the Java @Accessor on AbstractSoundInstance.volume (AbstractSoundInstanceAccessor.setVolume).
// Why it's off: No Horizon function mapped for AbstractSoundInstance.volume (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for AbstractSoundInstance.volume: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AbstractSoundInstance.volume -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/client/AbstractSoundInstanceAccessor.java:9:
// | @Accessor("volume")
// | void setVolume(float var1);
