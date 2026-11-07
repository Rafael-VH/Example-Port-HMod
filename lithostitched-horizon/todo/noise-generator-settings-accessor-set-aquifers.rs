// Not ported yet: the Java @Accessor on NoiseGeneratorSettings.aquifers (NoiseGeneratorSettingsAccessor.setAquifers).
// Why it's off: No Horizon function mapped for NoiseGeneratorSettings.aquifers (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for NoiseGeneratorSettings.aquifers: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   NoiseGeneratorSettings.aquifers -> (not mapped yet)
//   NoiseGeneratorSettings.noiseRouter -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/NoiseGeneratorSettingsAccessor.java:17:
// | @Accessor("aquifers")
// | @Mutable
// | void setAquifers(Optional<Config> var1);
