# Horizon port report: Continuity

`continuity` 3.0.1+26.3 · continuity-3.0.1+26.3.jar · 164 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 2 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 15 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 2 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `client`: me.pepperbell.continuity.client.ContinuityClient
- `modmenu`: me.pepperbell.continuity.client.config.ModMenuApiImpl
- `sodium:config_api_user`: me.pepperbell.continuity.client.config.SodiumConfigImpl

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| ClientPlayConnectionEvents.JOIN | `BiomeHolderManager` | hook `tick` | auto | The Fabric API layer runs JOIN on the window's first tick with a player after none, DISCONNECT on the first tick without one after. |
| ClientLifecycleEvents.CLIENT_STARTED | `ProcessingDataKeyRegistryImpl` | hook `frame` | auto | CLIENT_STARTED: the Fabric API layer runs it on the window's first frame. CLIENT_STOPPING has no hook in Horizon a46ce68 (c9d474d has native_world_lifecycle). |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(RETURN) on AtlasManager.prepareSharedState | `AtlasManagerMixin.continuity$onReturnPrepareSharedState` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for AtlasManager.prepareSharedState: run `match`. |
| @Inject(INVOKE) on AtlasManager.reload at Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;resourceManager()Lnet/minecraft/server/packs/resources/ResourceManager; | `AtlasManagerMixin.continuity$onHeadReload` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for AtlasManager.reload: run `match`. The call it hooks (SharedState.resourceManager) isn't mapped to a Rust function yet. |
| @Inject(RETURN) on AtlasManager.reload | `AtlasManagerMixin.continuity$onReturnReload` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for AtlasManager.reload: run `match`. |
| @Inject(RETURN) on CuboidItemModelWrapper.<init> | `CuboidItemModelWrapperMixin.onReturnInit` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for CuboidItemModelWrapper.<init>: run `match`. |
| @Inject(RETURN) on CuboidItemModelWrapper.update | `CuboidItemModelWrapperMixin.onReturnUpdate` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for CuboidItemModelWrapper.update: run `match`. |
| @WrapOperation(NEW) on Unbaked.bake at (Ljava/util/List;Lnet/minecraft/client/resources/model/geometry/QuadCollection;Lnet/minecraft/client/renderer/item/ModelRenderProperties;Lorg/joml/Matrix4fc;)Lnet/minecraft/client/renderer/item/CuboidItemModelWrapper; | `CuboidItemModelWrapperUnbakedMixin.wrapModelCtor` | WRAP_OPERATION | needs-mapping | No Horizon function known for Unbaked.bake: run `match`. |
| @Inject(TAIL) on MultiPackResourceManager.<init> | `MultiPackResourceManagerMixin.continuity$onTailInit` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for MultiPackResourceManager.<init>: run `match`. |
| @ModifyVariable(HEAD) on MultiPackResourceManager.getResource | `MultiPackResourceManagerMixin.continuity$redirectGetResourceId` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for MultiPackResourceManager.getResource: run `match`. |
| @ModifyVariable(HEAD) on MultiPackResourceManager.getResourceStack | `MultiPackResourceManagerMixin.continuity$redirectGetResourceStackId` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for MultiPackResourceManager.getResourceStack: run `match`. |
| @ModifyArg(INVOKE) on SpriteLoader.loadAndStitch at Ljava/util/concurrent/CompletableFuture;supplyAsync(Ljava/util/function/Supplier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture; | `SpriteLoaderMixin.continuity$modifySupplier` | MODIFY_ARG | needs-mapping | No Horizon function known for SpriteLoader.loadAndStitch: run `match`. The call it hooks (CompletableFuture.supplyAsync) isn't mapped to a Rust function yet. |
| @ModifyArg(INVOKE) on SpriteLoader.loadAndStitch at Ljava/util/concurrent/CompletableFuture;thenApply(Ljava/util/function/Function;)Ljava/util/concurrent/CompletableFuture; | `SpriteLoaderMixin.continuity$modifyFunction` | MODIFY_ARG | needs-mapping | No Horizon function known for SpriteLoader.loadAndStitch: run `match`. The call it hooks (CompletableFuture.thenApply) isn't mapped to a Rust function yet. |
| @Inject(RETURN) on SpriteLoader.stitch | `SpriteLoaderMixin.continuity$onReturnStitch` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for SpriteLoader.stitch: run `match`. |
| @ModifyVariable(LOAD) on SpriteSourceList.<init> | `SpriteSourceListMixin.continuity$modifySources` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for SpriteSourceList.<init>: run `match`. |
| @Inject(INVOKE) on SpriteSourceList.list at Lcom/google/common/collect/ImmutableList;builder()Lcom/google/common/collect/ImmutableList$Builder; | `SpriteSourceListMixin.continuity$afterLoadSources` | BEFORE_CALL on horizon_client_render::atlas_sources::list | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. The call it hooks (ImmutableList.builder) isn't mapped to a Rust function yet. |
| @Invoker on StateHolder.getNullableValue | `StateHolderAccessor.continuity$getNullableValue` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for StateHolder.getNullableValue: run `match`. |
| mixin into TextureAtlasSprite with no injectors (interfaces or @Shadow only) | `TextureAtlasSpriteMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends Screen | `ContinuityConfigScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 25 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

6 of 196 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/renderer/texture/atlas/SpriteSourceList#list` | horizon_client_render::atlas_sources::list (suggested) |
| `net/minecraft/core/Direction#getOpposite` | horizon_client::mobs::shulker::opposite (suggested) |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/util/Mth#lerp` | horizon_world::stepped_interpolation::lerp (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/google/common/collect/ImmutableList#builder` | method | 1 |
| `java/util/concurrent/CompletableFuture#supplyAsync` | method | 1 |
| `java/util/concurrent/CompletableFuture#thenApply` | method | 1 |
| `net/minecraft/client/Minecraft#getAtlasManager` | method | 1 |
| `net/minecraft/client/Minecraft#getBlockColors` | method | 1 |
| `net/minecraft/client/Minecraft#getModelManager` | method | 1 |
| `net/minecraft/client/Minecraft#gui` | field | 1 |
| `net/minecraft/client/Minecraft#levelExtractor` | field | 1 |
| `net/minecraft/client/color/block/BlockColors#getTintSource` | method | 1 |
| `net/minecraft/client/color/block/BlockTintSource#colorInWorld` | method | 1 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#centeredText` | method | 1 |
| `net/minecraft/client/gui/components/Button#builder` | method | 1 |
| `net/minecraft/client/gui/components/Button#setMessage` | method | 1 |
| `net/minecraft/client/gui/components/Button$Builder#bounds` | method | 1 |
| `net/minecraft/client/gui/components/Button$Builder#build` | method | 1 |
| `net/minecraft/client/gui/components/Button$Builder#tooltip` | method | 1 |
| `net/minecraft/client/gui/components/Tooltip#create` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#extractRenderState` | method | 1 |
| `net/minecraft/client/model/geom/builders/UVPair#pack` | method | 1 |
| `net/minecraft/client/model/geom/builders/UVPair#unpackU` | method | 1 |
| `net/minecraft/client/model/geom/builders/UVPair#unpackV` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#registryAccess` | method | 1 |
| `net/minecraft/client/renderer/Sheets#cutoutBlockItemSheet` | method | 1 |
| `net/minecraft/client/renderer/Sheets#translucentBlockItemSheet` | method | 1 |
| `net/minecraft/client/renderer/block/BlockAndTintGetter#EMPTY` | field | 1 |
| `net/minecraft/client/renderer/block/BlockAndTintGetter#getBiomeFabric` | method | 1 |
| `net/minecraft/client/renderer/block/BlockAndTintGetter#getBlockEntity` | method | 1 |
| `net/minecraft/client/renderer/block/BlockAndTintGetter#getBlockState` | method | 3 |
| `net/minecraft/client/renderer/block/BlockAndTintGetter#hasBiomes` | method | 1 |
| `net/minecraft/client/renderer/block/BlockStateModelSet#get` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel#emitQuads` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel#particleMaterial` | method | 1 |
| `net/minecraft/client/renderer/chunk/ChunkSectionLayer#CUTOUT` | field | 2 |
| `net/minecraft/client/renderer/chunk/ChunkSectionLayer#SOLID` | field | 1 |
| `net/minecraft/client/renderer/chunk/ChunkSectionLayer#TRANSLUCENT` | field | 2 |
| `net/minecraft/client/renderer/extract/LevelExtractor#allChanged` | method | 1 |
| `net/minecraft/client/renderer/item/CuboidItemModelWrapper#<init>` | method | 1 |
| `net/minecraft/client/renderer/item/CuboidItemModelWrapper#update` | method | 1 |
| `net/minecraft/client/renderer/item/CuboidItemModelWrapper$Unbaked#bake` | method | 1 |
| `net/minecraft/client/renderer/item/ItemModel$BakingContext#blockModelBaker` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#appendModelIdentityElement` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#setAnimated` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#emitter` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setQuads` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#hasBlending` | method | 1 |
| `net/minecraft/client/renderer/texture/MissingTextureAtlasSprite#getLocation` | method | 4 |
| `net/minecraft/client/renderer/texture/SpriteContents#isAnimated` | method | 2 |
| `net/minecraft/client/renderer/texture/SpriteContents#name` | method | 3 |
| `net/minecraft/client/renderer/texture/SpriteLoader#loadAndStitch` | method | 2 |
| `net/minecraft/client/renderer/texture/SpriteLoader#stitch` | method | 1 |
| `net/minecraft/client/renderer/texture/SpriteLoader$Preparations#getSprite` | method | 1 |
| `net/minecraft/client/renderer/texture/SpriteLoader$Preparations#missing` | method | 1 |
| `net/minecraft/client/renderer/texture/SpriteLoader$Preparations#regions` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlas#LOCATION_BLOCKS` | field | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlas#LOCATION_ITEMS` | field | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlas#spriteFinder` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#contents` | method | 5 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU0` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU1` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV0` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV1` | method | 2 |
| `net/minecraft/client/renderer/texture/atlas/SpriteResourceLoader#loadSprite` | method | 1 |
| `net/minecraft/client/renderer/texture/atlas/SpriteSourceList#<init>` | method | 1 |
| `net/minecraft/client/renderer/texture/atlas/sources/SingleFile#<init>` | method | 1 |
| `net/minecraft/client/resources/model/ModelBaker#materials` | method | 1 |
| `net/minecraft/client/resources/model/ModelManager#getBlockStateModelSet` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#<init>` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#direction` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#materialInfo` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#packedUV` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position0` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position1` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position2` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position3` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#<init>` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemGlintRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemGlintSpecialRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#layer` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#sprite` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#tintIndex` | method | 1 |
| `net/minecraft/client/resources/model/geometry/ItemQuads#<init>` | method | 1 |
| `net/minecraft/client/resources/model/geometry/ItemQuads#all` | method | 1 |
| `net/minecraft/client/resources/model/geometry/ItemQuads#isEmpty` | method | 1 |
| `net/minecraft/client/resources/model/geometry/ItemQuads#solid` | method | 1 |
| `net/minecraft/client/resources/model/geometry/ItemQuads#translucent` | method | 1 |
| `net/minecraft/client/resources/model/sprite/AtlasManager#PENDING_STITCH` | field | 1 |
| `net/minecraft/client/resources/model/sprite/AtlasManager#getAtlasOrThrow` | method | 1 |
| `net/minecraft/client/resources/model/sprite/AtlasManager#prepareSharedState` | method | 1 |
| `net/minecraft/client/resources/model/sprite/AtlasManager#reload` | method | 2 |
| `net/minecraft/client/resources/model/sprite/AtlasManager$PendingStitchResults#get` | method | 2 |
| `net/minecraft/client/resources/model/sprite/Material$Baked#sprite` | method | 1 |
| `net/minecraft/core/BlockPos#ZERO` | field | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 2 |
| `net/minecraft/core/BlockPos#getY` | method | 3 |
| `net/minecraft/core/BlockPos#getZ` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 5 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setWithOffset` | method | 7 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setY` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#containsKey` | method | 3 |
| `net/minecraft/core/DefaultedRegistry#getValue` | method | 3 |
| `net/minecraft/core/Direction#DOWN` | field | 6 |
| `net/minecraft/core/Direction#EAST` | field | 2 |
| `net/minecraft/core/Direction#NORTH` | field | 3 |
| `net/minecraft/core/Direction#SOUTH` | field | 3 |
| `net/minecraft/core/Direction#UP` | field | 7 |
| `net/minecraft/core/Direction#WEST` | field | 2 |
| `net/minecraft/core/Direction#fromAxisAndDirection` | method | 1 |
| `net/minecraft/core/Direction#getAxis` | method | 3 |
| `net/minecraft/core/Direction#getAxisDirection` | method | 2 |
| `net/minecraft/core/Direction#getClockWise` | method | 2 |
| `net/minecraft/core/Direction#getCounterClockWise` | method | 2 |
| `net/minecraft/core/Direction#ordinal` | method | 7 |
| `net/minecraft/core/Direction#valueOf` | method | 1 |
| `net/minecraft/core/Direction#values` | method | 5 |
| `net/minecraft/core/Direction$Axis#X` | field | 2 |
| `net/minecraft/core/Direction$Axis#Y` | field | 2 |
| `net/minecraft/core/Direction$Axis#Z` | field | 2 |
| `net/minecraft/core/Direction$Axis#ordinal` | method | 3 |
| `net/minecraft/core/Direction$Axis#values` | method | 1 |
| `net/minecraft/core/Direction$AxisDirection#NEGATIVE` | field | 1 |
| `net/minecraft/core/Direction$AxisDirection#POSITIVE` | field | 2 |
| `net/minecraft/core/Direction$Plane#HORIZONTAL` | field | 1 |
| `net/minecraft/core/Direction$Plane#iterator` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 1 |
| `net/minecraft/core/Registry#containsKey` | method | 2 |
| `net/minecraft/core/Registry#getValue` | method | 1 |
| `net/minecraft/core/Registry#keySet` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 1 |
| `net/minecraft/core/registries/Registries#BIOME` | field | 1 |
| `net/minecraft/data/AtlasIds#BLOCKS` | field | 3 |
| `net/minecraft/network/chat/CommonComponents#GUI_CANCEL` | field | 1 |
| `net/minecraft/network/chat/CommonComponents#GUI_DONE` | field | 1 |
| `net/minecraft/network/chat/CommonComponents#optionNameValue` | method | 1 |
| `net/minecraft/network/chat/CommonComponents#optionStatus` | method | 1 |
| `net/minecraft/network/chat/Component#copy` | method | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 1 |
| `net/minecraft/network/chat/Component#translatable` | method | 3 |
| `net/minecraft/network/chat/Style#withBold` | method | 1 |
| `net/minecraft/resources/Identifier#compareTo` | method | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 5 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 4 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 2 |
| `net/minecraft/resources/Identifier#getPath` | method | 6 |
| `net/minecraft/resources/Identifier#hashCode` | method | 1 |
| `net/minecraft/resources/Identifier#parse` | method | 1 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 3 |
| `net/minecraft/resources/Identifier#withPath` | method | 4 |
| `net/minecraft/server/packs/PackResources#getNamespaces` | method | 1 |
| `net/minecraft/server/packs/PackResources#listResources` | method | 1 |
| `net/minecraft/server/packs/PackResources#packId` | method | 2 |
| `net/minecraft/server/packs/PackType#CLIENT_RESOURCES` | field | 3 |
| `net/minecraft/server/packs/resources/IoSupplier#get` | method | 1 |
| `net/minecraft/server/packs/resources/MultiPackResourceManager#<init>` | method | 1 |
| `net/minecraft/server/packs/resources/MultiPackResourceManager#getResource` | method | 1 |
| `net/minecraft/server/packs/resources/MultiPackResourceManager#getResourceStack` | method | 1 |
| `net/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier#wait` | method | 1 |
| `net/minecraft/server/packs/resources/PreparableReloadListener$SharedState#get` | method | 3 |
| `net/minecraft/server/packs/resources/PreparableReloadListener$SharedState#resourceManager` | method | 3 |
| `net/minecraft/server/packs/resources/PreparableReloadListener$SharedState#set` | method | 2 |
| `net/minecraft/server/packs/resources/PreparableReloadListener$StateKey#<init>` | method | 2 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#sourcePackId` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 3 |
| `net/minecraft/server/packs/resources/ResourceManager#listPacks` | method | 1 |
| `net/minecraft/util/Mth#getSeed` | method | 1 |
| `net/minecraft/util/Mth#inverseLerp` | method | 1 |
| `net/minecraft/util/RandomSource#createThreadLocalInstance` | method | 2 |
| `net/minecraft/util/RandomSource#setSeed` | method | 2 |
| `net/minecraft/world/Nameable#getCustomName` | method | 1 |
| `net/minecraft/world/Nameable#hasCustomName` | method | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 1 |
| `net/minecraft/world/level/block/Block#getStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getAppearance` | method | 4 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 4 |
| `net/minecraft/world/level/block/state/BlockState#getLightEmission` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getSeed` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#hasProperty` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#isCollisionShapeFullBlock` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#isSolidRender` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition#getProperty` | method | 1 |
| `net/minecraft/world/level/block/state/StateHolder#getNullableValue` | method | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#AXIS` | field | 3 |
| `net/minecraft/world/level/block/state/properties/Property#getValue` | method | 1 |

</details>

