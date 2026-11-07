// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on EntityRenderer.extractShadow (MixinEntityRenderer.strength).
// Why it's off: No Horizon function mapped for EntityRenderer.extractShadow (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for EntityRenderer.extractShadow: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderer.extractShadow -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinEntityRenderer.java:24:
// | @ModifyExpressionValue(
// |    method = "extractShadow",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/entity/EntityRenderer;getShadowStrength(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;)F"
// |    )
// | )
// | private float strength(float original) {
// |    EMFEntityRenderState state = EMFState.state();
// |    return state != null && !Float.isNaN(state.shadowOpacity()) ? state.shadowOpacity() : original;
// | }
