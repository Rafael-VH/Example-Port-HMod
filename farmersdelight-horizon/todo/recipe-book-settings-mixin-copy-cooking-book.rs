// Not ported yet: the Java @ModifyReturnValue(RETURN) on RecipeBookSettings.copy (RecipeBookSettingsMixin.fdrf$copyCookingBook).
// Why it's off: No Horizon function mapped for RecipeBookSettings.copy (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for RecipeBookSettings.copy: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RecipeBookSettings.<clinit> -> (not mapped yet)
//   RecipeBookSettings.copy -> (not mapped yet)
//   RecipeBookSettings.getSettings -> (not mapped yet)
//   RecipeBookSettings.replaceFrom -> (not mapped yet)
//   RecipeBookSettings.updateSettings -> (not mapped yet)
//   TypeSettings.DEFAULT -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/RecipeBookSettingsMixin.java:46:
// | @ModifyReturnValue(method = "copy", at = @At("RETURN"))
// | private RecipeBookSettings fdrf$copyCookingBook(RecipeBookSettings original) {
// |    ((RecipeBookSettingsMixin)original).fdrf$cooking = this.fdrf$cooking;
// |    return original;
// | }
