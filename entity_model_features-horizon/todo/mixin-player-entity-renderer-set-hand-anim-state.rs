// Not ported yet: the Java @Inject(HEAD) on AvatarRenderer.renderHand (MixinPlayerEntityRenderer.emf$setHandAnimState).
// Why it's off: No Horizon function mapped for AvatarRenderer.renderHand (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for AvatarRenderer.renderHand: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SubmitNodeCollector.submitModelPart -> (not mapped yet)
//   LivingEntityRenderer.<init> -> (not mapped yet)
//   AvatarRenderer.renderHand -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinPlayerEntityRenderer.java:27:
// | @Inject(method = "renderHand", at = @At("HEAD"))
// | private void emf$setHandAnimState(CallbackInfo ci) {
// |    EMFEntityRenderState state = EMFEntityRenderState.manualPlayerState();
// |    state.setIsFirstPersonHand(true);
// |    ETFState.mount(state);
// | }
