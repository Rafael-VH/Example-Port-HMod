// Not ported yet: the Java @Accessor on SoundEngine.instanceToChannel (SoundEngineAccessor.getInstanceToChannel).
// Why it's off: No Horizon function mapped for SoundEngine.instanceToChannel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for SoundEngine.instanceToChannel: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SoundEngine.instanceToChannel -> (not mapped yet)
//   SoundEngine.invokeCalculateVolume -> (not mapped yet)
//
// Decompiled Java, team/creative/enhancedvisuals/mixin/SoundEngineAccessor.java:13:
// | @Accessor
// | Map<SoundInstance, ChannelHandle> getInstanceToChannel();
