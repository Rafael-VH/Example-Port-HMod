// Not ported yet: the Java @Inject(INVOKE) on ModelFeatureRenderer.prepareModel (Mixin_ModelRenderer.emf$animate).
// Why it's off: No Horizon function mapped for ModelFeatureRenderer.prepareModel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for ModelFeatureRenderer.prepareModel: run `match`.
// Note: The call it hooks (Model.setupAnim) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Model.root -> (not mapped yet)
//   Model.setupAnim -> (not mapped yet)
//   ModelFeatureRenderer.prepareModel -> (not mapped yet)
//   Submit.model -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/submits/Mixin_ModelRenderer.java:19:
// | @Inject(
// |    method = "prepareModel",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/model/Model;setupAnim(Ljava/lang/Object;)V", shift = Shift.AFTER)
// | )
// | private <S> void emf$animate(CallbackInfo ci, @Local(argsOnly = true) Submit<S> modelSubmit) {
// |    this.applyArmorBipedPose(modelSubmit);
// | }
