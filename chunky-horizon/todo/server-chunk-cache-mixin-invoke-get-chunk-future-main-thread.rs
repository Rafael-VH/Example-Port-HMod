// Not ported yet: the Java @Invoker on ServerChunkCache.getChunkFutureMainThread (ServerChunkCacheMixin.invokeGetChunkFutureMainThread).
// Why it's off: No Horizon function mapped for ServerChunkCache.getChunkFutureMainThread (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for ServerChunkCache.getChunkFutureMainThread: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerChunkCache.getChunkFutureMainThread -> (not mapped yet)
//   ServerChunkCache.invokeBroadcastChangedChunks -> (not mapped yet)
//   ServerChunkCache.invokeRunDistanceManagerUpdates -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/ServerChunkCacheMixin.java:14:
// | @Invoker("getChunkFutureMainThread")
// | CompletableFuture<ChunkResult<ChunkAccess>> invokeGetChunkFutureMainThread(int var1, int var2, ChunkStatus var3, boolean var4);
