// Not ported yet: the Java @ModifyArg(INVOKE) on ChunkHolder.updateFutures (ChunkHolderMixin.boostUnloadedPriority).
// Why it's off: No Horizon function mapped for ChunkHolder.updateFutures (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for ChunkHolder.updateFutures: run `match`.
// Note: The call it hooks (LevelChangeListener.onLevelChange) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChunkHolder.updateFutures -> (not mapped yet)
//   LevelChangeListener.onLevelChange -> (not mapped yet)
//   ChunkLevel.MAX_LEVEL -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/ChunkHolderMixin.java:15:
// | @ModifyArg(
// |    method = "updateFutures",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/server/level/ChunkHolder$LevelChangeListener;onLevelChange(Lnet/minecraft/world/level/ChunkPos;Ljava/util/function/IntSupplier;ILjava/util/function/IntConsumer;)V"
// |    ),
// |    index = 2,
// |    require = 0
// | )
// | private int boostUnloadedPriority(int newLevel) {
// |    if (newLevel > ChunkLevel.MAX_LEVEL) {
// |       newLevel = 16;
// |    }
// |
// |    return newLevel;
// | }
