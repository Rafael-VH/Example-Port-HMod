// Not ported yet: the Java @Inject(HEAD) on Minecraft.onResourceLoadFinished (MixinResourceReloadEnd.emf$reloadFinish).
// Why it's off: No Horizon function mapped for Minecraft.onResourceLoadFinished (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.onResourceLoadFinished: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.onResourceLoadFinished -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinResourceReloadEnd.java:13:
// | @Inject(method = "onResourceLoadFinished", at = @At("HEAD"))
// | private void emf$reloadFinish(CallbackInfo ci) {
// |    if (!EMF.testForForgeLoadingError()) {
// |       EMFManager.getInstance().modifyEBEIfRequired();
// |       EMFManager.getInstance().reloadEnd();
// |       EMF.isLoadingPhase = false;
// |    }
// | }
