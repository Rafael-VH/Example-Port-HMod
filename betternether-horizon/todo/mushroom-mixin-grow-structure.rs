// Not ported yet: the Java @Inject(HEAD) on MushroomBlock.performBonemeal (MushroomMixin.growStructure).
// Why it's off: No Horizon function mapped for MushroomBlock.performBonemeal (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for MushroomBlock.performBonemeal: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.below -> (not mapped yet)
//   ServerLevel.getBlockState -> (not mapped yet)
//   LevelReader.getBlockState -> (not mapped yet)
//   Blocks.BROWN_MUSHROOM -> (not mapped yet)
//   Blocks.RED_MUSHROOM -> (not mapped yet)
//   MushroomBlock.canSurvive -> (not mapped yet)
//   MushroomBlock.performBonemeal -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/MushroomMixin.java:30:
// | @Inject(method = "performBonemeal", at = @At("HEAD"), cancellable = true)
// | private void growStructure(ServerLevel world, RandomSource random, BlockPos pos, BlockState state, BonemealSource bonemealSource, CallbackInfo info) {
// |    if (BlocksHelper.isNetherMycelium(world.getBlockState(pos.below()))) {
// |       if (state.getBlock() == Blocks.RED_MUSHROOM) {
// |          NetherTrees.PATCH_BIG_RED_MUSHROOM.placeInWorld(WorldState.registryAccess(), world, pos, random);
// |          info.cancel();
// |       } else if (state.getBlock() == Blocks.BROWN_MUSHROOM) {
// |          NetherTrees.PATCH_BIG_BROWN_MUSHROOM.placeInWorld(WorldState.registryAccess(), world, pos, random);
// |          info.cancel();
// |       }
// |    }
// | }
