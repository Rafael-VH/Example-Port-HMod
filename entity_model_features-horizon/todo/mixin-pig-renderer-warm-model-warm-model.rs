// Not ported yet: the Java @Inject(INVOKE) on PigRenderer.submit (Mixin_PigRenderer_WarmModel.emf$warmModel).
// Why it's off: No Horizon function mapped for PigRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for PigRenderer.submit: run `match`.
// Note: The call it hooks (MobRenderer.submit) isn't mapped to a Rust function yet.
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
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/model/Mixin_PigRenderer_WarmModel.java:53:
// | @Inject(
// |    method = "submit(Lnet/minecraft/client/renderer/entity/state/PigRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/entity/MobRenderer;submit(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V"
// |    )
// | )
// | private void emf$warmModel(CallbackInfo ci, @Local(argsOnly = true) PigRenderState pigRenderState) {
// |    ETFEntityRenderState state = ((HoldsETFRenderState)pigRenderState).etf$getState();
// |    if (state != null && state.entity() != null && state.entity() instanceof Pig oink && "minecraft:warm".equals(oink.getVariant().getRegisteredName())) {
// |       this.model = (EntityModel)this.models.getModel(oink.isBaby());
// |    }
// |
// |    if (state != null
// |       && state.entity() != null
// |       && state.entity() instanceof Pig oink
// |       && "minecraft:cold".equals(oink.getVariant().getRegisteredName())
// |       && oink.isBaby()) {
// |       this.model = this.coldBabyModel;
// |    }
// | }
