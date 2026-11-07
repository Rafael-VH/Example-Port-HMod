# Horizon port report: Storage Drawers

`storagedrawers` 26.3.0.1 · StorageDrawers-fabric-26.3.0.1.jar · 392 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 1 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 4 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 0 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 30 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 11 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: com.jaquadro.minecraft.storagedrawers.StorageDrawers
- `client`: com.jaquadro.minecraft.storagedrawers.StorageDrawersClient
- `jade`: com.jaquadro.minecraft.storagedrawers.integration.Waila

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| ServerTickEvents.END_SERVER_TICK | `StorageDrawers` | hook `tick` | auto | The tick hook runs in every Game, the server's too: run server logic only when `!self.native_window()`. |
| AttackBlockCallback.EVENT | `CommonEvents` | – | unsupported | No interaction hook yet: needs a raw target in Horizon's game code. |
| ClientPlayNetworking.registerGlobalReceiver | `FabricClient` | – | unsupported | Custom packets aren't supported. |
| ClientPlayNetworking.send | `FabricClient` | – | unsupported | Custom packets aren't supported. |
| PayloadTypeRegistry.clientboundPlay | `FabricNetworking` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.register | `FabricNetworking` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.serverboundPlay | `FabricNetworking` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| ServerPlayNetworking.registerGlobalReceiver | `FabricNetworking` | – | unsupported | Custom packets aren't supported. |
| ServerPlayNetworking.send | `FabricNetworking` | – | unsupported | Custom packets aren't supported. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends HorizontalDirectionalBlock | `BlockController` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BlockControllerIO` | needs-translation | Overrides getController, newBlockEntity, makeFramedItem, newBlockEntity: the content ports, that behaviour needs native code. |
| extends HorizontalDirectionalBlock | `BlockFramingTable` | needs-translation | No mapping for this superclass yet. |
| extends FaceAttachedHorizontalDirectionalBlock | `BlockKeyButton` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BlockTrim` | needs-translation | Overrides setMatKey, setMatKey, getMatKey, getNameMatKey, getNameTypeKey, canUseForRetrim, makeFramedItem: the content ports, that behaviour needs native code. |
| extends HorizontalDirectionalBlock | `FaceSlotBlock` | needs-translation | No mapping for this superclass yet. |
| extends Block | `BlockMeta` | content | A plain block becomes a content-mod block copying a vanilla template. Overridden behaviour methods need native code. |
| extends Property | `IntegerSetProperty` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntity | `BaseBlockEntity` | unsupported | Block entities aren't supported in Horizon mods yet. |
| extends BlockEntityRenderState | `DrawersRenderState` | needs-translation | No mapping for this superclass yet. |
| extends BlockEntityRenderState | `FramingRenderState` | needs-translation | No mapping for this superclass yet. |
| extends CustomRecipe | `AddUpgradeRecipe` | needs-translation | No mapping for this superclass yet. |
| extends CustomRecipe | `KeyringRecipe` | needs-translation | No mapping for this superclass yet. |
| extends CustomRecipe | `PersonalKeyRecipe` | needs-translation | No mapping for this superclass yet. |
| extends CustomRecipe | `RemoteGroupUpgradeRecipe` | needs-translation | No mapping for this superclass yet. |
| extends CustomRecipe | `UpgradeDetachedDrawerRecipe` | needs-translation | No mapping for this superclass yet. |
| extends AbstractContainerMenu | `ContainerDrawers` | unsupported | Menus aren't supported. |
| extends AbstractContainerMenu | `ContainerFramingTable` | unsupported | Menus aren't supported. |
| extends Slot | `CraftResultSlot` | needs-translation | No mapping for this superclass yet. |
| extends AbstractContainerScreen | `DrawerScreen` | needs-translation | No mapping for this superclass yet. |
| extends AbstractContainerScreen | `FramingTableScreen` | needs-translation | No mapping for this superclass yet. |
| extends Slot | `RestrictedSlot` | needs-translation | No mapping for this superclass yet. |
| extends SimpleContainer | `SlotDrawer$EmptyInventory` | needs-translation | No mapping for this superclass yet. |
| extends Slot | `SlotDrawer` | needs-translation | No mapping for this superclass yet. |
| extends Slot | `SlotStorage` | needs-translation | No mapping for this superclass yet. |
| extends Slot | `SlotUpgrade` | needs-translation | No mapping for this superclass yet. |
| extends Item | `ItemDetachedDrawer` | needs-translation | Overrides getDefaultInstance, appendHoverText, getDescription, getTooltipImage, isHeavy, canFitInsideContainerItems: the content ports, that behaviour needs native code. |
| extends BlockItem | `ItemDrawers` | needs-translation | Overrides appendHoverText, getName, getDescription, isHeavy, getCapacityForBlock, canFitInsideContainerItems, doesSneakBypassUse: the content ports, that behaviour needs native code. |
| extends Item | `ItemKey` | needs-translation | Overrides createAttributes, canDestroyBlock, appendHoverText, getDescription, useOn, handleDrawerAttributes, isEnabled: the content ports, that behaviour needs native code. |
| extends Item | `ItemKeyring` | needs-translation | Overrides getKey, overrideOtherStackedOnMe, use, useOn, getKeyring, getTooltipImage, appendHoverText, getName, getDescription, onDestroyed, getActiveKey, getContents, rotateKeyring: the content ports, that behaviour needs native code. |
| extends BlockItem | `ItemTrim` | needs-translation | Overrides getName, doesSneakBypassUse: the content ports, that behaviour needs native code. |
| extends Item | `ItemUpgrade` | needs-translation | Overrides getNextGroupId, doesSneakBypassUse, getUpgradeGroup, appendHoverText, getDescription, isEnabled, setAllowMultiple, getAllowMultiple: the content ports, that behaviour needs native code. |
| extends TransientCraftingContainer | `CompactingHelper$InventoryLookup` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `storagedrawers.fabric.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| registers blocks or items in code | `ModCreativeTabs` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `FabricRegistry` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| 948 asset files, 216 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

45 of 659 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blitSprite` | horizon_client_render::ui::blit_sprite (suggested) |
| `net/minecraft/client/gui/GuiGraphicsExtractor#itemDecorations` | horizon_client_render::hud_overlays::item_decorations (suggested) |
| `net/minecraft/client/renderer/item/ItemStackRenderState#submit` | horizon_client::item_render::ItemRenderState::submit (suggested) |
| `net/minecraft/core/Direction#getOpposite` | horizon_client::mobs::shulker::opposite (suggested) |
| `net/minecraft/core/Registry#register` | content |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/nbt/CompoundTag#merge` | horizon_world::worldgen::structure::processor::merge (suggested) |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/network/codec/ByteBufCodecs#list` | horizon_protocol::item::skip::list (suggested) |
| `net/minecraft/server/level/ServerLevel#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/server/level/ServerLevel#scheduleTick` | horizon_world::level::Behaviours::schedule_tick (suggested) |
| `net/minecraft/server/level/ServerLevel#updateNeighbourForOutputSignal` | horizon_world::level::update::Sim<'_, L>::update_neighbour_for_output_signal (suggested) |
| `net/minecraft/util/ARGB#opaque` | horizon_client_render::level_loading::opaque (suggested) |
| `net/minecraft/util/Mth#ceil` | horizon_client_render::hud_layout::ceil (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/world/Container#stillValid` | horizon_client::lan::menu::Slots::still_valid (suggested) |
| `net/minecraft/world/Containers#dropItemStack` | horizon_world::level::removal::drop_item_stack (suggested) |
| `net/minecraft/world/entity/player/Inventory#add` | horizon_client::cauldron::add (suggested) |
| `net/minecraft/world/entity/player/Player#getViewVector` | horizon_client::mobs::goals::Mob::view_vector (suggested) |
| `net/minecraft/world/inventory/Slot#remove` | horizon_client::menu::Menu::remove (suggested) |
| `net/minecraft/world/inventory/Slot#setByPlayer` | horizon_client::menu::Menu::set_by_player (suggested) |
| `net/minecraft/world/item/Item#<init>` | content |
| `net/minecraft/world/item/Item#canDestroyBlock` | horizon_items::items::Items::can_destroy_block (suggested) |
| `net/minecraft/world/item/Item$Properties#stacksTo` | content |
| `net/minecraft/world/item/ItemStack#getCount` | horizon_world::loot::LootStack::get_count (suggested) |
| `net/minecraft/world/item/ItemStack#isSameItemSameComponents` | horizon_items::item_data::ItemStack::same_item_same_components (suggested) |
| `net/minecraft/world/item/ItemStack#matches` | horizon_client_render::vault::matches (suggested) |
| `net/minecraft/world/item/ItemStack#shrink` | horizon_client::game::dispenser::shrink (suggested) |
| `net/minecraft/world/item/ItemStack#transmuteCopy` | horizon_items::item_data::ItemStack::transmute (suggested) |
| `net/minecraft/world/item/ItemStackTemplate#create` | horizon_client::crafting::Template::create (suggested) |
| `net/minecraft/world/item/component/TooltipDisplay#shows` | horizon_client::item_tooltip::TooltipDisplay<'_>::shows (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/level/Level#scheduleTick` | horizon_world::level::Behaviours::schedule_tick (suggested) |
| `net/minecraft/world/level/Level#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/world/level/Level#updateNeighborsAt` | horizon_world::level::Behaviours::update_neighbors_at (suggested) |
| `net/minecraft/world/level/block/Block#<init>` | content |
| `net/minecraft/world/level/block/state/BlockState#canBeReplaced` | horizon_world::level::place::Sim<'_, L>::can_be_replaced (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canOcclude` | horizon_world::worldgen::placement::context::can_occlude (suggested) |
| `net/minecraft/world/phys/AABB#intersects` | horizon_core::geometry::Aabb::intersects_corners (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/vertex/PoseStack#last` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#mulPose` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#normal` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#pose` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#trustedNormals` | field | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertex` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setColor` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setLight` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setNormal` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv` | method | 1 |
| `net/minecraft/ChatFormatting#DARK_GRAY` | field | 1 |
| `net/minecraft/ChatFormatting#GRAY` | field | 2 |
| `net/minecraft/ChatFormatting#RED` | field | 3 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 4 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 1 |
| `net/minecraft/client/Minecraft#getAtlasManager` | method | 1 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 1 |
| `net/minecraft/client/Minecraft#getModelManager` | method | 2 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 1 |
| `net/minecraft/client/Minecraft#level` | field | 1 |
| `net/minecraft/client/Minecraft#schedule` | method | 2 |
| `net/minecraft/client/gui/Font$DisplayMode#POLYGON_OFFSET` | field | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#centeredText` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#item` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 5 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#tooltip` | method | 1 |
| `net/minecraft/client/gui/screens/MenuScreens#register` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#<init>` | method | 2 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractBackground` | method | 2 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractSlot` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#init` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#isHovering` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent#create` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/tooltip/DefaultTooltipPositioner#INSTANCE` | field | 1 |
| `net/minecraft/client/model/geom/builders/UVPair#pack` | method | 1 |
| `net/minecraft/client/model/geom/builders/UVPair#unpackU` | method | 1 |
| `net/minecraft/client/model/geom/builders/UVPair#unpackV` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getCooldowns` | method | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 4 |
| `net/minecraft/client/renderer/Sheets#cutoutBlockItemSheet` | method | 1 |
| `net/minecraft/client/renderer/Sheets#translucentBlockItemSheet` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitCustomGeometry` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitText` | method | 1 |
| `net/minecraft/client/renderer/block/BlockStateModelSet#get` | method | 2 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel#collectParts` | method | 4 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel#emitQuads` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel#materialFlags` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel#particleMaterial` | method | 2 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel$UnbakedRoot#bake` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel$UnbakedRoot#resolveDependencies` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel$UnbakedRoot#visualEqualityGroup` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModelPart#getQuads` | method | 3 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModelPart#materialFlags` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModelPart#particleMaterial` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModelPart#useAmbientOcclusion` | method | 1 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderer#extractRenderState` | method | 2 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context#font` | method | 1 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context#itemModelResolver` | method | 2 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderers#register` | method | 1 |
| `net/minecraft/client/renderer/blockentity/state/BlockEntityRenderState#<init>` | method | 2 |
| `net/minecraft/client/renderer/chunk/ChunkSectionLayer#CUTOUT` | field | 4 |
| `net/minecraft/client/renderer/chunk/ChunkSectionLayer#SOLID` | field | 3 |
| `net/minecraft/client/renderer/chunk/ChunkSectionLayer#TRANSLUCENT` | field | 3 |
| `net/minecraft/client/renderer/chunk/ChunkSectionLayer#ordinal` | method | 2 |
| `net/minecraft/client/renderer/chunk/ChunkSectionLayer#values` | method | 1 |
| `net/minecraft/client/renderer/item/ItemModel$BakingContext#blockModelBaker` | method | 1 |
| `net/minecraft/client/renderer/item/ItemModelResolver#updateForTopItem` | method | 2 |
| `net/minecraft/client/renderer/item/ItemModels#ID_MAPPER` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#<init>` | method | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#appendModelIdentityElement` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#isEmpty` | method | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#newLayer` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setExtents` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#setQuads` | method | 1 |
| `net/minecraft/client/renderer/item/ModelRenderProperties#applyToLayer` | method | 1 |
| `net/minecraft/client/renderer/item/ModelRenderProperties#fromResolvedModel` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#solidMovingBlock` | method | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 2 |
| `net/minecraft/client/renderer/texture/SpriteContents#height` | method | 1 |
| `net/minecraft/client/renderer/texture/SpriteContents#width` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlas#LOCATION_BLOCKS` | field | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#contents` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU0` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU1` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV0` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV1` | method | 2 |
| `net/minecraft/client/resources/language/I18n#get` | method | 1 |
| `net/minecraft/client/resources/model/ModelBaker#getModel` | method | 1 |
| `net/minecraft/client/resources/model/ModelManager#getBlockStateModelSet` | method | 2 |
| `net/minecraft/client/resources/model/ResolvedModel#getTopTextureSlots` | method | 1 |
| `net/minecraft/client/resources/model/cuboid/CuboidModel#fromStream` | method | 1 |
| `net/minecraft/client/resources/model/cuboid/CuboidModel#geometry` | method | 1 |
| `net/minecraft/client/resources/model/cuboid/CuboidModelElement#from` | method | 1 |
| `net/minecraft/client/resources/model/cuboid/CuboidModelElement#to` | method | 1 |
| `net/minecraft/client/resources/model/cuboid/UnbakedCuboidGeometry#elements` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#<init>` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#direction` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#materialInfo` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#packedUV0` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#packedUV1` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#packedUV2` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#packedUV3` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position0` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position1` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position2` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#position3` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#<init>` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemGlintRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemGlintSpecialRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#layer` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#lightEmission` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#shadeDirectionOverride` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#sprite` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#tintIndex` | method | 1 |
| `net/minecraft/client/resources/model/geometry/ItemQuads#split` | method | 1 |
| `net/minecraft/client/resources/model/sprite/AtlasManager#get` | method | 1 |
| `net/minecraft/client/resources/model/sprite/Material$Baked#<init>` | method | 2 |
| `net/minecraft/client/resources/model/sprite/Material$Baked#sprite` | method | 2 |
| `net/minecraft/client/resources/model/sprite/SpriteId#<init>` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 5 |
| `net/minecraft/core/BlockPos#above` | method | 1 |
| `net/minecraft/core/BlockPos#asLong` | method | 2 |
| `net/minecraft/core/BlockPos#below` | method | 2 |
| `net/minecraft/core/BlockPos#compareTo` | method | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 1 |
| `net/minecraft/core/BlockPos#distSqr` | method | 1 |
| `net/minecraft/core/BlockPos#distToCenterSqr` | method | 1 |
| `net/minecraft/core/BlockPos#east` | method | 1 |
| `net/minecraft/core/BlockPos#equals` | method | 3 |
| `net/minecraft/core/BlockPos#getX` | method | 13 |
| `net/minecraft/core/BlockPos#getY` | method | 12 |
| `net/minecraft/core/BlockPos#getZ` | method | 13 |
| `net/minecraft/core/BlockPos#north` | method | 1 |
| `net/minecraft/core/BlockPos#offset` | method | 3 |
| `net/minecraft/core/BlockPos#relative` | method | 3 |
| `net/minecraft/core/BlockPos#south` | method | 1 |
| `net/minecraft/core/BlockPos#west` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#get` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 5 |
| `net/minecraft/core/DefaultedRegistry#getResourceKey` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getValue` | method | 3 |
| `net/minecraft/core/DefaultedRegistry#stream` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 2 |
| `net/minecraft/core/Direction#EAST` | field | 6 |
| `net/minecraft/core/Direction#NORTH` | field | 8 |
| `net/minecraft/core/Direction#SOUTH` | field | 6 |
| `net/minecraft/core/Direction#UP` | field | 3 |
| `net/minecraft/core/Direction#WEST` | field | 6 |
| `net/minecraft/core/Direction#from2DDataValue` | method | 1 |
| `net/minecraft/core/Direction#getAxis` | method | 1 |
| `net/minecraft/core/Direction#getClockWise` | method | 1 |
| `net/minecraft/core/Direction#getCounterClockWise` | method | 1 |
| `net/minecraft/core/Direction#getName` | method | 1 |
| `net/minecraft/core/Direction#getUnitVec3i` | method | 1 |
| `net/minecraft/core/Direction#ordinal` | method | 12 |
| `net/minecraft/core/Direction#values` | method | 9 |
| `net/minecraft/core/Direction$Axis#X` | field | 1 |
| `net/minecraft/core/Holder#tags` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 1 |
| `net/minecraft/core/Holder$Reference#value` | method | 1 |
| `net/minecraft/core/HolderSet$Named#stream` | method | 1 |
| `net/minecraft/core/NonNullList#add` | method | 1 |
| `net/minecraft/core/NonNullList#create` | method | 2 |
| `net/minecraft/core/NonNullList#get` | method | 3 |
| `net/minecraft/core/NonNullList#iterator` | method | 1 |
| `net/minecraft/core/NonNullList#set` | method | 1 |
| `net/minecraft/core/NonNullList#size` | method | 1 |
| `net/minecraft/core/NonNullList#withSize` | method | 1 |
| `net/minecraft/core/Vec3i#ZERO` | field | 1 |
| `net/minecraft/core/component/DataComponentGetter#get` | method | 2 |
| `net/minecraft/core/component/DataComponentMap#keySet` | method | 1 |
| `net/minecraft/core/component/DataComponentMap$Builder#set` | method | 2 |
| `net/minecraft/core/component/DataComponentType#builder` | method | 1 |
| `net/minecraft/core/component/DataComponentType$Builder#build` | method | 1 |
| `net/minecraft/core/component/DataComponentType$Builder#networkSynchronized` | method | 1 |
| `net/minecraft/core/component/DataComponentType$Builder#persistent` | method | 1 |
| `net/minecraft/core/component/DataComponents#BLOCK_ENTITY_DATA` | field | 4 |
| `net/minecraft/core/component/DataComponents#CUSTOM_DATA` | field | 7 |
| `net/minecraft/core/component/DataComponents#CUSTOM_NAME` | field | 3 |
| `net/minecraft/core/component/DataComponents#MAX_STACK_SIZE` | field | 2 |
| `net/minecraft/core/component/DataComponents#TOOLTIP_DISPLAY` | field | 2 |
| `net/minecraft/core/component/DataComponents#TOOLTIP_STYLE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK_ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#CREATIVE_MODE_TAB` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#DATA_COMPONENT_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#MENU` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RECIPE_SERIALIZER` | field | 1 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 1 |
| `net/minecraft/core/registries/Registries#CREATIVE_MODE_TAB` | field | 1 |
| `net/minecraft/core/registries/Registries#ITEM` | field | 2 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 1 |
| `net/minecraft/nbt/CompoundTag#copy` | method | 1 |
| `net/minecraft/nbt/CompoundTag#get` | method | 1 |
| `net/minecraft/nbt/CompoundTag#keySet` | method | 1 |
| `net/minecraft/nbt/CompoundTag#put` | method | 1 |
| `net/minecraft/nbt/Tag#copy` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#readBlockPos` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#writeBlockPos` | method | 1 |
| `net/minecraft/network/chat/Component#empty` | method | 5 |
| `net/minecraft/network/chat/Component#equals` | method | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 3 |
| `net/minecraft/network/chat/Component#getVisualOrderText` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 3 |
| `net/minecraft/network/chat/Component#translatable` | method | 21 |
| `net/minecraft/network/chat/ComponentSerialization#CODEC` | field | 2 |
| `net/minecraft/network/chat/MutableComponent#getString` | method | 2 |
| `net/minecraft/network/chat/Style#EMPTY` | field | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#BOOL` | field | 2 |
| `net/minecraft/network/codec/ByteBufCodecs#INT` | field | 2 |
| `net/minecraft/network/codec/ByteBufCodecs#STRING_UTF8` | field | 1 |
| `net/minecraft/network/codec/StreamCodec#apply` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#composite` | method | 5 |
| `net/minecraft/network/codec/StreamCodec#map` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#of` | method | 6 |
| `net/minecraft/network/codec/StreamCodec#unit` | method | 1 |
| `net/minecraft/network/protocol/common/custom/CustomPacketPayload$Type#<init>` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundBlockEntityDataPacket#create` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundBlockEntityDataPacket#getTag` | method | 1 |
| `net/minecraft/resources/Identifier#CODEC` | field | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 1 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 11 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 5 |
| `net/minecraft/resources/Identifier#getPath` | method | 5 |
| `net/minecraft/resources/Identifier#parse` | method | 2 |
| `net/minecraft/resources/Identifier#tryParse` | method | 1 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 1 |
| `net/minecraft/resources/ResourceKey#create` | method | 3 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 1 |
| `net/minecraft/server/MinecraftServer#getPlayerList` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getBlockState` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 1 |
| `net/minecraft/server/level/ServerLevel#players` | method | 1 |
| `net/minecraft/server/level/ServerLevel#recipeAccess` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#blockPosition` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#getResourceOrThrow` | method | 1 |
| `net/minecraft/server/players/PlayerList#getPlayers` | method | 1 |
| `net/minecraft/sounds/SoundEvents#ITEM_PICKUP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WOOD_PLACE` | field | 1 |
| `net/minecraft/sounds/SoundSource#BLOCKS` | field | 1 |
| `net/minecraft/sounds/SoundSource#PLAYERS` | field | 1 |
| `net/minecraft/tags/BlockTags#DOES_NOT_BLOCK_HOPPERS` | field | 1 |
| `net/minecraft/tags/TagKey#create` | method | 1 |
| `net/minecraft/util/ARGB#color` | method | 1 |
| `net/minecraft/util/ExtraCodecs$LateBoundIdMapper#put` | method | 1 |
| `net/minecraft/util/FormattedCharSequence#forward` | method | 1 |
| `net/minecraft/util/Mth#positiveCeilDiv` | method | 1 |
| `net/minecraft/util/ProblemReporter#DISCARDING` | field | 7 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 1 |
| `net/minecraft/util/RandomSource#nextInt` | method | 1 |
| `net/minecraft/util/thread/BlockableEventLoop#execute` | method | 2 |
| `net/minecraft/world/Container#canPlaceItem` | method | 1 |
| `net/minecraft/world/Container#getItem` | method | 1 |
| `net/minecraft/world/Container#removeItem` | method | 1 |
| `net/minecraft/world/Container#stillValidBlockEntity` | method | 1 |
| `net/minecraft/world/InteractionHand#MAIN_HAND` | field | 3 |
| `net/minecraft/world/InteractionResult#CONSUME` | field | 4 |
| `net/minecraft/world/InteractionResult#FAIL` | field | 3 |
| `net/minecraft/world/InteractionResult#PASS` | field | 8 |
| `net/minecraft/world/InteractionResult#SUCCESS` | field | 7 |
| `net/minecraft/world/InteractionResult$Success#heldItemTransformedTo` | method | 2 |
| `net/minecraft/world/ItemStackWithSlot#<init>` | method | 1 |
| `net/minecraft/world/ItemStackWithSlot#CODEC` | field | 1 |
| `net/minecraft/world/ItemStackWithSlot#slot` | method | 1 |
| `net/minecraft/world/ItemStackWithSlot#stack` | method | 1 |
| `net/minecraft/world/SimpleContainer#<init>` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#<init>` | method | 1 |
| `net/minecraft/world/effect/MobEffects#SLOWNESS` | field | 1 |
| `net/minecraft/world/entity/Entity#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/EntitySelector#ENTITY_STILL_ALIVE` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlotGroup#MAINHAND` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#getOffhandItem` | method | 1 |
| `net/minecraft/world/entity/SlotAccess#set` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier#<init>` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/AttributeModifier$Operation#ADD_VALUE` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#ATTACK_DAMAGE` | field | 1 |
| `net/minecraft/world/entity/item/ItemEntity#<init>` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#discard` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getItem` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#push` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setItem` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getContainerSize` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#getDisplayName` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#getItem` | method | 4 |
| `net/minecraft/world/entity/player/Inventory#getSelectedItem` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#getSelectedSlot` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#player` | field | 2 |
| `net/minecraft/world/entity/player/Inventory#setItem` | method | 4 |
| `net/minecraft/world/entity/player/Player#addEffect` | method | 1 |
| `net/minecraft/world/entity/player/Player#blockPosition` | method | 1 |
| `net/minecraft/world/entity/player/Player#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/player/Player#getCooldowns` | method | 3 |
| `net/minecraft/world/entity/player/Player#getEyePosition` | method | 2 |
| `net/minecraft/world/entity/player/Player#getGameProfile` | method | 4 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 4 |
| `net/minecraft/world/entity/player/Player#getItemInHand` | method | 4 |
| `net/minecraft/world/entity/player/Player#getMainHandItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#getUUID` | method | 5 |
| `net/minecraft/world/entity/player/Player#isCreative` | method | 5 |
| `net/minecraft/world/entity/player/Player#isShiftKeyDown` | method | 4 |
| `net/minecraft/world/entity/player/Player#isSpectator` | method | 1 |
| `net/minecraft/world/entity/player/Player#level` | method | 3 |
| `net/minecraft/world/entity/player/Player#openMenu` | method | 1 |
| `net/minecraft/world/entity/player/Player#sendOverlayMessage` | method | 2 |
| `net/minecraft/world/entity/player/Player#setItemInHand` | method | 2 |
| `net/minecraft/world/entity/player/Player#tickCount` | field | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#<init>` | method | 2 |
| `net/minecraft/world/inventory/ClickAction#SECONDARY` | field | 1 |
| `net/minecraft/world/inventory/ContainerLevelAccess#create` | method | 1 |
| `net/minecraft/world/inventory/CraftingContainer#asCraftInput` | method | 1 |
| `net/minecraft/world/inventory/Slot#<init>` | method | 7 |
| `net/minecraft/world/inventory/Slot#allowModification` | method | 1 |
| `net/minecraft/world/inventory/Slot#container` | field | 1 |
| `net/minecraft/world/inventory/Slot#getContainerSlot` | method | 1 |
| `net/minecraft/world/inventory/Slot#getItem` | method | 4 |
| `net/minecraft/world/inventory/Slot#hasItem` | method | 2 |
| `net/minecraft/world/inventory/Slot#index` | field | 2 |
| `net/minecraft/world/inventory/Slot#onQuickCraft` | method | 3 |
| `net/minecraft/world/inventory/Slot#onTake` | method | 2 |
| `net/minecraft/world/inventory/Slot#safeTake` | method | 1 |
| `net/minecraft/world/inventory/Slot#set` | method | 2 |
| `net/minecraft/world/inventory/Slot#setChanged` | method | 2 |
| `net/minecraft/world/inventory/Slot#x` | field | 1 |
| `net/minecraft/world/inventory/Slot#y` | field | 1 |
| `net/minecraft/world/inventory/TransientCraftingContainer#<init>` | method | 1 |
| `net/minecraft/world/item/BlockItem#<init>` | method | 3 |
| `net/minecraft/world/item/BlockItem#appendHoverText` | method | 1 |
| `net/minecraft/world/item/BlockItem#getBlock` | method | 10 |
| `net/minecraft/world/item/BlockItem#getName` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#build` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#icon` | method | 1 |
| `net/minecraft/world/item/CreativeModeTab$Builder#title` | method | 1 |
| `net/minecraft/world/item/Item#appendHoverText` | method | 4 |
| `net/minecraft/world/item/Item#getDefaultInstance` | method | 1 |
| `net/minecraft/world/item/Item#getDefaultMaxStackSize` | method | 1 |
| `net/minecraft/world/item/Item#getDescriptionId` | method | 2 |
| `net/minecraft/world/item/Item#getName` | method | 2 |
| `net/minecraft/world/item/Item$Properties#<init>` | method | 1 |
| `net/minecraft/world/item/Item$Properties#attributes` | method | 1 |
| `net/minecraft/world/item/Item$Properties#overrideDescription` | method | 1 |
| `net/minecraft/world/item/Item$Properties#setId` | method | 1 |
| `net/minecraft/world/item/Item$Properties#useBlockDescriptionPrefix` | method | 1 |
| `net/minecraft/world/item/Item$TooltipContext#registries` | method | 2 |
| `net/minecraft/world/item/ItemCooldowns#addCooldown` | method | 3 |
| `net/minecraft/world/item/ItemCooldowns#getCooldownPercent` | method | 1 |
| `net/minecraft/world/item/ItemCooldowns#isOnCooldown` | method | 1 |
| `net/minecraft/world/item/ItemDisplayContext#GROUND` | field | 1 |
| `net/minecraft/world/item/ItemDisplayContext#GUI` | field | 1 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 21 |
| `net/minecraft/world/item/ItemStack#CODEC` | field | 7 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 37 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_CODEC` | field | 1 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_STREAM_CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStack#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/ItemStack#applyComponents` | method | 2 |
| `net/minecraft/world/item/ItemStack#copy` | method | 18 |
| `net/minecraft/world/item/ItemStack#copyWithCount` | method | 7 |
| `net/minecraft/world/item/ItemStack#get` | method | 11 |
| `net/minecraft/world/item/ItemStack#getBarColor` | method | 1 |
| `net/minecraft/world/item/ItemStack#getBarWidth` | method | 1 |
| `net/minecraft/world/item/ItemStack#getComponents` | method | 1 |
| `net/minecraft/world/item/ItemStack#getComponentsPatch` | method | 2 |
| `net/minecraft/world/item/ItemStack#getDisplayName` | method | 1 |
| `net/minecraft/world/item/ItemStack#getHoverName` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 41 |
| `net/minecraft/world/item/ItemStack#getMaxStackSize` | method | 4 |
| `net/minecraft/world/item/ItemStack#getOrDefault` | method | 12 |
| `net/minecraft/world/item/ItemStack#getStyledHoverName` | method | 1 |
| `net/minecraft/world/item/ItemStack#has` | method | 2 |
| `net/minecraft/world/item/ItemStack#hashItemAndComponents` | method | 2 |
| `net/minecraft/world/item/ItemStack#hashStackList` | method | 1 |
| `net/minecraft/world/item/ItemStack#isBarVisible` | method | 1 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 45 |
| `net/minecraft/world/item/ItemStack#isSameItem` | method | 3 |
| `net/minecraft/world/item/ItemStack#listMatches` | method | 1 |
| `net/minecraft/world/item/ItemStack#remove` | method | 3 |
| `net/minecraft/world/item/ItemStack#set` | method | 17 |
| `net/minecraft/world/item/ItemStack#setCount` | method | 18 |
| `net/minecraft/world/item/ItemStack#split` | method | 3 |
| `net/minecraft/world/item/ItemStack#toString` | method | 2 |
| `net/minecraft/world/item/ItemStack#typeHolder` | method | 1 |
| `net/minecraft/world/item/ItemUtils#onContainerDestroyed` | method | 1 |
| `net/minecraft/world/item/Items#BAMBOO` | field | 1 |
| `net/minecraft/world/item/Items#BRICK` | field | 1 |
| `net/minecraft/world/item/Items#CLAY_BALL` | field | 1 |
| `net/minecraft/world/item/Items#ENDER_PEARL` | field | 1 |
| `net/minecraft/world/item/Items#GLOWSTONE_DUST` | field | 1 |
| `net/minecraft/world/item/Items#IRON_NUGGET` | field | 1 |
| `net/minecraft/world/item/Items#MELON_SLICE` | field | 1 |
| `net/minecraft/world/item/Items#NETHER_BRICK` | field | 1 |
| `net/minecraft/world/item/Items#NETHER_WART` | field | 1 |
| `net/minecraft/world/item/Items#QUARTZ` | field | 1 |
| `net/minecraft/world/item/Items#SNOWBALL` | field | 1 |
| `net/minecraft/world/item/component/CustomData#EMPTY` | field | 3 |
| `net/minecraft/world/item/component/CustomData#copyTag` | method | 4 |
| `net/minecraft/world/item/component/CustomData#of` | method | 3 |
| `net/minecraft/world/item/component/ItemAttributeModifiers#builder` | method | 1 |
| `net/minecraft/world/item/component/ItemAttributeModifiers$Builder#add` | method | 1 |
| `net/minecraft/world/item/component/ItemAttributeModifiers$Builder#build` | method | 1 |
| `net/minecraft/world/item/component/TooltipDisplay#DEFAULT` | field | 2 |
| `net/minecraft/world/item/component/TypedEntityData#copyTagWithoutId` | method | 3 |
| `net/minecraft/world/item/component/TypedEntityData#of` | method | 2 |
| `net/minecraft/world/item/component/TypedEntityData#type` | method | 1 |
| `net/minecraft/world/item/context/BlockPlaceContext#getClickedPos` | method | 3 |
| `net/minecraft/world/item/context/BlockPlaceContext#getHorizontalDirection` | method | 3 |
| `net/minecraft/world/item/context/BlockPlaceContext#getItemInHand` | method | 2 |
| `net/minecraft/world/item/context/BlockPlaceContext#getLevel` | method | 3 |
| `net/minecraft/world/item/context/UseOnContext#getClickedPos` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getItemInHand` | method | 2 |
| `net/minecraft/world/item/context/UseOnContext#getLevel` | method | 1 |
| `net/minecraft/world/item/context/UseOnContext#getPlayer` | method | 2 |
| `net/minecraft/world/item/crafting/CraftingInput#getItem` | method | 5 |
| `net/minecraft/world/item/crafting/CraftingInput#size` | method | 3 |
| `net/minecraft/world/item/crafting/CraftingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#matches` | method | 1 |
| `net/minecraft/world/item/crafting/CustomRecipe#<init>` | method | 5 |
| `net/minecraft/world/item/crafting/Ingredient#items` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#of` | method | 3 |
| `net/minecraft/world/item/crafting/RecipeHolder#value` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager#recipes` | field | 1 |
| `net/minecraft/world/item/crafting/RecipeMap#byType` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeMap#getRecipesFor` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeSerializer#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeType#CRAFTING` | field | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipe#pattern` | field | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipe#result` | field | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#ingredients` | method | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#matches` | method | 2 |
| `net/minecraft/world/item/crafting/ShapedRecipePattern#of` | method | 2 |
| `net/minecraft/world/level/BlockGetter#getBlockEntity` | method | 2 |
| `net/minecraft/world/level/BlockGetter#getBlockState` | method | 1 |
| `net/minecraft/world/level/ChunkPos#pack` | method | 2 |
| `net/minecraft/world/level/ClipContext#<init>` | method | 2 |
| `net/minecraft/world/level/ClipContext$Block#OUTLINE` | field | 2 |
| `net/minecraft/world/level/ClipContext$Fluid#NONE` | field | 2 |
| `net/minecraft/world/level/Level#addFreshEntity` | method | 2 |
| `net/minecraft/world/level/Level#clip` | method | 2 |
| `net/minecraft/world/level/Level#gameEvent` | method | 1 |
| `net/minecraft/world/level/Level#getBlockEntity` | method | 7 |
| `net/minecraft/world/level/Level#getBlockState` | method | 8 |
| `net/minecraft/world/level/Level#getBlockTicks` | method | 2 |
| `net/minecraft/world/level/Level#getEntitiesOfClass` | method | 1 |
| `net/minecraft/world/level/Level#getGameTime` | method | 3 |
| `net/minecraft/world/level/Level#getRandom` | method | 1 |
| `net/minecraft/world/level/Level#getWorldBorder` | method | 1 |
| `net/minecraft/world/level/Level#isLoaded` | method | 3 |
| `net/minecraft/world/level/Level#levelEvent` | method | 1 |
| `net/minecraft/world/level/Level#playSound` | method | 1 |
| `net/minecraft/world/level/Level#registryAccess` | method | 5 |
| `net/minecraft/world/level/Level#sendBlockUpdated` | method | 2 |
| `net/minecraft/world/level/Level#setBlockAndUpdate` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#playSound` | method | 1 |
| `net/minecraft/world/level/LevelReader#getBlockState` | method | 2 |
| `net/minecraft/world/level/block/Block#asItem` | method | 1 |
| `net/minecraft/world/level/block/Block#box` | method | 4 |
| `net/minecraft/world/level/block/Block#byItem` | method | 4 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 8 |
| `net/minecraft/world/level/block/Block#getId` | method | 1 |
| `net/minecraft/world/level/block/Block#getStateDefinition` | method | 2 |
| `net/minecraft/world/level/block/Block#hasDynamicShape` | method | 1 |
| `net/minecraft/world/level/block/Block#isShapeFullBlock` | method | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 4 |
| `net/minecraft/world/level/block/Blocks#BAMBOO_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BRICKS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CLAY` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GLOWSTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MELON` | field | 1 |
| `net/minecraft/world/level/block/Blocks#NETHER_BRICKS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#NETHER_WART_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#QUARTZ_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SNOW_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/DoubleBlockCombiner$BlockType#FIRST` | field | 1 |
| `net/minecraft/world/level/block/DoubleBlockCombiner$BlockType#SECOND` | field | 1 |
| `net/minecraft/world/level/block/FaceAttachedHorizontalDirectionalBlock#<init>` | method | 1 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#<init>` | method | 3 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#playerWillDestroy` | method | 1 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#setPlacedBy` | method | 1 |
| `net/minecraft/world/level/block/Mirror#getRotation` | method | 1 |
| `net/minecraft/world/level/block/Rotation#rotate` | method | 1 |
| `net/minecraft/world/level/block/SoundType#STONE` | field | 1 |
| `net/minecraft/world/level/block/SoundType#WOOD` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#getBlockPos` | method | 3 |
| `net/minecraft/world/level/block/entity/BlockEntity#getBlockState` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#getLevel` | method | 2 |
| `net/minecraft/world/level/block/entity/BlockEntity#getType` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#isRemoved` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#loadAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#loadCustomOnly` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#saveAdditional` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#saveWithoutMetadata` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#setChanged` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityType#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityType$BlockEntitySupplier#create` | method | 5 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#air` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#isRedstoneConductor` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#isSuffocating` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#isViewBlocking` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#noOcclusion` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#of` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#setId` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#sound` | method | 1 |
| `net/minecraft/world/level/block/state/BlockBehaviour$Properties#strength` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 22 |
| `net/minecraft/world/level/block/state/BlockState#getMenuProvider` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getShape` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 15 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#isCollisionShapeFullBlock` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#rotate` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#setValue` | method | 10 |
| `net/minecraft/world/level/block/state/BlockState#updateNeighbourShapes` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition#any` | method | 3 |
| `net/minecraft/world/level/block/state/StateDefinition#getPossibleStates` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition#getProperty` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition$Builder#add` | method | 13 |
| `net/minecraft/world/level/block/state/properties/AttachFace#CEILING` | field | 2 |
| `net/minecraft/world/level/block/state/properties/AttachFace#FLOOR` | field | 2 |
| `net/minecraft/world/level/block/state/properties/AttachFace#WALL` | field | 2 |
| `net/minecraft/world/level/block/state/properties/AttachFace#ordinal` | method | 2 |
| `net/minecraft/world/level/block/state/properties/AttachFace#values` | method | 1 |
| `net/minecraft/world/level/block/state/properties/BlockSetType#OAK` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockSetType#buttonClickOff` | method | 1 |
| `net/minecraft/world/level/block/state/properties/BlockSetType#buttonClickOn` | method | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#HORIZONTAL_FACING` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#POWERED` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BooleanProperty#create` | method | 2 |
| `net/minecraft/world/level/block/state/properties/EnumProperty#create` | method | 4 |
| `net/minecraft/world/level/block/state/properties/IntegerProperty#create` | method | 1 |
| `net/minecraft/world/level/block/state/properties/Property#<init>` | method | 1 |
| `net/minecraft/world/level/block/state/properties/Property#equals` | method | 1 |
| `net/minecraft/world/level/block/state/properties/Property#generateHashCode` | method | 1 |
| `net/minecraft/world/level/block/state/properties/Property#getValue` | method | 1 |
| `net/minecraft/world/level/border/WorldBorder#isWithinBounds` | method | 1 |
| `net/minecraft/world/level/gameevent/GameEvent#BLOCK_ACTIVATE` | field | 1 |
| `net/minecraft/world/level/gameevent/GameEvent#BLOCK_DEACTIVATE` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#fromCorners` | method | 1 |
| `net/minecraft/world/level/storage/TagValueInput#create` | method | 7 |
| `net/minecraft/world/level/storage/TagValueOutput#buildResult` | method | 3 |
| `net/minecraft/world/level/storage/TagValueOutput#createWithContext` | method | 3 |
| `net/minecraft/world/level/storage/ValueInput#child` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput#childrenList` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#childrenListOrEmpty` | method | 2 |
| `net/minecraft/world/level/storage/ValueInput#getBooleanOr` | method | 4 |
| `net/minecraft/world/level/storage/ValueInput#getInt` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#getIntOr` | method | 6 |
| `net/minecraft/world/level/storage/ValueInput#getString` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#list` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput#read` | method | 5 |
| `net/minecraft/world/level/storage/ValueInput$TypedInputList#iterator` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput$ValueInputList#isEmpty` | method | 1 |
| `net/minecraft/world/level/storage/ValueInput$ValueInputList#iterator` | method | 3 |
| `net/minecraft/world/level/storage/ValueOutput#child` | method | 2 |
| `net/minecraft/world/level/storage/ValueOutput#childrenList` | method | 3 |
| `net/minecraft/world/level/storage/ValueOutput#discard` | method | 3 |
| `net/minecraft/world/level/storage/ValueOutput#list` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#putBoolean` | method | 4 |
| `net/minecraft/world/level/storage/ValueOutput#putByte` | method | 2 |
| `net/minecraft/world/level/storage/ValueOutput#putInt` | method | 6 |
| `net/minecraft/world/level/storage/ValueOutput#putString` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput#store` | method | 5 |
| `net/minecraft/world/level/storage/ValueOutput#storeNullable` | method | 2 |
| `net/minecraft/world/level/storage/ValueOutput$TypedOutputList#add` | method | 1 |
| `net/minecraft/world/level/storage/ValueOutput$ValueOutputList#addChild` | method | 3 |
| `net/minecraft/world/level/storage/loot/LootParams$Builder#getOptionalParameter` | method | 4 |
| `net/minecraft/world/level/storage/loot/parameters/LootContextParams#BLOCK_ENTITY` | field | 4 |
| `net/minecraft/world/phys/AABB#<init>` | method | 2 |
| `net/minecraft/world/phys/AABB#encapsulatingFullBlocks` | method | 2 |
| `net/minecraft/world/phys/AABB#getXsize` | method | 1 |
| `net/minecraft/world/phys/AABB#getYsize` | method | 1 |
| `net/minecraft/world/phys/AABB#maxX` | field | 1 |
| `net/minecraft/world/phys/AABB#maxY` | field | 2 |
| `net/minecraft/world/phys/AABB#minX` | field | 2 |
| `net/minecraft/world/phys/AABB#minY` | field | 2 |
| `net/minecraft/world/phys/AABB#minZ` | field | 2 |
| `net/minecraft/world/phys/AABB#move` | method | 2 |
| `net/minecraft/world/phys/AABB#of` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getBlockPos` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getDirection` | method | 3 |
| `net/minecraft/world/phys/BlockHitResult#getLocation` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getType` | method | 2 |
| `net/minecraft/world/phys/HitResult$Type#BLOCK` | field | 2 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 1 |
| `net/minecraft/world/phys/Vec3#add` | method | 2 |
| `net/minecraft/world/phys/Vec3#atCenterOf` | method | 3 |
| `net/minecraft/world/phys/Vec3#distanceTo` | method | 2 |
| `net/minecraft/world/phys/Vec3#scale` | method | 2 |
| `net/minecraft/world/phys/Vec3#x` | field | 2 |
| `net/minecraft/world/phys/Vec3#y` | field | 2 |
| `net/minecraft/world/phys/Vec3#z` | field | 2 |
| `net/minecraft/world/phys/shapes/BooleanOp#ONLY_FIRST` | field | 1 |
| `net/minecraft/world/phys/shapes/Shapes#block` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#box` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#empty` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#join` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#or` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#toAabbs` | method | 1 |
| `net/minecraft/world/ticks/LevelTickAccess#hasScheduledTick` | method | 2 |
| `net/minecraft/world/ticks/LevelTicks#allContainers` | field | 2 |

</details>

