// Not ported yet: the Java @Inject(HEAD) on ChunkSystemConverters.convertEntityChunkCompoundTag (MoonriseEntityChunkMixin.graveyard$upgradeAroundModEntities).
// Why it's off: No Horizon function mapped for ChunkSystemConverters.convertEntityChunkCompoundTag (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ChunkSystemConverters.convertEntityChunkCompoundTag: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChunkSystemConverters.convertEntityChunkCompoundTag -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/MoonriseEntityChunkMixin.java:21:
// | @Inject(method = "convertEntityChunkCompoundTag", at = @At("HEAD"), cancellable = true, remap = false, require = 0)
// | private static void graveyard$upgradeAroundModEntities(CompoundTag tag, ServerLevel level, CallbackInfoReturnable<CompoundTag> cir) {
// |    CompoundTag result = EntityChunkUpgrade.upgradeWithoutGraveyardEntities(tag, -1, copy -> convertEntityChunkCompoundTag(copy, level));
// |    if (result != null) {
// |       cir.setReturnValue(result);
// |    }
// | }
