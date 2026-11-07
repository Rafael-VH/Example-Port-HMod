// Not ported yet: the Java @Inject(HEAD) on FirstFloorRoomCollection.get1x2FrontEntrance (FirstFloorMixin.use1x2FrontTemplateList).
// Why it's off: No Horizon function mapped for FirstFloorRoomCollection.get1x2FrontEntrance (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for FirstFloorRoomCollection.get1x2FrontEntrance: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FirstFloorRoomCollection.get1x1 -> (not mapped yet)
//   FirstFloorRoomCollection.get1x1Secret -> (not mapped yet)
//   FirstFloorRoomCollection.get1x2FrontEntrance -> (not mapped yet)
//   FirstFloorRoomCollection.get1x2Secret -> (not mapped yet)
//   FirstFloorRoomCollection.get1x2SideEntrance -> (not mapped yet)
//   FirstFloorRoomCollection.get2x2 -> (not mapped yet)
//   FirstFloorRoomCollection.get2x2Secret -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/template/mansion/floor/FirstFloorMixin.java:35:
// | @Inject(method = "get1x2FrontEntrance", at = @At("HEAD"), cancellable = true)
// | public void use1x2FrontTemplateList(RandomSource random, boolean bl, CallbackInfoReturnable<String> cir) {
// |    if (Lithostitched.breaksSeedParity() && !bl) {
// |       cir.setReturnValue(this.lithostitched$getRandom("1x2_front", random));
// |    }
// | }
