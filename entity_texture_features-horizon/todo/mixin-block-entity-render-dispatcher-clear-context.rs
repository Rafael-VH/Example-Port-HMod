// Not ported yet: the Java @Inject(RETURN) on BlockEntityRenderDispatcher.submit (MixinBlockEntityRenderDispatcher.etf$clearContext).
// Why it's off: No Horizon function mapped for BlockEntityRenderDispatcher.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for BlockEntityRenderDispatcher.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockEntityRenderDispatcher.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/misc/MixinBlockEntityRenderDispatcher.java:32:
// | @Inject(method = "submit", at = @At("RETURN"))
// | private static void etf$clearContext(CallbackInfo ci, @Share("state_etf") LocalRef<ETFEntityRenderState> stateRef) {
// |    if (stateRef.get() != null) {
// |       ETFState.stackVerify((ETFEntityRenderState)stateRef.get());
// |       ETFState.unMount();
// |    }
// | }
