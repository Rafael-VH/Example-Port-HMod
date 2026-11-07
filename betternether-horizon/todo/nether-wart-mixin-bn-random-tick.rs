// Not ported yet: the Java @Inject(HEAD) on NetherWartBlock.randomTick (NetherWartMixin.bn_randomTick).
// Why it's off: No Horizon function mapped for NetherWartBlock.randomTick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for NetherWartBlock.randomTick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.below -> (not mapped yet)
//   ServerLevel.getBlockState -> (not mapped yet)
//   ServerLevel.setBlock -> horizon_world::level::Behaviours::set_block (suggested, high)
//   RandomSource.nextInt -> (not mapped yet)
//   NetherWartBlock.AGE -> (not mapped yet)
//   NetherWartBlock.mayPlaceOn -> (not mapped yet)
//   NetherWartBlock.randomTick -> (not mapped yet)
//   VegetationBlock.<init> -> (not mapped yet)
//   VegetationBlock.randomTick -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//   BlockState.getValue -> (not mapped yet)
//   BlockState.setValue -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/NetherWartMixin.java:33:
// | @Inject(method = "randomTick", at = @At("HEAD"), cancellable = true)
// | private void bn_randomTick(BlockState state, ServerLevel world, BlockPos pos, RandomSource random, CallbackInfo info) {
// |    super.randomTick(state, world, pos, random);
// |    int i = (Integer)state.getValue(NetherWartBlock.AGE);
// |    if (i < 3) {
// |       int chance = BlocksHelper.isFertile(world.getBlockState(pos.below())) ? 3 : 10;
// |       if (random.nextInt(chance) == 0) {
// |          state = (BlockState)state.setValue(NetherWartBlock.AGE, i + 1);
// |          world.setBlock(pos, state, 2);
// |       }
// |    }
// |
// |    info.cancel();
// | }
