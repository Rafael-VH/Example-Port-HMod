// Not ported yet: the Java @Inject(TAIL) on ItemStack.overrideOtherStackedOnMe (ItemStackMixin.afterOverrideOtherStackedOnMe).
// Why it's off: No Horizon function mapped for ItemStack.overrideOtherStackedOnMe (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ItemStack.overrideOtherStackedOnMe: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Player.containerMenu -> (not mapped yet)
//   Slot.container -> (not mapped yet)
//   Slot.set -> (not mapped yet)
//   ItemStack.copy -> (not mapped yet)
//   ItemStack.overrideOtherStackedOnMe -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/ItemStackMixin.java:17:
// | @Inject(method = "overrideOtherStackedOnMe", at = @At("TAIL"))
// | private void afterOverrideOtherStackedOnMe(
// |    ItemStack stack, Slot slot, ClickAction action, Player player, SlotAccess access, CallbackInfoReturnable<Boolean> cir
// | ) {
// |    boolean result = (Boolean)cir.getReturnValue();
// |    if (player.containerMenu instanceof AbstractBackpackMenu && !(slot.container instanceof Inventory) && result && this instanceof ItemStack clicked) {
// |       slot.set(clicked.copy());
// |    }
// | }
