// Not ported yet: the Java @Inject(HEAD) on NoiseBasedChunkGenerator.getBaseHeight (NoiseBasedChunkGeneratorHeightMixin.be_getBaseHeight).
// Why it's off: No Horizon function mapped for NoiseBasedChunkGenerator.getBaseHeight (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for NoiseBasedChunkGenerator.getBaseHeight: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.isBound -> (not mapped yet)
//   Holder.value -> (not mapped yet)
//   LevelHeightAccessor.getHeight -> (not mapped yet)
//   LevelHeightAccessor.getMinY -> (not mapped yet)
//   NoiseColumn.<init> -> (not mapped yet)
//   Block.defaultBlockState -> (not mapped yet)
//   Blocks.AIR -> (not mapped yet)
//   NoiseBasedChunkGenerator.getBaseColumn -> (not mapped yet)
//   NoiseBasedChunkGenerator.getBaseHeight -> (not mapped yet)
//   NoiseGeneratorSettings.defaultBlock -> (not mapped yet)
//   NoiseGeneratorSettings.noiseSettings -> (not mapped yet)
//   NoiseSettings.height -> (not mapped yet)
//   NoiseSettings.minY -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/NoiseBasedChunkGeneratorHeightMixin.java:29:
// | @Inject(method = "getBaseHeight", at = @At("HEAD"), cancellable = true)
// | private void be_getBaseHeight(int x, int z, Types type, LevelHeightAccessor level, RandomState random, CallbackInfoReturnable<Integer> cir) {
// |    if (this.be_isEndTarget()) {
// |       cir.setReturnValue(this.be_islandSurfaceY(x, z));
// |    }
// | }
