// Not ported yet: the Java @Inject(INVOKE) on LivingEntityRenderer.submit (MixinLivingEntityRenderer.falseAnimation).
// Why it's off: No Horizon function mapped for LivingEntityRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for LivingEntityRenderer.submit: run `match`.
// Note: The call it hooks (EntityModel.setupAnim) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Iterator.next -> (not mapped yet)
//   List.iterator -> (not mapped yet)
//   EntityModel.setupAnim -> (not mapped yet)
//   EntityRenderer.<init> -> (not mapped yet)
//   LivingEntityRenderer.getRenderType -> (not mapped yet)
//   LivingEntityRenderer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinLivingEntityRenderer.java:49:
// | @Inject(
// |    method = "submit(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/model/EntityModel;setupAnim(Ljava/lang/Object;)V", shift = Shift.AFTER)
// | )
// | private void falseAnimation(CallbackInfo ci, @Local(argsOnly = true) S renderState) {
// |    IEMFModel model = (IEMFModel)this.model;
// |    if (model.emf$isEMFModel() && model.emf$getEMFRootModel().hasAnimation()) {
// |       if (EMFAnimationPauseHandler.shouldAnimationsPause(EMFEntityRenderState.from(renderState))) {
// |          return;
// |       }
// |
// |       model.emf$getEMFRootModel().animateNoPause();
// |       if (this.getModel() instanceof HumanoidModel<?> humanoidModel) {
// |          EMFEntityRenderState state = (EMFEntityRenderState)((HoldsETFRenderState)renderState).etf$getState();
// |          if (state != null) {
// |             state.setBipedPose(new EMFBipedPose(humanoidModel));
// |          }
// |       }
// |    }
// | }
