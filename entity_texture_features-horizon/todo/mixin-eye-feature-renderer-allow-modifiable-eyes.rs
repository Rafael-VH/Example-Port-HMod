// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on EyesLayer.submit (MixinEyeFeatureRenderer.etf$allowModifiableEyes).
// Why it's off: No Horizon function mapped for EyesLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for EyesLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EyesLayer.submit -> (not mapped yet)
//   RenderTypes.eyes -> (not mapped yet)
//   Identifier.equals -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/feature/MixinEyeFeatureRenderer.java:17:
// | @ModifyExpressionValue(
// |    method = "submit",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/entity/layers/EyesLayer;renderType()Lnet/minecraft/client/renderer/rendertype/RenderType;"
// |    )
// | )
// | private RenderType etf$allowModifiableEyes(RenderType layer) {
// |    if (layer instanceof ETFRenderLayerWithTexture etf && etf.etf$getId().isPresent()) {
// |       Identifier id = etf.etf$getId().get();
// |       Identifier variant = ETFUtils2.getETFVariantNotNullForInjector(id);
// |       if (!id.equals(variant)) {
// |          ETFState.pushRenderLayerModifyState(false);
// |          RenderType layer2 = RenderTypes.eyes(variant);
// |          ETFState.popRenderLayerModifyState();
// |          return layer2;
// |       }
// |    }
// |
// |    return layer;
// | }
