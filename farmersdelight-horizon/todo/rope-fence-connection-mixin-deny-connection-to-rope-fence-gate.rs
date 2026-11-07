// Not ported yet: the Java @Inject(HEAD) on FenceBlock.connectsTo (RopeFenceConnectionMixin.denyConnectionToRopeFenceGate).
// Why it's off: No Horizon function mapped for FenceBlock.connectsTo (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for FenceBlock.connectsTo: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FenceBlock.connectsTo -> (not mapped yet)
//   FenceBlock.isSameFence -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/RopeFenceConnectionMixin.java:14:
// | @Inject(at = @At("HEAD"), method = "connectsTo", cancellable = true)
// | public void denyConnectionToRopeFenceGate(BlockState state, boolean isSideSolid, Direction direction, CallbackInfoReturnable<Boolean> cir) {
// |    if (state.is(ModBlocks.ROPE_FENCE_GATE.get())) {
// |       cir.setReturnValue(false);
// |    }
// | }
