// Not ported yet: the Java @Inject(TAIL) on ItemInHandLayer.submitArmWithItem (MixinHeldItemFeatureRenderer.emf$unsetHand).
// Why it's off: No Horizon function mapped for ItemInHandLayer.submitArmWithItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ItemInHandLayer.submitArmWithItem: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ArmedModel.translateToHand -> (not mapped yet)
//   ItemInHandLayer.submitArmWithItem -> (not mapped yet)
//   HumanoidArm.LEFT -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/feature/MixinHeldItemFeatureRenderer.java:51:
// | @Inject(method = "submitArmWithItem", at = @At("TAIL"))
// | private void emf$unsetHand(CallbackInfo ci) {
// |    EMFState.isInHand = false;
// |    EMFState.isInLeftHand = null;
// | }
