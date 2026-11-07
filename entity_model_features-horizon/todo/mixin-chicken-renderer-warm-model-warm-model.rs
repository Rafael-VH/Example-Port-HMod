// Not ported yet: the Java @Inject(INVOKE) on ChickenRenderer.submit (Mixin_ChickenRenderer_WarmModel.emf$warmModel).
// Why it's off: No Horizon function mapped for ChickenRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for ChickenRenderer.submit: run `match`.
// Note: The call it hooks (MobRenderer.submit) isn't mapped to a Rust function yet.
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
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/model/Mixin_ChickenRenderer_WarmModel.java:53:
// | @Inject(
// |    method = "submit(Lnet/minecraft/client/renderer/entity/state/ChickenRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/entity/MobRenderer;submit(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V"
// |    )
// | )
// | private void emf$warmModel(CallbackInfo ci, @Local(argsOnly = true) ChickenRenderState renderState) {
// |    ETFEntityRenderState state = ((HoldsETFRenderState)renderState).etf$getState();
// |    if (state != null && state.entity() != null && state.entity() instanceof Chicken cluck && "minecraft:warm".equals(cluck.getVariant().getRegisteredName())
// |       )
// |     {
// |       this.model = (EntityModel)this.models.getModel(cluck.isBaby());
// |    }
// |
// |    if (state != null
// |       && state.entity() != null
// |       && state.entity() instanceof Chicken cluck
// |       && "minecraft:cold".equals(cluck.getVariant().getRegisteredName())
// |       && cluck.isBaby()) {
// |       this.model = this.coldBabyModel;
// |    }
// | }
