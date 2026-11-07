// Not ported yet: the Java @Inject(INVOKE) on AvatarRenderer.renderHand (MixinPlayerEntityRenderer.emf$setHandAnims).
// Why it's off: No Horizon function mapped for AvatarRenderer.renderHand (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for AvatarRenderer.renderHand: run `match`.
// Note: The call it hooks (SubmitNodeCollector.submitModelPart) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SubmitNodeCollector.submitModelPart -> (not mapped yet)
//   LivingEntityRenderer.<init> -> (not mapped yet)
//   AvatarRenderer.renderHand -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinPlayerEntityRenderer.java:34:
// | @Inject(
// |    method = "renderHand",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/SubmitNodeCollector;submitModelPart(Lnet/minecraft/client/model/geom/ModelPart;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IILnet/minecraft/client/renderer/texture/UvMapping;)V"
// |    )
// | )
// | private void emf$setHandAnims(CallbackInfo ci, @Local(argsOnly = true) ModelPart modelPart) {
// |    if (modelPart instanceof EMFModelPartVanilla vanilla) {
// |       vanilla.isPlayerArm = true;
// |       vanilla.getRoot().animate();
// |    }
// | }
