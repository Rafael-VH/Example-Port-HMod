// Not ported yet: the Java @Inject(HEAD) on Piglin.populateDefaultEquipmentSlots (PiglinMixin.bn_populateDefaultEquipmentSlots).
// Why it's off: No Horizon function mapped for Piglin.populateDefaultEquipmentSlots (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Piglin.populateDefaultEquipmentSlots: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RandomSource.nextInt -> (not mapped yet)
//   EquipmentSlot.CHEST -> (not mapped yet)
//   EquipmentSlot.FEET -> (not mapped yet)
//   EquipmentSlot.HEAD -> (not mapped yet)
//   EquipmentSlot.LEGS -> (not mapped yet)
//   Piglin.isAdult -> (not mapped yet)
//   Piglin.populateDefaultEquipmentSlots -> (not mapped yet)
//   ItemStack.<init> -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/piglin/PiglinMixin.java:23:
// | @Inject(method = "populateDefaultEquipmentSlots", at = @At("HEAD"), cancellable = true)
// | void bn_populateDefaultEquipmentSlots(RandomSource randomSource, DifficultyInstance difficultyInstance, CallbackInfo ci) {
// |    if (((Piglin)this).isAdult() && (Boolean)Configs.GAME_RULES.piglinWearNetherArmor.get()) {
// |       int random = randomSource.nextInt(100);
// |       NetherSet set = null;
// |       if (random < 2) {
// |          set = NetherEquipmentItems.FLAMING_RUBY_SET;
// |       } else if (random < 10) {
// |          set = NetherEquipmentItems.NETHER_RUBY_SET;
// |       } else if (random < 30) {
// |          set = NetherEquipmentItems.CINCINNASITE_SET;
// |       }
// |
// |       if (set != null) {
// |          this.maybeWearArmor(EquipmentSlot.HEAD, new ItemStack(set.get(ArmorSlot.HELMET_SLOT)), randomSource);
// |          this.maybeWearArmor(EquipmentSlot.CHEST, new ItemStack(set.get(ArmorSlot.CHESTPLATE_SLOT)), randomSource);
// |          this.maybeWearArmor(EquipmentSlot.LEGS, new ItemStack(set.get(ArmorSlot.LEGGINGS_SLOT)), randomSource);
// |          this.maybeWearArmor(EquipmentSlot.FEET, new ItemStack(set.get(ArmorSlot.BOOTS_SLOT)), randomSource);
// |          ci.cancel();
// |       }
// |    }
// | }
