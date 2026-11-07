// Not ported yet: the Java @Accessor on BiomeGenerationSettings.carvers (BiomeGenerationSettingsAccessor.getCarvers).
// Why it's off: No Horizon function mapped for BiomeGenerationSettings.carvers (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for BiomeGenerationSettings.carvers: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BiomeGenerationSettings.<init> -> (not mapped yet)
//   BiomeGenerationSettings.carvers -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/BiomeGenerationSettingsAccessor.java:20:
// | @Accessor("carvers")
// | @Mutable
// | HolderSet<WorldCarver> getCarvers();
