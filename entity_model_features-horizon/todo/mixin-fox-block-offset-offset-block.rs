// Not ported yet: the Java @WrapWithCondition(INVOKE) on FoxHeldItemLayer.submit (Mixin_Fox_BlockOffset.offsetBlock).
// Why it's off: No Horizon function mapped for FoxHeldItemLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: Call `original` only when the condition holds.
// Note: No Horizon function known for FoxHeldItemLayer.submit: run `match`.
// Note: The call it hooks (PoseStack.translate) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.rotate -> (not mapped yet)
//   PoseStack.rotateDegrees -> horizon_client_render::camera::rotate_degrees (suggested, high)
//   PoseStack.translate -> (not mapped yet)
//   FoxHeldItemLayer.submit -> (not mapped yet)
//   RenderLayer.<init> -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/attachments/Mixin_Fox_BlockOffset.java:24:
// | @WrapWithCondition(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/FoxRenderState;FF)V",
// |    at = @At(value = "INVOKE", target = "Lcom/mojang/blaze3d/vertex/PoseStack;translate(FFF)V", ordinal = 0)
// | )
// | private boolean offsetBlock(
// |    PoseStack instance, float f, float g, float h, @Local(argsOnly = true) PoseStack poseStack, @Share("cancelRestOfHead") LocalBooleanRef cancelRestOfHead
// | ) {
// |    IEMFModel model = (IEMFModel)this.getParentModel();
// |    if (model.emf$isEMFModel()) {
// |       EMFModelPartRoot root = model.emf$getEMFRootModel();
// |       Consumer<PoseStack> positioner = root.getPositionerForAttachment(EMFAttachment.Type.FOX);
// |       if (positioner != null) {
// |          positioner.accept(poseStack);
// |          cancelRestOfHead.set(true);
// |          return false;
// |       }
// |    }
// |
// |    return true;
// | }
