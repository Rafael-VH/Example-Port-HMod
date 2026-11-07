// Not ported yet: the Java @Inject(HEAD) on CustomHeadLayer.submit (MixinHeadFeatureRenderer.emf$setHand).
// Why it's off: No Horizon function mapped for CustomHeadLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for CustomHeadLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CustomHeadLayer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/feature/MixinHeadFeatureRenderer.java:14:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;FF)V",
// |    at = @At("HEAD")
// | )
// | private void emf$setHand(CallbackInfo ci) {
// |    EMFState.isOnHead = true;
// | }
