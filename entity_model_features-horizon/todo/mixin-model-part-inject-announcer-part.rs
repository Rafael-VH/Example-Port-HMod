// Not ported yet: the Java @Inject(HEAD) on ModelPart.render (MixinModelPart.emf$injectAnnouncerPart).
// Why it's off: No Horizon function mapped for ModelPart.render (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ModelPart.render: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelPart.render -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinModelPart.java:28:
// | @Inject(method = "render(Lcom/mojang/blaze3d/vertex/PoseStack;Lcom/mojang/blaze3d/vertex/VertexConsumer;III)V", at = @At("HEAD"))
// | private void emf$injectAnnouncerPart(PoseStack poseStack, VertexConsumer vertexConsumer, int i, int j, int k, CallbackInfo ci) {
// |    if (EMFState.announceModels && this.emf$modelInfo != null) {
// |       EMFManager.getInstance().modelsAnnounced.add(this.emf$modelInfo);
// |    }
// | }
