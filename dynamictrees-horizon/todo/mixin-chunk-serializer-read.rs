// Not ported yet: the Java @Inject(HEAD) on class_2852.method_12395 (MixinChunkSerializer.read).
// Why it's off: No Horizon function mapped for class_2852.method_12395 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for class_2852.method_12395: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   class_2487.method_10547 -> (not mapped yet)
//   class_2487.method_10570 -> (not mapped yet)
//   class_2791.method_12004 -> (not mapped yet)
//   class_2818.field_12855 -> (not mapped yet)
//   class_2852.method_12395 -> (not mapped yet)
//   class_2852.method_12410 -> (not mapped yet)
//
// Decompiled Java, com/dtteam/dynamictrees/mixin/MixinChunkSerializer.java:23:
// | @Inject(
// |    at = @At("HEAD"),
// |    method = "method_12395(Lnet/minecraft/class_3218;Lnet/minecraft/class_4153;Lnet/minecraft/class_9240;Lnet/minecraft/class_1923;Lnet/minecraft/class_2487;)Lnet/minecraft/class_2839;"
// | )
// | private static void read(
// |    class_3218 level, class_4153 poiManager, class_9240 regionStorageInfo, class_1923 pos, class_2487 tag, CallbackInfoReturnable<class_2839> cir
// | ) {
// |    if ((Boolean)DTConfigs.SERVER.worldGen.get()) {
// |       byte[] circleData = tag.method_10547("GTCD");
// |       UniversalPoissonDiscProvider discProvider = DynamicTreeFeature.DISC_PROVIDER;
// |       discProvider.setChunkPoissonData(LevelContext.create(level), pos, circleData);
// |    }
// | }
