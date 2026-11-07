// Not ported yet: the Java @Inject(INVOKE) on PictureInPictureRenderer.prepare (Mixin_GuiEntityRenderer.emf$initRender2).
// Why it's off: No Horizon function mapped for PictureInPictureRenderer.prepare (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for PictureInPictureRenderer.prepare: run `match`.
// Note: The call it hooks (FeatureRenderDispatcher.renderAllFeatures) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PictureInPictureRenderer.prepare -> (not mapped yet)
//   EntityRenderState.lightCoords -> (not mapped yet)
//   FeatureRenderDispatcher.renderAllFeatures -> (not mapped yet)
//   GuiEntityRenderState.renderState -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/submit/Mixin_GuiEntityRenderer.java:19:
// | @Inject(
// |    method = "prepare",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher;renderAllFeatures(Lcom/mojang/renderpearl/api/commands/RenderPass;Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;)V"
// |    )
// | )
// | private <T extends PictureInPictureRenderState> void emf$initRender2(CallbackInfo ci, @Local(argsOnly = true) T guiEntityRenderState) {
// |    if (guiEntityRenderState instanceof GuiEntityRenderState gui) {
// |       assertEmfState(gui);
// |    }
// | }
