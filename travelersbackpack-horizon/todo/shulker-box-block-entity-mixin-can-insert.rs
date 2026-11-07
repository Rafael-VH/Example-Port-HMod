// Not ported yet: the Java @Inject(HEAD) on ShulkerBoxBlockEntity.canPlaceItemThroughFace (ShulkerBoxBlockEntityMixin.canInsert).
// Why it's off: No Horizon function mapped for ShulkerBoxBlockEntity.canPlaceItemThroughFace (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ShulkerBoxBlockEntity.canPlaceItemThroughFace: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ItemStack.getItem -> (not mapped yet)
//   ShulkerBoxBlockEntity.canPlaceItemThroughFace -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/ShulkerBoxBlockEntityMixin.java:14:
// | @Inject(at = @At("HEAD"), method = "canPlaceItemThroughFace", cancellable = true)
// | public void canInsert(int slot, ItemStack stack, Direction dir, CallbackInfoReturnable<Boolean> cir) {
// |    if (stack.getItem() instanceof TravelersBackpackItem) {
// |       cir.setReturnValue(false);
// |    }
// | }
