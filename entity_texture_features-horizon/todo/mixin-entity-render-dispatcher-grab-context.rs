// Not ported yet: the Java @Inject(HEAD) on EntityRenderDispatcher.submit (MixinEntityRenderDispatcher.etf$grabContext).
// Why it's off: No Horizon function mapped for EntityRenderDispatcher.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EntityRenderDispatcher.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderDispatcher.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/misc/MixinEntityRenderDispatcher.java:18:
// | @Inject(method = "submit", at = @At("HEAD"))
// | private <S extends EntityRenderState> void etf$grabContext(CallbackInfo ci, @Local S state, @Share("state_etf") LocalRef<ETFEntityRenderState> stateRef) {
// |    ETFEntityRenderState etf = ((HoldsETFRenderState)state).etf$getState();
// |    if (etf != null) {
// |       ETFState.mount(etf);
// |    }
// |
// |    stateRef.set(etf);
// | }
