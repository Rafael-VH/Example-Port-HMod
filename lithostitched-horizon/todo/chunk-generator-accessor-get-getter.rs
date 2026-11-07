// Not ported yet: the Java @Accessor on ChunkGenerator.generationSettingsGetter (ChunkGeneratorAccessor.getGetter).
// Why it's off: No Horizon function mapped for ChunkGenerator.generationSettingsGetter (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for ChunkGenerator.generationSettingsGetter: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChunkGenerator.biomeSource -> (not mapped yet)
//   ChunkGenerator.featuresPerStep -> (not mapped yet)
//   ChunkGenerator.generationSettingsGetter -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/ChunkGeneratorAccessor.java:25:
// | @Accessor("generationSettingsGetter")
// | Function<Holder<Biome>, BiomeGenerationSettings> getGetter();
