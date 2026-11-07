// Not ported yet: the Java @Accessor on SoundBufferLibrary.resourceManager (SoundBufferLibraryAccessor.getResourceManager).
// Why it's off: No Horizon function mapped for SoundBufferLibrary.resourceManager (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for SoundBufferLibrary.resourceManager: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SoundBufferLibrary.resourceManager -> (not mapped yet)
//
// Decompiled Java, team/creative/ambientsounds/mixin/SoundBufferLibraryAccessor.java:10:
// | @Accessor
// | ResourceProvider getResourceManager();
