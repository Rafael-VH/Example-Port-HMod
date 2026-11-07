// Not ported yet: the Java @Inject(TAIL) on Item.useOn (PlacePumpkinPieMixin.usePumpkinPie).
// Why it's off: No Horizon function mapped for Item.useOn (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for Item.useOn: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   InteractionResult.consumesAction -> (not mapped yet)
//   Player.isSecondaryUseActive -> (not mapped yet)
//   BlockItem.place -> horizon_world::level::place::Sim<'_, L>::place (suggested, high)
//   Item.useOn -> (not mapped yet)
//   ItemStack.is -> (not mapped yet)
//   Items.PUMPKIN_PIE -> (not mapped yet)
//   BlockPlaceContext.<init> -> (not mapped yet)
//   UseOnContext.getItemInHand -> (not mapped yet)
//   UseOnContext.getPlayer -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/PlacePumpkinPieMixin.java:19:
// | @Inject(method = "useOn", at = @At("TAIL"), cancellable = true)
// | private void usePumpkinPie(UseOnContext context, CallbackInfoReturnable<InteractionResult> cir) {
// |    if (context.getItemInHand().is(Items.PUMPKIN_PIE)) {
// |       if (Configuration.ENABLE_PUMPKIN_PIE_SNEAK_TO_PLACE.get()) {
// |          Player player = context.getPlayer();
// |          if (player == null || !player.isSecondaryUseActive()) {
// |             return;
// |          }
// |       }
// |
// |       BlockItem pie = (BlockItem)ModItems.DEBUG_PUMPKIN_PIE.get();
// |       InteractionResult placed = pie.place(new BlockPlaceContext(context));
// |       if (placed.consumesAction()) {
// |          cir.setReturnValue(placed);
// |       }
// |    }
// | }
