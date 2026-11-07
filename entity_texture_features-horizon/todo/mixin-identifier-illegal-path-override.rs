// Not ported yet: the Java @Inject(RETURN) on Identifier.isValidPath (MixinIdentifier.etf$illegalPathOverride).
// Why it's off: No Horizon function mapped for Identifier.isValidPath (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for Identifier.isValidPath: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Component.getString -> (not mapped yet)
//   Identifier.isValidPath -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/MixinIdentifier.java:19:
// | @Inject(method = "isValidPath", cancellable = true, at = @At("RETURN"), require = 0)
// | private static void etf$illegalPathOverride(String path, CallbackInfoReturnable<Boolean> cir) {
// |    if (ETF.config().getConfig() != null
// |       && ETF.config().getConfig().illegalPathSupportMode != ETFConfig.IllegalPathMode.None
// |       && !(Boolean)cir.getReturnValue()
// |       && path != null) {
// |       if (path.equals("DUMMY") || path.isBlank()) {
// |          return;
// |       }
// |
// |       switch (ETF.config().getConfig().illegalPathSupportMode) {
// |          case Entity:
// |             if ((path.contains("/entity/") || path.contains("optifine/") || path.contains("etf/") || path.contains("emf/"))
// |                && (path.endsWith(".png") || path.endsWith(".properties") || path.endsWith(".mcmeta") || path.endsWith(".jem") || path.endsWith(".jpm"))) {
// |                allowPathAndLog(path, cir);
// |             }
// |             break;
// |          case All:
// |             allowPathAndLog(path, cir);
// |             break;
// |          default:
// |             ETFUtils2.logWarn("this message should not appear #65164");
// |       }
// |    }
// | }
