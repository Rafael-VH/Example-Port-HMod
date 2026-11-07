// Not ported yet: the Java @Inject(HEAD) on Minecraft.onResourceLoadFinished (MixinResourceReload.etf$injected).
// Why it's off: No Horizon function mapped for Minecraft.onResourceLoadFinished (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.onResourceLoadFinished: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.onResourceLoadFinished -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/reloading/MixinResourceReload.java:13:
// | @Inject(method = "onResourceLoadFinished", at = @At("HEAD"))
// | private void etf$injected(CallbackInfo ci) {
// |    ETFUtils2.logMessage("reloading ETF data.");
// |    ETFManager.resetInstance();
// | }
