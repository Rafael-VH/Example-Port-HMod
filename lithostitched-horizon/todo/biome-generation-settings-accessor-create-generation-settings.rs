// Not ported yet: the Java @Invoker on BiomeGenerationSettings.<init> (BiomeGenerationSettingsAccessor.createGenerationSettings).
// Why it's off: No Horizon function mapped for BiomeGenerationSettings.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for BiomeGenerationSettings.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BiomeGenerationSettings.<init> -> (not mapped yet)
//   BiomeGenerationSettings.carvers -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/BiomeGenerationSettingsAccessor.java:15:
// | @Invoker("<init>")
// | static BiomeGenerationSettings createGenerationSettings(HolderSet<WorldCarver> carvers, List<HolderSet<PlacedFeature>> features) {
// |    throw new AssertionError();
// | }
