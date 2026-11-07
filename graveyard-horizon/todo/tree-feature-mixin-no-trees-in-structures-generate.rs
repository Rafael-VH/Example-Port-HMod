// Not ported yet: the Java @Inject(HEAD) on TreeFeature.doPlace (TreeFeatureMixin.noTreesInStructuresGenerate).
// Why it's off: No Horizon function mapped for TreeFeature.doPlace (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for TreeFeature.doPlace: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.below -> (not mapped yet)
//   WorldGenLevel.getBlockState -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   TreeFeature.doPlace -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/TreeFeatureMixin.java:18:
// | @Inject(
// |    method = "doPlace(Lnet/minecraft/world/level/WorldGenLevel;Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/BlockPos;Ljava/util/function/BiConsumer;Ljava/util/function/BiConsumer;Lnet/minecraft/world/level/levelgen/feature/foliageplacers/FoliagePlacer$FoliageSetter;)Z",
// |    at = @At("HEAD"),
// |    cancellable = true
// | )
// | private void noTreesInStructuresGenerate(
// |    WorldGenLevel world,
// |    RandomSource random,
// |    BlockPos pos,
// |    BiConsumer<BlockPos, BlockState> rootPlacerReplacer,
// |    BiConsumer<BlockPos, BlockState> trunkPlacerReplacer,
// |    FoliageSetter blockPlacer,
// |    CallbackInfoReturnable<Boolean> cir
// | ) {
// |    BlockState state = world.getBlockState(pos.below());
// |    if (state.is(TGTags.NATURAL_HELPER_BLOCKS)) {
// |       cir.setReturnValue(false);
// |    }
// | }
