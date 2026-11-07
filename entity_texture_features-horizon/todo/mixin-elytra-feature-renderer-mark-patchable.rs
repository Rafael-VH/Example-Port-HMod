// Not ported yet: the Java @Inject(HEAD) on WingsLayer.submit (MixinElytraFeatureRenderer.etf$markPatchable).
// Why it's off: No Horizon function mapped for WingsLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for WingsLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   WingsLayer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/feature/MixinElytraFeatureRenderer.java:13:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/HumanoidRenderState;FF)V",
// |    at = @At("HEAD")
// | )
// | private void etf$markPatchable(CallbackInfo ci) {
// |    ETFState.allowTexturePatching = true;
// | }
