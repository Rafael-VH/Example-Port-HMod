// Not ported yet: the Java @WrapWithCondition(INVOKE) on WitchItemLayer.applyTranslation (Mixin_Witch_BlockOffset.offsetBlock).
// Why it's off: No Horizon function mapped for WitchItemLayer.applyTranslation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: Call `original` only when the condition holds.
// Note: No Horizon function known for WitchItemLayer.applyTranslation: run `match`.
// Note: The call it hooks (ModelPart.translateAndRotate) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelPart.translateAndRotate -> (not mapped yet)
//   WitchModel.translateToHead -> (not mapped yet)
//   CrossedArmsItemLayer.<init> -> (not mapped yet)
//   WitchItemLayer.applyTranslation -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/attachments/Mixin_Witch_BlockOffset.java:27:
// | @WrapWithCondition(
// |    method = "applyTranslation(Lnet/minecraft/client/renderer/entity/state/WitchRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;)V",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/model/geom/ModelPart;translateAndRotate(Lcom/mojang/blaze3d/vertex/PoseStack;)V", ordinal = 0)
// | )
// | private boolean offsetBlock(
// |    ModelPart instance, PoseStack arg, @Local(argsOnly = true) PoseStack poseStack, @Share("needsCancel") LocalBooleanRef needsCancel
// | ) {
// |    IEMFModel model = (IEMFModel)this.getParentModel();
// |    if (model.emf$isEMFModel()) {
// |       EMFModelPartRoot root = model.emf$getEMFRootModel();
// |       Consumer<PoseStack> positioner = root.getPositionerForAttachment(EMFAttachment.Type.WITCH);
// |       if (positioner != null) {
// |          positioner.accept(poseStack);
// |          needsCancel.set(true);
// |          return false;
// |       }
// |    }
// |
// |    return true;
// | }
