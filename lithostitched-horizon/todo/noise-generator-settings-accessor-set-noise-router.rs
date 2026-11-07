// Not ported yet: the Java @Accessor on NoiseGeneratorSettings.noiseRouter (NoiseGeneratorSettingsAccessor.setNoiseRouter).
// Why it's off: No Horizon function mapped for NoiseGeneratorSettings.noiseRouter (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for NoiseGeneratorSettings.noiseRouter: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   NoiseGeneratorSettings.aquifers -> (not mapped yet)
//   NoiseGeneratorSettings.noiseRouter -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/NoiseGeneratorSettingsAccessor.java:13:
// | @Accessor("noiseRouter")
// | @Mutable
// | void setNoiseRouter(NoiseRouter var1);
