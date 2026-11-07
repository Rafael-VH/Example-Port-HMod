// Not ported yet: the Java @Inject(RETURN) on ChorusPlantBlock.updateShape (ChorusPlantBlockMixin.be_updateShape).
// Why it's off: No Horizon function mapped for ChorusPlantBlock.updateShape (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for ChorusPlantBlock.updateShape: run `match`.
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
// Decompiled Java, org/betterx/betterend/mixin/common/ChorusPlantBlockMixin.java:60:
// | @Inject(method = "updateShape", at = @At("RETURN"), cancellable = true)
// | private void be_updateShape(
// |    BlockState state,
// |    LevelReader world,
// |    ScheduledTickAccess scheduledTickAccess,
// |    BlockPos pos,
// |    Direction direction,
// |    BlockPos posFrom,
// |    BlockState newState,
// |    RandomSource randomSource,
// |    CallbackInfoReturnable<BlockState> info
// | ) {
// |    BlockState plant = (BlockState)info.getReturnValue();
// |    if (plant.is(Blocks.CHORUS_PLANT) && world.getBlockState(pos.below()).is(CommonBlockTags.END_STONES)) {
// |       plant = (BlockState)plant.setValue(BlockStateProperties.DOWN, true);
// |       info.setReturnValue(plant);
// |    }
// | }
