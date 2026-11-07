// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on NoiseBasedChunkGenerator.doFill (NoiseBasedChunkGeneratorTerrainMixin.be_islandTerrain).
// Why it's off: No Horizon function mapped for NoiseBasedChunkGenerator.doFill (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for NoiseBasedChunkGenerator.doFill: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.value -> (not mapped yet)
//   NoiseBasedChunkGenerator.doFill -> (not mapped yet)
//   NoiseBasedChunkGenerator.iterateNoiseColumn -> (not mapped yet)
//   NoiseGeneratorSettings.noiseSettings -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/NoiseBasedChunkGeneratorTerrainMixin.java:21:
// | @ModifyExpressionValue(
// |    method = {"doFill", "iterateNoiseColumn"},
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/levelgen/NoiseRouter;finalDensity()Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;"
// |    )
// | )
// | private DensityFunction be_islandTerrain(DensityFunction finalDensity) {
// |    NoiseGeneratorSettings generatorSettings = (NoiseGeneratorSettings)this.settings.value();
// |    return !BETargetChecker.class.cast(generatorSettings).be_isTarget()
// |       ? finalDensity
// |       : TerrainGenerator.withIslandTerrain(finalDensity, generatorSettings.noiseSettings());
// | }
