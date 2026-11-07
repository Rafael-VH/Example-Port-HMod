// Not ported yet: the Java @WrapWithCondition(INVOKE) on ChunkMap.onLevelChange (ChunkMapPatchMixin.suppressRaceCondition).
// Why it's off: No Horizon function mapped for ChunkMap.onLevelChange (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: Call `original` only when the condition holds.
// Note: No Horizon function known for ChunkMap.onLevelChange: run `match`.
// Note: The call it hooks (ChunkTaskDispatcher.onLevelChange) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChunkMap.onLevelChange -> (not mapped yet)
//   ChunkTaskDispatcher.onLevelChange -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/ChunkMapPatchMixin.java:20:
// | @WrapWithCondition(
// |    method = "onLevelChange",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/server/level/ChunkTaskDispatcher;onLevelChange(Lnet/minecraft/world/level/ChunkPos;Ljava/util/function/IntSupplier;ILjava/util/function/IntConsumer;)V"
// |    ),
// |    require = 0
// | )
// | private boolean suppressRaceCondition(ChunkTaskDispatcher instance, ChunkPos pos, IntSupplier oldLevel, int newLevel, IntConsumer setQueueLevel) {
// |    return instance == this.worldgenTaskDispatcher;
// | }
