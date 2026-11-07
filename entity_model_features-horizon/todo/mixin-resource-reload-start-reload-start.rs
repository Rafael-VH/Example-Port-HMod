// Not ported yet: the Java @Inject(HEAD) on Minecraft.reloadResourcePacks (MixinResourceReloadStart.emf$reloadStart).
// Why it's off: No Horizon function mapped for Minecraft.reloadResourcePacks (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.reloadResourcePacks: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.reloadResourcePacks -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinResourceReloadStart.java:13:
// | @Inject(method = "reloadResourcePacks(ZLnet/minecraft/client/GameLoadCookie;)Ljava/util/concurrent/CompletableFuture;", at = @At("HEAD"))
// | private void emf$reloadStart(CallbackInfoReturnable<Float> cir) {
// |    EMF.isLoadingPhase = true;
// |    if (!EMF.testForForgeLoadingError()) {
// |       EMFManager.resetInstance();
// |    }
// | }
