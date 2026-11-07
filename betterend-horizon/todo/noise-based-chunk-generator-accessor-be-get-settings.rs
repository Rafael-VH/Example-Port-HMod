// Not ported yet: the Java @Accessor on NoiseBasedChunkGenerator.settings (NoiseBasedChunkGeneratorAccessor.be_getSettings).
// Why it's off: No Horizon function mapped for NoiseBasedChunkGenerator.settings (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for NoiseBasedChunkGenerator.settings: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   NoiseBasedChunkGenerator.settings -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/NoiseBasedChunkGeneratorAccessor.java:11:
// | @Accessor("settings")
// | Holder<NoiseGeneratorSettings> be_getSettings();
