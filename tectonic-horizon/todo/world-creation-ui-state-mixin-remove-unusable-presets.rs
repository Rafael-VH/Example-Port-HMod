// Not ported yet: the Java @Inject(RETURN) on WorldCreationUiState.getNonEmptyList (WorldCreationUiStateMixin.tectonic$removeUnusablePresets).
// Why it's off: No Horizon function mapped for WorldCreationUiState.getNonEmptyList (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for WorldCreationUiState.getNonEmptyList: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   WorldCreationUiState.getNonEmptyList -> (not mapped yet)
//   WorldTypeEntry.preset -> (not mapped yet)
//   Holder.is -> (not mapped yet)
//   Registries.WORLD_PRESET -> (not mapped yet)
//   TagKey.create -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/client/WorldCreationUiStateMixin.java:25:
// | @Inject(method = "getNonEmptyList", at = @At("RETURN"), cancellable = true)
// | private static void tectonic$removeUnusablePresets(
// |    Registry<WorldPreset> registry, TagKey<WorldPreset> tag, CallbackInfoReturnable<Optional<List<WorldTypeEntry>>> cir
// | ) {
// |    if (((Optional)cir.getReturnValue()).isPresent()) {
// |       ArrayList<WorldTypeEntry> list = new ArrayList<>((Collection<? extends WorldTypeEntry>)((Optional)cir.getReturnValue()).get());
// |       list.removeIf(entry -> entry.preset() != null && entry.preset().is(INCOMPATIBLE));
// |       cir.setReturnValue(Optional.of(list));
// |    }
// | }
