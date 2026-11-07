// Not ported yet: the Java @Inject(TAIL) on Minecraft.<init>* (MinecraftClientMixin.onInit).
// Why it's off: No Horizon function mapped for Minecraft.<init>* (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: Wildcard or unknown selector `<init>*`.
//
// Decompiled Java, org/betterx/betternether/mixin/client/MinecraftClientMixin.java:13:
// | @Inject(method = "<init>*", at = @At("TAIL"))
// | private void onInit(GameConfig args, CallbackInfo info) {
// |    Config.save();
// | }
