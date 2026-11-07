// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on ParrotOnShoulderLayer.<init> (MixinParrotEntityModel.emf$injectParrotShoulderLayer).
// Why it's off: No Horizon function mapped for ParrotOnShoulderLayer.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for ParrotOnShoulderLayer.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ParrotModel.createBodyLayer -> (not mapped yet)
//   ModelLayerLocation.<init> -> (not mapped yet)
//   LayerDefinition.bakeRoot -> (not mapped yet)
//   ParrotOnShoulderLayer.<init> -> (not mapped yet)
//   ParrotOnShoulderLayer.submitOnShoulder -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinParrotEntityModel.java:25:
// | @ModifyExpressionValue(
// |    method = "<init>",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/model/geom/EntityModelSet;bakeLayer(Lnet/minecraft/client/model/geom/ModelLayerLocation;)Lnet/minecraft/client/model/geom/ModelPart;"
// |    )
// | )
// | private ModelPart emf$injectParrotShoulderLayer(ModelPart original) {
// |    return EMF.testForForgeLoadingError()
// |       ? original
// |       : EMFManager.getInstance().injectIntoModelRootGetter(emf$parrot_shoulder, ParrotModel.createBodyLayer().bakeRoot());
// | }
