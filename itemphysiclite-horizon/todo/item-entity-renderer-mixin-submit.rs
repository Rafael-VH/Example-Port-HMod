// Not ported yet: the Java @Inject(HEAD) on ItemEntityRenderer.submit (ItemEntityRendererMixin.submit).
// Why it's off: No Horizon function mapped for ItemEntityRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ItemEntityRenderer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderer.<init> -> (not mapped yet)
//   EntityRenderer.submit -> (not mapped yet)
//   ItemEntityRenderer.extractRenderState -> (not mapped yet)
//   ItemEntityRenderer.submit -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysiclite/mixin/ItemEntityRendererMixin.java:31:
// | @Inject(
// |    method = "submit(Lnet/minecraft/client/renderer/entity/state/ItemEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At("HEAD"),
// |    cancellable = true,
// |    require = 1
// | )
// | private void submit(ItemEntityRenderState state, PoseStack poseStack, SubmitNodeCollector collector, CameraRenderState camera, CallbackInfo ci) {
// |    if (ItemPhysicLite.submit(state, poseStack, collector, camera, this.random)) {
// |       super.submit(state, poseStack, collector, camera);
// |       ci.cancel();
// |    }
// | }
