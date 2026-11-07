// Not ported yet: the Java @Inject(INVOKE) on ItemInHandLayer.submitArmWithItem (MixinHeldItemFeatureRenderer.emf$transform2).
// Why it's off: No Horizon function mapped for ItemInHandLayer.submitArmWithItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for ItemInHandLayer.submitArmWithItem: run `match`.
// Note: The call it hooks (ArmedModel.translateToHand) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ArmedModel.translateToHand -> (not mapped yet)
//   ItemInHandLayer.submitArmWithItem -> (not mapped yet)
//   HumanoidArm.LEFT -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/feature/MixinHeldItemFeatureRenderer.java:39:
// | @Inject(
// |    method = "submitArmWithItem",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/model/ArmedModel;translateToHand(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lnet/minecraft/world/entity/HumanoidArm;Lcom/mojang/blaze3d/vertex/PoseStack;)V",
// |       shift = Shift.AFTER
// |    )
// | )
// | private void emf$transform2(CallbackInfo ci) {
// |    EMFState.isInHandItemLayerTransform = false;
// | }
