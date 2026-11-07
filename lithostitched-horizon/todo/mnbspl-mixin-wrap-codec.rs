// Not ported yet: the Java @Redirect(INVOKE) on MultiNoiseBiomeSourceParameterList.<clinit> (MNBSPLMixin.wrapCodec).
// Why it's off: No Horizon function mapped for MultiNoiseBiomeSourceParameterList.<clinit> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: A redirect is a WRAP_OPERATION that doesn't call `original`.
// Note: No Horizon function known for MultiNoiseBiomeSourceParameterList.<clinit>: run `match`.
// Note: The call it hooks (RecordCodecBuilder.create) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RecordCodecBuilder.create -> (not mapped yet)
//   Biome.CODEC -> (not mapped yet)
//   ParameterList.codec -> (not mapped yet)
//   MultiNoiseBiomeSourceParameterList.<clinit> -> (not mapped yet)
//   MultiNoiseBiomeSourceParameterList.parameters -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/mnbs/MNBSPLMixin.java:48:
// | @Redirect(
// |    method = "<clinit>",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lcom/mojang/serialization/codecs/RecordCodecBuilder;create(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;"
// |    )
// | )
// | private static Codec<MultiNoiseBiomeSourceParameterList> wrapCodec(
// |    Function<Instance<MultiNoiseBiomeSourceParameterList>, ? extends App<Mu<MultiNoiseBiomeSourceParameterList>, MultiNoiseBiomeSourceParameterList>> builder
// | ) {
// |    return CodecExtender.extend(
// |       RecordCodecBuilder.create(builder),
// |       (instance, wrapper) -> instance.group(
// |             wrapper,
// |             ParameterList.codec(Biome.CODEC.fieldOf("biome")).optionalFieldOf("lithostitched:biomes").forGetter(mnbspl -> Optional.of(mnbspl.parameters())),
// |             Biome.CODEC.optionalFieldOf("lithostitched:migration_biome").forGetter(mnbspl -> ((MNBSPLDuck)mnbspl).lithostitched$getMigrationBiome())
// |          )
// |          .apply(instance, (mnbspl, parameters, biome) -> {
// |             MNBSPLDuck duck = (MNBSPLDuck)mnbspl;
// |             parameters.ifPresent(duck::lithostitched$setParameters);
// |             duck.lithostitched$setMigrationBiome(biome);
// |             return mnbspl;
// |          })
// |    );
// | }
