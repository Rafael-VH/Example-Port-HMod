// Not ported yet: the Java @Inject(HEAD) on Minecraft.reloadResourcePacks (MixinResourceReload.emf$reloadStart).
// Why it's off: No Horizon function mapped for Minecraft.reloadResourcePacks (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.reloadResourcePacks: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.reloadResourcePacks -> (not mapped yet)
//
// Decompiled Java, traben/entity_sound_features/mixin/mixins/MixinResourceReload.java:12:
// | @Inject(method = "reloadResourcePacks()Ljava/util/concurrent/CompletableFuture;", at = @At("HEAD"))
// | private void emf$reloadStart(CallbackInfoReturnable<Float> cir) {
// |    ESFSoundContext.resetContext();
// | }
