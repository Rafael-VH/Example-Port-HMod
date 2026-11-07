// Not ported yet: the Java @Inject(RETURN) on class_2852.method_12410 (MixinChunkSerializer.write).
// Why it's off: No Horizon function mapped for class_2852.method_12410 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for class_2852.method_12410: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   class_2487.method_10547 -> (not mapped yet)
//   class_2487.method_10570 -> (not mapped yet)
//   class_2791.method_12004 -> (not mapped yet)
//   class_2818.field_12855 -> (not mapped yet)
//   class_2852.method_12395 -> (not mapped yet)
//   class_2852.method_12410 -> (not mapped yet)
//
// Decompiled Java, com/dtteam/dynamictrees/mixin/MixinChunkSerializer.java:37:
// | @Inject(at = @At("RETURN"), method = "method_12410(Lnet/minecraft/class_3218;Lnet/minecraft/class_2791;)Lnet/minecraft/class_2487;")
// | private static void write(class_3218 level, class_2791 chunk, CallbackInfoReturnable<class_2487> cir) {
// |    if ((Boolean)DTConfigs.SERVER.worldGen.get()) {
// |       LevelContext levelContext = LevelContext.create(level);
// |       UniversalPoissonDiscProvider discProvider = DynamicTreeFeature.DISC_PROVIDER;
// |       class_1923 chunkPos = chunk.method_12004();
// |       byte[] circleData = discProvider.getChunkPoissonData(levelContext, chunkPos);
// |       ((class_2487)cir.getReturnValue()).method_10570("GTCD", circleData);
// |       if (chunk instanceof class_2818 && !((class_2818)chunk).field_12855) {
// |          discProvider.unloadChunkPoissonData(levelContext, chunkPos);
// |       }
// |    }
// | }
