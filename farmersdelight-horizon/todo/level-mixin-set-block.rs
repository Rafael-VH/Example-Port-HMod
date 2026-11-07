// Not ported yet: the Java @WrapOperation(INVOKE) on Level.removeBlock (LevelMixin.fdrf$setBlock).
// Why it's off: Can't read the signature of `horizon_world::level::Behaviours::set_block` for the wrapper.
// horizon-mixins kind: WRAP_OPERATION on `horizon_world::level::update::Sim<'_, L>::remove_block`
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Level.removeBlock -> horizon_world::level::update::Sim<'_, L>::remove_block (suggested, high)
//   Level.setBlock -> horizon_world::level::Behaviours::set_block (suggested, high)
//   BlockState.getBlock -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/LevelMixin.java:18:
// | @WrapOperation(
// |    method = "removeBlock",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/Level;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z"
// |    )
// | )
// | public boolean fdrf$setBlock(Level instance, BlockPos pos, BlockState blockState, int updateFlags, Operation<Boolean> original) {
// |    BlockState oldState = this.getBlockState(pos);
// |    boolean returnValue = (Boolean)original.call(new Object[]{instance, pos, blockState, updateFlags});
// |    return oldState.getBlock() instanceof ReplaceOnRemoval replaceOnRemoval && replaceOnRemoval.onRemove(oldState, blockState, instance, pos, updateFlags)
// |       ? true
// |       : returnValue;
// | }
