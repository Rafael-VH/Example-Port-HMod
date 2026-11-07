// Not ported yet: the Java @Inject(TAIL) on ParrotOnShoulderLayer.submitOnShoulder (MixinParrotEntityModel.emf$parrot2).
// Why it's off: No Horizon function mapped for ParrotOnShoulderLayer.submitOnShoulder (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ParrotOnShoulderLayer.submitOnShoulder: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ParrotModel.createBodyLayer -> (not mapped yet)
//   ModelLayerLocation.<init> -> (not mapped yet)
//   LayerDefinition.bakeRoot -> (not mapped yet)
//   ParrotOnShoulderLayer.<init> -> (not mapped yet)
//   ParrotOnShoulderLayer.submitOnShoulder -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinParrotEntityModel.java:44:
// | @Inject(method = "submitOnShoulder", at = @At("TAIL"))
// | private void emf$parrot2(CallbackInfo ci) {
// |    EMFState.isInShoulderMethod = false;
// | }
