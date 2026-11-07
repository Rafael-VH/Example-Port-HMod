# Horizon port report: CreativeCore

`creativecore` 2.14.24 · CreativeCore_FABRIC_v2.14.24_mc26.3.jar · 838 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 9 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 3 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 22 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 12 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 10 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: team.creative.creativecore.CreativeCore
- `client`: team.creative.creativecore.client.CreativeCoreClient
- `modmenu`: team.creative.creativecore.client.ModMenuImpl

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| CommandRegistrationCallback.EVENT | `CreativeFabricLoader` | hook `command` | needs-translation | Server commands. Horizon's command hook is client-side. |
| ClientCommandRegistrationCallback.EVENT | `CreativeFabricLoader` | hook `command` | auto | The Fabric API layer's command engine runs the Brigadier tree: literals, integer/long/float/double/bool/string arguments with bounds, executes; requires and suggests are left out. |
| ClientTickEvents.END_CLIENT_TICK | `CreativeFabricLoader` | hook `tick` | auto | The Fabric API layer (jfabricapi.rs) runs START_CLIENT_TICK, then the level ticks (START/END_LEVEL_TICK, while in a world), then END_CLIENT_TICK, from Horizon's tick hook, in the window's Game only. |
| HudElementRegistry.addLast | `CreativeFabricLoader` | hook `hud` | auto |  |
| ServerTickEvents.END_LEVEL_TICK | `CreativeFabricLoader` | hook `tick` | auto | The tick hook runs in every Game, the server's too: run server logic only when `!self.native_window()`. |
| ServerTickEvents.START_LEVEL_TICK | `CreativeFabricLoader` | hook `tick` | auto | The tick hook runs in every Game, the server's too: run server logic only when `!self.native_window()`. |
| ClientLifecycleEvents.CLIENT_STARTED | `CreativeFabricLoader` | hook `frame` | auto | CLIENT_STARTED: the Fabric API layer runs it on the window's first frame. CLIENT_STOPPING has no hook in Horizon a46ce68 (c9d474d has native_world_lifecycle). |
| ClientTickEvents.START_CLIENT_TICK | `CreativeCoreClient` | hook `tick` | auto | The Fabric API layer (jfabricapi.rs) runs START_CLIENT_TICK, then the level ticks (START/END_LEVEL_TICK, while in a world), then END_CLIENT_TICK, from Horizon's tick hook, in the window's Game only. |
| KeyMappingHelper.registerKeyMapping | `CreativeFabricClientLoader` | hook `key` | auto | Mojang-named variant of KeyBindingHelper. |
| ScreenEvents.AFTER_INIT | `CreativeFabricClientLoader` | – | unsupported | No screen hook in Horizon yet. |
| PayloadTypeRegistry.clientboundPlay | `CreativeNetwork` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.register | `CreativeNetwork` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.serverboundPlay | `CreativeNetwork` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| ServerPlayNetworking.registerGlobalReceiver | `CreativeNetwork` | – | unsupported | Custom packets aren't supported. |
| ClientPlayNetworking.send | `CreativeNetwork` | – | unsupported | Custom packets aren't supported. |
| ServerPlayNetworking.send | `CreativeNetwork` | – | unsupported | Custom packets aren't supported. |
| ServerPlayNetworking.createClientboundPacket | `CreativeNetwork` | – | unsupported | Custom packets aren't supported. |
| ClientPlayNetworking.registerGlobalReceiver | `CreativeNetworkClient` | – | unsupported | Custom packets aren't supported. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(HEAD) on AbstractContainerScreen.mouseReleased | `AbstractContainerScreenMixin.handleWhenYouReleaseMouse` | HEAD | needs-mapping | No Horizon function known for AbstractContainerScreen.mouseReleased: run `match`. |
| @Accessor on ArgumentTypeInfos.BY_CLASS | `ArgumentTypeInfosAccessor.getByClass` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ArgumentTypeInfos.BY_CLASS: run `match`. |
| @Redirect(INVOKE) on ClientConfigurationPacketListenerImpl.handleConfigurationFinished at Lnet/minecraft/network/RegistryFriendlyByteBuf;decorator(Lnet/minecraft/core/RegistryAccess;)Ljava/util/function/Function; | `ClientConfigurationPacketListenerImplMixin.handleConfigurationFinished` | WRAP_OPERATION | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. No Horizon function known for ClientConfigurationPacketListenerImpl.handleConfigurationFinished: run `match`. The call it hooks (RegistryFriendlyByteBuf.decorator) isn't mapped to a Rust function yet. |
| @Inject(TAIL) on ComponentSerialization.bootstrap | `ComponentSerializationMixin.register` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ComponentSerialization.bootstrap: run `match`. |
| @Inject(INVOKE) on Connection.setupInboundProtocol at Lnet/minecraft/network/UnconfiguredPipelineHandler;setupInboundProtocol(Lnet/minecraft/network/ProtocolInfo;)Lnet/minecraft/network/UnconfiguredPipelineHandler$InboundConfigurationTask; | `ConnectionMixin.setupInboundProtocol` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for Connection.setupInboundProtocol: run `match`. The call it hooks (UnconfiguredPipelineHandler.setupInboundProtocol) isn't mapped to a Rust function yet. |
| @Invoker on FilePackResources.callGetPathFromLocation | `FilePackResourcesAccessor.callGetPathFromLocation` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for FilePackResources.callGetPathFromLocation: run `match`. |
| @Accessor on FilePackResources.zipFileAccess | `FilePackResourcesAccessor.getZipFileAccess` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for FilePackResources.zipFileAccess: run `match`. |
| @Accessor on FixedPathPackResources.pathsForType | `FixedPathPackResourcesAccessor.getPathsForType` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for FixedPathPackResources.pathsForType: run `match`. |
| @Inject(HEAD) on GameRenderer.render | `GameRendererMixin.render` | HEAD on hook `frame` | auto | Target mapping is approximate. |
| @WrapOperation(NEW) on GuiGraphicsExtractor.<init> at (I)Lorg/joml/Matrix3x2fStack; | `GuiGraphicsExtractorMixin.modifyStackSize` | WRAP_OPERATION | needs-mapping | No Horizon function known for GuiGraphicsExtractor.<init>: run `match`. |
| @WrapOperation(MIXINEXTRAS:EXPRESSION) on LivingEntity.completeUsingItem | `LivingEntityMixin.hook` | WRAP_OPERATION | needs-mapping | No Horizon function known for LivingEntity.completeUsingItem: run `match`. |
| @Invoker on ModNioPackResources.callGetFilename | `ModNioResourcePackAccessor.callGetFilename` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ModNioPackResources.callGetFilename: run `match`. |
| @Invoker on ModNioPackResources.callGetPath | `ModNioResourcePackAccessor.callGetPath` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ModNioPackResources.callGetPath: run `match`. |
| @Accessor on MouseHandler.lastHandleMovementTime | `MouseHandlerAccessor.getLastHandleMovementTime` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for MouseHandler.lastHandleMovementTime: run `match`. |
| @Accessor on PathPackResources.root | `PathPackResourcesAccessor.getRoot` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for PathPackResources.root: run `match`. |
| @WrapOperation(INVOKE) on PlayerList.placeNewPlayer at Lnet/minecraft/network/protocol/UnboundProtocol;bind(Ljava/util/function/Function;Ljava/lang/Object;)Lnet/minecraft/network/ProtocolInfo; | `PlayerListMixin.bind` | WRAP_OPERATION on horizon_client::lan::hub::Hub::place_new_player | needs-mapping | The call it hooks (UnboundProtocol.bind) isn't mapped to a Rust function yet. |
| mixin into ScissorStack with no injectors (interfaces or @Shadow only) | `ScissorStackMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Accessor on ServerCommonPacketListenerImpl.connection | `ServerCommonPacketListenerImplAccessor.getConnection` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ServerCommonPacketListenerImpl.connection: run `match`. |
| @Redirect(INVOKE) on ServerConfigurationPacketListenerImpl.handleConfigurationFinished at Lnet/minecraft/network/RegistryFriendlyByteBuf;decorator(Lnet/minecraft/core/RegistryAccess;)Ljava/util/function/Function; | `ServerConfigurationPacketListenerImplMixin.handleConfigurationFinished` | WRAP_OPERATION | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. No Horizon function known for ServerConfigurationPacketListenerImpl.handleConfigurationFinished: run `match`. The call it hooks (RegistryFriendlyByteBuf.decorator) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on Shapes.joinUnoptimized at Lnet/minecraft/world/phys/shapes/Shapes;createIndexMerger(ILit/unimi/dsi/fastutil/doubles/DoubleList;Lit/unimi/dsi/fastutil/doubles/DoubleList;ZZ)Lnet/minecraft/world/phys/shapes/IndexMerger; | `ShapesMixin.joinUnoptimized` | BEFORE_CALL on horizon_core::geometry::shape::join::VoxelShape::join_unoptimized | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. The call it hooks (Shapes.createIndexMerger) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on Shapes.joinIsNotEmpty at Lnet/minecraft/world/phys/shapes/Shapes;createIndexMerger(ILit/unimi/dsi/fastutil/doubles/DoubleList;Lit/unimi/dsi/fastutil/doubles/DoubleList;ZZ)Lnet/minecraft/world/phys/shapes/IndexMerger; | `ShapesMixin.joinIsNotEmpty` | BEFORE_CALL on horizon_core::geometry::shape::join::VoxelShape::join_is_not_empty | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. The call it hooks (Shapes.createIndexMerger) isn't mapped to a Rust function yet. |
| @Invoker on SharedZipFileAccess.callGetOrCreateZipFile | `SharedZipFileAccessAccessor.callGetOrCreateZipFile` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for SharedZipFileAccess.callGetOrCreateZipFile: run `match`. |
| @Accessor on StringSplitter.widthProvider | `StringSplitterAccessor.getWidthProvider` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for StringSplitter.widthProvider: run `match`. |
| @Accessor on VoxelShape.shape | `VoxelShapeAccessor.setShape` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for VoxelShape.shape: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends SimplePreparableReloadListener | `CreativeCoreClient$1` | needs-translation | No mapping for this superclass yet. |
| extends AbstractContainerScreen | `ContainerScreenIntegration` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `GuiScreenIntegration` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends AbstractTickableSoundInstance | `EntitySound` | needs-translation | No mapping for this superclass yet. |
| extends AbstractContainerMenu | `ContainerIntegration` | unsupported | Menus aren't supported. |
| extends BundlePacket | `BundlePacketWrapper` | needs-translation | No mapping for this superclass yet. |
| extends RegistryFriendlyByteBuf | `CreativeByteBuf` | needs-translation | No mapping for this superclass yet. |
| extends DiscreteVoxelShape | `BoxesVoxelShape$1` | needs-translation | No mapping for this superclass yet. |
| extends SliceShape | `BoxesVoxelShape` | needs-translation | No mapping for this superclass yet. |
| extends StringSplitter | `AdvancedStringSplitter` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `creativecore.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| registers blocks or items in code | `CreativeCore` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| registers blocks or items in code | `CreativeFabricLoader$2` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| 13 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

45 of 682 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `com/mojang/blaze3d/platform/Window#calculateScale` | horizon_client_input::window::calculate_scale (suggested) |
| `net/minecraft/client/KeyboardHandler#setClipboard` | horizon_client::game::debug_keys::Game<'_>::set_clipboard (suggested) |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#hasControlDown` | horizon_client::game::Game<'_>::has_control_down (suggested) |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/OptionInstance#set` | horizon_client_render::options_file::OptionsFile::set (suggested) |
| `net/minecraft/client/gui/Font#plainSubstrByWidth` | horizon_client_render::font::Font::fit (suggested) |
| `net/minecraft/client/gui/Font#split` | horizon_client_render::font::Font::split_formatted (suggested) |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/gui/GuiGraphicsExtractor#itemDecorations` | horizon_client_render::hud_overlays::item_decorations (suggested) |
| `net/minecraft/client/gui/GuiGraphicsExtractor#requestCursor` | horizon_client_render::widgets::request_cursor (suggested) |
| `net/minecraft/client/gui/GuiGraphicsExtractor#textHighlight` | horizon_client_render::hud::HudBatch::text_highlight (suggested) |
| `net/minecraft/client/renderer/GameRenderer#render` | hook `frame` |
| `net/minecraft/client/resources/sounds/SimpleSoundInstance#forUI` | horizon_client_render::sound_engine::SoundInstance::for_ui (suggested) |
| `net/minecraft/core/Direction#getOpposite` | horizon_client::mobs::shulker::opposite (suggested) |
| `net/minecraft/core/Registry#register` | content |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/network/chat/Style#applyTo` | horizon_client_core::text::Style::inherit (suggested) |
| `net/minecraft/server/players/PlayerList#placeNewPlayer` | horizon_client::lan::hub::Hub::place_new_player (suggested) |
| `net/minecraft/util/Mth#ceil` | horizon_client_render::hud_layout::ceil (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/util/Mth#lerp` | horizon_world::stepped_interpolation::lerp (suggested) |
| `net/minecraft/util/StringUtil#filterText` | horizon_client_input::clipboard::filter_text (suggested) |
| `net/minecraft/util/StringUtil#isAllowedChatCharacter` | horizon_client_input::clipboard::is_allowed_chat_character (suggested) |
| `net/minecraft/world/Containers#dropItemStack` | horizon_world::level::removal::drop_item_stack (suggested) |
| `net/minecraft/world/inventory/Slot#remove` | horizon_client::menu::Menu::remove (suggested) |
| `net/minecraft/world/item/ItemStack#getCount` | horizon_world::loot::LootStack::get_count (suggested) |
| `net/minecraft/world/item/ItemStack#isSameItemSameComponents` | horizon_items::item_data::ItemStack::same_item_same_components (suggested) |
| `net/minecraft/world/item/ItemStack#matches` | horizon_client_render::vault::matches (suggested) |
| `net/minecraft/world/item/ItemStack#shrink` | horizon_client::game::dispenser::shrink (suggested) |
| `net/minecraft/world/level/Level#destroyBlock` | horizon_world::level::update::Sim<'_, L>::destroy_block (suggested) |
| `net/minecraft/world/level/Level#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/world/level/Level#getSkyDarken` | horizon_world::level::sky_darken (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/level/Level#removeBlock` | horizon_world::level::update::Sim<'_, L>::remove_block (suggested) |
| `net/minecraft/world/level/Level#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/world/level/block/state/BlockState#handleNeighborChanged` | horizon_world::level::Behaviours::handle_neighbor_changed (suggested) |
| `net/minecraft/world/phys/AABB#intersects` | horizon_core::geometry::Aabb::intersects_corners (suggested) |
| `net/minecraft/world/phys/shapes/Shapes#joinIsNotEmpty` | horizon_core::geometry::shape::join::VoxelShape::join_is_not_empty (suggested) |
| `net/minecraft/world/phys/shapes/Shapes#joinUnoptimized` | horizon_core::geometry::shape::join::VoxelShape::join_unoptimized (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/Blaze3D#getTime` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderTarget#getDepthTexture` | method | 2 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledHeight` | method | 3 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledWidth` | method | 3 |
| `com/mojang/blaze3d/platform/Window#getHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getWidth` | method | 1 |
| `com/mojang/blaze3d/platform/Window#setGuiScale` | method | 1 |
| `com/mojang/blaze3d/platform/cursor/CursorTypes#IBEAM` | field | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getDevice` | method | 2 |
| `com/mojang/blaze3d/vertex/DefaultVertexFormat#BLOCK` | field | 1 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#pose` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertex` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertexWith2DPose` | method | 6 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setColor` | method | 7 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setNormal` | method | 1 |
| `net/fabricmc/fabric/impl/resource/pack/ModNioPackResources#callGetFilename` | method | 1 |
| `net/fabricmc/fabric/impl/resource/pack/ModNioPackResources#callGetPath` | method | 1 |
| `net/minecraft/ChatFormatting#BLUE` | field | 2 |
| `net/minecraft/ChatFormatting#BOLD` | field | 1 |
| `net/minecraft/ChatFormatting#GRAY` | field | 1 |
| `net/minecraft/ChatFormatting#GREEN` | field | 3 |
| `net/minecraft/ChatFormatting#RED` | field | 3 |
| `net/minecraft/ChatFormatting#RESET` | field | 1 |
| `net/minecraft/ChatFormatting#WHITE` | field | 2 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 6 |
| `net/minecraft/client/ComponentCollector#<init>` | method | 1 |
| `net/minecraft/client/ComponentCollector#append` | method | 2 |
| `net/minecraft/client/ComponentCollector#getResult` | method | 1 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 2 |
| `net/minecraft/client/KeyMapping#matches` | method | 2 |
| `net/minecraft/client/KeyMapping$Category#register` | method | 1 |
| `net/minecraft/client/KeyboardHandler#getClipboard` | method | 1 |
| `net/minecraft/client/Minecraft#execute` | method | 1 |
| `net/minecraft/client/Minecraft#gameMode` | field | 1 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 2 |
| `net/minecraft/client/Minecraft#getCameraEntity` | method | 1 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 2 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 2 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 1 |
| `net/minecraft/client/Minecraft#getWindow` | method | 3 |
| `net/minecraft/client/Minecraft#gui` | field | 5 |
| `net/minecraft/client/Minecraft#hasShiftDown` | method | 4 |
| `net/minecraft/client/Minecraft#isEnforceUnicode` | method | 1 |
| `net/minecraft/client/Minecraft#isPaused` | method | 1 |
| `net/minecraft/client/Minecraft#keyboardHandler` | field | 1 |
| `net/minecraft/client/Minecraft#level` | field | 1 |
| `net/minecraft/client/Minecraft#mouseHandler` | field | 1 |
| `net/minecraft/client/Minecraft#onTextInputFocusChange` | method | 1 |
| `net/minecraft/client/Minecraft#options` | field | 3 |
| `net/minecraft/client/MouseHandler#lastHandleMovementTime` | method | 1 |
| `net/minecraft/client/OptionInstance#get` | method | 1 |
| `net/minecraft/client/Options#guiScale` | method | 1 |
| `net/minecraft/client/Options#keyDrop` | field | 1 |
| `net/minecraft/client/Options#keyInventory` | field | 1 |
| `net/minecraft/client/StringSplitter#<init>` | method | 1 |
| `net/minecraft/client/StringSplitter#stringWidth` | method | 1 |
| `net/minecraft/client/StringSplitter#widthProvider` | method | 1 |
| `net/minecraft/client/StringSplitter$WidthProvider#getWidth` | method | 2 |
| `net/minecraft/client/gui/Font#getSplitter` | method | 2 |
| `net/minecraft/client/gui/Gui#screen` | method | 3 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#<init>` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiHeight` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiWidth` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#item` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 17 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#scissorStack` | field | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 7 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#tooltip` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor$ScissorStack#peek` | method | 1 |
| `net/minecraft/client/gui/components/TextCursorUtils#extractAppendCursor` | method | 1 |
| `net/minecraft/client/gui/components/TextCursorUtils#extractInsertCursor` | method | 1 |
| `net/minecraft/client/gui/components/TextCursorUtils#isCursorVisible` | method | 1 |
| `net/minecraft/client/gui/components/events/GuiEventListener#mouseDragged` | method | 1 |
| `net/minecraft/client/gui/narration/NarratableEntry$NarrationPriority#NONE` | field | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#<init>` | method | 1 |
| `net/minecraft/client/gui/render/TextureSetup#noTexture` | method | 1 |
| `net/minecraft/client/gui/screens/MenuScreens#register` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#height` | field | 2 |
| `net/minecraft/client/gui/screens/Screen#mouseReleased` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#rebuildWidgets` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#resize` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#width` | field | 2 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#charTyped` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#keyPressed` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#keyReleased` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#mouseReleased` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#rebuildWidgets` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent#create` | method | 2 |
| `net/minecraft/client/gui/screens/inventory/tooltip/DefaultTooltipPositioner#INSTANCE` | field | 1 |
| `net/minecraft/client/input/CharacterEvent#codepoint` | method | 1 |
| `net/minecraft/client/input/CharacterEvent#isAllowedChatCharacter` | method | 1 |
| `net/minecraft/client/input/KeyEvent#hasControlDown` | method | 1 |
| `net/minecraft/client/input/KeyEvent#hasControlDownWithQuirk` | method | 1 |
| `net/minecraft/client/input/KeyEvent#hasShiftDown` | method | 1 |
| `net/minecraft/client/input/KeyEvent#isConfirmation` | method | 1 |
| `net/minecraft/client/input/KeyEvent#isCopy` | method | 1 |
| `net/minecraft/client/input/KeyEvent#isCut` | method | 1 |
| `net/minecraft/client/input/KeyEvent#isEscape` | method | 1 |
| `net/minecraft/client/input/KeyEvent#isPaste` | method | 1 |
| `net/minecraft/client/input/KeyEvent#isSelectAll` | method | 1 |
| `net/minecraft/client/input/KeyEvent#keycode` | method | 1 |
| `net/minecraft/client/input/KeyEvent#shortcutKey` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#button` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#buttonInfo` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#x` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#y` | method | 1 |
| `net/minecraft/client/input/MouseButtonInfo#button` | method | 17 |
| `net/minecraft/client/input/MouseButtonInfo#equals` | method | 1 |
| `net/minecraft/client/input/MouseButtonInfo#hasShiftDown` | method | 2 |
| `net/minecraft/client/multiplayer/ClientCommonPacketListenerImpl#<init>` | method | 1 |
| `net/minecraft/client/multiplayer/ClientConfigurationPacketListenerImpl#handleConfigurationFinished` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#registryAccess` | method | 1 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#getPlayerMode` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#registryAccess` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#mainRenderTarget` | method | 2 |
| `net/minecraft/client/renderer/RenderPipelines#GUI` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 1 |
| `net/minecraft/client/renderer/state/gui/GuiRenderState#addGuiElement` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiRenderState#addText` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#<init>` | method | 1 |
| `net/minecraft/client/resources/language/I18n#get` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#materialInfo` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#isTinted` | method | 1 |
| `net/minecraft/client/resources/sounds/AbstractTickableSoundInstance#<init>` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#createUnseededRandom` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#play` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getPlayerOrException` | method | 1 |
| `net/minecraft/commands/Commands#getDispatcher` | method | 1 |
| `net/minecraft/commands/Commands#literal` | method | 1 |
| `net/minecraft/commands/SharedSuggestionProvider#suggest` | method | 1 |
| `net/minecraft/commands/arguments/EntityArgument#parse` | method | 1 |
| `net/minecraft/commands/arguments/EntityArgument#players` | method | 1 |
| `net/minecraft/commands/arguments/selector/EntitySelector#findPlayers` | method | 1 |
| `net/minecraft/commands/synchronization/ArgumentTypeInfos#BY_CLASS` | method | 1 |
| `net/minecraft/commands/synchronization/SingletonArgumentInfo#contextFree` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 6 |
| `net/minecraft/core/BlockPos#CODEC` | field | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 2 |
| `net/minecraft/core/BlockPos#equals` | method | 2 |
| `net/minecraft/core/BlockPos#get` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 9 |
| `net/minecraft/core/BlockPos#getY` | method | 9 |
| `net/minecraft/core/BlockPos#getZ` | method | 9 |
| `net/minecraft/core/BlockPos#relative` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setX` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setY` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setZ` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 10 |
| `net/minecraft/core/DefaultedRegistry#getTagOrEmpty` | method | 4 |
| `net/minecraft/core/DefaultedRegistry#getTags` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#getValue` | method | 6 |
| `net/minecraft/core/DefaultedRegistry#iterator` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 2 |
| `net/minecraft/core/Direction#EAST` | field | 2 |
| `net/minecraft/core/Direction#NORTH` | field | 2 |
| `net/minecraft/core/Direction#SOUTH` | field | 2 |
| `net/minecraft/core/Direction#UP` | field | 2 |
| `net/minecraft/core/Direction#WEST` | field | 2 |
| `net/minecraft/core/Direction#getAxis` | method | 2 |
| `net/minecraft/core/Direction#ordinal` | method | 2 |
| `net/minecraft/core/Direction#values` | method | 2 |
| `net/minecraft/core/Direction$Axis#X` | field | 6 |
| `net/minecraft/core/Direction$Axis#Y` | field | 6 |
| `net/minecraft/core/Direction$Axis#Z` | field | 6 |
| `net/minecraft/core/Direction$Axis#ordinal` | method | 10 |
| `net/minecraft/core/Direction$Axis#values` | method | 5 |
| `net/minecraft/core/Holder#unwrapKey` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 6 |
| `net/minecraft/core/Holder$Reference#getRegisteredName` | method | 2 |
| `net/minecraft/core/Holder$Reference#is` | method | 4 |
| `net/minecraft/core/Holder$Reference#unwrapKey` | method | 2 |
| `net/minecraft/core/Holder$Reference#value` | method | 3 |
| `net/minecraft/core/HolderLookup$Provider#lookup` | method | 2 |
| `net/minecraft/core/HolderLookup$RegistryLookup#get` | method | 2 |
| `net/minecraft/core/HolderLookup$RegistryLookup#getOrThrow` | method | 1 |
| `net/minecraft/core/HolderSet$Named#getRandomElement` | method | 1 |
| `net/minecraft/core/HolderSet$Named#unwrapKey` | method | 2 |
| `net/minecraft/core/NonNullList#iterator` | method | 1 |
| `net/minecraft/core/Registry#get` | method | 1 |
| `net/minecraft/core/Registry#getKey` | method | 2 |
| `net/minecraft/core/Registry#getOrThrow` | method | 1 |
| `net/minecraft/core/Registry#getRandomElementOf` | method | 1 |
| `net/minecraft/core/Registry#getTags` | method | 2 |
| `net/minecraft/core/Registry#getValue` | method | 5 |
| `net/minecraft/core/Registry#key` | method | 5 |
| `net/minecraft/core/Registry#keySet` | method | 2 |
| `net/minecraft/core/Registry#wrapAsHolder` | method | 2 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 1 |
| `net/minecraft/core/SectionPos#getX` | method | 2 |
| `net/minecraft/core/SectionPos#getY` | method | 2 |
| `net/minecraft/core/SectionPos#getZ` | method | 2 |
| `net/minecraft/core/SectionPos#of` | method | 3 |
| `net/minecraft/core/Vec3i#<init>` | method | 8 |
| `net/minecraft/core/Vec3i#ZERO` | field | 1 |
| `net/minecraft/core/Vec3i#equals` | method | 1 |
| `net/minecraft/core/Vec3i#getX` | method | 12 |
| `net/minecraft/core/Vec3i#getY` | method | 12 |
| `net/minecraft/core/Vec3i#getZ` | method | 12 |
| `net/minecraft/core/component/DataComponentMap#iterator` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch#STREAM_CODEC` | field | 1 |
| `net/minecraft/core/component/DataComponents#COOKING_FUEL` | field | 1 |
| `net/minecraft/core/component/DataComponents#ITEM_NAME` | field | 2 |
| `net/minecraft/core/component/TypedDataComponent#type` | method | 1 |
| `net/minecraft/core/component/TypedDataComponent#value` | method | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#DATA_COMPONENT_TYPE` | field | 3 |
| `net/minecraft/core/registries/BuiltInRegistries#MENU` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#MOB_EFFECT` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#SOUND_EVENT` | field | 2 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 3 |
| `net/minecraft/core/registries/Registries#ITEM` | field | 4 |
| `net/minecraft/locale/Language#getInstance` | method | 3 |
| `net/minecraft/locale/Language#getOrDefault` | method | 1 |
| `net/minecraft/locale/Language#getVisualOrder` | method | 2 |
| `net/minecraft/locale/Language#has` | method | 1 |
| `net/minecraft/nbt/ByteArrayTag#<init>` | method | 1 |
| `net/minecraft/nbt/ByteArrayTag#getAsByteArray` | method | 1 |
| `net/minecraft/nbt/ByteArrayTag#size` | method | 1 |
| `net/minecraft/nbt/ByteTag#asBoolean` | method | 1 |
| `net/minecraft/nbt/ByteTag#asByte` | method | 1 |
| `net/minecraft/nbt/ByteTag#byteValue` | method | 1 |
| `net/minecraft/nbt/ByteTag#valueOf` | method | 3 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 30 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 5 |
| `net/minecraft/nbt/CompoundTag#get` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getBooleanOr` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getCompoundOrEmpty` | method | 4 |
| `net/minecraft/nbt/CompoundTag#getIntArray` | method | 5 |
| `net/minecraft/nbt/CompoundTag#getIntOr` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getListOrEmpty` | method | 9 |
| `net/minecraft/nbt/CompoundTag#getLongArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getString` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getStringOr` | method | 16 |
| `net/minecraft/nbt/CompoundTag#isEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#keySet` | method | 4 |
| `net/minecraft/nbt/CompoundTag#put` | method | 11 |
| `net/minecraft/nbt/CompoundTag#putBoolean` | method | 2 |
| `net/minecraft/nbt/CompoundTag#putInt` | method | 3 |
| `net/minecraft/nbt/CompoundTag#putIntArray` | method | 5 |
| `net/minecraft/nbt/CompoundTag#putLongArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putString` | method | 16 |
| `net/minecraft/nbt/CompoundTag#read` | method | 5 |
| `net/minecraft/nbt/CompoundTag#store` | method | 5 |
| `net/minecraft/nbt/CompoundTag#toString` | method | 2 |
| `net/minecraft/nbt/DoubleTag#doubleValue` | method | 1 |
| `net/minecraft/nbt/DoubleTag#valueOf` | method | 1 |
| `net/minecraft/nbt/EndTag#INSTANCE` | field | 2 |
| `net/minecraft/nbt/FloatTag#floatValue` | method | 1 |
| `net/minecraft/nbt/FloatTag#valueOf` | method | 1 |
| `net/minecraft/nbt/IntArrayTag#<init>` | method | 1 |
| `net/minecraft/nbt/IntArrayTag#getAsIntArray` | method | 1 |
| `net/minecraft/nbt/IntArrayTag#size` | method | 1 |
| `net/minecraft/nbt/IntTag#asInt` | method | 1 |
| `net/minecraft/nbt/IntTag#intValue` | method | 3 |
| `net/minecraft/nbt/IntTag#valueOf` | method | 2 |
| `net/minecraft/nbt/ListTag#<init>` | method | 11 |
| `net/minecraft/nbt/ListTag#add` | method | 11 |
| `net/minecraft/nbt/ListTag#get` | method | 2 |
| `net/minecraft/nbt/ListTag#getCompoundOrEmpty` | method | 5 |
| `net/minecraft/nbt/ListTag#getFirst` | method | 1 |
| `net/minecraft/nbt/ListTag#getListOrEmpty` | method | 1 |
| `net/minecraft/nbt/ListTag#getLongArray` | method | 2 |
| `net/minecraft/nbt/ListTag#getStringOr` | method | 3 |
| `net/minecraft/nbt/ListTag#size` | method | 11 |
| `net/minecraft/nbt/LongArrayTag#<init>` | method | 3 |
| `net/minecraft/nbt/LongArrayTag#getAsLongArray` | method | 1 |
| `net/minecraft/nbt/LongArrayTag#size` | method | 1 |
| `net/minecraft/nbt/LongTag#longValue` | method | 2 |
| `net/minecraft/nbt/LongTag#valueOf` | method | 1 |
| `net/minecraft/nbt/NbtAccounter#unlimitedHeap` | method | 3 |
| `net/minecraft/nbt/NbtOps#INSTANCE` | field | 4 |
| `net/minecraft/nbt/ShortTag#shortValue` | method | 1 |
| `net/minecraft/nbt/ShortTag#valueOf` | method | 1 |
| `net/minecraft/nbt/StringTag#asString` | method | 1 |
| `net/minecraft/nbt/StringTag#valueOf` | method | 4 |
| `net/minecraft/nbt/Tag#asString` | method | 1 |
| `net/minecraft/nbt/TagParser#parseCompoundFully` | method | 2 |
| `net/minecraft/network/Connection#setupInboundProtocol` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#readNullable` | method | 2 |
| `net/minecraft/network/FriendlyByteBuf#writeNullable` | method | 2 |
| `net/minecraft/network/ProtocolInfo#codec` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#<init>` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#decorator` | method | 2 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readBlockPos` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readBoolean` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readByte` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readDouble` | method | 6 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readFloat` | method | 5 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readIdentifier` | method | 4 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readInt` | method | 4 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readLong` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readNbt` | method | 3 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readRegistryKey` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readShort` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readUtf` | method | 4 |
| `net/minecraft/network/RegistryFriendlyByteBuf#registryAccess` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeBlockPos` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeBoolean` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeByte` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeDouble` | method | 6 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeFloat` | method | 5 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeIdentifier` | method | 4 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeInt` | method | 4 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeLong` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeNbt` | method | 3 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeShort` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeUtf` | method | 4 |
| `net/minecraft/network/UnconfiguredPipelineHandler#setupInboundProtocol` | method | 1 |
| `net/minecraft/network/chat/Component#copy` | method | 1 |
| `net/minecraft/network/chat/Component#empty` | method | 2 |
| `net/minecraft/network/chat/Component#getContents` | method | 3 |
| `net/minecraft/network/chat/Component#getSiblings` | method | 3 |
| `net/minecraft/network/chat/Component#getString` | method | 3 |
| `net/minecraft/network/chat/Component#getStyle` | method | 2 |
| `net/minecraft/network/chat/Component#getVisualOrderText` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 35 |
| `net/minecraft/network/chat/Component#translatable` | method | 21 |
| `net/minecraft/network/chat/Component#visit` | method | 2 |
| `net/minecraft/network/chat/ComponentContents#visit` | method | 1 |
| `net/minecraft/network/chat/ComponentSerialization#CODEC` | field | 1 |
| `net/minecraft/network/chat/ComponentSerialization#STREAM_CODEC` | field | 1 |
| `net/minecraft/network/chat/ComponentSerialization#bootstrap` | method | 1 |
| `net/minecraft/network/chat/FormattedText#getString` | method | 1 |
| `net/minecraft/network/chat/FormattedText#of` | method | 2 |
| `net/minecraft/network/chat/FormattedText#visit` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#create` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#getContents` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#getSiblings` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#getString` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#setStyle` | method | 2 |
| `net/minecraft/network/chat/Style#EMPTY` | field | 6 |
| `net/minecraft/network/chat/Style#withColor` | method | 1 |
| `net/minecraft/network/chat/contents/PlainTextContents$LiteralContents#text` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#decode` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#encode` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#of` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#ofMember` | method | 1 |
| `net/minecraft/network/protocol/BundlePacket#<init>` | method | 1 |
| `net/minecraft/network/protocol/BundlePacket#subPackets` | method | 1 |
| `net/minecraft/network/protocol/Packet#handle` | method | 1 |
| `net/minecraft/network/protocol/PacketFlow#CLIENTBOUND` | field | 2 |
| `net/minecraft/network/protocol/PacketFlow#SERVERBOUND` | field | 1 |
| `net/minecraft/network/protocol/UnboundProtocol#bind` | method | 2 |
| `net/minecraft/network/protocol/common/custom/CustomPacketPayload$Type#<init>` | method | 1 |
| `net/minecraft/network/protocol/common/custom/CustomPacketPayload$Type#id` | method | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 3 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 2 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 6 |
| `net/minecraft/resources/Identifier#getPath` | method | 5 |
| `net/minecraft/resources/Identifier#hashCode` | method | 1 |
| `net/minecraft/resources/Identifier#parse` | method | 19 |
| `net/minecraft/resources/Identifier#toString` | method | 23 |
| `net/minecraft/resources/Identifier#tryBuild` | method | 6 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 2 |
| `net/minecraft/resources/ResourceKey#create` | method | 4 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 3 |
| `net/minecraft/resources/ResourceKey#registry` | method | 1 |
| `net/minecraft/resources/ResourceKey#registryKey` | method | 1 |
| `net/minecraft/server/MinecraftServer#createCommandSourceStack` | method | 1 |
| `net/minecraft/server/MinecraftServer#execute` | method | 1 |
| `net/minecraft/server/MinecraftServer#getCommands` | method | 1 |
| `net/minecraft/server/MinecraftServer#getPlayerList` | method | 1 |
| `net/minecraft/server/MinecraftServer#isSingleplayer` | method | 1 |
| `net/minecraft/server/MinecraftServer#isSingleplayerOwner` | method | 1 |
| `net/minecraft/server/MinecraftServer#operatorUserPermissions` | method | 1 |
| `net/minecraft/server/MinecraftServer#registryAccess` | method | 1 |
| `net/minecraft/server/level/ChunkMap#getPlayers` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#chunkMap` | field | 1 |
| `net/minecraft/server/level/ServerChunkCache#sendToTrackingPlayers` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#sendToTrackingPlayersAndSelf` | method | 1 |
| `net/minecraft/server/level/ServerLevel#dimension` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getChunkSource` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#closeContainer` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#containerMenu` | field | 1 |
| `net/minecraft/server/level/ServerPlayer#gameMode` | field | 1 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#nameAndId` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#openMenu` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#permissions` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#registryAccess` | method | 4 |
| `net/minecraft/server/level/ServerPlayerGameMode#getGameModeForPlayer` | method | 1 |
| `net/minecraft/server/network/ServerCommonPacketListenerImpl#<init>` | method | 1 |
| `net/minecraft/server/network/ServerCommonPacketListenerImpl#connection` | method | 1 |
| `net/minecraft/server/network/ServerConfigurationPacketListenerImpl#handleConfigurationFinished` | method | 1 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#getPlayer` | method | 1 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#player` | field | 1 |
| `net/minecraft/server/packs/FilePackResources#callGetPathFromLocation` | method | 1 |
| `net/minecraft/server/packs/FilePackResources#zipFileAccess` | method | 1 |
| `net/minecraft/server/packs/FilePackResources$SharedZipFileAccess#callGetOrCreateZipFile` | method | 1 |
| `net/minecraft/server/packs/FixedPathPackResources#pathsForType` | method | 1 |
| `net/minecraft/server/packs/PackType#CLIENT_RESOURCES` | field | 1 |
| `net/minecraft/server/packs/PackType#getDirectory` | method | 1 |
| `net/minecraft/server/packs/PathPackResources#root` | method | 1 |
| `net/minecraft/server/packs/PathPackResources#validatePath` | method | 1 |
| `net/minecraft/server/packs/VanillaPackResources#fullResources` | method | 1 |
| `net/minecraft/server/packs/resources/ReloadableResourceManager#registerReloadListener` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#source` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 1 |
| `net/minecraft/server/packs/resources/SimplePreparableReloadListener#<init>` | method | 1 |
| `net/minecraft/server/permissions/LevelBasedPermissionSet#level` | method | 1 |
| `net/minecraft/server/permissions/PermissionLevel#id` | method | 1 |
| `net/minecraft/server/permissions/PermissionSet#hasPermission` | method | 1 |
| `net/minecraft/server/permissions/Permissions#COMMANDS_GAMEMASTER` | field | 1 |
| `net/minecraft/server/players/PlayerList#getOps` | method | 1 |
| `net/minecraft/server/players/ServerOpList#get` | method | 1 |
| `net/minecraft/server/players/ServerOpListEntry#permissions` | method | 1 |
| `net/minecraft/sounds/SoundEvents#GLOW_ITEM_FRAME_ROTATE_ITEM` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ITEM_FRAME_ADD_ITEM` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ITEM_FRAME_REMOVE_ITEM` | field | 1 |
| `net/minecraft/sounds/SoundEvents#UI_BUTTON_CLICK` | field | 18 |
| `net/minecraft/tags/TagKey#create` | method | 7 |
| `net/minecraft/tags/TagKey#isFor` | method | 1 |
| `net/minecraft/tags/TagKey#location` | method | 9 |
| `net/minecraft/util/ARGB#alpha` | method | 1 |
| `net/minecraft/util/ExtraCodecs$LateBoundIdMapper#put` | method | 1 |
| `net/minecraft/util/FileUtil#decomposePath` | method | 1 |
| `net/minecraft/util/FileUtil#resolvePath` | method | 1 |
| `net/minecraft/util/FormattedCharSequence#forward` | method | 1 |
| `net/minecraft/util/Mth#sqrt` | method | 1 |
| `net/minecraft/util/Prediction#PREDICTED` | field | 3 |
| `net/minecraft/util/RandomSource#create` | method | 1 |
| `net/minecraft/util/StringDecomposer#iterateFormatted` | method | 3 |
| `net/minecraft/util/Unit#INSTANCE` | field | 1 |
| `net/minecraft/util/Util#getMillis` | method | 1 |
| `net/minecraft/util/Util#offsetByCodepoints` | method | 1 |
| `net/minecraft/world/Container#getContainerSize` | method | 6 |
| `net/minecraft/world/Container#getItem` | method | 4 |
| `net/minecraft/world/Container#getMaxStackSize` | method | 1 |
| `net/minecraft/world/Container#setChanged` | method | 2 |
| `net/minecraft/world/Container#setItem` | method | 2 |
| `net/minecraft/world/InteractionHand#MAIN_HAND` | field | 1 |
| `net/minecraft/world/InteractionHand#OFF_HAND` | field | 1 |
| `net/minecraft/world/SimpleContainer#<init>` | method | 3 |
| `net/minecraft/world/SimpleContainer#getContainerSize` | method | 1 |
| `net/minecraft/world/SimpleContainer#getItem` | method | 1 |
| `net/minecraft/world/SimpleContainer#setItem` | method | 2 |
| `net/minecraft/world/SimpleMenuProvider#<init>` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#<init>` | method | 1 |
| `net/minecraft/world/entity/Entity#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/Entity#getEyePosition` | method | 1 |
| `net/minecraft/world/entity/Entity#level` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#completeUsingItem` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getSelectedSlot` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#player` | field | 1 |
| `net/minecraft/world/entity/player/Inventory#setItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#addItem` | method | 2 |
| `net/minecraft/world/entity/player/Player#blockInteractionRange` | method | 1 |
| `net/minecraft/world/entity/player/Player#blockPosition` | method | 1 |
| `net/minecraft/world/entity/player/Player#containerMenu` | field | 3 |
| `net/minecraft/world/entity/player/Player#drop` | method | 3 |
| `net/minecraft/world/entity/player/Player#gameMode` | method | 1 |
| `net/minecraft/world/entity/player/Player#getAttachedOrCreate` | method | 1 |
| `net/minecraft/world/entity/player/Player#getDirection` | method | 1 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 3 |
| `net/minecraft/world/entity/player/Player#getItemInHand` | method | 1 |
| `net/minecraft/world/entity/player/Player#getXRot` | method | 1 |
| `net/minecraft/world/entity/player/Player#inventoryMenu` | field | 1 |
| `net/minecraft/world/entity/player/Player#isCreative` | method | 2 |
| `net/minecraft/world/entity/player/Player#level` | method | 9 |
| `net/minecraft/world/entity/player/Player#nameAndId` | method | 1 |
| `net/minecraft/world/entity/player/Player#registryAccess` | method | 3 |
| `net/minecraft/world/entity/player/Player#setAttached` | method | 1 |
| `net/minecraft/world/flag/FeatureFlags#VANILLA_SET` | field | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#<init>` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#broadcastChanges` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#canItemQuickReplace` | method | 2 |
| `net/minecraft/world/inventory/AbstractContainerMenu#removed` | method | 1 |
| `net/minecraft/world/inventory/ClickAction#PRIMARY` | field | 1 |
| `net/minecraft/world/inventory/ClickAction#SECONDARY` | field | 1 |
| `net/minecraft/world/inventory/InventoryMenu#getItems` | method | 1 |
| `net/minecraft/world/inventory/MenuType#<init>` | method | 1 |
| `net/minecraft/world/inventory/Slot#<init>` | method | 2 |
| `net/minecraft/world/inventory/Slot#getContainerSlot` | method | 4 |
| `net/minecraft/world/inventory/Slot#getItem` | method | 5 |
| `net/minecraft/world/inventory/Slot#getMaxStackSize` | method | 5 |
| `net/minecraft/world/inventory/Slot#hasItem` | method | 4 |
| `net/minecraft/world/inventory/Slot#mayPickup` | method | 3 |
| `net/minecraft/world/inventory/Slot#mayPlace` | method | 4 |
| `net/minecraft/world/inventory/Slot#onTake` | method | 2 |
| `net/minecraft/world/inventory/Slot#set` | method | 3 |
| `net/minecraft/world/item/Item#builtInRegistryHolder` | method | 2 |
| `net/minecraft/world/item/Item#getDescriptionId` | method | 1 |
| `net/minecraft/world/item/Item#getId` | method | 1 |
| `net/minecraft/world/item/Item$Properties#<init>` | method | 1 |
| `net/minecraft/world/item/Item$Properties#setId` | method | 1 |
| `net/minecraft/world/item/Item$TooltipContext#EMPTY` | field | 1 |
| `net/minecraft/world/item/Item$TooltipContext#of` | method | 1 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 11 |
| `net/minecraft/world/item/ItemStack#CODEC` | field | 4 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 12 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_CODEC` | field | 3 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/ItemStack#copy` | method | 8 |
| `net/minecraft/world/item/ItemStack#get` | method | 2 |
| `net/minecraft/world/item/ItemStack#getComponents` | method | 1 |
| `net/minecraft/world/item/ItemStack#getDamageValue` | method | 1 |
| `net/minecraft/world/item/ItemStack#getDisplayName` | method | 2 |
| `net/minecraft/world/item/ItemStack#getHoverName` | method | 2 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 13 |
| `net/minecraft/world/item/ItemStack#getMaxStackSize` | method | 3 |
| `net/minecraft/world/item/ItemStack#getTooltipLines` | method | 2 |
| `net/minecraft/world/item/ItemStack#grow` | method | 3 |
| `net/minecraft/world/item/ItemStack#has` | method | 1 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 10 |
| `net/minecraft/world/item/ItemStack#overrideOtherStackedOnMe` | method | 1 |
| `net/minecraft/world/item/ItemStack#overrideStackedOnOther` | method | 1 |
| `net/minecraft/world/item/ItemStack#set` | method | 2 |
| `net/minecraft/world/item/ItemStack#setCount` | method | 4 |
| `net/minecraft/world/item/Items#AIR` | field | 1 |
| `net/minecraft/world/item/Items#COAL` | field | 1 |
| `net/minecraft/world/item/TooltipFlag$Default#NORMAL` | field | 2 |
| `net/minecraft/world/item/component/ItemContainerContents#fromItems` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getBlockState` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getFluidState` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getHeight` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getMinY` | method | 1 |
| `net/minecraft/world/level/ChunkPos#<init>` | method | 2 |
| `net/minecraft/world/level/ChunkPos#x` | method | 2 |
| `net/minecraft/world/level/ChunkPos#z` | method | 2 |
| `net/minecraft/world/level/GameType#ADVENTURE` | field | 2 |
| `net/minecraft/world/level/GameType#CREATIVE` | field | 2 |
| `net/minecraft/world/level/GameType#SURVIVAL` | field | 2 |
| `net/minecraft/world/level/GameType#byId` | method | 1 |
| `net/minecraft/world/level/GameType#getId` | method | 1 |
| `net/minecraft/world/level/GameType#getShortDisplayName` | method | 2 |
| `net/minecraft/world/level/GameType#values` | method | 1 |
| `net/minecraft/world/level/Level#addParticle` | method | 1 |
| `net/minecraft/world/level/Level#dimensionType` | method | 1 |
| `net/minecraft/world/level/Level#enabledFeatures` | method | 1 |
| `net/minecraft/world/level/Level#environmentAttributes` | method | 1 |
| `net/minecraft/world/level/Level#gameEvent` | method | 1 |
| `net/minecraft/world/level/Level#getBiomeManager` | method | 1 |
| `net/minecraft/world/level/Level#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/Level#getBlockState` | method | 3 |
| `net/minecraft/world/level/Level#getBlockTicks` | method | 1 |
| `net/minecraft/world/level/Level#getChunk` | method | 1 |
| `net/minecraft/world/level/Level#getChunkAt` | method | 1 |
| `net/minecraft/world/level/Level#getChunkSource` | method | 2 |
| `net/minecraft/world/level/Level#getEntities` | method | 2 |
| `net/minecraft/world/level/Level#getFluidState` | method | 1 |
| `net/minecraft/world/level/Level#getFluidTicks` | method | 1 |
| `net/minecraft/world/level/Level#getLevelData` | method | 1 |
| `net/minecraft/world/level/Level#getLightEngine` | method | 1 |
| `net/minecraft/world/level/Level#getRandom` | method | 1 |
| `net/minecraft/world/level/Level#getSeaLevel` | method | 1 |
| `net/minecraft/world/level/Level#getServer` | method | 3 |
| `net/minecraft/world/level/Level#getUncachedNoiseBiome` | method | 1 |
| `net/minecraft/world/level/Level#getWorldBorder` | method | 1 |
| `net/minecraft/world/level/Level#isFluidAtPosition` | method | 1 |
| `net/minecraft/world/level/Level#isStateAtPosition` | method | 1 |
| `net/minecraft/world/level/Level#levelEvent` | method | 1 |
| `net/minecraft/world/level/Level#nextSubTickCount` | method | 1 |
| `net/minecraft/world/level/Level#playSound` | method | 3 |
| `net/minecraft/world/level/Level#players` | method | 1 |
| `net/minecraft/world/level/Level#registryAccess` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#getBlockState` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#isClientSide` | method | 2 |
| `net/minecraft/world/level/block/Block#builtInRegistryHolder` | method | 5 |
| `net/minecraft/world/level/block/Block#byItem` | method | 6 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 2 |
| `net/minecraft/world/level/block/Block#getFriction` | method | 1 |
| `net/minecraft/world/level/block/Block#getId` | method | 1 |
| `net/minecraft/world/level/block/Block#getName` | method | 1 |
| `net/minecraft/world/level/block/Block#stateById` | method | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BARRIER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DIRT` | field | 1 |
| `net/minecraft/world/level/block/Mirror#FRONT_BACK` | field | 1 |
| `net/minecraft/world/level/block/Mirror#LEFT_RIGHT` | field | 1 |
| `net/minecraft/world/level/block/Mirror#ordinal` | method | 2 |
| `net/minecraft/world/level/block/Mirror#values` | method | 1 |
| `net/minecraft/world/level/block/Rotation#CLOCKWISE_180` | field | 2 |
| `net/minecraft/world/level/block/Rotation#CLOCKWISE_90` | field | 1 |
| `net/minecraft/world/level/block/Rotation#COUNTERCLOCKWISE_90` | field | 1 |
| `net/minecraft/world/level/block/Rotation#ordinal` | method | 2 |
| `net/minecraft/world/level/block/Rotation#values` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#getBlockPos` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#getLevel` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 4 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#hasProperty` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#isSolid` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getLevel` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getPos` | method | 1 |
| `net/minecraft/world/level/material/Fluid#getTickDelay` | method | 1 |
| `net/minecraft/world/phys/AABB#<init>` | method | 3 |
| `net/minecraft/world/phys/AABB#contains` | method | 2 |
| `net/minecraft/world/phys/AABB#max` | method | 1 |
| `net/minecraft/world/phys/AABB#maxX` | field | 8 |
| `net/minecraft/world/phys/AABB#maxY` | field | 8 |
| `net/minecraft/world/phys/AABB#maxZ` | field | 8 |
| `net/minecraft/world/phys/AABB#min` | method | 1 |
| `net/minecraft/world/phys/AABB#minX` | field | 8 |
| `net/minecraft/world/phys/AABB#minY` | field | 8 |
| `net/minecraft/world/phys/AABB#minZ` | field | 8 |
| `net/minecraft/world/phys/AABB#setMaxX` | method | 1 |
| `net/minecraft/world/phys/AABB#setMaxY` | method | 1 |
| `net/minecraft/world/phys/AABB#setMaxZ` | method | 1 |
| `net/minecraft/world/phys/AABB#setMinX` | method | 1 |
| `net/minecraft/world/phys/AABB#setMinY` | method | 1 |
| `net/minecraft/world/phys/AABB#setMinZ` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#<init>` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/phys/BlockHitResult#getLocation` | method | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 10 |
| `net/minecraft/world/phys/Vec3#add` | method | 1 |
| `net/minecraft/world/phys/Vec3#distanceToSqr` | method | 3 |
| `net/minecraft/world/phys/Vec3#get` | method | 1 |
| `net/minecraft/world/phys/Vec3#x` | field | 10 |
| `net/minecraft/world/phys/Vec3#y` | field | 10 |
| `net/minecraft/world/phys/Vec3#z` | field | 10 |
| `net/minecraft/world/phys/shapes/BooleanOp#AND` | field | 1 |
| `net/minecraft/world/phys/shapes/BooleanOp#FIRST` | field | 1 |
| `net/minecraft/world/phys/shapes/BooleanOp#NOT_AND` | field | 1 |
| `net/minecraft/world/phys/shapes/BooleanOp#NOT_SAME` | field | 1 |
| `net/minecraft/world/phys/shapes/BooleanOp#ONLY_FIRST` | field | 1 |
| `net/minecraft/world/phys/shapes/BooleanOp#ONLY_SECOND` | field | 1 |
| `net/minecraft/world/phys/shapes/BooleanOp#OR` | field | 1 |
| `net/minecraft/world/phys/shapes/BooleanOp#SAME` | field | 1 |
| `net/minecraft/world/phys/shapes/BooleanOp#SECOND` | field | 1 |
| `net/minecraft/world/phys/shapes/BooleanOp#getClass` | method | 2 |
| `net/minecraft/world/phys/shapes/DiscreteVoxelShape#<init>` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#box` | method | 1 |
| `net/minecraft/world/phys/shapes/Shapes#createIndexMerger` | method | 2 |
| `net/minecraft/world/phys/shapes/Shapes$DoubleLineConsumer#consume` | method | 1 |
| `net/minecraft/world/phys/shapes/SliceShape#<init>` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#isEmpty` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#shape` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#toAabbs` | method | 2 |

</details>

