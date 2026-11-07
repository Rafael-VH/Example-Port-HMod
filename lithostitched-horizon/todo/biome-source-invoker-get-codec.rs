// Not ported yet: the Java @Invoker on BiomeSource.codec (BiomeSourceInvoker.getCodec).
// Why it's off: No Horizon function mapped for BiomeSource.codec (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for BiomeSource.codec: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BiomeSource.codec -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/BiomeSourceInvoker.java:10:
// | @Invoker("codec")
// | MapCodec<? extends BiomeSource> getCodec();
