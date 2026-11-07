// Not ported yet: the Java @ModifyVariable(HEAD) on MultiPackResourceManager.getResourceStack (MultiPackResourceManagerMixin.continuity$redirectGetResourceStackId).
// Why it's off: No Horizon function mapped for MultiPackResourceManager.getResourceStack (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for MultiPackResourceManager.getResourceStack: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PackType.CLIENT_RESOURCES -> (not mapped yet)
//   MultiPackResourceManager.<init> -> (not mapped yet)
//   MultiPackResourceManager.getResource -> (not mapped yet)
//   MultiPackResourceManager.getResourceStack -> (not mapped yet)
//
// Decompiled Java, me/pepperbell/continuity/client/mixin/MultiPackResourceManagerMixin.java:31:
// | @ModifyVariable(method = "getResourceStack(Lnet/minecraft/resources/Identifier;)Ljava/util/List;", at = @At("HEAD"), argsOnly = true)
// | private Identifier continuity$redirectGetResourceStackId(Identifier id) {
// |    return this.continuity$useRedirects ? ResourceRedirectHandler.redirect(id) : id;
// | }
