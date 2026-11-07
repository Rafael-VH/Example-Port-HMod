// Not ported yet: the Java @Inject(RETURN) on AbstractFurnaceBlockEntity.getTotalCookTime (AbstractFurnaceBlockEntityMixin.betternether$getTotalCookTime).
// Why it's off: No Horizon function mapped for AbstractFurnaceBlockEntity.getTotalCookTime (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for AbstractFurnaceBlockEntity.getTotalCookTime: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PlayerTrigger.trigger -> (not mapped yet)
//   BlockPos.getX -> (not mapped yet)
//   BlockPos.getY -> (not mapped yet)
//   BlockPos.getZ -> (not mapped yet)
//   ServerLevel.getNearestPlayer -> (not mapped yet)
//   ItemStack.is -> (not mapped yet)
//   AbstractFurnaceBlockEntity.getBurnDuration -> (not mapped yet)
//   AbstractFurnaceBlockEntity.getTotalCookTime -> (not mapped yet)
//   BlockEntity.getBlockPos -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/AbstractFurnaceBlockEntityMixin.java:40:
// | @Inject(
// |    method = "getTotalCookTime(Lnet/minecraft/world/item/crafting/RecipeHolder;Lnet/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity;)I",
// |    at = @At("RETURN"),
// |    cancellable = true
// | )
// | private static void betternether$getTotalCookTime(
// |    RecipeHolder<? extends AbstractCookingRecipe> recipe, AbstractFurnaceBlockEntity inventory, CallbackInfoReturnable<Integer> cir
// | ) {
// |    if (inventory instanceof ChangebleCookTime cct) {
// |       int val = (Integer)cir.getReturnValue();
// |       cir.setReturnValue(cct.changeCookTime(val));
// |    }
// | }
