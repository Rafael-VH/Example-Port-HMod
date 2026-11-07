// Not ported yet: the Java @Inject(TAIL) on ChickenRenderer.<init> (Mixin_ChickenRenderer_WarmModel.emf$createWarmModels).
// Why it's off: No Horizon function mapped for ChickenRenderer.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ChickenRenderer.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AdultAndBabyModelPair.<init> -> (not mapped yet)
//   AdultAndBabyModelPair.getModel -> (not mapped yet)
//   AdultChickenModel.<init> -> (not mapped yet)
//   AdultChickenModel.createBodyLayer -> (not mapped yet)
//   BabyChickenModel.<init> -> (not mapped yet)
//   BabyChickenModel.createBodyLayer -> (not mapped yet)
//   ModelLayerLocation.<init> -> (not mapped yet)
//   LayerDefinition.bakeRoot -> (not mapped yet)
//   ChickenRenderer.<init> -> (not mapped yet)
//   ChickenRenderer.submit -> (not mapped yet)
//   MobRenderer.<init> -> (not mapped yet)
//   MobRenderer.submit -> (not mapped yet)
//   Holder.getRegisteredName -> (not mapped yet)
//   Chicken.getVariant -> (not mapped yet)
//   Chicken.isBaby -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/model/Mixin_ChickenRenderer_WarmModel.java:40:
// | @Inject(method = "<init>", at = @At("TAIL"))
// | private void emf$createWarmModels(Context context, CallbackInfo ci) {
// |    if (!EMF.testForForgeLoadingError()) {
// |       this.coldBabyModel = new BabyChickenModel(
// |          EMFManager.getInstance().injectIntoModelRootGetter(emf$cold_baby, BabyChickenModel.createBodyLayer().bakeRoot())
// |       );
// |       this.models = new AdultAndBabyModelPair(
// |          new AdultChickenModel(EMFManager.getInstance().injectIntoModelRootGetter(emf$warm, AdultChickenModel.createBodyLayer().bakeRoot())),
// |          new BabyChickenModel(EMFManager.getInstance().injectIntoModelRootGetter(emf$warm_baby, BabyChickenModel.createBodyLayer().bakeRoot()))
// |       );
// |    }
// | }
