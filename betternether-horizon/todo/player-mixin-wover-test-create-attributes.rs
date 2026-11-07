// Not ported yet: the Java @WrapOperation(INVOKE) on Player.createAttributes (PlayerMixin.wover_test_createAttributes).
// Why it's off: No Horizon function mapped for Player.createAttributes (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for Player.createAttributes: run `match`.
// Note: The call it hooks (LivingEntity.createLivingAttributes) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LivingEntity.createLivingAttributes -> (not mapped yet)
//   Builder.add -> (not mapped yet)
//   Player.createAttributes -> (not mapped yet)
//   Player.getDestroySpeed -> (not mapped yet)
//   ItemStack.getDestroySpeed -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/obsidian_breaker/PlayerMixin.java:17:
// | @WrapOperation(
// |    method = "createAttributes",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/entity/LivingEntity;createLivingAttributes()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;"
// |    )
// | )
// | private static Builder wover_test_createAttributes(Operation<Builder> original) {
// |    return ((Builder)original.call(new Object[0])).add(NetherEnchantments.OBSIDIAN_BLOCK_BREAK_SPEED);
// | }
