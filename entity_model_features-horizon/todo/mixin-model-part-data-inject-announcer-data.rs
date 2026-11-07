// Not ported yet: the Java @Inject(RETURN) on PartDefinition.bake (MixinModelPartData.emf$injectAnnouncerData).
// Why it's off: No Horizon function mapped for PartDefinition.bake (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for PartDefinition.bake: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PartDefinition.bake -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinModelPartData.java:13:
// | @Inject(method = "bake", at = @At("RETURN"))
// | private void emf$injectAnnouncerData(int textureWidth, int textureHeight, CallbackInfoReturnable<ModelPart> cir) {
// |    if (cir.getReturnValue() instanceof IEMFTextureSizeSupplier) {
// |       ((IEMFTextureSizeSupplier)cir.getReturnValue()).emf$setTextureSize(new int[]{textureWidth, textureHeight});
// |    }
// | }
