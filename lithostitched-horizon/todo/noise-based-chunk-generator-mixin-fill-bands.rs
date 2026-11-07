// Not ported yet: the Java @Inject(HEAD) on NoiseBasedChunkGenerator.buildTerrain (NoiseBasedChunkGeneratorMixin.fillBands).
// Why it's off: No Horizon function mapped for NoiseBasedChunkGenerator.buildTerrain (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for NoiseBasedChunkGenerator.buildTerrain: run `match`.
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
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/NoiseBasedChunkGeneratorMixin.java:37:
// | @Inject(method = "buildTerrain", at = @At("HEAD"))
// | private void fillBands(
// |    ChunkAccess chunk,
// |    Blender blender,
// |    RandomState randomState,
// |    StructureManager structureManager,
// |    BiomeManager biomeManager,
// |    @Nullable WorldGenRegion carverBiomeRegion,
// |    Set<Holder<Biome>> possibleBiomes,
// |    CallbackInfoReturnable<CompletableFuture<ChunkAccess>> cir
// | ) {
// |    RegistryLookup<Bandlands> registry = carverBiomeRegion.registryAccess().lookupOrThrow(LithostitchedRegistries.BANDLANDS);
// |    registry.listElements()
// |       .forEach(
// |          holder -> ((Bandlands)holder.value())
// |             .fillBands(((SurfaceSystemAccessor)randomState.surfaceSystem()).getNoiseRandom().fromHashOf(Identifier.withDefaultNamespace("clay_bands")))
// |       );
// | }
