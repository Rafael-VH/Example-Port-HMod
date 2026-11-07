// Not ported yet: the Java @Inject(TAIL) on LivingEntityRenderer.submit (MixinLivingEntityRenderer.emf$postRender).
// Why it's off: No Horizon function mapped for LivingEntityRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for LivingEntityRenderer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.popPose -> (not mapped yet)
//   Iterator.next -> (not mapped yet)
//   List.iterator -> (not mapped yet)
//   EntityRenderer.<init> -> (not mapped yet)
//   LivingEntityRenderer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/MixinLivingEntityRenderer.java:63:
// | @Inject(
// |    method = "submit(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At("TAIL")
// | )
// | private void emf$postRender(CallbackInfo ci, @Share("stateCaptureEntity") LocalRef<ETFEntityRenderState> stateCaptureEntity) {
// |    if (stateCaptureEntity.get() != null) {
// |       ETFState.stackVerify((ETFEntityRenderState)stateCaptureEntity.get());
// |    }
// | }
