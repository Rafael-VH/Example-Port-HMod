// Not ported yet: the Java @WrapWithCondition(INVOKE) on ParrotOnShoulderLayer.submitOnShoulder (Mixin_Parrot_ShoulderOffset.offsetBlock).
// Why it's off: No Horizon function mapped for ParrotOnShoulderLayer.submitOnShoulder (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: Call `original` only when the condition holds.
// Note: No Horizon function known for ParrotOnShoulderLayer.submitOnShoulder: run `match`.
// Note: The call it hooks (PoseStack.translate) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.<init> -> (not mapped yet)
//   PoseStack.last -> (not mapped yet)
//   PoseStack.translate -> (not mapped yet)
//   Pose.pose -> (not mapped yet)
//   ParrotOnShoulderLayer.submitOnShoulder -> (not mapped yet)
//   RenderLayer.<init> -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/attachments/Mixin_Parrot_ShoulderOffset.java:26:
// | @WrapWithCondition(method = "submitOnShoulder", at = @At(value = "INVOKE", target = "Lcom/mojang/blaze3d/vertex/PoseStack;translate(FFF)V", ordinal = 0))
// | private boolean offsetBlock(PoseStack instance, float f, float g, float h, @Local(argsOnly = true) boolean isLeft) {
// |    IEMFModel model = (IEMFModel)this.getParentModel();
// |    if (model.emf$isEMFModel()) {
// |       EMFModelPartRoot root = model.emf$getEMFRootModel();
// |       Consumer<PoseStack> positioner = root.getPositionerForAttachment(isLeft ? EMFAttachment.Type.PARROT_LEFT : EMFAttachment.Type.PARROT_RIGHT);
// |       if (positioner != null) {
// |          positioner.accept(instance);
// |          return false;
// |       }
// |
// |       if (((EMFConfig)EMF.config().getConfig()).parrotShoulderPositionAnimatesByDefault) {
// |          positioner = root.getPositionerForAttachment(isLeft ? EMFAttachment.Type.PARROT_LEFT_AUTO : EMFAttachment.Type.PARROT_RIGHT_AUTO);
// |          if (positioner != null) {
// |             PoseStack tempMatrix = new PoseStack();
// |             positioner.accept(tempMatrix);
// |             Vector4f pos = new Vector4f(isLeft ? 0.4F : -0.4F, -1.5F, 0.0F, 1.0F);
// |             tempMatrix.last().pose().transform(pos);
// |             instance.translate(pos.x(), -1.5F + pos.y(), pos.z());
// |             return false;
// |          }
// |       }
// |    }
// |
// |    return true;
// | }
