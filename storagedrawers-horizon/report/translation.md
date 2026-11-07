# Java translated to Rust

Translated 41 of 2076 methods (1 more is done another way in Horizon) and 76 of 492 static fields to Rust (5 of those call the game, through Horizon's helpers); 1 of 1 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 1709 | `IItemRepository.insertItem(2)`: its class isn't translated (it's an interface) |
| Other Java this translator doesn't handle yet | 226 | `IDrawerCapability.getCapability(2)`: it has no body |
| Uses Minecraft classes (needs Horizon's game API) | 65 | `BlockFramingTable.getTableDirection(2)`: it uses Minecraft's `Direction` |
| Uses Java collections other than List (Map, Set...) | 18 | `LockAttribute.getBitfield(1)`: it uses `EnumSet` (java.util) |
| Calls the game where Horizon has no helper yet | 9 | `StandardDrawerGroup.DrawerData.getStackSize(1)`: it calls Minecraft's ItemStack.getMaxStackSize (no Horizon helper for it yet) |
| Uses inheritance (extends, implements, instanceof) | 3 | `BlockEntityFramingTable.isItemValidMaterial(1)`: `instanceof` |
| Uses null | 3 | `LocalIntegrationRegistry.isModLoaded(1)`: a nullable object kept in a changing static |
| Done another way in Horizon | 1 | `StorageDrawersClient.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook |
| Uses Fabric API | 1 | `CommonEvents.init(0)`: it uses Fabric's `AttackBlockCallback`.EVENT |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| com/jaquadro/minecraft/storagedrawers/StorageDrawers.java:36 | `init` | `on_init`, called from the hook, without: the call to `init(..)`: it uses ModCommonConfig.INSTANCE, which isn't translated (it uses ModCommonConfig, which isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config))); the call to `init(..)`: it uses ModClientConfig.INSTANCE, which isn't translated (it uses ModClientConfig, which isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config))); the call to `ModBlocks.init(..)`: it calls ModBlocks.init, which isn't translated; the call to `ModItems.init(..)`: it calls ModItems.init, which isn't translated; the call to `ModCreativeTabs.init(..)`: it calls ModCreativeTabs.init, which isn't translated; the call to `ModBlockEntities.init(..)`: it calls ModBlockEntities.init, which isn't translated; the call to `ModContainers.init(..)`: it calls ModContainers.init, which isn't translated; the call to `ModDataComponents.init(..)`: it calls ModDataComponents.init, which isn't translated; the call to `ModRecipes.init(..)`: it calls ModRecipes.init, which isn't translated; the call to `INSTANCE.init(..)`: it uses ModNetworking.INSTANCE, which isn't translated (it uses ModNetworking, which isn't translated (it implements `ChameleonInit` (com.texelsaurus.minecraft.chameleon.api))); the call to `CommonEvents.init(..)`: it calls CommonEvents.init, which isn't translated; the call to `PlatformCapabilities.initHandlers(..)`: it calls PlatformCapabilities.initHandlers, which isn't translated; the call to `LocalIntegrationRegistry.initialize(..)`: it calls LocalIntegrationRegistry.initialize, which isn't translated; the call to `init(..)`: it calls LocalIntegrationRegistry.instance, which isn't translated; the call to `postInit(..)`: it calls LocalIntegrationRegistry.instance, which isn't translated |

## Every class


### `IDrawerCapability`: it's an interface

- [ ] `IDrawerCapability.getCapability(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/capabilities/IDrawerCapability.java:7)

### `IDrawerCapabilityProvider`: it's an interface

- [ ] `IDrawerCapabilityProvider.getCapability(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/api/capabilities/IDrawerCapabilityProvider.java:6)

### `IItemHandler`: it's an interface

- [ ] `IItemHandler.getSlots(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemHandler.java:6)
- [ ] `IItemHandler.getStackInSlot(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemHandler.java:8)
- [ ] `IItemHandler.insertItem(3)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemHandler.java:10)
- [ ] `IItemHandler.extractItem(3)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemHandler.java:12)
- [ ] `IItemHandler.getSlotLimit(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemHandler.java:14)
- [ ] `IItemHandler.isItemValid(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemHandler.java:16)

### `IItemRepository`: it's an interface

- [ ] `IItemRepository.getAllItems(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:9)
- [ ] `IItemRepository.insertItem(3)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:12)
- [ ] `IItemRepository.insertItem(2)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:15)
- [ ] `IItemRepository.extractItem(4)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:20)
- [ ] `IItemRepository.extractItem(3)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:23)
- [ ] `IItemRepository.getStoredItemCount(2)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:28)
- [ ] `IItemRepository.getStoredItemCount(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:33)
- [ ] `IItemRepository.getRemainingItemCapacity(2)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:37)
- [ ] `IItemRepository.getRemainingItemCapacity(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:44)
- [ ] `IItemRepository.getItemCapacity(2)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:48)
- [ ] `IItemRepository.getItemCapacity(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:53)

### `IItemRepository.DefaultPredicate`: it's an interface


### `IItemRepository.ItemRecord`: it uses Minecraft's `ItemStack`

- [ ] `IItemRepository.ItemRecord.ItemRecord(2)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java:65)

### `IDrawerConfig`: it's an interface

- [ ] `IDrawerConfig.getUnitsPerSlot(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/config/IDrawerConfig.java:4)

### `FrameMaterial`: its constants carry values

- [ ] `FrameMaterial.FrameMaterial(2)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/framing/FrameMaterial.java:14)
- [ ] `FrameMaterial.getSerializedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/framing/FrameMaterial.java:19)
- [ ] `FrameMaterial.getTagKey(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/framing/FrameMaterial.java:24)

### `IFramedBlock`: it's an interface

- [ ] `IFramedBlock.getFramedBlockEntity(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedBlock.java:11)
- [ ] `IFramedBlock.supportsFrameMaterial(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedBlock.java:13)
- [ ] `IFramedBlock.getStateDefinition(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedBlock.java:15)

### `IFramedBlockEntity`: it's an interface

- [ ] `IFramedBlockEntity.material(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedBlockEntity.java:4)

### `IFramedMaterials`: it's an interface

- [ ] `IFramedMaterials.getHostBlock(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedMaterials.java:7)
- [ ] `IFramedMaterials.setHostBlock(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedMaterials.java:10)
- [ ] `IFramedMaterials.getMaterial(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedMaterials.java:12)
- [ ] `IFramedMaterials.setMaterial(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedMaterials.java:15)

### `IFramedSourceBlock`: it's an interface

- [ ] `IFramedSourceBlock.makeFramedItem(4)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedSourceBlock.java:6)

### `IStorageDrawersApi`: it's an interface


### `IRenderRegistry`: it's an interface

- [ ] `IRenderRegistry.registerPreLabelRenderHandler(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/registry/IRenderRegistry.java:6)

### `IWailaRegistry`: it's an interface

- [ ] `IWailaRegistry.registerTooltipHandler(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/registry/IWailaRegistry.java:4)

### `IWailaTooltipHandler`: it's an interface

- [ ] `IWailaTooltipHandler.transformItemName(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/registry/IWailaTooltipHandler.java:6)

### `IRenderLabel`: it's an interface

- [ ] `IRenderLabel.render(5)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/render/IRenderLabel.java:7)

### `IInteractionProvider`: it's an interface

- [ ] `IInteractionProvider.getProviderID(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/security/IInteractionProvider.java:8)
- [ ] `IInteractionProvider.canInteract(3)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/security/IInteractionProvider.java:10)

### `ISecurityProvider`: it's an interface

- [ ] `ISecurityProvider.getProviderID(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/security/ISecurityProvider.java:8)
- [ ] `ISecurityProvider.hasOwnership(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/security/ISecurityProvider.java:10)
- [ ] `ISecurityProvider.hasAccess(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/security/ISecurityProvider.java:12)

### `StorageDrawersApi`

- [ ] `StorageDrawersApi.instance`: it uses IStorageDrawersApi, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/StorageDrawersApi.java:4)
- [x] `StorageDrawersApi.VERSION` (com/jaquadro/minecraft/storagedrawers/api/StorageDrawersApi.java:5)
- [ ] `StorageDrawersApi.instance(0)`: it uses IStorageDrawersApi, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/StorageDrawersApi.java:7)

### `IPortable`: it's an interface

- [ ] `IPortable.isHeavy(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/IPortable.java:7)

### `IProtectable`: it's an interface

- [ ] `IProtectable.getOwner(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/IProtectable.java:7)
- [ ] `IProtectable.setOwner(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/IProtectable.java:9)
- [ ] `IProtectable.getSecurityProvider(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/IProtectable.java:11)
- [ ] `IProtectable.setSecurityProvider(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/IProtectable.java:13)

### `ISealable`: it's an interface

- [ ] `ISealable.isSealed(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/ISealable.java:4)
- [ ] `ISealable.setIsSealed(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/ISealable.java:6)

### `IVoidable`: it's an interface

- [ ] `IVoidable.isVoid(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/IVoidable.java:5)

### `LockAttribute`

- [ ] `LockAttribute.getFlagValue(0)`: it calls LockAttribute.ordinal, which isn't in the mod's source (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/LockAttribute.java:9)
- [ ] `LockAttribute.getBitfield(1)`: it uses `EnumSet` (java.util) (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/LockAttribute.java:13)
- [ ] `LockAttribute.getEnumSet(1)`: it uses `EnumSet` (java.util) (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/LockAttribute.java:26)

### `MagnetDim`


### `BlockType`


### `Drawers`

- [ ] `Drawers.DISABLED`: it uses IDrawer, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:8)
- [ ] `Drawers.DISABLED_FRACTIONAL`: it uses IFractionalDrawer, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:9)

### `Drawers.DisabledDrawer`: it implements `IDrawer`

- [ ] `Drawers.DisabledDrawer.getStoredItemPrototype(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:12)
- [ ] `Drawers.DisabledDrawer.setStoredItem(1)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:18)
- [ ] `Drawers.DisabledDrawer.getStoredItemCount(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:24)
- [ ] `Drawers.DisabledDrawer.setStoredItemCount(1)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:29)
- [ ] `Drawers.DisabledDrawer.getMaxCapacity(1)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:33)
- [ ] `Drawers.DisabledDrawer.getRemainingCapacity(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:38)
- [ ] `Drawers.DisabledDrawer.canItemBeStored(2)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:43)
- [ ] `Drawers.DisabledDrawer.canItemBeExtracted(2)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:48)
- [ ] `Drawers.DisabledDrawer.isEmpty(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:53)
- [ ] `Drawers.DisabledDrawer.isEnabled(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:58)
- [ ] `Drawers.DisabledDrawer.copy(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:63)

### `Drawers.DisabledFractionalDrawer`: it extends `DisabledDrawer`

- [ ] `Drawers.DisabledFractionalDrawer.getConversionRate(0)`: its class isn't translated (it extends `DisabledDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:70)
- [ ] `Drawers.DisabledFractionalDrawer.getStoredItemRemainder(0)`: its class isn't translated (it extends `DisabledDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:75)
- [ ] `Drawers.DisabledFractionalDrawer.isSmallestUnit(0)`: its class isn't translated (it extends `DisabledDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:80)
- [ ] `Drawers.DisabledFractionalDrawer.copy(0)`: its class isn't translated (it extends `DisabledDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:85)

### `Drawers.WrappedDrawer`: it implements `IDrawer`

- [ ] `Drawers.WrappedDrawer.WrappedDrawer(1)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:94)
- [ ] `Drawers.WrappedDrawer.getStoredItemPrototype(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:98)
- [ ] `Drawers.WrappedDrawer.setStoredItem(1)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:104)
- [ ] `Drawers.WrappedDrawer.getStoredItemCount(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:110)
- [ ] `Drawers.WrappedDrawer.setStoredItemCount(1)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:115)
- [ ] `Drawers.WrappedDrawer.getMaxCapacity(1)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:120)
- [ ] `Drawers.WrappedDrawer.getRemainingCapacity(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:125)
- [ ] `Drawers.WrappedDrawer.canItemBeStored(2)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:130)
- [ ] `Drawers.WrappedDrawer.canItemBeExtracted(2)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:135)
- [ ] `Drawers.WrappedDrawer.isEmpty(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:140)
- [ ] `Drawers.WrappedDrawer.isEnabled(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:145)
- [ ] `Drawers.WrappedDrawer.getAttributes(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:150)
- [ ] `Drawers.WrappedDrawer.copy(0)`: its class isn't translated (it implements `IDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:156)

### `Drawers.WrappedFractionalDrawer`: it extends `WrappedDrawer`

- [ ] `Drawers.WrappedFractionalDrawer.WrappedFractionalDrawer(1)`: its class isn't translated (it extends `WrappedDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:165)
- [ ] `Drawers.WrappedFractionalDrawer.getConversionRate(0)`: its class isn't translated (it extends `WrappedDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:170)
- [ ] `Drawers.WrappedFractionalDrawer.getStoredItemRemainder(0)`: its class isn't translated (it extends `WrappedDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:175)
- [ ] `Drawers.WrappedFractionalDrawer.isSmallestUnit(0)`: its class isn't translated (it extends `WrappedDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:180)
- [ ] `Drawers.WrappedFractionalDrawer.copy(0)`: its class isn't translated (it extends `WrappedDrawer`) (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java:185)

### `EmptyDrawerAttributes`: it implements `IDrawerAttributes`

- [ ] `EmptyDrawerAttributes.EMPTY`: it uses EmptyDrawerAttributes, which isn't translated (it implements `IDrawerAttributes`) (com/jaquadro/minecraft/storagedrawers/api/storage/EmptyDrawerAttributes.java:4)

### `EnumBasicDrawer`: its constants carry values

- [ ] `EnumBasicDrawer.META_LOOKUP`: it uses EnumBasicDrawer, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/storage/EnumBasicDrawer.java:13)
- [ ] `EnumBasicDrawer.EnumBasicDrawer(5)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/storage/EnumBasicDrawer.java:20)
- [ ] `EnumBasicDrawer.getMetadata(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/storage/EnumBasicDrawer.java:28)
- [ ] `EnumBasicDrawer.getDrawerCount(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/storage/EnumBasicDrawer.java:32)
- [ ] `EnumBasicDrawer.isHalfDepth(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/storage/EnumBasicDrawer.java:37)
- [ ] `EnumBasicDrawer.getUnlocalizedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/storage/EnumBasicDrawer.java:42)
- [ ] `EnumBasicDrawer.byMetadata(1)`: it uses EnumBasicDrawer, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/storage/EnumBasicDrawer.java:46)
- [ ] `EnumBasicDrawer.toString(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/storage/EnumBasicDrawer.java:54)
- [ ] `EnumBasicDrawer.getSerializedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/api/storage/EnumBasicDrawer.java:59)

### `IControlGroup`: it's an interface

- [ ] `IControlGroup.getDrawerGroup(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IControlGroup.java:8)
- [ ] `IControlGroup.getGroupControllableAttributes(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IControlGroup.java:10)
- [ ] `IControlGroup.getBoundControlGroup(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IControlGroup.java:12)
- [ ] `IControlGroup.getBoundRemoteNodes(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IControlGroup.java:14)
- [ ] `IControlGroup.validateRemoteNode(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IControlGroup.java:16)
- [ ] `IControlGroup.invalidateRemoteNode(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IControlGroup.java:18)
- [ ] `IControlGroup.addRemoteNode(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IControlGroup.java:20)
- [ ] `IControlGroup.isSoftBindingValid(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IControlGroup.java:22)

### `IDrawer`: it's an interface

- [ ] `IDrawer.getStoredItemPrototype(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:8)
- [ ] `IDrawer.setStoredItem(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:11)
- [ ] `IDrawer.setStoredItem(2)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:14)
- [ ] `IDrawer.getStoredItemCount(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:21)
- [ ] `IDrawer.setStoredItemCount(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:23)
- [ ] `IDrawer.adjustStoredItemCount(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:25)
- [ ] `IDrawer.getMaxCapacity(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:40)
- [ ] `IDrawer.getMaxCapacity(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:44)
- [ ] `IDrawer.getAcceptingMaxCapacity(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:46)
- [ ] `IDrawer.getRemainingCapacity(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:50)
- [ ] `IDrawer.getAcceptingRemainingCapacity(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:52)
- [ ] `IDrawer.getStoredItemStackSize(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:56)
- [ ] `IDrawer.canItemBeStored(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:61)
- [ ] `IDrawer.canItemBeStored(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:63)
- [ ] `IDrawer.canItemBeStoredManual(2)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:67)
- [ ] `IDrawer.canItemBeExtracted(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:71)
- [ ] `IDrawer.canItemBeExtracted(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:73)
- [ ] `IDrawer.isEmpty(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:77)
- [ ] `IDrawer.isEnabled(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:79)
- [ ] `IDrawer.isMissing(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:83)
- [ ] `IDrawer.canDetach(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:87)
- [ ] `IDrawer.setDetached(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:91)
- [ ] `IDrawer.getAttributes(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:94)
- [ ] `IDrawer.copy(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java:99)

### `IDrawerAttributes`: it's an interface

- [ ] `IDrawerAttributes.canItemLock(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:6)
- [ ] `IDrawerAttributes.isItemLocked(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:10)
- [ ] `IDrawerAttributes.isConcealed(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:14)
- [ ] `IDrawerAttributes.isSealed(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:18)
- [ ] `IDrawerAttributes.isShowingQuantity(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:22)
- [ ] `IDrawerAttributes.getPriority(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:26)
- [ ] `IDrawerAttributes.isVoid(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:30)
- [ ] `IDrawerAttributes.hasFillLevel(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:34)
- [ ] `IDrawerAttributes.isUnlimitedStorage(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:38)
- [ ] `IDrawerAttributes.isUnlimitedVending(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:42)
- [ ] `IDrawerAttributes.isDictConvertible(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:46)
- [ ] `IDrawerAttributes.isBalancedFill(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:50)
- [ ] `IDrawerAttributes.isHopper(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:54)
- [ ] `IDrawerAttributes.isMagnet(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:58)
- [ ] `IDrawerAttributes.isSuspended(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java:62)

### `IDrawerAttributesGroupControl`: it's an interface

- [ ] `IDrawerAttributesGroupControl.toggleConcealed(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesGroupControl.java:7)
- [ ] `IDrawerAttributesGroupControl.setIsConcealed(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesGroupControl.java:11)
- [ ] `IDrawerAttributesGroupControl.toggleItemLocked(2)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesGroupControl.java:15)
- [ ] `IDrawerAttributesGroupControl.setItemLocked(3)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesGroupControl.java:19)
- [ ] `IDrawerAttributesGroupControl.toggleIsShowingQuantity(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesGroupControl.java:23)
- [ ] `IDrawerAttributesGroupControl.setIsShowingQuantity(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesGroupControl.java:27)
- [ ] `IDrawerAttributesGroupControl.toggleIsSuspended(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesGroupControl.java:31)
- [ ] `IDrawerAttributesGroupControl.setIsSuspended(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesGroupControl.java:35)

### `IDrawerAttributesModifiable`: it's an interface

- [ ] `IDrawerAttributesModifiable.setIsConcealed(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:6)
- [ ] `IDrawerAttributesModifiable.setItemLocked(2)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:10)
- [ ] `IDrawerAttributesModifiable.setIsShowingQuantity(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:14)
- [ ] `IDrawerAttributesModifiable.setIsSealed(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:18)
- [ ] `IDrawerAttributesModifiable.setPriority(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:22)
- [ ] `IDrawerAttributesModifiable.setIsVoid(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:26)
- [ ] `IDrawerAttributesModifiable.setHasFillLevel(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:30)
- [ ] `IDrawerAttributesModifiable.setIsUnlimitedStorage(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:34)
- [ ] `IDrawerAttributesModifiable.setIsUnlimitedVending(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:38)
- [ ] `IDrawerAttributesModifiable.setIsDictConvertible(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:42)
- [ ] `IDrawerAttributesModifiable.setIsBalancedFill(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:46)
- [ ] `IDrawerAttributesModifiable.setIsHopper(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:50)
- [ ] `IDrawerAttributesModifiable.setIsMagnet(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:54)
- [ ] `IDrawerAttributesModifiable.setIsSuspended(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java:58)

### `IDrawerGeometry`: it's an interface

- [ ] `IDrawerGeometry.isHalfDepth(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerGeometry.java:4)
- [ ] `IDrawerGeometry.getDrawerCount(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerGeometry.java:6)

### `IDrawerGroup`: it's an interface

- [ ] `IDrawerGroup.getDrawerCount(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerGroup.java:7)
- [ ] `IDrawerGroup.getDrawer(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerGroup.java:9)
- [ ] `IDrawerGroup.getAccessibleDrawerSlots(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerGroup.java:12)
- [ ] `IDrawerGroup.isGroupValid(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerGroup.java:14)
- [ ] `IDrawerGroup.hasMissingDrawers(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerGroup.java:18)

### `IFractionalDrawer`: it's an interface

- [ ] `IFractionalDrawer.getConversionRate(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IFractionalDrawer.java:4)
- [ ] `IFractionalDrawer.getStoredItemRemainder(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IFractionalDrawer.java:6)
- [ ] `IFractionalDrawer.isSmallestUnit(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IFractionalDrawer.java:8)
- [ ] `IFractionalDrawer.copy(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/api/storage/IFractionalDrawer.java:10)

### `INetworked`: it's an interface

- [ ] `INetworked.supportsDirectControllerLink(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/INetworked.java:6)
- [ ] `INetworked.getBoundControlGroup(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/INetworked.java:10)
- [ ] `INetworked.getSoftBoundControlGroups(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/INetworked.java:14)
- [ ] `INetworked.softBindControlGroup(1)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/INetworked.java:18)
- [ ] `INetworked.canRecurseSearch(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/INetworked.java:21)
- [ ] `INetworked.unbindControlGroup(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/INetworked.java:25)
- [ ] `INetworked.scheduleValidation(0)`: its class isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/api/storage/INetworked.java:28)

### `BlockCompDrawers`: it extends `BlockDrawers`

- [ ] `BlockCompDrawers.SLOTS`: it uses Minecraft's `EnumProperty` (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:29)
- [ ] `BlockCompDrawers.BlockCompDrawers(4)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:31)
- [ ] `BlockCompDrawers.BlockCompDrawers(4)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:35)
- [ ] `BlockCompDrawers.BlockCompDrawers(3)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:40)
- [x] `BlockCompDrawers.calcUnits(2)` (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:45)
- [ ] `BlockCompDrawers.BlockCompDrawers(2)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:49)
- [ ] `BlockCompDrawers.BlockCompDrawers(1)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:54)
- [ ] `BlockCompDrawers.createBlockStateDefinition(1)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:59)
- [ ] `BlockCompDrawers.getFaceSlot(2)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:65)
- [ ] `BlockCompDrawers.setPlacedBy(5)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:80)
- [ ] `BlockCompDrawers.newBlockEntity(2)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:96)
- [ ] `BlockCompDrawers.makeFramedItem(4)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java:100)

### `BlockController`: it extends Minecraft's `HorizontalDirectionalBlock`

- [ ] `BlockController.BlockController(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:46)
- [ ] `BlockController.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:50)
- [ ] `BlockController.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:54)
- [ ] `BlockController.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:58)
- [ ] `BlockController.useItemOn(7)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:65)
- [ ] `BlockController.useController(6)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:78)
- [ ] `BlockController.toggle(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:130)
- [ ] `BlockController.toggle(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:165)
- [ ] `BlockController.togglePersonal(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:187)
- [ ] `BlockController.tick(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:203)
- [ ] `BlockController.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:213)
- [ ] `BlockController.makeFramedItem(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockController.java:217)

### `BlockControllerIO`: it extends Minecraft's `Block`

- [ ] `BlockControllerIO.BlockControllerIO(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockControllerIO.java:21)
- [ ] `BlockControllerIO.getController(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockControllerIO.java:25)
- [ ] `BlockControllerIO.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockControllerIO.java:30)
- [ ] `BlockControllerIO.makeFramedItem(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockControllerIO.java:34)

### `BlockDrawers`: it extends `FaceSlotBlock`

- [ ] `BlockDrawers.AABB_NORTH_FULL`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:84)
- [ ] `BlockDrawers.AABB_SOUTH_FULL`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:85)
- [ ] `BlockDrawers.AABB_WEST_FULL`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:86)
- [ ] `BlockDrawers.AABB_EAST_FULL`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:87)
- [ ] `BlockDrawers.AABB_NORTH_HALF`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:88)
- [ ] `BlockDrawers.AABB_SOUTH_HALF`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:89)
- [ ] `BlockDrawers.AABB_WEST_HALF`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:90)
- [ ] `BlockDrawers.AABB_EAST_HALF`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:91)
- [ ] `BlockDrawers.HOPPER_INSIDE`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:92)
- [ ] `BlockDrawers.AABB_NORTH_HOPPER`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:93)
- [ ] `BlockDrawers.AABB_SOUTH_HOPPER`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:94)
- [ ] `BlockDrawers.AABB_WEST_HOPPER`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:95)
- [ ] `BlockDrawers.AABB_EAST_HOPPER`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:96)
- [ ] `BlockDrawers.lastLeftClick`: it uses `UUID` (java.util) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:97)
- [ ] `BlockDrawers.BlockDrawers(4)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:109)
- [ ] `BlockDrawers.BlockDrawers(4)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:131)
- [ ] `BlockDrawers.createBlockStateDefinition(1)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:137)
- [ ] `BlockDrawers.retrimBlock(3)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:141)
- [ ] `BlockDrawers.retrimType(0)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:145)
- [ ] `BlockDrawers.repartitionBlock(3)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:149)
- [ ] `BlockDrawers.getDrawerCount(0)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:153)
- [ ] `BlockDrawers.isHalfDepth(0)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:157)
- [ ] `BlockDrawers.getStorageUnits(0)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:161)
- [ ] `BlockDrawers.getNameTypeKey(0)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:165)
- [ ] `BlockDrawers.getShape(4)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:170)
- [ ] `BlockDrawers.isPathfindable(2)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:196)
- [ ] `BlockDrawers.getStateForPlacement(1)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:200)
- [ ] `BlockDrawers.setPlacedBy(5)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:204)
- [ ] `BlockDrawers.getDrawerSlot(2)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:255)
- [ ] `BlockDrawers.hitAny(2)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:260)
- [ ] `BlockDrawers.hitTop(1)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:265)
- [ ] `BlockDrawers.hitLeft(2)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:270)
- [ ] `BlockDrawers.useSlotInvertible(1)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:275)
- [ ] `BlockDrawers.useSlot(1)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:286)
- [ ] `BlockDrawers.putSlot(2)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:451)
- [ ] `BlockDrawers.takeSlot(2)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:475)
- [ ] `BlockDrawers.interactPullDrawer(1)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:518)
- [ ] `BlockDrawers.interactReturnDrawer(2)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:538)
- [ ] `BlockDrawers.pullDrawer(3)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:563)
- [ ] `BlockDrawers.giveOrDropItemStack(2)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:590)
- [ ] `BlockDrawers.openUI(1)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:609)
- [ ] `BlockDrawers.getMenuProvider(3)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:622)
- [ ] `BlockDrawers.dropItemStack(4)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:628)
- [ ] `BlockDrawers.getDrops(2)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:635)
- [ ] `BlockDrawers.getMainDrop(2)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:642)
- [ ] `BlockDrawers.isSignalSource(1)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:682)
- [ ] `BlockDrawers.getSignal(4)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:686)
- [ ] `BlockDrawers.getDirectSignal(4)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:695)
- [ ] `BlockDrawers.affectNeighborsAfterRemoval(4)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:699)
- [ ] `BlockDrawers.hasAnalogOutputSignal(1)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:705)
- [ ] `BlockDrawers.getAnalogOutputSignal(4)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:709)
- [ ] `BlockDrawers.useShapeForLightOcclusion(1)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:718)
- [ ] `BlockDrawers.entityInside(6)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:722)
- [ ] `BlockDrawers.getInteractionShape(3)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:732)
- [ ] `BlockDrawers.tick(4)`: its class isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java:737)

### `BlockDrawers.GeometryType`


### `BlockFramingTable`: it extends Minecraft's `HorizontalDirectionalBlock`

- [ ] `BlockFramingTable.PART`: it uses Minecraft's `EnumProperty` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:39)
- [ ] `BlockFramingTable.TABLE_TOP`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:40)
- [ ] `BlockFramingTable.TABLE_BOTTOM_NORTH`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:41)
- [ ] `BlockFramingTable.TABLE_BOTTOM_SOUTH`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:42)
- [ ] `BlockFramingTable.TABLE_BOTTOM_WEST`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:43)
- [ ] `BlockFramingTable.TABLE_BOTTOM_EAST`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:44)
- [ ] `BlockFramingTable.TABLE_SHAPE_NORTH`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:45)
- [ ] `BlockFramingTable.TABLE_SHAPE_SOUTH`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:46)
- [ ] `BlockFramingTable.TABLE_SHAPE_WEST`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:47)
- [ ] `BlockFramingTable.TABLE_SHAPE_EAST`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:48)
- [ ] `BlockFramingTable.BlockFramingTable(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:50)
- [ ] `BlockFramingTable.getTableDirection(2)`: it uses Minecraft's `Direction` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:55)
- [ ] `BlockFramingTable.getNeighborDirection(2)`: it uses Minecraft's `Direction` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:60)
- [ ] `BlockFramingTable.getShape(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:64)
- [ ] `BlockFramingTable.playerWillDestroy(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:77)
- [ ] `BlockFramingTable.preventCreativeDropFromLeft(4)`: it uses Minecraft's `Level` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:88)
- [ ] `BlockFramingTable.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:107)
- [ ] `BlockFramingTable.getConnectedDirection(1)`: it uses Minecraft's `Direction` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:118)
- [ ] `BlockFramingTable.getBlockType(1)`: it uses Minecraft's `BlockState` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:123)
- [ ] `BlockFramingTable.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:128)
- [ ] `BlockFramingTable.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:132)
- [ ] `BlockFramingTable.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:137)
- [ ] `BlockFramingTable.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:148)
- [ ] `BlockFramingTable.getMenuProvider(3)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:161)
- [ ] `BlockFramingTable.openUI(3)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java:167)

### `BlockKeyButton`: it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`

- [ ] `BlockKeyButton.POWERED`: it uses Minecraft's `BooleanProperty` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:33)
- [ ] `BlockKeyButton.CEILING_AABB_X`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:34)
- [ ] `BlockKeyButton.CEILING_AABB_Z`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:35)
- [ ] `BlockKeyButton.FLOOR_AABB_X`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:36)
- [ ] `BlockKeyButton.FLOOR_AABB_Z`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:37)
- [ ] `BlockKeyButton.NORTH_AABB`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:38)
- [ ] `BlockKeyButton.SOUTH_AABB`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:39)
- [ ] `BlockKeyButton.WEST_AABB`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:40)
- [ ] `BlockKeyButton.EAST_AABB`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:41)
- [ ] `BlockKeyButton.PRESSED_CEILING_AABB_X`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:42)
- [ ] `BlockKeyButton.PRESSED_CEILING_AABB_Z`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:43)
- [ ] `BlockKeyButton.PRESSED_FLOOR_AABB_X`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:44)
- [ ] `BlockKeyButton.PRESSED_FLOOR_AABB_Z`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:45)
- [ ] `BlockKeyButton.PRESSED_NORTH_AABB`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:46)
- [ ] `BlockKeyButton.PRESSED_SOUTH_AABB`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:47)
- [ ] `BlockKeyButton.PRESSED_WEST_AABB`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:48)
- [ ] `BlockKeyButton.PRESSED_EAST_AABB`: it uses Minecraft's `VoxelShape` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:49)
- [ ] `BlockKeyButton.BlockKeyButton(2)`: its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:52)
- [ ] `BlockKeyButton.getShape(4)`: its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:61)
- [ ] `BlockKeyButton.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:89)
- [ ] `BlockKeyButton.press(3)`: its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:124)
- [ ] `BlockKeyButton.playSound(4)`: its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:130)
- [ ] `BlockKeyButton.getSound(1)`: its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:134)
- [ ] `BlockKeyButton.tick(4)`: its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:138)
- [ ] `BlockKeyButton.checkPressed(3)`: its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:144)
- [ ] `BlockKeyButton.updateNeighbours(3)`: its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:153)
- [ ] `BlockKeyButton.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java:158)

### `BlockStandardDrawers`: it extends `BlockDrawers`

- [ ] `BlockStandardDrawers.BlockStandardDrawers(4)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:35)
- [ ] `BlockStandardDrawers.BlockStandardDrawers(4)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:39)
- [ ] `BlockStandardDrawers.BlockStandardDrawers(3)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:44)
- [x] `BlockStandardDrawers.calcUnits(2)` (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:49)
- [ ] `BlockStandardDrawers.setMatKey(1)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:53)
- [ ] `BlockStandardDrawers.setMatKey(1)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:59)
- [ ] `BlockStandardDrawers.getMatKey(0)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:64)
- [ ] `BlockStandardDrawers.getNameMatKey(0)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:68)
- [ ] `BlockStandardDrawers.getFaceSlot(2)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:72)
- [ ] `BlockStandardDrawers.retrimBlock(3)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:98)
- [ ] `BlockStandardDrawers.repartitionBlock(3)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:131)
- [ ] `BlockStandardDrawers.newBlockEntity(2)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:188)
- [ ] `BlockStandardDrawers.makeFramedItem(4)`: its class isn't translated (it extends `BlockDrawers`) (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java:193)

### `BlockTrim`: it extends Minecraft's `Block`

- [ ] `BlockTrim.BlockTrim(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockTrim.java:17)
- [ ] `BlockTrim.setMatKey(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockTrim.java:21)
- [ ] `BlockTrim.setMatKey(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockTrim.java:27)
- [ ] `BlockTrim.getMatKey(0)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockTrim.java:32)
- [ ] `BlockTrim.getNameMatKey(0)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockTrim.java:36)
- [ ] `BlockTrim.getNameTypeKey(0)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockTrim.java:40)
- [ ] `BlockTrim.canUseForRetrim(0)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockTrim.java:44)
- [ ] `BlockTrim.makeFramedItem(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/BlockTrim.java:48)

### `EnumCompDrawer`: its constants carry values

- [ ] `EnumCompDrawer.META_LOOKUP`: it uses EnumCompDrawer, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java:12)
- [ ] `EnumCompDrawer.OPEN_SLOTS_LOOKUP`: it uses EnumCompDrawer, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java:13)
- [ ] `EnumCompDrawer.EnumCompDrawer(3)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java:18)
- [ ] `EnumCompDrawer.isHalfDepth(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java:24)
- [ ] `EnumCompDrawer.getDrawerCount(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java:29)
- [ ] `EnumCompDrawer.getMetadata(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java:34)
- [ ] `EnumCompDrawer.getOpenSlots(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java:38)
- [ ] `EnumCompDrawer.byMetadata(1)`: it uses EnumCompDrawer, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java:42)
- [ ] `EnumCompDrawer.byOpenSlots(1)`: it uses EnumCompDrawer, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java:50)
- [ ] `EnumCompDrawer.toString(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java:54)
- [ ] `EnumCompDrawer.getSerializedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java:59)

### `EnumFramingTablePart`: its constants carry values

- [ ] `EnumFramingTablePart.EnumFramingTablePart(1)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumFramingTablePart.java:11)
- [ ] `EnumFramingTablePart.toString(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumFramingTablePart.java:15)
- [ ] `EnumFramingTablePart.getSerializedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/block/EnumFramingTablePart.java:20)

### `FaceSlotBlock`: it extends Minecraft's `HorizontalDirectionalBlock`

- [ ] `FaceSlotBlock.lastLeftActionTick`: it uses `UUID` (java.util) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:29)
- [ ] `FaceSlotBlock.FaceSlotBlock(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:31)
- [ ] `FaceSlotBlock.rightAction(5)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:35)
- [ ] `FaceSlotBlock.leftAction(5)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:62)
- [ ] `FaceSlotBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:94)
- [ ] `FaceSlotBlock.attack(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:100)
- [ ] `FaceSlotBlock.useSlot(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:109)
- [ ] `FaceSlotBlock.useSlotInvertible(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:113)
- [ ] `FaceSlotBlock.putSlot(2)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:117)
- [ ] `FaceSlotBlock.takeSlot(2)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:121)
- [ ] `FaceSlotBlock.getFaceSlot(2)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:125)
- [ ] `FaceSlotBlock.normalizeHitVec(1)`: it creates Minecraft's `Vec3` (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:130)
- [ ] `FaceSlotBlock.getFaceSlot(2)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:139)
- [ ] `FaceSlotBlock.hitWithinArea(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:143)
- [ ] `FaceSlotBlock.hitWithinY(3)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:147)
- [ ] `FaceSlotBlock.hitWithinX(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:151)

### `FaceSlotBlock.InteractContext`: it uses Minecraft's `BlockState`

- [ ] `FaceSlotBlock.InteractContext.InteractContext(6)`: its class isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:169)
- [ ] `FaceSlotBlock.InteractContext.getCheckedEntity(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:178)
- [ ] `FaceSlotBlock.InteractContext.getCheckedEntity(2)`: it's generic (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java:182)

### `BlockFramedCompDrawers`: it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)

- [ ] `BlockFramedCompDrawers.BlockFramedCompDrawers(4)`: its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedCompDrawers.java:26)
- [ ] `BlockFramedCompDrawers.BlockFramedCompDrawers(4)`: its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedCompDrawers.java:30)
- [ ] `BlockFramedCompDrawers.BlockFramedCompDrawers(3)`: its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedCompDrawers.java:35)
- [ ] `BlockFramedCompDrawers.setPlacedBy(5)`: its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedCompDrawers.java:40)
- [ ] `BlockFramedCompDrawers.getMainDrop(2)`: its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedCompDrawers.java:50)
- [ ] `BlockFramedCompDrawers.getCloneItemStack(4)`: its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedCompDrawers.java:60)
- [ ] `BlockFramedCompDrawers.getFramedBlockEntity(2)`: its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedCompDrawers.java:70)
- [ ] `BlockFramedCompDrawers.supportsFrameMaterial(1)`: its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedCompDrawers.java:75)
- [ ] `BlockFramedCompDrawers.getShadeBrightness(3)`: its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedCompDrawers.java:80)

### `BlockFramedController`: it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)

- [ ] `BlockFramedController.BlockFramedController(1)`: its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedController.java:28)
- [ ] `BlockFramedController.setPlacedBy(5)`: its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedController.java:32)
- [ ] `BlockFramedController.getDrops(2)`: its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedController.java:41)
- [ ] `BlockFramedController.getMainDrop(2)`: its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedController.java:48)
- [ ] `BlockFramedController.getCloneItemStack(4)`: its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedController.java:61)
- [ ] `BlockFramedController.getFramedBlockEntity(2)`: its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedController.java:71)
- [ ] `BlockFramedController.supportsFrameMaterial(1)`: its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedController.java:76)
- [ ] `BlockFramedController.getShadeBrightness(3)`: its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedController.java:81)

### `BlockFramedControllerIO`: it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)

- [ ] `BlockFramedControllerIO.BlockFramedControllerIO(1)`: its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedControllerIO.java:28)
- [ ] `BlockFramedControllerIO.setPlacedBy(5)`: its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedControllerIO.java:32)
- [ ] `BlockFramedControllerIO.getDrops(2)`: its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedControllerIO.java:41)
- [ ] `BlockFramedControllerIO.getMainDrop(2)`: its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedControllerIO.java:48)
- [ ] `BlockFramedControllerIO.getCloneItemStack(4)`: its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedControllerIO.java:61)
- [ ] `BlockFramedControllerIO.getFramedBlockEntity(2)`: its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedControllerIO.java:71)
- [ ] `BlockFramedControllerIO.supportsFrameMaterial(1)`: its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedControllerIO.java:76)
- [ ] `BlockFramedControllerIO.getShadeBrightness(3)`: its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedControllerIO.java:84)

### `BlockFramedStandardDrawers`: it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)

- [ ] `BlockFramedStandardDrawers.BlockFramedStandardDrawers(4)`: its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedStandardDrawers.java:27)
- [ ] `BlockFramedStandardDrawers.BlockFramedStandardDrawers(4)`: its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedStandardDrawers.java:31)
- [ ] `BlockFramedStandardDrawers.BlockFramedStandardDrawers(3)`: its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedStandardDrawers.java:36)
- [ ] `BlockFramedStandardDrawers.retrimType(0)`: its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedStandardDrawers.java:41)
- [ ] `BlockFramedStandardDrawers.setPlacedBy(5)`: its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedStandardDrawers.java:46)
- [ ] `BlockFramedStandardDrawers.getMainDrop(2)`: its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedStandardDrawers.java:56)
- [ ] `BlockFramedStandardDrawers.getCloneItemStack(4)`: its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedStandardDrawers.java:66)
- [ ] `BlockFramedStandardDrawers.getFramedBlockEntity(2)`: its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedStandardDrawers.java:76)
- [ ] `BlockFramedStandardDrawers.supportsFrameMaterial(1)`: its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedStandardDrawers.java:81)
- [ ] `BlockFramedStandardDrawers.getShadeBrightness(3)`: its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedStandardDrawers.java:86)

### `BlockFramedTrim`: it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)

- [ ] `BlockFramedTrim.BlockFramedTrim(1)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:33)
- [ ] `BlockFramedTrim.setPlacedBy(5)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:37)
- [ ] `BlockFramedTrim.canUseForRetrim(0)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:46)
- [ ] `BlockFramedTrim.getDrops(2)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:51)
- [ ] `BlockFramedTrim.getMainDrop(2)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:58)
- [ ] `BlockFramedTrim.getCloneItemStack(4)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:71)
- [ ] `BlockFramedTrim.newBlockEntity(2)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:81)
- [ ] `BlockFramedTrim.getFramedBlockEntity(2)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:85)
- [ ] `BlockFramedTrim.supportsFrameMaterial(1)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:90)
- [ ] `BlockFramedTrim.getShadeBrightness(3)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:98)
- [ ] `BlockFramedTrim.propagatesSkylightDown(1)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:112)
- [ ] `BlockFramedTrim.getVisualShape(4)`: its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)) (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java:116)

### `KeyType`

- [ ] `KeyType.TYPES`: it creates `Object2ObjectArrayMap` (it.unimi.dsi.fastutil.objects) (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:10)
- [ ] `KeyType.CODEC`: it uses Minecraft's `Codec` (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:11)
- [ ] `KeyType.DRAWER`: it calls KeyType.register, which isn't translated (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:12)
- [ ] `KeyType.CONCEALMENT`: it calls KeyType.register, which isn't translated (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:13)
- [ ] `KeyType.PERSONAL`: it calls KeyType.register, which isn't translated (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:14)
- [ ] `KeyType.QUANTIFY`: it calls KeyType.register, which isn't translated (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:15)
- [ ] `KeyType.SUSPEND`: it calls KeyType.register, which isn't translated (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:16)
- [x] `KeyType.KeyType(1)` (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:9)
- [ ] `KeyType.register(1)`: it uses KeyType.TYPES, which isn't translated (it creates `Object2ObjectArrayMap` (it.unimi.dsi.fastutil.objects)) (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:18)
- [ ] `KeyType.values(0)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:23)
- [ ] `KeyType.isEnabled(0)`: `this` used as a value (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:27)
- [x] `KeyType.name(0)` (com/jaquadro/minecraft/storagedrawers/block/KeyType.java:9)

### `BlockMeta`: it extends Minecraft's `Block`

- [ ] `BlockMeta.BlockMeta(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMeta.java:7)

### `BlockMetaFacing`: it extends `BlockMeta`

- [ ] `BlockMetaFacing.FACING`: it uses Minecraft's `EnumProperty` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacing.java:16)
- [ ] `BlockMetaFacing.BlockMetaFacing(1)`: its class isn't translated (it extends `BlockMeta`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacing.java:18)
- [ ] `BlockMetaFacing.createBlockStateDefinition(1)`: its class isn't translated (it extends `BlockMeta`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacing.java:22)
- [ ] `BlockMetaFacing.rotate(2)`: its class isn't translated (it extends `BlockMeta`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacing.java:26)
- [ ] `BlockMetaFacing.mirror(2)`: its class isn't translated (it extends `BlockMeta`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacing.java:31)

### `BlockMetaFacingSized`: it extends `BlockMetaFacing`

- [ ] `BlockMetaFacingSized.HALF`: it uses Minecraft's `BooleanProperty` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSized.java:11)
- [ ] `BlockMetaFacingSized.BlockMetaFacingSized(1)`: its class isn't translated (it extends `BlockMetaFacing`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSized.java:13)
- [ ] `BlockMetaFacingSized.createBlockStateDefinition(1)`: its class isn't translated (it extends `BlockMetaFacing`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSized.java:17)

### `BlockMetaFacingSizedOpen`


### `BlockMetaFacingSizedOpen.Open2`: it extends `BlockMetaFacingSized`

- [ ] `BlockMetaFacingSizedOpen.Open2.SLOTS`: it uses Minecraft's `EnumProperty` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedOpen.java:13)
- [ ] `BlockMetaFacingSizedOpen.Open2.Open2(1)`: its class isn't translated (it extends `BlockMetaFacingSized`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedOpen.java:17)
- [ ] `BlockMetaFacingSizedOpen.Open2.createBlockStateDefinition(1)`: its class isn't translated (it extends `BlockMetaFacingSized`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedOpen.java:21)

### `BlockMetaFacingSizedOpen.Open3`: it extends `BlockMetaFacingSized`

- [ ] `BlockMetaFacingSizedOpen.Open3.SLOTS`: it uses Minecraft's `EnumProperty` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedOpen.java:28)
- [ ] `BlockMetaFacingSizedOpen.Open3.Open3(1)`: its class isn't translated (it extends `BlockMetaFacingSized`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedOpen.java:32)
- [ ] `BlockMetaFacingSizedOpen.Open3.createBlockStateDefinition(1)`: its class isn't translated (it extends `BlockMetaFacingSized`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedOpen.java:36)

### `BlockMetaFacingSizedSlotted`


### `BlockMetaFacingSizedSlotted.Label`: it extends `BlockMetaFacingSized`

- [ ] `BlockMetaFacingSizedSlotted.Label.SLOT`: it uses Minecraft's `IntegerProperty` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java:13)
- [ ] `BlockMetaFacingSizedSlotted.Label.Label(1)`: its class isn't translated (it extends `BlockMetaFacingSized`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java:15)
- [ ] `BlockMetaFacingSizedSlotted.Label.createBlockStateDefinition(1)`: its class isn't translated (it extends `BlockMetaFacingSized`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java:19)

### `BlockMetaFacingSizedSlotted.Slots124`: it extends `BlockMetaFacingSized`

- [ ] `BlockMetaFacingSizedSlotted.Slots124.SLOTS`: it uses IntegerSetProperty, which isn't translated (it extends Minecraft's `Property`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java:26)
- [ ] `BlockMetaFacingSizedSlotted.Slots124.Slots124(1)`: its class isn't translated (it extends `BlockMetaFacingSized`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java:28)
- [ ] `BlockMetaFacingSizedSlotted.Slots124.createBlockStateDefinition(1)`: its class isn't translated (it extends `BlockMetaFacingSized`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java:32)

### `BlockMetaFacingSizedSlotted.Slots23`: it extends `BlockMetaFacingSized`

- [ ] `BlockMetaFacingSizedSlotted.Slots23.SLOTS`: it uses IntegerSetProperty, which isn't translated (it extends Minecraft's `Property`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java:39)
- [ ] `BlockMetaFacingSizedSlotted.Slots23.Slots23(1)`: its class isn't translated (it extends `BlockMetaFacingSized`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java:41)
- [ ] `BlockMetaFacingSizedSlotted.Slots23.createBlockStateDefinition(1)`: its class isn't translated (it extends `BlockMetaFacingSized`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java:45)

### `BlockMetaTrans`: it extends `BlockMeta`

- [ ] `BlockMetaTrans.TRANS`: it uses Minecraft's `BooleanProperty` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaTrans.java:11)
- [ ] `BlockMetaTrans.BlockMetaTrans(1)`: its class isn't translated (it extends `BlockMeta`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaTrans.java:13)
- [ ] `BlockMetaTrans.createBlockStateDefinition(1)`: its class isn't translated (it extends `BlockMeta`) (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaTrans.java:17)

### `IntegerSetProperty`: it extends Minecraft's `Property`

- [ ] `IntegerSetProperty.IntegerSetProperty(2)`: its class isn't translated (it extends Minecraft's `Property`) (com/jaquadro/minecraft/storagedrawers/block/state/IntegerSetProperty.java:14)
- [ ] `IntegerSetProperty.getPossibleValues(0)`: its class isn't translated (it extends Minecraft's `Property`) (com/jaquadro/minecraft/storagedrawers/block/state/IntegerSetProperty.java:19)
- [ ] `IntegerSetProperty.equals(1)`: its class isn't translated (it extends Minecraft's `Property`) (com/jaquadro/minecraft/storagedrawers/block/state/IntegerSetProperty.java:23)
- [ ] `IntegerSetProperty.generateHashCode(0)`: its class isn't translated (it extends Minecraft's `Property`) (com/jaquadro/minecraft/storagedrawers/block/state/IntegerSetProperty.java:31)
- [ ] `IntegerSetProperty.create(2)`: it uses IntegerSetProperty, which isn't translated (it extends Minecraft's `Property`) (com/jaquadro/minecraft/storagedrawers/block/state/IntegerSetProperty.java:35)
- [ ] `IntegerSetProperty.getValue(1)`: its class isn't translated (it extends Minecraft's `Property`) (com/jaquadro/minecraft/storagedrawers/block/state/IntegerSetProperty.java:40)
- [ ] `IntegerSetProperty.getName(1)`: its class isn't translated (it extends Minecraft's `Property`) (com/jaquadro/minecraft/storagedrawers/block/state/IntegerSetProperty.java:49)
- [ ] `IntegerSetProperty.getInternalIndex(1)`: its class isn't translated (it extends Minecraft's `Property`) (com/jaquadro/minecraft/storagedrawers/block/state/IntegerSetProperty.java:53)

### `BaseBlockEntity`: it extends Minecraft's `BlockEntity`

- [ ] `BaseBlockEntity.BaseBlockEntity(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:25)
- [ ] `BaseBlockEntity.hasDataPacket(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:29)
- [ ] `BaseBlockEntity.dataPacketRequiresRenderUpdate(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:33)
- [ ] `BaseBlockEntity.injectData(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:37)
- [ ] `BaseBlockEntity.injectPortableData(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:45)
- [ ] `BaseBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:53)
- [ ] `BaseBlockEntity.onLoadFinished(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:63)
- [ ] `BaseBlockEntity.read(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:66)
- [ ] `BaseBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:70)
- [ ] `BaseBlockEntity.readPortable(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:76)
- [ ] `BaseBlockEntity.writePortable(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:84)
- [ ] `BaseBlockEntity.readFixed(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:92)
- [ ] `BaseBlockEntity.writeFixed(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:100)
- [ ] `BaseBlockEntity.trapLoadFailure(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:108)
- [ ] `BaseBlockEntity.restoreLoadFailure(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:112)
- [ ] `BaseBlockEntity.loadDidFail(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:120)
- [ ] `BaseBlockEntity.getUpdateTag(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:124)
- [ ] `BaseBlockEntity.getUpdatePacket(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:129)
- [ ] `BaseBlockEntity.onDataPacket(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:133)
- [ ] `BaseBlockEntity.markBlockForUpdate(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:144)
- [ ] `BaseBlockEntity.markBlockForUpdateClient(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:151)
- [ ] `BaseBlockEntity.markBlockForRenderUpdate(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java:158)

### `BlockEntityController`: it extends `BaseBlockEntity`

- [x] `BlockEntityController.PRI_LOCKED` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:63)
- [x] `BlockEntityController.PRI_NORMAL` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:64)
- [x] `BlockEntityController.PRI_LOCKED_VOID` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:65)
- [x] `BlockEntityController.PRI_VOID` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:66)
- [x] `BlockEntityController.PRI_EMPTY` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:67)
- [x] `BlockEntityController.PRI_LOCKED_EMPTY` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:68)
- [x] `BlockEntityController.PRI_DISABLED` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:69)
- [ ] `BlockEntityController.material(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:86)
- [ ] `BlockEntityController.getRenderData(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:90)
- [ ] `BlockEntityController.getAttributes(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:96)
- [ ] `BlockEntityController.getSlotPriorityGroup(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:109)
- [ ] `BlockEntityController.getSlotPriority(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:115)
- [ ] `BlockEntityController.BlockEntityController(3)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:137)
- [ ] `BlockEntityController.BlockEntityController(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:143)
- [ ] `BlockEntityController.getBlock(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:147)
- [ ] `BlockEntityController.printDebugInfo(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:155)
- [ ] `BlockEntityController.getDrawerGroup(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:162)
- [ ] `BlockEntityController.getGroupControllableAttributes(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:167)
- [ ] `BlockEntityController.getBoundControlGroup(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:172)
- [ ] `BlockEntityController.getBoundRemoteNodes(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:177)
- [ ] `BlockEntityController.validateRemoteNode(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:182)
- [ ] `BlockEntityController.invalidateRemoteNode(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:191)
- [ ] `BlockEntityController.addRemoteNode(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:200)
- [ ] `BlockEntityController.setRemoved(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:209)
- [ ] `BlockEntityController.isSoftBindingValid(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:232)
- [ ] `BlockEntityController.onEntityLoad(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:242)
- [ ] `BlockEntityController.isGroupValid(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:262)
- [ ] `BlockEntityController.interactPutItemsIntoInventory(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:267)
- [ ] `BlockEntityController.insertItems(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:305)
- [ ] `BlockEntityController.insertItems(3)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:309)
- [ ] `BlockEntityController.toggleProtection(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:316)
- [ ] `BlockEntityController.clearProtection(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:337)
- [ ] `BlockEntityController.getSlotsValidAttributes(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:346)
- [ ] `BlockEntityController.resetCache(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:358)
- [ ] `BlockEntityController.isValidIO(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:363)
- [ ] `BlockEntityController.updateCache(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:368)
- [ ] `BlockEntityController.indexSlotRecords(1)`: its body couldn't be read: a class declared inside a method (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:383)
- [ ] `BlockEntityController.sortSlotRecords(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:396)
- [ ] `BlockEntityController.rebuildPrimaryLookup(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:409)
- [ ] `BlockEntityController.containsNullEntries(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:425)
- [ ] `BlockEntityController.flattenLists(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:437)
- [ ] `BlockEntityController.clearRecordInfo(2)`: its body couldn't be read: a class declared inside a method (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:451)
- [ ] `BlockEntityController.updateRecordInfo(3)`: its body couldn't be read: a class declared inside a method (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:462)
- [ ] `BlockEntityController.populateNodes(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:536)
- [ ] `BlockEntityController.populateRoot(3)`: its body couldn't be read: a class declared inside a method (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:560)
- [ ] `BlockEntityController.getGroupForDrawerSlot(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:602)
- [ ] `BlockEntityController.getGroupForSlotRecord(1)`: its body couldn't be read: a class declared inside a method (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:611)
- [ ] `BlockEntityController.getLocalDrawerSlot(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:625)
- [ ] `BlockEntityController.readFixed(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:634)
- [ ] `BlockEntityController.dataPacketRequiresRenderUpdate(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:642)
- [ ] `BlockEntityController.getDrawerCount(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:647)
- [ ] `BlockEntityController.getDrawer(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:652)
- [ ] `BlockEntityController.getAccessibleDrawerSlots(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:659)
- [ ] `BlockEntityController.getItemRepository(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:664)
- [ ] `BlockEntityController.getCapability(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:668)
- [ ] `BlockEntityController.getBalanceDrawers(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:673)

### `BlockEntityController.GroupAttributeController`: it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)

- [ ] `BlockEntityController.GroupAttributeController.GroupAttributeController(0)`: its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:700)
- [ ] `BlockEntityController.GroupAttributeController.GroupAttributeController(1)`: its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:703)
- [ ] `BlockEntityController.GroupAttributeController.setIsConcealed(1)`: its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:707)
- [ ] `BlockEntityController.GroupAttributeController.setIsShowingQuantity(1)`: its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:713)
- [ ] `BlockEntityController.GroupAttributeController.setIsSuspended(1)`: its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:719)
- [ ] `BlockEntityController.GroupAttributeController.setItemLocked(3)`: its body couldn't be read: a class declared inside a method (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:725)
- [ ] `BlockEntityController.GroupAttributeController.toggleConcealed(0)`: its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:735)
- [ ] `BlockEntityController.GroupAttributeController.toggleIsShowingQuantity(0)`: its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:741)
- [ ] `BlockEntityController.GroupAttributeController.toggleIsSuspended(0)`: its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:749)
- [ ] `BlockEntityController.GroupAttributeController.toggleItemLocked(2)`: its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:755)

### `BlockEntityController.ItemRepository`: it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities)

- [ ] `BlockEntityController.ItemRepository.ItemRepository(1)`: its class isn't translated (it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:765)
- [ ] `BlockEntityController.ItemRepository.insertItem(3)`: its body couldn't be read: a class declared inside a method (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:769)
- [ ] `BlockEntityController.ItemRepository.getValidDrawer(3)`: its class isn't translated (it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:861)
- [ ] `BlockEntityController.ItemRepository.isValidDrawer(4)`: its class isn't translated (it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:872)
- [ ] `BlockEntityController.ItemRepository.extractItem(4)`: its class isn't translated (it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:885)
- [ ] `BlockEntityController.ItemRepository.hasAccess(2)`: its class isn't translated (it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:943)

### `BlockEntityController.ProtectedItemRepository`: it extends `ItemRepository`

- [ ] `BlockEntityController.ProtectedItemRepository.ProtectedItemRepository(2)`: its class isn't translated (it extends `ItemRepository`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:952)
- [ ] `BlockEntityController.ProtectedItemRepository.ProtectedItemRepository(3)`: its class isn't translated (it extends `ItemRepository`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:956)
- [ ] `BlockEntityController.ProtectedItemRepository.hasAccess(2)`: its class isn't translated (it extends `ItemRepository`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:962)

### `BlockEntityController.SlotRecord`: it implements `Comparable`

- [ ] `BlockEntityController.SlotRecord.SlotRecord(3)`: its class isn't translated (it implements `Comparable`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:980)
- [ ] `BlockEntityController.SlotRecord.compareTo(1)`: its class isn't translated (it implements `Comparable`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:986)

### `BlockEntityController.StorageRecord`: it uses IDrawerGroup, which isn't translated (it's an interface)

- [ ] `BlockEntityController.StorageRecord.clear(0)`: its class isn't translated (it uses IDrawerGroup, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java:1009)

### `BlockEntityControllerIO`: it extends `BaseBlockEntity`

- [x] `BlockEntityControllerIO.drawerSlots` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:29)
- [ ] `BlockEntityControllerIO.BlockEntityControllerIO(3)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:34)
- [ ] `BlockEntityControllerIO.BlockEntityControllerIO(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:40)
- [ ] `BlockEntityControllerIO.getDrawerGroup(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:44)
- [ ] `BlockEntityControllerIO.getGroupControllableAttributes(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:50)
- [ ] `BlockEntityControllerIO.getBoundControlGroup(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:56)
- [ ] `BlockEntityControllerIO.isSoftBindingValid(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:62)
- [ ] `BlockEntityControllerIO.getBoundRemoteNodes(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:68)
- [ ] `BlockEntityControllerIO.validateRemoteNode(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:73)
- [ ] `BlockEntityControllerIO.invalidateRemoteNode(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:77)
- [ ] `BlockEntityControllerIO.addRemoteNode(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:81)
- [ ] `BlockEntityControllerIO.isGroupValid(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:86)
- [ ] `BlockEntityControllerIO.bindController(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:91)
- [ ] `BlockEntityControllerIO.getControllerPos(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:97)
- [ ] `BlockEntityControllerIO.getController(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:101)
- [ ] `BlockEntityControllerIO.getItemRepository(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:105)
- [ ] `BlockEntityControllerIO.getAccessibleDrawerSlots(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:109)
- [ ] `BlockEntityControllerIO.getDrawerCount(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:115)
- [ ] `BlockEntityControllerIO.getDrawer(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:121)
- [ ] `BlockEntityControllerIO.setChanged(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:128)
- [ ] `BlockEntityControllerIO.getCapability(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:137)
- [ ] `BlockEntityControllerIO.material(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:142)
- [ ] `BlockEntityControllerIO.getRenderData(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:146)

### `BlockEntityControllerIO.ItemRepositoryProxy`: it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)

- [ ] `BlockEntityControllerIO.ItemRepositoryProxy.getAllItems(0)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:153)
- [ ] `BlockEntityControllerIO.ItemRepositoryProxy.insertItem(3)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:162)
- [ ] `BlockEntityControllerIO.ItemRepositoryProxy.extractItem(4)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:171)
- [ ] `BlockEntityControllerIO.ItemRepositoryProxy.getStoredItemCount(2)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:180)
- [ ] `BlockEntityControllerIO.ItemRepositoryProxy.getRemainingItemCapacity(2)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:188)
- [ ] `BlockEntityControllerIO.ItemRepositoryProxy.getItemCapacity(2)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java:196)

### `BlockEntityDrawers`: it extends `BaseBlockEntity`

- [ ] `BlockEntityDrawers.BlockEntityDrawers(3)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:103)
- [ ] `BlockEntityDrawers.checkBoundController(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:112)
- [ ] `BlockEntityDrawers.validateBoundController(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:144)
- [ ] `BlockEntityDrawers.onEntityLoad(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:150)
- [ ] `BlockEntityDrawers.scheduleConditionalTick(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:172)
- [ ] `BlockEntityDrawers.supportsDirectControllerLink(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:186)
- [ ] `BlockEntityDrawers.getBoundControlGroup(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:191)
- [ ] `BlockEntityDrawers.getSoftBoundControlGroups(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:196)
- [ ] `BlockEntityDrawers.softBindControlGroup(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:202)
- [ ] `BlockEntityDrawers.canRecurseSearch(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:207)
- [ ] `BlockEntityDrawers.unbindControlGroup(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:217)
- [ ] `BlockEntityDrawers.scheduleValidation(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:222)
- [ ] `BlockEntityDrawers.getGroup(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:227)
- [ ] `BlockEntityDrawers.getGroup(1)`: it uses IDrawerGroup, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:230)
- [ ] `BlockEntityDrawers.getDrawerAttributes(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:234)
- [ ] `BlockEntityDrawers.getDrawerAttributes(1)`: it uses IDrawerAttributes, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:239)
- [ ] `BlockEntityDrawers.upgrades(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:243)
- [ ] `BlockEntityDrawers.material(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:247)
- [ ] `BlockEntityDrawers.isGroupValid(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:251)
- [ ] `BlockEntityDrawers.getDrawerCapacity(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:256)
- [ ] `BlockEntityDrawers.getEffectiveDrawerCapacity(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:261)
- [ ] `BlockEntityDrawers.emptySlotCanBeCleared(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:265)
- [ ] `BlockEntityDrawers.preRemoveSideEffects(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:270)
- [ ] `BlockEntityDrawers.getOwner(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:281)
- [ ] `BlockEntityDrawers.setOwner(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:286)
- [ ] `BlockEntityDrawers.getSecurityProvider(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:302)
- [ ] `BlockEntityDrawers.setSecurityProvider(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:307)
- [ ] `BlockEntityDrawers.onAttributeChanged(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:324)
- [ ] `BlockEntityDrawers.refreshMagnetBound(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:328)
- [ ] `BlockEntityDrawers.isRedstone(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:339)
- [ ] `BlockEntityDrawers.getRedstoneLevel(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:343)
- [ ] `BlockEntityDrawers.getCombinedRedstoneLevel(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:356)
- [ ] `BlockEntityDrawers.getMinRedstoneLevel(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:378)
- [ ] `BlockEntityDrawers.getMaxRedstoneLevel(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:399)
- [ ] `BlockEntityDrawers.takeItemsFromSlot(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:412)
- [ ] `BlockEntityDrawers.takeItemsFromSlot(3)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:417)
- [ ] `BlockEntityDrawers.putItemsIntoSlot(3)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:439)
- [ ] `BlockEntityDrawers.putItemsIntoSlot(4)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:443)
- [ ] `BlockEntityDrawers.interactPutCurrentItemIntoSlot(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:471)
- [ ] `BlockEntityDrawers.interactPutCurrentInventoryIntoSlot(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:486)
- [ ] `BlockEntityDrawers.interactPutItemsIntoSlot(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:512)
- [ ] `BlockEntityDrawers.interactReplaceDrawer(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:529)
- [ ] `BlockEntityDrawers.interactReplaceDrawer(3)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:533)
- [ ] `BlockEntityDrawers.readPortable(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:568)
- [ ] `BlockEntityDrawers.writePortable(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:595)
- [ ] `BlockEntityDrawers.setChanged(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:640)
- [ ] `BlockEntityDrawers.syncClientCount(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:649)
- [ ] `BlockEntityDrawers.clientUpdateCount(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:663)
- [ ] `BlockEntityDrawers.clientUpdateCountAsync(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:669)
- [ ] `BlockEntityDrawers.dataPacketRequiresRenderUpdate(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:676)
- [ ] `BlockEntityDrawers.getDrawerCount(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:681)
- [ ] `BlockEntityDrawers.getDrawer(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:687)
- [ ] `BlockEntityDrawers.getAccessibleDrawerSlots(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:694)
- [ ] `BlockEntityDrawers.getName(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:700)
- [ ] `BlockEntityDrawers.getCustomName(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:709)
- [ ] `BlockEntityDrawers.applyImplicitComponents(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:713)
- [ ] `BlockEntityDrawers.collectImplicitComponents(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:718)
- [ ] `BlockEntityDrawers.removeComponentsFromTag(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:723)
- [ ] `BlockEntityDrawers.getCapability(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:727)
- [ ] `BlockEntityDrawers.getRenderData(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:732)
- [ ] `BlockEntityDrawers.pushItemsTick(3)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:738)
- [ ] `BlockEntityDrawers.entityInside(4)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:756)
- [ ] `BlockEntityDrawers.suckInItems(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:766)
- [ ] `BlockEntityDrawers.getItemEntitiesInRange(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:783)
- [ ] `BlockEntityDrawers.addItemEntity(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:789)

### `BlockEntityDrawers.ContentProvider`: it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)

- [ ] `BlockEntityDrawers.ContentProvider.ContentProvider(1)`: its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:822)
- [ ] `BlockEntityDrawers.ContentProvider.createContent(1)`: its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:826)
- [ ] `BlockEntityDrawers.ContentProvider.getDisplayName(0)`: its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:830)
- [ ] `BlockEntityDrawers.ContentProvider.createMenu(3)`: its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:834)

### `BlockEntityDrawers.DrawerAttributes`: it extends `BasicDrawerAttributes` (com.jaquadro.minecraft.storagedrawers.capabilities)

- [ ] `BlockEntityDrawers.DrawerAttributes.onAttributeChanged(0)`: its class isn't translated (it extends `BasicDrawerAttributes` (com.jaquadro.minecraft.storagedrawers.capabilities)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:847)

### `BlockEntityDrawers.DrawerUpgradeData`: it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)

- [ ] `BlockEntityDrawers.DrawerUpgradeData.DrawerUpgradeData(0)`: its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:867)
- [ ] `BlockEntityDrawers.DrawerUpgradeData.canAddUpgrade(1)`: its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:871)
- [ ] `BlockEntityDrawers.DrawerUpgradeData.canRemoveUpgrade(1)`: its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:883)
- [ ] `BlockEntityDrawers.DrawerUpgradeData.canSwapUpgrade(2)`: its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:900)
- [ ] `BlockEntityDrawers.DrawerUpgradeData.onUpgradeChanged(2)`: its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:931)
- [ ] `BlockEntityDrawers.DrawerUpgradeData.stackCapacityCheck(1)`: its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java:946)

### `BlockEntityDrawersComp`: it extends `BlockEntityDrawers`

- [ ] `BlockEntityDrawersComp.BlockEntityDrawersComp(3)`: its class isn't translated (it extends `BlockEntityDrawers`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:29)
- [ ] `BlockEntityDrawersComp.emptySlotCanBeCleared(1)`: its class isn't translated (it extends `BlockEntityDrawers`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:33)
- [ ] `BlockEntityDrawersComp.dataPacketRequiresRenderUpdate(0)`: its class isn't translated (it extends `BlockEntityDrawers`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:44)
- [ ] `BlockEntityDrawersComp.clientUpdateCount(2)`: its class isn't translated (it extends `BlockEntityDrawers`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:49)
- [ ] `BlockEntityDrawersComp.clientUpdateCountAsync(1)`: its class isn't translated (it extends `BlockEntityDrawers`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:56)

### `BlockEntityDrawersComp.GroupData`: it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)

- [ ] `BlockEntityDrawersComp.GroupData.GroupData(1)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:63)
- [ ] `BlockEntityDrawersComp.GroupData.getWorld(0)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:67)
- [ ] `BlockEntityDrawersComp.GroupData.isGroupValid(0)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:72)
- [ ] `BlockEntityDrawersComp.GroupData.log(1)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:77)
- [ ] `BlockEntityDrawersComp.GroupData.getStackCapacity(0)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:84)
- [ ] `BlockEntityDrawersComp.GroupData.onItemChanged(0)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:89)
- [ ] `BlockEntityDrawersComp.GroupData.onAmountChanged(0)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:117)
- [ ] `BlockEntityDrawersComp.GroupData.getCapability(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:133)
- [ ] `BlockEntityDrawersComp.GroupData.getOwner(0)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:140)
- [ ] `BlockEntityDrawersComp.GroupData.setOwner(1)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:145)
- [ ] `BlockEntityDrawersComp.GroupData.getSecurityProvider(0)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:150)
- [ ] `BlockEntityDrawersComp.GroupData.setSecurityProvider(1)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:155)
- [ ] `BlockEntityDrawersComp.GroupData.getBoundControlGroup(0)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:160)
- [ ] `BlockEntityDrawersComp.GroupData.getSoftBoundControlGroups(0)`: its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:165)

### `BlockEntityDrawersComp.Slot2`: it extends `BlockEntityDrawersComp`

- [ ] `BlockEntityDrawersComp.Slot2.Slot2(2)`: its class isn't translated (it extends `BlockEntityDrawersComp`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:174)
- [ ] `BlockEntityDrawersComp.Slot2.getGroup(0)`: its class isn't translated (it extends `BlockEntityDrawersComp`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:179)
- [ ] `BlockEntityDrawersComp.Slot2.onAttributeChanged(0)`: its class isn't translated (it extends `BlockEntityDrawersComp`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:185)

### `BlockEntityDrawersComp.Slot3`: it extends `BlockEntityDrawersComp`

- [ ] `BlockEntityDrawersComp.Slot3.Slot3(2)`: its class isn't translated (it extends `BlockEntityDrawersComp`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:195)
- [ ] `BlockEntityDrawersComp.Slot3.getGroup(0)`: its class isn't translated (it extends `BlockEntityDrawersComp`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:200)
- [ ] `BlockEntityDrawersComp.Slot3.onAttributeChanged(0)`: its class isn't translated (it extends `BlockEntityDrawersComp`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java:206)

### `BlockEntityDrawersStandard`: it extends `BlockEntityDrawers`

- [ ] `BlockEntityDrawersStandard.BlockEntityDrawersStandard(3)`: its class isn't translated (it extends `BlockEntityDrawers`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:20)

### `BlockEntityDrawersStandard.GroupData`: it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)

- [ ] `BlockEntityDrawersStandard.GroupData.GroupData(1)`: its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:25)
- [ ] `BlockEntityDrawersStandard.GroupData.createDrawer(1)`: its body couldn't be read: an inner class creation (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:29)
- [ ] `BlockEntityDrawersStandard.GroupData.isGroupValid(0)`: its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:35)
- [ ] `BlockEntityDrawersStandard.GroupData.getCapability(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:40)
- [ ] `BlockEntityDrawersStandard.GroupData.getOwner(0)`: its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:47)
- [ ] `BlockEntityDrawersStandard.GroupData.setOwner(1)`: its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:52)
- [ ] `BlockEntityDrawersStandard.GroupData.getSecurityProvider(0)`: its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:57)
- [ ] `BlockEntityDrawersStandard.GroupData.setSecurityProvider(1)`: its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:62)
- [ ] `BlockEntityDrawersStandard.GroupData.getBoundControlGroup(0)`: its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:67)
- [ ] `BlockEntityDrawersStandard.GroupData.getSoftBoundControlGroups(0)`: its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:72)

### `BlockEntityDrawersStandard.Slot1`: it extends `BlockEntityDrawersStandard`

- [ ] `BlockEntityDrawersStandard.Slot1.Slot1(2)`: its class isn't translated (it extends `BlockEntityDrawersStandard`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:81)
- [ ] `BlockEntityDrawersStandard.Slot1.getGroup(0)`: its class isn't translated (it extends `BlockEntityDrawersStandard`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:86)
- [ ] `BlockEntityDrawersStandard.Slot1.onAttributeChanged(0)`: its class isn't translated (it extends `BlockEntityDrawersStandard`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:92)

### `BlockEntityDrawersStandard.Slot2`: it extends `BlockEntityDrawersStandard`

- [ ] `BlockEntityDrawersStandard.Slot2.Slot2(2)`: its class isn't translated (it extends `BlockEntityDrawersStandard`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:102)
- [ ] `BlockEntityDrawersStandard.Slot2.getGroup(0)`: its class isn't translated (it extends `BlockEntityDrawersStandard`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:107)
- [ ] `BlockEntityDrawersStandard.Slot2.onAttributeChanged(0)`: its class isn't translated (it extends `BlockEntityDrawersStandard`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:113)

### `BlockEntityDrawersStandard.Slot4`: it extends `BlockEntityDrawersStandard`

- [ ] `BlockEntityDrawersStandard.Slot4.Slot4(2)`: its class isn't translated (it extends `BlockEntityDrawersStandard`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:123)
- [ ] `BlockEntityDrawersStandard.Slot4.getGroup(0)`: its class isn't translated (it extends `BlockEntityDrawersStandard`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:128)
- [ ] `BlockEntityDrawersStandard.Slot4.onAttributeChanged(0)`: its class isn't translated (it extends `BlockEntityDrawersStandard`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:134)

### `BlockEntityDrawersStandard.StandardDrawerData`: it extends `DrawerData`

- [ ] `BlockEntityDrawersStandard.StandardDrawerData.StandardDrawerData(2)`: its class isn't translated (it extends `DrawerData`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:144)
- [ ] `BlockEntityDrawersStandard.StandardDrawerData.StandardDrawerData(1)`: its class isn't translated (it extends `DrawerData`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:149)
- [ ] `BlockEntityDrawersStandard.StandardDrawerData.getStackCapacity(0)`: its class isn't translated (it extends `DrawerData`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:154)
- [ ] `BlockEntityDrawersStandard.StandardDrawerData.onItemChanged(0)`: its class isn't translated (it extends `DrawerData`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:165)
- [ ] `BlockEntityDrawersStandard.StandardDrawerData.onAmountChanged(0)`: its class isn't translated (it extends `DrawerData`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:173)
- [ ] `BlockEntityDrawersStandard.StandardDrawerData.copy(0)`: its body couldn't be read: an inner class creation (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java:181)

### `BlockEntityFramingTable`: it extends `BaseBlockEntity`

- [x] `BlockEntityFramingTable.SLOT_INPUT` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:40)
- [x] `BlockEntityFramingTable.SLOT_SIDE` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:41)
- [x] `BlockEntityFramingTable.SLOT_TRIM` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:42)
- [x] `BlockEntityFramingTable.SLOT_FRONT` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:43)
- [x] `BlockEntityFramingTable.SLOT_RESULT` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:44)
- [ ] `BlockEntityFramingTable.BlockEntityFramingTable(3)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:51)
- [ ] `BlockEntityFramingTable.BlockEntityFramingTable(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:57)
- [x] `BlockEntityFramingTable.isMaterialSlot(1)` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:61)
- [ ] `BlockEntityFramingTable.material(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:65)
- [ ] `BlockEntityFramingTable.inventory(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:69)
- [ ] `BlockEntityFramingTable.readFixed(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:73)
- [ ] `BlockEntityFramingTable.writeFixed(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:80)
- [ ] `BlockEntityFramingTable.isItemValidTarget(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:92)
- [ ] `BlockEntityFramingTable.isItemValidMaterial(1)`: `instanceof` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:115)
- [ ] `BlockEntityFramingTable.getName(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:143)
- [ ] `BlockEntityFramingTable.getCustomName(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:147)
- [ ] `BlockEntityFramingTable.applyImplicitComponents(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:151)
- [ ] `BlockEntityFramingTable.collectImplicitComponents(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:156)
- [ ] `BlockEntityFramingTable.removeComponentsFromTag(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:161)
- [ ] `BlockEntityFramingTable.readPortable(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:165)
- [ ] `BlockEntityFramingTable.writePortable(1)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:171)
- [ ] `BlockEntityFramingTable.preRemoveSideEffects(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:179)
- [ ] `BlockEntityFramingTable.getRenderBoundingBox(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:187)

### `BlockEntityFramingTable.BlockInventory`: it implements Minecraft's `Container`

- [ ] `BlockEntityFramingTable.BlockInventory.BlockInventory(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:195)
- [ ] `BlockEntityFramingTable.BlockInventory.getContainerSize(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:199)
- [ ] `BlockEntityFramingTable.BlockInventory.isEmpty(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:203)
- [ ] `BlockEntityFramingTable.BlockInventory.canPlaceItem(2)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:215)
- [ ] `BlockEntityFramingTable.BlockInventory.getItem(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:223)
- [ ] `BlockEntityFramingTable.BlockInventory.removeItem(2)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:234)
- [ ] `BlockEntityFramingTable.BlockInventory.removeItemNoUpdate(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:245)
- [ ] `BlockEntityFramingTable.BlockInventory.setItem(2)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:255)
- [ ] `BlockEntityFramingTable.BlockInventory.setChanged(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:275)
- [ ] `BlockEntityFramingTable.BlockInventory.setInputItem(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:280)
- [ ] `BlockEntityFramingTable.BlockInventory.rebuildResult(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:315)
- [ ] `BlockEntityFramingTable.BlockInventory.stillValid(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:348)
- [ ] `BlockEntityFramingTable.BlockInventory.clearContent(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:352)

### `BlockEntityFramingTable.ContentProvider`: it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)

- [ ] `BlockEntityFramingTable.ContentProvider.ContentProvider(1)`: its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:362)
- [ ] `BlockEntityFramingTable.ContentProvider.createContent(1)`: its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:366)
- [ ] `BlockEntityFramingTable.ContentProvider.getDisplayName(0)`: its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:370)
- [ ] `BlockEntityFramingTable.ContentProvider.createMenu(3)`: its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java:374)

### `BlockEntityTrim`: it extends `BaseBlockEntity`

- [ ] `BlockEntityTrim.BlockEntityTrim(3)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityTrim.java:16)
- [ ] `BlockEntityTrim.BlockEntityTrim(2)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityTrim.java:21)
- [ ] `BlockEntityTrim.material(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityTrim.java:25)
- [ ] `BlockEntityTrim.dataPacketRequiresRenderUpdate(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityTrim.java:29)
- [ ] `BlockEntityTrim.getRenderData(0)`: its class isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityTrim.java:34)

### `DrawerModelProperties`: it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)

- [ ] `DrawerModelProperties.INSTANCE`: it uses DrawerModelProperties, which isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/DrawerModelProperties.java:21)
- [ ] `DrawerModelProperties.DrawerModelProperties(0)`: its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/DrawerModelProperties.java:27)
- [ ] `DrawerModelProperties.DrawerModelProperties(1)`: its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/DrawerModelProperties.java:34)
- [ ] `DrawerModelProperties.getModelData(1)`: it uses DrawerModelProperties, which isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/DrawerModelProperties.java:41)
- [ ] `DrawerModelProperties.makeContext(3)`: its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/DrawerModelProperties.java:45)
- [ ] `DrawerModelProperties.makeContext(1)`: its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/DrawerModelProperties.java:51)

### `FramedModelProperties`: it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)

- [ ] `FramedModelProperties.INSTANCE`: it uses FramedModelProperties, which isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/FramedModelProperties.java:19)
- [ ] `FramedModelProperties.FramedModelProperties(0)`: its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/FramedModelProperties.java:22)
- [ ] `FramedModelProperties.FramedModelProperties(1)`: its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/FramedModelProperties.java:26)
- [ ] `FramedModelProperties.getModelData(1)`: it uses FramedModelProperties, which isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/FramedModelProperties.java:30)
- [ ] `FramedModelProperties.makeContext(3)`: its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/FramedModelProperties.java:34)
- [ ] `FramedModelProperties.makeContext(1)`: its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)) (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/FramedModelProperties.java:43)

### `RenderDataProvider`: it's an interface

- [ ] `RenderDataProvider.getRenderData(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/RenderDataProvider.java:6)

### `PlatformBlockEntityController`: it extends `BlockEntityController`

- [ ] `PlatformBlockEntityController.PlatformBlockEntityController(2)`: its class isn't translated (it extends `BlockEntityController`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityController.java:9)
- [ ] `PlatformBlockEntityController.getRenderData(0)`: its class isn't translated (it extends `BlockEntityController`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityController.java:13)

### `PlatformBlockEntityControllerIO`: it extends `BlockEntityControllerIO`

- [ ] `PlatformBlockEntityControllerIO.PlatformBlockEntityControllerIO(2)`: its class isn't translated (it extends `BlockEntityControllerIO`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityControllerIO.java:9)
- [ ] `PlatformBlockEntityControllerIO.getRenderData(0)`: its class isn't translated (it extends `BlockEntityControllerIO`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityControllerIO.java:13)

### `PlatformBlockEntityDrawersComp`: it extends `BlockEntityDrawersComp`

- [ ] `PlatformBlockEntityDrawersComp.PlatformBlockEntityDrawersComp(3)`: its class isn't translated (it extends `BlockEntityDrawersComp`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersComp.java:10)

### `PlatformBlockEntityDrawersComp.Slot2`: it extends `Slot2`

- [ ] `PlatformBlockEntityDrawersComp.Slot2.Slot2(2)`: its class isn't translated (it extends `Slot2`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersComp.java:15)
- [ ] `PlatformBlockEntityDrawersComp.Slot2.getRenderData(0)`: its class isn't translated (it extends `Slot2`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersComp.java:19)

### `PlatformBlockEntityDrawersComp.Slot3`: it extends `Slot3`

- [ ] `PlatformBlockEntityDrawersComp.Slot3.Slot3(2)`: its class isn't translated (it extends `Slot3`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersComp.java:27)
- [ ] `PlatformBlockEntityDrawersComp.Slot3.getRenderData(0)`: its class isn't translated (it extends `Slot3`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersComp.java:31)

### `PlatformBlockEntityDrawersStandard`: it extends `BlockEntityDrawersStandard`

- [ ] `PlatformBlockEntityDrawersStandard.PlatformBlockEntityDrawersStandard(3)`: its class isn't translated (it extends `BlockEntityDrawersStandard`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersStandard.java:10)

### `PlatformBlockEntityDrawersStandard.Slot1`: it extends `Slot1`

- [ ] `PlatformBlockEntityDrawersStandard.Slot1.Slot1(2)`: its class isn't translated (it extends `Slot1`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersStandard.java:15)
- [ ] `PlatformBlockEntityDrawersStandard.Slot1.getRenderData(0)`: its class isn't translated (it extends `Slot1`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersStandard.java:19)

### `PlatformBlockEntityDrawersStandard.Slot2`: it extends `Slot2`

- [ ] `PlatformBlockEntityDrawersStandard.Slot2.Slot2(2)`: its class isn't translated (it extends `Slot2`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersStandard.java:27)
- [ ] `PlatformBlockEntityDrawersStandard.Slot2.getRenderData(0)`: its class isn't translated (it extends `Slot2`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersStandard.java:31)

### `PlatformBlockEntityDrawersStandard.Slot4`: it extends `Slot4`

- [ ] `PlatformBlockEntityDrawersStandard.Slot4.Slot4(2)`: its class isn't translated (it extends `Slot4`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersStandard.java:39)
- [ ] `PlatformBlockEntityDrawersStandard.Slot4.getRenderData(0)`: its class isn't translated (it extends `Slot4`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersStandard.java:43)

### `PlatformBlockEntityTrim`: it extends `BlockEntityTrim`

- [ ] `PlatformBlockEntityTrim.PlatformBlockEntityTrim(2)`: its class isn't translated (it extends `BlockEntityTrim`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityTrim.java:9)
- [ ] `PlatformBlockEntityTrim.getRenderData(0)`: its class isn't translated (it extends `BlockEntityTrim`) (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityTrim.java:13)

### `BlockEntityDataShim`

- [ ] `BlockEntityDataShim.read(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/BlockEntityDataShim.java:7)
- [ ] `BlockEntityDataShim.write(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/BlockEntityDataShim.java:9)

### `ControllerData`: it extends `BlockEntityDataShim`

- [ ] `ControllerData.read(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerData.java:15)
- [ ] `ControllerData.write(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerData.java:22)
- [ ] `ControllerData.getCoord(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerData.java:36)
- [ ] `ControllerData.getController(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerData.java:40)
- [ ] `ControllerData.bind(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerData.java:57)
- [ ] `ControllerData.bindCoord(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerData.java:61)
- [ ] `ControllerData.needsValidation(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerData.java:78)
- [ ] `ControllerData.setNeedsValidation(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerData.java:82)

### `ControllerHostData`: it extends `BlockEntityDataShim`

- [ ] `ControllerHostData.read(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerHostData.java:23)
- [ ] `ControllerHostData.write(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerHostData.java:33)
- [ ] `ControllerHostData.validateRemoteNodes(2)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerHostData.java:45)
- [ ] `ControllerHostData.validateRemoteNode(2)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerHostData.java:69)
- [ ] `ControllerHostData.addRemoteNode(2)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerHostData.java:87)
- [ ] `ControllerHostData.removeRemoteNode(2)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerHostData.java:109)
- [ ] `ControllerHostData.getRemoteNodes(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerHostData.java:129)

### `DetachedDrawerData`: it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)

- [ ] `DetachedDrawerData.DetachedDrawerData(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:18)
- [ ] `DetachedDrawerData.DetachedDrawerData(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:25)
- [ ] `DetachedDrawerData.DetachedDrawerData(2)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:29)
- [ ] `DetachedDrawerData.DetachedDrawerData(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:35)
- [ ] `DetachedDrawerData.DetachedDrawerData(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:39)
- [ ] `DetachedDrawerData.getStorageMultiplier(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:46)
- [ ] `DetachedDrawerData.setStorageMultiplier(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:50)
- [ ] `DetachedDrawerData.isHeavy(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:54)
- [ ] `DetachedDrawerData.setIsHeavy(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:58)
- [ ] `DetachedDrawerData.getStoredItemPrototype(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:62)
- [ ] `DetachedDrawerData.setStoredItem(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:68)
- [ ] `DetachedDrawerData.setStoredItemRaw(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:74)
- [ ] `DetachedDrawerData.getStoredItemCount(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:82)
- [ ] `DetachedDrawerData.setStoredItemCount(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:87)
- [ ] `DetachedDrawerData.setStoredItemCountRaw(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:91)
- [ ] `DetachedDrawerData.getMaxCapacity(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:95)
- [ ] `DetachedDrawerData.getRemainingCapacity(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:100)
- [ ] `DetachedDrawerData.canItemBeStored(2)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:105)
- [ ] `DetachedDrawerData.canItemBeExtracted(2)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:110)
- [ ] `DetachedDrawerData.isEmpty(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:115)
- [ ] `DetachedDrawerData.copy(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:120)
- [ ] `DetachedDrawerData.serializeNBT(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:125)
- [ ] `DetachedDrawerData.deserializeNBT(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java:139)

### `FractionalDrawerGroup`: it extends `BlockEntityDataShim`

- [ ] `FractionalDrawerGroup.FractionalDrawerGroup(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:32)
- [ ] `FractionalDrawerGroup.getDrawerCount(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:42)
- [ ] `FractionalDrawerGroup.getDrawer(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:47)
- [ ] `FractionalDrawerGroup.getAccessibleDrawerSlots(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:52)
- [ ] `FractionalDrawerGroup.getPooledCount(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:57)
- [ ] `FractionalDrawerGroup.setPooledCount(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:61)
- [ ] `FractionalDrawerGroup.read(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:65)
- [ ] `FractionalDrawerGroup.write(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:70)
- [ ] `FractionalDrawerGroup.syncAttributes(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:75)
- [ ] `FractionalDrawerGroup.getWorld(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:79)
- [ ] `FractionalDrawerGroup.log(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:83)
- [ ] `FractionalDrawerGroup.getStackCapacity(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:86)
- [ ] `FractionalDrawerGroup.onItemChanged(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:90)
- [ ] `FractionalDrawerGroup.onAmountChanged(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:93)

### `FractionalDrawerGroup.FractionalDrawer`: it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)

- [ ] `FractionalDrawerGroup.FractionalDrawer.FractionalDrawer(2)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:100)
- [ ] `FractionalDrawerGroup.FractionalDrawer.FractionalDrawer(1)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:105)
- [ ] `FractionalDrawerGroup.FractionalDrawer.getStoredItemPrototype(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:109)
- [ ] `FractionalDrawerGroup.FractionalDrawer.setStoredItem(1)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:115)
- [ ] `FractionalDrawerGroup.FractionalDrawer.getStoredItemCount(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:125)
- [ ] `FractionalDrawerGroup.FractionalDrawer.setStoredItemCount(1)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:130)
- [ ] `FractionalDrawerGroup.FractionalDrawer.adjustStoredItemCount(1)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:135)
- [ ] `FractionalDrawerGroup.FractionalDrawer.getMaxCapacity(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:140)
- [ ] `FractionalDrawerGroup.FractionalDrawer.getMaxCapacity(1)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:145)
- [ ] `FractionalDrawerGroup.FractionalDrawer.getAcceptingMaxCapacity(1)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:150)
- [ ] `FractionalDrawerGroup.FractionalDrawer.getRemainingCapacity(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:155)
- [ ] `FractionalDrawerGroup.FractionalDrawer.getAcceptingRemainingCapacity(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:160)
- [ ] `FractionalDrawerGroup.FractionalDrawer.canItemBeStored(2)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:165)
- [ ] `FractionalDrawerGroup.FractionalDrawer.canItemBeStoredManual(2)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:170)
- [ ] `FractionalDrawerGroup.FractionalDrawer.canItemBeExtracted(2)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:175)
- [ ] `FractionalDrawerGroup.FractionalDrawer.isEmpty(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:180)
- [ ] `FractionalDrawerGroup.FractionalDrawer.isEnabled(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:185)
- [ ] `FractionalDrawerGroup.FractionalDrawer.getConversionRate(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:190)
- [ ] `FractionalDrawerGroup.FractionalDrawer.getStoredItemRemainder(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:195)
- [ ] `FractionalDrawerGroup.FractionalDrawer.isSmallestUnit(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:200)
- [ ] `FractionalDrawerGroup.FractionalDrawer.getAttributes(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:205)
- [ ] `FractionalDrawerGroup.FractionalDrawer.copy(0)`: its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:211)

### `FractionalDrawerGroup.FractionalStorage`: it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)

- [ ] `FractionalDrawerGroup.FractionalStorage.FractionalStorage(2)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:230)
- [ ] `FractionalDrawerGroup.FractionalStorage.getAttributes(0)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:249)
- [ ] `FractionalDrawerGroup.FractionalStorage.getPooledCount(0)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:259)
- [ ] `FractionalDrawerGroup.FractionalStorage.setPooledCount(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:263)
- [ ] `FractionalDrawerGroup.FractionalStorage.getStack(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:270)
- [ ] `FractionalDrawerGroup.FractionalStorage.baseStack(0)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:275)
- [ ] `FractionalDrawerGroup.FractionalStorage.baseRate(0)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:280)
- [ ] `FractionalDrawerGroup.FractionalStorage.setStoredItem(2)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:284)
- [ ] `FractionalDrawerGroup.FractionalStorage.getStoredCount(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:307)
- [ ] `FractionalDrawerGroup.FractionalStorage.setStoredItemCount(2)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:316)
- [ ] `FractionalDrawerGroup.FractionalStorage.adjustStoredItemCount(2)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:335)
- [ ] `FractionalDrawerGroup.FractionalStorage.getMaxCapacity(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:378)
- [ ] `FractionalDrawerGroup.FractionalStorage.getMaxCapacity(2)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:398)
- [ ] `FractionalDrawerGroup.FractionalStorage.getAcceptingMaxCapacity(2)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:420)
- [ ] `FractionalDrawerGroup.FractionalStorage.getRemainingCapacity(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:425)
- [ ] `FractionalDrawerGroup.FractionalStorage.getAcceptingRemainingCapacity(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:440)
- [ ] `FractionalDrawerGroup.FractionalStorage.isEmpty(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:455)
- [ ] `FractionalDrawerGroup.FractionalStorage.isEnabled(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:459)
- [ ] `FractionalDrawerGroup.FractionalStorage.canItemBeStored(4)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:463)
- [ ] `FractionalDrawerGroup.FractionalStorage.canItemBeExtracted(3)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:476)
- [ ] `FractionalDrawerGroup.FractionalStorage.getConversionRate(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:484)
- [ ] `FractionalDrawerGroup.FractionalStorage.getStoredItemRemainder(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:488)
- [ ] `FractionalDrawerGroup.FractionalStorage.isSmallestUnit(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:496)
- [ ] `FractionalDrawerGroup.FractionalStorage.reset(0)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:500)
- [ ] `FractionalDrawerGroup.FractionalStorage.populateSlotsFromCache(0)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:512)
- [ ] `FractionalDrawerGroup.FractionalStorage.populateSlots(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:520)
- [ ] `FractionalDrawerGroup.FractionalStorage.populateRawSlot(3)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:586)
- [ ] `FractionalDrawerGroup.FractionalStorage.normalizeGroup(0)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:596)
- [ ] `FractionalDrawerGroup.FractionalStorage.serializeNBT(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:627)
- [ ] `FractionalDrawerGroup.FractionalStorage.deserializeNBT(1)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:642)
- [ ] `FractionalDrawerGroup.FractionalStorage.syncAttributes(0)`: its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java:675)

### `MaterialData`: it extends `BlockEntityDataShim`

- [ ] `MaterialData.EMPTY`: it uses MaterialData, which isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:18)
- [ ] `MaterialData.CODEC`: it uses Minecraft's `Codec` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:19)
- [ ] `MaterialData.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:28)
- [ ] `MaterialData.MaterialData(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:48)
- [ ] `MaterialData.MaterialData(4)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:52)
- [ ] `MaterialData.MaterialData(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:59)
- [ ] `MaterialData.getFrameBase(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:69)
- [ ] `MaterialData.getSide(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:74)
- [ ] `MaterialData.getFront(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:79)
- [ ] `MaterialData.getTrim(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:84)
- [ ] `MaterialData.getEffectiveSide(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:89)
- [ ] `MaterialData.getEffectiveFront(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:94)
- [ ] `MaterialData.getEffectiveTrim(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:99)
- [ ] `MaterialData.isMatOpaque(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:104)
- [ ] `MaterialData.allMatOpaque(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:108)
- [ ] `MaterialData.setFrameBase(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:114)
- [ ] `MaterialData.setSide(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:118)
- [ ] `MaterialData.setFront(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:122)
- [ ] `MaterialData.setTrim(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:126)
- [ ] `MaterialData.clear(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:130)
- [ ] `MaterialData.isEmpty(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:136)
- [ ] `MaterialData.read(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:140)
- [ ] `MaterialData.read(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:148)
- [ ] `MaterialData.write(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:156)
- [ ] `MaterialData.getHostBlock(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:183)
- [ ] `MaterialData.setHostBlock(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:189)
- [ ] `MaterialData.getMaterial(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:194)
- [ ] `MaterialData.setMaterial(2)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java:204)

### `StandardDrawerGroup`: it extends `BlockEntityDataShim`

- [ ] `StandardDrawerGroup.StandardDrawerGroup(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:26)
- [ ] `StandardDrawerGroup.getDrawerCount(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:37)
- [ ] `StandardDrawerGroup.getDrawer(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:42)
- [ ] `StandardDrawerGroup.getAccessibleDrawerSlots(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:48)
- [ ] `StandardDrawerGroup.read(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:53)
- [ ] `StandardDrawerGroup.write(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:63)
- [ ] `StandardDrawerGroup.createDrawer(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:74)
- [ ] `StandardDrawerGroup.syncAttributes(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:77)
- [ ] `StandardDrawerGroup.syncSlots(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:83)

### `StandardDrawerGroup.DrawerData`: it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)

- [ ] `StandardDrawerGroup.DrawerData.DrawerData(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:121)
- [ ] `StandardDrawerGroup.DrawerData.DrawerData(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:128)
- [ ] `StandardDrawerGroup.DrawerData.getAttributes(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:136)
- [ ] `StandardDrawerGroup.DrawerData.getStoredItemPrototype(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:147)
- [ ] `StandardDrawerGroup.DrawerData.setStoredItem(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:153)
- [ ] `StandardDrawerGroup.DrawerData.setStoredItem(2)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:159)
- [ ] `StandardDrawerGroup.DrawerData.setStoredItemRaw(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:196)
- [ ] `StandardDrawerGroup.DrawerData.getStoredItemCount(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:211)
- [ ] `StandardDrawerGroup.DrawerData.setStoredItemCount(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:221)
- [ ] `StandardDrawerGroup.DrawerData.setStoredItemCount(2)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:226)
- [ ] `StandardDrawerGroup.DrawerData.setStoredItemCountRaw(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:241)
- [ ] `StandardDrawerGroup.DrawerData.adjustStoredItemCount(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:245)
- [ ] `StandardDrawerGroup.DrawerData.adjustStoredItemCount(2)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:250)
- [ ] `StandardDrawerGroup.DrawerData.getMaxCapacity(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:284)
- [ ] `StandardDrawerGroup.DrawerData.getStackSize(1)`: it calls Minecraft's ItemStack.getMaxStackSize (no Horizon helper for it yet) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:302)
- [ ] `StandardDrawerGroup.DrawerData.getAcceptingMaxCapacity(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:306)
- [ ] `StandardDrawerGroup.DrawerData.getRemainingCapacity(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:316)
- [ ] `StandardDrawerGroup.DrawerData.getAcceptingRemainingCapacity(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:326)
- [ ] `StandardDrawerGroup.DrawerData.canItemBeStored(2)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:336)
- [ ] `StandardDrawerGroup.DrawerData.canItemBeStoredManual(2)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:341)
- [ ] `StandardDrawerGroup.DrawerData.canItemBeStored(3)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:346)
- [ ] `StandardDrawerGroup.DrawerData.canItemBeExtracted(2)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:361)
- [ ] `StandardDrawerGroup.DrawerData.isEmpty(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:370)
- [ ] `StandardDrawerGroup.DrawerData.reset(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:375)
- [ ] `StandardDrawerGroup.DrawerData.serializeNBT(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:385)
- [ ] `StandardDrawerGroup.DrawerData.deserializeNBT(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:393)
- [ ] `StandardDrawerGroup.DrawerData.syncAttributes(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:399)
- [ ] `StandardDrawerGroup.DrawerData.getStackCapacity(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:405)
- [ ] `StandardDrawerGroup.DrawerData.onItemChanged(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:409)
- [ ] `StandardDrawerGroup.DrawerData.onAmountChanged(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:412)
- [ ] `StandardDrawerGroup.DrawerData.canDetach(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:415)
- [ ] `StandardDrawerGroup.DrawerData.isMissing(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:420)
- [ ] `StandardDrawerGroup.DrawerData.setDetached(1)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:425)
- [ ] `StandardDrawerGroup.DrawerData.copy(0)`: its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java:436)

### `UpgradeData`: it extends `BlockEntityDataShim`

- [ ] `UpgradeData.UpgradeData(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:46)
- [ ] `UpgradeData.setDrawerAttributes(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:53)
- [ ] `UpgradeData.getSlotCount(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:58)
- [ ] `UpgradeData.getUpgrade(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:62)
- [ ] `UpgradeData.hasEmptySlot(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:68)
- [ ] `UpgradeData.addUpgrade(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:72)
- [ ] `UpgradeData.setUpgrade(2)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:82)
- [ ] `UpgradeData.canAddUpgrade(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:117)
- [ ] `UpgradeData.canRemoveUpgrade(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:143)
- [ ] `UpgradeData.canSwapUpgrade(2)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:148)
- [ ] `UpgradeData.getStorageMultiplier(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:152)
- [ ] `UpgradeData.getMagnetRange(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:156)
- [ ] `UpgradeData.getMagnetActiveRate(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:164)
- [ ] `UpgradeData.getMagnetIdleRate(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:168)
- [ ] `UpgradeData.getRedstoneType(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:172)
- [ ] `UpgradeData.hasOneStackUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:176)
- [ ] `UpgradeData.hasUnlimitedUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:180)
- [ ] `UpgradeData.hasVendingUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:184)
- [ ] `UpgradeData.hasConversionUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:188)
- [ ] `UpgradeData.hasIlluminationUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:192)
- [ ] `UpgradeData.hasbalancedFillUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:196)
- [ ] `UpgradeData.hasHopperUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:200)
- [ ] `UpgradeData.hasMagnetUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:204)
- [ ] `UpgradeData.hasRemoteUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:208)
- [ ] `UpgradeData.hasPortabilityUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:212)
- [ ] `UpgradeData.getNextUpgradeSlot(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:222)
- [ ] `UpgradeData.getRemoteUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:232)
- [ ] `UpgradeData.unbindRemoteUpgrade(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:246)
- [ ] `UpgradeData.updateRemoteUpgradeBinding(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:258)
- [ ] `UpgradeData.syncUpgrades(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:270)
- [ ] `UpgradeData.syncStorageMultiplier(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:323)
- [ ] `UpgradeData.syncMagnetRange(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:338)
- [ ] `UpgradeData.syncRedstoneLevel(0)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:376)
- [ ] `UpgradeData.read(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:387)
- [ ] `UpgradeData.write(1)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:400)
- [ ] `UpgradeData.onUpgradeChanged(2)`: its class isn't translated (it extends `BlockEntityDataShim`) (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java:411)

### `FrameHelper`

- [ ] `FrameHelper.makeFramedItem(5)`: it uses IFramedBlock, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/block/tile/util/FrameHelper.java:14)

### `BasicDrawerAttributes`: it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)

- [ ] `BasicDrawerAttributes.canItemLock(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:22)
- [ ] `BasicDrawerAttributes.isItemLocked(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:27)
- [ ] `BasicDrawerAttributes.setItemLocked(2)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:32)
- [ ] `BasicDrawerAttributes.isConcealed(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:47)
- [ ] `BasicDrawerAttributes.setIsConcealed(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:52)
- [ ] `BasicDrawerAttributes.isVoid(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:62)
- [ ] `BasicDrawerAttributes.setIsVoid(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:67)
- [ ] `BasicDrawerAttributes.hasFillLevel(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:77)
- [ ] `BasicDrawerAttributes.setHasFillLevel(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:82)
- [ ] `BasicDrawerAttributes.isShowingQuantity(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:92)
- [ ] `BasicDrawerAttributes.setIsShowingQuantity(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:97)
- [ ] `BasicDrawerAttributes.getPriority(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:107)
- [ ] `BasicDrawerAttributes.setPriority(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:112)
- [ ] `BasicDrawerAttributes.isUnlimitedStorage(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:122)
- [ ] `BasicDrawerAttributes.setIsUnlimitedStorage(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:127)
- [ ] `BasicDrawerAttributes.isUnlimitedVending(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:137)
- [ ] `BasicDrawerAttributes.setIsUnlimitedVending(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:142)
- [ ] `BasicDrawerAttributes.isDictConvertible(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:152)
- [ ] `BasicDrawerAttributes.setIsDictConvertible(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:157)
- [ ] `BasicDrawerAttributes.isBalancedFill(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:167)
- [ ] `BasicDrawerAttributes.setIsBalancedFill(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:172)
- [ ] `BasicDrawerAttributes.isHopper(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:182)
- [ ] `BasicDrawerAttributes.setIsHopper(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:187)
- [ ] `BasicDrawerAttributes.isMagnet(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:197)
- [ ] `BasicDrawerAttributes.setIsMagnet(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:202)
- [ ] `BasicDrawerAttributes.isSuspended(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:212)
- [ ] `BasicDrawerAttributes.setIsSuspended(1)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:217)
- [ ] `BasicDrawerAttributes.onAttributeChanged(0)`: its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)) (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java:227)

### `Capabilities`

- [ ] `Capabilities.DRAWER_ATTRIBUTES`: it uses ChameleonCapability, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/capabilities/Capabilities.java:12)
- [ ] `Capabilities.DRAWER_GROUP`: it uses ChameleonCapability, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/capabilities/Capabilities.java:14)
- [ ] `Capabilities.ITEM_REPOSITORY`: it uses ChameleonCapability, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/capabilities/Capabilities.java:16)
- [ ] `Capabilities.ITEM_HANDLER`: it uses ChameleonCapability, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/capabilities/Capabilities.java:18)

### `DrawerItemHandler`: it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)

- [ ] `DrawerItemHandler.DrawerItemHandler(1)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:15)
- [ ] `DrawerItemHandler.getSlots(0)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:19)
- [ ] `DrawerItemHandler.getStackInSlot(1)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:24)
- [ ] `DrawerItemHandler.insertItem(3)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:46)
- [ ] `DrawerItemHandler.insertItemFullScan(2)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:77)
- [ ] `DrawerItemHandler.insertItemInternal(3)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:94)
- [ ] `DrawerItemHandler.extractItem(3)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:116)
- [ ] `DrawerItemHandler.getSlotLimit(1)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:138)
- [ ] `DrawerItemHandler.isItemValid(2)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:157)
- [ ] `DrawerItemHandler.isSuspended(0)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:162)
- [ ] `DrawerItemHandler.slotIsVirtual(1)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:171)
- [ ] `DrawerItemHandler.stackResult(2)`: its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java:175)

### `DrawerItemRepository`: it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)

- [ ] `DrawerItemRepository.DrawerItemRepository(1)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java:17)
- [ ] `DrawerItemRepository.getAllItems(0)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java:21)
- [ ] `DrawerItemRepository.insertItem(3)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java:45)
- [ ] `DrawerItemRepository.extractItem(4)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java:101)
- [ ] `DrawerItemRepository.getStoredItemCount(2)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java:131)
- [ ] `DrawerItemRepository.getRemainingItemCapacity(2)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java:152)
- [ ] `DrawerItemRepository.getItemCapacity(2)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java:173)
- [ ] `DrawerItemRepository.testPredicateInsert(3)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java:194)
- [ ] `DrawerItemRepository.testPredicateExtract(3)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java:200)
- [ ] `DrawerItemRepository.stackResult(2)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java:206)
- [ ] `DrawerItemRepository.isSuspended(0)`: its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)) (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java:212)

### `PlatformCapabilities`

- [ ] `PlatformCapabilities.DRAWER_ATTRIBUTES`: it uses FabricCapability, which isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/capabilities/PlatformCapabilities.java:20)
- [ ] `PlatformCapabilities.DRAWER_GROUP`: it uses FabricCapability, which isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/capabilities/PlatformCapabilities.java:21)
- [ ] `PlatformCapabilities.ITEM_REPOSITORY`: it uses FabricCapability, which isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/capabilities/PlatformCapabilities.java:22)
- [ ] `PlatformCapabilities.ITEM_HANDLER`: it uses FabricCapability, which isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/capabilities/PlatformCapabilities.java:23)
- [ ] `PlatformCapabilities.cast(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/capabilities/PlatformCapabilities.java:25)
- [ ] `PlatformCapabilities.initHandlers(0)`: it calls FabricCapabilities.reigsterCapability, which isn't translated (com/jaquadro/minecraft/storagedrawers/capabilities/PlatformCapabilities.java:29)

### `ClientUtil`

- [x] `ClientUtil.getLocalPlayer(0)` in game_side.rs (com/jaquadro/minecraft/storagedrawers/client/ClientUtil.java:8)
- [ ] `ClientUtil.getLocalGameProfile(0)`: it uses Minecraft's `GameProfile` (com/jaquadro/minecraft/storagedrawers/client/ClientUtil.java:12)

### `ClientDetachedDrawerTooltip`: it implements Minecraft's `ClientTooltipComponent`

- [x] `ClientDetachedDrawerTooltip.BACKGROUND_SPRITE` (com/jaquadro/minecraft/storagedrawers/client/gui/ClientDetachedDrawerTooltip.java:15)
- [ ] `ClientDetachedDrawerTooltip.ClientDetachedDrawerTooltip(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientDetachedDrawerTooltip.java:19)
- [ ] `ClientDetachedDrawerTooltip.getHeight(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientDetachedDrawerTooltip.java:24)
- [ ] `ClientDetachedDrawerTooltip.getWidth(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientDetachedDrawerTooltip.java:28)
- [ ] `ClientDetachedDrawerTooltip.backgroundWidth(0)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientDetachedDrawerTooltip.java:32)
- [ ] `ClientDetachedDrawerTooltip.backgroundHeight(0)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientDetachedDrawerTooltip.java:36)
- [ ] `ClientDetachedDrawerTooltip.extractImage(6)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientDetachedDrawerTooltip.java:40)
- [ ] `ClientDetachedDrawerTooltip.renderSlot(4)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientDetachedDrawerTooltip.java:52)

### `ClientKeyringTooltip`: it implements Minecraft's `ClientTooltipComponent`

- [x] `ClientKeyringTooltip.SLOT_HIGHLIGHT_BACK_SPRITE` (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:17)
- [x] `ClientKeyringTooltip.SLOT_HIGHLIGHT_FRONT_SPRITE` (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:18)
- [x] `ClientKeyringTooltip.SLOT_BACKGROUND_SPRITE` (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:19)
- [x] `ClientKeyringTooltip.SLOT_MARGIN` (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:20)
- [x] `ClientKeyringTooltip.SLOT_SIZE` (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:21)
- [x] `ClientKeyringTooltip.GRID_WIDTH` (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:22)
- [ ] `ClientKeyringTooltip.ClientKeyringTooltip(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:25)
- [ ] `ClientKeyringTooltip.getHeight(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:29)
- [ ] `ClientKeyringTooltip.getWidth(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:33)
- [ ] `ClientKeyringTooltip.backgroundWidth(0)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:37)
- [ ] `ClientKeyringTooltip.backgroundHeight(0)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:41)
- [ ] `ClientKeyringTooltip.getContentXOffset(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:45)
- [ ] `ClientKeyringTooltip.itemGridHeight(0)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:49)
- [ ] `ClientKeyringTooltip.gridSizeY(0)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:53)
- [ ] `ClientKeyringTooltip.slotCount(0)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:57)
- [ ] `ClientKeyringTooltip.extractImage(6)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:61)
- [ ] `ClientKeyringTooltip.renderEmptyBundleTooltip(6)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:69)
- [ ] `ClientKeyringTooltip.renderBundleWithItemsTooltip(6)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:72)
- [ ] `ClientKeyringTooltip.getShownItems(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:95)
- [x] `ClientKeyringTooltip.shouldRenderSurplusText(3)` (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:100)
- [ ] `ClientKeyringTooltip.getAmountOfHiddenItems(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:104)
- [x] `ClientKeyringTooltip.shouldRenderItemSlot(2)` in game_side.rs (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:108)
- [x] `ClientKeyringTooltip.renderCount(5)` in game_side.rs (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:112)
- [ ] `ClientKeyringTooltip.renderSlot(7)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:116)
- [ ] `ClientKeyringTooltip.drawSelectedItemTooltip(5)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java:133)

### `BakedModelProvider`

- [ ] `BakedModelProvider.makeStandardDrawerModel(1)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/BakedModelProvider.java:13)
- [ ] `BakedModelProvider.makeFramedDrawerModel(2)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/BakedModelProvider.java:18)
- [ ] `BakedModelProvider.makeFramedStandardDrawerModel(1)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/BakedModelProvider.java:25)
- [ ] `BakedModelProvider.makeFramedCompDrawerModel(2)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/BakedModelProvider.java:29)
- [ ] `BakedModelProvider.makeFramedComp2DrawerModel(1)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/BakedModelProvider.java:36)
- [ ] `BakedModelProvider.makeFramedComp3DrawerModel(1)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/BakedModelProvider.java:40)
- [ ] `BakedModelProvider.makeFramedTrimModel(1)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/BakedModelProvider.java:44)
- [ ] `BakedModelProvider.makeFramedControllerModel(1)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/BakedModelProvider.java:49)
- [ ] `BakedModelProvider.makeFramedControllerIOModel(1)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/BakedModelProvider.java:54)

### `DrawerModelContext`: it extends `FramedModelContext`

- [ ] `DrawerModelContext.DrawerModelContext(1)`: its class isn't translated (it extends `FramedModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/DrawerModelContext.java:15)
- [ ] `DrawerModelContext.DrawerModelContext(2)`: its class isn't translated (it extends `FramedModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/DrawerModelContext.java:19)
- [ ] `DrawerModelContext.attr(0)`: its class isn't translated (it extends `FramedModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/DrawerModelContext.java:23)
- [ ] `DrawerModelContext.group(0)`: its class isn't translated (it extends `FramedModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/DrawerModelContext.java:27)
- [ ] `DrawerModelContext.protectable(0)`: its class isn't translated (it extends `FramedModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/DrawerModelContext.java:31)
- [ ] `DrawerModelContext.protectable(1)`: its class isn't translated (it extends `FramedModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/DrawerModelContext.java:35)
- [ ] `DrawerModelContext.group(1)`: its class isn't translated (it extends `FramedModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/DrawerModelContext.java:40)
- [ ] `DrawerModelContext.attr(1)`: its class isn't translated (it extends `FramedModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/DrawerModelContext.java:45)
- [ ] `DrawerModelContext.materialData(1)`: its class isn't translated (it extends `FramedModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/DrawerModelContext.java:50)

### `FramedModelContext`: it extends `ModelContext`

- [ ] `FramedModelContext.FramedModelContext(2)`: its class isn't translated (it extends `ModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/FramedModelContext.java:10)
- [ ] `FramedModelContext.FramedModelContext(1)`: its class isn't translated (it extends `ModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/FramedModelContext.java:14)
- [ ] `FramedModelContext.materialData(0)`: its class isn't translated (it extends `ModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/FramedModelContext.java:18)
- [ ] `FramedModelContext.materialData(1)`: its class isn't translated (it extends `ModelContext`) (com/jaquadro/minecraft/storagedrawers/client/model/context/FramedModelContext.java:22)

### `ModelContext`: it uses Minecraft's `BlockState`

- [ ] `ModelContext.ModelContext(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/client/model/context/ModelContext.java:11)
- [ ] `ModelContext.ModelContext(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/client/model/context/ModelContext.java:16)
- [ ] `ModelContext.ModelContext(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/client/model/context/ModelContext.java:21)
- [ ] `ModelContext.state(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/client/model/context/ModelContext.java:26)
- [ ] `ModelContext.randomSource(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/client/model/context/ModelContext.java:30)

### `CombinedModelDecorator`: it's generic

- [ ] `CombinedModelDecorator.add(1)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/CombinedModelDecorator.java:15)
- [ ] `CombinedModelDecorator.shouldRenderItem(0)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/CombinedModelDecorator.java:19)
- [ ] `CombinedModelDecorator.shouldRenderBase(1)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/CombinedModelDecorator.java:30)
- [ ] `CombinedModelDecorator.shouldRenderBase(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/CombinedModelDecorator.java:41)
- [ ] `CombinedModelDecorator.getRenderTypes(1)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/CombinedModelDecorator.java:52)
- [ ] `CombinedModelDecorator.emitQuads(3)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/CombinedModelDecorator.java:75)
- [ ] `CombinedModelDecorator.emitItemQuads(4)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/CombinedModelDecorator.java:82)

### `DecoratorRenderType`

- [ ] `DecoratorRenderType.fromItemType(1)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/decorator/DecoratorRenderType.java:12)
- [ ] `DecoratorRenderType.fromItemType(1)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/decorator/DecoratorRenderType.java:25)
- [ ] `DecoratorRenderType.toChunkType(1)`: it uses Minecraft's `ChunkSectionLayer` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/DecoratorRenderType.java:33)
- [ ] `DecoratorRenderType.toItemType(1)`: it uses Minecraft's `RenderType` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/DecoratorRenderType.java:45)

### `DrawerModelDecorator`: it extends `ModelDecorator`

- [x] `DrawerModelDecorator.defaultRenderTypes` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/DrawerModelDecorator.java:23)
- [ ] `DrawerModelDecorator.DrawerModelDecorator(1)`: its class isn't translated (it extends `ModelDecorator`) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/DrawerModelDecorator.java:25)
- [ ] `DrawerModelDecorator.getRenderTypes(1)`: its class isn't translated (it extends `ModelDecorator`) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/DrawerModelDecorator.java:29)
- [ ] `DrawerModelDecorator.emitQuads(3)`: its class isn't translated (it extends `ModelDecorator`) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/DrawerModelDecorator.java:34)
- [ ] `DrawerModelDecorator.emitDecoratedQuads(3)`: its class isn't translated (it extends `ModelDecorator`) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/DrawerModelDecorator.java:44)

### `MaterialModelDecorator`: it's generic

- [ ] `MaterialModelDecorator.replacementCache`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:31)
- [x] `MaterialModelDecorator.defaultRenderList` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:32)
- [x] `MaterialModelDecorator.defaultShadedRenderList` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:33)
- [ ] `MaterialModelDecorator.MaterialModelDecorator(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:35)
- [ ] `MaterialModelDecorator.shouldRenderItem(0)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:40)
- [ ] `MaterialModelDecorator.shouldRenderBase(1)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:45)
- [ ] `MaterialModelDecorator.shouldRenderBase(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:56)
- [ ] `MaterialModelDecorator.getRenderTypes(1)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:61)
- [ ] `MaterialModelDecorator.emitQuads(3)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:66)
- [ ] `MaterialModelDecorator.emitItemQuads(4)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:88)
- [ ] `MaterialModelDecorator.getReplacementModel(3)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:110)
- [ ] `MaterialModelDecorator.emitFramedQuads(3)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:138)
- [ ] `MaterialModelDecorator.emitFramedOverlayQuads(3)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:169)
- [ ] `MaterialModelDecorator.getStoreModel(2)`: it has no body (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:182)
- [ ] `MaterialModelDecorator.getStoreModel(3)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:184)

### `MaterialModelDecorator.Facing`: it's generic

- [ ] `MaterialModelDecorator.Facing.Facing(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:189)
- [ ] `MaterialModelDecorator.Facing.getStoreModel(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:193)

### `MaterialModelDecorator.FacingSized`: it's generic

- [ ] `MaterialModelDecorator.FacingSized.FacingSized(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:201)
- [ ] `MaterialModelDecorator.FacingSized.getStoreModel(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:205)

### `MaterialModelDecorator.FacingSizedOpen`: it's generic

- [ ] `MaterialModelDecorator.FacingSizedOpen.FacingSizedOpen(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:218)
- [ ] `MaterialModelDecorator.FacingSizedOpen.getStoreModel(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:222)

### `MaterialModelDecorator.FacingSizedSlotted`: it's generic

- [ ] `MaterialModelDecorator.FacingSizedSlotted.FacingSizedSlotted(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:237)
- [ ] `MaterialModelDecorator.FacingSizedSlotted.getStoreModel(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:241)

### `MaterialModelDecorator.Single`: it's generic

- [ ] `MaterialModelDecorator.Single.Single(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:256)
- [ ] `MaterialModelDecorator.Single.getStoreModel(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java:260)

### `ModelDecorator`: it's generic

- [ ] `ModelDecorator.shouldRenderItem(0)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/ModelDecorator.java:12)
- [ ] `ModelDecorator.shouldRenderBase(1)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/ModelDecorator.java:16)
- [ ] `ModelDecorator.shouldRenderBase(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/ModelDecorator.java:20)
- [ ] `ModelDecorator.getRenderTypes(1)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/ModelDecorator.java:24)
- [ ] `ModelDecorator.emitQuads(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/ModelDecorator.java:28)
- [ ] `ModelDecorator.emitQuads(3)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/ModelDecorator.java:32)
- [ ] `ModelDecorator.emitItemQuads(3)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/ModelDecorator.java:35)
- [ ] `ModelDecorator.emitItemQuads(4)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/decorator/ModelDecorator.java:39)

### `DrawerModelGeometry`

- [x] `DrawerModelGeometry.geometryDataLoaded` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelGeometry.java:24)
- [ ] `DrawerModelGeometry.loadGeometryData(0)`: it calls DrawerModelGeometry.populateGeometryData, which isn't translated (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelGeometry.java:26)
- [ ] `DrawerModelGeometry.populateGeometryData(5)`: it uses BlockDrawers, which isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelGeometry.java:102)
- [ ] `DrawerModelGeometry.populateGeometryData(3)`: it uses BlockDrawers, which isn't translated (it extends `FaceSlotBlock`) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelGeometry.java:120)
- [ ] `DrawerModelGeometry.getBlockModel(1)`: it uses Minecraft's `CuboidModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelGeometry.java:149)
- [ ] `DrawerModelGeometry.getElements(1)`: it uses Minecraft's `CuboidModelElement` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelGeometry.java:164)

### `DrawerModelStore`

- [ ] `DrawerModelStore.FramedStandardDrawerMaterials`: it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:22)
- [ ] `DrawerModelStore.FramedComp2DrawerMaterials`: it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:27)
- [ ] `DrawerModelStore.FramedComp3DrawerMaterials`: it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:32)
- [ ] `DrawerModelStore.FramedControllerMaterials`: it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:37)
- [ ] `DrawerModelStore.FramedControllerIOMaterials`: it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:42)
- [ ] `DrawerModelStore.FramedTrimMaterials`: it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:46)
- [ ] `DrawerModelStore.modelStore`: it uses Minecraft's `BlockState` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:49)
- [ ] `DrawerModelStore.replacementStore`: it uses Minecraft's `BlockState` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:50)
- [ ] `DrawerModelStore.locationStore`: it uses Minecraft's `BlockState` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:51)
- [ ] `DrawerModelStore.INSTANCE`: it uses DecorationSet, which isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:52)
- [ ] `DrawerModelStore.missingSlots1`: it uses DynamicPart, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:53)
- [ ] `DrawerModelStore.missingSlots2`: it uses DynamicPart, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:54)
- [ ] `DrawerModelStore.missingSlots4`: it uses DynamicPart, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:57)
- [ ] `DrawerModelStore.missingSlots`: it uses DynamicPart, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:63)
- [ ] `DrawerModelStore.addLocation(1)`: it uses Minecraft's `BlockState` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:67)
- [x] `DrawerModelStore.getVariant(0)` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:73)
- [ ] `DrawerModelStore.getVariant(1)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:77)
- [ ] `DrawerModelStore.getVariant(1)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:81)
- [ ] `DrawerModelStore.getVariant(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:85)
- [ ] `DrawerModelStore.getVariant(3)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:89)
- [ ] `DrawerModelStore.getVariant(3)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:93)
- [ ] `DrawerModelStore.getSlotVariant(3)`: it uses Minecraft's `Direction` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:97)
- [ ] `DrawerModelStore.getVariant(1)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:101)
- [ ] `DrawerModelStore.getVariant(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:105)
- [ ] `DrawerModelStore.getVariant(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:109)
- [ ] `DrawerModelStore.getVariant(3)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:113)
- [ ] `DrawerModelStore.getVariant(4)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:117)
- [ ] `DrawerModelStore.getVariant(4)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:121)
- [ ] `DrawerModelStore.getSlotVariant(4)`: it uses DynamicPart, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:125)
- [ ] `DrawerModelStore.getModelLocations(0)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:129)
- [ ] `DrawerModelStore.tryAddModel(2)`: it uses Minecraft's `BlockState` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:133)
- [ ] `DrawerModelStore.getModel(1)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:141)
- [ ] `DrawerModelStore.getModel(1)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:150)
- [ ] `DrawerModelStore.getModel(1)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:154)
- [ ] `DrawerModelStore.getModel(2)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:158)
- [ ] `DrawerModelStore.getModel(2)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:162)
- [ ] `DrawerModelStore.getModel(3)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:166)
- [ ] `DrawerModelStore.getModel(4)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:170)
- [ ] `DrawerModelStore.getModel(4)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:174)
- [ ] `DrawerModelStore.getReplacementModel(2)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:178)
- [ ] `DrawerModelStore.getReplacementModel(2)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:202)
- [ ] `DrawerModelStore.getReplacementModel(5)`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:206)

### `DrawerModelStore.DecorationSet`: it uses Minecraft's `BlockState`

- [ ] `DrawerModelStore.DecorationSet.DecorationSet(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:226)
- [ ] `DrawerModelStore.DecorationSet.add(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:236)
- [ ] `DrawerModelStore.DecorationSet.add(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:248)
- [ ] `DrawerModelStore.DecorationSet.addOverlay(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:523)
- [ ] `DrawerModelStore.DecorationSet.isTargetedModel(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:527)

### `DrawerModelStore.DynamicPart`: its constants carry values

- [ ] `DrawerModelStore.DynamicPart.DynamicPart(1)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:576)
- [ ] `DrawerModelStore.DynamicPart.getName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:580)

### `DrawerModelStore.FrameMatSet`: it uses DynamicPart, which isn't translated (its constants carry values)

- [ ] `DrawerModelStore.FrameMatSet.sidePart(0)`: its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:592)
- [ ] `DrawerModelStore.FrameMatSet.trimPart(0)`: its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:596)
- [ ] `DrawerModelStore.FrameMatSet.frontPart(0)`: its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:600)
- [ ] `DrawerModelStore.FrameMatSet.shadeFrontPart(0)`: its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:604)
- [ ] `DrawerModelStore.FrameMatSet.shadeSidePart(0)`: its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:608)
- [ ] `DrawerModelStore.FrameMatSet.sidePart(1)`: its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:612)
- [ ] `DrawerModelStore.FrameMatSet.trimPart(1)`: its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:617)
- [ ] `DrawerModelStore.FrameMatSet.frontPart(1)`: its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:622)
- [ ] `DrawerModelStore.FrameMatSet.shadeFrontPart(1)`: its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:627)
- [ ] `DrawerModelStore.FrameMatSet.shadeSidePart(1)`: its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)) (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java:632)

### `ItemModelStore`

- [ ] `ItemModelStore.models`: it uses Minecraft's `BlockState` (com/jaquadro/minecraft/storagedrawers/client/model/ItemModelStore.java:9)

### `ModelContextSupplier`: it's an interface

- [ ] `ModelContextSupplier.makeContext(3)`: it has no body (com/jaquadro/minecraft/storagedrawers/client/model/ModelContextSupplier.java:10)
- [ ] `ModelContextSupplier.makeContext(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/client/model/ModelContextSupplier.java:12)

### `ModelLoadPlugin`: it implements Fabric's `ModelLoadingPlugin`

- [ ] `ModelLoadPlugin.initialize(1)`: its class isn't translated (it implements Fabric's `ModelLoadingPlugin`) (com/jaquadro/minecraft/storagedrawers/client/model/ModelLoadPlugin.java:20)

### `ModelLoadPlugin.UnbakedProxyModel`: it implements Minecraft's `UnbakedRoot`

- [ ] `ModelLoadPlugin.UnbakedProxyModel.UnbakedProxyModel(2)`: its class isn't translated (it implements Minecraft's `UnbakedRoot`) (com/jaquadro/minecraft/storagedrawers/client/model/ModelLoadPlugin.java:41)
- [ ] `ModelLoadPlugin.UnbakedProxyModel.bake(2)`: its class isn't translated (it implements Minecraft's `UnbakedRoot`) (com/jaquadro/minecraft/storagedrawers/client/model/ModelLoadPlugin.java:46)
- [ ] `ModelLoadPlugin.UnbakedProxyModel.resolveDependencies(1)`: its class isn't translated (it implements Minecraft's `UnbakedRoot`) (com/jaquadro/minecraft/storagedrawers/client/model/ModelLoadPlugin.java:90)
- [ ] `ModelLoadPlugin.UnbakedProxyModel.visualEqualityGroup(1)`: its class isn't translated (it implements Minecraft's `UnbakedRoot`) (com/jaquadro/minecraft/storagedrawers/client/model/ModelLoadPlugin.java:94)

### `ParentModel`: it implements Minecraft's `BlockStateModel`

- [ ] `ParentModel.ParentModel(1)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/jaquadro/minecraft/storagedrawers/client/model/ParentModel.java:14)
- [ ] `ParentModel.collectParts(2)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/jaquadro/minecraft/storagedrawers/client/model/ParentModel.java:18)
- [ ] `ParentModel.particleMaterial(0)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/jaquadro/minecraft/storagedrawers/client/model/ParentModel.java:22)
- [ ] `ParentModel.materialFlags(0)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/jaquadro/minecraft/storagedrawers/client/model/ParentModel.java:26)

### `PlatformDecoratedModel`: it's generic

- [ ] `PlatformDecoratedModel.meshCache`: it uses Minecraft's `BlockStateModel` (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:65)
- [x] `PlatformDecoratedModel.decoratorRenderTypes` (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:66)
- [ ] `PlatformDecoratedModel.PlatformDecoratedModel(3)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:70)
- [ ] `PlatformDecoratedModel.PlatformDecoratedModel(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:77)
- [ ] `PlatformDecoratedModel.emitQuads(6)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:84)
- [ ] `PlatformDecoratedModel.getMesh(4)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:126)
- [ ] `PlatformDecoratedModel.buildMesh(4)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:136)

### `PlatformDecoratedModel.PlatformDecoratedItemModel`: it implements Minecraft's `ItemModel`

- [ ] `PlatformDecoratedModel.PlatformDecoratedItemModel.PlatformDecoratedItemModel(3)`: its class isn't translated (it implements Minecraft's `ItemModel`) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:169)
- [ ] `PlatformDecoratedModel.PlatformDecoratedItemModel.update(7)`: its class isn't translated (it implements Minecraft's `ItemModel`) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:176)
- [ ] `PlatformDecoratedModel.PlatformDecoratedItemModel.setProperty(3)`: it's generic (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:256)

### `PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked`: it implements `Unbaked`

- [ ] `PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:262)
- [ ] `PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.Unbaked(2)`: its class isn't translated (it implements `Unbaked`) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:261)
- [ ] `PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.type(0)`: its class isn't translated (it implements `Unbaked`) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:270)
- [ ] `PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.bake(2)`: its class isn't translated (it implements `Unbaked`) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:274)
- [ ] `PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.resolveDependencies(1)`: its class isn't translated (it implements `Unbaked`) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:282)
- [ ] `PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.model(0)`: its class isn't translated (it implements `Unbaked`) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:261)
- [ ] `PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.variant(0)`: its class isn't translated (it implements `Unbaked`) (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java:261)

### `SpriteReplacementModel`: it extends `ParentModel`

- [ ] `SpriteReplacementModel.SpriteReplacementModel(2)`: its class isn't translated (it extends `ParentModel`) (com/jaquadro/minecraft/storagedrawers/client/model/SpriteReplacementModel.java:26)
- [ ] `SpriteReplacementModel.SpriteReplacementModel(3)`: its class isn't translated (it extends `ParentModel`) (com/jaquadro/minecraft/storagedrawers/client/model/SpriteReplacementModel.java:31)
- [ ] `SpriteReplacementModel.SpriteReplacementModel(3)`: its class isn't translated (it extends `ParentModel`) (com/jaquadro/minecraft/storagedrawers/client/model/SpriteReplacementModel.java:37)
- [ ] `SpriteReplacementModel.SpriteReplacementModel(2)`: its class isn't translated (it extends `ParentModel`) (com/jaquadro/minecraft/storagedrawers/client/model/SpriteReplacementModel.java:48)
- [ ] `SpriteReplacementModel.collectParts(2)`: its class isn't translated (it extends `ParentModel`) (com/jaquadro/minecraft/storagedrawers/client/model/SpriteReplacementModel.java:52)
- [ ] `SpriteReplacementModel.particleMaterial(0)`: its class isn't translated (it extends `ParentModel`) (com/jaquadro/minecraft/storagedrawers/client/model/SpriteReplacementModel.java:71)

### `BlockEntityDrawersRenderer`: it implements Minecraft's `BlockEntityRenderer`

- [ ] `BlockEntityDrawersRenderer.TEXT_COLOR_TRANSPARENT`: it calls Minecraft's `ARGB`.color (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:50)
- [ ] `BlockEntityDrawersRenderer.ITEM_LIGHT_ROTATION_3D`: it creates `Matrix3f` (org.joml) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:51)
- [x] `BlockEntityDrawersRenderer.sideRotationY2D` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:52)
- [ ] `BlockEntityDrawersRenderer.BlockEntityDrawersRenderer(1)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:54)
- [ ] `BlockEntityDrawersRenderer.createRenderState(0)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:58)
- [ ] `BlockEntityDrawersRenderer.extractRenderState(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:62)
- [ ] `BlockEntityDrawersRenderer.submit(4)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:91)
- [ ] `BlockEntityDrawersRenderer.playerBehindBlock(2)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:110)
- [ ] `BlockEntityDrawersRenderer.renderFastItemSet(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:127)
- [ ] `BlockEntityDrawersRenderer.renderText(7)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:153)
- [ ] `BlockEntityDrawersRenderer.renderFastItem(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:197)
- [ ] `BlockEntityDrawersRenderer.alignRendering(2)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:228)
- [ ] `BlockEntityDrawersRenderer.getRotationYForSide2D(1)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:234)
- [ ] `BlockEntityDrawersRenderer.renderIndicator(4)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:238)

### `BlockEntityDrawersRenderer.QuadBuilder`: it implements Minecraft's `CustomGeometryRenderer`

- [x] `BlockEntityDrawersRenderer.QuadBuilder.TEXTURE_IND_1` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:250)
- [x] `BlockEntityDrawersRenderer.QuadBuilder.TEXTURE_IND_2` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:251)
- [x] `BlockEntityDrawersRenderer.QuadBuilder.TEXTURE_IND_4` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:252)
- [x] `BlockEntityDrawersRenderer.QuadBuilder.TEXTURE_IND_COMP_3` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:253)
- [x] `BlockEntityDrawersRenderer.QuadBuilder.TEXTURE_IND_COMP_2` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:254)
- [ ] `BlockEntityDrawersRenderer.QuadBuilder.MAT_IND_1`: it uses Minecraft's `SpriteId` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:255)
- [ ] `BlockEntityDrawersRenderer.QuadBuilder.MAT_IND_2`: it uses Minecraft's `SpriteId` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:256)
- [ ] `BlockEntityDrawersRenderer.QuadBuilder.MAT_IND_4`: it uses Minecraft's `SpriteId` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:257)
- [ ] `BlockEntityDrawersRenderer.QuadBuilder.MAT_IND_COMP_3`: it uses Minecraft's `SpriteId` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:258)
- [ ] `BlockEntityDrawersRenderer.QuadBuilder.MAT_IND_COMP_2`: it uses Minecraft's `SpriteId` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:259)
- [ ] `BlockEntityDrawersRenderer.QuadBuilder.QuadBuilder(1)`: its class isn't translated (it implements Minecraft's `CustomGeometryRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:262)
- [ ] `BlockEntityDrawersRenderer.QuadBuilder.render(2)`: its class isn't translated (it implements Minecraft's `CustomGeometryRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:266)
- [x] `BlockEntityDrawersRenderer.QuadBuilder.getIndEnd(6)` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:326)
- [ ] `BlockEntityDrawersRenderer.QuadBuilder.addQuad(13)`: it uses Minecraft's `Pose` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:335)
- [ ] `BlockEntityDrawersRenderer.QuadBuilder.addVertex(9)`: it uses Minecraft's `Pose` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java:356)

### `BlockEntityFramingRenderer`: it implements Minecraft's `BlockEntityRenderer`

- [ ] `BlockEntityFramingRenderer.BlockEntityFramingRenderer(1)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityFramingRenderer.java:31)
- [ ] `BlockEntityFramingRenderer.createRenderState(0)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityFramingRenderer.java:35)
- [ ] `BlockEntityFramingRenderer.extractRenderState(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityFramingRenderer.java:39)
- [ ] `BlockEntityFramingRenderer.getRenderState(3)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityFramingRenderer.java:58)
- [ ] `BlockEntityFramingRenderer.submit(4)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityFramingRenderer.java:64)
- [ ] `BlockEntityFramingRenderer.renderSlot(9)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityFramingRenderer.java:75)
- [ ] `BlockEntityFramingRenderer.getRenderBoundingBox(1)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityFramingRenderer.java:129)

### `DrawersRenderState`: it extends Minecraft's `BlockEntityRenderState`


### `DrawersRenderState.SlotState`: it uses Minecraft's `ItemStackRenderState`

- [ ] `DrawersRenderState.SlotState.SlotState(3)`: its class isn't translated (it uses Minecraft's `ItemStackRenderState`) (com/jaquadro/minecraft/storagedrawers/client/renderer/state/DrawersRenderState.java:19)
- [ ] `DrawersRenderState.SlotState.itemState(0)`: its class isn't translated (it uses Minecraft's `ItemStackRenderState`) (com/jaquadro/minecraft/storagedrawers/client/renderer/state/DrawersRenderState.java:19)
- [ ] `DrawersRenderState.SlotState.count(0)`: its class isn't translated (it uses Minecraft's `ItemStackRenderState`) (com/jaquadro/minecraft/storagedrawers/client/renderer/state/DrawersRenderState.java:19)
- [ ] `DrawersRenderState.SlotState.limit(0)`: its class isn't translated (it uses Minecraft's `ItemStackRenderState`) (com/jaquadro/minecraft/storagedrawers/client/renderer/state/DrawersRenderState.java:19)

### `FramingRenderState`: it extends Minecraft's `BlockEntityRenderState`


### `ControllerBinding`

- [x] `ControllerBinding.EMPTY` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:12)
- [ ] `ControllerBinding.CODEC`: it uses Minecraft's `Codec` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:13)
- [ ] `ControllerBinding.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:22)
- [x] `ControllerBinding.ControllerBinding(4)` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:11)
- [ ] `ControllerBinding.ControllerBinding(1)`: it uses Minecraft's `BlockPos` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:34)
- [ ] `ControllerBinding.blockPos(0)`: it uses Minecraft's `BlockPos` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:38)
- [ ] `ControllerBinding.equals(1)`: it uses `Object` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:42)
- [ ] `ControllerBinding.hashCode(0)`: it calls `Objects` (java.util).hash (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:51)
- [x] `ControllerBinding.toString(0)` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:56)
- [x] `ControllerBinding.valid(0)` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:11)
- [x] `ControllerBinding.x(0)` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:11)
- [x] `ControllerBinding.y(0)` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:11)
- [x] `ControllerBinding.z(0)` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java:11)

### `DetachedDrawerContents`: it implements Minecraft's `TooltipComponent`

- [ ] `DetachedDrawerContents.EMPTY`: it uses DetachedDrawerContents, which isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:9)
- [ ] `DetachedDrawerContents.CODEC`: it uses Minecraft's `Codec` (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:10)
- [ ] `DetachedDrawerContents.DetachedDrawerContents(4)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:24)
- [ ] `DetachedDrawerContents.DetachedDrawerContents(3)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:31)
- [ ] `DetachedDrawerContents.getItem(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:35)
- [ ] `DetachedDrawerContents.getItemPrototype(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:39)
- [ ] `DetachedDrawerContents.getItemCount(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:43)
- [ ] `DetachedDrawerContents.getStackLimit(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:47)
- [ ] `DetachedDrawerContents.isHeavy(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:51)
- [ ] `DetachedDrawerContents.equals(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:55)
- [ ] `DetachedDrawerContents.hashCode(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:69)
- [ ] `DetachedDrawerContents.toString(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java:74)

### `DrawerCountData`

- [ ] `DrawerCountData.CODEC`: it uses Minecraft's `Codec` (com/jaquadro/minecraft/storagedrawers/components/item/DrawerCountData.java:8)
- [x] `DrawerCountData.EMPTY` (com/jaquadro/minecraft/storagedrawers/components/item/DrawerCountData.java:11)
- [x] `DrawerCountData.DrawerCountData(1)` (com/jaquadro/minecraft/storagedrawers/components/item/DrawerCountData.java:7)
- [ ] `DrawerCountData.equals(1)`: it uses `Object` (com/jaquadro/minecraft/storagedrawers/components/item/DrawerCountData.java:13)
- [ ] `DrawerCountData.hashCode(0)`: it calls `Objects` (java.util).hashCode (com/jaquadro/minecraft/storagedrawers/components/item/DrawerCountData.java:22)
- [x] `DrawerCountData.toString(0)` (com/jaquadro/minecraft/storagedrawers/components/item/DrawerCountData.java:27)
- [x] `DrawerCountData.count(0)` (com/jaquadro/minecraft/storagedrawers/components/item/DrawerCountData.java:7)

### `FrameData`: it uses Minecraft's `ItemStack`

- [ ] `FrameData.EMPTY`: it uses FrameData, which isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:11)
- [ ] `FrameData.CODEC`: it uses Minecraft's `Codec` (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:12)
- [ ] `FrameData.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:21)
- [ ] `FrameData.FrameData(4)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:10)
- [ ] `FrameData.FrameData(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:33)
- [ ] `FrameData.FrameData(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:37)
- [ ] `FrameData.asMaterialData(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:41)
- [ ] `FrameData.equals(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:45)
- [ ] `FrameData.hashCode(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:59)
- [ ] `FrameData.toString(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:67)
- [ ] `FrameData.base(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:10)
- [ ] `FrameData.side(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:10)
- [ ] `FrameData.trim(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:10)
- [ ] `FrameData.front(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java:10)

### `KeyringContents`: it implements Minecraft's `TooltipComponent`

- [ ] `KeyringContents.EMPTY`: it uses KeyringContents, which isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:19)
- [ ] `KeyringContents.CODEC`: it uses Minecraft's `Codec` (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:20)
- [ ] `KeyringContents.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:21)
- [x] `KeyringContents.NO_STACK_INDEX` (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:24)
- [x] `KeyringContents.NO_SELECTED_ITEM_INDEX` (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:25)
- [ ] `KeyringContents.KeyringContents(2)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:29)
- [ ] `KeyringContents.KeyringContents(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:34)
- [ ] `KeyringContents.getItemUnsafe(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:38)
- [ ] `KeyringContents.itemCopyStream(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:42)
- [ ] `KeyringContents.items(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:46)
- [ ] `KeyringContents.itemsCopy(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:50)
- [ ] `KeyringContents.size(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:54)
- [ ] `KeyringContents.isEmpty(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:58)
- [ ] `KeyringContents.getSelectedItem(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:62)
- [ ] `KeyringContents.hasSelectedItem(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:66)
- [ ] `KeyringContents.getMaxShowSize(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:70)
- [ ] `KeyringContents.getShowRowSize(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:74)
- [ ] `KeyringContents.getNumberOfItemsToShow(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:78)
- [ ] `KeyringContents.equals(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:88)
- [ ] `KeyringContents.hashCode(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:97)
- [ ] `KeyringContents.toString(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:102)

### `KeyringContents.Mutable`: it uses Minecraft's `ItemStack`

- [ ] `KeyringContents.Mutable.Mutable(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:111)
- [ ] `KeyringContents.Mutable.size(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:116)
- [ ] `KeyringContents.Mutable.clearItems(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:120)
- [ ] `KeyringContents.Mutable.findStackIndex(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:126)
- [ ] `KeyringContents.Mutable.getMaxAmountToAdd(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:136)
- [ ] `KeyringContents.Mutable.tryInsert(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:140)
- [ ] `KeyringContents.Mutable.tryTransfer(2)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:159)
- [ ] `KeyringContents.Mutable.toggleSelectedItem(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:165)
- [ ] `KeyringContents.Mutable.indexIsOutsideAllowedBounds(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:169)
- [ ] `KeyringContents.Mutable.removeOne(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:173)
- [ ] `KeyringContents.Mutable.toImmutable(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java:178)

### `CompTierRegistry`: it uses Record, which isn't translated (it uses Minecraft's `ItemStack`)

- [ ] `CompTierRegistry.INSTANCE`: it uses CompTierRegistry, which isn't translated (it uses Record, which isn't translated (it uses Minecraft's `ItemStack`)) (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java:15)
- [ ] `CompTierRegistry.initialize(0)`: it uses ModCommonConfig, which isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)) (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java:20)
- [ ] `CompTierRegistry.register(3)`: it uses Minecraft's `ItemStack` (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java:45)
- [ ] `CompTierRegistry.validateRuleSyntax(1)`: it calls `Integer`.parseInt (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java:63)
- [ ] `CompTierRegistry.register(1)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java:83)
- [ ] `CompTierRegistry.register(1)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java:87)
- [ ] `CompTierRegistry.unregisterUpperTarget(1)`: it uses Minecraft's `ItemStack` (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java:111)
- [ ] `CompTierRegistry.unregisterLowerTarget(1)`: it uses Minecraft's `ItemStack` (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java:122)
- [ ] `CompTierRegistry.findHigherTier(1)`: it uses Minecraft's `ItemStack` (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java:133)
- [ ] `CompTierRegistry.findLowerTier(1)`: it uses Minecraft's `ItemStack` (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java:147)

### `CompTierRegistry.Record`: it uses Minecraft's `ItemStack`

- [ ] `CompTierRegistry.Record.Record(3)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java:168)

### `ConfigItemList`: it uses Minecraft's `Item`

- [ ] `ConfigItemList.initialize(0)`: its class isn't translated (it uses Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java:18)
- [ ] `ConfigItemList.innerInitialize(0)`: its class isn't translated (it uses Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java:29)
- [ ] `ConfigItemList.isListed(1)`: its class isn't translated (it uses Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java:32)
- [ ] `ConfigItemList.registerNamespace(1)`: its class isn't translated (it uses Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java:51)
- [ ] `ConfigItemList.logRegisterNamespace(1)`: its class isn't translated (it uses Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java:62)
- [ ] `ConfigItemList.registerItem(1)`: its class isn't translated (it uses Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java:65)
- [ ] `ConfigItemList.logRegisterItem(1)`: its class isn't translated (it uses Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java:76)
- [ ] `ConfigItemList.register(1)`: its class isn't translated (it uses Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java:79)
- [ ] `ConfigItemList.register(1)`: its class isn't translated (it uses Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java:83)
- [ ] `ConfigItemList.unregisterNamespace(1)`: its class isn't translated (it uses Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java:99)
- [ ] `ConfigItemList.unregisterItem(1)`: its class isn't translated (it uses Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java:103)

### `ConversionRegistry`: it uses Minecraft's `TagKey`

- [ ] `ConversionRegistry.INSTANCE`: it uses ConversionRegistry, which isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:17)
- [ ] `ConversionRegistry.initialize(0)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:22)
- [ ] `ConversionRegistry.addBlacklist(1)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:45)
- [ ] `ConversionRegistry.addBlacklist(2)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:50)
- [ ] `ConversionRegistry.addBlacklist(1)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:54)
- [ ] `ConversionRegistry.addWhitelist(1)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:67)
- [ ] `ConversionRegistry.addWhitelist(2)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:71)
- [ ] `ConversionRegistry.addWhitelist(3)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:76)
- [ ] `ConversionRegistry.addWhitelist(2)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:80)
- [ ] `ConversionRegistry.addEquivGroup(1)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:93)
- [ ] `ConversionRegistry.itemsShareEquivGroup(2)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:113)
- [ ] `ConversionRegistry.getEquivItems(1)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:126)
- [ ] `ConversionRegistry.isEntryValid(1)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java:142)

### `MaterialBlacklist`: it extends `ConfigItemList`

- [ ] `MaterialBlacklist.INSTANCE`: it uses MaterialBlacklist, which isn't translated (it extends `ConfigItemList`) (com/jaquadro/minecraft/storagedrawers/config/MaterialBlacklist.java:8)
- [ ] `MaterialBlacklist.innerInitialize(0)`: its class isn't translated (it extends `ConfigItemList`) (com/jaquadro/minecraft/storagedrawers/config/MaterialBlacklist.java:10)
- [ ] `MaterialBlacklist.isBlacklisted(1)`: its class isn't translated (it extends `ConfigItemList`) (com/jaquadro/minecraft/storagedrawers/config/MaterialBlacklist.java:15)
- [ ] `MaterialBlacklist.logRegisterNamespace(1)`: its class isn't translated (it extends `ConfigItemList`) (com/jaquadro/minecraft/storagedrawers/config/MaterialBlacklist.java:19)
- [ ] `MaterialBlacklist.logRegisterItem(1)`: its class isn't translated (it extends `ConfigItemList`) (com/jaquadro/minecraft/storagedrawers/config/MaterialBlacklist.java:26)

### `ModClientConfig`: it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)

- [ ] `ModClientConfig.INSTANCE`: it uses ModClientConfig, which isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)) (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:8)
- [ ] `ModClientConfig.ModClientConfig(0)`: its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)) (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:14)
- [ ] `ModClientConfig.context(0)`: its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)) (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:17)
- [ ] `ModClientConfig.init(0)`: its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)) (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:21)

### `ModClientConfig.ConfigSection`

- [x] `ModClientConfig.ConfigSection.ConfigSection(2)` (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:32)
- [ ] `ModClientConfig.ConfigSection.build(0)`: unknown static ModClientConfig.this (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:37)
- [x] `ModClientConfig.ConfigSection.buildEntries(0)` (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:48)

### `ModClientConfig.General`: it uses ConfigEntry, which isn't translated (it's generic)

- [ ] `ModClientConfig.General.General(0)`: its class isn't translated (it uses ConfigEntry, which isn't translated (it's generic)) (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:56)

### `ModClientConfig.Integration`: it uses ConfigEntry, which isn't translated (it's generic)

- [ ] `ModClientConfig.Integration.Integration(0)`: its class isn't translated (it uses ConfigEntry, which isn't translated (it's generic)) (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:71)

### `ModClientConfig.Render`: it uses ConfigEntry, which isn't translated (it's generic)

- [ ] `ModClientConfig.Render.Render(0)`: its class isn't translated (it uses ConfigEntry, which isn't translated (it's generic)) (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:88)

### `ModClientConfig.Render.FramedDrawers`: it extends `ConfigSection`

- [ ] `ModClientConfig.Render.FramedDrawers.FramedDrawers(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:115)
- [ ] `ModClientConfig.Render.FramedDrawers.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:119)
- [ ] `ModClientConfig.Render.FramedDrawers.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java:125)

### `ModCommonConfig`: it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)

- [ ] `ModCommonConfig.INSTANCE`: it uses ModCommonConfig, which isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:12)
- [ ] `ModCommonConfig.ModCommonConfig(0)`: its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:21)
- [ ] `ModCommonConfig.context(0)`: its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:24)
- [ ] `ModCommonConfig.init(0)`: its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:28)

### `ModCommonConfig.ConfigSection`

- [x] `ModCommonConfig.ConfigSection.ConfigSection(2)` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:42)
- [ ] `ModCommonConfig.ConfigSection.build(0)`: unknown static ModCommonConfig.this (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:47)
- [x] `ModCommonConfig.ConfigSection.buildEntries(0)` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:58)

### `ModCommonConfig.Controller`: it uses ConfigEntry, which isn't translated (it's generic)

- [ ] `ModCommonConfig.Controller.Controller(0)`: its class isn't translated (it uses ConfigEntry, which isn't translated (it's generic)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:65)

### `ModCommonConfig.Drawers`: it uses DrawerConfig, which isn't translated (it extends `ConfigSection`)

- [ ] `ModCommonConfig.Drawers.Drawers(0)`: its class isn't translated (it uses DrawerConfig, which isn't translated (it extends `ConfigSection`)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:96)
- [ ] `ModCommonConfig.Drawers.anyHeavyDrawers(0)`: its class isn't translated (it uses DrawerConfig, which isn't translated (it extends `ConfigSection`)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:132)
- [ ] `ModCommonConfig.Drawers.getBaseStackStorage(0)`: its class isn't translated (it uses DrawerConfig, which isn't translated (it extends `ConfigSection`)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:136)

### `ModCommonConfig.Drawers.Compacting`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Drawers.Compacting.Compacting(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:155)
- [ ] `ModCommonConfig.Drawers.Compacting.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:159)
- [ ] `ModCommonConfig.Drawers.Compacting.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:166)

### `ModCommonConfig.Drawers.Detached`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Drawers.Detached.Detached(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:199)
- [ ] `ModCommonConfig.Drawers.Detached.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:203)
- [ ] `ModCommonConfig.Drawers.Detached.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:211)

### `ModCommonConfig.Drawers.DrawerConfig`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Drawers.DrawerConfig.DrawerConfig(3)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:220)
- [ ] `ModCommonConfig.Drawers.DrawerConfig.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:225)
- [ ] `ModCommonConfig.Drawers.DrawerConfig.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:230)
- [ ] `ModCommonConfig.Drawers.DrawerConfig.getUnitsPerSlot(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:235)

### `ModCommonConfig.Drawers.Filled`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Drawers.Filled.Filled(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:259)
- [ ] `ModCommonConfig.Drawers.Filled.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:263)
- [ ] `ModCommonConfig.Drawers.Filled.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:270)

### `ModCommonConfig.Drawers.Framed`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Drawers.Framed.Framed(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:303)
- [ ] `ModCommonConfig.Drawers.Framed.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:307)
- [ ] `ModCommonConfig.Drawers.Framed.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:315)

### `ModCommonConfig.Drawers.Storage`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Drawers.Storage.Storage(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:346)
- [ ] `ModCommonConfig.Drawers.Storage.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:350)
- [ ] `ModCommonConfig.Drawers.Storage.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:357)

### `ModCommonConfig.DropMode`

- [x] `ModCommonConfig.DropMode.fromValueIgnoreCase(1)` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:369)

### `ModCommonConfig.General`: it uses ConfigEntry, which isn't translated (it's generic)

- [ ] `ModCommonConfig.General.General(0)`: its class isn't translated (it uses ConfigEntry, which isn't translated (it's generic)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:386)

### `ModCommonConfig.Integration`: it uses Waila, which isn't translated (it extends `ConfigSection`)

- [ ] `ModCommonConfig.Integration.Integration(0)`: its class isn't translated (it uses Waila, which isn't translated (it extends `ConfigSection`)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:414)

### `ModCommonConfig.Integration.FTBChunks`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Integration.FTBChunks.FTBChunks(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:434)
- [ ] `ModCommonConfig.Integration.FTBChunks.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:438)
- [ ] `ModCommonConfig.Integration.FTBChunks.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:444)

### `ModCommonConfig.Integration.FTBTeams`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Integration.FTBTeams.FTBTeams(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:458)
- [ ] `ModCommonConfig.Integration.FTBTeams.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:462)
- [ ] `ModCommonConfig.Integration.FTBTeams.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:469)

### `ModCommonConfig.Integration.Waila`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Integration.Waila.Waila(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:486)
- [ ] `ModCommonConfig.Integration.Waila.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:490)
- [ ] `ModCommonConfig.Integration.Waila.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:498)

### `ModCommonConfig.Mode`

- [x] `ModCommonConfig.Mode.fromValueIgnoreCase(1)` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:510)

### `ModCommonConfig.Tools`: it uses Key, which isn't translated (it extends `ConfigSection`)

- [ ] `ModCommonConfig.Tools.Tools(0)`: its class isn't translated (it uses Key, which isn't translated (it extends `ConfigSection`)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:529)

### `ModCommonConfig.Tools.Key`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Tools.Key.Key(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:558)
- [ ] `ModCommonConfig.Tools.Key.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:562)
- [ ] `ModCommonConfig.Tools.Key.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:567)

### `ModCommonConfig.Tools.QuantifyKey`: it extends `Key`

- [ ] `ModCommonConfig.Tools.QuantifyKey.QuantifyKey(2)`: its class isn't translated (it extends `Key`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:578)
- [ ] `ModCommonConfig.Tools.QuantifyKey.buildEntries(0)`: its class isn't translated (it extends `Key`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:582)
- [ ] `ModCommonConfig.Tools.QuantifyKey.build(0)`: its class isn't translated (it extends `Key`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:588)

### `ModCommonConfig.Upgrades`: it uses StorageTierUpgrade, which isn't translated (it extends `Upgrade`)

- [ ] `ModCommonConfig.Upgrades.Upgrades(0)`: its class isn't translated (it uses StorageTierUpgrade, which isn't translated (it extends `Upgrade`)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:617)
- [ ] `ModCommonConfig.Upgrades.getStorageUpgrade(1)`: its class isn't translated (it uses StorageTierUpgrade, which isn't translated (it extends `Upgrade`)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:673)
- [ ] `ModCommonConfig.Upgrades.getLevelMult(1)`: its class isn't translated (it uses StorageTierUpgrade, which isn't translated (it extends `Upgrade`)) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:686)

### `ModCommonConfig.Upgrades.ConversionUpgrade`: it extends `Upgrade`

- [ ] `ModCommonConfig.Upgrades.ConversionUpgrade.ConversionUpgrade(2)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:744)
- [ ] `ModCommonConfig.Upgrades.ConversionUpgrade.buildEntries(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:748)
- [ ] `ModCommonConfig.Upgrades.ConversionUpgrade.build(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:758)

### `ModCommonConfig.Upgrades.IlluminationUpgrade`: it extends `Upgrade`

- [ ] `ModCommonConfig.Upgrades.IlluminationUpgrade.IlluminationUpgrade(2)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:772)
- [ ] `ModCommonConfig.Upgrades.IlluminationUpgrade.buildEntries(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:776)
- [ ] `ModCommonConfig.Upgrades.IlluminationUpgrade.build(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:783)

### `ModCommonConfig.Upgrades.MagnetTierUpgrade`: it extends `Upgrade`

- [ ] `ModCommonConfig.Upgrades.MagnetTierUpgrade.MagnetTierUpgrade(4)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:794)
- [ ] `ModCommonConfig.Upgrades.MagnetTierUpgrade.buildEntries(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:811)
- [ ] `ModCommonConfig.Upgrades.MagnetTierUpgrade.build(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:819)

### `ModCommonConfig.Upgrades.MagnetUpgrade`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Upgrades.MagnetUpgrade.MagnetUpgrade(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:840)
- [ ] `ModCommonConfig.Upgrades.MagnetUpgrade.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:844)
- [ ] `ModCommonConfig.Upgrades.MagnetUpgrade.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:854)

### `ModCommonConfig.Upgrades.RedstoneUpgrade`: it extends `Upgrade`

- [ ] `ModCommonConfig.Upgrades.RedstoneUpgrade.RedstoneUpgrade(2)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:871)
- [ ] `ModCommonConfig.Upgrades.RedstoneUpgrade.buildEntries(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:875)
- [ ] `ModCommonConfig.Upgrades.RedstoneUpgrade.build(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:883)

### `ModCommonConfig.Upgrades.RemoteUpgrade`: it extends `Upgrade`

- [ ] `ModCommonConfig.Upgrades.RemoteUpgrade.RemoteUpgrade(2)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:908)
- [ ] `ModCommonConfig.Upgrades.RemoteUpgrade.buildEntries(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:912)
- [ ] `ModCommonConfig.Upgrades.RemoteUpgrade.build(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:920)

### `ModCommonConfig.Upgrades.StorageTierUpgrade`: it extends `Upgrade`

- [ ] `ModCommonConfig.Upgrades.StorageTierUpgrade.StorageTierUpgrade(3)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:929)
- [ ] `ModCommonConfig.Upgrades.StorageTierUpgrade.buildEntries(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:934)
- [ ] `ModCommonConfig.Upgrades.StorageTierUpgrade.build(0)`: its class isn't translated (it extends `Upgrade`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:940)

### `ModCommonConfig.Upgrades.Upgrade`: it extends `ConfigSection`

- [ ] `ModCommonConfig.Upgrades.Upgrade.Upgrade(2)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:949)
- [ ] `ModCommonConfig.Upgrades.Upgrade.buildEntries(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:953)
- [ ] `ModCommonConfig.Upgrades.Upgrade.build(0)`: its class isn't translated (it extends `ConfigSection`) (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java:958)

### `PlayerConfig`

- [ ] `PlayerConfig.serverPlayerConfigSettings`: it uses `UUID` (java.util) (com/jaquadro/minecraft/storagedrawers/config/PlayerConfig.java:11)
- [ ] `PlayerConfig.getBooleanValue(2)`: it uses PlayerConfig.serverPlayerConfigSettings, which isn't translated (it uses `UUID` (java.util)) (com/jaquadro/minecraft/storagedrawers/config/PlayerConfig.java:13)
- [ ] `PlayerConfig.getInvertShift(1)`: it calls PlayerConfig.getBooleanValue, which isn't translated (com/jaquadro/minecraft/storagedrawers/config/PlayerConfig.java:22)
- [ ] `PlayerConfig.getInvertClick(1)`: it calls PlayerConfig.getBooleanValue, which isn't translated (com/jaquadro/minecraft/storagedrawers/config/PlayerConfig.java:31)

### `PlayerConfigSetting`: it's generic

- [ ] `PlayerConfigSetting.PlayerConfigSetting(3)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/config/PlayerConfigSetting.java:10)
- [ ] `PlayerConfigSetting.getValue(0)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/config/PlayerConfigSetting.java:16)

### `StorageBlacklist`: it extends `ConfigItemList`

- [ ] `StorageBlacklist.INSTANCE`: it uses StorageBlacklist, which isn't translated (it extends `ConfigItemList`) (com/jaquadro/minecraft/storagedrawers/config/StorageBlacklist.java:14)
- [ ] `StorageBlacklist.innerInitialize(0)`: its class isn't translated (it extends `ConfigItemList`) (com/jaquadro/minecraft/storagedrawers/config/StorageBlacklist.java:16)
- [ ] `StorageBlacklist.isBlacklisted(1)`: its class isn't translated (it extends `ConfigItemList`) (com/jaquadro/minecraft/storagedrawers/config/StorageBlacklist.java:21)
- [ ] `StorageBlacklist.logRegisterNamespace(1)`: its class isn't translated (it extends `ConfigItemList`) (com/jaquadro/minecraft/storagedrawers/config/StorageBlacklist.java:39)
- [ ] `StorageBlacklist.logRegisterItem(1)`: its class isn't translated (it extends `ConfigItemList`) (com/jaquadro/minecraft/storagedrawers/config/StorageBlacklist.java:46)

### `Api`: it implements `IStorageDrawersApi` (com.jaquadro.minecraft.storagedrawers.api)

- [ ] `Api.instance`: it uses IStorageDrawersApi, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/Api.java:6)
- [ ] `Api.Api(0)`: its class isn't translated (it implements `IStorageDrawersApi` (com.jaquadro.minecraft.storagedrawers.api)) (com/jaquadro/minecraft/storagedrawers/core/Api.java:8)

### `CommonEvents`

- [ ] `CommonEvents.init(0)`: it uses Fabric's `AttackBlockCallback`.EVENT (com/jaquadro/minecraft/storagedrawers/core/CommonEvents.java:16)

### `ModBlockEntities`

- [ ] `ModBlockEntities.BLOCK_ENTITIES`: it uses ChameleonRegistry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:29)
- [ ] `ModBlockEntities.DRAWER_TYPES`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:31)
- [ ] `ModBlockEntities.FRAMING_TABLE_TYPES`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:32)
- [ ] `ModBlockEntities.STANDARD_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:33)
- [ ] `ModBlockEntities.STANDARD_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:36)
- [ ] `ModBlockEntities.STANDARD_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:39)
- [ ] `ModBlockEntities.FRACTIONAL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:42)
- [ ] `ModBlockEntities.FRACTIONAL_DRAWERS_3`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:45)
- [ ] `ModBlockEntities.CONTROLLER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:48)
- [ ] `ModBlockEntities.CONTROLLER_IO`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:51)
- [ ] `ModBlockEntities.TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:54)
- [ ] `ModBlockEntities.FRAMING_TABLE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:57)
- [x] `ModBlockEntities.ModBlockEntities(0)` (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:61)
- [ ] `ModBlockEntities.registerDrawerBlockEntityType(4)`: it's generic (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:64)
- [ ] `ModBlockEntities.registerBlockEntityType(4)`: it's generic (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:72)
- [ ] `ModBlockEntities.registerControllerBlockEntityType(2)`: it's generic (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:80)
- [ ] `ModBlockEntities.registerControllerIOBlockEntityType(2)`: it's generic (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:86)
- [ ] `ModBlockEntities.init(1)`: it uses ModBlockEntities.BLOCK_ENTITIES, which isn't translated (it uses ChameleonRegistry, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:92)
- [ ] `ModBlockEntities.getDrawerTypes(0)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java:96)

### `ModBlocks`

- [ ] `ModBlocks.BLOCKS`: it uses ChameleonRegistry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:46)
- [x] `ModBlocks.EXCLUDE_ITEMS` (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:47)
- [ ] `ModBlocks.OAK_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:48)
- [ ] `ModBlocks.OAK_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:49)
- [ ] `ModBlocks.OAK_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:50)
- [ ] `ModBlocks.OAK_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:51)
- [ ] `ModBlocks.OAK_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:52)
- [ ] `ModBlocks.OAK_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:53)
- [ ] `ModBlocks.SPRUCE_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:54)
- [ ] `ModBlocks.SPRUCE_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:55)
- [ ] `ModBlocks.SPRUCE_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:56)
- [ ] `ModBlocks.SPRUCE_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:57)
- [ ] `ModBlocks.SPRUCE_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:58)
- [ ] `ModBlocks.SPRUCE_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:59)
- [ ] `ModBlocks.BIRCH_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:60)
- [ ] `ModBlocks.BIRCH_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:61)
- [ ] `ModBlocks.BIRCH_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:62)
- [ ] `ModBlocks.BIRCH_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:63)
- [ ] `ModBlocks.BIRCH_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:64)
- [ ] `ModBlocks.BIRCH_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:65)
- [ ] `ModBlocks.JUNGLE_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:66)
- [ ] `ModBlocks.JUNGLE_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:67)
- [ ] `ModBlocks.JUNGLE_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:68)
- [ ] `ModBlocks.JUNGLE_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:69)
- [ ] `ModBlocks.JUNGLE_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:70)
- [ ] `ModBlocks.JUNGLE_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:71)
- [ ] `ModBlocks.ACACIA_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:72)
- [ ] `ModBlocks.ACACIA_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:73)
- [ ] `ModBlocks.ACACIA_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:74)
- [ ] `ModBlocks.ACACIA_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:75)
- [ ] `ModBlocks.ACACIA_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:76)
- [ ] `ModBlocks.ACACIA_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:77)
- [ ] `ModBlocks.DARK_OAK_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:78)
- [ ] `ModBlocks.DARK_OAK_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:79)
- [ ] `ModBlocks.DARK_OAK_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:80)
- [ ] `ModBlocks.DARK_OAK_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:81)
- [ ] `ModBlocks.DARK_OAK_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:82)
- [ ] `ModBlocks.DARK_OAK_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:83)
- [ ] `ModBlocks.MANGROVE_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:84)
- [ ] `ModBlocks.MANGROVE_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:85)
- [ ] `ModBlocks.MANGROVE_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:86)
- [ ] `ModBlocks.MANGROVE_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:87)
- [ ] `ModBlocks.MANGROVE_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:88)
- [ ] `ModBlocks.MANGROVE_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:89)
- [ ] `ModBlocks.CHERRY_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:90)
- [ ] `ModBlocks.CHERRY_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:91)
- [ ] `ModBlocks.CHERRY_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:92)
- [ ] `ModBlocks.CHERRY_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:93)
- [ ] `ModBlocks.CHERRY_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:94)
- [ ] `ModBlocks.CHERRY_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:95)
- [ ] `ModBlocks.BAMBOO_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:96)
- [ ] `ModBlocks.BAMBOO_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:97)
- [ ] `ModBlocks.BAMBOO_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:98)
- [ ] `ModBlocks.BAMBOO_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:99)
- [ ] `ModBlocks.BAMBOO_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:100)
- [ ] `ModBlocks.BAMBOO_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:101)
- [ ] `ModBlocks.CRIMSON_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:102)
- [ ] `ModBlocks.CRIMSON_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:103)
- [ ] `ModBlocks.CRIMSON_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:104)
- [ ] `ModBlocks.CRIMSON_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:105)
- [ ] `ModBlocks.CRIMSON_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:106)
- [ ] `ModBlocks.CRIMSON_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:107)
- [ ] `ModBlocks.WARPED_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:108)
- [ ] `ModBlocks.WARPED_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:109)
- [ ] `ModBlocks.WARPED_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:110)
- [ ] `ModBlocks.WARPED_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:111)
- [ ] `ModBlocks.WARPED_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:112)
- [ ] `ModBlocks.WARPED_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:113)
- [ ] `ModBlocks.PALE_OAK_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:114)
- [ ] `ModBlocks.PALE_OAK_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:115)
- [ ] `ModBlocks.PALE_OAK_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:116)
- [ ] `ModBlocks.PALE_OAK_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:117)
- [ ] `ModBlocks.PALE_OAK_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:118)
- [ ] `ModBlocks.PALE_OAK_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:119)
- [ ] `ModBlocks.FRAMED_FULL_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:120)
- [ ] `ModBlocks.FRAMED_FULL_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:121)
- [ ] `ModBlocks.FRAMED_FULL_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:122)
- [ ] `ModBlocks.FRAMED_HALF_DRAWERS_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:123)
- [ ] `ModBlocks.FRAMED_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:124)
- [ ] `ModBlocks.FRAMED_HALF_DRAWERS_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:125)
- [ ] `ModBlocks.COMPACTING_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:126)
- [ ] `ModBlocks.COMPACTING_DRAWERS_3`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:127)
- [ ] `ModBlocks.COMPACTING_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:128)
- [ ] `ModBlocks.COMPACTING_HALF_DRAWERS_3`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:129)
- [ ] `ModBlocks.FRAMED_COMPACTING_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:130)
- [ ] `ModBlocks.FRAMED_COMPACTING_DRAWERS_3`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:133)
- [ ] `ModBlocks.FRAMED_COMPACTING_HALF_DRAWERS_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:136)
- [ ] `ModBlocks.FRAMED_COMPACTING_HALF_DRAWERS_3`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:139)
- [ ] `ModBlocks.OAK_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:142)
- [ ] `ModBlocks.SPRUCE_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:143)
- [ ] `ModBlocks.BIRCH_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:144)
- [ ] `ModBlocks.JUNGLE_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:145)
- [ ] `ModBlocks.ACACIA_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:146)
- [ ] `ModBlocks.DARK_OAK_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:147)
- [ ] `ModBlocks.MANGROVE_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:148)
- [ ] `ModBlocks.CHERRY_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:149)
- [ ] `ModBlocks.BAMBOO_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:150)
- [ ] `ModBlocks.CRIMSON_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:151)
- [ ] `ModBlocks.WARPED_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:152)
- [ ] `ModBlocks.PALE_OAK_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:153)
- [ ] `ModBlocks.FRAMED_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:154)
- [ ] `ModBlocks.CONTROLLER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:155)
- [ ] `ModBlocks.CONTROLLER_IO`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:156)
- [ ] `ModBlocks.FRAMED_CONTROLLER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:157)
- [ ] `ModBlocks.FRAMED_CONTROLLER_IO`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:160)
- [ ] `ModBlocks.FRAMING_TABLE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:163)
- [ ] `ModBlocks.META_LOCKED`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:164)
- [ ] `ModBlocks.META_CLAIMED`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:165)
- [ ] `ModBlocks.META_LOCKED_CLAIMED`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:166)
- [ ] `ModBlocks.META_VOID_ICON`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:167)
- [ ] `ModBlocks.META_SHROUD_ICON`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:168)
- [ ] `ModBlocks.META_SUSPEND_ICON`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:169)
- [ ] `ModBlocks.META_MAGNET_ICON`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:170)
- [ ] `ModBlocks.META_PRIORITY_P1_ICON`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:171)
- [ ] `ModBlocks.META_PRIORITY_P2_ICON`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:172)
- [ ] `ModBlocks.META_PRIORITY_N1_ICON`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:173)
- [ ] `ModBlocks.META_PRIORITY_N2_ICON`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:174)
- [ ] `ModBlocks.META_INDICATOR`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:175)
- [ ] `ModBlocks.META_COMP_INDICATOR`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:176)
- [ ] `ModBlocks.META_RIGHT_LABEL`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:177)
- [ ] `ModBlocks.META_HOPPER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:178)
- [ ] `ModBlocks.META_MISSING_1_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:179)
- [ ] `ModBlocks.META_MISSING_2_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:180)
- [ ] `ModBlocks.META_MISSING_2_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:181)
- [ ] `ModBlocks.META_MISSING_4_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:182)
- [ ] `ModBlocks.META_MISSING_4_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:183)
- [ ] `ModBlocks.META_MISSING_4_3`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:184)
- [ ] `ModBlocks.META_MISSING_4_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:185)
- [ ] `ModBlocks.META_FRAMED_DRAWERS_SIDE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:186)
- [ ] `ModBlocks.META_FRAMED_DRAWERS_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:187)
- [ ] `ModBlocks.META_FRAMED_DRAWERS_FRONT`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:188)
- [ ] `ModBlocks.META_FRAMED_DRAWERS_SHADING`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:189)
- [ ] `ModBlocks.META_FRAMED_TRIM_SIDE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:190)
- [ ] `ModBlocks.META_FRAMED_TRIM_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:191)
- [ ] `ModBlocks.META_FRAMED_CONTROLLER_SIDE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:192)
- [ ] `ModBlocks.META_FRAMED_CONTROLLER_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:193)
- [ ] `ModBlocks.META_FRAMED_CONTROLLER_FRONT`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:194)
- [ ] `ModBlocks.META_FRAMED_CONTROLLER_SHADING`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:195)
- [ ] `ModBlocks.META_FRAMED_CONTROLLER_IO_SIDE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:196)
- [ ] `ModBlocks.META_FRAMED_CONTROLLER_IO_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:197)
- [ ] `ModBlocks.META_FRAMED_CONTROLLER_IO_SHADING`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:198)
- [ ] `ModBlocks.META_FRAMED_COMPDRAWERS_2_SIDE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:199)
- [ ] `ModBlocks.META_FRAMED_COMPDRAWERS_2_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:200)
- [ ] `ModBlocks.META_FRAMED_COMPDRAWERS_2_FRONT`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:201)
- [ ] `ModBlocks.META_FRAMED_COMPDRAWERS_2_SHADING`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:202)
- [ ] `ModBlocks.META_FRAMED_COMPDRAWERS_3_SIDE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:203)
- [ ] `ModBlocks.META_FRAMED_COMPDRAWERS_3_TRIM`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:204)
- [ ] `ModBlocks.META_FRAMED_COMPDRAWERS_3_FRONT`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:205)
- [ ] `ModBlocks.META_FRAMED_COMPDRAWERS_3_SHADING`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:206)
- [ ] `ModBlocks.KEYBUTTON_DRAWER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:207)
- [ ] `ModBlocks.KEYBUTTON_QUANTIFY`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:210)
- [ ] `ModBlocks.KEYBUTTON_CONCEALMENT`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:213)
- [x] `ModBlocks.ModBlocks(0)` (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:217)
- [x] `ModBlocks.modLoc(1)` (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:220)
- [ ] `ModBlocks.modKey(1)`: it uses Minecraft's `ResourceKey` (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:224)
- [ ] `ModBlocks.getStandardConfig(2)`: it uses IDrawerConfig, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:228)
- [ ] `ModBlocks.getCompConfig(1)`: it uses IDrawerConfig, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:241)
- [ ] `ModBlocks.registerWoodenDrawerBlock(3)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:246)
- [ ] `ModBlocks.registerWoodenDrawerBlock(4)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:250)
- [ ] `ModBlocks.registerWoodenDrawerBlock(4)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:255)
- [ ] `ModBlocks.registerFramedDrawerBlock(3)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:265)
- [ ] `ModBlocks.registerCompactingDrawerBlock(3)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:276)
- [ ] `ModBlocks.registerFramedCompactingDrawerBlock(3)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:281)
- [ ] `ModBlocks.registerTrimBlock(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:288)
- [ ] `ModBlocks.registerTrimBlock(2)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:292)
- [ ] `ModBlocks.registerTrimBlock(2)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:296)
- [ ] `ModBlocks.registerFramedTrimBlock(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:301)
- [ ] `ModBlocks.registerControllerBlock(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:308)
- [ ] `ModBlocks.registerControllerIOBlock(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:312)
- [ ] `ModBlocks.registerFramingTableBlock(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:316)
- [ ] `ModBlocks.registerMetaBlock(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:320)
- [ ] `ModBlocks.registerMetaTransBlock(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:325)
- [ ] `ModBlocks.registerMetaFacingBlock(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:330)
- [ ] `ModBlocks.registerMetaFacingSizedBlock(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:335)
- [ ] `ModBlocks.registerMetaFacingSizedSlotted23Block(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:340)
- [ ] `ModBlocks.registerMetaFacingSizedSlotted124Block(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:345)
- [ ] `ModBlocks.registerMetaFacingSizedLabelBlock(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:350)
- [ ] `ModBlocks.registerMetaFacingSizedOpen2Block(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:355)
- [ ] `ModBlocks.registerMetaFacingSizedOpen3Block(1)`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:360)
- [ ] `ModBlocks.getWoodenBlockProperties(0)`: it calls Minecraft's `Properties`.of (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:365)
- [ ] `ModBlocks.getWoodenDrawerBlockProperties(0)`: it calls ModBlocks.getWoodenBlockProperties, which isn't translated (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:369)
- [ ] `ModBlocks.getStoneBlockProperties(0)`: it calls Minecraft's `Properties`.of (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:373)
- [ ] `ModBlocks.getStoneDrawerBlockProperties(0)`: it calls ModBlocks.getStoneBlockProperties, which isn't translated (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:377)
- [ ] `ModBlocks.init(1)`: it uses ModBlocks.BLOCKS, which isn't translated (it uses ChameleonRegistry, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:381)
- [ ] `ModBlocks.getBlocksOfType(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:385)
- [ ] `ModBlocks.getDrawers(0)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:389)
- [ ] `ModBlocks.getFramedDrawers(0)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:393)
- [ ] `ModBlocks.getControllers(0)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:397)
- [ ] `ModBlocks.getControllerIOs(0)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:401)
- [ ] `ModBlocks.getDrawersOfType(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:405)
- [ ] `ModBlocks.getDrawersOfTypeAndSize(2)`: it's generic (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:409)
- [ ] `ModBlocks.getDrawersOfTypeAndSizeAndDepth(3)`: it's generic (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:413)
- [ ] `ModBlocks.getDrawersofTypeAndDepth(2)`: it's generic (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:417)
- [ ] `ModBlocks.getFramedTrim(0)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:421)
- [ ] `ModBlocks.getFramedBlocks(0)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:425)
- [ ] `ModBlocks.predFalse(3)`: it uses Minecraft's `BlockState` (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java:429)

### `ModBlockVariants`

- [ ] `ModBlockVariants.registerVariant(2)`: it uses ChameleonRegistry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockVariants.java:12)
- [ ] `ModBlockVariants.registerVariantItem(2)`: it uses ChameleonRegistry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModBlockVariants.java:22)

### `ModBlockVariants.VariantData`: it uses RegistryEntry, which isn't translated (it's an interface)

- [ ] `ModBlockVariants.VariantData.VariantData(1)`: its class isn't translated (it uses RegistryEntry, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/core/ModBlockVariants.java:42)

### `ModContainers`

- [ ] `ModContainers.CONTAINERS`: it uses ChameleonRegistry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModContainers.java:17)
- [ ] `ModContainers.DRAWER_CONTAINER_1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModContainers.java:18)
- [ ] `ModContainers.DRAWER_CONTAINER_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModContainers.java:21)
- [ ] `ModContainers.DRAWER_CONTAINER_4`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModContainers.java:24)
- [ ] `ModContainers.DRAWER_CONTAINER_COMP_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModContainers.java:27)
- [ ] `ModContainers.DRAWER_CONTAINER_COMP_3`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModContainers.java:30)
- [ ] `ModContainers.FRAMING_TABLE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModContainers.java:33)
- [ ] `ModContainers.init(1)`: it uses ModContainers.CONTAINERS, which isn't translated (it uses ChameleonRegistry, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/core/ModContainers.java:37)

### `ModCreativeTabs`

- [ ] `ModCreativeTabs.MAIN`: it uses Minecraft's `ResourceKey` (com/jaquadro/minecraft/storagedrawers/core/ModCreativeTabs.java:20)
- [ ] `ModCreativeTabs.MAIN_TAB`: it uses Minecraft's `CreativeModeTab` (com/jaquadro/minecraft/storagedrawers/core/ModCreativeTabs.java:23)
- [ ] `ModCreativeTabs.init(1)`: it uses Minecraft's `BuiltInRegistries`.CREATIVE_MODE_TAB (com/jaquadro/minecraft/storagedrawers/core/ModCreativeTabs.java:28)

### `ModDataComponents`

- [ ] `ModDataComponents.COMPONENTS`: it uses ChameleonRegistry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModDataComponents.java:16)
- [ ] `ModDataComponents.CONTROLLER_BINDING`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModDataComponents.java:18)
- [ ] `ModDataComponents.DRAWER_COUNT`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModDataComponents.java:21)
- [ ] `ModDataComponents.KEYRING_CONTENTS`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModDataComponents.java:24)
- [ ] `ModDataComponents.FRAME_DATA`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModDataComponents.java:27)
- [ ] `ModDataComponents.DETACHED_DRAWER_CONTENTS`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModDataComponents.java:30)
- [ ] `ModDataComponents.init(1)`: it uses ModDataComponents.COMPONENTS, which isn't translated (it uses ChameleonRegistry, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/core/ModDataComponents.java:34)

### `ModItems`

- [ ] `ModItems.ITEMS`: it uses ChameleonRegistry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:57)
- [ ] `ModItems.EXCLUDE_ITEMS_CREATIVE_TAB`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:58)
- [ ] `ModItems.OBSIDIAN_STORAGE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:59)
- [ ] `ModItems.COPPER_STORAGE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:62)
- [ ] `ModItems.IRON_STORAGE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:65)
- [ ] `ModItems.GOLD_STORAGE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:68)
- [ ] `ModItems.EMERALD_STORAGE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:71)
- [ ] `ModItems.DIAMOND_STORAGE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:74)
- [ ] `ModItems.NETHERITE_STORAGE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:77)
- [ ] `ModItems.ONE_STACK_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:80)
- [ ] `ModItems.VOID_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:81)
- [ ] `ModItems.CREATIVE_STORAGE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:82)
- [ ] `ModItems.CREATIVE_VENDING_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:85)
- [ ] `ModItems.CONVERSION_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:88)
- [ ] `ModItems.REDSTONE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:89)
- [ ] `ModItems.MIN_REDSTONE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:92)
- [ ] `ModItems.MAX_REDSTONE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:95)
- [ ] `ModItems.ILLUMINATION_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:98)
- [ ] `ModItems.FILL_LEVEL_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:101)
- [ ] `ModItems.BALANCE_FILL_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:102)
- [ ] `ModItems.PORTABILITY_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:103)
- [ ] `ModItems.HOPPER_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:104)
- [ ] `ModItems.MAGNET_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:105)
- [ ] `ModItems.MAGNET_UPGRADE_2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:108)
- [ ] `ModItems.MAGNET_UPGRADE_3`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:111)
- [ ] `ModItems.REMOTE_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:114)
- [ ] `ModItems.REMOTE_UPGRADE_BOUND`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:115)
- [ ] `ModItems.REMOTE_GROUP_UPGRADE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:118)
- [ ] `ModItems.REMOTE_GROUP_UPGRADE_BOUND`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:121)
- [ ] `ModItems.UPGRADE_TEMPLATE`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:124)
- [ ] `ModItems.DETACHED_DRAWER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:125)
- [ ] `ModItems.DETACHED_DRAWER_FULL`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:126)
- [ ] `ModItems.DRAWER_KEY`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:129)
- [ ] `ModItems.QUANTIFY_KEY`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:130)
- [ ] `ModItems.SHROUD_KEY`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:131)
- [ ] `ModItems.PERSONAL_KEY`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:132)
- [ ] `ModItems.PERSONAL_KEY_COFH`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:133)
- [ ] `ModItems.PERSONAL_KEY_FTB`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:134)
- [ ] `ModItems.PERSONAL_KEY_UNLOCK`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:135)
- [ ] `ModItems.SUSPEND_KEY`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:138)
- [ ] `ModItems.PRIORITY_KEY`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:139)
- [ ] `ModItems.PRIORITY_KEY_P1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:140)
- [ ] `ModItems.PRIORITY_KEY_P2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:141)
- [ ] `ModItems.PRIORITY_KEY_N1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:142)
- [ ] `ModItems.PRIORITY_KEY_N2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:143)
- [ ] `ModItems.DRAWER_PULLER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:144)
- [ ] `ModItems.KEYRING`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:145)
- [ ] `ModItems.KEYRING_DRAWER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:146)
- [ ] `ModItems.KEYRING_QUANTIFY`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:149)
- [ ] `ModItems.KEYRING_SHROUD`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:152)
- [ ] `ModItems.KEYRING_PERSONAL`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:155)
- [ ] `ModItems.KEYRING_PERSONAL_COFH`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:158)
- [ ] `ModItems.KEYRING_PERSONAL_FTB`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:161)
- [ ] `ModItems.KEYRING_PERSONAL_UNLOCK`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:164)
- [ ] `ModItems.KEYRING_SUSPEND`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:167)
- [ ] `ModItems.KEYRING_PRIORITY`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:170)
- [ ] `ModItems.KEYRING_PRIORITY_P1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:173)
- [ ] `ModItems.KEYRING_PRIORITY_P2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:176)
- [ ] `ModItems.KEYRING_PRIORITY_N1`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:179)
- [ ] `ModItems.KEYRING_PRIORITY_N2`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:182)
- [ ] `ModItems.KEYRING_PULLER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:185)
- [ ] `ModItems.register(3)`: it's generic (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:189)
- [x] `ModItems.ModItems(0)` (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:193)
- [ ] `ModItems.init(1)`: it uses ModItems.EXCLUDE_ITEMS_CREATIVE_TAB, which isn't translated (it uses RegistryEntry, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:196)
- [x] `ModItems.modLoc(1)` (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:228)
- [ ] `ModItems.modKey(1)`: it uses Minecraft's `ResourceKey` (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:232)
- [ ] `ModItems.modKey(1)`: it uses Minecraft's `ResourceKey` (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:236)
- [ ] `ModItems.registerBlock(2)`: it uses ChameleonRegistry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:240)
- [ ] `ModItems.getItemsOfType(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:260)
- [ ] `ModItems.getKeys(0)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:264)
- [ ] `ModItems.getKeyrings(0)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/core/ModItems.java:268)

### `ModNetworking`: it implements `ChameleonInit` (com.texelsaurus.minecraft.chameleon.api)

- [ ] `ModNetworking.INSTANCE`: it uses ModNetworking, which isn't translated (it implements `ChameleonInit` (com.texelsaurus.minecraft.chameleon.api)) (com/jaquadro/minecraft/storagedrawers/core/ModNetworking.java:9)
- [ ] `ModNetworking.init(1)`: its class isn't translated (it implements `ChameleonInit` (com.texelsaurus.minecraft.chameleon.api)) (com/jaquadro/minecraft/storagedrawers/core/ModNetworking.java:11)

### `ModRecipes`

- [ ] `ModRecipes.RECIPES`: it uses ChameleonRegistry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModRecipes.java:16)
- [ ] `ModRecipes.UPGRADE_RECIPE_SERIALIZER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModRecipes.java:18)
- [ ] `ModRecipes.KEYRING_RECIPE_SERIALIZER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModRecipes.java:21)
- [ ] `ModRecipes.REMOTE_GROUP_UPGRADE_SERIALIZER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModRecipes.java:24)
- [ ] `ModRecipes.DETACHED_UPGRADE_RECIPE_SERIALIZER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModRecipes.java:27)
- [ ] `ModRecipes.PERSONAL_KEY_RECIPE_SERIALIZER`: it uses RegistryEntry, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/core/ModRecipes.java:30)
- [ ] `ModRecipes.init(1)`: it uses ModRecipes.RECIPES, which isn't translated (it uses ChameleonRegistry, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/core/ModRecipes.java:34)

### `ModSecurity`

- [ ] `ModSecurity.registry`: it uses SecurityRegistry, which isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/core/ModSecurity.java:6)

### `PlayerEventListener`

- [ ] `PlayerEventListener.applyDebuff(1)`: it calls Minecraft's LocalPlayer.addEffect (no Horizon helper for it yet) (com/jaquadro/minecraft/storagedrawers/core/PlayerEventListener.java:13)
- [ ] `PlayerEventListener.onPlayerTick(1)`: it uses Minecraft's LocalPlayer.tickCount (no Horizon helper for it yet) (com/jaquadro/minecraft/storagedrawers/core/PlayerEventListener.java:17)
- [ ] `PlayerEventListener.checkItemDebuf(2)`: `instanceof` (com/jaquadro/minecraft/storagedrawers/core/PlayerEventListener.java:32)

### `AddUpgradeRecipe`: it extends Minecraft's `CustomRecipe`

- [ ] `AddUpgradeRecipe.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/jaquadro/minecraft/storagedrawers/core/recipe/AddUpgradeRecipe.java:31)
- [ ] `AddUpgradeRecipe.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/jaquadro/minecraft/storagedrawers/core/recipe/AddUpgradeRecipe.java:32)
- [ ] `AddUpgradeRecipe.matches(2)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/AddUpgradeRecipe.java:35)
- [ ] `AddUpgradeRecipe.assemble(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/AddUpgradeRecipe.java:40)
- [ ] `AddUpgradeRecipe.findContext(2)`: its body couldn't be read: an anonymous class (com/jaquadro/minecraft/storagedrawers/core/recipe/AddUpgradeRecipe.java:64)
- [ ] `AddUpgradeRecipe.getSerializer(0)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/AddUpgradeRecipe.java:129)

### `AddUpgradeRecipe.Context`: it uses Minecraft's `ItemStack`


### `KeyringRecipe`: it extends Minecraft's `CustomRecipe`

- [ ] `KeyringRecipe.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/jaquadro/minecraft/storagedrawers/core/recipe/KeyringRecipe.java:24)
- [ ] `KeyringRecipe.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/jaquadro/minecraft/storagedrawers/core/recipe/KeyringRecipe.java:25)
- [ ] `KeyringRecipe.PATTERN`: it uses Minecraft's `ShapedRecipePattern` (com/jaquadro/minecraft/storagedrawers/core/recipe/KeyringRecipe.java:26)
- [ ] `KeyringRecipe.matches(2)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/KeyringRecipe.java:30)
- [ ] `KeyringRecipe.assemble(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/KeyringRecipe.java:34)
- [ ] `KeyringRecipe.getSerializer(0)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/KeyringRecipe.java:56)

### `PersonalKeyRecipe`: it extends Minecraft's `CustomRecipe`

- [ ] `PersonalKeyRecipe.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/jaquadro/minecraft/storagedrawers/core/recipe/PersonalKeyRecipe.java:22)
- [ ] `PersonalKeyRecipe.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/jaquadro/minecraft/storagedrawers/core/recipe/PersonalKeyRecipe.java:23)
- [ ] `PersonalKeyRecipe.matches(2)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/PersonalKeyRecipe.java:27)
- [ ] `PersonalKeyRecipe.findPersonalKey(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/PersonalKeyRecipe.java:32)
- [ ] `PersonalKeyRecipe.checkPersonalKey(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/PersonalKeyRecipe.java:53)
- [ ] `PersonalKeyRecipe.ftbEnabled(0)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/PersonalKeyRecipe.java:62)
- [ ] `PersonalKeyRecipe.assemble(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/PersonalKeyRecipe.java:66)
- [ ] `PersonalKeyRecipe.getSerializer(0)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/PersonalKeyRecipe.java:86)

### `RemoteGroupUpgradeRecipe`: it extends Minecraft's `CustomRecipe`

- [ ] `RemoteGroupUpgradeRecipe.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/jaquadro/minecraft/storagedrawers/core/recipe/RemoteGroupUpgradeRecipe.java:22)
- [ ] `RemoteGroupUpgradeRecipe.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/jaquadro/minecraft/storagedrawers/core/recipe/RemoteGroupUpgradeRecipe.java:23)
- [ ] `RemoteGroupUpgradeRecipe.PATTERN`: it uses Minecraft's `ShapedRecipePattern` (com/jaquadro/minecraft/storagedrawers/core/recipe/RemoteGroupUpgradeRecipe.java:26)
- [ ] `RemoteGroupUpgradeRecipe.matches(2)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/RemoteGroupUpgradeRecipe.java:30)
- [ ] `RemoteGroupUpgradeRecipe.assemble(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/RemoteGroupUpgradeRecipe.java:34)
- [ ] `RemoteGroupUpgradeRecipe.getSerializer(0)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/RemoteGroupUpgradeRecipe.java:53)

### `UpgradeDetachedDrawerRecipe`: it extends Minecraft's `CustomRecipe`

- [ ] `UpgradeDetachedDrawerRecipe.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/jaquadro/minecraft/storagedrawers/core/recipe/UpgradeDetachedDrawerRecipe.java:32)
- [ ] `UpgradeDetachedDrawerRecipe.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/jaquadro/minecraft/storagedrawers/core/recipe/UpgradeDetachedDrawerRecipe.java:33)
- [ ] `UpgradeDetachedDrawerRecipe.matches(2)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/UpgradeDetachedDrawerRecipe.java:38)
- [ ] `UpgradeDetachedDrawerRecipe.assemble(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/UpgradeDetachedDrawerRecipe.java:43)
- [ ] `UpgradeDetachedDrawerRecipe.findContext(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/UpgradeDetachedDrawerRecipe.java:76)
- [ ] `UpgradeDetachedDrawerRecipe.getSerializer(0)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (com/jaquadro/minecraft/storagedrawers/core/recipe/UpgradeDetachedDrawerRecipe.java:112)

### `UpgradeDetachedDrawerRecipe.Context`: it uses Minecraft's `ItemStack`


### `DrawerOverlay`

- [ ] `DrawerOverlay.getOverlay(1)`: it uses Minecraft's `Component` (com/jaquadro/minecraft/storagedrawers/integration/DrawerOverlay.java:25)
- [ ] `DrawerOverlay.addContent(3)`: it uses Minecraft's `Component` (com/jaquadro/minecraft/storagedrawers/integration/DrawerOverlay.java:37)
- [ ] `DrawerOverlay.addStackLimit(3)`: it uses Minecraft's `Component` (com/jaquadro/minecraft/storagedrawers/integration/DrawerOverlay.java:79)
- [ ] `DrawerOverlay.addStatus(3)`: it uses Minecraft's `Component` (com/jaquadro/minecraft/storagedrawers/integration/DrawerOverlay.java:98)

### `IntegrationModule`

- [ ] `IntegrationModule.getModID(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/integration/IntegrationModule.java:4)
- [x] `IntegrationModule.versionCheck(0)` (com/jaquadro/minecraft/storagedrawers/integration/IntegrationModule.java:6)
- [x] `IntegrationModule.versionPattern(0)` (com/jaquadro/minecraft/storagedrawers/integration/IntegrationModule.java:11)
- [ ] `IntegrationModule.init(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/integration/IntegrationModule.java:15)
- [ ] `IntegrationModule.postInit(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/integration/IntegrationModule.java:17)

### `IntegrationRegistry`

- [x] `IntegrationRegistry.IntegrationRegistry(1)` (com/jaquadro/minecraft/storagedrawers/integration/IntegrationRegistry.java:12)
- [x] `IntegrationRegistry.add(1)` (com/jaquadro/minecraft/storagedrawers/integration/IntegrationRegistry.java:17)
- [ ] `IntegrationRegistry.init(0)`: it calls IntegrationModule.getModID, which isn't translated (com/jaquadro/minecraft/storagedrawers/integration/IntegrationRegistry.java:23)
- [ ] `IntegrationRegistry.postInit(0)`: it calls IntegrationModule.postInit, which isn't translated (com/jaquadro/minecraft/storagedrawers/integration/IntegrationRegistry.java:39)
- [ ] `IntegrationRegistry.isModLoaded(1)`: it calls IntegrationModule.getModID, which isn't translated (com/jaquadro/minecraft/storagedrawers/integration/IntegrationRegistry.java:45)

### `LocalIntegrationRegistry`

- [x] `LocalIntegrationRegistry.instance` (com/jaquadro/minecraft/storagedrawers/integration/LocalIntegrationRegistry.java:4)
- [ ] `LocalIntegrationRegistry.initialize(0)`: it calls LocalIntegrationRegistry.instance, which isn't translated (com/jaquadro/minecraft/storagedrawers/integration/LocalIntegrationRegistry.java:7)
- [x] `LocalIntegrationRegistry.LocalIntegrationRegistry(0)` (com/jaquadro/minecraft/storagedrawers/integration/LocalIntegrationRegistry.java:11)
- [ ] `LocalIntegrationRegistry.isModLoaded(1)`: a nullable object kept in a changing static (com/jaquadro/minecraft/storagedrawers/integration/LocalIntegrationRegistry.java:14)
- [ ] `LocalIntegrationRegistry.instance(0)`: a nullable object kept in a changing static (com/jaquadro/minecraft/storagedrawers/integration/LocalIntegrationRegistry.java:22)

### `Waila`: it implements `IWailaPlugin` (snownee.jade.api)

- [ ] `Waila.registerClient(1)`: its class isn't translated (it implements `IWailaPlugin` (snownee.jade.api)) (com/jaquadro/minecraft/storagedrawers/integration/Waila.java:23)

### `Waila.WailaDrawer`: it implements `IBlockComponentProvider` (snownee.jade.api)

- [ ] `Waila.WailaDrawer.getIcon(3)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/jaquadro/minecraft/storagedrawers/integration/Waila.java:34)
- [ ] `Waila.WailaDrawer.appendTooltip(3)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/jaquadro/minecraft/storagedrawers/integration/Waila.java:39)
- [ ] `Waila.WailaDrawer.getUid(0)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/jaquadro/minecraft/storagedrawers/integration/Waila.java:48)

### `ContainerDrawers`: it extends Minecraft's `AbstractContainerMenu`

- [x] `ContainerDrawers.InventoryX` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:28)
- [x] `ContainerDrawers.InventoryY` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:29)
- [x] `ContainerDrawers.HotbarY` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:30)
- [x] `ContainerDrawers.UpgradeX` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:31)
- [x] `ContainerDrawers.UpgradeY` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:32)
- [ ] `ContainerDrawers.ContainerDrawers(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:40)
- [ ] `ContainerDrawers.getBlockEntity(2)`: it uses BlockEntityDrawers, which isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:44)
- [ ] `ContainerDrawers.ContainerDrawers(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:55)
- [ ] `ContainerDrawers.getStorageSlotX(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:100)
- [ ] `ContainerDrawers.getStorageSlotY(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:104)
- [ ] `ContainerDrawers.getStorageSlots(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:108)
- [ ] `ContainerDrawers.getUpgradeSlots(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:112)
- [ ] `ContainerDrawers.getStackCapacity(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:116)
- [ ] `ContainerDrawers.stillValid(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:120)
- [ ] `ContainerDrawers.quickMoveStack(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java:124)

### `ContainerDrawers1`: it extends `ContainerDrawers`

- [x] `ContainerDrawers1.slotCoordinates` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers1.java:10)
- [ ] `ContainerDrawers1.ContainerDrawers1(3)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers1.java:12)
- [ ] `ContainerDrawers1.ContainerDrawers1(3)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers1.java:16)
- [ ] `ContainerDrawers1.getStorageSlotX(1)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers1.java:20)
- [ ] `ContainerDrawers1.getStorageSlotY(1)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers1.java:25)

### `ContainerDrawers2`: it extends `ContainerDrawers`

- [x] `ContainerDrawers2.slotCoordinates` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers2.java:10)
- [ ] `ContainerDrawers2.ContainerDrawers2(3)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers2.java:12)
- [ ] `ContainerDrawers2.ContainerDrawers2(3)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers2.java:16)
- [ ] `ContainerDrawers2.getStorageSlotX(1)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers2.java:20)
- [ ] `ContainerDrawers2.getStorageSlotY(1)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers2.java:25)

### `ContainerDrawers4`: it extends `ContainerDrawers`

- [x] `ContainerDrawers4.slotCoordinates` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers4.java:10)
- [ ] `ContainerDrawers4.ContainerDrawers4(3)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers4.java:12)
- [ ] `ContainerDrawers4.ContainerDrawers4(3)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers4.java:16)
- [ ] `ContainerDrawers4.getStorageSlotX(1)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers4.java:20)
- [ ] `ContainerDrawers4.getStorageSlotY(1)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers4.java:25)

### `ContainerDrawersComp2`: it extends `ContainerDrawers`

- [x] `ContainerDrawersComp2.slotCoordinates` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp2.java:10)
- [ ] `ContainerDrawersComp2.ContainerDrawersComp2(3)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp2.java:12)
- [ ] `ContainerDrawersComp2.ContainerDrawersComp2(3)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp2.java:16)
- [ ] `ContainerDrawersComp2.getStorageSlotX(1)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp2.java:20)
- [ ] `ContainerDrawersComp2.getStorageSlotY(1)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp2.java:25)

### `ContainerDrawersComp3`: it extends `ContainerDrawers`

- [x] `ContainerDrawersComp3.slotCoordinates` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp3.java:10)
- [ ] `ContainerDrawersComp3.ContainerDrawersComp3(3)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp3.java:12)
- [ ] `ContainerDrawersComp3.ContainerDrawersComp3(3)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp3.java:16)
- [ ] `ContainerDrawersComp3.getStorageSlotX(1)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp3.java:20)
- [ ] `ContainerDrawersComp3.getStorageSlotY(1)`: its class isn't translated (it extends `ContainerDrawers`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp3.java:25)

### `ContainerFramingTable`: it extends Minecraft's `AbstractContainerMenu`

- [x] `ContainerFramingTable.InventoryX` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:20)
- [x] `ContainerFramingTable.InventoryY` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:21)
- [x] `ContainerFramingTable.HotbarY` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:22)
- [x] `ContainerFramingTable.InputX` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:23)
- [x] `ContainerFramingTable.InputY` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:24)
- [x] `ContainerFramingTable.MaterialSideX` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:25)
- [x] `ContainerFramingTable.MaterialSideY` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:26)
- [x] `ContainerFramingTable.MaterialTrimX` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:27)
- [x] `ContainerFramingTable.MaterialTrimY` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:28)
- [x] `ContainerFramingTable.MaterialFrontX` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:29)
- [x] `ContainerFramingTable.MaterialFrontY` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:30)
- [x] `ContainerFramingTable.OutputX` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:31)
- [x] `ContainerFramingTable.OutputY` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:32)
- [ ] `ContainerFramingTable.ContainerFramingTable(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:46)
- [ ] `ContainerFramingTable.ContainerFramingTable(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:50)
- [ ] `ContainerFramingTable.quickMoveStack(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:79)
- [ ] `ContainerFramingTable.stillValid(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java:132)

### `CraftResultSlot`: it extends Minecraft's `Slot`

- [ ] `CraftResultSlot.CraftResultSlot(7)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/CraftResultSlot.java:18)
- [ ] `CraftResultSlot.mayPlace(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/CraftResultSlot.java:25)
- [ ] `CraftResultSlot.remove(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/CraftResultSlot.java:29)
- [ ] `CraftResultSlot.onTake(2)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/CraftResultSlot.java:38)
- [ ] `CraftResultSlot.onQuickCraft(2)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/CraftResultSlot.java:58)

### `DrawerInventoryHelper`

- [ ] `DrawerInventoryHelper.RANDOM`: it creates `Random` (java.util) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerInventoryHelper.java:15)
- [ ] `DrawerInventoryHelper.dropUpgradeItems(3)`: it uses Minecraft's `Level` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerInventoryHelper.java:17)
- [ ] `DrawerInventoryHelper.dropInventoryItems(3)`: it uses Minecraft's `Level` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerInventoryHelper.java:28)
- [ ] `DrawerInventoryHelper.spawnItemStack(5)`: it uses Minecraft's `Level` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerInventoryHelper.java:56)

### `DrawerScreen`: it extends Minecraft's `AbstractContainerScreen`

- [x] `DrawerScreen.guiTextures1` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:19)
- [x] `DrawerScreen.guiTextures2` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:20)
- [x] `DrawerScreen.guiTextures4` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:21)
- [x] `DrawerScreen.guiTexturesComp2` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:22)
- [x] `DrawerScreen.guiTexturesComp3` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:23)
- [x] `DrawerScreen.smDisabledX` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:24)
- [x] `DrawerScreen.smDisabledY` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:25)
- [x] `DrawerScreen.smMissingY` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:26)
- [ ] `DrawerScreen.DrawerScreen(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:30)
- [ ] `DrawerScreen.init(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:36)
- [ ] `DrawerScreen.extractLabels(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:40)
- [ ] `DrawerScreen.extractBackground(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:48)
- [ ] `DrawerScreen.extractSlot(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:74)
- [ ] `DrawerScreen.renderDrawerBar(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:90)
- [ ] `DrawerScreen.renderDrawerCooldown(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:99)
- [ ] `DrawerScreen.renderDrawerCount(5)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:109)
- [ ] `DrawerScreen.isHovering(6)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:139)

### `DrawerScreen.Compacting2`: it extends `DrawerScreen`

- [ ] `DrawerScreen.Compacting2.Compacting2(3)`: its class isn't translated (it extends `DrawerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:150)

### `DrawerScreen.Compacting3`: it extends `DrawerScreen`

- [ ] `DrawerScreen.Compacting3.Compacting3(3)`: its class isn't translated (it extends `DrawerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:156)

### `DrawerScreen.Slot1`: it extends `DrawerScreen`

- [ ] `DrawerScreen.Slot1.Slot1(3)`: its class isn't translated (it extends `DrawerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:162)

### `DrawerScreen.Slot2`: it extends `DrawerScreen`

- [ ] `DrawerScreen.Slot2.Slot2(3)`: its class isn't translated (it extends `DrawerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:168)

### `DrawerScreen.Slot4`: it extends `DrawerScreen`

- [ ] `DrawerScreen.Slot4.Slot4(3)`: its class isn't translated (it extends `DrawerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java:174)

### `DrawerStackStorage`: it extends Fabric's `SingleStackStorage`

- [ ] `DrawerStackStorage.DrawerStackStorage(2)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:23)
- [ ] `DrawerStackStorage.updateSlot(1)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:28)
- [ ] `DrawerStackStorage.getStack(0)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:32)
- [ ] `DrawerStackStorage.setStack(1)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:41)
- [ ] `DrawerStackStorage.getCapacity(1)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:49)
- [ ] `DrawerStackStorage.isAccessible(0)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:53)
- [ ] `DrawerStackStorage.getDrawerAttributes(1)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:62)
- [ ] `DrawerStackStorage.checkControllerVoid(1)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:75)
- [ ] `DrawerStackStorage.insert(3)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:94)
- [ ] `DrawerStackStorage.extract(3)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:124)
- [ ] `DrawerStackStorage.releaseSnapshot(1)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:132)
- [ ] `DrawerStackStorage.onFinalCommit(0)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:136)
- [ ] `DrawerStackStorage.rebalanceBalancedFill(0)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java:157)

### `DrawerStorageImpl`: it extends Fabric's `CombinedStorage`

- [ ] `DrawerStorageImpl.WRAPPERS`: it uses IDrawerGroup, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStorageImpl.java:18)
- [ ] `DrawerStorageImpl.of(1)`: it uses DrawerStorageImpl, which isn't translated (it extends Fabric's `CombinedStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStorageImpl.java:22)
- [ ] `DrawerStorageImpl.DrawerStorageImpl(1)`: its class isn't translated (it extends Fabric's `CombinedStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStorageImpl.java:28)
- [ ] `DrawerStorageImpl.getSlots(0)`: its class isn't translated (it extends Fabric's `CombinedStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStorageImpl.java:34)
- [ ] `DrawerStorageImpl.resizeSlotList(0)`: its class isn't translated (it extends Fabric's `CombinedStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStorageImpl.java:38)
- [ ] `DrawerStorageImpl.getSlotCount(0)`: its class isn't translated (it extends Fabric's `CombinedStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStorageImpl.java:53)
- [ ] `DrawerStorageImpl.getSlot(1)`: its class isn't translated (it extends Fabric's `CombinedStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStorageImpl.java:57)
- [ ] `DrawerStorageImpl.getDrawer(1)`: its class isn't translated (it extends Fabric's `CombinedStorage`) (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStorageImpl.java:61)

### `FramingTableScreen`: it extends Minecraft's `AbstractContainerScreen`

- [x] `FramingTableScreen.guiTextires` (com/jaquadro/minecraft/storagedrawers/inventory/FramingTableScreen.java:12)
- [ ] `FramingTableScreen.FramingTableScreen(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/FramingTableScreen.java:16)
- [ ] `FramingTableScreen.extractLabels(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/FramingTableScreen.java:21)
- [ ] `FramingTableScreen.extractBackground(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/jaquadro/minecraft/storagedrawers/inventory/FramingTableScreen.java:26)

### `InventoryUpgrade`: it implements Minecraft's `Container`

- [x] `InventoryUpgrade.upgradeCapacity` (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:14)
- [ ] `InventoryUpgrade.InventoryUpgrade(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:17)
- [ ] `InventoryUpgrade.getContainerSize(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:21)
- [ ] `InventoryUpgrade.isEmpty(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:25)
- [ ] `InventoryUpgrade.getItem(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:39)
- [ ] `InventoryUpgrade.removeItem(2)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:44)
- [ ] `InventoryUpgrade.removeItemNoUpdate(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:58)
- [ ] `InventoryUpgrade.setItem(2)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:63)
- [ ] `InventoryUpgrade.getMaxStackSize(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:69)
- [ ] `InventoryUpgrade.setChanged(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:73)
- [ ] `InventoryUpgrade.stillValid(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:79)
- [ ] `InventoryUpgrade.startOpen(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:90)
- [ ] `InventoryUpgrade.stopOpen(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:93)
- [ ] `InventoryUpgrade.canPlaceItem(2)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:96)
- [ ] `InventoryUpgrade.clearContent(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:106)
- [ ] `InventoryUpgrade.getStorageMultiplier(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:109)
- [ ] `InventoryUpgrade.getStackCapacity(0)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:113)
- [ ] `InventoryUpgrade.slotIsLocked(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:125)
- [ ] `InventoryUpgrade.canAddUpgrade(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:135)
- [ ] `InventoryUpgrade.canRemoveUpgrade(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:145)
- [ ] `InventoryUpgrade.canRemoveStorageUpgrade(1)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:155)
- [ ] `InventoryUpgrade.canSwapUpgrade(2)`: its class isn't translated (it implements Minecraft's `Container`) (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java:165)

### `ItemStackHelper`

- [x] `ItemStackHelper.getTrueItem(1)` in game_side.rs (com/jaquadro/minecraft/storagedrawers/inventory/ItemStackHelper.java:11)
- [x] `ItemStackHelper.getItemPrototype(1)` in game_side.rs (com/jaquadro/minecraft/storagedrawers/inventory/ItemStackHelper.java:15)
- [ ] `ItemStackHelper.getMaxStackSize(1)`: it calls Minecraft's ItemStack.getOrDefault (no Horizon helper for it yet) (com/jaquadro/minecraft/storagedrawers/inventory/ItemStackHelper.java:20)
- [ ] `ItemStackHelper.encodeItemStack(1)`: it calls Minecraft's ItemStack.set (no Horizon helper for it yet) (com/jaquadro/minecraft/storagedrawers/inventory/ItemStackHelper.java:24)
- [ ] `ItemStackHelper.encodeItemStack(2)`: it calls Minecraft's ItemStack.setCount (no Horizon helper for it yet) (com/jaquadro/minecraft/storagedrawers/inventory/ItemStackHelper.java:39)
- [ ] `ItemStackHelper.decodeItemStack(1)`: it calls ItemStackHelper.decodedCount, which isn't translated (com/jaquadro/minecraft/storagedrawers/inventory/ItemStackHelper.java:55)
- [ ] `ItemStackHelper.decodeItemStackPrototype(1)`: it calls ItemStackHelper.stripDecoding, which isn't translated (com/jaquadro/minecraft/storagedrawers/inventory/ItemStackHelper.java:62)
- [ ] `ItemStackHelper.decodedCount(1)`: it calls Minecraft's ItemStack.get (no Horizon helper for it yet) (com/jaquadro/minecraft/storagedrawers/inventory/ItemStackHelper.java:68)
- [ ] `ItemStackHelper.stripDecoding(1)`: it calls Minecraft's ItemStack.remove (no Horizon helper for it yet) (com/jaquadro/minecraft/storagedrawers/inventory/ItemStackHelper.java:73)
- [ ] `ItemStackHelper.isStackEncoded(1)`: it calls Minecraft's ItemStack.get (no Horizon helper for it yet) (com/jaquadro/minecraft/storagedrawers/inventory/ItemStackHelper.java:79)

### `RestrictedSlot`: it extends Minecraft's `Slot`

- [ ] `RestrictedSlot.RestrictedSlot(4)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/RestrictedSlot.java:9)
- [ ] `RestrictedSlot.mayPlace(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/RestrictedSlot.java:13)

### `SlotDrawer`: it extends Minecraft's `Slot`

- [ ] `SlotDrawer.emptyInventory`: it uses Minecraft's `Container` (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:16)
- [ ] `SlotDrawer.SlotDrawer(5)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:21)
- [ ] `SlotDrawer.mayPlace(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:28)
- [ ] `SlotDrawer.getItem(0)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:32)
- [ ] `SlotDrawer.set(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:37)
- [ ] `SlotDrawer.onQuickCraft(2)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:43)
- [ ] `SlotDrawer.getMaxStackSize(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:46)
- [ ] `SlotDrawer.mayPickup(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:50)
- [ ] `SlotDrawer.remove(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:54)
- [ ] `SlotDrawer.getDrawerGroup(0)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:63)
- [ ] `SlotDrawer.getDrawer(0)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:67)
- [ ] `SlotDrawer.isSameInventory(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:71)

### `SlotDrawer.EmptyInventory`: it extends Minecraft's `SimpleContainer`

- [ ] `SlotDrawer.EmptyInventory.EmptyInventory(0)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:76)
- [ ] `SlotDrawer.EmptyInventory.getSlotsForFace(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:80)
- [ ] `SlotDrawer.EmptyInventory.canPlaceItemThroughFace(3)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:84)
- [ ] `SlotDrawer.EmptyInventory.canTakeItemThroughFace(3)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java:88)

### `SlotStorage`: it extends Minecraft's `Slot`

- [ ] `SlotStorage.SlotStorage(4)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotStorage.java:11)
- [ ] `SlotStorage.SlotStorage(5)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotStorage.java:15)
- [ ] `SlotStorage.mayPickup(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotStorage.java:20)
- [ ] `SlotStorage.isMissing(0)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotStorage.java:24)

### `SlotUpgrade`: it extends Minecraft's `Slot`

- [ ] `SlotUpgrade.SlotUpgrade(4)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotUpgrade.java:13)
- [ ] `SlotUpgrade.mayPlace(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotUpgrade.java:17)
- [ ] `SlotUpgrade.mayPickup(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotUpgrade.java:27)
- [ ] `SlotUpgrade.canTakeStack(0)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotUpgrade.java:31)
- [ ] `SlotUpgrade.canTakeStack(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotUpgrade.java:35)
- [ ] `SlotUpgrade.canSwapStack(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/jaquadro/minecraft/storagedrawers/inventory/SlotUpgrade.java:54)

### `DetachedDrawerTooltip`: it implements Minecraft's `TooltipComponent`

- [ ] `DetachedDrawerTooltip.DetachedDrawerTooltip(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/inventory/tooltip/DetachedDrawerTooltip.java:6)
- [ ] `DetachedDrawerTooltip.contents(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/inventory/tooltip/DetachedDrawerTooltip.java:6)

### `KeyringTooltip`: it implements Minecraft's `TooltipComponent`

- [ ] `KeyringTooltip.KeyringTooltip(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/inventory/tooltip/KeyringTooltip.java:6)
- [ ] `KeyringTooltip.contents(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/inventory/tooltip/KeyringTooltip.java:6)

### `EnumUpgradeCreative`: its constants carry values

- [ ] `EnumUpgradeCreative.META_LOOKUP`: it uses EnumUpgradeCreative, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeCreative.java:10)
- [ ] `EnumUpgradeCreative.EnumUpgradeCreative(3)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeCreative.java:15)
- [ ] `EnumUpgradeCreative.getMetadata(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeCreative.java:21)
- [ ] `EnumUpgradeCreative.getUnlocalizedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeCreative.java:25)
- [ ] `EnumUpgradeCreative.byMetadata(1)`: it uses EnumUpgradeCreative, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeCreative.java:29)
- [ ] `EnumUpgradeCreative.toString(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeCreative.java:37)
- [ ] `EnumUpgradeCreative.getSerializedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeCreative.java:42)

### `EnumUpgradeMagnet`: its constants carry values

- [ ] `EnumUpgradeMagnet.META_LOOKUP`: it uses EnumUpgradeMagnet, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeMagnet.java:11)
- [ ] `EnumUpgradeMagnet.EnumUpgradeMagnet(4)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeMagnet.java:17)
- [ ] `EnumUpgradeMagnet.getMetadata(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeMagnet.java:24)
- [ ] `EnumUpgradeMagnet.getLevel(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeMagnet.java:28)
- [ ] `EnumUpgradeMagnet.getUnlocalizedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeMagnet.java:32)
- [ ] `EnumUpgradeMagnet.byMetadata(1)`: it uses EnumUpgradeMagnet, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeMagnet.java:36)
- [ ] `EnumUpgradeMagnet.toString(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeMagnet.java:44)
- [ ] `EnumUpgradeMagnet.getSerializedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeMagnet.java:49)

### `EnumUpgradeRedstone`: its constants carry values

- [ ] `EnumUpgradeRedstone.META_LOOKUP`: it uses EnumUpgradeRedstone, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeRedstone.java:11)
- [ ] `EnumUpgradeRedstone.EnumUpgradeRedstone(3)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeRedstone.java:16)
- [ ] `EnumUpgradeRedstone.getMetadata(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeRedstone.java:22)
- [ ] `EnumUpgradeRedstone.getUnlocalizedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeRedstone.java:26)
- [ ] `EnumUpgradeRedstone.byMetadata(1)`: it uses EnumUpgradeRedstone, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeRedstone.java:30)
- [ ] `EnumUpgradeRedstone.toString(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeRedstone.java:38)
- [ ] `EnumUpgradeRedstone.getSerializedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeRedstone.java:43)

### `EnumUpgradeStorage`: its constants carry values

- [ ] `EnumUpgradeStorage.META_LOOKUP`: it uses EnumUpgradeStorage, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeStorage.java:15)
- [ ] `EnumUpgradeStorage.LEVEL_LOOKUP`: it uses EnumUpgradeStorage, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeStorage.java:16)
- [ ] `EnumUpgradeStorage.EnumUpgradeStorage(4)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeStorage.java:22)
- [ ] `EnumUpgradeStorage.getMetadata(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeStorage.java:29)
- [ ] `EnumUpgradeStorage.getLevel(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeStorage.java:33)
- [ ] `EnumUpgradeStorage.getUnlocalizedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeStorage.java:37)
- [ ] `EnumUpgradeStorage.byMetadata(1)`: it uses EnumUpgradeStorage, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeStorage.java:41)
- [ ] `EnumUpgradeStorage.byLevel(1)`: it uses EnumUpgradeStorage, which isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeStorage.java:49)
- [ ] `EnumUpgradeStorage.toString(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeStorage.java:57)
- [ ] `EnumUpgradeStorage.getSerializedName(0)`: its class isn't translated (its constants carry values) (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeStorage.java:62)

### `ItemDetachedDrawer`: it extends Minecraft's `Item`

- [ ] `ItemDetachedDrawer.ItemDetachedDrawer(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemDetachedDrawer.java:30)
- [ ] `ItemDetachedDrawer.getDefaultInstance(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemDetachedDrawer.java:34)
- [ ] `ItemDetachedDrawer.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemDetachedDrawer.java:44)
- [ ] `ItemDetachedDrawer.getDescription(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemDetachedDrawer.java:52)
- [ ] `ItemDetachedDrawer.getTooltipImage(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemDetachedDrawer.java:59)
- [ ] `ItemDetachedDrawer.isHeavy(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemDetachedDrawer.java:70)
- [ ] `ItemDetachedDrawer.canFitInsideContainerItems(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemDetachedDrawer.java:82)

### `ItemDrawerKey`: it extends `ItemKey`

- [ ] `ItemDrawerKey.ItemDrawerKey(1)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawerKey.java:9)
- [ ] `ItemDrawerKey.handleDrawerAttributes(1)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawerKey.java:13)
- [ ] `ItemDrawerKey.isEnabled(0)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawerKey.java:20)

### `ItemDrawerPuller`: it extends `ItemKey`

- [ ] `ItemDrawerPuller.ItemDrawerPuller(1)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawerPuller.java:7)
- [ ] `ItemDrawerPuller.isEnabled(0)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawerPuller.java:11)

### `ItemDrawers`: it extends Minecraft's `BlockItem`

- [ ] `ItemDrawers.ItemDrawers(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawers.java:32)
- [ ] `ItemDrawers.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawers.java:36)
- [ ] `ItemDrawers.getName(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawers.java:52)
- [ ] `ItemDrawers.getDescription(0)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawers.java:67)
- [ ] `ItemDrawers.isHeavy(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawers.java:72)
- [ ] `ItemDrawers.getCapacityForBlock(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawers.java:94)
- [ ] `ItemDrawers.canFitInsideContainerItems(0)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawers.java:100)
- [ ] `ItemDrawers.doesSneakBypassUse(4)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/jaquadro/minecraft/storagedrawers/item/ItemDrawers.java:104)

### `ItemFramedDrawers`: it extends `ItemDrawers`

- [ ] `ItemFramedDrawers.ItemFramedDrawers(2)`: its class isn't translated (it extends `ItemDrawers`) (com/jaquadro/minecraft/storagedrawers/item/ItemFramedDrawers.java:12)
- [ ] `ItemFramedDrawers.placeBlock(2)`: its class isn't translated (it extends `ItemDrawers`) (com/jaquadro/minecraft/storagedrawers/item/ItemFramedDrawers.java:16)

### `ItemFramedTrim`: it extends `ItemTrim`

- [ ] `ItemFramedTrim.ItemFramedTrim(2)`: its class isn't translated (it extends `ItemTrim`) (com/jaquadro/minecraft/storagedrawers/item/ItemFramedTrim.java:12)
- [ ] `ItemFramedTrim.placeBlock(2)`: its class isn't translated (it extends `ItemTrim`) (com/jaquadro/minecraft/storagedrawers/item/ItemFramedTrim.java:16)

### `ItemKey`: it extends Minecraft's `Item`

- [ ] `ItemKey.ItemKey(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKey.java:34)
- [ ] `ItemKey.createAttributes(0)`: it uses Minecraft's `ItemAttributeModifiers` (com/jaquadro/minecraft/storagedrawers/item/ItemKey.java:38)
- [ ] `ItemKey.canDestroyBlock(5)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKey.java:44)
- [ ] `ItemKey.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKey.java:50)
- [ ] `ItemKey.getDescription(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKey.java:58)
- [ ] `ItemKey.useOn(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKey.java:63)
- [ ] `ItemKey.handleDrawerAttributes(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKey.java:91)
- [ ] `ItemKey.isEnabled(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKey.java:94)

### `ItemKeyring`: it extends Minecraft's `Item`

- [x] `ItemKeyring.TAG_ITEMS` (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:37)
- [ ] `ItemKeyring.ItemKeyring(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:40)
- [ ] `ItemKeyring.getKey(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:45)
- [ ] `ItemKeyring.overrideOtherStackedOnMe(6)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:49)
- [ ] `ItemKeyring.use(3)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:99)
- [ ] `ItemKeyring.useOn(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:114)
- [ ] `ItemKeyring.getKeyring(1)`: `instanceof ItemKey` on an item (no single item id for it) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:131)
- [ ] `ItemKeyring.getTooltipImage(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:140)
- [ ] `ItemKeyring.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:151)
- [ ] `ItemKeyring.getName(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:159)
- [ ] `ItemKeyring.getDescription(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:176)
- [ ] `ItemKeyring.onDestroyed(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:181)
- [ ] `ItemKeyring.getActiveKey(2)`: it uses KeyringContents, which isn't translated (it implements Minecraft's `TooltipComponent`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:189)
- [ ] `ItemKeyring.getContents(1)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:196)
- [ ] `ItemKeyring.rotateKeyring(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java:201)

### `ItemPersonalKey`: it extends `ItemKey`

- [ ] `ItemPersonalKey.ItemPersonalKey(2)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemPersonalKey.java:11)
- [ ] `ItemPersonalKey.handleDrawerAttributes(1)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemPersonalKey.java:16)
- [ ] `ItemPersonalKey.getSecurityProviderKey(0)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemPersonalKey.java:21)
- [ ] `ItemPersonalKey.isEnabled(0)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemPersonalKey.java:25)

### `ItemPriorityKey`: it extends `ItemKey`

- [ ] `ItemPriorityKey.priorityLookup`: it uses ItemPriorityKey, which isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemPriorityKey.java:19)
- [ ] `ItemPriorityKey.ItemPriorityKey(3)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemPriorityKey.java:21)
- [ ] `ItemPriorityKey.getNextKey(0)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemPriorityKey.java:28)
- [ ] `ItemPriorityKey.handleDrawerAttributes(1)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemPriorityKey.java:32)
- [ ] `ItemPriorityKey.use(3)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemPriorityKey.java:43)
- [ ] `ItemPriorityKey.isEnabled(0)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemPriorityKey.java:58)

### `ItemQuantifyKey`: it extends `ItemKey`

- [ ] `ItemQuantifyKey.ItemQuantifyKey(1)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemQuantifyKey.java:8)
- [ ] `ItemQuantifyKey.handleDrawerAttributes(1)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemQuantifyKey.java:12)
- [ ] `ItemQuantifyKey.isEnabled(0)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemQuantifyKey.java:17)

### `ItemShroudKey`: it extends `ItemKey`

- [ ] `ItemShroudKey.ItemShroudKey(1)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemShroudKey.java:8)
- [ ] `ItemShroudKey.handleDrawerAttributes(1)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemShroudKey.java:12)
- [ ] `ItemShroudKey.isEnabled(0)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemShroudKey.java:17)

### `ItemSuspendKey`: it extends `ItemKey`

- [ ] `ItemSuspendKey.ItemSuspendKey(1)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemSuspendKey.java:8)
- [ ] `ItemSuspendKey.handleDrawerAttributes(1)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemSuspendKey.java:12)
- [ ] `ItemSuspendKey.isEnabled(0)`: its class isn't translated (it extends `ItemKey`) (com/jaquadro/minecraft/storagedrawers/item/ItemSuspendKey.java:17)

### `ItemTrim`: it extends Minecraft's `BlockItem`

- [ ] `ItemTrim.ItemTrim(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/jaquadro/minecraft/storagedrawers/item/ItemTrim.java:16)
- [ ] `ItemTrim.getName(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/jaquadro/minecraft/storagedrawers/item/ItemTrim.java:20)
- [ ] `ItemTrim.doesSneakBypassUse(4)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/jaquadro/minecraft/storagedrawers/item/ItemTrim.java:33)

### `ItemUpgrade`: it extends Minecraft's `Item`

- [x] `ItemUpgrade.nextGroupId` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java:19)
- [ ] `ItemUpgrade.ItemUpgrade(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java:23)
- [ ] `ItemUpgrade.ItemUpgrade(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java:27)
- [ ] `ItemUpgrade.getNextGroupId(0)`: `++` inside an expression (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java:32)
- [ ] `ItemUpgrade.doesSneakBypassUse(4)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java:36)
- [ ] `ItemUpgrade.getUpgradeGroup(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java:40)
- [ ] `ItemUpgrade.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java:44)
- [ ] `ItemUpgrade.getDescription(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java:52)
- [ ] `ItemUpgrade.isEnabled(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java:57)
- [ ] `ItemUpgrade.setAllowMultiple(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java:61)
- [ ] `ItemUpgrade.getAllowMultiple(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java:65)

### `ItemUpgradeBalance`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeBalance.ItemUpgradeBalance(1)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeBalance.java:10)
- [ ] `ItemUpgradeBalance.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeBalance.java:14)
- [ ] `ItemUpgradeBalance.getDescription(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeBalance.java:19)

### `ItemUpgradeConversion`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeConversion.ItemUpgradeConversion(1)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeConversion.java:7)
- [ ] `ItemUpgradeConversion.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeConversion.java:11)

### `ItemUpgradeCreative`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeCreative.creativeGroupId`: it calls ItemUpgrade.getNextGroupId, which isn't translated (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeCreative.java:7)
- [ ] `ItemUpgradeCreative.ItemUpgradeCreative(2)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeCreative.java:10)
- [ ] `ItemUpgradeCreative.ItemUpgradeCreative(3)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeCreative.java:14)
- [ ] `ItemUpgradeCreative.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeCreative.java:19)

### `ItemUpgradeFillLevel`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeFillLevel.ItemUpgradeFillLevel(1)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeFillLevel.java:7)
- [ ] `ItemUpgradeFillLevel.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeFillLevel.java:11)

### `ItemUpgradeHopper`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeHopper.ItemUpgradeHopper(1)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeHopper.java:7)
- [ ] `ItemUpgradeHopper.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeHopper.java:11)

### `ItemUpgradeIllumination`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeIllumination.ItemUpgradeIllumination(1)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeIllumination.java:7)
- [ ] `ItemUpgradeIllumination.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeIllumination.java:11)

### `ItemUpgradeMagnet`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeMagnet.magnetGroupId`: it calls ItemUpgrade.getNextGroupId, which isn't translated (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:17)
- [ ] `ItemUpgradeMagnet.ItemUpgradeMagnet(2)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:20)
- [ ] `ItemUpgradeMagnet.ItemUpgradeMagnet(3)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:24)
- [ ] `ItemUpgradeMagnet.getDescription(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:30)
- [ ] `ItemUpgradeMagnet.appendHoverText(5)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:36)
- [ ] `ItemUpgradeMagnet.buildRangeString(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:48)
- [ ] `ItemUpgradeMagnet.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:72)
- [ ] `ItemUpgradeMagnet.getActiveSpeed(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:77)
- [ ] `ItemUpgradeMagnet.getIdleSpeed(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:81)
- [ ] `ItemUpgradeMagnet.getHorzRange(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:85)
- [ ] `ItemUpgradeMagnet.getUpRange(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:89)
- [ ] `ItemUpgradeMagnet.getDownRange(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:93)
- [ ] `ItemUpgradeMagnet.getIndexedRange(1)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:97)
- [ ] `ItemUpgradeMagnet.getConfig(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java:101)

### `ItemUpgradeOneStack`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeOneStack.ItemUpgradeOneStack(1)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeOneStack.java:7)
- [ ] `ItemUpgradeOneStack.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeOneStack.java:11)

### `ItemUpgradePortability`: it extends `ItemUpgrade`

- [ ] `ItemUpgradePortability.ItemUpgradePortability(1)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradePortability.java:7)
- [ ] `ItemUpgradePortability.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradePortability.java:11)

### `ItemUpgradeRedstone`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeRedstone.redstoneGroupId`: it calls ItemUpgrade.getNextGroupId, which isn't translated (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRedstone.java:7)
- [ ] `ItemUpgradeRedstone.ItemUpgradeRedstone(2)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRedstone.java:10)
- [ ] `ItemUpgradeRedstone.ItemUpgradeRedstone(3)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRedstone.java:14)
- [ ] `ItemUpgradeRedstone.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRedstone.java:19)

### `ItemUpgradeRemote`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeRemote.remoteGroupId`: it calls ItemUpgrade.getNextGroupId, which isn't translated (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:25)
- [ ] `ItemUpgradeRemote.ItemUpgradeRemote(3)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:29)
- [ ] `ItemUpgradeRemote.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:35)
- [ ] `ItemUpgradeRemote.appendHoverText(5)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:44)
- [ ] `ItemUpgradeRemote.getBoundPosition(1)`: it uses Minecraft's `BlockPos` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:55)
- [ ] `ItemUpgradeRemote.setBoundController(2)`: it uses BlockEntityController, which isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:64)
- [ ] `ItemUpgradeRemote.setUnbound(1)`: it uses ModCommonConfig.INSTANCE, which isn't translated (it uses ModCommonConfig, which isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config))) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:85)
- [ ] `ItemUpgradeRemote.copyControllerBinding(2)`: `null` (Rust has no null: this needs an Option) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:95)
- [ ] `ItemUpgradeRemote.getBoundController(2)`: it uses BlockEntityController, which isn't translated (it extends `BaseBlockEntity`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:119)
- [ ] `ItemUpgradeRemote.validateInventory(2)`: Minecraft's Inventory kept in a variable or field (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:132)
- [ ] `ItemUpgradeRemote.isGroupUpgrade(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:143)
- [ ] `ItemUpgradeRemote.isBound(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java:147)

### `ItemUpgradeStorage`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeStorage.storageGroupId`: it calls ItemUpgrade.getNextGroupId, which isn't translated (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeStorage.java:9)
- [ ] `ItemUpgradeStorage.ItemUpgradeStorage(2)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeStorage.java:12)
- [ ] `ItemUpgradeStorage.ItemUpgradeStorage(3)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeStorage.java:16)
- [ ] `ItemUpgradeStorage.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeStorage.java:22)
- [ ] `ItemUpgradeStorage.getDescription(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeStorage.java:27)

### `ItemUpgradeVoid`: it extends `ItemUpgrade`

- [ ] `ItemUpgradeVoid.ItemUpgradeVoid(1)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeVoid.java:7)
- [ ] `ItemUpgradeVoid.isEnabled(0)`: its class isn't translated (it extends `ItemUpgrade`) (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeVoid.java:11)

### `ModConstants`

- [x] `ModConstants.MOD_ID` (com/jaquadro/minecraft/storagedrawers/ModConstants.java:6)
- [x] `ModConstants.loc(1)` (com/jaquadro/minecraft/storagedrawers/ModConstants.java:8)

### `ModServices`

- [x] `ModServices.log` (com/jaquadro/minecraft/storagedrawers/ModServices.java:9)
- [ ] `ModServices.RESOURCE_FACTORY`: it uses ResourceFactory, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/ModServices.java:10)
- [ ] `ModServices.load(1)`: it's generic (com/jaquadro/minecraft/storagedrawers/ModServices.java:12)

### `CountUpdateMessage`: it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)

- [ ] `CountUpdateMessage.TYPE`: no constructor of Type takes 1 values (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java:16)
- [ ] `CountUpdateMessage.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java:17)
- [ ] `CountUpdateMessage.CountUpdateMessage(5)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java:15)
- [ ] `CountUpdateMessage.CountUpdateMessage(3)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java:31)
- [ ] `CountUpdateMessage.type(0)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java:35)
- [ ] `CountUpdateMessage.handleMessage(2)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java:39)
- [ ] `CountUpdateMessage.x(0)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java:15)
- [ ] `CountUpdateMessage.y(0)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java:15)
- [ ] `CountUpdateMessage.z(0)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java:15)
- [ ] `CountUpdateMessage.slot(0)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java:15)
- [ ] `CountUpdateMessage.count(0)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java:15)

### `CountUpdateMessageHandler`

- [ ] `CountUpdateMessageHandler.handle(1)`: it uses CountUpdateMessage, which isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessageHandler.java:10)

### `PlayerBoolConfigMessage`: it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)

- [ ] `PlayerBoolConfigMessage.TYPE`: no constructor of Type takes 1 values (com/jaquadro/minecraft/storagedrawers/network/PlayerBoolConfigMessage.java:19)
- [ ] `PlayerBoolConfigMessage.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/jaquadro/minecraft/storagedrawers/network/PlayerBoolConfigMessage.java:20)
- [ ] `PlayerBoolConfigMessage.PlayerBoolConfigMessage(3)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/PlayerBoolConfigMessage.java:18)
- [ ] `PlayerBoolConfigMessage.type(0)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/PlayerBoolConfigMessage.java:30)
- [ ] `PlayerBoolConfigMessage.handleMessage(2)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/PlayerBoolConfigMessage.java:34)
- [ ] `PlayerBoolConfigMessage.uuid(0)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/PlayerBoolConfigMessage.java:18)
- [ ] `PlayerBoolConfigMessage.key(0)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/PlayerBoolConfigMessage.java:18)
- [ ] `PlayerBoolConfigMessage.value(0)`: its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)) (com/jaquadro/minecraft/storagedrawers/network/PlayerBoolConfigMessage.java:18)

### `DefaultInteractionProvider`: it implements `IInteractionProvider` (com.jaquadro.minecraft.storagedrawers.api.security)

- [ ] `DefaultInteractionProvider.getProviderID(0)`: its class isn't translated (it implements `IInteractionProvider` (com.jaquadro.minecraft.storagedrawers.api.security)) (com/jaquadro/minecraft/storagedrawers/security/DefaultInteractionProvider.java:9)
- [ ] `DefaultInteractionProvider.canInteract(3)`: its class isn't translated (it implements `IInteractionProvider` (com.jaquadro.minecraft.storagedrawers.api.security)) (com/jaquadro/minecraft/storagedrawers/security/DefaultInteractionProvider.java:14)

### `DefaultSecurityProvider`: it implements `ISecurityProvider` (com.jaquadro.minecraft.storagedrawers.api.security)

- [ ] `DefaultSecurityProvider.getProviderID(0)`: its class isn't translated (it implements `ISecurityProvider` (com.jaquadro.minecraft.storagedrawers.api.security)) (com/jaquadro/minecraft/storagedrawers/security/DefaultSecurityProvider.java:9)
- [ ] `DefaultSecurityProvider.hasOwnership(2)`: its class isn't translated (it implements `ISecurityProvider` (com.jaquadro.minecraft.storagedrawers.api.security)) (com/jaquadro/minecraft/storagedrawers/security/DefaultSecurityProvider.java:14)
- [ ] `DefaultSecurityProvider.hasAccess(2)`: its class isn't translated (it implements `ISecurityProvider` (com.jaquadro.minecraft.storagedrawers.api.security)) (com/jaquadro/minecraft/storagedrawers/security/DefaultSecurityProvider.java:19)

### `SecurityManager`

- [ ] `SecurityManager.defaultProvider`: it uses ISecurityProvider, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/security/SecurityManager.java:15)
- [ ] `SecurityManager.defaultInteractProvider`: it uses IInteractionProvider, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/security/SecurityManager.java:16)
- [ ] `SecurityManager.hasOwnership(2)`: it uses Minecraft's `GameProfile` (com/jaquadro/minecraft/storagedrawers/security/SecurityManager.java:18)
- [ ] `SecurityManager.hasAccess(2)`: it uses IProtectable, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/security/SecurityManager.java:31)
- [ ] `SecurityManager.canInteract(3)`: it uses Minecraft's `BlockPos` (com/jaquadro/minecraft/storagedrawers/security/SecurityManager.java:44)
- [ ] `SecurityManager.clientHasAccess(1)`: it uses IProtectable, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/security/SecurityManager.java:58)

### `SecurityRegistry`: it uses ISecurityProvider, which isn't translated (it's an interface)

- [ ] `SecurityRegistry.registerProvider(1)`: its class isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/security/SecurityRegistry.java:13)
- [ ] `SecurityRegistry.registerProvider(1)`: its class isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/security/SecurityRegistry.java:17)
- [ ] `SecurityRegistry.getProvider(1)`: its class isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/security/SecurityRegistry.java:21)
- [ ] `SecurityRegistry.getInteractionProvider(1)`: its class isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/security/SecurityRegistry.java:25)
- [ ] `SecurityRegistry.getAllInteractionProviders(0)`: its class isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)) (com/jaquadro/minecraft/storagedrawers/security/SecurityRegistry.java:29)

### `PlatformResourceFactory`: it implements `ResourceFactory`

- [ ] `PlatformResourceFactory.createBlockEntityDrawersStandard(1)`: its class isn't translated (it implements `ResourceFactory`) (com/jaquadro/minecraft/storagedrawers/service/PlatformResourceFactory.java:17)
- [ ] `PlatformResourceFactory.createBlockEntityDrawersComp(1)`: its class isn't translated (it implements `ResourceFactory`) (com/jaquadro/minecraft/storagedrawers/service/PlatformResourceFactory.java:27)
- [ ] `PlatformResourceFactory.createBlockEntityController(0)`: its class isn't translated (it implements `ResourceFactory`) (com/jaquadro/minecraft/storagedrawers/service/PlatformResourceFactory.java:36)
- [ ] `PlatformResourceFactory.createBlockEntityControllerIO(0)`: its class isn't translated (it implements `ResourceFactory`) (com/jaquadro/minecraft/storagedrawers/service/PlatformResourceFactory.java:41)
- [ ] `PlatformResourceFactory.createBlockEntityFramingTable(0)`: its class isn't translated (it implements `ResourceFactory`) (com/jaquadro/minecraft/storagedrawers/service/PlatformResourceFactory.java:46)
- [ ] `PlatformResourceFactory.createBlockEntityTrim(0)`: its class isn't translated (it implements `ResourceFactory`) (com/jaquadro/minecraft/storagedrawers/service/PlatformResourceFactory.java:51)

### `ResourceFactory`: it's an interface

- [ ] `ResourceFactory.createBlockEntityDrawersStandard(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/service/ResourceFactory.java:12)
- [ ] `ResourceFactory.createBlockEntityDrawersComp(1)`: it has no body (com/jaquadro/minecraft/storagedrawers/service/ResourceFactory.java:14)
- [ ] `ResourceFactory.createBlockEntityController(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/service/ResourceFactory.java:16)
- [ ] `ResourceFactory.createBlockEntityControllerIO(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/service/ResourceFactory.java:18)
- [ ] `ResourceFactory.createBlockEntityFramingTable(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/service/ResourceFactory.java:20)
- [ ] `ResourceFactory.createBlockEntityTrim(0)`: it has no body (com/jaquadro/minecraft/storagedrawers/service/ResourceFactory.java:22)

### `StorageDrawers`: it implements Fabric's `ModInitializer`

- [ ] `StorageDrawers.api`: it uses Api, which isn't translated (it implements `IStorageDrawersApi` (com.jaquadro.minecraft.storagedrawers.api)) (com/jaquadro/minecraft/storagedrawers/StorageDrawers.java:33)
- [x] `StorageDrawers.gameplayRegistriesInitialized` (com/jaquadro/minecraft/storagedrawers/StorageDrawers.java:34)
- [ ] `StorageDrawers.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (com/jaquadro/minecraft/storagedrawers/StorageDrawers.java:36)
- [x] `StorageDrawers.rl(1)` (com/jaquadro/minecraft/storagedrawers/StorageDrawers.java:69)

### `StorageDrawersClient`: it implements Fabric's `ClientModInitializer`

- [ ] `StorageDrawersClient.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook (com/jaquadro/minecraft/storagedrawers/StorageDrawersClient.java:26)

### `StorageUtil`

- [ ] `StorageUtil.rebalanceDrawers(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/storage/StorageUtil.java:18)
- [ ] `StorageUtil.rebalanceDrawers(3)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/storage/StorageUtil.java:22)
- [ ] `StorageUtil.rebalanceDrawers(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/storage/StorageUtil.java:29)
- [ ] `StorageUtil.rebalanceDrawers(3)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/storage/StorageUtil.java:33)
- [ ] `StorageUtil.onNetwork(1)`: it uses IDrawerGroup, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/storage/StorageUtil.java:52)
- [ ] `StorageUtil.getRebalanceDrawers(3)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/storage/StorageUtil.java:61)
- [ ] `StorageUtil.rebalanceDrawers(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/storage/StorageUtil.java:81)
- [ ] `StorageUtil.rebalanceDrawers(1)`: it uses `Stream` (java.util.stream) (com/jaquadro/minecraft/storagedrawers/storage/StorageUtil.java:87)

### `CompactingHelper`: it uses Minecraft's `Level`

- [ ] `CompactingHelper.lookup1`: it uses InventoryLookup, which isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:31)
- [ ] `CompactingHelper.lookup2`: it uses InventoryLookup, which isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:32)
- [ ] `CompactingHelper.lookup3`: it uses InventoryLookup, which isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:33)
- [ ] `CompactingHelper.CompactingHelper(1)`: its class isn't translated (it uses Minecraft's `Level`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:36)
- [ ] `CompactingHelper.findHigherTier(1)`: its class isn't translated (it uses Minecraft's `Level`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:40)
- [ ] `CompactingHelper.findLowerTier(1)`: its class isn't translated (it uses Minecraft's `Level`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:107)
- [ ] `CompactingHelper.findAllMatchingRecipes(1)`: its class isn't translated (it uses Minecraft's `Level`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:181)
- [ ] `CompactingHelper.findMatchingModCandidate(2)`: its class isn't translated (it uses Minecraft's `Level`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:198)
- [ ] `CompactingHelper.tryMatch(2)`: its class isn't translated (it uses Minecraft's `Level`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:213)
- [ ] `CompactingHelper.setupLookup(2)`: its class isn't translated (it uses Minecraft's `Level`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:260)

### `CompactingHelper.InventoryLookup`: it extends Minecraft's `TransientCraftingContainer`

- [ ] `CompactingHelper.InventoryLookup.InventoryLookup(2)`: its class isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:273)
- [ ] `CompactingHelper.InventoryLookup.getContainerSize(0)`: its class isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:278)
- [ ] `CompactingHelper.InventoryLookup.isEmpty(0)`: its class isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:282)
- [ ] `CompactingHelper.InventoryLookup.getItem(1)`: its class isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:292)
- [ ] `CompactingHelper.InventoryLookup.setItem(2)`: its class isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:296)
- [ ] `CompactingHelper.InventoryLookup.removeItemNoUpdate(1)`: its class isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:302)
- [ ] `CompactingHelper.InventoryLookup.removeItem(2)`: its class isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:307)
- [ ] `CompactingHelper.InventoryLookup.clearContent(0)`: its class isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:312)
- [ ] `CompactingHelper.InventoryLookup.getItems(0)`: its class isn't translated (it extends Minecraft's `TransientCraftingContainer`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:315)

### `CompactingHelper.Result`: it uses Minecraft's `ItemStack`

- [ ] `CompactingHelper.Result.Result(2)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:325)
- [ ] `CompactingHelper.Result.getStack(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:330)
- [ ] `CompactingHelper.Result.getSize(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java:335)

### `ComponentUtil`

- [ ] `ComponentUtil.getSplitDescription(1)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/ComponentUtil.java:13)
- [ ] `ComponentUtil.getSplitDescription(1)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/ComponentUtil.java:20)
- [ ] `ComponentUtil.appendSplitDescription(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/ComponentUtil.java:24)
- [ ] `ComponentUtil.appendSplitDescription(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/ComponentUtil.java:28)
- [ ] `ComponentUtil.appendSplitDescription(3)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/ComponentUtil.java:32)
- [ ] `ComponentUtil.appendSplitDescription(3)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/ComponentUtil.java:38)

### `CountFormatter`

- [ ] `CountFormatter.format(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/CountFormatter.java:7)
- [ ] `CountFormatter.format(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/CountFormatter.java:11)
- [ ] `CountFormatter.formatStackNotation(1)`: it uses IDrawer, which isn't translated (it's an interface) (com/jaquadro/minecraft/storagedrawers/util/CountFormatter.java:15)
- [x] `CountFormatter.fromStackNotation(2)` (com/jaquadro/minecraft/storagedrawers/util/CountFormatter.java:19)
- [ ] `CountFormatter.formatExact(1)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/CountFormatter.java:38)
- [ ] `CountFormatter.formatExact(1)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/CountFormatter.java:42)
- [ ] `CountFormatter.formatApprox(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/CountFormatter.java:46)
- [ ] `CountFormatter.formatApprox(2)`: it's overloaded with the same number of parameters (com/jaquadro/minecraft/storagedrawers/util/CountFormatter.java:50)

### `ItemCollectionRegistry`: it's generic

- [ ] `ItemCollectionRegistry.register(2)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/util/ItemCollectionRegistry.java:14)
- [ ] `ItemCollectionRegistry.getEntries(1)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/util/ItemCollectionRegistry.java:19)
- [ ] `ItemCollectionRegistry.remove(1)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/util/ItemCollectionRegistry.java:23)
- [ ] `ItemCollectionRegistry.clear(1)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/util/ItemCollectionRegistry.java:27)
- [ ] `ItemCollectionRegistry.clear(0)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/util/ItemCollectionRegistry.java:34)
- [ ] `ItemCollectionRegistry.entrySet(0)`: its class isn't translated (it's generic) (com/jaquadro/minecraft/storagedrawers/util/ItemCollectionRegistry.java:40)

### `ItemStackMatcher`: it uses Minecraft's `ItemStack`

- [ ] `ItemStackMatcher.EMPTY`: it uses ItemStackMatcher, which isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/util/ItemStackMatcher.java:7)
- [ ] `ItemStackMatcher.ItemStackMatcher(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/util/ItemStackMatcher.java:11)
- [ ] `ItemStackMatcher.matches(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/jaquadro/minecraft/storagedrawers/util/ItemStackMatcher.java:15)
- [ ] `ItemStackMatcher.areItemsEqual(2)`: it calls Minecraft's `ItemStack`.isSameItem (com/jaquadro/minecraft/storagedrawers/util/ItemStackMatcher.java:19)

### `ItemStackTagMatcher`: it extends `ItemStackMatcher`

- [ ] `ItemStackTagMatcher.ItemStackTagMatcher(1)`: its class isn't translated (it extends `ItemStackMatcher`) (com/jaquadro/minecraft/storagedrawers/util/ItemStackTagMatcher.java:14)
- [ ] `ItemStackTagMatcher.matches(1)`: its class isn't translated (it extends `ItemStackMatcher`) (com/jaquadro/minecraft/storagedrawers/util/ItemStackTagMatcher.java:19)
- [ ] `ItemStackTagMatcher.refreshTagMatches(0)`: its class isn't translated (it extends `ItemStackMatcher`) (com/jaquadro/minecraft/storagedrawers/util/ItemStackTagMatcher.java:45)

### `WorldUtils`: another class has the same name, in a different package

- [ ] `WorldUtils.WorldUtils(0)`: its class isn't translated (another class has the same name, in a different package) (com/jaquadro/minecraft/storagedrawers/util/WorldUtils.java:17)
- [ ] `WorldUtils.rayTraceEyes(3)`: it uses Minecraft's `BlockHitResult` (com/jaquadro/minecraft/storagedrawers/util/WorldUtils.java:20)
- [ ] `WorldUtils.getBlockEntity(3)`: it's generic (com/jaquadro/minecraft/storagedrawers/util/WorldUtils.java:29)

### `ChameleonInit`: it's an interface

- [ ] `ChameleonInit.init(1)`: it has no body (com/texelsaurus/minecraft/chameleon/api/ChameleonInit.java:4)

### `ChameleonInit.InitContext`


### `ChameleonCapability`: it's an interface

- [ ] `ChameleonCapability.id(0)`: it has no body (com/texelsaurus/minecraft/chameleon/capabilities/ChameleonCapability.java:8)
- [ ] `ChameleonCapability.getCapability(2)`: it has no body (com/texelsaurus/minecraft/chameleon/capabilities/ChameleonCapability.java:10)

### `FabricCapability`: it's generic

- [ ] `FabricCapability.FabricCapability(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/capabilities/FabricCapability.java:16)
- [ ] `FabricCapability.id(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/capabilities/FabricCapability.java:20)
- [ ] `FabricCapability.getCapability(2)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/capabilities/FabricCapability.java:25)
- [ ] `FabricCapability.getCapability(1)`: it's generic (com/texelsaurus/minecraft/chameleon/capabilities/FabricCapability.java:30)
- [ ] `FabricCapability.register(2)`: it's generic (com/texelsaurus/minecraft/chameleon/capabilities/FabricCapability.java:40)

### `IFabricCapability`: it's an interface

- [ ] `IFabricCapability.getCapability(1)`: it has no body (com/texelsaurus/minecraft/chameleon/capabilities/IFabricCapability.java:8)
- [ ] `IFabricCapability.register(2)`: it has no body (com/texelsaurus/minecraft/chameleon/capabilities/IFabricCapability.java:10)

### `ChameleonServices`

- [ ] `ChameleonServices.REGISTRY`: it uses ChameleonRegistries, which isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/ChameleonServices.java:13)
- [ ] `ChameleonServices.NETWORK`: it uses ChameleonNetworking, which isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/ChameleonServices.java:14)
- [ ] `ChameleonServices.CONFIG`: it uses ChameleonConfig, which isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/ChameleonServices.java:15)
- [ ] `ChameleonServices.CAPABILITY`: it uses ChameleonCapabilities, which isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/ChameleonServices.java:16)
- [ ] `ChameleonServices.CONTAINER`: it uses ChameleonContainer, which isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/ChameleonServices.java:17)
- [ ] `ChameleonServices.PLATFORM`: it uses ChameleonPlatform, which isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/ChameleonServices.java:18)
- [ ] `ChameleonServices.RENDER`: it uses ChameleonRender, which isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/ChameleonServices.java:19)
- [ ] `ChameleonServices.load(1)`: it's generic (com/texelsaurus/minecraft/chameleon/ChameleonServices.java:21)
- [ ] `ChameleonServices.clientLoad(1)`: it's generic (com/texelsaurus/minecraft/chameleon/ChameleonServices.java:25)

### `FabricClient`

- [ ] `FabricClient.registerPacket(1)`: it's generic (com/texelsaurus/minecraft/chameleon/client/FabricClient.java:8)
- [ ] `FabricClient.sendToServer(1)`: it's generic (com/texelsaurus/minecraft/chameleon/client/FabricClient.java:12)

### `ConfigSpec`: a function kept in a variable or field (only parameters can be)

- [ ] `ConfigSpec.init(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (com/texelsaurus/minecraft/chameleon/config/ConfigSpec.java:10)
- [ ] `ConfigSpec.setLoaded(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (com/texelsaurus/minecraft/chameleon/config/ConfigSpec.java:13)
- [ ] `ConfigSpec.isLoaded(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (com/texelsaurus/minecraft/chameleon/config/ConfigSpec.java:21)
- [ ] `ConfigSpec.onLoad(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (com/texelsaurus/minecraft/chameleon/config/ConfigSpec.java:25)

### `ForgeApiConfig`: it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)

- [ ] `ForgeApiConfig.ForgeApiConfig(0)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:17)
- [ ] `ForgeApiConfig.ForgeApiConfig(1)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:21)
- [ ] `ForgeApiConfig.init(2)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:25)
- [ ] `ForgeApiConfig.create(1)`: it's generic (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:43)
- [ ] `ForgeApiConfig.define(2)`: it's generic (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:48)
- [ ] `ForgeApiConfig.defineInRange(5)`: it's generic (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:53)
- [ ] `ForgeApiConfig.defineEnum(2)`: it's generic (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:58)
- [ ] `ForgeApiConfig.defineList(3)`: it's generic (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:63)
- [ ] `ForgeApiConfig.pushGroup(1)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:71)
- [ ] `ForgeApiConfig.popGroup(0)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:76)
- [ ] `ForgeApiConfig.comment(1)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:81)
- [ ] `ForgeApiConfig.comment(1)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:86)

### `ForgeApiConfig.ForgeConfigEntry`: it's generic

- [ ] `ForgeApiConfig.ForgeConfigEntry.ForgeConfigEntry(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:95)
- [ ] `ForgeApiConfig.ForgeConfigEntry.build(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:99)
- [ ] `ForgeApiConfig.ForgeConfigEntry.get(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:109)
- [ ] `ForgeApiConfig.ForgeConfigEntry.set(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:114)
- [ ] `ForgeApiConfig.ForgeConfigEntry.define(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:121)

### `ForgeApiConfig.ForgeConfigEntryEnum`: it's generic

- [ ] `ForgeApiConfig.ForgeConfigEntryEnum.ForgeConfigEntryEnum(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:127)
- [ ] `ForgeApiConfig.ForgeConfigEntryEnum.define(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:131)

### `ForgeApiConfig.ForgeConfigEntryList`: it's generic

- [ ] `ForgeApiConfig.ForgeConfigEntryList.ForgeConfigEntryList(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:140)
- [ ] `ForgeApiConfig.ForgeConfigEntryList.validator(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:144)
- [ ] `ForgeApiConfig.ForgeConfigEntryList.define(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:152)

### `ForgeApiConfig.ForgeConfigEntryRange`: it's generic

- [ ] `ForgeApiConfig.ForgeConfigEntryRange.ForgeConfigEntryRange(2)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:161)
- [ ] `ForgeApiConfig.ForgeConfigEntryRange.define(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java:166)

### `StaticConfig`: it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)

- [ ] `StaticConfig.StaticConfig(0)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:10)
- [ ] `StaticConfig.StaticConfig(1)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:14)
- [ ] `StaticConfig.create(1)`: it's generic (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:18)
- [ ] `StaticConfig.init(2)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:23)
- [ ] `StaticConfig.define(2)`: it's generic (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:31)
- [ ] `StaticConfig.defineInRange(5)`: it's generic (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:36)
- [ ] `StaticConfig.defineEnum(2)`: it's generic (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:41)
- [ ] `StaticConfig.defineList(3)`: it's generic (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:46)
- [ ] `StaticConfig.pushGroup(1)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:51)
- [ ] `StaticConfig.popGroup(0)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:55)
- [ ] `StaticConfig.comment(1)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:59)
- [ ] `StaticConfig.comment(1)`: its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)) (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:63)

### `StaticConfig.FabricConfigEntry`: it's generic

- [ ] `StaticConfig.FabricConfigEntry.build(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:68)
- [ ] `StaticConfig.FabricConfigEntry.set(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:73)
- [ ] `StaticConfig.FabricConfigEntry.get(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java:77)

### `ContainerContent`: it's an interface

- [ ] `ContainerContent.serializer(0)`: it has no body (com/texelsaurus/minecraft/chameleon/inventory/ContainerContent.java:4)

### `ContainerContentSerializer`: it's an interface

- [ ] `ContainerContentSerializer.from(1)`: it has no body (com/texelsaurus/minecraft/chameleon/inventory/ContainerContentSerializer.java:7)
- [ ] `ContainerContentSerializer.to(2)`: it has no body (com/texelsaurus/minecraft/chameleon/inventory/ContainerContentSerializer.java:10)

### `ContentMenuProvider`: it's an interface

- [ ] `ContentMenuProvider.createContent(1)`: it has no body (com/texelsaurus/minecraft/chameleon/inventory/ContentMenuProvider.java:8)
- [ ] `ContentMenuProvider.openMenu(1)`: its class isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/inventory/ContentMenuProvider.java:10)

### `PositionContent`: it implements `ContainerContent` (com.texelsaurus.minecraft.chameleon.inventory)

- [ ] `PositionContent.SERIALIZER`: it uses ContainerContentSerializer, which isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/inventory/content/PositionContent.java:14)
- [ ] `PositionContent.PositionContent(1)`: its class isn't translated (it implements `ContainerContent` (com.texelsaurus.minecraft.chameleon.inventory)) (com/texelsaurus/minecraft/chameleon/inventory/content/PositionContent.java:13)
- [ ] `PositionContent.serializer(0)`: its class isn't translated (it implements `ContainerContent` (com.texelsaurus.minecraft.chameleon.inventory)) (com/texelsaurus/minecraft/chameleon/inventory/content/PositionContent.java:16)
- [ ] `PositionContent.getBlockEntity(2)`: it's generic (com/texelsaurus/minecraft/chameleon/inventory/content/PositionContent.java:21)
- [ ] `PositionContent.getOrNull(3)`: it's generic (com/texelsaurus/minecraft/chameleon/inventory/content/PositionContent.java:25)
- [ ] `PositionContent.pos(0)`: its class isn't translated (it implements `ContainerContent` (com.texelsaurus.minecraft.chameleon.inventory)) (com/texelsaurus/minecraft/chameleon/inventory/content/PositionContent.java:13)

### `PositionContent.Serializer`: it implements `ContainerContentSerializer` (com.texelsaurus.minecraft.chameleon.inventory)

- [ ] `PositionContent.Serializer.from(1)`: its class isn't translated (it implements `ContainerContentSerializer` (com.texelsaurus.minecraft.chameleon.inventory)) (com/texelsaurus/minecraft/chameleon/inventory/content/PositionContent.java:30)
- [ ] `PositionContent.Serializer.to(2)`: its class isn't translated (it implements `ContainerContentSerializer` (com.texelsaurus.minecraft.chameleon.inventory)) (com/texelsaurus/minecraft/chameleon/inventory/content/PositionContent.java:35)

### `ChameleonPacket`: it's an interface

- [ ] `ChameleonPacket.handleMessage(2)`: it has no body (com/texelsaurus/minecraft/chameleon/network/ChameleonPacket.java:8)

### `ChameleonRegistry`: it's an interface

- [ ] `ChameleonRegistry.register(2)`: it has no body (com/texelsaurus/minecraft/chameleon/registry/ChameleonRegistry.java:9)
- [ ] `ChameleonRegistry.getEntries(0)`: it has no body (com/texelsaurus/minecraft/chameleon/registry/ChameleonRegistry.java:11)
- [ ] `ChameleonRegistry.stream(0)`: its class isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/registry/ChameleonRegistry.java:13)

### `FabricRegistry`: it's generic

- [ ] `FabricRegistry.FabricRegistry(2)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/registry/FabricRegistry.java:14)
- [ ] `FabricRegistry.register(2)`: it's generic (com/texelsaurus/minecraft/chameleon/registry/FabricRegistry.java:19)
- [ ] `FabricRegistry.getEntries(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/registry/FabricRegistry.java:25)
- [ ] `FabricRegistry.init(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/registry/FabricRegistry.java:30)

### `FabricRegistryEntry`: it's generic

- [ ] `FabricRegistryEntry.FabricRegistryEntry(2)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/registry/FabricRegistryEntry.java:11)
- [ ] `FabricRegistryEntry.getId(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/registry/FabricRegistryEntry.java:16)
- [ ] `FabricRegistryEntry.get(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/registry/FabricRegistryEntry.java:21)

### `RegistryEntry`: it's an interface

- [ ] `RegistryEntry.getId(0)`: it has no body (com/texelsaurus/minecraft/chameleon/registry/RegistryEntry.java:7)

### `RegistryEntryList`: it's generic

- [ ] `RegistryEntryList.add(1)`: it's generic (com/texelsaurus/minecraft/chameleon/registry/RegistryEntryList.java:10)
- [ ] `RegistryEntryList.getEntries(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/registry/RegistryEntryList.java:15)

### `ChameleonBlockModelPart`: it's an interface

- [ ] `ChameleonBlockModelPart.setRenderType(1)`: its class isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/render/ChameleonBlockModelPart.java:7)
- [ ] `ChameleonBlockModelPart.getRenderType(0)`: it has no body (com/texelsaurus/minecraft/chameleon/render/ChameleonBlockModelPart.java:10)

### `FabricReplacementBlockPart`: it extends `ReplacementBlockPart`

- [ ] `FabricReplacementBlockPart.FabricReplacementBlockPart(2)`: its class isn't translated (it extends `ReplacementBlockPart`) (com/texelsaurus/minecraft/chameleon/render/FabricReplacementBlockPart.java:10)
- [ ] `FabricReplacementBlockPart.setRenderType(1)`: its class isn't translated (it extends `ReplacementBlockPart`) (com/texelsaurus/minecraft/chameleon/render/FabricReplacementBlockPart.java:14)
- [ ] `FabricReplacementBlockPart.getRenderType(0)`: its class isn't translated (it extends `ReplacementBlockPart`) (com/texelsaurus/minecraft/chameleon/render/FabricReplacementBlockPart.java:19)

### `ReplacementBlockPart`: it implements `ChameleonBlockModelPart`

- [ ] `ReplacementBlockPart.ReplacementBlockPart(2)`: its class isn't translated (it implements `ChameleonBlockModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPart.java:19)
- [ ] `ReplacementBlockPart.ReplacementBlockPart(2)`: its class isn't translated (it implements `ChameleonBlockModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPart.java:29)
- [ ] `ReplacementBlockPart.getQuads(1)`: its class isn't translated (it implements `ChameleonBlockModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPart.java:33)
- [ ] `ReplacementBlockPart.useAmbientOcclusion(0)`: its class isn't translated (it implements `ChameleonBlockModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPart.java:37)
- [ ] `ReplacementBlockPart.particleMaterial(0)`: its class isn't translated (it implements `ChameleonBlockModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPart.java:41)
- [ ] `ReplacementBlockPart.materialFlags(0)`: its class isn't translated (it implements `ChameleonBlockModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPart.java:45)
- [ ] `ReplacementBlockPart.remapQuad(2)`: its class isn't translated (it implements `ChameleonBlockModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPart.java:49)
- [ ] `ReplacementBlockPart.remapPackedUV(2)`: its class isn't translated (it implements `ChameleonBlockModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPart.java:68)
- [ ] `ReplacementBlockPart.getUnInterpolatedU(2)`: its class isn't translated (it implements `ChameleonBlockModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPart.java:77)
- [ ] `ReplacementBlockPart.getUnInterpolatedV(2)`: its class isn't translated (it implements `ChameleonBlockModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPart.java:82)

### `ReplacementBlockPartDef`: it uses Minecraft's `BlockStateModelPart`

- [ ] `ReplacementBlockPartDef.ReplacementBlockPartDef(2)`: its class isn't translated (it uses Minecraft's `BlockStateModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPartDef.java:10)
- [ ] `ReplacementBlockPartDef.getPart(0)`: its class isn't translated (it uses Minecraft's `BlockStateModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPartDef.java:15)
- [ ] `ReplacementBlockPartDef.getSprite(0)`: its class isn't translated (it uses Minecraft's `BlockStateModelPart`) (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPartDef.java:19)

### `ChameleonCapabilities`: it's an interface

- [ ] `ChameleonCapabilities.create(3)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonCapabilities.java:7)

### `ChameleonConfig`: it's an interface

- [ ] `ChameleonConfig.create(1)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:10)
- [ ] `ChameleonConfig.init(2)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:12)
- [ ] `ChameleonConfig.define(2)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:14)
- [ ] `ChameleonConfig.defineInRange(5)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:16)
- [ ] `ChameleonConfig.defineInRange(4)`: its class isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:18)
- [ ] `ChameleonConfig.defineEnum(2)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:22)
- [ ] `ChameleonConfig.defineList(3)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:24)
- [ ] `ChameleonConfig.comment(1)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:26)
- [ ] `ChameleonConfig.comment(1)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:28)
- [ ] `ChameleonConfig.pushGroup(1)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:30)
- [ ] `ChameleonConfig.popGroup(0)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:32)

### `ChameleonConfig.ConfigEntry`: it's generic

- [ ] `ChameleonConfig.ConfigEntry.name(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:41)
- [ ] `ChameleonConfig.ConfigEntry.defaultValue(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:46)
- [ ] `ChameleonConfig.ConfigEntry.comment(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:51)
- [ ] `ChameleonConfig.ConfigEntry.range(2)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:56)
- [ ] `ChameleonConfig.ConfigEntry.build(0)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:62)
- [ ] `ChameleonConfig.ConfigEntry.accept(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:64)
- [ ] `ChameleonConfig.ConfigEntry.set(1)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java:69)

### `ChameleonConfig.Type`


### `ChameleonContainer`: it's an interface

- [ ] `ChameleonContainer.getContainerSupplier(2)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonContainer.java:15)
- [ ] `ChameleonContainer.openMenu(2)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonContainer.java:19)

### `ChameleonContainer.ChameleonContainerFactory`: it's an interface

- [ ] `ChameleonContainer.ChameleonContainerFactory.create(3)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonContainer.java:22)
- [ ] `ChameleonContainer.ChameleonContainerFactory.create(2)`: its class isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/service/ChameleonContainer.java:24)

### `ChameleonNetworking`: it's an interface

- [ ] `ChameleonNetworking.registerPacket(3)`: it's generic (com/texelsaurus/minecraft/chameleon/service/ChameleonNetworking.java:12)
- [ ] `ChameleonNetworking.registerPacketInternal(3)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonNetworking.java:16)
- [ ] `ChameleonNetworking.sendToPlayer(2)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonNetworking.java:18)
- [ ] `ChameleonNetworking.sendToPlayersNear(6)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonNetworking.java:20)
- [ ] `ChameleonNetworking.sendToServer(1)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonNetworking.java:22)

### `ChameleonPlatform`: it's an interface

- [ ] `ChameleonPlatform.isPhysicalClient(0)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonPlatform.java:4)

### `ChameleonRegistries`: it's an interface

- [ ] `ChameleonRegistries.create(2)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonRegistries.java:7)

### `ChameleonRender`: it's an interface

- [ ] `ChameleonRender.createReplacementPart(2)`: it has no body (com/texelsaurus/minecraft/chameleon/service/ChameleonRender.java:8)

### `FabricCapabilities`: it implements `ChameleonCapabilities`

- [ ] `FabricCapabilities.capabilties`: it uses IFabricCapability, which isn't translated (it's an interface) (com/texelsaurus/minecraft/chameleon/service/FabricCapabilities.java:16)
- [ ] `FabricCapabilities.create(3)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricCapabilities.java:18)
- [ ] `FabricCapabilities.reigsterCapability(1)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricCapabilities.java:23)

### `FabricCapabilities.DeferredCapability`: it's generic

- [ ] `FabricCapabilities.DeferredCapability.DeferredCapability(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/FabricCapabilities.java:31)
- [ ] `FabricCapabilities.DeferredCapability.id(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/FabricCapabilities.java:35)
- [ ] `FabricCapabilities.DeferredCapability.getCapability(2)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/FabricCapabilities.java:40)
- [ ] `FabricCapabilities.DeferredCapability.getCapability(1)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricCapabilities.java:50)
- [ ] `FabricCapabilities.DeferredCapability.register(2)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricCapabilities.java:60)

### `FabricConfig`: it implements `ChameleonConfig`

- [ ] `FabricConfig.FabricConfig(0)`: its class isn't translated (it implements `ChameleonConfig`) (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:13)
- [ ] `FabricConfig.FabricConfig(1)`: its class isn't translated (it implements `ChameleonConfig`) (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:17)
- [ ] `FabricConfig.init(2)`: its class isn't translated (it implements `ChameleonConfig`) (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:25)
- [ ] `FabricConfig.create(1)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:30)
- [ ] `FabricConfig.define(2)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:35)
- [ ] `FabricConfig.defineInRange(5)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:40)
- [ ] `FabricConfig.defineEnum(2)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:45)
- [ ] `FabricConfig.defineList(3)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:50)
- [ ] `FabricConfig.pushGroup(1)`: its class isn't translated (it implements `ChameleonConfig`) (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:55)
- [ ] `FabricConfig.popGroup(0)`: its class isn't translated (it implements `ChameleonConfig`) (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:60)
- [ ] `FabricConfig.comment(1)`: its class isn't translated (it implements `ChameleonConfig`) (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:65)
- [ ] `FabricConfig.comment(1)`: its class isn't translated (it implements `ChameleonConfig`) (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java:70)

### `FabricContainer`: it implements `ChameleonContainer`

- [ ] `FabricContainer.getContainerSupplier(2)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricContainer.java:22)
- [ ] `FabricContainer.openMenu(2)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricContainer.java:34)

### `FabricContainer.PlatformContainerFactory`: it's generic

- [ ] `FabricContainer.PlatformContainerFactory.PlatformContainerFactory(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/FabricContainer.java:39)
- [ ] `FabricContainer.PlatformContainerFactory.getDisplayName(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/FabricContainer.java:42)
- [ ] `FabricContainer.PlatformContainerFactory.createMenu(3)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/FabricContainer.java:46)
- [ ] `FabricContainer.PlatformContainerFactory.shouldCloseCurrentScreen(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/FabricContainer.java:51)
- [ ] `FabricContainer.PlatformContainerFactory.getScreenOpeningData(1)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/FabricContainer.java:55)
- [ ] `FabricContainer.PlatformContainerFactory.provider(0)`: its class isn't translated (it's generic) (com/texelsaurus/minecraft/chameleon/service/FabricContainer.java:39)

### `FabricNetworking`: it implements `ChameleonNetworking`

- [ ] `FabricNetworking.registerPacketInternal(3)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricNetworking.java:16)
- [ ] `FabricNetworking.sendToPlayer(2)`: its class isn't translated (it implements `ChameleonNetworking`) (com/texelsaurus/minecraft/chameleon/service/FabricNetworking.java:31)
- [ ] `FabricNetworking.sendToPlayersNear(6)`: its class isn't translated (it implements `ChameleonNetworking`) (com/texelsaurus/minecraft/chameleon/service/FabricNetworking.java:36)
- [ ] `FabricNetworking.sendToServer(1)`: its class isn't translated (it implements `ChameleonNetworking`) (com/texelsaurus/minecraft/chameleon/service/FabricNetworking.java:45)

### `FabricPlatform`: it implements `ChameleonPlatform`

- [ ] `FabricPlatform.isPhysicalClient(0)`: its class isn't translated (it implements `ChameleonPlatform`) (com/texelsaurus/minecraft/chameleon/service/FabricPlatform.java:7)

### `FabricRegistries`: it implements `ChameleonRegistries`

- [ ] `FabricRegistries.create(2)`: it's generic (com/texelsaurus/minecraft/chameleon/service/FabricRegistries.java:8)

### `FabricRender`: it implements `ChameleonRender`

- [ ] `FabricRender.createReplacementPart(2)`: its class isn't translated (it implements `ChameleonRender`) (com/texelsaurus/minecraft/chameleon/service/FabricRender.java:9)

### `WorldUtils`: another class has the same name, in a different package

- [ ] `WorldUtils.WorldUtils(0)`: its class isn't translated (another class has the same name, in a different package) (com/texelsaurus/minecraft/chameleon/util/WorldUtils.java:17)
- [ ] `WorldUtils.rayTraceEyes(3)`: it uses Minecraft's `BlockHitResult` (com/texelsaurus/minecraft/chameleon/util/WorldUtils.java:20)
- [ ] `WorldUtils.getBlockEntity(3)`: it's generic (com/texelsaurus/minecraft/chameleon/util/WorldUtils.java:29)
