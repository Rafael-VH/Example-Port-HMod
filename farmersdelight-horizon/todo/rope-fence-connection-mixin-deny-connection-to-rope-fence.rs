// Not ported yet: the Java @Inject(HEAD) on FenceBlock.isSameFence (RopeFenceConnectionMixin.denyConnectionToRopeFence).
// Why it's off: No Horizon function mapped for FenceBlock.isSameFence (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for FenceBlock.isSameFence: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FenceBlock.connectsTo -> (not mapped yet)
//   FenceBlock.isSameFence -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/RopeFenceConnectionMixin.java:21:
// | @Inject(at = @At("HEAD"), method = "isSameFence", cancellable = true)
// | public void denyConnectionToRopeFence(BlockState state, CallbackInfoReturnable<Boolean> cir) {
// |    if (state.is(ModBlocks.ROPE_FENCE.get())) {
// |       cir.setReturnValue(false);
// |    }
// | }
