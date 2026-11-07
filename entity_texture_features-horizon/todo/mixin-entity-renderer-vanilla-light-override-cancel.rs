// Not ported yet: the Java @ModifyReturnValue(RETURN) on EntityRenderer.getPackedLightCoords (MixinEntityRenderer.etf$vanillaLightOverrideCancel).
// Why it's off: No Horizon function mapped for EntityRenderer.getPackedLightCoords (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for EntityRenderer.getPackedLightCoords: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderer.extractRenderState -> (not mapped yet)
//   EntityRenderer.getPackedLightCoords -> (not mapped yet)
//   EntityRenderer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/MixinEntityRenderer.java:26:
// | @ModifyReturnValue(method = "getPackedLightCoords", at = @At("RETURN"))
// | private int etf$vanillaLightOverrideCancel(int light, @Local(argsOnly = true) T entity, @Local(argsOnly = true) float tickDelta) {
// |    return ETF.config().getConfig().getLightOverride(entity, tickDelta, light);
// | }
