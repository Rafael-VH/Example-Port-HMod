// Not ported yet: the Java @Inject(INVOKE) on AtlasManager.reload (AtlasManagerMixin.continuity$onHeadReload).
// Why it's off: No Horizon function mapped for AtlasManager.reload (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for AtlasManager.reload: run `match`.
// Note: The call it hooks (SharedState.resourceManager) isn't mapped to a Rust function yet.
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
// Decompiled Java, me/pepperbell/continuity/client/mixin/AtlasManagerMixin.java:31:
// | @Inject(
// |    method = "reload(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;resourceManager()Lnet/minecraft/server/packs/resources/ResourceManager;"
// |    )
// | )
// | private void continuity$onHeadReload(
// |    SharedState currentReload,
// |    Executor prepareExecutor,
// |    PreparationBarrier preparationBarrier,
// |    Executor applyExecutor,
// |    CallbackInfoReturnable<CompletableFuture<Void>> cir,
// |    @Local PendingStitchResults stitchResults
// | ) {
// |    EmissiveSuffixLoader.load(currentReload.resourceManager());
// |    CompletableFuture<Set<Identifier>> blockAtlasExtraIdsFuture = (CompletableFuture<Set<Identifier>>)currentReload.get(
// |       CtmResourceReloader.BLOCK_ATLAS_EXTRA_IDS_FUTURE_KEY
// |    );
// |    CompletableFuture<Boolean> wrapEmissiveFuture = (CompletableFuture<Boolean>)currentReload.get(ModelWrappingHandler.WRAP_EMISSIVE_FUTURE_KEY);
// |    stitchResults.get(AtlasIds.BLOCKS).whenComplete((preparations, t) -> wrapEmissiveFuture.complete(false));
// |    SpriteLoaderLoadContext.THREAD_LOCAL.set(new SpriteLoaderLoadContextImpl(blockAtlasExtraIdsFuture, wrapEmissiveFuture));
// | }
