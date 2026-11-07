// Not ported yet: the Java @Inject(HEAD) on ChorusPlantBlock.canSurvive (ChorusPlantBlockMixin.be_canSurvive).
// Why it's off: No Horizon function mapped for ChorusPlantBlock.canSurvive (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ChorusPlantBlock.canSurvive: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.below -> (not mapped yet)
//   BlockPlaceContext.canPlace -> (not mapped yet)
//   BlockPlaceContext.getClickedPos -> (not mapped yet)
//   BlockPlaceContext.getLevel -> (not mapped yet)
//   BlockGetter.getBlockState -> (not mapped yet)
//   Level.getBlockState -> (not mapped yet)
//   LevelReader.getBlockState -> (not mapped yet)
//   Block.<init> -> content. A block's properties become its content-mod JSON (template, hardness, light, sounds).
//   Blocks.CHORUS_PLANT -> (not mapped yet)
//   Blocks.END_STONE -> (not mapped yet)
//   ChorusPlantBlock.canSurvive -> (not mapped yet)
//   ChorusPlantBlock.getStateForPlacement -> (not mapped yet)
//   ChorusPlantBlock.getStateWithConnections -> (not mapped yet)
//   ChorusPlantBlock.updateShape -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   BlockState.setValue -> (not mapped yet)
//   BlockStateProperties.DOWN -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/ChorusPlantBlockMixin.java:52:
// | @Inject(method = "canSurvive", at = @At("HEAD"), cancellable = true)
// | private void be_canSurvive(BlockState state, LevelReader world, BlockPos pos, CallbackInfoReturnable<Boolean> info) {
// |    BlockState down = world.getBlockState(pos.below());
// |    if (down.is(EndTerrainBlocks.CHORUS_NYLIUM.block()) || down.is(Blocks.END_STONE)) {
// |       info.setReturnValue(true);
// |    }
// | }
