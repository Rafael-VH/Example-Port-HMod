// Not ported yet: the Java @Inject(TAIL) on HumanoidArmorLayer.submit (Mixin_HumanoidArmorLayer_AddBaseModelPoseRef.emf$endRender).
// Why it's off: No Horizon function mapped for HumanoidArmorLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for HumanoidArmorLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HumanoidArmorLayer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/submits/Mixin_HumanoidArmorLayer_AddBaseModelPoseRef.java:29:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/HumanoidRenderState;FF)V",
// |    at = @At("TAIL")
// | )
// | private void emf$endRender(CallbackInfo ci, @Local(argsOnly = true) S humanoidRenderState) {
// |    EMFEntityRenderState state = EMFEntityRenderState.from(humanoidRenderState);
// |    if (state != null) {
// |       EMFBipedPose pose = state.getBipedPose();
// |       if (pose != null) {
// |          pose.applyToSubmit = false;
// |       }
// |    }
// | }
