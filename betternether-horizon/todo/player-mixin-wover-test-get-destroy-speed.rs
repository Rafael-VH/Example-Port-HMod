// Not ported yet: the Java @WrapOperation(INVOKE) on Player.getDestroySpeed (PlayerMixin.wover_test_getDestroySpeed).
// Why it's off: No Horizon function mapped for Player.getDestroySpeed (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for Player.getDestroySpeed: run `match`.
// Note: The call it hooks (ItemStack.getDestroySpeed) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LivingEntity.createLivingAttributes -> (not mapped yet)
//   Builder.add -> (not mapped yet)
//   Player.createAttributes -> (not mapped yet)
//   Player.getDestroySpeed -> (not mapped yet)
//   ItemStack.getDestroySpeed -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/obsidian_breaker/PlayerMixin.java:28:
// | @WrapOperation(
// |    method = "getDestroySpeed",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/item/ItemStack;getDestroySpeed(Lnet/minecraft/world/level/block/state/BlockState;)F")
// | )
// | float wover_test_getDestroySpeed(ItemStack instance, BlockState blockState, Operation<Float> original) {
// |    LivingEntity entity = (LivingEntity)this;
// |    return ObsidianBreaker.modifyObsidianBreakerSpeed(blockState, (Float)original.call(new Object[]{instance, blockState}), entity);
// | }
