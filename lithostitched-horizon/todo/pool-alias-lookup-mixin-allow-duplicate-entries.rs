// Not ported yet: the Java @Redirect(INVOKE) on PoolAliasLookup.create (PoolAliasLookupMixin.lithostitched$allowDuplicateEntries).
// Why it's off: No Horizon function mapped for PoolAliasLookup.create (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: A redirect is a WRAP_OPERATION that doesn't call `original`.
// Note: No Horizon function known for PoolAliasLookup.create: run `match`.
// Note: The call it hooks (Builder.build) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Builder.build -> (not mapped yet)
//   PoolAliasLookup.create -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/PoolAliasLookupMixin.java:12:
// | @Redirect(
// |    method = "create",
// |    at = @At(value = "INVOKE", remap = false, target = "Lcom/google/common/collect/ImmutableMap$Builder;build()Lcom/google/common/collect/ImmutableMap;")
// | )
// | private static ImmutableMap<?, ?> lithostitched$allowDuplicateEntries(Builder<?, ?> instance) {
// |    return instance.buildKeepingLast();
// | }
