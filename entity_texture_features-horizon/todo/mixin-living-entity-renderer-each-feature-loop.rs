// Not ported yet: the Java @Inject(INVOKE) on LivingEntityRenderer.submit (MixinLivingEntityRenderer.emf$eachFeatureLoop).
// Why it's off: No Horizon function mapped for LivingEntityRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for LivingEntityRenderer.submit: run `match`.
// Note: The call it hooks (Iterator.next) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.popPose -> (not mapped yet)
//   Iterator.next -> (not mapped yet)
//   List.iterator -> (not mapped yet)
//   EntityRenderer.<init> -> (not mapped yet)
//   LivingEntityRenderer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/MixinLivingEntityRenderer.java:53:
// | @Inject(
// |    method = "submit(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At(value = "INVOKE", target = "Ljava/util/Iterator;next()Ljava/lang/Object;")
// | )
// | private void emf$eachFeatureLoop(CallbackInfo ci, @Share("stateCaptureEntity") LocalRef<ETFEntityRenderState> stateCaptureEntity) {
// |    if (stateCaptureEntity.get() != null) {
// |       ETFState.stackVerify((ETFEntityRenderState)stateCaptureEntity.get());
// |    }
// | }
