// Not ported yet: the Java @Inject(RETURN) on LevelRenderer.executeSolid (LevelRendererMixin.be_releaseSolidRenderPass).
// Why it's off: No Horizon function mapped for LevelRenderer.executeSolid (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for LevelRenderer.executeSolid: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelRenderer.executeSolid -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/client/LevelRendererMixin.java:20:
// | @Inject(method = "executeSolid", at = @At("RETURN"))
// | private void be_releaseSolidRenderPass(CallbackInfo ci) {
// |    BetterEndSkyRenderer.setActiveSolidRenderPass(null);
// | }
