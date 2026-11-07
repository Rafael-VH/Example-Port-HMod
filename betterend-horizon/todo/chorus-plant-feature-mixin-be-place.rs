// Not ported yet: the Java @Inject(HEAD) on ChorusPlantFeature.place (ChorusPlantFeatureMixin.be_place).
// Why it's off: No Horizon function mapped for ChorusPlantFeature.place (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ChorusPlantFeature.place: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.below -> (not mapped yet)
//   WorldGenLevel.getBlockState -> (not mapped yet)
//   WorldGenLevel.isEmptyBlock -> (not mapped yet)
//   Blocks.CHORUS_PLANT -> (not mapped yet)
//   ChorusFlowerBlock.generatePlant -> (not mapped yet)
//   PipeBlock.DOWN -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   BlockState.setValue -> (not mapped yet)
//   ChorusPlantFeature.place -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/ChorusPlantFeatureMixin.java:22:
// | @Inject(method = "place", at = @At("HEAD"), cancellable = true)
// | private void be_place(
// |    WorldGenLevel structureWorldAccess, ChunkGenerator generator, RandomSource random, BlockPos blockPos, CallbackInfoReturnable<Boolean> info
// | ) {
// |    if (structureWorldAccess.isEmptyBlock(blockPos) && structureWorldAccess.getBlockState(blockPos.below()).is(EndTerrainBlocks.CHORUS_NYLIUM.block())) {
// |       ChorusFlowerBlock.generatePlant(structureWorldAccess, blockPos, random, MHelper.randRange(8, 16, random));
// |       BlockState bottom = structureWorldAccess.getBlockState(blockPos);
// |       if (bottom.is(Blocks.CHORUS_PLANT)) {
// |          BlocksHelper.setWithoutUpdate(structureWorldAccess, blockPos, (BlockState)bottom.setValue(PipeBlock.DOWN, true));
// |       }
// |
// |       info.setReturnValue(true);
// |    }
// | }
