// Not ported yet: the Java @ModifyReturnValue(RETURN) on VillagerType.byBiome (VillagerTypeMixin.useBiomeTags).
// Why it's off: No Horizon function mapped for VillagerType.byBiome (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for VillagerType.byBiome: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DefaultedRegistry.registryKeySet -> (not mapped yet)
//   Holder.is -> (not mapped yet)
//   BuiltInRegistries.VILLAGER_TYPE -> (not mapped yet)
//   Registries.VILLAGER_TYPE -> (not mapped yet)
//   VillagerType.byBiome -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/villager/VillagerTypeMixin.java:27:
// | @ModifyReturnValue(method = "byBiome", at = @At("RETURN"))
// | private static ResourceKey<VillagerType> useBiomeTags(ResourceKey<VillagerType> originalType, @Local(argsOnly = true, ordinal = 0) Holder<Biome> biome) {
// |    for (ResourceKey<VillagerType> type : BuiltInRegistries.VILLAGER_TYPE.registryKeySet()) {
// |       TagKey<Biome> biomeTag = TYPE_TO_TAG_CACHE.computeIfAbsent(type, __ -> LithostitchedBiomeTags.createVillagerTypeTag(type));
// |       if (biome.is(biomeTag)) {
// |          return type;
// |       }
// |    }
// |
// |    return originalType;
// | }
