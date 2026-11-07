// Not ported yet: the Java @Inject(TAIL) on PictureInPictureRenderer.prepare (Mixin_GuiEntityRenderer.emf$endRender).
// Why it's off: No Horizon function mapped for PictureInPictureRenderer.prepare (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for PictureInPictureRenderer.prepare: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PictureInPictureRenderer.prepare -> (not mapped yet)
//   EntityRenderState.lightCoords -> (not mapped yet)
//   FeatureRenderDispatcher.renderAllFeatures -> (not mapped yet)
//   GuiEntityRenderState.renderState -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/submit/Mixin_GuiEntityRenderer.java:32:
// | @Inject(method = "prepare", at = @At("TAIL"))
// | private <T extends PictureInPictureRenderState> void emf$endRender(CallbackInfo ci, @Local(argsOnly = true) T guiEntityRenderState) {
// |    if (guiEntityRenderState instanceof GuiEntityRenderState gui) {
// |       end(gui);
// |    }
// | }
