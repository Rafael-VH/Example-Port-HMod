// Not ported yet: the Java @Inject(INVOKE) on CarriedBlockLayer.submit (Mixin_Enderman_BlockOffset.offsetBlock).
// Why it's off: No Horizon function mapped for CarriedBlockLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for CarriedBlockLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.rotateDegrees -> horizon_client_render::camera::rotate_degrees (suggested, high)
//   CarriedBlockLayer.submit -> (not mapped yet)
//   RenderLayer.<init> -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/attachments/Mixin_Enderman_BlockOffset.java:22:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/EndermanRenderState;FF)V",
// |    at = @At(value = "INVOKE", target = "Lcom/mojang/blaze3d/vertex/PoseStack;rotateDegrees(Lcom/mojang/math/Axis;F)V", ordinal = 0)
// | )
// | private void offsetBlock(CallbackInfo ci, @Local(argsOnly = true) PoseStack poseStack) {
// |    IEMFModel model = (IEMFModel)this.getParentModel();
// |    if (model.emf$isEMFModel()) {
// |       EMFModelPartRoot root = model.emf$getEMFRootModel();
// |       Consumer<PoseStack> positioner = root.getPositionerForAttachment(EMFAttachment.Type.ENDERMAN);
// |       if (positioner != null) {
// |          positioner.accept(poseStack);
// |       }
// |    }
// | }
