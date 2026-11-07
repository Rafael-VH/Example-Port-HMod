// Not ported yet: the Java @Inject(HEAD) on Model.renderType (MixinModel.emf$discoverEMFModel).
// Why it's off: No Horizon function mapped for Model.renderType (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Model.renderType: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Model.<init> -> (not mapped yet)
//   Model.renderType -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinModel.java:26:
// | @Inject(method = "<init>", at = @At("TAIL"))
// | private void emf$discoverEMFModel(ModelPart modelPart, Function<?, ?> function, CallbackInfo ci) {
// |    if (!EMF.testForForgeLoadingError()) {
// |       if (modelPart instanceof EMFModelPartRoot root) {
// |          this.emf$thisEMFModelRoot = root;
// |       } else if (modelPart instanceof EMFModelPart nonRoot) {
// |          this.emf$thisEMFModelRoot = nonRoot.getRoot();
// |       }
// |
// |       EMFManager.lastCreatedRootModelPart = null;
// |    }
// | }
