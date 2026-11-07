// Not ported yet: the Java @Inject(HEAD) on SlimeOuterLayer.submit (MixinSlimeOverlayFeatureRenderer.emf$setLayerForOverrides).
// Why it's off: No Horizon function mapped for SlimeOuterLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for SlimeOuterLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SlimeOuterLayer.submit -> (not mapped yet)
//   RenderTypes.entityTranslucent -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/feature/MixinSlimeOverlayFeatureRenderer.java:13:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/SlimeRenderState;FF)V",
// |    at = @At("HEAD")
// | )
// | private void emf$setLayerForOverrides(CallbackInfo ci) {
// |    if (EMFState.state() != null) {
// |       EMFState.state().setLayerFactory(RenderTypes::entityTranslucent);
// |    }
// | }
