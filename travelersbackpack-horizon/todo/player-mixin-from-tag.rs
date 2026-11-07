// Not ported yet: the Java @Inject(HEAD) on Player.readAdditionalSaveData (PlayerMixin.fromTag).
// Why it's off: No Horizon function mapped for Player.readAdditionalSaveData (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Player.readAdditionalSaveData: run `match`.
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
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/PlayerMixin.java:137:
// | @Inject(method = "readAdditionalSaveData", at = @At("HEAD"))
// | private void fromTag(ValueInput view, CallbackInfo ci) {
// |    Optional<ValueInput> cca = view.child("cardinal_components");
// |    cca.ifPresent(nested -> {
// |       Optional<ValueInput> backpack = nested.child("travelersbackpack:travelersbackpack");
// |       backpack.ifPresent(nestedBackpack -> {
// |          ItemStack stack = nestedBackpack.read("Wearable", ItemStack.OPTIONAL_CODEC).orElseGet(() -> new ItemStack(Items.AIR, 0));
// |          this.setAttached(ModAttachmentTypes.TRAVELERS_BACKPACK, new BackpackAttachment(stack));
// |       });
// |    });
// | }
