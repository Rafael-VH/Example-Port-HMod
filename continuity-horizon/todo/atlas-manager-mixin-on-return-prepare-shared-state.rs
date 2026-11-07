// Not ported yet: the Java @Inject(RETURN) on AtlasManager.prepareSharedState (AtlasManagerMixin.continuity$onReturnPrepareSharedState).
// Why it's off: No Horizon function mapped for AtlasManager.prepareSharedState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for AtlasManager.prepareSharedState: run `match`.
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
// Decompiled Java, me/pepperbell/continuity/client/mixin/AtlasManagerMixin.java:26:
// | @Inject(method = "prepareSharedState(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;)V", at = @At("RETURN"))
// | private void continuity$onReturnPrepareSharedState(SharedState currentReload, CallbackInfo ci) {
// |    currentReload.set(ModelWrappingHandler.WRAP_EMISSIVE_FUTURE_KEY, new CompletableFuture());
// | }
