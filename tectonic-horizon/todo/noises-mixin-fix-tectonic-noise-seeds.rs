// Not ported yet: the Java @ModifyArg(INVOKE) on Noises.instantiate (NoisesMixin.tectonic$fixTectonicNoiseSeeds).
// Why it's off: No Horizon function mapped for Noises.instantiate (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for Noises.instantiate: run `match`.
// Note: The call it hooks (PositionalRandomFactory.fromHashOf) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Identifier.getNamespace -> (not mapped yet)
//   Identifier.getPath -> (not mapped yet)
//   Identifier.withDefaultNamespace -> (not mapped yet)
//   Noises.instantiate -> (not mapped yet)
//   PositionalRandomFactory.fromHashOf -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/NoisesMixin.java:11:
// | @ModifyArg(
// |    method = "instantiate",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/levelgen/PositionalRandomFactory;fromHashOf(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/util/RandomSource;"
// |    )
// | )
// | private static Identifier tectonic$fixTectonicNoiseSeeds(Identifier name) {
// |    if (name.getNamespace().equals("tectonic")) {
// |       String path = name.getPath();
// |       if (path.startsWith("parameter/")) {
// |          return Identifier.withDefaultNamespace(path.substring(10));
// |       }
// |    }
// |
// |    return name;
// | }
