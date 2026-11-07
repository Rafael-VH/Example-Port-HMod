// Not ported yet: the Java @Inject(RETURN) on AbstractFurnaceBlockEntity.getBurnDuration (AbstractFurnaceBlockEntityMixin.betternether$creditCrystalBurn).
// Why it's off: No Horizon function mapped for AbstractFurnaceBlockEntity.getBurnDuration (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for AbstractFurnaceBlockEntity.getBurnDuration: run `match`.
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
// Decompiled Java, org/betterx/betternether/mixin/common/AbstractFurnaceBlockEntityMixin.java:26:
// | @Inject(method = "getBurnDuration", at = @At("RETURN"))
// | private void betternether$creditCrystalBurn(ServerLevel level, ItemStack itemStack, CallbackInfoReturnable<Integer> cir) {
// |    if (cir.getReturnValueI() > 0
// |       && (
// |          itemStack.is(((BlockGloomsculkCrystal)NetherTerrainBlocks.GLOOMSCULK_CRYSTAL.block()).asItem())
// |             || itemStack.is(((BlockGloomsculkCrystal)NetherTerrainBlocks.GLOOMSCULK_GEODE_CRYSTAL.block()).asItem())
// |       )) {
// |       BlockPos pos = ((BlockEntity)this).getBlockPos();
// |       if (level.getNearestPlayer(pos.getX() + 0.5, pos.getY() + 0.5, pos.getZ() + 0.5, 8.0, false) instanceof ServerPlayer serverPlayer) {
// |          BNCriterion.BURNED_GLOOMSCULK_CRYSTAL.trigger(serverPlayer);
// |       }
// |    }
// | }
