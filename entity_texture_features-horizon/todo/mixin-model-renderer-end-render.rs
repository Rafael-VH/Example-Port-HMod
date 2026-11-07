// Not ported yet: the Java @Inject(TAIL) on ModelFeatureRenderer.prepareModel (Mixin_ModelRenderer.emf$endRender).
// Why it's off: No Horizon function mapped for ModelFeatureRenderer.prepareModel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ModelFeatureRenderer.prepareModel: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelFeatureRenderer.prepareModel -> (not mapped yet)
//   Submit.lightCoords -> (not mapped yet)
//   Submit.state -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/submit/Mixin_ModelRenderer.java:66:
// | @Inject(method = "prepareModel", at = @At("TAIL"))
// | private <S> void emf$endRender(CallbackInfo ci, @Local(argsOnly = true) Submit<S> modelSubmit) {
// |    tailLogic(modelSubmit);
// | }
