// Not ported yet: the Java @Inject(HEAD) on VillagerModel.setupAnim (MixinVillagerModel.emf$assertLayerFactory).
// Why it's off: No Horizon function mapped for VillagerModel.setupAnim (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for VillagerModel.setupAnim: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   VillagerModel.setupAnim -> (not mapped yet)
//   RenderTypes.entityCutout -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/model/MixinVillagerModel.java:13:
// | @Inject(method = "setupAnim(Lnet/minecraft/client/renderer/entity/state/VillagerRenderState;)V", at = @At("HEAD"))
// | private void emf$assertLayerFactory(CallbackInfo ci) {
// |    if (EMFState.state() != null) {
// |       EMFState.state().setLayerFactory(RenderTypes::entityCutout);
// |    }
// | }
