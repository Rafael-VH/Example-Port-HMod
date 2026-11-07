// Not ported yet: the Java @ModifyExpressionValue(FIELD) on FlameFeatureRenderer.prepare (MixinFlameFeatureRenderer.height).
// Why it's off: No Horizon function mapped for FlameFeatureRenderer.prepare (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for FlameFeatureRenderer.prepare: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Pose.translate -> (not mapped yet)
//   FlameFeatureRenderer.prepare -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/submits/MixinFlameFeatureRenderer.java:40:
// | @ModifyExpressionValue(
// |    method = "prepare",
// |    at = @At(value = "FIELD", target = "Lnet/minecraft/client/renderer/entity/state/EntityRenderState;boundingBoxHeight:F", opcode = 180)
// | )
// | private float height(float original, @Local EntityRenderState vanilla) {
// |    EMFEntityRenderState state = EMFEntityRenderState.from(vanilla);
// |    return state != null && !Float.isNaN(state.fireHeight()) ? state.fireHeight() : original;
// | }
