// Not ported yet: the Java @Inject(TAIL) on PigRenderer.<init> (Mixin_PigRenderer_WarmModel.emf$createWarmModels).
// Why it's off: No Horizon function mapped for PigRenderer.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for PigRenderer.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AdultAndBabyModelPair.<init> -> (not mapped yet)
//   AdultAndBabyModelPair.getModel -> (not mapped yet)
//   BabyPigModel.<init> -> (not mapped yet)
//   BabyPigModel.createBodyLayer -> (not mapped yet)
//   PigModel.<init> -> (not mapped yet)
//   PigModel.createBodyLayer -> (not mapped yet)
//   ModelLayerLocation.<init> -> (not mapped yet)
//   CubeDeformation.NONE -> (not mapped yet)
//   LayerDefinition.bakeRoot -> (not mapped yet)
//   MobRenderer.<init> -> (not mapped yet)
//   MobRenderer.submit -> (not mapped yet)
//   PigRenderer.<init> -> (not mapped yet)
//   PigRenderer.submit -> (not mapped yet)
//   Holder.getRegisteredName -> (not mapped yet)
//   Pig.getVariant -> (not mapped yet)
//   Pig.isBaby -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/model/Mixin_PigRenderer_WarmModel.java:40:
// | @Inject(method = "<init>", at = @At("TAIL"))
// | private void emf$createWarmModels(Context context, CallbackInfo ci) {
// |    if (!EMF.testForForgeLoadingError()) {
// |       this.coldBabyModel = new BabyPigModel(
// |          EMFManager.getInstance().injectIntoModelRootGetter(emf$cold_baby, BabyPigModel.createBodyLayer(CubeDeformation.NONE).bakeRoot())
// |       );
// |       this.models = new AdultAndBabyModelPair(
// |          new PigModel(EMFManager.getInstance().injectIntoModelRootGetter(emf$warm, PigModel.createBodyLayer(CubeDeformation.NONE).bakeRoot())),
// |          new BabyPigModel(EMFManager.getInstance().injectIntoModelRootGetter(emf$warm_baby, BabyPigModel.createBodyLayer(CubeDeformation.NONE).bakeRoot()))
// |       );
// |    }
// | }
