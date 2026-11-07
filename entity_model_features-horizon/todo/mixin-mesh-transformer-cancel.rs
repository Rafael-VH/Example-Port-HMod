// Not ported yet: the Java @Inject(HEAD) on MeshTransformer.lambda$scaling$0 (MixinMeshTransformer.emf$cancel).
// Why it's off: No Horizon function mapped for MeshTransformer.lambda$scaling$0 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for MeshTransformer.lambda$scaling$0: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MeshTransformer.lambda$scaling$0 -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/exporting/MixinMeshTransformer.java:13:
// | @Inject(method = "lambda$scaling$0", at = @At("HEAD"), cancellable = true)
// | private static void emf$cancel(float f, float g, MeshDefinition meshDefinition, CallbackInfoReturnable<MeshDefinition> cir) {
// |    if (EMF.tempDisableModelModifications) {
// |       cir.setReturnValue(meshDefinition);
// |    }
// | }
