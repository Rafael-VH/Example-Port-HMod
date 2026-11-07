// Not ported yet: the Java @Inject(RETURN) on AtlasManager.reload (AtlasManagerMixin.continuity$onReturnReload).
// Why it's off: No Horizon function mapped for AtlasManager.reload (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for AtlasManager.reload: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AtlasManager.prepareSharedState -> (not mapped yet)
//   AtlasManager.reload -> (not mapped yet)
//   PendingStitchResults.get -> (not mapped yet)
//   AtlasIds.BLOCKS -> (not mapped yet)
//   SharedState.get -> (not mapped yet)
//   SharedState.resourceManager -> (not mapped yet)
//   SharedState.set -> (not mapped yet)
//
// Decompiled Java, me/pepperbell/continuity/client/mixin/AtlasManagerMixin.java:55:
// | @Inject(
// |    method = "reload(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;",
// |    at = @At("RETURN")
// | )
// | private void continuity$onReturnReload(CallbackInfoReturnable<CompletableFuture<Void>> cir) {
// |    SpriteLoaderLoadContext.THREAD_LOCAL.remove();
// | }
