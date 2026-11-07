// Not ported yet: the Java @ModifyVariable(LOAD) on SpriteSourceList.<init> (SpriteSourceListMixin.continuity$modifySources).
// Why it's off: No Horizon function mapped for SpriteSourceList.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for SpriteSourceList.<init>: run `match`.
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
// Decompiled Java, me/pepperbell/continuity/client/mixin/SpriteSourceListMixin.java:33:
// | @ModifyVariable(method = "<init>(Ljava/util/List;)V", at = @At(value = "LOAD", ordinal = 0), argsOnly = true, ordinal = 0)
// | private List<SpriteSource> continuity$modifySources(List<SpriteSource> sources) {
// |    SpriteSourceListInitContext context = SpriteSourceListInitContext.THREAD_LOCAL.get();
// |    if (context != null) {
// |       Set<Identifier> extraIds = context.getExtraIds();
// |       if (!extraIds.isEmpty()) {
// |          List<SpriteSource> extraSources = new ObjectArrayList();
// |
// |          for (Identifier extraId : extraIds) {
// |             extraSources.add(new SingleFile(extraId, Optional.empty()));
// |          }
// |
// |          if (!(sources instanceof ArrayList)) {
// |             List<SpriteSource> mutableSources = new ArrayList<>(extraSources);
// |             mutableSources.addAll(sources);
// |             return mutableSources;
// |          }
// |
// |          sources.addAll(0, extraSources);
// |       }
// |    }
// |
// |    return sources;
// | }
