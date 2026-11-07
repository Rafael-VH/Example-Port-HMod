// Not ported yet: the Java @Inject(HEAD) on ParrotOnShoulderLayer.submitOnShoulder (MixinParrotEntityModel.emf$parrot1).
// Why it's off: No Horizon function mapped for ParrotOnShoulderLayer.submitOnShoulder (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ParrotOnShoulderLayer.submitOnShoulder: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ParrotModel.createBodyLayer -> (not mapped yet)
//   ModelLayerLocation.<init> -> (not mapped yet)
//   LayerDefinition.bakeRoot -> (not mapped yet)
//   ParrotOnShoulderLayer.<init> -> (not mapped yet)
//   ParrotOnShoulderLayer.submitOnShoulder -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinParrotEntityModel.java:38:
// | @Inject(method = "submitOnShoulder", at = @At("HEAD"))
// | private void emf$parrot1(CallbackInfo ci, @Local(argsOnly = true) boolean isLeft) {
// |    EMFState.isInShoulderMethod = true;
// |    EMFState.isLeftShoulder = isLeft;
// | }
