// Not ported yet: the Java @Inject(RETURN) on NetherWartBlock.mayPlaceOn (NetherWartMixin.canStay).
// Why it's off: No Horizon function mapped for NetherWartBlock.mayPlaceOn (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for NetherWartBlock.mayPlaceOn: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.below -> (not mapped yet)
//   ServerLevel.getBlockState -> (not mapped yet)
//   ServerLevel.setBlock -> horizon_world::level::Behaviours::set_block (suggested, high)
//   RandomSource.nextInt -> (not mapped yet)
//   NetherWartBlock.AGE -> (not mapped yet)
//   NetherWartBlock.mayPlaceOn -> (not mapped yet)
//   NetherWartBlock.randomTick -> (not mapped yet)
//   VegetationBlock.<init> -> (not mapped yet)
//   VegetationBlock.randomTick -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//   BlockState.getValue -> (not mapped yet)
//   BlockState.setValue -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/NetherWartMixin.java:26:
// | @Inject(method = "mayPlaceOn", at = @At("RETURN"), locals = LocalCapture.CAPTURE_FAILHARD, cancellable = true)
// | private void canStay(BlockState floor, BlockGetter view, BlockPos pos, CallbackInfoReturnable<Boolean> info) {
// |    if (floor.getBlock() == NetherTerrainBlocks.FARMLAND.block()) {
// |       info.setReturnValue(true);
// |    }
// | }
