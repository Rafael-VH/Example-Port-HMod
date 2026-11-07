// Not ported yet: the Java @WrapWithCondition(INVOKE) on CrossedArmsItemLayer.applyTranslation (Mixin_Villager_BlockOffset.offsetBlock).
// Why it's off: No Horizon function mapped for CrossedArmsItemLayer.applyTranslation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: Call `original` only when the condition holds.
// Note: No Horizon function known for CrossedArmsItemLayer.applyTranslation: run `match`.
// Note: The call it hooks (VillagerLikeModel.translateToArms) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   VillagerLikeModel.translateToArms -> (not mapped yet)
//   CrossedArmsItemLayer.applyTranslation -> (not mapped yet)
//   RenderLayer.<init> -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/attachments/Mixin_Villager_BlockOffset.java:22:
// | @WrapWithCondition(
// |    method = "applyTranslation",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/model/VillagerLikeModel;translateToArms(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;)V",
// |       ordinal = 0
// |    )
// | )
// | private boolean offsetBlock(VillagerLikeModel instance, EntityRenderState entityRenderState, PoseStack poseStack) {
// |    IEMFModel model = (IEMFModel)this.getParentModel();
// |    if (model.emf$isEMFModel()) {
// |       EMFModelPartRoot root = model.emf$getEMFRootModel();
// |       Consumer<PoseStack> positioner = root.getPositionerForAttachment(EMFAttachment.Type.VILLAGER);
// |       if (positioner != null) {
// |          positioner.accept(poseStack);
// |          return false;
// |       }
// |    }
// |
// |    return true;
// | }
