// Not ported yet: the Java @Inject(HEAD) on SecondFloorRoomCollection.get1x1 (SecondFloorMixin.use1x1TemplateList).
// Why it's off: No Horizon function mapped for SecondFloorRoomCollection.get1x1 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for SecondFloorRoomCollection.get1x1: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SecondFloorRoomCollection.get1x1 -> (not mapped yet)
//   SecondFloorRoomCollection.get1x1Secret -> (not mapped yet)
//   SecondFloorRoomCollection.get1x2FrontEntrance -> (not mapped yet)
//   SecondFloorRoomCollection.get1x2Secret -> (not mapped yet)
//   SecondFloorRoomCollection.get1x2SideEntrance -> (not mapped yet)
//   SecondFloorRoomCollection.get2x2 -> (not mapped yet)
//   SecondFloorRoomCollection.get2x2Secret -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/template/mansion/floor/SecondFloorMixin.java:14:
// | @Inject(method = "get1x1", at = @At("HEAD"), cancellable = true)
// | public void use1x1TemplateList(RandomSource random, CallbackInfoReturnable<String> cir) {
// |    if (Lithostitched.breaksSeedParity()) {
// |       cir.setReturnValue(this.lithostitched$getRandom("1x1", random));
// |    }
// | }
