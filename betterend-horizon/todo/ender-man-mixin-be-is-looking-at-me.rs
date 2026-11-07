// Not ported yet: the Java @Inject(HEAD) on Enderman.isBeingStaredBy (EnderManMixin.be_isLookingAtMe).
// Why it's off: No Horizon function mapped for Enderman.isBeingStaredBy (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Enderman.isBeingStaredBy: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EquipmentSlot.HEAD -> (not mapped yet)
//   Enderman.isBeingStaredBy -> (not mapped yet)
//   Player.getItemBySlot -> (not mapped yet)
//   Player.hasEffect -> horizon_client::mobs::PlayerView::has_effect (suggested, high)
//   Player.isCreative -> (not mapped yet)
//   EnchantmentHelper.has -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/EnderManMixin.java:17:
// | @Inject(method = "isBeingStaredBy", at = @At("HEAD"), cancellable = true)
// | private void be_isLookingAtMe(Player player, CallbackInfoReturnable<Boolean> info) {
// |    ItemStack headItem = player.getItemBySlot(EquipmentSlot.HEAD);
// |    if (player.isCreative() || player.hasEffect(EndStatusEffects.END_VEIL) || EnchantmentHelper.has(headItem, EndEnchantments.END_VEIL_STATE)) {
// |       info.setReturnValue(false);
// |    }
// | }
