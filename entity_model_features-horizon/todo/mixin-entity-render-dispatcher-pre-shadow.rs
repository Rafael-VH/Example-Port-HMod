// Not ported yet: the Java @Inject(INVOKE) on EntityRenderDispatcher.submit (MixinEntityRenderDispatcher.preShadow).
// Why it's off: No Horizon function mapped for EntityRenderDispatcher.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for EntityRenderDispatcher.submit: run `match`.
// Note: The call it hooks (SubmitNodeCollector.submitShadow) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.translate -> (not mapped yet)
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   SubmitNodeCollector.submitShadow -> (not mapped yet)
//   EntityRenderDispatcher.submit -> (not mapped yet)
//   EntityRenderer.extractShadow -> (not mapped yet)
//   EntityRenderer.submit -> (not mapped yet)
//   EntityRenderState.x -> (not mapped yet)
//   EntityRenderState.z -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinEntityRenderDispatcher.java:68:
// | @Inject(
// |    method = "submit",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/SubmitNodeCollector;submitShadow(Lcom/mojang/blaze3d/vertex/PoseStack;FLjava/util/List;)V"
// |    )
// | )
// | private <S extends EntityRenderState> void preShadow(CallbackInfo ci, @Local PoseStack poseStack, @Local(argsOnly = true) S ogState) {
// |    EMFEntityRenderState state = EMFEntityRenderState.from(ogState);
// |    if (state != null && (!Float.isNaN(state.shadowX()) || !Float.isNaN(state.shadowZ()))) {
// |       poseStack.translate(Float.isNaN(state.shadowX()) ? 0.0F : state.shadowX(), 0.0F, Float.isNaN(state.shadowZ()) ? 0.0F : state.shadowZ());
// |    }
// | }
