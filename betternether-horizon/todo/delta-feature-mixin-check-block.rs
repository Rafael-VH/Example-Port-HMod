// Not ported yet: the Java @Inject(HEAD) on DeltaFeature.isClear (DeltaFeatureMixin.checkBlock).
// Why it's off: No Horizon function mapped for DeltaFeature.isClear (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for DeltaFeature.isClear: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelAccessor.getBlockState -> (not mapped yet)
//   BlockState.canBeReplaced -> horizon_world::level::place::Sim<'_, L>::can_be_replaced (suggested, high)
//   BlockState.getBlock -> (not mapped yet)
//   BlockState.getFluidState -> (not mapped yet)
//   DeltaFeature.isClear -> (not mapped yet)
//   FluidState.isEmpty -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/DeltaFeatureMixin.java:17:
// | @Inject(method = "isClear", at = @At("HEAD"), cancellable = true)
// | private void checkBlock(LevelAccessor worldAccess, BlockPos blockPos, CallbackInfoReturnable<Boolean> info) {
// |    BlockState blockState = worldAccess.getBlockState(blockPos);
// |    if (!bn_isValidBlock(blockState)) {
// |       info.setReturnValue(false);
// |       info.cancel();
// |    }
// | }
