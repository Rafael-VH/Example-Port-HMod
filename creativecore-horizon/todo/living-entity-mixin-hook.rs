// Not ported yet: the Java @WrapOperation(MIXINEXTRAS:EXPRESSION) on LivingEntity.completeUsingItem (LivingEntityMixin.hook).
// Why it's off: No Horizon function mapped for LivingEntity.completeUsingItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for LivingEntity.completeUsingItem: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LivingEntity.completeUsingItem -> (not mapped yet)
//   ItemStack.copy -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/LivingEntityMixin.java:28:
// | )
// | @Expression("this.useItem.finishUsingItem(this.level(), this)")
// | public ItemStack hook(ItemStack instance, Level level, LivingEntity livingEntity, Operation<ItemStack> original) {
// |    ((ICreativeLoader.FinishConsuming)CreativeFabricLoader.FINISH_CONSUMING.invoker()).accept(livingEntity, instance.copy());
// |    return (ItemStack)original.call(new Object[]{instance, level, livingEntity});
// | }
