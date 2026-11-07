// Not ported yet: the Java @Inject(HEAD) on CraftingMenu.quickMoveStack (CraftingScreenHandlerMixin.graveyard$blockLockedStaffShiftClick).
// Why it's off: No Horizon function mapped for CraftingMenu.quickMoveStack (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for CraftingMenu.quickMoveStack: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   NonNullList.get -> (not mapped yet)
//   ClientboundContainerSetSlotPacket.<init> -> (not mapped yet)
//   ServerLevel.recipeAccess -> (not mapped yet)
//   ServerPlayer.connection -> (not mapped yet)
//   ServerGamePacketListenerImpl.send -> (not mapped yet)
//   AbstractContainerMenu.containerId -> (not mapped yet)
//   AbstractContainerMenu.incrementStateId -> (not mapped yet)
//   AbstractContainerMenu.setRemoteSlot -> (not mapped yet)
//   CraftingContainer.asCraftInput -> (not mapped yet)
//   CraftingMenu.quickMoveStack -> (not mapped yet)
//   CraftingMenu.slotChangedCraftingGrid -> (not mapped yet)
//   CraftingMenu.slots -> (not mapped yet)
//   ResultContainer.setItem -> (not mapped yet)
//   Slot.mayPickup -> (not mapped yet)
//   ItemStack.EMPTY -> (not mapped yet)
//   RecipeManager.getRecipeFor -> (not mapped yet)
//   RecipeType.CRAFTING -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/CraftingScreenHandlerMixin.java:49:
// | @Inject(method = "quickMoveStack", at = @At("HEAD"), cancellable = true)
// | private void graveyard$blockLockedStaffShiftClick(Player player, int slot, CallbackInfoReturnable<ItemStack> info) {
// |    CraftingMenu self = (CraftingMenu)this;
// |    if (slot == 0 && !((Slot)self.slots.get(0)).mayPickup(player)) {
// |       info.setReturnValue(ItemStack.EMPTY);
// |    }
// | }
