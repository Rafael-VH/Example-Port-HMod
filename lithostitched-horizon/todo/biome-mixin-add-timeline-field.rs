// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on Biome.<clinit> (BiomeMixin.addTimelineField).
// Why it's off: No Horizon function mapped for Biome.<clinit> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for Biome.<clinit>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HolderSet.direct -> (not mapped yet)
//   HolderSet.stream -> (not mapped yet)
//   Registries.TIMELINE -> (not mapped yet)
//   RegistryCodecs.holderSet -> (not mapped yet)
//   Biome.<clinit> -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/timeline/BiomeMixin.java:41:
// | @ModifyExpressionValue(
// |    method = "<clinit>",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lcom/mojang/serialization/codecs/RecordCodecBuilder;create(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;"
// |    )
// | )
// | private static Codec<Biome> addTimelineField(Codec<Biome> codec) {
// |    return CodecExtender.extend(
// |       codec,
// |       (instance, wrapper) -> instance.group(
// |             wrapper,
// |             RegistryCodecs.holderSet(Registries.TIMELINE)
// |                .lenientOptionalFieldOf("lithostitched:timelines", HolderSet.direct(new Holder[0]))
// |                .forGetter(biome -> BiomeTimelineDuck.cast(biome).lithostitched$getTimelines())
// |          )
// |          .apply(instance, (biome, timelines) -> {
// |             BiomeTimelineDuck.cast(biome).lithostitched$setTimelines(timelines);
// |             return biome;
// |          })
// |    );
// | }
