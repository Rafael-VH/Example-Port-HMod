// Not ported yet: the Java @Inject(RETURN) on ItemStack.getTooltipLines (ItemStackMixin.be_getTooltip).
// Why it's off: No Horizon function mapped for ItemStack.getTooltipLines (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for ItemStack.getTooltipLines: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ItemStack.getTooltipLines -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/client/ItemStackMixin.java:18:
// | @Inject(method = "getTooltipLines", at = @At("RETURN"))
// | private void be_getTooltip(TooltipContext tooltipContext, @Nullable Player player, TooltipFlag tooltipFlag, CallbackInfoReturnable<List<Component>> info) {
// |    ((ItemTooltipCallback)ItemTooltipCallback.EVENT.invoker())
// |       .getTooltip(player, ItemStack.class.cast(this), tooltipFlag, (List<Component>)info.getReturnValue());
// | }
