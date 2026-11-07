// Not ported yet: the Java @Inject(TAIL) on InventoryChangeTrigger.trigger (InventoryChangedCriterionMixin.graveyard$countSkeletonDecoration).
// Why it's off: No Horizon function mapped for InventoryChangeTrigger.trigger (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for InventoryChangeTrigger.trigger: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   InventoryChangeTrigger.trigger -> (not mapped yet)
//   Inventory.getContainerSize -> (not mapped yet)
//   Inventory.getItem -> (not mapped yet)
//   ItemStack.getItem -> (not mapped yet)
//   ItemStack.is -> (not mapped yet)
//   ItemStack.isEmpty -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/InventoryChangedCriterionMixin.java:21:
// | @Inject(
// |    method = "trigger(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/entity/player/Inventory;Lnet/minecraft/world/item/ItemStack;)V",
// |    at = @At("TAIL")
// | )
// | private void graveyard$countSkeletonDecoration(ServerPlayer player, Inventory inventory, ItemStack stack, CallbackInfo callback) {
// |    if (!stack.isEmpty() && stack.is(TGTags.SKELETON_DECORATION)) {
// |       Set<Item> distinct = new HashSet<>();
// |
// |       for (int slot = 0; slot < inventory.getContainerSize(); slot++) {
// |          ItemStack held = inventory.getItem(slot);
// |          if (held.is(TGTags.SKELETON_DECORATION)) {
// |             distinct.add(held.getItem());
// |          }
// |       }
// |
// |       if (distinct.size() >= 10) {
// |          TGAdvancements.BONE_COLLECTOR.trigger(player);
// |       }
// |    }
// | }
