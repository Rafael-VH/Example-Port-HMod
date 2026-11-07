// Not ported yet: the Java @Inject(TAIL) on ComponentSerialization.bootstrap (ComponentSerializationMixin.register).
// Why it's off: No Horizon function mapped for ComponentSerialization.bootstrap (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ComponentSerialization.bootstrap: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ComponentSerialization.bootstrap -> (not mapped yet)
//   LateBoundIdMapper.put -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/ComponentSerializationMixin.java:15:
// | @Inject(at = @At("TAIL"), method = "bootstrap")
// | private static void register(LateBoundIdMapper<String, MapCodec<? extends ComponentContents>> lateBoundIdMapper, CallbackInfo info) {
// |    lateBoundIdMapper.put("stack", ContentItemStack.CODEC);
// | }
