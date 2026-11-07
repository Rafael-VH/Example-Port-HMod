// Not ported yet: the Java @Invoker on ChunkMap.readChunk (ChunkMapMixin.invokeReadChunk).
// Why it's off: No Horizon function mapped for ChunkMap.readChunk (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for ChunkMap.readChunk: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChunkMap.getVisibleChunkIfPresent -> (not mapped yet)
//   ChunkMap.invokeTick -> (not mapped yet)
//   ChunkMap.mainThreadExecutor -> (not mapped yet)
//   ChunkMap.readChunk -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/ChunkMapMixin.java:20:
// | @Invoker("readChunk")
// | CompletableFuture<Optional<CompoundTag>> invokeReadChunk(ChunkPos var1);
