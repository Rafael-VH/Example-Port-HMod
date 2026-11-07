// Not ported yet: the Java @Inject(INVOKE) on LivingEntityRenderer.submit (MixinLivingEntityRenderer.emf$grabEntity).
// Why it's off: No Horizon function mapped for LivingEntityRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for LivingEntityRenderer.submit: run `match`.
// Note: The call it hooks (List.iterator) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Iterator.next -> (not mapped yet)
//   List.iterator -> (not mapped yet)
//   EntityModel.setupAnim -> (not mapped yet)
//   EntityRenderer.<init> -> (not mapped yet)
//   LivingEntityRenderer.getRenderType -> (not mapped yet)
//   LivingEntityRenderer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinLivingEntityRenderer.java:91:
// | @Inject(
// |    method = "submit(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At(value = "INVOKE", target = "Ljava/util/List;iterator()Ljava/util/Iterator;")
// | )
// | private void emf$grabEntity(CallbackInfo ci, @Share("stateCaptureStatic") LocalRef<EMFState.EMFStateStaticSnapshot> stateCaptureStatic) {
// |    EMFState.isLayerPhase = true;
// |    EMFState.isMainPhase = false;
// |    stateCaptureStatic.set(EMFState.captureStatics());
// | }
