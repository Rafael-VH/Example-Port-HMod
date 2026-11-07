// Not ported yet: the Java @Inject(INVOKE) on DispenserBlock.dispenseFrom (CuttingBoardDispenserMixin.onCuttingBoardDispenseFromInject).
// Why it's off: `horizon_client::game::machines::Game<'_>::dispense_from` has a `?` in its body: horizon-mixins doesn't rewrite such bodies with BEFORE_CALL. A HEAD hook on it, or a BEFORE_CALL on a function it calls, may work instead.
// horizon-mixins kind: BEFORE_CALL on `horizon_client::game::machines::Game<'_>::dispense_from`
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: The call it hooks (DispenserBlock.getDispenseMethod) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.relative -> (not mapped yet)
//   ServerLevel.getBlockState -> (not mapped yet)
//   DispenserBlock.FACING -> (not mapped yet)
//   DispenserBlock.dispenseFrom -> horizon_client::game::machines::Game<'_>::dispense_from (suggested, high)
//   DispenserBlock.getDispenseMethod -> (not mapped yet)
//   DispenserBlockEntity.setItem -> (not mapped yet)
//   BlockState.getValue -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/CuttingBoardDispenserMixin.java:28:
// | @Inject(
// |    method = "dispenseFrom",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/block/DispenserBlock;getDispenseMethod(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/core/dispenser/DispenseItemBehavior;"
// |    ),
// |    locals = LocalCapture.CAPTURE_FAILHARD,
// |    cancellable = true
// | )
// | public void onCuttingBoardDispenseFromInject(
// |    ServerLevel level, BlockState state, BlockPos pos, CallbackInfo ci, DispenserBlockEntity dispenser, BlockSource source, int slot, ItemStack stack
// | ) {
// |    BlockState facingState = level.getBlockState(pos.relative((Direction)state.getValue(DispenserBlock.FACING)));
// |    if (Configuration.ENABLE_DISPENSER_TOOLS_CUTTING_BOARD.get() && facingState.is(ModBlocks.CUTTING_BOARD.get())) {
// |       dispenser.setItem(slot, CuttingBoardDispenseBehavior.INSTANCE.dispense(source, stack));
// |       ci.cancel();
// |    }
// | }
