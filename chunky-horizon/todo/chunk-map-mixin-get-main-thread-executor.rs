// Not ported yet: the Java @Accessor on ChunkMap.mainThreadExecutor (ChunkMapMixin.getMainThreadExecutor).
// Why it's off: No Horizon function mapped for ChunkMap.mainThreadExecutor (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for ChunkMap.mainThreadExecutor: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChunkMap.getVisibleChunkIfPresent -> (not mapped yet)
//   ChunkMap.invokeTick -> (not mapped yet)
//   ChunkMap.mainThreadExecutor -> (not mapped yet)
//   ChunkMap.readChunk -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/ChunkMapMixin.java:26:
// | @Accessor
// | BlockableEventLoop<Runnable> getMainThreadExecutor();
