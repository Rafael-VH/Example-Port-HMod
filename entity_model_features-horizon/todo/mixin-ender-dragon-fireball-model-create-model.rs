// Not ported yet: the Java @Inject(TAIL) on DragonFireballRenderer.<init> (Mixin_EnderDragonFireball_Model.emf$createModel).
// Why it's off: No Horizon function mapped for DragonFireballRenderer.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for DragonFireballRenderer.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelLayerLocation.<init> -> (not mapped yet)
//   ModelPart.<init> -> (not mapped yet)
//   SubmitNodeCollector.submitModel -> (not mapped yet)
//   DragonFireballRenderer.<init> -> (not mapped yet)
//   DragonFireballRenderer.submit -> (not mapped yet)
//   EntityRenderState.lightCoords -> (not mapped yet)
//   OverlayTexture.NO_OVERLAY -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/model/Mixin_EnderDragonFireball_Model.java:39:
// | @Inject(method = "<init>", at = @At("TAIL"))
// | private void emf$createModel(Context context, CallbackInfo ci) {
// |    if (!EMF.testForForgeLoadingError()) {
// |       ModelPart possibleModel = EMFManager.getInstance()
// |          .injectIntoModelRootGetter(emf$fireball, new ModelPart(List.of(), Map.of("fireball", new ModelPart(List.of(), Map.of()))));
// |       if (possibleModel instanceof EMFModelPartRoot) {
// |          this.fireball = new EntityModel<EntityRenderState>(possibleModel) {};
// |       }
// |    }
// | }
