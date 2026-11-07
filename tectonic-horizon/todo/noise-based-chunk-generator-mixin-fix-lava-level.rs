// Not ported yet: the Java @Inject(RETURN) on NoiseBasedChunkGenerator.<init> (NoiseBasedChunkGeneratorMixin.tectonic$fixLavaLevel).
// Why it's off: No Horizon function mapped for NoiseBasedChunkGenerator.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for NoiseBasedChunkGenerator.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SharedConstants.DEBUG_DISABLE_FLUID_GENERATION -> (not mapped yet)
//   Holder.unwrapKey -> (not mapped yet)
//   Holder.value -> (not mapped yet)
//   Identifier.getPath -> (not mapped yet)
//   ResourceKey.identifier -> (not mapped yet)
//   Block.defaultBlockState -> (not mapped yet)
//   Blocks.AIR -> (not mapped yet)
//   Blocks.LAVA -> (not mapped yet)
//   DimensionType.MIN_Y -> (not mapped yet)
//   FluidStatus.<init> -> (not mapped yet)
//   NoiseBasedChunkGenerator.<init> -> (not mapped yet)
//   NoiseGeneratorSettings.defaultFluid -> (not mapped yet)
//   NoiseGeneratorSettings.seaLevel -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/NoiseBasedChunkGeneratorMixin.java:31:
// | @Inject(method = "<init>", at = @At("RETURN"))
// | private void tectonic$fixLavaLevel(BiomeSource source, Holder<NoiseGeneratorSettings> settings, CallbackInfo ci) {
// |    if (((ConfigState)Tectonic.CONFIG.getState()).enabled() && settings.unwrapKey().map(key -> key.identifier().getPath().equals("overworld")).orElse(false)) {
// |       this.globalFluidPicker = Suppliers.memoize(() -> {
// |          int lavaLevel = ((ConfigState)Tectonic.CONFIG.getState()).globalTerrain.heightLimits.minY + 10;
// |          FluidStatus lavaStatus = new FluidStatus(lavaLevel, Blocks.LAVA.defaultBlockState());
// |          int seaLevel = ((NoiseGeneratorSettings)settings.value()).seaLevel();
// |          FluidStatus seaStatus = new FluidStatus(seaLevel, ((NoiseGeneratorSettings)settings.value()).defaultFluid());
// |          FluidStatus disabledStatus = new FluidStatus(DimensionType.MIN_Y * 2, Blocks.AIR.defaultBlockState());
// |          return (x, y, z) -> {
// |             if (SharedConstants.DEBUG_DISABLE_FLUID_GENERATION) {
// |                return disabledStatus;
// |             } else {
// |                return y < Math.min(lavaLevel, seaLevel) ? lavaStatus : seaStatus;
// |             }
// |          };
// |       });
// |    }
// | }
