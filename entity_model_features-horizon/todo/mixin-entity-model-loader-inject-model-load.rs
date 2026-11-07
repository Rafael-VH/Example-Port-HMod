// Not ported yet: the Java @Inject(RETURN) on EntityModelSet.bakeLayer (MixinEntityModelLoader.emf$injectModelLoad).
// Why it's off: No Horizon function mapped for EntityModelSet.bakeLayer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for EntityModelSet.bakeLayer: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityModelSet.bakeLayer -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinEntityModelLoader.java:15:
// | @Inject(method = "bakeLayer", at = @At("RETURN"), cancellable = true)
// | private void emf$injectModelLoad(ModelLayerLocation layer, CallbackInfoReturnable<ModelPart> cir) {
// |    if (!EMF.testForForgeLoadingError()) {
// |       cir.setReturnValue(EMFManager.getInstance().injectIntoModelRootGetter(layer, (ModelPart)cir.getReturnValue()));
// |    }
// | }
