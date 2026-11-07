// Not ported yet: the Java @Inject(HEAD) on CraftingMenu.slotChangedCraftingGrid (CraftingScreenHandlerMixin.graveyard$requireKnownStaffRecipe).
// Why it's off: No Horizon function mapped for CraftingMenu.slotChangedCraftingGrid (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for CraftingMenu.slotChangedCraftingGrid: run `match`.
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
// Decompiled Java, com/lion/graveyard/mixin/CraftingScreenHandlerMixin.java:27:
// | @Inject(method = "slotChangedCraftingGrid", at = @At("HEAD"), cancellable = true)
// | private static void graveyard$requireKnownStaffRecipe(
// |    AbstractContainerMenu handler,
// |    ServerLevel world,
// |    Player player,
// |    CraftingContainer craftingInventory,
// |    ResultContainer resultInventory,
// |    @Nullable RecipeHolder<CraftingRecipe> lastRecipe,
// |    CallbackInfo info
// | ) {
// |    if (player instanceof ServerPlayer serverPlayer) {
// |       Optional<RecipeHolder<CraftingRecipe>> match = world.recipeAccess()
// |          .getRecipeFor(RecipeType.CRAFTING, craftingInventory.asCraftInput(), world, lastRecipe);
// |       if (!match.isEmpty() && StaffRecipeLock.isLocked(serverPlayer, match.get())) {
// |          resultInventory.setItem(0, ItemStack.EMPTY);
// |          handler.setRemoteSlot(0, ItemStack.EMPTY);
// |          serverPlayer.connection.send(new ClientboundContainerSetSlotPacket(handler.containerId, handler.incrementStateId(), 0, ItemStack.EMPTY));
// |          info.cancel();
// |       }
// |    }
// | }
