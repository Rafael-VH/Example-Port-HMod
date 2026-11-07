// Not ported yet: the Java @Inject(RETURN) on CreeperPowerLayer.getTextureLocation (MixinCreeperChargeFeatureRenderer.emf$getTextureRedirect).
// Why it's off: No Horizon function mapped for CreeperPowerLayer.getTextureLocation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for CreeperPowerLayer.getTextureLocation: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CreeperPowerLayer.getTextureLocation -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/feature/MixinCreeperChargeFeatureRenderer.java:21:
// | @Inject(method = "getTextureLocation", at = @At("RETURN"), cancellable = true)
// | private void emf$getTextureRedirect(CallbackInfoReturnable<Identifier> cir) {
// |    if (this.model != null && ((IEMFModel)this.model).emf$isEMFModel()) {
// |       EMFModelPartRoot root = ((IEMFModel)this.model).emf$getEMFRootModel();
// |       if (root != null) {
// |          Identifier texture = root.getTopLevelJemTexture();
// |          if (texture != null) {
// |             cir.setReturnValue(texture);
// |          }
// |       }
// |    }
// | }
