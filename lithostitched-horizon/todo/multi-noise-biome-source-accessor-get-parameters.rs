// Not ported yet: the Java @Accessor on MultiNoiseBiomeSource.parameters (MultiNoiseBiomeSourceAccessor.getParameters).
// Why it's off: No Horizon function mapped for MultiNoiseBiomeSource.parameters (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for MultiNoiseBiomeSource.parameters: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MultiNoiseBiomeSource.parameters -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/MultiNoiseBiomeSourceAccessor.java:14:
// | @Accessor("parameters")
// | Either<ParameterList<Holder<Biome>>, Holder<MultiNoiseBiomeSourceParameterList>> getParameters();
