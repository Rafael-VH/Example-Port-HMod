// Not ported yet: the Java @ModifyExpressionValue(FIELD) on FlameFeatureRenderer.prepare (MixinFlameFeatureRenderer.width).
// Why it's off: No Horizon function mapped for FlameFeatureRenderer.prepare (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for FlameFeatureRenderer.prepare: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Pose.translate -> (not mapped yet)
//   FlameFeatureRenderer.prepare -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/submits/MixinFlameFeatureRenderer.java:16:
// | @ModifyExpressionValue(
// |    method = "prepare",
// |    at = @At(value = "FIELD", target = "Lnet/minecraft/client/renderer/entity/state/EntityRenderState;boundingBoxWidth:F", opcode = 180)
// | )
// | private float width(float original, @Local EntityRenderState vanilla, @Local Pose pose) {
// |    EMFEntityRenderState state = EMFEntityRenderState.from(vanilla);
// |    if (state == null) {
// |       return original;
// |    }
// |
// |    if (state.needsToModifyFire()) {
// |       pose.translate(
// |          Float.isNaN(state.fireX()) ? 0.0F : state.fireX(),
// |          Float.isNaN(state.fireY()) ? 0.0F : state.fireY(),
// |          Float.isNaN(state.fireZ()) ? 0.0F : state.fireZ()
// |       );
// |       if (!Float.isNaN(state.fireScale())) {
// |          return state.fireScale();
// |       }
// |    }
// |
// |    return original;
// | }
