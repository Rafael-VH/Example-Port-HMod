// Not ported yet: the Java @Inject(TAIL) on WolfCollarLayer.<init> (MixinWolfCollarFeatureRenderer.setEmf$Model).
// Why it's off: No Horizon function mapped for WolfCollarLayer.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for WolfCollarLayer.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AdultWolfModel.<init> -> (not mapped yet)
//   AdultWolfModel.createBodyLayer -> (not mapped yet)
//   BabyWolfModel.<init> -> (not mapped yet)
//   BabyWolfModel.createBodyLayer -> (not mapped yet)
//   ModelLayerLocation.<init> -> (not mapped yet)
//   CubeDeformation.NONE -> (not mapped yet)
//   LayerDefinition.bakeRoot -> (not mapped yet)
//   MeshDefinition.getRoot -> (not mapped yet)
//   PartDefinition.bake -> (not mapped yet)
//   RenderLayerParent.getModel -> (not mapped yet)
//   RenderLayer.<init> -> (not mapped yet)
//   RenderLayer.getParentModel -> (not mapped yet)
//   WolfCollarLayer.<init> -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/feature/MixinWolfCollarFeatureRenderer.java:39:
// | @Inject(method = "<init>", at = @At("TAIL"))
// | private void setEmf$Model(RenderLayerParent<?, ?> featureRendererContext, CallbackInfo ci) {
// |    if (!EMF.testForForgeLoadingError()) {
// |       ModelPart collarModel = EMFManager.getInstance()
// |          .injectIntoModelRootGetter(emf$collar_layer, AdultWolfModel.createBodyLayer(CubeDeformation.NONE).getRoot().bake(64, 32));
// |       if (collarModel instanceof EMFModelPartRoot || ((IEMFModel)featureRendererContext.getModel()).emf$isEMFModel()) {
// |          try {
// |             if (featureRendererContext.getModel() instanceof IEMFWolfCollarHolder holder) {
// |                holder.emf$setCollarModel(new AdultWolfModel(collarModel));
// |             }
// |          } catch (Exception var9) {
// |          }
// |       }
// |
// |       ModelPart collarModelBaby = EMFManager.getInstance().injectIntoModelRootGetter(emf$collar_layer_baby, BabyWolfModel.createBodyLayer().bakeRoot());
// |       if (collarModelBaby instanceof EMFModelPartRoot
// |          || EMFManager.getInstance()
// |             .injectIntoModelRootGetter(new ModelLayerLocation(EMFUtils.res("minecraft", "wolf_baby"), "main"), BabyWolfModel.createBodyLayer().bakeRoot()) instanceof EMFModelPart
// |          )
// |        {
// |          try {
// |             if (featureRendererContext instanceof AgeableMobRendererAccessor ageModelsAccessor
// |                && ageModelsAccessor.getBabyModel() instanceof IEMFWolfCollarHolder holder) {
// |                holder.emf$setCollarModel(new BabyWolfModel(collarModelBaby));
// |             }
// |          } catch (Exception var8) {
// |          }
// |       }
// |    }
// | }
