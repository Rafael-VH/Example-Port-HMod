// Not ported yet: the Java @Accessor on Biome.attributes (BiomeAccessor.setAttributes).
// Why it's off: No Horizon function mapped for Biome.attributes (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Biome.attributes: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Biome.attributes -> (not mapped yet)
//   Biome.climateSettings -> (not mapped yet)
//   Biome.generationSettings -> (not mapped yet)
//   Biome.specialEffects -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/BiomeAccessor.java:33:
// | @Accessor("attributes")
// | @Mutable
// | void setAttributes(EnvironmentAttributeMap var1);
