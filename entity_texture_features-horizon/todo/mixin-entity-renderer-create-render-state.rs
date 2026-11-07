// Not ported yet: the Java @Inject(TAIL) on EntityRenderer.extractRenderState (MixinEntityRenderer.etf$createRenderState).
// Why it's off: No Horizon function mapped for EntityRenderer.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for EntityRenderer.extractRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderer.extractRenderState -> (not mapped yet)
//   EntityRenderer.getPackedLightCoords -> (not mapped yet)
//   EntityRenderer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/MixinEntityRenderer.java:21:
// | @Inject(method = "extractRenderState", at = @At("TAIL"))
// | private void etf$createRenderState(CallbackInfo ci, @Local(argsOnly = true) Entity entity, @Local(argsOnly = true) S state) {
// |    ((HoldsETFRenderState)state).etf$initState((ETFEntity)entity);
// | }
