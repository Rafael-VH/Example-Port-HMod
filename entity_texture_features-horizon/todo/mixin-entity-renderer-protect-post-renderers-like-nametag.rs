// Not ported yet: the Java @Inject(HEAD) on EntityRenderer.submit (MixinEntityRenderer.etf$protectPostRenderersLikeNametag).
// Why it's off: No Horizon function mapped for EntityRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EntityRenderer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderer.extractRenderState -> (not mapped yet)
//   EntityRenderer.getPackedLightCoords -> (not mapped yet)
//   EntityRenderer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/MixinEntityRenderer.java:31:
// | @Inject(method = "submit", at = @At("HEAD"))
// | private void etf$protectPostRenderersLikeNametag(CallbackInfo ci) {
// |    ETFState.pushRenderLayerModifyState(false);
// | }
