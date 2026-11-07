// Not ported yet: the Java @Inject(HEAD) on LayerRenderState.submit (MixinBlockEntityWithoutLevelRenderer_skull.emf$setRenderFactory).
// Why it's off: No Horizon function mapped for LayerRenderState.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for LayerRenderState.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LayerRenderState.submit -> (not mapped yet)
//   RenderTypes.entityCutoutZOffset -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinBlockEntityWithoutLevelRenderer_skull.java:23:
// | @Inject(method = "submit", at = @At("HEAD"))
// | private void emf$setRenderFactory(CallbackInfo ci) {
// |    EMFEntityRenderState state = EMFState.state();
// |    if (this.specialRenderer instanceof SkullSpecialRenderer && state != null) {
// |       state.setLayerFactory(RenderTypes::entityCutoutZOffset);
// |    }
// | }
