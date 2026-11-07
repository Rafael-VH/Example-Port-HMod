// Not ported yet: the Java @WrapOperation(INVOKE) on ChunkMap.<init> (ChunkMapMixin.wrapNoiseRouter).
// Why it's off: No Horizon function mapped for ChunkMap.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for ChunkMap.<init>: run `match`.
// Note: The call it hooks (RandomState.create) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChunkMap.<init> -> (not mapped yet)
//   ServerLevel.dimension -> (not mapped yet)
//   Config.<init> -> (not mapped yet)
//   Config.barrierNoise -> (not mapped yet)
//   Config.exclusion -> (not mapped yet)
//   Config.fluidLevelFloodednessNoise -> (not mapped yet)
//   Config.fluidLevelSpreadNoise -> (not mapped yet)
//   Config.lavaNoise -> (not mapped yet)
//   Config.surfaceLevel -> (not mapped yet)
//   NoiseGeneratorSettings.aquifers -> (not mapped yet)
//   NoiseGeneratorSettings.noiseRouter -> (not mapped yet)
//   NoiseRouter.<init> -> (not mapped yet)
//   NoiseRouter.chunkSurfaceLevel -> (not mapped yet)
//   NoiseRouter.continents -> (not mapped yet)
//   NoiseRouter.depth -> (not mapped yet)
//   NoiseRouter.erosion -> (not mapped yet)
//   NoiseRouter.finalDensity -> (not mapped yet)
//   NoiseRouter.ridges -> (not mapped yet)
//   NoiseRouter.temperature -> (not mapped yet)
//   NoiseRouter.vegetation -> (not mapped yet)
//   RandomState.create -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/ChunkMapMixin.java:28:
// | @WrapOperation(
// |    method = "<init>",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/levelgen/RandomState;create(Lnet/minecraft/core/HolderGetter;JLnet/minecraft/world/level/levelgen/NoiseGeneratorSettings;)Lnet/minecraft/world/level/levelgen/RandomState;"
// |    )
// | )
// | private RandomState wrapNoiseRouter(
// |    HolderGetter<NormalNoise> noises,
// |    long seed,
// |    NoiseGeneratorSettings settings,
// |    Operation<RandomState> init,
// |    ServerLevel level,
// |    @Local(name = "registryAccess") RegistryAccess registryAccess
// | ) {
// |    NoiseGeneratorSettingsAccessor accessor = (NoiseGeneratorSettingsAccessor)settings;
// |    NoiseRouter router = settings.noiseRouter();
// |    List<WrapNoiseRouterModifier> routerModifiers = ModifierManager.getModifiersOfType(registryAccess, WrapNoiseRouterModifier.CODEC)
// |       .stream()
// |       .map(Entry::getValue)
// |       .filter(modifier -> level.dimension().equals(modifier.dimension()))
// |       .toList();
// |    if (!routerModifiers.isEmpty()) {
// |       accessor.setNoiseRouter(
// |          new NoiseRouter(
// |             WrapNoiseRouterModifier.modifyRouterFunction(NoiseRouterTarget.TEMPERATURE, router.temperature(), routerModifiers),
// |             WrapNoiseRouterModifier.modifyRouterFunction(NoiseRouterTarget.VEGETATION, router.vegetation(), routerModifiers),
// |             WrapNoiseRouterModifier.modifyRouterFunction(NoiseRouterTarget.CONTINENTS, router.continents(), routerModifiers),
// |             WrapNoiseRouterModifier.modifyRouterFunction(NoiseRouterTarget.EROSION, router.erosion(), routerModifiers),
// |             WrapNoiseRouterModifier.modifyRouterFunction(NoiseRouterTarget.DEPTH, router.depth(), routerModifiers),
// |             WrapNoiseRouterModifier.modifyRouterFunction(NoiseRouterTarget.RIDGES, router.ridges(), routerModifiers),
// |             WrapNoiseRouterModifier.modifyRouterFunction(NoiseRouterTarget.CHUNK_SURFACE_LEVEL, router.chunkSurfaceLevel(), routerModifiers),
// |             WrapNoiseRouterModifier.modifyRouterFunction(NoiseRouterTarget.FINAL_DENSITY, router.finalDensity(), routerModifiers)
// |          )
// |       );
// |    }
// |
// |    Config aquifers = (Config)settings.aquifers().orElse(null);
// |    if (aquifers != null) {
// |       List<WrapAquifersModifier> aquiferModifiers = ModifierManager.getModifiersOfType(registryAccess, WrapAquifersModifier.CODEC)
// |          .stream()
// |          .map(Entry::getValue)
// |          .filter(modifier -> level.dimension().equals(modifier.dimension()))
// |          .toList();
// |       if (!aquiferModifiers.isEmpty()) {
// |          accessor.setAquifers(
// |             Optional.of(
// |                new Config(
// |                   WrapAquifersModifier.modifyAquiferFunction(AquiferTarget.BARRIER, aquifers.barrierNoise(), aquiferModifiers),
// |                   WrapAquifersModifier.modifyAquiferFunction(AquiferTarget.FLUID_LEVEL_FLOODEDNESS, aquifers.fluidLevelFloodednessNoise(), aquiferModifiers),
// |                   WrapAquifersModifier.modifyAquiferFunction(AquiferTarget.FLUID_LEVEL_SPREAD, aquifers.fluidLevelSpreadNoise(), aquiferModifiers),
// |                   WrapAquifersModifier.modifyAquiferFunction(AquiferTarget.LAVA, aquifers.lavaNoise(), aquiferModifiers),
// |                   WrapAquifersModifier.modifyAquiferFunction(AquiferTarget.EXCLUSION, aquifers.exclusion(), aquiferModifiers),
// |                   WrapAquifersModifier.modifyAquiferFunction(AquiferTarget.SURFACE_LEVEL, aquifers.surfaceLevel(), aquiferModifiers)
// |                )
// |             )
// |          );
// |       }
// |    }
// |
// |    return (RandomState)init.call(new Object[]{noises, seed, settings});
// | }
