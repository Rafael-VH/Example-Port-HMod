// Not ported yet: the Java @WrapWithCondition(INVOKE) on CustomHeadLayer.submit (Mixin_Head_BlockOffset.offsetBlock).
// Why it's off: No Horizon function mapped for CustomHeadLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: Call `original` only when the condition holds.
// Note: No Horizon function known for CustomHeadLayer.submit: run `match`.
// Note: The call it hooks (ModelPart.translateAndRotate) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HeadedModel.translateToHead -> (not mapped yet)
//   ModelPart.translateAndRotate -> (not mapped yet)
//   CustomHeadLayer.submit -> (not mapped yet)
//   RenderLayer.<init> -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/attachments/Mixin_Head_BlockOffset.java:24:
// | @WrapWithCondition(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;FF)V",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/model/geom/ModelPart;translateAndRotate(Lcom/mojang/blaze3d/vertex/PoseStack;)V", ordinal = 0)
// | )
// | private boolean offsetBlock(ModelPart instance, PoseStack poseStack, @Share("needsCancel") LocalBooleanRef needsCancel) {
// |    IEMFModel model = (IEMFModel)this.getParentModel();
// |    if (model.emf$isEMFModel()) {
// |       EMFModelPartRoot root = model.emf$getEMFRootModel();
// |       Consumer<PoseStack> positioner = root.getPositionerForAttachment(EMFAttachment.Type.HEAD);
// |       if (positioner != null) {
// |          positioner.accept(poseStack);
// |          needsCancel.set(true);
// |          return false;
// |       }
// |    }
// |
// |    return true;
// | }
