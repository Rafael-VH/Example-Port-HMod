// Not ported yet: the Java @Inject(RETURN) on AvatarRenderer.renderHand (MixinPlayerEntityRenderer.emf$unsetHand).
// Why it's off: No Horizon function mapped for AvatarRenderer.renderHand (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for AvatarRenderer.renderHand: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SubmitNodeCollector.submitModelPart -> (not mapped yet)
//   LivingEntityRenderer.<init> -> (not mapped yet)
//   AvatarRenderer.renderHand -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinPlayerEntityRenderer.java:48:
// | @Inject(method = "renderHand", at = @At("RETURN"))
// | private void emf$unsetHand(CallbackInfo ci) {
// |    EMFEntityRenderState state = EMFState.state();
// |    if (state != null && state.isManualPlayerState()) {
// |       ETFState.unMount();
// |    }
// | }
