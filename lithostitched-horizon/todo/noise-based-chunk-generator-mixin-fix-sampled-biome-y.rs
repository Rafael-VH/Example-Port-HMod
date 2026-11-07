// Not ported yet: the Java @WrapOperation(INVOKE) on NoiseBasedChunkGenerator.spawnOriginalMobs (NoiseBasedChunkGeneratorMixin.fixSampledBiomeY).
// Why it's off: No Horizon function mapped for NoiseBasedChunkGenerator.spawnOriginalMobs (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for NoiseBasedChunkGenerator.spawnOriginalMobs: run `match`.
// Note: The call it hooks (BlockPos.atY) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.atY -> (not mapped yet)
//   BlockPos.getX -> (not mapped yet)
//   BlockPos.getZ -> (not mapped yet)
//   Reference.value -> (not mapped yet)
//   RegistryLookup.listElements -> (not mapped yet)
//   RegistryAccess.lookupOrThrow -> (not mapped yet)
//   Identifier.withDefaultNamespace -> (not mapped yet)
//   WorldGenRegion.getChunk -> (not mapped yet)
//   WorldGenRegion.registryAccess -> (not mapped yet)
//   ChunkAccess.getHeight -> (not mapped yet)
//   Types.MOTION_BLOCKING_NO_LEAVES -> (not mapped yet)
//   NoiseBasedChunkGenerator.buildTerrain -> (not mapped yet)
//   NoiseBasedChunkGenerator.spawnOriginalMobs -> (not mapped yet)
//   PositionalRandomFactory.fromHashOf -> (not mapped yet)
//   RandomState.surfaceSystem -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/NoiseBasedChunkGeneratorMixin.java:32:
// | @WrapOperation(method = "spawnOriginalMobs", at = @At(value = "INVOKE", target = "Lnet/minecraft/core/BlockPos;atY(I)Lnet/minecraft/core/BlockPos;"))
// | private BlockPos fixSampledBiomeY(BlockPos pos, int y, Operation<BlockPos> original, @Local(argsOnly = true, ordinal = 0) WorldGenRegion worldGenRegion) {
// |    return pos.atY(worldGenRegion.getChunk(pos).getHeight(Types.MOTION_BLOCKING_NO_LEAVES, pos.getX(), pos.getZ()));
// | }
