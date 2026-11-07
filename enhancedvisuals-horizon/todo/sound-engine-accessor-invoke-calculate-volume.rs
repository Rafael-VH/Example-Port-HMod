// Not ported yet: the Java @Invoker on SoundEngine.invokeCalculateVolume (SoundEngineAccessor.invokeCalculateVolume).
// Why it's off: No Horizon function mapped for SoundEngine.invokeCalculateVolume (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for SoundEngine.invokeCalculateVolume: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SoundEngine.instanceToChannel -> (not mapped yet)
//   SoundEngine.invokeCalculateVolume -> (not mapped yet)
//
// Decompiled Java, team/creative/enhancedvisuals/mixin/SoundEngineAccessor.java:16:
// | @Invoker
// | float invokeCalculateVolume(SoundInstance var1);
