// Not ported yet: the Java @Inject(HEAD) on ModelFeatureRenderer.prepareModel (Mixin_ModelRenderer.emf$initRender).
// Why it's off: No Horizon function mapped for ModelFeatureRenderer.prepareModel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ModelFeatureRenderer.prepareModel: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelFeatureRenderer.prepareModel -> (not mapped yet)
//   Submit.lightCoords -> (not mapped yet)
//   Submit.state -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/submit/Mixin_ModelRenderer.java:61:
// | @Inject(method = "prepareModel", at = @At("HEAD"))
// | private <S> void emf$initRender(CallbackInfo ci, @Local(argsOnly = true) Submit<S> modelSubmit) {
// |    this.headLogic(modelSubmit);
// | }
