// Not ported yet: the Java @Inject(HEAD) on LevelRenderer.executeSolid (LevelRendererMixin.be_captureSolidRenderPass).
// Why it's off: No Horizon function mapped for LevelRenderer.executeSolid (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for LevelRenderer.executeSolid: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelRenderer.executeSolid -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/client/LevelRendererMixin.java:15:
// | @Inject(method = "executeSolid", at = @At("HEAD"))
// | private void be_captureSolidRenderPass(ChunkSectionsToRender chunkSectionsToRender, PreparedFrame featureFrame, RenderPass renderPass, CallbackInfo ci) {
// |    BetterEndSkyRenderer.setActiveSolidRenderPass(renderPass);
// | }
