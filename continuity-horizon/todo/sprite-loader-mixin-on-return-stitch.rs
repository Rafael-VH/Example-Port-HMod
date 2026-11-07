// Not ported yet: the Java @Inject(RETURN) on SpriteLoader.stitch (SpriteLoaderMixin.continuity$onReturnStitch).
// Why it's off: No Horizon function mapped for SpriteLoader.stitch (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for SpriteLoader.stitch: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CompletableFuture.supplyAsync -> (not mapped yet)
//   CompletableFuture.thenApply -> (not mapped yet)
//   SpriteLoader.loadAndStitch -> (not mapped yet)
//   SpriteLoader.stitch -> (not mapped yet)
//   Preparations.regions -> (not mapped yet)
//
// Decompiled Java, me/pepperbell/continuity/client/mixin/SpriteLoaderMixin.java:126:
// | @Inject(
// |    method = "stitch(Ljava/util/List;ILjava/util/concurrent/Executor;)Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;",
// |    at = @At("RETURN")
// | )
// | private void continuity$onReturnStitch(
// |    List<SpriteContents> spriteContentsList, int mipmapLevels, Executor executor, CallbackInfoReturnable<Preparations> cir
// | ) {
// |    SpriteLoaderStitchContext context = SpriteLoaderStitchContext.THREAD_LOCAL.get();
// |    if (context != null) {
// |       Map<Identifier, Identifier> emissiveIdMap = context.getEmissiveIdMap();
// |       Map<Identifier, TextureAtlasSprite> sprites = ((Preparations)cir.getReturnValue()).regions();
// |       MutableBoolean hasEmissives = new MutableBoolean(false);
// |       emissiveIdMap.forEach((id, emissiveId) -> {
// |          TextureAtlasSprite sprite = sprites.get(id);
// |          if (sprite != null) {
// |             TextureAtlasSprite emissiveSprite = sprites.get(emissiveId);
// |             if (emissiveSprite != null) {
// |                ((TextureAtlasSpriteExtension)sprite).continuity$setEmissiveSprite(emissiveSprite);
// |                hasEmissives.setTrue();
// |             }
// |          }
// |       });
// |       context.setHasEmissives(hasEmissives.booleanValue());
// |    }
// | }
