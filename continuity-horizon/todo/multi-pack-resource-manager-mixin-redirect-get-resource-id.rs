// Not ported yet: the Java @ModifyVariable(HEAD) on MultiPackResourceManager.getResource (MultiPackResourceManagerMixin.continuity$redirectGetResourceId).
// Why it's off: No Horizon function mapped for MultiPackResourceManager.getResource (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for MultiPackResourceManager.getResource: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PackType.CLIENT_RESOURCES -> (not mapped yet)
//   MultiPackResourceManager.<init> -> (not mapped yet)
//   MultiPackResourceManager.getResource -> (not mapped yet)
//   MultiPackResourceManager.getResourceStack -> (not mapped yet)
//
// Decompiled Java, me/pepperbell/continuity/client/mixin/MultiPackResourceManagerMixin.java:26:
// | @ModifyVariable(method = "getResource(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;", at = @At("HEAD"), argsOnly = true)
// | private Identifier continuity$redirectGetResourceId(Identifier id) {
// |    return this.continuity$useRedirects ? ResourceRedirectHandler.redirect(id) : id;
// | }
