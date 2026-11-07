// Not ported yet: the Java @Inject(TAIL) on EntityRenderer.submit (MixinEntityRenderer.etf$revertForRenderersThatCallSuperFirst).
// Why it's off: No Horizon function mapped for EntityRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for EntityRenderer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderer.extractRenderState -> (not mapped yet)
//   EntityRenderer.getPackedLightCoords -> (not mapped yet)
//   EntityRenderer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/MixinEntityRenderer.java:36:
// | @Inject(method = "submit", at = @At("TAIL"))
// | private void etf$revertForRenderersThatCallSuperFirst(CallbackInfo ci) {
// |    ETFState.popRenderLayerModifyState();
// | }
