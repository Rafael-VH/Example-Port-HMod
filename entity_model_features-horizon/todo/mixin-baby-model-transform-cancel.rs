// Not ported yet: the Java @Inject(HEAD) on BabyModelTransform.apply (MixinBabyModelTransform.emf$cancel).
// Why it's off: No Horizon function mapped for BabyModelTransform.apply (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for BabyModelTransform.apply: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BabyModelTransform.apply -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/exporting/MixinBabyModelTransform.java:13:
// | @Inject(method = "apply", at = @At("HEAD"), cancellable = true)
// | private void emf$cancel(MeshDefinition meshDefinition, CallbackInfoReturnable<MeshDefinition> cir) {
// |    if (EMF.tempDisableModelModifications) {
// |       cir.setReturnValue(meshDefinition);
// |    }
// | }
