// Not ported yet: the Java @Inject(RETURN) on WingsLayer.submit (MixinElytraFeatureRenderer.etf$markPatchableEnd).
// Why it's off: No Horizon function mapped for WingsLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for WingsLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   WingsLayer.submit -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/feature/MixinElytraFeatureRenderer.java:21:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/HumanoidRenderState;FF)V",
// |    at = @At("RETURN")
// | )
// | private void etf$markPatchableEnd(CallbackInfo ci) {
// |    ETFState.allowTexturePatching = false;
// | }
