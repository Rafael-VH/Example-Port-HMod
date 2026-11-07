// Not ported yet: the Java @Inject(TAIL) on RecipeBookSettings.replaceFrom (RecipeBookSettingsMixin.fdrf$replaceFromCookingBook).
// Why it's off: No Horizon function mapped for RecipeBookSettings.replaceFrom (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for RecipeBookSettings.replaceFrom: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RecipeBookSettings.<clinit> -> (not mapped yet)
//   RecipeBookSettings.copy -> (not mapped yet)
//   RecipeBookSettings.getSettings -> (not mapped yet)
//   RecipeBookSettings.replaceFrom -> (not mapped yet)
//   RecipeBookSettings.updateSettings -> (not mapped yet)
//   TypeSettings.DEFAULT -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/RecipeBookSettingsMixin.java:52:
// | @Inject(method = "replaceFrom", at = @At("TAIL"))
// | private void fdrf$replaceFromCookingBook(RecipeBookSettings other, CallbackInfo ci) {
// |    this.fdrf$cooking = ((RecipeBookSettingsMixin)other).fdrf$cooking;
// | }
