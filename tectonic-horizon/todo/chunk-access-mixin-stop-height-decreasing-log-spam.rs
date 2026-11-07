// Not ported yet: the Java @Inject(HEAD) on ChunkAccess.getOrCreateOffsetList (ChunkAccessMixin.tectonic$stopHeightDecreasingLogSpam).
// Why it's off: No Horizon function mapped for ChunkAccess.getOrCreateOffsetList (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ChunkAccess.getOrCreateOffsetList: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChunkAccess.getOrCreateOffsetList -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/ChunkAccessMixin.java:13:
// | @Inject(method = "getOrCreateOffsetList", at = @At("HEAD"), cancellable = true)
// | private static void tectonic$stopHeightDecreasingLogSpam(ShortList[] positions, int index, CallbackInfoReturnable<ShortList> cir) {
// |    if (index >= positions.length) {
// |       cir.setReturnValue(new ShortArrayList());
// |    }
// | }
