// Not ported yet: the Java @Inject(HEAD) on ItemStack.finishUsingItem (ItemStackMixin.fdrf$onItemUseFinished).
// Why it's off: No Horizon function mapped for ItemStack.finishUsingItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ItemStack.finishUsingItem: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ItemStack.finishUsingItem -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/ItemStackMixin.java:14:
// | @Inject(method = "finishUsingItem", at = @At("HEAD"))
// | private void fdrf$onItemUseFinished(Level level, LivingEntity livingEntity, CallbackInfoReturnable<ItemStack> cir) {
// |    CommonEvents.onItemUseFinished(level, livingEntity, (ItemStack)this);
// | }
