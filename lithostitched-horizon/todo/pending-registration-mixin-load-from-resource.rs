// Not ported yet: the Java @Inject(INVOKE) on PendingRegistration.loadFromResource (PendingRegistrationMixin.loadFromResource).
// Why it's off: No Horizon function mapped for PendingRegistration.loadFromResource (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for PendingRegistration.loadFromResource: run `match`.
// Note: The call it hooks (Decoder.parse) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Decoder.parse -> (not mapped yet)
//   PendingRegistration.loadFromResource -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/predicate/PendingRegistrationMixin.java:21:
// | @Inject(
// |    method = "loadFromResource",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lcom/mojang/serialization/Decoder;parse(Lcom/mojang/serialization/DynamicOps;Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;"
// |    ),
// |    cancellable = true
// | )
// | private static <T> void loadFromResource(
// |    Decoder<T> elementDecoder,
// |    RegistryOps<JsonElement> ops,
// |    ResourceKey<T> elementKey,
// |    Resource thunk,
// |    CallbackInfoReturnable<Either<T, Exception>> cir,
// |    @Local(name = "json") JsonElement json
// | ) {
// |    if (json.isJsonObject()) {
// |       JsonObject object = json.getAsJsonObject();
// |       if (object.has("predicate")) {
// |          if (!LoadPredicate.CODEC.parse(ops, object.get("predicate")).result().orElse(LoadPredicate.alwaysTrue()).test()) {
// |             cir.setReturnValue(Either.right(new StubException()));
// |          }
// |       }
// |    }
// | }
