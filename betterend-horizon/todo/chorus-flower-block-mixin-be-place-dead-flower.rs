// Not ported yet: the Java @Inject(HEAD) on ChorusFlowerBlock.placeDeadFlower (ChorusFlowerBlockMixin.be_placeDeadFlower).
// Why it's off: No Horizon function mapped for ChorusFlowerBlock.placeDeadFlower (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ChorusFlowerBlock.placeDeadFlower: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.above -> (not mapped yet)
//   BlockPos.below -> (not mapped yet)
//   BlockPos.getY -> (not mapped yet)
//   ServerLevel.getBlockState -> (not mapped yet)
//   ServerLevel.isEmptyBlock -> (not mapped yet)
//   Level.getBlockState -> (not mapped yet)
//   Level.levelEvent -> (not mapped yet)
//   Level.setBlock -> horizon_world::level::Behaviours::set_block (suggested, high)
//   LevelReader.getBlockState -> (not mapped yet)
//   Block.<init> -> content. A block's properties become its content-mod JSON (template, hardness, light, sounds).
//   Block.box -> (not mapped yet)
//   Block.defaultBlockState -> (not mapped yet)
//   Block.getShape -> (not mapped yet)
//   Blocks.CHORUS_PLANT -> (not mapped yet)
//   ChorusFlowerBlock.AGE -> (not mapped yet)
//   ChorusFlowerBlock.canSurvive -> (not mapped yet)
//   ChorusFlowerBlock.placeDeadFlower -> (not mapped yet)
//   ChorusFlowerBlock.randomTick -> (not mapped yet)
//   ChorusPlantBlock.DOWN -> (not mapped yet)
//   ChorusPlantBlock.UP -> (not mapped yet)
//   BlockState.getValue -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   BlockState.setValue -> (not mapped yet)
//   BlockStateProperties.AGE_5 -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/ChorusFlowerBlockMixin.java:81:
// | @Inject(method = "placeDeadFlower", at = @At("HEAD"), cancellable = true)
// | private void be_placeDeadFlower(Level world, BlockPos pos, CallbackInfo info) {
// |    BlockState down = world.getBlockState(pos.below());
// |    if (down.is(Blocks.CHORUS_PLANT) || down.is(CommonBlockTags.END_STONES)) {
// |       world.setBlock(pos, (BlockState)this.defaultBlockState().setValue(BlockStateProperties.AGE_5, 5), 2);
// |       world.levelEvent(1034, pos, 0);
// |    }
// |
// |    info.cancel();
// | }
