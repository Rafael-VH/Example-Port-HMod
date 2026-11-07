// Not ported yet: the Java @Inject(TAIL) on LightningRodBlock.onLightningStrike (LightningRodBlockMixin.bn_onLightningStrike).
// Why it's off: No Horizon function mapped for LightningRodBlock.onLightningStrike (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for LightningRodBlock.onLightningStrike: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MutableBlockPos.<init> -> (not mapped yet)
//   MutableBlockPos.setWithOffset -> (not mapped yet)
//   Direction.DOWN -> (not mapped yet)
//   Direction.EAST -> (not mapped yet)
//   Direction.NORTH -> (not mapped yet)
//   Direction.SOUTH -> (not mapped yet)
//   Direction.UP -> (not mapped yet)
//   Direction.WEST -> (not mapped yet)
//   Level.getBlockState -> (not mapped yet)
//   Blocks.CRYING_OBSIDIAN -> (not mapped yet)
//   Blocks.OBSIDIAN -> (not mapped yet)
//   LightningRodBlock.onLightningStrike -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/LightningRodBlockMixin.java:25:
// | @Inject(method = "onLightningStrike", at = @At("TAIL"))
// | void bn_onLightningStrike(BlockState blockState, Level level, BlockPos blockPos, CallbackInfo ci) {
// |    MutableBlockPos mutableBlockPos = new MutableBlockPos();
// |
// |    for (Direction dir : bn_update_directions) {
// |       mutableBlockPos.setWithOffset(blockPos, dir);
// |       BlockState state = level.getBlockState(mutableBlockPos);
// |       if (state.is(Blocks.OBSIDIAN)) {
// |          BNObsidian.onLightningUpdate(level, mutableBlockPos, Blocks.CRYING_OBSIDIAN);
// |       } else if (state.is(Blocks.CRYING_OBSIDIAN)) {
// |          BNObsidian.onLightningUpdate(level, mutableBlockPos, NetherObsidianBlocks.WEEPING_OBSIDIAN.block());
// |       }
// |    }
// | }
