// Not ported yet: the Java @ModifyArg(INVOKE) on SpriteLoader.loadAndStitch (SpriteLoaderMixin.continuity$modifyFunction).
// Why it's off: No Horizon function mapped for SpriteLoader.loadAndStitch (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for SpriteLoader.loadAndStitch: run `match`.
// Note: The call it hooks (CompletableFuture.thenApply) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CompletableFuture.supplyAsync -> (not mapped yet)
//   CompletableFuture.thenApply -> (not mapped yet)
//   SpriteLoader.loadAndStitch -> (not mapped yet)
//   SpriteLoader.stitch -> (not mapped yet)
//   Preparations.regions -> (not mapped yet)
//
// Decompiled Java, me/pepperbell/continuity/client/mixin/SpriteLoaderMixin.java:85:
// | @ModifyArg(
// |    method = "loadAndStitch(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/resources/Identifier;ILjava/util/concurrent/Executor;Ljava/util/Set;)Ljava/util/concurrent/CompletableFuture;",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Ljava/util/concurrent/CompletableFuture;thenApply(Ljava/util/function/Function;)Ljava/util/concurrent/CompletableFuture;",
// |       ordinal = 0
// |    ),
// |    index = 0
// | )
// | private Function<List<SpriteContents>, Preparations> continuity$modifyFunction(Function<List<SpriteContents>, Preparations> function) {
// |    SpriteLoaderLoadContext context = SpriteLoaderLoadContext.THREAD_LOCAL.get();
// |    if (context != null) {
// |       SpriteLoaderLoadContext.EmissiveControl emissiveControl = context.getEmissiveControl(this.location);
// |       if (emissiveControl != null) {
// |          return spriteContentsList -> {
// |             final Map<Identifier, Identifier> emissiveIdMap = emissiveControl.getEmissiveIdMap();
// |             if (emissiveIdMap != null) {
// |                SpriteLoaderStitchContext.THREAD_LOCAL.set(new SpriteLoaderStitchContext() {
// |                   @Override
// |                   public Map<Identifier, Identifier> getEmissiveIdMap() {
// |                      return emissiveIdMap;
// |                   }
// |
// |                   @Override
// |                   public void setHasEmissives(boolean hasEmissives) {
// |                      emissiveControl.setHasEmissives(hasEmissives);
// |                   }
// |                });
// |                Preparations result = function.apply(spriteContentsList);
// |                SpriteLoaderStitchContext.THREAD_LOCAL.remove();
// |                return result;
// |             } else {
// |                return function.apply(spriteContentsList);
// |             }
// |          };
// |       }
// |    }
// |
// |    return function;
// | }
