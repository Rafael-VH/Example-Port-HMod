// Not ported yet: the Java @Inject(HEAD) on Block.dropResources (BlockBehaviourMixin.bn_getDrops).
// Why it's off: No Horizon function mapped for Block.dropResources (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Block.dropResources: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Block.dropResources -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/BlockBehaviourMixin.java:20:
// | @Inject(
// |    method = "dropResources(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/ItemStack;)V",
// |    at = @At("HEAD"),
// |    cancellable = true
// | )
// | private static void bn_getDrops(
// |    BlockState brokenBlock, Level level, BlockPos blockPos, BlockEntity blockEntity, Entity breakingEntity, ItemStack breakingItem, CallbackInfo ci
// | ) {
// |    if (level instanceof ServerLevel server
// |       && breakingEntity instanceof Player player
// |       && RubyFire.getDrops(brokenBlock, server, blockPos, player, breakingItem)) {
// |       ci.cancel();
// |    }
// | }
