// Not ported yet: the Java @Inject(HEAD) on BlockEntityRenderState.extractBase (MixinBlockEntityRenderState.etf$createRenderState).
// Why it's off: No Horizon function mapped for BlockEntityRenderState.extractBase (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for BlockEntityRenderState.extractBase: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockEntityRenderState.extractBase -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/misc/MixinBlockEntityRenderState.java:37:
// | @Inject(method = "extractBase", at = @At("HEAD"))
// | private static void etf$createRenderState(
// |    BlockEntity blockEntity, BlockEntityRenderState blockEntityRenderState, CrumblingOverlay crumblingOverlay, CallbackInfo ci
// | ) {
// |    HoldsETFRenderState holder = (HoldsETFRenderState)blockEntityRenderState;
// |    holder.etf$initState((ETFEntity)blockEntity);
// | }
