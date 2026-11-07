// Not ported yet: the Java @Inject(INVOKE) on PandaHoldsItemLayer.submit (Mixin_Panda_BlockOffset.offsetBlock).
// Why it's off: No Horizon function mapped for PandaHoldsItemLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for PandaHoldsItemLayer.submit: run `match`.
// Note: The call it hooks (PoseStack.translate) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.translate -> (not mapped yet)
//   PandaHoldsItemLayer.submit -> (not mapped yet)
//   RenderLayer.<init> -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/attachments/Mixin_Panda_BlockOffset.java:22:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/PandaRenderState;FF)V",
// |    at = @At(value = "INVOKE", target = "Lcom/mojang/blaze3d/vertex/PoseStack;translate(FFF)V")
// | )
// | private void offsetBlock(CallbackInfo ci, @Local(argsOnly = true) PoseStack poseStack) {
// |    IEMFModel model = (IEMFModel)this.getParentModel();
// |    if (model.emf$isEMFModel()) {
// |       EMFModelPartRoot root = model.emf$getEMFRootModel();
// |       Consumer<PoseStack> positioner = root.getPositionerForAttachment(EMFAttachment.Type.DOLPHIN);
// |       if (positioner != null) {
// |          positioner.accept(poseStack);
// |       }
// |    }
// | }
