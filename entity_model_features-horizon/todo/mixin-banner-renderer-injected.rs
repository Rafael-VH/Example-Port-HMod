// Not ported yet: the Java @Inject(HEAD) on BannerRenderer.submitPatterns (MixinBannerRenderer.emf$injected).
// Why it's off: No Horizon function mapped for BannerRenderer.submitPatterns (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for BannerRenderer.submitPatterns: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BannerRenderer.submitPatterns -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinBannerRenderer.java:12:
// | @Inject(method = "submitPatterns", at = @At("HEAD"))
// | private static void emf$injected(CallbackInfo ci) {
// |    EMFManager.getInstance().entityRenderCount++;
// | }
