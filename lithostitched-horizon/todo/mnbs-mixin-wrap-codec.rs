// Not ported yet: the Java @Redirect(INVOKE) on MultiNoiseBiomeSource.<clinit> (MNBSMixin.wrapCodec).
// Why it's off: No Horizon function mapped for MultiNoiseBiomeSource.<clinit> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: A redirect is a WRAP_OPERATION that doesn't call `original`.
// Note: No Horizon function known for MultiNoiseBiomeSource.<clinit>: run `match`.
// Note: The call it hooks (MapCodec.xmap) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MapCodec.xmap -> (not mapped yet)
//   Holder.is -> (not mapped yet)
//   Reference.isBound -> (not mapped yet)
//   Reference.value -> (not mapped yet)
//   HolderGetter.get -> (not mapped yet)
//   Registries.MULTI_NOISE_BIOME_SOURCE_PARAMETER_LIST -> (not mapped yet)
//   RegistryOps.retrieveGetter -> (not mapped yet)
//   ParameterList.values -> (not mapped yet)
//   MultiNoiseBiomeSource.<clinit> -> (not mapped yet)
//   MultiNoiseBiomeSourceParameterLists.OVERWORLD -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/mnbs/MNBSMixin.java:46:
// | @Redirect(
// |    method = "<clinit>",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lcom/mojang/serialization/MapCodec;xmap(Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;"
// |    )
// | )
// | private static MapCodec<MultiNoiseBiomeSource> wrapCodec(
// |    MapCodec<Either<ParameterList<Holder<Biome>>, Holder<MultiNoiseBiomeSourceParameterList>>> original,
// |    Function<? super Either<ParameterList<Holder<Biome>>, Holder<MultiNoiseBiomeSourceParameterList>>, ? extends MultiNoiseBiomeSource> to,
// |    Function<? super MultiNoiseBiomeSource, ? extends Either<ParameterList<Holder<Biome>>, Holder<MultiNoiseBiomeSourceParameterList>>> from
// | ) {
// |    return CodecExtender.extend(
// |       original.xmap(to, from),
// |       (instance, wrapper) -> instance.group(wrapper, RegistryOps.retrieveGetter(Registries.MULTI_NOISE_BIOME_SOURCE_PARAMETER_LIST))
// |          .apply(instance, (mnbs, lookup) -> {
// |             MNBSDuck duck = (MNBSDuck)mnbs;
// |             Either<ParameterList<Holder<Biome>>, Holder<MultiNoiseBiomeSourceParameterList>> biomeEntries = duck.lithostitched$getEntries();
// |             if (biomeEntries.left().isPresent()) {
// |                List<Pair<ParameterPoint, Holder<Biome>>> rawEntries = ((ParameterList)biomeEntries.left().get()).values();
// |                Optional<Reference<MultiNoiseBiomeSourceParameterList>> overworldPreset = lookup.get(MultiNoiseBiomeSourceParameterLists.OVERWORLD);
// |                if (overworldPreset.isEmpty() || !overworldPreset.get().isBound()) {
// |                   return mnbs;
// |                }
// |
// |                Optional<Holder<Biome>> migrationBiome = ((MNBSPLDuck)overworldPreset.get().value()).lithostitched$getMigrationBiome();
// |                if (migrationBiome.isPresent() && ((Holder)rawEntries.getLast().getSecond()).is(migrationBiome.get())) {
// |                   duck.lithostitched$setEntries(Either.right(overworldPreset.get()));
// |                }
// |             }
// |
// |             return mnbs;
// |          })
// |    );
// | }
