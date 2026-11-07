// Not ported yet: the Java @Invoker on ChunkMap.getVisibleChunkIfPresent (ChunkMapMixin.invokeGetVisibleChunkIfPresent).
// Why it's off: No Horizon function mapped for ChunkMap.getVisibleChunkIfPresent (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for ChunkMap.getVisibleChunkIfPresent: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChunkMap.getVisibleChunkIfPresent -> (not mapped yet)
//   ChunkMap.invokeTick -> (not mapped yet)
//   ChunkMap.mainThreadExecutor -> (not mapped yet)
//   ChunkMap.readChunk -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/ChunkMapMixin.java:17:
// | @Invoker("getVisibleChunkIfPresent")
// | ChunkHolder invokeGetVisibleChunkIfPresent(long var1);
