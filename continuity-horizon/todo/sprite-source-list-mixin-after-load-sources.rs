// Not ported yet: the Java @Inject(INVOKE) on SpriteSourceList.list (SpriteSourceListMixin.continuity$afterLoadSources).
// Why it's off: `horizon_client_render::atlas_sources::list` has a closure in its body: horizon-mixins doesn't rewrite such bodies with BEFORE_CALL. A HEAD hook on it, or a BEFORE_CALL on a function it calls, may work instead.
// horizon-mixins kind: BEFORE_CALL on `horizon_client_render::atlas_sources::list`
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: The call it hooks (ImmutableList.builder) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ImmutableList.builder -> (not mapped yet)
//   SpriteResourceLoader.loadSprite -> (not mapped yet)
//   SpriteSourceList.<init> -> (not mapped yet)
//   SpriteSourceList.list -> horizon_client_render::atlas_sources::list (suggested, high)
//   SingleFile.<init> -> (not mapped yet)
//   Identifier.getPath -> (not mapped yet)
//   Identifier.withPath -> (not mapped yet)
//   ResourceManager.getResource -> (not mapped yet)
//
// Decompiled Java, me/pepperbell/continuity/client/mixin/SpriteSourceListMixin.java:58:
// | @Inject(
// |    method = "list(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/List;",
// |    at = @At(value = "INVOKE", target = "Lcom/google/common/collect/ImmutableList;builder()Lcom/google/common/collect/ImmutableList$Builder;", remap = false),
// |    locals = LocalCapture.CAPTURE_FAILHARD
// | )
// | private void continuity$afterLoadSources(
// |    ResourceManager resourceManager,
// |    CallbackInfoReturnable<List<Function<SpriteResourceLoader, SpriteContents>>> cir,
// |    Map<Identifier, DiscardableLoader> loaders
// | ) {
// |    SpriteSourceListListContext context = SpriteSourceListListContext.THREAD_LOCAL.get();
// |    if (context != null) {
// |       String emissiveSuffix = EmissiveSuffixLoader.getEmissiveSuffix();
// |       if (emissiveSuffix != null) {
// |          Map<Identifier, DiscardableLoader> emissiveLoaders = new Object2ObjectOpenHashMap();
// |          Map<Identifier, Identifier> emissiveIdMap = new Object2ObjectOpenHashMap();
// |          loaders.forEach((id, supplier) -> {
// |             if (!id.getPath().endsWith(emissiveSuffix)) {
// |                Identifier emissiveId = id.withPath(id.getPath() + emissiveSuffix);
// |                if (!loaders.containsKey(emissiveId)) {
// |                   Identifier emissiveLocation = emissiveId.withPath("textures/" + emissiveId.getPath() + ".png");
// |                   Optional<Resource> optionalResource = resourceManager.getResource(emissiveLocation);
// |                   if (optionalResource.isPresent()) {
// |                      Resource resource = optionalResource.get();
// |                      emissiveLoaders.put(emissiveId, resourceLoader -> resourceLoader.loadSprite(emissiveId, resource));
// |                      emissiveIdMap.put(id, emissiveId);
// |                   }
// |                } else {
// |                   emissiveIdMap.put(id, emissiveId);
// |                }
// |             }
// |          });
// |          loaders.putAll(emissiveLoaders);
// |          context.setEmissiveIdMap(emissiveIdMap);
// |       } else {
// |          context.setEmissiveIdMap(Collections.emptyMap());
// |       }
// |    }
// | }
