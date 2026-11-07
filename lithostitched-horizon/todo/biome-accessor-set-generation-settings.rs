// Not ported yet: the Java @Accessor on Biome.generationSettings (BiomeAccessor.setGenerationSettings).
// Why it's off: No Horizon function mapped for Biome.generationSettings (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Biome.generationSettings: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Biome.attributes -> (not mapped yet)
//   Biome.climateSettings -> (not mapped yet)
//   Biome.generationSettings -> (not mapped yet)
//   Biome.specialEffects -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/BiomeAccessor.java:29:
// | @Accessor("generationSettings")
// | @Mutable
// | void setGenerationSettings(BiomeGenerationSettings var1);
