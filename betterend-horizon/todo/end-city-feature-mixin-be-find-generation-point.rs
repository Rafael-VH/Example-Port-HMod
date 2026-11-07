// Not ported yet: the Java @Inject(HEAD) on EndCityStructure.findGenerationPoint (EndCityFeatureMixin.be_findGenerationPoint).
// Why it's off: No Horizon function mapped for EndCityStructure.findGenerationPoint (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EndCityStructure.findGenerationPoint: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   WorldgenRandom.nextInt -> (not mapped yet)
//   GenerationContext.random -> (not mapped yet)
//   EndCityStructure.findGenerationPoint -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/EndCityFeatureMixin.java:15:
// | @Inject(method = "findGenerationPoint", at = @At("HEAD"), cancellable = true)
// | private void be_findGenerationPoint(GenerationContext context, CallbackInfoReturnable<Optional<GenerationStub>> info) {
// |    if (GeneratorOptions.useNewGenerator()) {
// |       int chance = GeneratorOptions.getEndCityFailChance();
// |       if (chance > 0 && context.random().nextInt(chance) != 0) {
// |          info.setReturnValue(Optional.empty());
// |       }
// |    }
// | }
