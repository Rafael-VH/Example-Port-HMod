// Not ported yet: the Java @Inject(RETURN) on MushroomBlock.canSurvive (MushroomMixin.canStay).
// Why it's off: No Horizon function mapped for MushroomBlock.canSurvive (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for MushroomBlock.canSurvive: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.below -> (not mapped yet)
//   ServerLevel.getBlockState -> (not mapped yet)
//   LevelReader.getBlockState -> (not mapped yet)
//   Blocks.BROWN_MUSHROOM -> (not mapped yet)
//   Blocks.RED_MUSHROOM -> (not mapped yet)
//   MushroomBlock.canSurvive -> (not mapped yet)
//   MushroomBlock.performBonemeal -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/MushroomMixin.java:23:
// | @Inject(method = "canSurvive", at = @At(value = "RETURN", ordinal = 1), locals = LocalCapture.CAPTURE_FAILHARD, cancellable = true)
// | private void canStay(BlockState state, LevelReader world, BlockPos pos, CallbackInfoReturnable<Boolean> info) {
// |    if (BlocksHelper.isNetherMycelium(world.getBlockState(pos.below()))) {
// |       info.setReturnValue(true);
// |    }
// | }
