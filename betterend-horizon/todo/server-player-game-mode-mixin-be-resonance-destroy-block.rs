// Not ported yet: the Java @Inject(HEAD) on ServerPlayerGameMode.destroyBlock (ServerPlayerGameModeMixin.be_resonanceDestroyBlock).
// Why it's off: No Horizon function mapped for ServerPlayerGameMode.destroyBlock (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ServerPlayerGameMode.destroyBlock: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.getX -> (not mapped yet)
//   BlockPos.getY -> (not mapped yet)
//   BlockPos.getZ -> (not mapped yet)
//   MutableBlockPos.<init> -> (not mapped yet)
//   MutableBlockPos.immutable -> (not mapped yet)
//   MutableBlockPos.set -> (not mapped yet)
//   Direction.getStepX -> (not mapped yet)
//   Direction.getStepY -> (not mapped yet)
//   Direction.getStepZ -> (not mapped yet)
//   MinecraftServer.isUnderSpawnProtection -> (not mapped yet)
//   ServerLevel.getBlockEntity -> (not mapped yet)
//   ServerLevel.getBlockState -> (not mapped yet)
//   ServerLevel.getServer -> (not mapped yet)
//   ServerLevel.removeBlock -> horizon_world::level::update::Sim<'_, L>::remove_block (suggested, high)
//   ServerLevel.sendBlockUpdated -> (not mapped yet)
//   ServerPlayer.blockActionRestricted -> horizon_client::first_person::block_action_restricted (suggested, high)
//   ServerPlayer.canUseGameMasterBlocks -> horizon_client::game::command_blocks::Game<'_>::can_use_game_master_blocks (suggested, high)
//   ServerPlayer.getMainHandItem -> (not mapped yet)
//   ServerPlayer.getNearestViewDirection -> (not mapped yet)
//   ServerPlayer.hasCorrectToolForDrops -> (not mapped yet)
//   ServerPlayer.isShiftKeyDown -> (not mapped yet)
//   ServerPlayer.preventsBlockDrops -> (not mapped yet)
//   ServerPlayerGameMode.destroyBlock -> (not mapped yet)
//   ItemStack.canDestroyBlock -> (not mapped yet)
//   ItemStack.copy -> (not mapped yet)
//   ItemStack.is -> (not mapped yet)
//   ItemStack.mineBlock -> (not mapped yet)
//   Block.destroy -> (not mapped yet)
//   Block.playerDestroy -> (not mapped yet)
//   Block.playerWillDestroy -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//   BlockState.getDestroySpeed -> (not mapped yet)
//   BlockState.isAir -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/ServerPlayerGameModeMixin.java:36:
// | @Inject(method = "destroyBlock", at = @At("HEAD"), cancellable = true)
// | private void be_resonanceDestroyBlock(BlockPos pos, CallbackInfoReturnable<Boolean> info) {
// |    ItemStack mainHand = this.player.getMainHandItem();
// |    if (mainHand.is(CommonItemTags.HAMMERS)) {
// |       if (!this.player.isShiftKeyDown()) {
// |          int resonanceLevel = EnchantmentUtils.getItemEnchantmentLevel(this.level, EndEnchantments.RESONANCE.key(), mainHand);
// |          if (resonanceLevel > 0) {
// |             BlockState primaryState = this.level.getBlockState(pos);
// |             if (!mainHand.canDestroyBlock(primaryState, this.level, pos, this.player)) {
// |                info.setReturnValue(false);
// |             } else {
// |                ItemStack destroyedWith = mainHand.copy();
// |                BlockState primaryAdjustedState = this.be_breakOne(pos, primaryState, destroyedWith, true);
// |                if (primaryAdjustedState == null) {
// |                   info.setReturnValue(false);
// |                } else {
// |                   int radius = resonanceLevel;
// |                   Direction facing = this.player.getNearestViewDirection();
// |                   int[] xRange = be_axisRange(facing.getStepX(), radius);
// |                   int[] yRange = be_axisRange(facing.getStepY(), radius);
// |                   int[] zRange = be_axisRange(facing.getStepZ(), radius);
// |                   MutableBlockPos cursor = new MutableBlockPos();
// |
// |                   for (int dx = xRange[0]; dx <= xRange[1]; dx++) {
// |                      for (int dy = yRange[0]; dy <= yRange[1]; dy++) {
// |                         for (int dz = zRange[0]; dz <= zRange[1]; dz++) {
// |                            if (dx != 0 || dy != 0 || dz != 0) {
// |                               cursor.set(pos.getX() + dx, pos.getY() + dy, pos.getZ() + dz);
// |                               BlockPos extraPos = cursor.immutable();
// |                               this.be_breakOne(extraPos, this.level.getBlockState(extraPos), destroyedWith, false);
// |                            }
// |                         }
// |                      }
// |                   }
// |
// |                   if (!this.player.preventsBlockDrops()) {
// |                      mainHand.mineBlock(this.level, primaryAdjustedState, pos, this.player);
// |                   }
// |
// |                   info.setReturnValue(true);
// |                }
// |             }
// |          }
// |       }
// |    }
// | }
