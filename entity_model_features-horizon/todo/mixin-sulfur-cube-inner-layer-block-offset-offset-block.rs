// Not ported yet: the Java @Inject(INVOKE) on SulfurCubeInnerLayer.submit (MixinSulfurCubeInnerLayer_BlockOffset.offsetBlock).
// Why it's off: No Horizon function mapped for SulfurCubeInnerLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for SulfurCubeInnerLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.rotateDegrees -> horizon_client_render::camera::rotate_degrees (suggested, high)
//   SulfurCubeInnerLayer.submit -> (not mapped yet)
//   SulfurCubeRenderState.isBaby -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/attachments/MixinSulfurCubeInnerLayer_BlockOffset.java:28:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/SulfurCubeRenderState;FF)V",
// |    at = @At(value = "INVOKE", target = "Lcom/mojang/blaze3d/vertex/PoseStack;rotateDegrees(Lcom/mojang/math/Axis;F)V")
// | )
// | private void offsetBlock(CallbackInfo ci, @Local(argsOnly = true) PoseStack poseStack, @Local(argsOnly = true) SulfurCubeRenderState vanilla) {
// |    IEMFModel model = (IEMFModel)(vanilla.isBaby ? this.smallModel : this.normalModel);
// |    if (model.emf$isEMFModel()) {
// |       EMFModelPartRoot root = model.emf$getEMFRootModel();
// |       Consumer<PoseStack> positioner = root.getPositionerForAttachment(EMFAttachment.Type.SULFUR_CUBE);
// |       if (positioner != null) {
// |          root.animate();
// |          positioner.accept(poseStack);
// |       }
// |    }
// | }
