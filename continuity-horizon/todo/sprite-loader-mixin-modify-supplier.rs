// Not ported yet: the Java @ModifyArg(INVOKE) on SpriteLoader.loadAndStitch (SpriteLoaderMixin.continuity$modifySupplier).
// Why it's off: No Horizon function mapped for SpriteLoader.loadAndStitch (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for SpriteLoader.loadAndStitch: run `match`.
// Note: The call it hooks (CompletableFuture.supplyAsync) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CompletableFuture.supplyAsync -> (not mapped yet)
//   CompletableFuture.thenApply -> (not mapped yet)
//   SpriteLoader.loadAndStitch -> (not mapped yet)
//   SpriteLoader.stitch -> (not mapped yet)
//   Preparations.regions -> (not mapped yet)
//
// Decompiled Java, me/pepperbell/continuity/client/mixin/SpriteLoaderMixin.java:36:
// | @ModifyArg(
// |    method = "loadAndStitch(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/resources/Identifier;ILjava/util/concurrent/Executor;Ljava/util/Set;)Ljava/util/concurrent/CompletableFuture;",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Ljava/util/concurrent/CompletableFuture;supplyAsync(Ljava/util/function/Supplier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;",
// |       ordinal = 0
// |    ),
// |    index = 0
// | )
// | private Supplier<List<Function<SpriteResourceLoader, SpriteContents>>> continuity$modifySupplier(
// |    Supplier<List<Function<SpriteResourceLoader, SpriteContents>>> supplier
// | ) {
// |    SpriteLoaderLoadContext context = SpriteLoaderLoadContext.THREAD_LOCAL.get();
// |    if (context != null) {
// |       CompletableFuture<Set<Identifier>> extraIdsFuture = context.getExtraIdsFuture(this.location);
// |       SpriteLoaderLoadContext.EmissiveControl emissiveControl = context.getEmissiveControl(this.location);
// |       if (extraIdsFuture != null && emissiveControl != null) {
// |          return () -> {
// |             SpriteSourceListInitContext.THREAD_LOCAL.set(extraIdsFuture::join);
// |             SpriteSourceListListContext.THREAD_LOCAL.set(emissiveControl::setEmissiveIdMap);
// |             List<Function<SpriteResourceLoader, SpriteContents>> list = supplier.get();
// |             SpriteSourceListInitContext.THREAD_LOCAL.remove();
// |             SpriteSourceListListContext.THREAD_LOCAL.remove();
// |             return list;
// |          };
// |       }
// |
// |       if (extraIdsFuture != null) {
// |          return () -> {
// |             SpriteSourceListInitContext.THREAD_LOCAL.set(extraIdsFuture::join);
// |             List<Function<SpriteResourceLoader, SpriteContents>> list = supplier.get();
// |             SpriteSourceListInitContext.THREAD_LOCAL.remove();
// |             return list;
// |          };
// |       }
// |
// |       if (emissiveControl != null) {
// |          return () -> {
// |             SpriteSourceListListContext.THREAD_LOCAL.set(emissiveControl::setEmissiveIdMap);
// |             List<Function<SpriteResourceLoader, SpriteContents>> list = supplier.get();
// |             SpriteSourceListListContext.THREAD_LOCAL.remove();
// |             return list;
// |          };
// |       }
// |    }
// |
// |    return supplier;
// | }
