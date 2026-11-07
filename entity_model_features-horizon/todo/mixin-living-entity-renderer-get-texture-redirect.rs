// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on LivingEntityRenderer.getRenderType (MixinLivingEntityRenderer.emf$getTextureRedirect).
// Why it's off: No Horizon function mapped for LivingEntityRenderer.getRenderType (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for LivingEntityRenderer.getRenderType: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Iterator.next -> (not mapped yet)
//   List.iterator -> (not mapped yet)
//   EntityModel.setupAnim -> (not mapped yet)
//   EntityRenderer.<init> -> (not mapped yet)
//   LivingEntityRenderer.getRenderType -> (not mapped yet)
//   LivingEntityRenderer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinLivingEntityRenderer.java:70:
// | @ModifyExpressionValue(
// |    method = "getRenderType",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/entity/LivingEntityRenderer;getTextureLocation(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;)Lnet/minecraft/resources/Identifier;"
// |    )
// | )
// | private Identifier emf$getTextureRedirect(Identifier original) {
// |    if (((IEMFModel)this.model).emf$isEMFModel()) {
// |       EMFModelPartRoot root = ((IEMFModel)this.model).emf$getEMFRootModel();
// |       if (root != null) {
// |          Identifier texture = root.getTopLevelJemTexture();
// |          if (texture != null) {
// |             return texture;
// |          }
// |       }
// |    }
// |
// |    return original;
// | }
