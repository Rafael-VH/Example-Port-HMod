// Not ported yet: the Java @Inject(HEAD) on BlockEntityRenderDispatcher.submit (MixinBlockEntityRenderDispatcher.etf$grabContext).
// Why it's off: No Horizon function mapped for BlockEntityRenderDispatcher.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for BlockEntityRenderDispatcher.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockEntityRenderDispatcher.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/misc/MixinBlockEntityRenderDispatcher.java:20:
// | @Inject(method = "submit", at = @At("HEAD"))
// | private static <S extends BlockEntityRenderState> void etf$grabContext(
// |    CallbackInfo ci, @Local(argsOnly = true) S state, @Share("state_etf") LocalRef<ETFEntityRenderState> stateRef
// | ) {
// |    ETFEntityRenderState etf = ((HoldsETFRenderState)state).etf$getState();
// |    if (etf != null) {
// |       ETFState.mount(etf);
// |    }
// |
// |    stateRef.set(etf);
// | }
