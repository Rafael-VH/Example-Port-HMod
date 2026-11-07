// Not ported yet: the Java @Inject(HEAD) on Entity.isInWater (EntityMixin.travelersbackpack$isInWater).
// Why it's off: No Horizon function mapped for Entity.isInWater (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Entity.isInWater: run `match`.
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
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/entity/item/EntityMixin.java:21:
// | @Inject(method = "isInWater", at = @At("HEAD"), cancellable = true)
// | private void travelersbackpack$isInWater(CallbackInfoReturnable<Boolean> cir) {
// |    if (this.travelersbackpack$isBackpack() && (Boolean)TravelersBackpackConfig.SERVER.backpackSettings.voidProtection.get()) {
// |       Entity entity = (Entity)this;
// |       if (entity.getY() < entity.level().getMinY() + 1) {
// |          cir.setReturnValue(true);
// |       }
// |    }
// | }
