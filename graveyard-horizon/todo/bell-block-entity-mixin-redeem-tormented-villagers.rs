// Not ported yet: the Java @Inject(TAIL) on BellBlockEntity.onHit (BellBlockEntityMixin.graveyard$redeemTormentedVillagers).
// Why it's off: No Horizon function mapped for BellBlockEntity.onHit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for BellBlockEntity.onHit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerLevel.getGameTime -> (not mapped yet)
//   BellBlockEntity.onHit -> (not mapped yet)
//   BlockEntity.getBlockPos -> (not mapped yet)
//   BlockEntity.getLevel -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/BellBlockEntityMixin.java:20:
// | @Inject(method = "onHit", at = @At("TAIL"))
// | private void graveyard$redeemTormentedVillagers(Direction direction, CallbackInfo callback) {
// |    BlockEntity bell = (BlockEntity)this;
// |    if (bell.getLevel() instanceof ServerLevel world) {
// |       if (world.getGameTime() - this.graveyard$lastScanTime < 20L) {
// |          return;
// |       }
// |
// |       this.graveyard$lastScanTime = world.getGameTime();
// |       TormentedVillagerEntity.onBellRung(world, bell.getBlockPos());
// |    }
// | }
