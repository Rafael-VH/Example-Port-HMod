// Not ported yet: the Java @Inject(HEAD) on SecondFloorRoomCollection.get2x2Secret (SecondFloorMixin.use2x2SecretTemplateList).
// Why it's off: No Horizon function mapped for SecondFloorRoomCollection.get2x2Secret (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for SecondFloorRoomCollection.get2x2Secret: run `match`.
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
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/template/mansion/floor/SecondFloorMixin.java:56:
// | @Inject(method = "get2x2Secret", at = @At("HEAD"), cancellable = true)
// | public void use2x2SecretTemplateList(RandomSource random, CallbackInfoReturnable<String> cir) {
// |    if (Lithostitched.breaksSeedParity()) {
// |       cir.setReturnValue(this.lithostitched$getRandom("2x2_secret", random));
// |    }
// | }
