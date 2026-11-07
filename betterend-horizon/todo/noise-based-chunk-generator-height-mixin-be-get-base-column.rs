// Not ported yet: the Java @Inject(HEAD) on NoiseBasedChunkGenerator.getBaseColumn (NoiseBasedChunkGeneratorHeightMixin.be_getBaseColumn).
// Why it's off: No Horizon function mapped for NoiseBasedChunkGenerator.getBaseColumn (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for NoiseBasedChunkGenerator.getBaseColumn: run `match`.
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
// Decompiled Java, org/betterx/betterend/mixin/common/NoiseBasedChunkGeneratorHeightMixin.java:36:
// | @Inject(method = "getBaseColumn", at = @At("HEAD"), cancellable = true)
// | private void be_getBaseColumn(int x, int z, LevelHeightAccessor level, RandomState random, CallbackInfoReturnable<NoiseColumn> cir) {
// |    if (this.be_isEndTarget()) {
// |       int surfaceY = this.be_islandSurfaceY(x, z);
// |       int minY = level.getMinY();
// |       int height = level.getHeight();
// |       BlockState solid = ((NoiseGeneratorSettings)this.settings.value()).defaultBlock();
// |       BlockState air = Blocks.AIR.defaultBlockState();
// |       BlockState[] column = new BlockState[height];
// |
// |       for (int i = 0; i < height; i++) {
// |          column[i] = minY + i <= surfaceY ? solid : air;
// |       }
// |
// |       cir.setReturnValue(new NoiseColumn(minY, column));
// |    }
// | }
