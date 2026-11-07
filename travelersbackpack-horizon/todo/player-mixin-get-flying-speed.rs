// Not ported yet: the Java @Inject(RETURN) on Player.getFlyingSpeed (PlayerMixin.getFlyingSpeed).
// Why it's off: No Horizon function mapped for Player.getFlyingSpeed (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for Player.getFlyingSpeed: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   NonNullList.get -> (not mapped yet)
//   NonNullList.size -> (not mapped yet)
//   MobEffectInstance.<init> -> (not mapped yet)
//   MobEffects.SLOWNESS -> (not mapped yet)
//   LivingEntity.<init> -> (not mapped yet)
//   Abilities.flying -> (not mapped yet)
//   Inventory.getNonEquipmentItems -> (not mapped yet)
//   Player.addEffect -> (not mapped yet)
//   Player.attack -> horizon_client::game::Game<'_>::attack (suggested, high)
//   Player.getFlyingSpeed -> (not mapped yet)
//   Player.getInventory -> (not mapped yet)
//   Player.getOffhandItem -> (not mapped yet)
//   Player.isCreative -> (not mapped yet)
//   Player.level -> (not mapped yet)
//   Player.readAdditionalSaveData -> (not mapped yet)
//   Player.tick -> horizon_client::attack_strength::AttackStrength::tick (suggested, high)
//   ItemStack.<init> -> (not mapped yet)
//   ItemStack.OPTIONAL_CODEC -> (not mapped yet)
//   ItemStack.getItem -> (not mapped yet)
//   Items.AIR -> (not mapped yet)
//   Level.getGameTime -> (not mapped yet)
//   Level.isClientSide -> horizon_world::level::is_client_side (suggested, high)
//   ValueInput.child -> (not mapped yet)
//   ValueInput.read -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/PlayerMixin.java:109:
// | @Inject(method = "getFlyingSpeed", at = @At("RETURN"), cancellable = true)
// | protected void getFlyingSpeed(CallbackInfoReturnable<Float> cir) {
// |    if (this instanceof Player player
// |       && BackpackAbilities.ABILITIES.checkBackpack(player, ModItems.FOX_TRAVELERS_BACKPACK)
// |       && (!this.abilities.flying || this.isPassenger())) {
// |       cir.setReturnValue(this.isSprinting() ? 0.038999997F : 0.033F);
// |    }
// | }
