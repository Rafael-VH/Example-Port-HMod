// Not ported yet: the Java @Inject(RETURN) on BiomeColors.getAverageWaterColor (BiomeColorsMixin.be_getWaterColor).
// Why it's off: No Horizon function mapped for BiomeColors.getAverageWaterColor (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for BiomeColors.getAverageWaterColor: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BiomeColors.getAverageWaterColor -> (not mapped yet)
//   BlockAndTintGetter.getBlockState -> (not mapped yet)
//   BlockPos.getX -> (not mapped yet)
//   BlockPos.getY -> (not mapped yet)
//   BlockPos.getZ -> (not mapped yet)
//   MutableBlockPos.<init> -> (not mapped yet)
//   MutableBlockPos.setX -> (not mapped yet)
//   MutableBlockPos.setY -> (not mapped yet)
//   MutableBlockPos.setZ -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/client/BiomeColorsMixin.java:26:
// | @Inject(method = "getAverageWaterColor", at = @At("RETURN"), cancellable = true)
// | private static void be_getWaterColor(BlockAndTintGetter world, BlockPos pos, CallbackInfoReturnable<Integer> info) {
// |    if (world != null && (Boolean)Configs.CLIENT_CONFIG.sulfurWaterColor.get()) {
// |       try {
// |          MutableBlockPos mut = new MutableBlockPos();
// |          mut.setY(pos.getY());
// |
// |          for (int i = 0; i < OFFSETS.length; i++) {
// |             mut.setX(pos.getX() + OFFSETS[i].x);
// |             mut.setZ(pos.getZ() + OFFSETS[i].y);
// |             BlockState state = world.getBlockState(mut);
// |             if (state != null && state.is(EndStoneBlocks.BRIMSTONE.block())) {
// |                info.setReturnValue(i < 4 ? POISON_COLOR : STREAM_COLOR);
// |                return;
// |             }
// |          }
// |       } catch (RuntimeException var6) {
// |       }
// |    }
// | }
