// Not ported yet: the Java @Inject(TAIL) on ItemEntity.setItem (ItemEntityMixin.travelersbackpack$onSetItem).
// Why it's off: No Horizon function mapped for ItemEntity.setItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ItemEntity.setItem: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ParticleTypes.DRAGON_BREATH -> (not mapped yet)
//   PowerParticleOption.create -> (not mapped yet)
//   ServerLevel.sendParticles -> horizon_client::game::fishing::Game<'_>::send_particles (suggested, high)
//   Mth.cos -> (not mapped yet)
//   Mth.sin -> (not mapped yet)
//   RandomSource.nextDouble -> (not mapped yet)
//   RandomSource.nextFloat -> (not mapped yet)
//   Entity.<init> -> (not mapped yet)
//   ItemEntity.fireImmune -> (not mapped yet)
//   ItemEntity.ignoreExplosion -> (not mapped yet)
//   ItemEntity.playerTouch -> (not mapped yet)
//   ItemEntity.setItem -> (not mapped yet)
//   ItemEntity.tick -> horizon_client::entities::ItemEntity::inside_block (suggested, high)
//   Player.blockPosition -> (not mapped yet)
//   Player.level -> (not mapped yet)
//   ItemStack.getCount -> horizon_world::loot::LootStack::get_count (suggested, high)
//   ItemStack.getItem -> (not mapped yet)
//   ItemStack.isEmpty -> (not mapped yet)
//   Level.getMinY -> horizon_client::game::Dim::min_y (suggested, high)
//   Level.isClientSide -> horizon_world::level::is_client_side (suggested, high)
//   Vec3.ZERO -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/entity/item/ItemEntityMixin.java:65:
// | @Inject(method = "setItem", at = @At("TAIL"))
// | private void travelersbackpack$onSetItem(ItemStack stack, CallbackInfo ci) {
// |    if (stack.getItem() instanceof TravelersBackpackItem) {
// |       boolean neverDespawn = (Boolean)TravelersBackpackConfig.SERVER.backpackSettings.neverDespawn.get();
// |       if (neverDespawn) {
// |          this.setUnlimitedLifetime();
// |       }
// |    }
// | }
