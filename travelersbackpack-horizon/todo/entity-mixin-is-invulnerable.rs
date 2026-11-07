// Not ported yet: the Java @Inject(HEAD) on Entity.isInvulnerable (EntityMixin.travelersbackpack$isInvulnerable).
// Why it's off: No Horizon function mapped for Entity.isInvulnerable (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Entity.isInvulnerable: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.getY -> `self.native_player()?.y`. The local player only.
//   Entity.isInWater -> (not mapped yet)
//   Entity.isInvulnerable -> (not mapped yet)
//   Entity.level -> (not mapped yet)
//   Entity.onBelowWorld -> (not mapped yet)
//   ItemEntity.getItem -> (not mapped yet)
//   ItemStack.getItem -> (not mapped yet)
//   Level.getMinY -> horizon_client::game::Dim::min_y (suggested, high)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/entity/item/EntityMixin.java:31:
// | @Inject(method = "isInvulnerable", at = @At("HEAD"), cancellable = true)
// | private void travelersbackpack$isInvulnerable(CallbackInfoReturnable<Boolean> cir) {
// |    if (this.travelersbackpack$isBackpack() && (Boolean)TravelersBackpackConfig.SERVER.backpackSettings.invulnerableBackpack.get()) {
// |       cir.setReturnValue(true);
// |    }
// | }
