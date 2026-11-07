// Not ported yet: the Java @Invoker on ServerChunkCache.invokeRunDistanceManagerUpdates (ServerChunkCacheMixin.invokeRunDistanceManagerUpdates).
// Why it's off: No Horizon function mapped for ServerChunkCache.invokeRunDistanceManagerUpdates (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for ServerChunkCache.invokeRunDistanceManagerUpdates: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerChunkCache.getChunkFutureMainThread -> (not mapped yet)
//   ServerChunkCache.invokeBroadcastChangedChunks -> (not mapped yet)
//   ServerChunkCache.invokeRunDistanceManagerUpdates -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/ServerChunkCacheMixin.java:17:
// | @Invoker
// | boolean invokeRunDistanceManagerUpdates();
