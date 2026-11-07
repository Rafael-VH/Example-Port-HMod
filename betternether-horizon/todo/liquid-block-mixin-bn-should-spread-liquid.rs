// Not ported yet: the Java @Inject(INVOKE) on LiquidBlock.shouldSpreadLiquid (LiquidBlockMixin.bn_shouldSpreadLiquid).
// Why it's off: The call it hooks (Lnet/minecraft/world/level/Level;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z) isn't mapped to a Rust function.
// horizon-mixins kind: BEFORE_CALL on `horizon_world::level::fluids::Sim<'_, L>::should_spread_liquid`
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: The call it hooks (Level.setBlockAndUpdate) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PlayerTrigger.trigger -> (not mapped yet)
//   BlockPos.below -> (not mapped yet)
//   BlockPos.getX -> (not mapped yet)
//   BlockPos.getY -> (not mapped yet)
//   BlockPos.getZ -> (not mapped yet)
//   Level.getBlockState -> (not mapped yet)
//   Level.getEntitiesOfClass -> (not mapped yet)
//   Level.setBlockAndUpdate -> (not mapped yet)
//   Blocks.OBSIDIAN -> (not mapped yet)
//   Blocks.SOUL_SAND -> (not mapped yet)
//   Blocks.SOUL_SOIL -> (not mapped yet)
//   LiquidBlock.shouldSpreadLiquid -> horizon_world::level::fluids::Sim<'_, L>::should_spread_liquid (suggested, high)
//   BlockState.is -> (not mapped yet)
//   AABB.<init> -> (not mapped yet)
//   AABB.inflate -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/LiquidBlockMixin.java:29:
// | @Inject(
// |    method = "shouldSpreadLiquid",
// |    locals = LocalCapture.CAPTURE_FAILSOFT,
// |    cancellable = true,
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/Level;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z",
// |       ordinal = 0
// |    )
// | )
// | void bn_shouldSpreadLiquid(
// |    Level level,
// |    BlockPos blockPos,
// |    BlockState arg2,
// |    CallbackInfoReturnable<Boolean> cir,
// |    boolean bl,
// |    UnmodifiableIterator var5,
// |    Direction direction,
// |    BlockPos blockPos2,
// |    Block block
// | ) {
// |    if (block == Blocks.OBSIDIAN) {
// |       BlockState belowState = level.getBlockState(blockPos.below());
// |       if (belowState.is(Blocks.SOUL_SOIL) || belowState.is(Blocks.SOUL_SAND)) {
// |          level.setBlockAndUpdate(blockPos, ((BNObsidian)NetherObsidianBlocks.BLUE_OBSIDIAN.block()).defaultBlockState());
// |          this.fizz(level, blockPos);
// |          int x = blockPos.getX();
// |          int y = blockPos.getY();
// |          int z = blockPos.getZ();
// |
// |          for (ServerPlayer serverPlayer : level.getEntitiesOfClass(ServerPlayer.class, new AABB(x, y, z, x, y - 4, z).inflate(10.0, 5.0, 10.0))) {
// |             BNCriterion.BREW_BLUE.trigger(serverPlayer);
// |          }
// |
// |          cir.setReturnValue(false);
// |          cir.cancel();
// |       }
// |    }
// | }
