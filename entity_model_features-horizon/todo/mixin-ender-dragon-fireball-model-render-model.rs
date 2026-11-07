// Not ported yet: the Java @Inject(HEAD) on DragonFireballRenderer.submit (Mixin_EnderDragonFireball_Model.emf$renderModel).
// Why it's off: No Horizon function mapped for DragonFireballRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for DragonFireballRenderer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelLayerLocation.<init> -> (not mapped yet)
//   ModelPart.<init> -> (not mapped yet)
//   SubmitNodeCollector.submitModel -> (not mapped yet)
//   DragonFireballRenderer.<init> -> (not mapped yet)
//   DragonFireballRenderer.submit -> (not mapped yet)
//   EntityRenderState.lightCoords -> (not mapped yet)
//   OverlayTexture.NO_OVERLAY -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/model/Mixin_EnderDragonFireball_Model.java:50:
// | @Inject(method = "submit", at = @At("HEAD"), cancellable = true)
// | private void emf$renderModel(
// |    CallbackInfo ci,
// |    @Local(argsOnly = true) PoseStack poseStack,
// |    @Local(argsOnly = true) EntityRenderState entityRenderState,
// |    @Local(argsOnly = true) SubmitNodeCollector submitNodeCollector
// | ) {
// |    if (this.fireball != null) {
// |       submitNodeCollector.submitModel(this.fireball, entityRenderState, poseStack, RENDER_TYPE, entityRenderState.lightCoords, OverlayTexture.NO_OVERLAY, 0);
// |       ci.cancel();
// |    }
// | }
