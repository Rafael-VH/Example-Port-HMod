// Not ported yet: the Java @Inject(RETURN) on ChorusPlantBlock.getStateWithConnections (ChorusPlantBlockMixin.be_getStateForPlacement).
// Why it's off: No Horizon function mapped for ChorusPlantBlock.getStateWithConnections (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for ChorusPlantBlock.getStateWithConnections: run `match`.
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
// Decompiled Java, org/betterx/betterend/mixin/common/ChorusPlantBlockMixin.java:30:
// | @Inject(
// |    method = "getStateForPlacement(Lnet/minecraft/world/item/context/BlockPlaceContext;)Lnet/minecraft/world/level/block/state/BlockState;",
// |    at = @At("RETURN"),
// |    cancellable = true
// | )
// | private void be_getStateForPlacement(BlockPlaceContext ctx, CallbackInfoReturnable<BlockState> info) {
// |    BlockPos pos = ctx.getClickedPos();
// |    Level world = ctx.getLevel();
// |    BlockState plant = (BlockState)info.getReturnValue();
// |    if (ctx.canPlace() && plant.is(Blocks.CHORUS_PLANT) && world.getBlockState(pos.below()).is(CommonBlockTags.END_STONES)) {
// |       info.setReturnValue((BlockState)plant.setValue(BlockStateProperties.DOWN, true));
// |    }
// | }
