// Not ported yet: the Java @Inject(HEAD) on PiglinBrute.populateDefaultEquipmentSlots (PiglinBruteMixin.bn_populateDefaultEquipmentSlots).
// Why it's off: No Horizon function mapped for PiglinBrute.populateDefaultEquipmentSlots (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for PiglinBrute.populateDefaultEquipmentSlots: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RandomSource.nextInt -> (not mapped yet)
//   EquipmentSlot.HEAD -> (not mapped yet)
//   PiglinBrute.populateDefaultEquipmentSlots -> (not mapped yet)
//   PiglinBrute.setItemSlot -> (not mapped yet)
//   ItemStack.<init> -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/piglin/PiglinBruteMixin.java:19:
// | @Inject(method = "populateDefaultEquipmentSlots", at = @At("HEAD"), cancellable = true)
// | void bn_populateDefaultEquipmentSlots(RandomSource randomSource, DifficultyInstance difficultyInstance, CallbackInfo ci) {
// |    if ((Boolean)Configs.GAME_RULES.piglinWearNetherArmor.get()) {
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
// |          ((PiglinBrute)this).setItemSlot(EquipmentSlot.HEAD, new ItemStack(set.get(ArmorSlot.HELMET_SLOT)));
// |          ci.cancel();
// |       }
// |    }
// | }
