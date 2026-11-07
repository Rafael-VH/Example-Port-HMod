// Not ported yet: the Java @Inject(TAIL) on EnvironmentAttributeSystem.addDynamicLayers (EnvironmentAttributeSystemMixin.addBiomeTimelines).
// Why it's off: No Horizon function mapped for EnvironmentAttributeSystem.addDynamicLayers (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for EnvironmentAttributeSystem.addDynamicLayers: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.value -> (not mapped yet)
//   Reference.value -> (not mapped yet)
//   HolderSet.iterator -> (not mapped yet)
//   HolderSet.stream -> (not mapped yet)
//   IdMap.iterator -> (not mapped yet)
//   Registry.asHolderIdMap -> (not mapped yet)
//   Registry.listElements -> (not mapped yet)
//   RegistryAccess.lookupOrThrow -> (not mapped yet)
//   Registries.BIOME -> (not mapped yet)
//   AttributeType.spatialLerp -> (not mapped yet)
//   EnvironmentAttribute.isSpatiallyInterpolated -> (not mapped yet)
//   EnvironmentAttribute.type -> (not mapped yet)
//   TimeBased.applyTimeBased -> (not mapped yet)
//   EnvironmentAttributeSystem.addDynamicLayers -> (not mapped yet)
//   LerpFunction.apply -> (not mapped yet)
//   Level.clockManager -> (not mapped yet)
//   Level.getBiomeManager -> (not mapped yet)
//   Level.registryAccess -> (not mapped yet)
//   BiomeManager.getNoiseBiomeAtPosition -> (not mapped yet)
//   Vec3.x -> (not mapped yet)
//   Vec3.y -> (not mapped yet)
//   Vec3.z -> (not mapped yet)
//   Timeline.attributes -> (not mapped yet)
//   Timeline.createTrackSampler -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/timeline/EnvironmentAttributeSystemMixin.java:37:
// | @Inject(method = "addDynamicLayers", at = @At("TAIL"))
// | private static void addBiomeTimelines(Builder builder, Level level, CallbackInfo ci) {
// |    Registry<Biome> biomes = level.registryAccess().lookupOrThrow(Registries.BIOME);
// |    BiomeManager biomeManager = level.getBiomeManager();
// |    Stream<EnvironmentAttribute<?>> attributesProvidedByBiomeTimelines = biomes.listElements()
// |       .flatMap(
// |          biome -> BiomeTimelineDuck.cast((Biome)biome.value())
// |             .lithostitched$getTimelines()
// |             .stream()
// |             .flatMap(timeline -> ((Timeline)timeline.value()).attributes().stream())
// |       )
// |       .distinct();
// |    attributesProvidedByBiomeTimelines.forEach(attribute -> addBiomeTimelineLayerForAttribute(builder, attribute, biomes, biomeManager, level.clockManager()));
// | }
