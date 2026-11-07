// Not ported yet: the Java @Invoker on ServerChunkCache.invokeBroadcastChangedChunks (ServerChunkCacheMixin.invokeBroadcastChangedChunks).
// Why it's off: No Horizon function mapped for ServerChunkCache.invokeBroadcastChangedChunks (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for ServerChunkCache.invokeBroadcastChangedChunks: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerChunkCache.getChunkFutureMainThread -> (not mapped yet)
//   ServerChunkCache.invokeBroadcastChangedChunks -> (not mapped yet)
//   ServerChunkCache.invokeRunDistanceManagerUpdates -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/ServerChunkCacheMixin.java:20:
// | @Invoker
// | void invokeBroadcastChangedChunks(ProfilerFiller var1);
