// Not ported yet: the Java @Inject(HEAD) on ItemInHandLayer.submitArmWithItem (MixinHeldItemFeatureRenderer.emf$setHand).
// Why it's off: No Horizon function mapped for ItemInHandLayer.submitArmWithItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ItemInHandLayer.submitArmWithItem: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ArmedModel.translateToHand -> (not mapped yet)
//   ItemInHandLayer.submitArmWithItem -> (not mapped yet)
//   HumanoidArm.LEFT -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/feature/MixinHeldItemFeatureRenderer.java:21:
// | @Inject(method = "submitArmWithItem", at = @At("HEAD"))
// | private void emf$setHand(CallbackInfo ci, @Local HumanoidArm arm) {
// |    EMFState.isInHand = true;
// |    EMFState.isInLeftHand = arm == HumanoidArm.LEFT;
// | }
