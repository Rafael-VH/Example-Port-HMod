// Not ported yet: the Java @Inject(RETURN) on BlockStateModelLoader.lambda$loadBlockStates$0 (BlockStateModelLoaderMixin.be_switchModelOnLoad).
// Why it's off: No Horizon function mapped for BlockStateModelLoader.lambda$loadBlockStates$0 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for BlockStateModelLoader.lambda$loadBlockStates$0: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockStateModelLoader.lambda$loadBlockStates$0 -> (not mapped yet)
//   FileToIdConverter.idToFile -> (not mapped yet)
//   FileToIdConverter.json -> (not mapped yet)
//   Identifier.withDefaultNamespace -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/client/BlockStateModelLoaderMixin.java:24:
// | @Inject(method = "lambda$loadBlockStates$0", at = @At("RETURN"), cancellable = true)
// | private static void be_switchModelOnLoad(ResourceManager resourceManager, CallbackInfoReturnable<Map<Identifier, List<Resource>>> cir) {
// |    if (GeneratorOptions.changeChorusPlant()) {
// |       Map<Identifier, List<Resource>> resources = new HashMap<>((Map<? extends Identifier, ? extends List<Resource>>)cir.getReturnValue());
// |       be_replaceDefinition(resources, "chorus_plant");
// |       be_replaceDefinition(resources, "chorus_flower");
// |       cir.setReturnValue(resources);
// |    }
// | }
