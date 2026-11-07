# Java translated to Rust

Translated 237 of 5325 methods (3 more are done another way in Horizon) and 82 of 206 static fields to Rust (1 of those call the game, through Horizon's helpers); 0 of 6 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 3967 | `GuiClientTreeCheckBox.GuiClientTreeCheckBox(1)`: its class isn't translated (it's generic) |
| Other Java this translator doesn't handle yet | 994 | `Dist.isDedicatedServer(0)`: it has no body |
| Uses Minecraft classes (needs Horizon's game API) | 83 | `CreativeCoreClient.commands(2)`: it uses Minecraft's `CommandDispatcher` |
| Uses other libraries | 20 | `ConfigTypeConveration.getGenericType(1)`: it uses `Type` (java.lang.reflect) |
| Calls the game where Horizon has no helper yet | 7 | `GuiClientControl.getScreenRect(0)`: it calls Minecraft's Minecraft.getWindow (no Horizon helper for it yet) |
| Uses JOML maths (vectors, matrices) | 6 | `VectorUtils.set(3)`: it uses `Vector3d` (org.joml) |
| Uses Java collections other than List (Map, Set...) | 6 | `CreativeCoreClient.splitLine(3)`: it uses `Stream` (java.util.stream) |
| Done another way in Horizon | 3 | `ModMenuImpl.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |
| Shares one object between names (Java references) | 1 | `Matrix3.compute_svd(3)`: it changes an array or object through a second name (tmp) |
| Uses inheritance (extends, implements, instanceof) | 1 | `CreativeCoreClient.clientTick(1)`: `instanceof` |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| team/creative/creativecore/common/config/event/ConfigEventHandler.java:50 | `started` | not translated: it calls ConfigEventHandler.load, which isn't translated |
| team/creative/creativecore/CreativeFabricLoader.java:97 | `command` | not translated: its command tree: no `dispatcher.register(..)` the converter could read |
| team/creative/creativecore/CreativeFabricLoader.java:104 | `tick` | not translated: unknown name `run` |
| team/creative/creativecore/CreativeFabricLoader.java:109 | `hud` | not translated: a handler that isn't a lambda or a Class::method reference |
| team/creative/creativecore/CreativeFabricLoader.java:178 | `started` | not translated: unknown name `run` |
| team/creative/creativecore/CreativeCore.java:71 | `init` | not translated: it uses CreativeCore.GUI_CONTAINER, which isn't translated (it uses Minecraft's `MenuType`) |

## Every class


### `team/creative/creativecore/client/gui/control/menu/GuiClientMenu.java`: couldn't be read: expected `;` but found `>` (line 20)


### `team/creative/creativecore/common/util/filter/BiFilter.java`: couldn't be read: expected a name but found `...` (line 23)


### `team/creative/creativecore/common/util/filter/Filter.java`: couldn't be read: expected a name but found `...` (line 21)


### `Dist`: its constants carry values

- [ ] `Dist.isDedicatedServer(0)`: it has no body (net/neoforged/api/distmarker/Dist.java:27)
- [ ] `Dist.isClient(0)`: it has no body (net/neoforged/api/distmarker/Dist.java:29)

### `Mod`: it's an annotation

- [ ] `Mod.value(0)`: it has no body (net/neoforged/fml/common/Mod.java:9)
- [ ] `Mod.dist(0)`: it has no body (net/neoforged/fml/common/Mod.java:11)

### `ClientLoader`: it's an interface

- [ ] `ClientLoader.onInitializeClient(0)`: it has no body (team/creative/creativecore/client/ClientLoader.java:7)
- [ ] `ClientLoader.registerClientCommands(1)`: it's generic (team/creative/creativecore/client/ClientLoader.java:9)

### `GuiClientTreeCheckBox`: it's generic

- [ ] `GuiClientTreeCheckBox.GuiClientTreeCheckBox(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/config/gui/GuiClientTreeCheckBox.java:10)
- [ ] `GuiClientTreeCheckBox.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/config/gui/GuiClientTreeCheckBox.java:14)

### `CreativeCoreClient`: it implements Fabric's `ClientModInitializer`

- [ ] `CreativeCoreClient.LOADER`: it uses ICreativeClientLoader, which isn't translated (it's an interface) (team/creative/creativecore/client/CreativeCoreClient.java:54)
- [ ] `CreativeCoreClient.mc`: it uses Minecraft's `Minecraft` (team/creative/creativecore/client/CreativeCoreClient.java:55)
- [x] `CreativeCoreClient.MOD_CONFIGS` (team/creative/creativecore/client/CreativeCoreClient.java:56)
- [x] `CreativeCoreClient.getModConfigs(0)` (team/creative/creativecore/client/CreativeCoreClient.java:58)
- [x] `CreativeCoreClient.registerClientConfig(1)` (team/creative/creativecore/client/CreativeCoreClient.java:62)
- [ ] `CreativeCoreClient.getFrameTime(0)`: it uses CreativeCoreClient.mc, which isn't translated (it uses Minecraft's `Minecraft`) (team/creative/creativecore/client/CreativeCoreClient.java:66)
- [ ] `CreativeCoreClient.commands(2)`: it uses Minecraft's `CommandDispatcher` (team/creative/creativecore/client/CreativeCoreClient.java:70)
- [ ] `CreativeCoreClient.onInitializeClient(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/client/CreativeCoreClient.java:77)
- [ ] `CreativeCoreClient.clientTick(1)`: `instanceof` (team/creative/creativecore/client/CreativeCoreClient.java:105)
- [ ] `CreativeCoreClient.postBackgroundEvent(4)`: it uses Minecraft's `Screen` (team/creative/creativecore/client/CreativeCoreClient.java:111)
- [ ] `CreativeCoreClient.gatherTooltipComponents(7)`: it uses Minecraft's `ClientTooltipComponent` (team/creative/creativecore/client/CreativeCoreClient.java:114)
- [ ] `CreativeCoreClient.gatherTooltipComponentsFromElements(6)`: it uses Minecraft's `ClientTooltipComponent` (team/creative/creativecore/client/CreativeCoreClient.java:130)
- [ ] `CreativeCoreClient.splitLine(3)`: it uses `Stream` (java.util.stream) (team/creative/creativecore/client/CreativeCoreClient.java:169)
- [ ] `CreativeCoreClient.makeParticleRenderTypeComparator(1)`: it uses `Comparator` (java.util) (team/creative/creativecore/client/CreativeCoreClient.java:175)
- [ ] `CreativeCoreClient.loader(0)`: it uses ICreativeClientLoader, which isn't translated (it's an interface) (team/creative/creativecore/client/CreativeCoreClient.java:190)

### `CreativeFabricClientLoader`: it implements `ICreativeClientLoader`

- [ ] `CreativeFabricClientLoader.registerKeybind(1)`: its class isn't translated (it implements `ICreativeClientLoader`) (team/creative/creativecore/client/CreativeFabricClientLoader.java:21)
- [ ] `CreativeFabricClientLoader.registerCategory(1)`: its class isn't translated (it implements `ICreativeClientLoader`) (team/creative/creativecore/client/CreativeFabricClientLoader.java:26)
- [ ] `CreativeFabricClientLoader.registerModifyTooltip(1)`: its class isn't translated (it implements `ICreativeClientLoader`) (team/creative/creativecore/client/CreativeFabricClientLoader.java:31)
- [ ] `CreativeFabricClientLoader.registerMenu(2)`: it's generic (team/creative/creativecore/client/CreativeFabricClientLoader.java:36)
- [ ] `CreativeFabricClientLoader.addScreenWidget(2)`: it's generic (team/creative/creativecore/client/CreativeFabricClientLoader.java:43)

### `GuiClientCheckList`: it's generic

- [ ] `GuiClientCheckList.GuiClientCheckList(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientCheckList.java:10)
- [ ] `GuiClientCheckList.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientCheckList.java:14)
- [ ] `GuiClientCheckList.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientCheckList.java:20)
- [ ] `GuiClientCheckList.reflowInternal(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientCheckList.java:26)

### `GuiClientComboBox`: it's generic

- [ ] `GuiClientComboBox.GuiClientComboBox(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:27)
- [ ] `GuiClientComboBox.hasSearchbar(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:31)
- [ ] `GuiClientComboBox.updateDisplay(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:36)
- [ ] `GuiClientComboBox.setSearchbar(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:44)
- [ ] `GuiClientComboBox.setDirection(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:49)
- [ ] `GuiClientComboBox.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:54)
- [ ] `GuiClientComboBox.selected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:69)
- [ ] `GuiClientComboBox.selected(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:75)
- [ ] `GuiClientComboBox.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:81)
- [ ] `GuiClientComboBox.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:96)
- [ ] `GuiClientComboBox.indexOf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:101)
- [ ] `GuiClientComboBox.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:112)
- [ ] `GuiClientComboBox.previous(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:122)
- [ ] `GuiClientComboBox.selectedIndex(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:132)
- [ ] `GuiClientComboBox.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:137)
- [ ] `GuiClientComboBox.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:144)
- [ ] `GuiClientComboBox.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:151)
- [ ] `GuiClientComboBox.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:162)
- [ ] `GuiClientComboBox.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:173)
- [ ] `GuiClientComboBox.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:180)
- [ ] `GuiClientComboBox.createBox(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:185)
- [ ] `GuiClientComboBox.looseFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:191)
- [ ] `GuiClientComboBox.lines(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBox.java:198)

### `GuiClientComboBoxExtension`: it's generic

- [ ] `GuiClientComboBoxExtension.GuiClientComboBoxExtension(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxExtension.java:17)
- [ ] `GuiClientComboBoxExtension.init(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxExtension.java:21)
- [ ] `GuiClientComboBoxExtension.creator(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxExtension.java:32)
- [ ] `GuiClientComboBoxExtension.looseFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxExtension.java:36)
- [ ] `GuiClientComboBoxExtension.reloadControls(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxExtension.java:41)
- [ ] `GuiClientComboBoxExtension.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxExtension.java:83)
- [ ] `GuiClientComboBoxExtension.maxHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxExtension.java:93)

### `GuiClientComboBoxFlexible`: it's generic

- [ ] `GuiClientComboBoxFlexible.GuiClientComboBoxFlexible(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxFlexible.java:15)
- [ ] `GuiClientComboBoxFlexible.init(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxFlexible.java:19)
- [ ] `GuiClientComboBoxFlexible.updateDisplay(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxFlexible.java:24)
- [ ] `GuiClientComboBoxFlexible.forceSelect(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxFlexible.java:34)
- [ ] `GuiClientComboBoxFlexible.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxFlexible.java:48)
- [ ] `GuiClientComboBoxFlexible.selected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxFlexible.java:54)

### `GuiClientComboBoxTree`: it's generic

- [ ] `GuiClientComboBoxTree.GuiClientComboBoxTree(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:27)
- [ ] `GuiClientComboBoxTree.init(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:31)
- [ ] `GuiClientComboBoxTree.hasSearchbar(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:36)
- [ ] `GuiClientComboBoxTree.setSearchbar(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:41)
- [ ] `GuiClientComboBoxTree.setDirection(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:46)
- [ ] `GuiClientComboBoxTree.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:51)
- [ ] `GuiClientComboBoxTree.selected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:58)
- [ ] `GuiClientComboBoxTree.selected(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:64)
- [ ] `GuiClientComboBoxTree.select(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:70)
- [ ] `GuiClientComboBoxTree.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:80)
- [ ] `GuiClientComboBoxTree.updateDisplay(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:90)
- [ ] `GuiClientComboBoxTree.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:98)
- [ ] `GuiClientComboBoxTree.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:105)
- [ ] `GuiClientComboBoxTree.createBox(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:110)
- [ ] `GuiClientComboBoxTree.looseFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientComboBoxTree.java:116)

### `GuiClientListBoxBase`: it's generic

- [ ] `GuiClientListBoxBase.GuiClientListBoxBase(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientListBoxBase.java:7)
- [ ] `GuiClientListBoxBase.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientListBoxBase.java:11)
- [ ] `GuiClientListBoxBase.reflowInternal(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientListBoxBase.java:17)

### `GuiClientStackSelector`: it's generic

- [ ] `GuiClientStackSelector.GuiClientStackSelector(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:22)
- [ ] `GuiClientStackSelector.setCollector(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:26)
- [ ] `GuiClientStackSelector.hasSearchbar(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:31)
- [ ] `GuiClientStackSelector.setSearchbar(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:36)
- [ ] `GuiClientStackSelector.selectFirst(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:41)
- [ ] `GuiClientStackSelector.updateCollectedStacks(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:54)
- [ ] `GuiClientStackSelector.setSelectedForce(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:62)
- [ ] `GuiClientStackSelector.setSelected(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:73)
- [ ] `GuiClientStackSelector.getStacks(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:88)
- [ ] `GuiClientStackSelector.getSelected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:93)
- [ ] `GuiClientStackSelector.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:98)
- [ ] `GuiClientStackSelector.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:105)
- [ ] `GuiClientStackSelector.createBox(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:110)
- [ ] `GuiClientStackSelector.looseFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelector.java:116)

### `GuiClientStackSelectorExtension`: it's generic

- [ ] `GuiClientStackSelectorExtension.GuiClientStackSelectorExtension(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelectorExtension.java:27)
- [ ] `GuiClientStackSelectorExtension.init(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelectorExtension.java:44)
- [ ] `GuiClientStackSelectorExtension.creator(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelectorExtension.java:49)
- [ ] `GuiClientStackSelectorExtension.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelectorExtension.java:53)
- [ ] `GuiClientStackSelectorExtension.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelectorExtension.java:60)
- [ ] `GuiClientStackSelectorExtension.reflowInternal(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelectorExtension.java:66)
- [ ] `GuiClientStackSelectorExtension.reloadControls(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelectorExtension.java:72)
- [ ] `GuiClientStackSelectorExtension.looseFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/collection/GuiClientStackSelectorExtension.java:118)

### `GuiFocusControl`: it's generic

- [ ] `GuiFocusControl.GuiFocusControl(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/GuiFocusControl.java:10)
- [ ] `GuiFocusControl.isFocused(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/GuiFocusControl.java:14)
- [ ] `GuiFocusControl.focus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/GuiFocusControl.java:19)
- [ ] `GuiFocusControl.looseFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/GuiFocusControl.java:25)
- [ ] `GuiFocusControl.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/GuiFocusControl.java:31)
- [ ] `GuiFocusControl.focusChanged(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/GuiFocusControl.java:37)

### `GuiClientInventoryGrid`: it's generic

- [ ] `GuiClientInventoryGrid.GuiClientInventoryGrid(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGrid.java:15)
- [ ] `GuiClientInventoryGrid.setGridDim(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGrid.java:19)
- [ ] `GuiClientInventoryGrid.minWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGrid.java:25)
- [ ] `GuiClientInventoryGrid.minHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGrid.java:30)
- [ ] `GuiClientInventoryGrid.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGrid.java:35)
- [ ] `GuiClientInventoryGrid.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGrid.java:53)
- [ ] `GuiClientInventoryGrid.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGrid.java:77)
- [ ] `GuiClientInventoryGrid.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGrid.java:82)
- [ ] `GuiClientInventoryGrid.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGrid.java:87)

### `GuiClientInventoryGridPreview`: it's generic

- [ ] `GuiClientInventoryGridPreview.GuiClientInventoryGridPreview(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGridPreview.java:17)
- [ ] `GuiClientInventoryGridPreview.setGridDim(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGridPreview.java:21)
- [ ] `GuiClientInventoryGridPreview.minWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGridPreview.java:27)
- [ ] `GuiClientInventoryGridPreview.minHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGridPreview.java:32)
- [ ] `GuiClientInventoryGridPreview.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGridPreview.java:37)
- [ ] `GuiClientInventoryGridPreview.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGridPreview.java:55)
- [ ] `GuiClientInventoryGridPreview.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGridPreview.java:79)
- [ ] `GuiClientInventoryGridPreview.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGridPreview.java:84)
- [ ] `GuiClientInventoryGridPreview.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientInventoryGridPreview.java:89)

### `GuiClientSlot`: it's generic

- [ ] `GuiClientSlot.GuiClientSlot(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlot.java:19)
- [ ] `GuiClientSlot.itemManager(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlot.java:23)
- [ ] `GuiClientSlot.inventory(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlot.java:27)
- [ ] `GuiClientSlot.changed(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlot.java:31)
- [ ] `GuiClientSlot.getStackToRender(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlot.java:38)
- [ ] `GuiClientSlot.keyPressed(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlot.java:52)
- [ ] `GuiClientSlot.mouseScrolled(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlot.java:62)
- [ ] `GuiClientSlot.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlot.java:77)
- [ ] `GuiClientSlot.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlot.java:112)
- [ ] `GuiClientSlot.slot(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlot.java:119)

### `GuiClientSlotBase`: it's generic

- [x] `GuiClientSlotBase.SLOT_SIZE` (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:19)
- [ ] `GuiClientSlotBase.HOVER`: it uses DisplayColor, which isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:20)
- [ ] `GuiClientSlotBase.GuiClientSlotBase(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:23)
- [ ] `GuiClientSlotBase.getStackToRender(0)`: it has no body (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:27)
- [ ] `GuiClientSlotBase.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:29)
- [ ] `GuiClientSlotBase.getTooltip(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:34)
- [ ] `GuiClientSlotBase.hasTooltipExtraSpaceAfterFirstLine(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:41)
- [ ] `GuiClientSlotBase.isHovered(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:46)
- [ ] `GuiClientSlotBase.renderContent(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:50)
- [ ] `GuiClientSlotBase.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:66)
- [ ] `GuiClientSlotBase.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:70)
- [ ] `GuiClientSlotBase.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:75)
- [ ] `GuiClientSlotBase.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:79)
- [ ] `GuiClientSlotBase.maxWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:83)
- [ ] `GuiClientSlotBase.maxHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:88)
- [ ] `GuiClientSlotBase.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:93)
- [ ] `GuiClientSlotBase.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:98)
- [ ] `GuiClientSlotBase.minWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:103)
- [ ] `GuiClientSlotBase.minHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotBase.java:108)

### `GuiClientSlotViewer`: it's generic

- [ ] `GuiClientSlotViewer.GuiClientSlotViewer(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotViewer.java:8)
- [ ] `GuiClientSlotViewer.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotViewer.java:12)
- [ ] `GuiClientSlotViewer.getStackToRender(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/inventory/GuiClientSlotViewer.java:17)

### `GuiClientMenuRoot`: it's generic

- [ ] `GuiClientMenuRoot.GuiClientMenuRoot(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuRoot.java:17)
- [ ] `GuiClientMenuRoot.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuRoot.java:21)
- [ ] `GuiClientMenuRoot.isRoot(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuRoot.java:27)
- [ ] `GuiClientMenuRoot.root(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuRoot.java:32)
- [ ] `GuiClientMenuRoot.parentCreator(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuRoot.java:37)
- [ ] `GuiClientMenuRoot.translate(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuRoot.java:42)
- [ ] `GuiClientMenuRoot.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuRoot.java:52)

### `GuiClientMenuSub`: it's generic

- [ ] `GuiClientMenuSub.GuiClientMenuSub(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuSub.java:13)
- [ ] `GuiClientMenuSub.isRoot(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuSub.java:17)
- [ ] `GuiClientMenuSub.setRoot(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuSub.java:22)
- [ ] `GuiClientMenuSub.root(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuSub.java:27)
- [ ] `GuiClientMenuSub.parentCreator(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/menu/GuiClientMenuSub.java:32)

### `GuiClientPanel`: it's generic

- [ ] `GuiClientPanel.GuiClientPanel(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientPanel.java:8)
- [ ] `GuiClientPanel.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientPanel.java:12)

### `GuiClientScrollX`: it's generic

- [ ] `GuiClientScrollX.GuiClientScrollX(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:23)
- [ ] `GuiClientScrollX.setHovered(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:27)
- [ ] `GuiClientScrollX.setHover(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:32)
- [ ] `GuiClientScrollX.getOffsetX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:37)
- [ ] `GuiClientScrollX.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:42)
- [ ] `GuiClientScrollX.onScrolled(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:47)
- [ ] `GuiClientScrollX.mouseScrolled(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:57)
- [ ] `GuiClientScrollX.scroll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:67)
- [ ] `GuiClientScrollX.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:72)
- [ ] `GuiClientScrollX.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:83)
- [ ] `GuiClientScrollX.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:102)
- [ ] `GuiClientScrollX.needsScrollbar(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:108)
- [ ] `GuiClientScrollX.renderContent(8)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:112)
- [ ] `GuiClientScrollX.minWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:152)
- [ ] `GuiClientScrollX.minHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:157)
- [ ] `GuiClientScrollX.endlessX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:162)
- [ ] `GuiClientScrollX.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:167)
- [ ] `GuiClientScrollX.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollX.java:173)

### `GuiClientScrollXY`: it's generic

- [ ] `GuiClientScrollXY.GuiClientScrollXY(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:29)
- [ ] `GuiClientScrollXY.getOffsetX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:33)
- [ ] `GuiClientScrollXY.getOffsetY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:38)
- [ ] `GuiClientScrollXY.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:43)
- [ ] `GuiClientScrollXY.onScrolledX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:48)
- [ ] `GuiClientScrollXY.onScrolledY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:58)
- [ ] `GuiClientScrollXY.mouseScrolled(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:68)
- [ ] `GuiClientScrollXY.scroll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:78)
- [ ] `GuiClientScrollXY.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:109)
- [ ] `GuiClientScrollXY.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:124)
- [ ] `GuiClientScrollXY.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:157)
- [ ] `GuiClientScrollXY.needsScrollbarX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:163)
- [ ] `GuiClientScrollXY.needsScrollbarY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:167)
- [ ] `GuiClientScrollXY.renderContent(8)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:171)
- [ ] `GuiClientScrollXY.minWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:235)
- [ ] `GuiClientScrollXY.minHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:240)
- [ ] `GuiClientScrollXY.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:245)
- [ ] `GuiClientScrollXY.updateWidth(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:251)
- [ ] `GuiClientScrollXY.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:261)
- [ ] `GuiClientScrollXY.updateHeight(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollXY.java:267)

### `GuiClientScrollY`: it's generic

- [ ] `GuiClientScrollY.GuiClientScrollY(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:25)
- [ ] `GuiClientScrollY.setScrollWhenCTRL(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:29)
- [ ] `GuiClientScrollY.setHovered(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:34)
- [ ] `GuiClientScrollY.setHover(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:39)
- [ ] `GuiClientScrollY.getOffsetY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:44)
- [ ] `GuiClientScrollY.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:49)
- [ ] `GuiClientScrollY.onScrolled(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:54)
- [ ] `GuiClientScrollY.mouseScrolled(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:64)
- [ ] `GuiClientScrollY.scroll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:74)
- [ ] `GuiClientScrollY.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:81)
- [ ] `GuiClientScrollY.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:92)
- [ ] `GuiClientScrollY.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:111)
- [ ] `GuiClientScrollY.needsScrollbar(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:117)
- [ ] `GuiClientScrollY.renderContent(8)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:121)
- [ ] `GuiClientScrollY.minWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:161)
- [ ] `GuiClientScrollY.minHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:166)
- [ ] `GuiClientScrollY.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:171)
- [ ] `GuiClientScrollY.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:180)
- [ ] `GuiClientScrollY.endlessY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientScrollY.java:186)

### `GuiClientTable`: it's generic

- [ ] `GuiClientTable.GuiClientTable(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:15)
- [ ] `GuiClientTable.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:19)

### `GuiClientTable.GuiTableGroup`: it extends `GuiControlRect` (team.creative.creativecore.client.gui)

- [ ] `GuiClientTable.GuiTableGroup.GuiTableGroup(0)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:54)
- [ ] `GuiClientTable.GuiTableGroup.getMinWidth(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:58)
- [ ] `GuiClientTable.GuiTableGroup.getMaxWidth(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:72)
- [ ] `GuiClientTable.GuiTableGroup.getPreferredWidth(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:86)
- [ ] `GuiClientTable.GuiTableGroup.getMinHeight(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:97)
- [ ] `GuiClientTable.GuiTableGroup.getMaxHeight(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:111)
- [ ] `GuiClientTable.GuiTableGroup.getPreferredHeight(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:125)
- [ ] `GuiClientTable.GuiTableGroup.setWidth(2)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:136)
- [ ] `GuiClientTable.GuiTableGroup.setHeight(2)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:147)
- [ ] `GuiClientTable.GuiTableGroup.setX(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:158)
- [ ] `GuiClientTable.GuiTableGroup.setY(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:167)
- [ ] `GuiClientTable.GuiTableGroup.flowX(0)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:176)
- [ ] `GuiClientTable.GuiTableGroup.flowY(0)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:183)
- [ ] `GuiClientTable.GuiTableGroup.isExpandableX(0)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:190)
- [ ] `GuiClientTable.GuiTableGroup.isExpandableY(0)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTable.java:201)

### `GuiClientTableScrollable`: it's generic

- [ ] `GuiClientTableScrollable.GuiClientTableScrollable(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:18)
- [ ] `GuiClientTableScrollable.createScrollBox(1)`: its body couldn't be read: `var` (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:22)
- [ ] `GuiClientTableScrollable.createTopRow(1)`: its body couldn't be read: an anonymous class (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:46)
- [ ] `GuiClientTableScrollable.createFirstCol(1)`: its body couldn't be read: an anonymous class (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:61)
- [ ] `GuiClientTableScrollable.createCols(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:76)
- [ ] `GuiClientTableScrollable.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:111)
- [ ] `GuiClientTableScrollable.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:116)
- [ ] `GuiClientTableScrollable.createRows(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:152)
- [ ] `GuiClientTableScrollable.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:180)
- [ ] `GuiClientTableScrollable.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:185)
- [ ] `GuiClientTableScrollable.endlessX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:211)
- [ ] `GuiClientTableScrollable.endlessY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTableScrollable.java:216)

### `GuiClientTabs`: it's generic

- [ ] `GuiClientTabs.GuiClientTabs(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:21)
- [ ] `GuiClientTabs.init(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:25)
- [ ] `GuiClientTabs.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:31)
- [ ] `GuiClientTabs.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:37)
- [ ] `GuiClientTabs.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:48)
- [ ] `GuiClientTabs.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:53)
- [ ] `GuiClientTabs.index(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:80)
- [ ] `GuiClientTabs.createSpecialParent(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:85)

### `GuiClientTabs.GuiClientTabSpecialParent`: it extends `GuiClientParent` (team.creative.creativecore.client.gui)

- [ ] `GuiClientTabs.GuiClientTabSpecialParent.GuiClientTabSpecialParent(1)`: its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:91)
- [ ] `GuiClientTabs.GuiClientTabSpecialParent.getControlFormatting(0)`: its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:95)
- [ ] `GuiClientTabs.GuiClientTabSpecialParent.preferredWidth(1)`: its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:100)
- [ ] `GuiClientTabs.GuiClientTabSpecialParent.preferredWidthOriginal(1)`: its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:111)
- [ ] `GuiClientTabs.GuiClientTabSpecialParent.preferredHeight(2)`: its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:115)
- [ ] `GuiClientTabs.GuiClientTabSpecialParent.preferredHeightOriginal(2)`: its class isn't translated (it extends `GuiClientParent` (team.creative.creativecore.client.gui)) (team/creative/creativecore/client/gui/control/parent/GuiClientTabs.java:126)

### `GuiClientArraySlider`: it's generic

- [ ] `GuiClientArraySlider.GuiClientArraySlider(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientArraySlider.java:9)
- [ ] `GuiClientArraySlider.getTextByValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientArraySlider.java:13)
- [ ] `GuiClientArraySlider.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientArraySlider.java:18)
- [ ] `GuiClientArraySlider.setValues(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientArraySlider.java:23)
- [ ] `GuiClientArraySlider.get(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientArraySlider.java:31)

### `GuiClientButton`: it's generic

- [ ] `GuiClientButton.GuiClientButton(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButton.java:14)
- [ ] `GuiClientButton.setHoverEffect(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButton.java:18)
- [ ] `GuiClientButton.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButton.java:23)
- [ ] `GuiClientButton.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButton.java:36)
- [ ] `GuiClientButton.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButton.java:46)
- [ ] `GuiClientButton.setPressed(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButton.java:51)

### `GuiClientButtonContext`: it's generic

- [ ] `GuiClientButtonContext.GuiClientButtonContext(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonContext.java:24)
- [ ] `GuiClientButtonContext.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonContext.java:28)
- [ ] `GuiClientButtonContext.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonContext.java:36)
- [ ] `GuiClientButtonContext.createBox(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonContext.java:44)

### `GuiClientButtonContext.GuiButtonContextMenu`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiClientButtonContext.GuiButtonContextMenu.GuiButtonContextMenu(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonContext.java:60)

### `GuiClientButtonContext.GuiClientButtonContextMenu`: it's generic

- [ ] `GuiClientButtonContext.GuiClientButtonContextMenu.GuiClientButtonContextMenu(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonContext.java:68)
- [ ] `GuiClientButtonContext.GuiClientButtonContextMenu.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonContext.java:72)
- [ ] `GuiClientButtonContext.GuiClientButtonContextMenu.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonContext.java:80)

### `GuiClientButtonHold`: it's generic

- [x] `GuiClientButtonHold.INITIAL_WAIT` (team/creative/creativecore/client/gui/control/simple/GuiClientButtonHold.java:7)
- [x] `GuiClientButtonHold.CONTINOUS_WAIT` (team/creative/creativecore/client/gui/control/simple/GuiClientButtonHold.java:8)
- [ ] `GuiClientButtonHold.GuiClientButtonHold(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonHold.java:13)
- [ ] `GuiClientButtonHold.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonHold.java:17)
- [ ] `GuiClientButtonHold.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonHold.java:25)
- [ ] `GuiClientButtonHold.tick(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonHold.java:31)
- [ ] `GuiClientButtonHold.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonHold.java:47)

### `GuiClientButtonIcon`: it's generic

- [ ] `GuiClientButtonIcon.GuiClientButtonIcon(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonIcon.java:14)
- [ ] `GuiClientButtonIcon.getBackground(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonIcon.java:18)
- [ ] `GuiClientButtonIcon.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonIcon.java:23)
- [ ] `GuiClientButtonIcon.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonIcon.java:28)
- [ ] `GuiClientButtonIcon.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonIcon.java:33)
- [ ] `GuiClientButtonIcon.setPressed(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientButtonIcon.java:43)

### `GuiClientCheckBox`: it's generic

- [x] `GuiClientCheckBox.CHECKBOX_WIDTH` (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:17)
- [ ] `GuiClientCheckBox.PARTIAL_STYLE`: it creates a DisplayColor, which isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:18)
- [ ] `GuiClientCheckBox.GuiClientCheckBox(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:23)
- [ ] `GuiClientCheckBox.setPartial(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:27)
- [ ] `GuiClientCheckBox.getPartial(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:39)
- [ ] `GuiClientCheckBox.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:44)
- [ ] `GuiClientCheckBox.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:49)
- [ ] `GuiClientCheckBox.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:54)
- [ ] `GuiClientCheckBox.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:59)
- [ ] `GuiClientCheckBox.consumeChanged(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:84)
- [ ] `GuiClientCheckBox.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:89)
- [ ] `GuiClientCheckBox.get(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:103)
- [ ] `GuiClientCheckBox.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckBox.java:108)

### `GuiClientCheckButtonIcon`: it's generic

- [ ] `GuiClientCheckButtonIcon.GuiClientCheckButtonIcon(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckButtonIcon.java:13)
- [ ] `GuiClientCheckButtonIcon.setOnIcon(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckButtonIcon.java:17)
- [ ] `GuiClientCheckButtonIcon.setOffIcon(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckButtonIcon.java:22)
- [ ] `GuiClientCheckButtonIcon.getState(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckButtonIcon.java:27)
- [ ] `GuiClientCheckButtonIcon.setState(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckButtonIcon.java:32)
- [ ] `GuiClientCheckButtonIcon.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientCheckButtonIcon.java:41)

### `GuiClientColoredSteppedSlider`: it's generic

- [ ] `GuiClientColoredSteppedSlider.GuiClientColoredSteppedSlider(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColoredSteppedSlider.java:18)
- [ ] `GuiClientColoredSteppedSlider.setPicker(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColoredSteppedSlider.java:22)
- [ ] `GuiClientColoredSteppedSlider.setPart(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColoredSteppedSlider.java:27)
- [ ] `GuiClientColoredSteppedSlider.setValue(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColoredSteppedSlider.java:32)
- [ ] `GuiClientColoredSteppedSlider.setValueSilent(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColoredSteppedSlider.java:41)
- [ ] `GuiClientColoredSteppedSlider.renderContent(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColoredSteppedSlider.java:49)

### `GuiClientColorPlate`: it's generic

- [ ] `GuiClientColorPlate.GuiClientColorPlate(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColorPlate.java:15)
- [ ] `GuiClientColorPlate.setColor(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColorPlate.java:19)
- [ ] `GuiClientColorPlate.getColor(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColorPlate.java:28)
- [ ] `GuiClientColorPlate.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColorPlate.java:33)
- [ ] `GuiClientColorPlate.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColorPlate.java:38)
- [ ] `GuiClientColorPlate.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColorPlate.java:45)
- [ ] `GuiClientColorPlate.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColorPlate.java:49)
- [ ] `GuiClientColorPlate.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColorPlate.java:53)
- [ ] `GuiClientColorPlate.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientColorPlate.java:58)

### `GuiClientHexButton`: it extends `GuiClientButton`

- [ ] `GuiClientHexButton.GuiClientHexButton(1)`: its class isn't translated (it extends `GuiClientButton`) (team/creative/creativecore/client/gui/control/simple/GuiClientHexButton.java:16)
- [ ] `GuiClientHexButton.setConsumer(2)`: its class isn't translated (it extends `GuiClientButton`) (team/creative/creativecore/client/gui/control/simple/GuiClientHexButton.java:20)
- [ ] `GuiClientHexButton.colorChanged(0)`: its class isn't translated (it extends `GuiClientButton`) (team/creative/creativecore/client/gui/control/simple/GuiClientHexButton.java:27)
- [ ] `GuiClientHexButton.createBox(1)`: its class isn't translated (it extends `GuiClientButton`) (team/creative/creativecore/client/gui/control/simple/GuiClientHexButton.java:35)

### `GuiClientIcon`: it's generic

- [ ] `GuiClientIcon.GuiClientIcon(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:21)
- [ ] `GuiClientIcon.setIcon(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:25)
- [ ] `GuiClientIcon.setColor(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:30)
- [ ] `GuiClientIcon.setShadow(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:35)
- [ ] `GuiClientIcon.setSquared(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:40)
- [ ] `GuiClientIcon.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:45)
- [ ] `GuiClientIcon.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:49)
- [ ] `GuiClientIcon.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:53)
- [ ] `GuiClientIcon.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:58)
- [ ] `GuiClientIcon.getBackground(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:63)
- [ ] `GuiClientIcon.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:68)
- [ ] `GuiClientIcon.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientIcon.java:122)

### `GuiClientLabel`: it's generic

- [ ] `GuiClientLabel.GuiClientLabel(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:16)
- [ ] `GuiClientLabel.setDefaultColor(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:20)
- [ ] `GuiClientLabel.setDropShadow(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:25)
- [ ] `GuiClientLabel.setAlign(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:30)
- [ ] `GuiClientLabel.setVAlign(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:35)
- [ ] `GuiClientLabel.setTitle(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:40)
- [ ] `GuiClientLabel.setTitle(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:45)
- [ ] `GuiClientLabel.setText(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:50)
- [ ] `GuiClientLabel.setScale(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:54)
- [ ] `GuiClientLabel.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:59)
- [ ] `GuiClientLabel.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:64)
- [ ] `GuiClientLabel.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:69)
- [ ] `GuiClientLabel.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:74)
- [ ] `GuiClientLabel.minWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:79)
- [ ] `GuiClientLabel.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:84)
- [ ] `GuiClientLabel.minHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:89)
- [ ] `GuiClientLabel.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientLabel.java:94)

### `GuiClientLabelHover`: it extends `GuiClientLabel`

- [ ] `GuiClientLabelHover.GuiClientLabelHover(1)`: its class isn't translated (it extends `GuiClientLabel`) (team/creative/creativecore/client/gui/control/simple/GuiClientLabelHover.java:11)
- [ ] `GuiClientLabelHover.renderContent(6)`: its class isn't translated (it extends `GuiClientLabel`) (team/creative/creativecore/client/gui/control/simple/GuiClientLabelHover.java:15)

### `GuiClientListEntry`: it's generic

- [ ] `GuiClientListEntry.GuiClientListEntry(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientListEntry.java:11)
- [ ] `GuiClientListEntry.renderContent(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientListEntry.java:15)
- [ ] `GuiClientListEntry.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientListEntry.java:29)

### `GuiClientProgressbar`: it's generic

- [ ] `GuiClientProgressbar.GuiClientProgressbar(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientProgressbar.java:13)
- [ ] `GuiClientProgressbar.getTooltip(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientProgressbar.java:17)
- [ ] `GuiClientProgressbar.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientProgressbar.java:31)
- [ ] `GuiClientProgressbar.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientProgressbar.java:36)
- [ ] `GuiClientProgressbar.renderProgress(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientProgressbar.java:49)
- [ ] `GuiClientProgressbar.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientProgressbar.java:53)
- [ ] `GuiClientProgressbar.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientProgressbar.java:57)
- [ ] `GuiClientProgressbar.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientProgressbar.java:61)
- [ ] `GuiClientProgressbar.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientProgressbar.java:66)

### `GuiClientSeekBar`: it's generic

- [ ] `GuiClientSeekBar.GuiClientSeekBar(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:25)
- [ ] `GuiClientSeekBar.init(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:29)
- [ ] `GuiClientSeekBar.setOnTimeUpdate(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:36)
- [ ] `GuiClientSeekBar.setOnLastTimeUpdate(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:41)
- [ ] `GuiClientSeekBar.setPosition(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:46)
- [ ] `GuiClientSeekBar.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:59)
- [ ] `GuiClientSeekBar.renderProgress(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:69)
- [ ] `GuiClientSeekBar.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:73)
- [ ] `GuiClientSeekBar.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:85)
- [ ] `GuiClientSeekBar.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:103)
- [ ] `GuiClientSeekBar.tick(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:111)
- [ ] `GuiClientSeekBar.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:119)
- [ ] `GuiClientSeekBar.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:124)
- [ ] `GuiClientSeekBar.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:128)
- [ ] `GuiClientSeekBar.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:132)
- [ ] `GuiClientSeekBar.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSeekBar.java:137)

### `GuiClientShowItem`: it's generic

- [ ] `GuiClientShowItem.GuiClientShowItem(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientShowItem.java:10)
- [ ] `GuiClientShowItem.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientShowItem.java:14)
- [ ] `GuiClientShowItem.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientShowItem.java:18)
- [ ] `GuiClientShowItem.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientShowItem.java:22)
- [ ] `GuiClientShowItem.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientShowItem.java:27)
- [ ] `GuiClientShowItem.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientShowItem.java:32)
- [ ] `GuiClientShowItem.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientShowItem.java:37)

### `GuiClientSlider`: it's generic

- [ ] `GuiClientSlider.GuiClientSlider(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:30)
- [ ] `GuiClientSlider.getTextByValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:34)
- [ ] `GuiClientSlider.getTextfieldValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:39)
- [ ] `GuiClientSlider.getPercentage(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:44)
- [ ] `GuiClientSlider.setParser(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:49)
- [ ] `GuiClientSlider.setSliderSize(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:54)
- [ ] `GuiClientSlider.setMinSlider(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:59)
- [ ] `GuiClientSlider.setMaxSlider(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:68)
- [ ] `GuiClientSlider.textfield(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:77)
- [ ] `GuiClientSlider.setEnabled(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:81)
- [ ] `GuiClientSlider.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:90)
- [ ] `GuiClientSlider.createTextfield(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:120)
- [ ] `GuiClientSlider.closeTextField(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:126)
- [ ] `GuiClientSlider.keyPressed(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:140)
- [ ] `GuiClientSlider.charTyped(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:154)
- [ ] `GuiClientSlider.setBounds(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:159)
- [ ] `GuiClientSlider.setMaxValue(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:166)
- [ ] `GuiClientSlider.setMinValue(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:174)
- [ ] `GuiClientSlider.setValue(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:182)
- [ ] `GuiClientSlider.getValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:200)
- [ ] `GuiClientSlider.getMinValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:205)
- [ ] `GuiClientSlider.getMaxValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:210)
- [ ] `GuiClientSlider.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:215)
- [ ] `GuiClientSlider.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:220)
- [ ] `GuiClientSlider.looseFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:237)
- [ ] `GuiClientSlider.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:246)
- [ ] `GuiClientSlider.renderContent(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:253)
- [ ] `GuiClientSlider.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:266)
- [ ] `GuiClientSlider.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:270)
- [ ] `GuiClientSlider.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:277)
- [ ] `GuiClientSlider.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:284)
- [ ] `GuiClientSlider.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSlider.java:289)

### `GuiClientStateButton`: it's generic

- [ ] `GuiClientStateButton.GuiClientStateButton(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:19)
- [ ] `GuiClientStateButton.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:23)
- [ ] `GuiClientStateButton.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:38)
- [ ] `GuiClientStateButton.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:45)
- [ ] `GuiClientStateButton.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:52)
- [ ] `GuiClientStateButton.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:63)
- [ ] `GuiClientStateButton.selected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:74)
- [ ] `GuiClientStateButton.selected(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:80)
- [ ] `GuiClientStateButton.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:86)
- [ ] `GuiClientStateButton.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:101)
- [ ] `GuiClientStateButton.indexOf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:106)
- [ ] `GuiClientStateButton.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:117)
- [ ] `GuiClientStateButton.previous(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:127)
- [ ] `GuiClientStateButton.updateDisplay(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:137)
- [ ] `GuiClientStateButton.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButton.java:145)

### `GuiClientStateButtonIcon`: it's generic

- [ ] `GuiClientStateButtonIcon.GuiClientStateButtonIcon(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButtonIcon.java:12)
- [ ] `GuiClientStateButtonIcon.getState(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButtonIcon.java:16)
- [ ] `GuiClientStateButtonIcon.setIcons(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButtonIcon.java:21)
- [ ] `GuiClientStateButtonIcon.setState(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButtonIcon.java:27)
- [ ] `GuiClientStateButtonIcon.previousState(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButtonIcon.java:36)
- [ ] `GuiClientStateButtonIcon.nextState(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButtonIcon.java:50)
- [ ] `GuiClientStateButtonIcon.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientStateButtonIcon.java:64)

### `GuiClientSteppedSlider`: it's generic

- [ ] `GuiClientSteppedSlider.GuiClientSteppedSlider(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSteppedSlider.java:11)
- [ ] `GuiClientSteppedSlider.setSteppedParser(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSteppedSlider.java:15)
- [ ] `GuiClientSteppedSlider.mouseScrolled(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSteppedSlider.java:20)
- [ ] `GuiClientSteppedSlider.getTextByValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSteppedSlider.java:26)
- [ ] `GuiClientSteppedSlider.getTextfieldValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSteppedSlider.java:31)
- [ ] `GuiClientSteppedSlider.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSteppedSlider.java:36)
- [ ] `GuiClientSteppedSlider.createTextfield(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSteppedSlider.java:42)
- [ ] `GuiClientSteppedSlider.getIntValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSteppedSlider.java:47)
- [ ] `GuiClientSteppedSlider.getIntMaxValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSteppedSlider.java:52)
- [ ] `GuiClientSteppedSlider.getIntMinValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientSteppedSlider.java:57)

### `GuiClientTabButton`: it's generic

- [ ] `GuiClientTabButton.GuiClientTabButton(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTabButton.java:23)
- [ ] `GuiClientTabButton.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTabButton.java:27)
- [ ] `GuiClientTabButton.selected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTabButton.java:47)
- [ ] `GuiClientTabButton.selected(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTabButton.java:53)
- [ ] `GuiClientTabButton.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTabButton.java:59)
- [ ] `GuiClientTabButton.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTabButton.java:78)
- [ ] `GuiClientTabButton.indexOf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTabButton.java:83)
- [ ] `GuiClientTabButton.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTabButton.java:94)
- [ ] `GuiClientTabButton.previous(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTabButton.java:104)
- [ ] `GuiClientTabButton.index(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTabButton.java:114)

### `GuiClientTextfield`: it's generic

- [ ] `GuiClientTextfield.GuiClientTextfield(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:43)
- [ ] `GuiClientTextfield.setFloatOnly(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:47)
- [ ] `GuiClientTextfield.setNumbersIncludingNegativeOnly(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:63)
- [ ] `GuiClientTextfield.setNumbersOnly(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:79)
- [ ] `GuiClientTextfield.setHexOnly(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:94)
- [ ] `GuiClientTextfield.setText(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:107)
- [ ] `GuiClientTextfield.getText(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:127)
- [ ] `GuiClientTextfield.onTextChanged(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:132)
- [ ] `GuiClientTextfield.setSuggestion(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:136)
- [ ] `GuiClientTextfield.getHighlighted(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:141)
- [ ] `GuiClientTextfield.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:147)
- [ ] `GuiClientTextfield.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:152)
- [ ] `GuiClientTextfield.renderContent(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:156)
- [ ] `GuiClientTextfield.writeText(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:212)
- [ ] `GuiClientTextfield.deleteText(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:237)
- [ ] `GuiClientTextfield.deleteWords(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:245)
- [ ] `GuiClientTextfield.deleteChars(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:255)
- [ ] `GuiClientTextfield.deleteCharsToPos(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:259)
- [ ] `GuiClientTextfield.getWordPosition(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:276)
- [ ] `GuiClientTextfield.getWordPosition(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:280)
- [ ] `GuiClientTextfield.getWordPosition(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:284)
- [ ] `GuiClientTextfield.moveCursor(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:314)
- [ ] `GuiClientTextfield.getCursorPos(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:318)
- [ ] `GuiClientTextfield.moveCursorTo(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:322)
- [ ] `GuiClientTextfield.setCursorPosition(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:329)
- [ ] `GuiClientTextfield.moveCursorToStart(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:334)
- [ ] `GuiClientTextfield.moveCursorToEnd(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:338)
- [ ] `GuiClientTextfield.keyPressed(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:343)
- [ ] `GuiClientTextfield.canWrite(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:407)
- [ ] `GuiClientTextfield.charTyped(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:411)
- [ ] `GuiClientTextfield.findClickedPositionInText(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:423)
- [ ] `GuiClientTextfield.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:430)
- [ ] `GuiClientTextfield.mouseDoubleClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:441)
- [ ] `GuiClientTextfield.mouseDragged(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:451)
- [ ] `GuiClientTextfield.testForDoubleClick(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:458)
- [ ] `GuiClientTextfield.getCursorPosition(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:463)
- [ ] `GuiClientTextfield.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:467)
- [ ] `GuiClientTextfield.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:471)
- [ ] `GuiClientTextfield.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:475)
- [ ] `GuiClientTextfield.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:480)
- [ ] `GuiClientTextfield.setHighlightPos(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:485)
- [ ] `GuiClientTextfield.scrollTo(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:490)
- [ ] `GuiClientTextfield.setValidator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:510)
- [ ] `GuiClientTextfield.setMaxStringLength(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/simple/GuiClientTextfield.java:515)

### `GuiClientTimeline`: it's generic

- [x] `GuiClientTimeline.MAXIMUM_ZOOM` (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:21)
- [ ] `GuiClientTimeline.CURSOR_HIGHLIGHT`: it creates a DisplayColor, which isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:22)
- [ ] `GuiClientTimeline.GuiClientTimeline(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:31)
- [ ] `GuiClientTimeline.scrolledX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:35)
- [ ] `GuiClientTimeline.afterDurationSet(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:39)
- [ ] `GuiClientTimeline.resized(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:50)
- [ ] `GuiClientTimeline.updateTickWidth(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:55)
- [ ] `GuiClientTimeline.setLastZoom(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:59)
- [ ] `GuiClientTimeline.getLastZoom(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:63)
- [ ] `GuiClientTimeline.adjustKeyPositionX(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:67)
- [ ] `GuiClientTimeline.adjustKeysPositionX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:73)
- [ ] `GuiClientTimeline.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:85)
- [ ] `GuiClientTimeline.render(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:96)
- [ ] `GuiClientTimeline.mouseScrolled(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:103)
- [ ] `GuiClientTimeline.scrolled(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:116)
- [ ] `GuiClientTimeline.getTickWidth(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:128)
- [ ] `GuiClientTimeline.getTickWidthAimed(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:132)
- [ ] `GuiClientTimeline.getTimeAt(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:136)
- [ ] `GuiClientTimeline.getTimeAtAimed(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimeline.java:141)

### `GuiClientTimelineChannel`: it's generic

- [ ] `GuiClientTimelineChannel.GuiClientTimelineChannel(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java:15)
- [ ] `GuiClientTimelineChannel.timeline(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java:19)
- [ ] `GuiClientTimelineChannel.keyInitialFlow(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java:23)
- [ ] `GuiClientTimelineChannel.dragKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java:33)
- [ ] `GuiClientTimelineChannel.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java:38)
- [ ] `GuiClientTimelineChannel.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java:48)
- [ ] `GuiClientTimelineChannel.getOffsetX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java:53)
- [ ] `GuiClientTimelineChannel.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java:58)
- [ ] `GuiClientTimelineChannel.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java:71)
- [ ] `GuiClientTimelineChannel.mouseScrolled(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java:81)
- [ ] `GuiClientTimelineChannel.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineChannel.java:87)

### `GuiClientTimelineHeader`: it's generic

- [ ] `GuiClientTimelineHeader.GuiClientTimelineHeader(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineHeader.java:20)
- [ ] `GuiClientTimelineHeader.timeline(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineHeader.java:24)
- [ ] `GuiClientTimelineHeader.handler(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineHeader.java:28)
- [ ] `GuiClientTimelineHeader.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineHeader.java:32)
- [ ] `GuiClientTimelineHeader.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineHeader.java:40)
- [ ] `GuiClientTimelineHeader.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineHeader.java:51)
- [ ] `GuiClientTimelineHeader.mouseScrolled(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineHeader.java:56)
- [ ] `GuiClientTimelineHeader.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineHeader.java:62)
- [ ] `GuiClientTimelineHeader.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineHeader.java:114)

### `GuiClientTimelineKey`: it's generic

- [x] `GuiClientTimelineKey.DRAG_TIME` (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:17)
- [ ] `GuiClientTimelineKey.GuiClientTimelineKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:21)
- [ ] `GuiClientTimelineKey.createChildRect(4)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:25)
- [ ] `GuiClientTimelineKey.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:32)
- [ ] `GuiClientTimelineKey.mouseDragged(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:54)
- [ ] `GuiClientTimelineKey.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:66)
- [ ] `GuiClientTimelineKey.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:71)
- [ ] `GuiClientTimelineKey.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:75)
- [ ] `GuiClientTimelineKey.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:79)
- [ ] `GuiClientTimelineKey.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:84)
- [ ] `GuiClientTimelineKey.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:89)
- [ ] `GuiClientTimelineKey.render(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:94)
- [ ] `GuiClientTimelineKey.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:120)
- [ ] `GuiClientTimelineKey.setSelected(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/timeline/GuiClientTimelineKey.java:124)

### `GuiClientTree`: it's generic

- [x] `GuiClientTree.LEVEL_SPACING` (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:23)
- [ ] `GuiClientTree.GuiClientTree(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:35)
- [ ] `GuiClientTree.setKeepOrder(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:39)
- [ ] `GuiClientTree.keepOrder(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:44)
- [ ] `GuiClientTree.reflowTree(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:49)
- [ ] `GuiClientTree.setLineThickness(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:57)
- [ ] `GuiClientTree.offsetByLevel(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:63)
- [ ] `GuiClientTree.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:67)
- [ ] `GuiClientTree.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:82)
- [ ] `GuiClientTree.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:96)
- [ ] `GuiClientTree.renderContent(8)`: its body couldn't be read: a labelled statement (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:102)
- [ ] `GuiClientTree.createPosition(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:222)
- [ ] `GuiClientTree.calculatePosition(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:228)
- [ ] `GuiClientTree.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:253)
- [ ] `GuiClientTree.isDragged(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:266)
- [ ] `GuiClientTree.startDrag(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:271)
- [ ] `GuiClientTree.endDrag(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:276)

### `GuiClientTree.GuiTreeLine`

- [ ] `GuiClientTree.GuiTreeLine.GuiTreeLine(3)`: unknown static GuiClientTree.this (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:293)
- [x] `GuiClientTree.GuiTreeLine.set(2)` (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:298)
- [ ] `GuiClientTree.GuiTreeLine.render(1)`: it uses Minecraft's `GuiGraphicsExtractor` (team/creative/creativecore/client/gui/control/tree/GuiClientTree.java:304)

### `GuiClientTreeItem`: it's generic

- [ ] `GuiClientTreeItem.GuiClientTreeItem(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTreeItem.java:12)
- [ ] `GuiClientTreeItem.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTreeItem.java:16)
- [ ] `GuiClientTreeItem.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTreeItem.java:27)
- [ ] `GuiClientTreeItem.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTreeItem.java:42)
- [ ] `GuiClientTreeItem.mouseDoubleClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTreeItem.java:52)
- [ ] `GuiClientTreeItem.testForDoubleClick(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTreeItem.java:60)
- [ ] `GuiClientTreeItem.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/control/tree/GuiClientTreeItem.java:65)

### `GuiClientTreeItem.ItemClickState`


### `GuiExtensionCreator`: it's generic

- [ ] `GuiExtensionCreator.GuiExtensionCreator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:16)
- [ ] `GuiExtensionCreator.toggle(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:20)
- [ ] `GuiExtensionCreator.toggle(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:24)
- [ ] `GuiExtensionCreator.open(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:32)
- [ ] `GuiExtensionCreator.open(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:36)
- [ ] `GuiExtensionCreator.open(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:40)
- [ ] `GuiExtensionCreator.get(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:49)
- [ ] `GuiExtensionCreator.close(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:53)
- [ ] `GuiExtensionCreator.hasLostFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:61)
- [ ] `GuiExtensionCreator.hasExtension(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:65)
- [ ] `GuiExtensionCreator.checkShouldClose(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:69)
- [ ] `GuiExtensionCreator.markLostFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:75)
- [ ] `GuiExtensionCreator.markKeptFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:79)

### `GuiExtensionCreator.ExtensionDirection`: its constants carry values

- [ ] `GuiExtensionCreator.ExtensionDirection.apply(4)`: it has no body (team/creative/creativecore/client/gui/extension/GuiExtensionCreator.java:138)

### `GuiClientControl`: it's generic

- [ ] `GuiClientControl.getScreenRect(0)`: it calls Minecraft's Minecraft.getWindow (no Horizon helper for it yet) (team/creative/creativecore/client/gui/GuiClientControl.java:42)
- [ ] `GuiClientControl.scissor(2)`: it uses CreativeGuiGraphics, which isn't translated (it's an interface) (team/creative/creativecore/client/gui/GuiClientControl.java:47)
- [ ] `GuiClientControl.GuiClientControl(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:60)
- [ ] `GuiClientControl.getParent(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:64)
- [ ] `GuiClientControl.setVisible(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:69)
- [ ] `GuiClientControl.setFixed(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:74)
- [ ] `GuiClientControl.setFixedX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:80)
- [ ] `GuiClientControl.setFixedY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:85)
- [ ] `GuiClientControl.setExpandable(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:90)
- [ ] `GuiClientControl.setExpandableX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:96)
- [ ] `GuiClientControl.setExpandableY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:101)
- [ ] `GuiClientControl.setDim(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:106)
- [ ] `GuiClientControl.setDim(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:111)
- [ ] `GuiClientControl.setEnabled(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:116)
- [ ] `GuiClientControl.removeFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:121)
- [ ] `GuiClientControl.setFormatting(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:126)
- [ ] `GuiClientControl.setCustomFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:131)
- [ ] `GuiClientControl.isExpandableX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:136)
- [ ] `GuiClientControl.isExpandableY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:141)
- [ ] `GuiClientControl.getStyle(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:146)
- [ ] `GuiClientControl.raiseEvent(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:154)
- [ ] `GuiClientControl.createChildRect(4)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:158)
- [ ] `GuiClientControl.flowX(2)`: it has no body (team/creative/creativecore/client/gui/GuiClientControl.java:162)
- [ ] `GuiClientControl.flowY(3)`: it has no body (team/creative/creativecore/client/gui/GuiClientControl.java:164)
- [ ] `GuiClientControl.minWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:166)
- [ ] `GuiClientControl.preferredWidth(1)`: it has no body (team/creative/creativecore/client/gui/GuiClientControl.java:170)
- [ ] `GuiClientControl.maxWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:172)
- [ ] `GuiClientControl.minHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:176)
- [ ] `GuiClientControl.preferredHeight(2)`: it has no body (team/creative/creativecore/client/gui/GuiClientControl.java:180)
- [ ] `GuiClientControl.maxHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:182)
- [ ] `GuiClientControl.testForDoubleClick(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:186)
- [ ] `GuiClientControl.isInteractable(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:190)
- [ ] `GuiClientControl.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:194)
- [ ] `GuiClientControl.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:197)
- [ ] `GuiClientControl.mouseDoubleClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:201)
- [ ] `GuiClientControl.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:205)
- [ ] `GuiClientControl.mouseDragged(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:208)
- [ ] `GuiClientControl.mouseScrolled(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:211)
- [ ] `GuiClientControl.keyPressed(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:215)
- [ ] `GuiClientControl.keyReleased(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:219)
- [ ] `GuiClientControl.charTyped(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:223)
- [ ] `GuiClientControl.looseFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:227)
- [ ] `GuiClientControl.defaultFormatting(0)`: it has no body (team/creative/creativecore/client/gui/GuiClientControl.java:230)
- [ ] `GuiClientControl.getControlFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:232)
- [ ] `GuiClientControl.getContentOffset(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:236)
- [ ] `GuiClientControl.getBorder(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:240)
- [ ] `GuiClientControl.getBackground(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:244)
- [ ] `GuiClientControl.render(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:248)
- [ ] `GuiClientControl.renderContent(8)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:273)
- [ ] `GuiClientControl.renderContent(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:293)
- [ ] `GuiClientControl.renderContent(3)`: it has no body (team/creative/creativecore/client/gui/GuiClientControl.java:297)
- [ ] `GuiClientControl.getTooltip(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:299)
- [ ] `GuiClientControl.hasTooltipExtraSpaceAfterFirstLine(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:303)
- [ ] `GuiClientControl.getTooltipEvent(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:307)
- [ ] `GuiClientControl.setTooltip(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:327)
- [ ] `GuiClientControl.setTooltip(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:336)
- [ ] `GuiClientControl.playSound(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:341)
- [ ] `GuiClientControl.playSound(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:345)
- [ ] `GuiClientControl.playSound(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:350)
- [ ] `GuiClientControl.playSound(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:355)
- [ ] `GuiClientControl.playSound(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientControl.java:360)

### `GuiClientLayer`: it extends `GuiClientParent`

- [x] `GuiClientLayer.MINIMUM_OUTER_SPACING` (team/creative/creativecore/client/gui/GuiClientLayer.java:17)
- [ ] `GuiClientLayer.GuiClientLayer(1)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:20)
- [ ] `GuiClientLayer.managers(0)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:24)
- [ ] `GuiClientLayer.getControlFormatting(0)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:28)
- [ ] `GuiClientLayer.getWidth(0)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:33)
- [ ] `GuiClientLayer.getHeight(0)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:37)
- [ ] `GuiClientLayer.reflow(0)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:41)
- [ ] `GuiClientLayer.getStyle(0)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:104)
- [ ] `GuiClientLayer.renderContent(6)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:109)
- [ ] `GuiClientLayer.renderContent(3)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:118)
- [ ] `GuiClientLayer.getSettings(0)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:122)
- [ ] `GuiClientLayer.hasGrayBackground(0)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:126)
- [ ] `GuiClientLayer.mouseClicked(3)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:130)
- [ ] `GuiClientLayer.mouseReleased(3)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:145)
- [ ] `GuiClientLayer.keyPressed(1)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:154)
- [ ] `GuiClientLayer.applyOffset(2)`: its class isn't translated (it extends `GuiClientParent`) (team/creative/creativecore/client/gui/GuiClientLayer.java:174)

### `GuiClientParent`: it's generic

- [ ] `GuiClientParent.GuiClientParent(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:30)
- [ ] `GuiClientParent.initControlList(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:34)
- [ ] `GuiClientParent.controls(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:39)
- [ ] `GuiClientParent.hoverControls(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:44)
- [ ] `GuiClientParent.all(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:49)
- [ ] `GuiClientParent.setScale(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:54)
- [ ] `GuiClientParent.scaleFactor(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:60)
- [ ] `GuiClientParent.scaleFactorInv(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:64)
- [ ] `GuiClientParent.getOffsetY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:68)
- [ ] `GuiClientParent.getOffsetX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:72)
- [ ] `GuiClientParent.setAlign(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:76)
- [ ] `GuiClientParent.setVAlign(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:81)
- [ ] `GuiClientParent.setSpacing(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:86)
- [ ] `GuiClientParent.setFlow(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:91)
- [ ] `GuiClientParent.renderControls(10)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:96)
- [ ] `GuiClientParent.renderControl(8)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:131)
- [ ] `GuiClientParent.renderContent(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:137)
- [ ] `GuiClientParent.renderContent(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:165)
- [ ] `GuiClientParent.isMouseOverHovered(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:169)
- [ ] `GuiClientParent.getTooltipEvent(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:185)
- [ ] `GuiClientParent.testForDoubleClick(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:210)
- [ ] `GuiClientParent.mouseMoved(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:227)
- [ ] `GuiClientParent.mouseClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:242)
- [ ] `GuiClientParent.mouseDoubleClicked(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:263)
- [ ] `GuiClientParent.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:287)
- [ ] `GuiClientParent.mouseDragged(6)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:302)
- [ ] `GuiClientParent.mouseScrolled(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:317)
- [ ] `GuiClientParent.keyPressed(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:334)
- [ ] `GuiClientParent.keyReleased(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:345)
- [ ] `GuiClientParent.charTyped(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:356)
- [ ] `GuiClientParent.looseFocus(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:367)
- [ ] `GuiClientParent.flowX(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:374)
- [ ] `GuiClientParent.flowY(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:387)
- [ ] `GuiClientParent.endlessX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:401)
- [ ] `GuiClientParent.endlessY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:405)
- [ ] `GuiClientParent.minWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:409)
- [ ] `GuiClientParent.preferredWidth(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:416)
- [ ] `GuiClientParent.minHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:423)
- [ ] `GuiClientParent.preferredHeight(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:431)
- [ ] `GuiClientParent.defaultFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:439)
- [ ] `GuiClientParent.applyOffset(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiClientParent.java:444)

### `GuiClientRegistrySided`

- [x] `GuiClientRegistrySided.register(0)` (team/creative/creativecore/client/gui/GuiClientRegistrySided.java:4)

### `GuiControlRect`: it uses GuiClientControl, which isn't translated (it's generic)

- [ ] `GuiControlRect.getMinWidth(2)`: it uses GuiClientControl, which isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiControlRect.java:12)
- [ ] `GuiControlRect.getPreferredWidth(2)`: it uses GuiClientControl, which isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiControlRect.java:23)
- [ ] `GuiControlRect.getMaxWidth(2)`: it uses GuiClientControl, which isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiControlRect.java:34)
- [ ] `GuiControlRect.getMinHeight(3)`: it uses GuiClientControl, which isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiControlRect.java:45)
- [ ] `GuiControlRect.getPreferredHeight(3)`: it uses GuiClientControl, which isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiControlRect.java:56)
- [ ] `GuiControlRect.getMaxHeight(3)`: it uses GuiClientControl, which isn't translated (it's generic) (team/creative/creativecore/client/gui/GuiControlRect.java:67)
- [ ] `GuiControlRect.GuiControlRect(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:78)
- [ ] `GuiControlRect.getX(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:82)
- [ ] `GuiControlRect.centerX(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:86)
- [ ] `GuiControlRect.setX(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:90)
- [ ] `GuiControlRect.getY(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:96)
- [ ] `GuiControlRect.centerY(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:100)
- [ ] `GuiControlRect.setY(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:104)
- [ ] `GuiControlRect.getWidth(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:110)
- [ ] `GuiControlRect.getHeight(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:114)
- [ ] `GuiControlRect.getBottom(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:118)
- [ ] `GuiControlRect.getRight(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:122)
- [ ] `GuiControlRect.setRight(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:126)
- [ ] `GuiControlRect.setBottom(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:130)
- [ ] `GuiControlRect.getContentWidth(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:134)
- [ ] `GuiControlRect.getContentHeight(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:138)
- [ ] `GuiControlRect.clampWidth(2)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:142)
- [ ] `GuiControlRect.setWidth(2)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:156)
- [ ] `GuiControlRect.clampHeight(2)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:162)
- [ ] `GuiControlRect.setHeight(2)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:176)
- [ ] `GuiControlRect.addWidth(2)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:182)
- [ ] `GuiControlRect.addHeight(2)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:188)
- [ ] `GuiControlRect.isMaxWidth(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:194)
- [ ] `GuiControlRect.isMaxHeight(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:198)
- [ ] `GuiControlRect.getMinWidth(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:202)
- [ ] `GuiControlRect.getMaxWidth(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:207)
- [ ] `GuiControlRect.getPreferredWidth(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:212)
- [ ] `GuiControlRect.getMinHeight(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:216)
- [ ] `GuiControlRect.getMaxHeight(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:221)
- [ ] `GuiControlRect.getPreferredHeight(1)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:226)
- [ ] `GuiControlRect.flowX(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:230)
- [ ] `GuiControlRect.flowY(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:235)
- [ ] `GuiControlRect.inside(2)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:241)
- [ ] `GuiControlRect.insideLocalPos(2)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:245)
- [ ] `GuiControlRect.isExpandableX(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:249)
- [ ] `GuiControlRect.isExpandableY(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:253)
- [ ] `GuiControlRect.rectCopy(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:257)
- [ ] `GuiControlRect.toString(0)`: its class isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/client/gui/GuiControlRect.java:261)

### `IGuiClientIntegratedParent`: it's an interface

- [ ] `IGuiClientIntegratedParent.render(6)`: it uses IGuiIntegratedParent, which isn't translated (it's an interface) (team/creative/creativecore/client/gui/IGuiClientIntegratedParent.java:23)
- [ ] `IGuiClientIntegratedParent.toScreenRect(2)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/gui/IGuiClientIntegratedParent.java:70)

### `ContainerScreenIntegration`: it extends Minecraft's `AbstractContainerScreen`

- [ ] `ContainerScreenIntegration.ContainerScreenIntegration(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:28)
- [ ] `ContainerScreenIntegration.init(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:33)
- [ ] `ContainerScreenIntegration.rebuildWidgets(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:37)
- [ ] `ContainerScreenIntegration.mouseDragged(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:45)
- [ ] `ContainerScreenIntegration.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:49)
- [ ] `ContainerScreenIntegration.clientTick(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:70)
- [ ] `ContainerScreenIntegration.getWidth(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:77)
- [ ] `ContainerScreenIntegration.getHeight(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:88)
- [ ] `ContainerScreenIntegration.mouseMoved(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:99)
- [ ] `ContainerScreenIntegration.mouseScrolled(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:103)
- [ ] `ContainerScreenIntegration.keyPressed(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:107)
- [ ] `ContainerScreenIntegration.keyReleased(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:111)
- [ ] `ContainerScreenIntegration.charTyped(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:115)
- [ ] `ContainerScreenIntegration.provider(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:119)
- [ ] `ContainerScreenIntegration.getLayers(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:124)
- [ ] `ContainerScreenIntegration.getTopLayer(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:129)
- [ ] `ContainerScreenIntegration.openLayer(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:134)
- [ ] `ContainerScreenIntegration.closeLayer(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:139)
- [ ] `ContainerScreenIntegration.send(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:144)
- [ ] `ContainerScreenIntegration.isContainer(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:149)
- [ ] `ContainerScreenIntegration.isClient(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:154)
- [ ] `ContainerScreenIntegration.getPlayer(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:159)
- [ ] `ContainerScreenIntegration.closeTopLayer(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:164)
- [ ] `ContainerScreenIntegration.closeLayer(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:169)
- [ ] `ContainerScreenIntegration.createDist(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:174)
- [ ] `ContainerScreenIntegration.createDist(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (team/creative/creativecore/client/gui/integration/ContainerScreenIntegration.java:179)

### `GuiScreenIntegration`: it extends Minecraft's `Screen`

- [ ] `GuiScreenIntegration.GuiScreenIntegration(1)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:28)
- [ ] `GuiScreenIntegration.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:35)
- [ ] `GuiScreenIntegration.rebuildWidgets(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:43)
- [ ] `GuiScreenIntegration.getWidth(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:51)
- [ ] `GuiScreenIntegration.getHeight(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:62)
- [ ] `GuiScreenIntegration.clientTick(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:73)
- [ ] `GuiScreenIntegration.removed(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:80)
- [ ] `GuiScreenIntegration.isContainer(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:86)
- [ ] `GuiScreenIntegration.isClient(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:91)
- [ ] `GuiScreenIntegration.getPlayer(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:96)
- [ ] `GuiScreenIntegration.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:101)
- [ ] `GuiScreenIntegration.getLayers(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:105)
- [ ] `GuiScreenIntegration.getTopLayer(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:110)
- [ ] `GuiScreenIntegration.openLayer(1)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:115)
- [ ] `GuiScreenIntegration.closeLayer(1)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:122)
- [ ] `GuiScreenIntegration.closeLayer(1)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:130)
- [ ] `GuiScreenIntegration.closeTopLayer(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:144)
- [ ] `GuiScreenIntegration.mouseMoved(2)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:149)
- [ ] `GuiScreenIntegration.mouseScrolled(4)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:153)
- [ ] `GuiScreenIntegration.send(1)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:157)
- [ ] `GuiScreenIntegration.provider(0)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:161)
- [ ] `GuiScreenIntegration.createDist(1)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:166)
- [ ] `GuiScreenIntegration.createDist(1)`: its class isn't translated (it extends Minecraft's `Screen`) (team/creative/creativecore/client/gui/integration/GuiScreenIntegration.java:171)

### `ScreenEventListener`: it implements Minecraft's `GuiEventListener`

- [x] `ScreenEventListener.DOUBLE_CLICK_TIME` (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:19)
- [ ] `ScreenEventListener.ScreenEventListener(2)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:29)
- [ ] `ScreenEventListener.getTopLayer(0)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:34)
- [ ] `ScreenEventListener.getOffsetX(0)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:38)
- [ ] `ScreenEventListener.getOffsetY(0)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:42)
- [ ] `ScreenEventListener.tick(0)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:46)
- [ ] `ScreenEventListener.getEventTime(0)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:52)
- [ ] `ScreenEventListener.fireRemaingEvents(0)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:56)
- [ ] `ScreenEventListener.mouseMoved(2)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:68)
- [ ] `ScreenEventListener.mouseClicked(2)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:72)
- [ ] `ScreenEventListener.mouseReleased(1)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:92)
- [ ] `ScreenEventListener.mouseDragged(3)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:103)
- [ ] `ScreenEventListener.mouseScrolled(4)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:112)
- [ ] `ScreenEventListener.keyPressed(1)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:116)
- [ ] `ScreenEventListener.keyReleased(1)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:120)
- [ ] `ScreenEventListener.charTyped(1)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:124)
- [ ] `ScreenEventListener.setFocused(1)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:128)
- [ ] `ScreenEventListener.isFocused(0)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:133)
- [ ] `ScreenEventListener.isMouseOver(2)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:137)
- [ ] `ScreenEventListener.updateNarration(1)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:141)
- [ ] `ScreenEventListener.narrationPriority(0)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:144)
- [ ] `ScreenEventListener.capturesInput(0)`: its class isn't translated (it implements Minecraft's `GuiEventListener`) (team/creative/creativecore/client/gui/integration/ScreenEventListener.java:148)

### `GuiClientManager`: it's generic

- [ ] `GuiClientManager.GuiClientManager(1)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/manager/GuiClientManager.java:11)
- [ ] `GuiClientManager.renderOverlay(4)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/manager/GuiClientManager.java:15)
- [ ] `GuiClientManager.mouseReleased(3)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/manager/GuiClientManager.java:18)
- [ ] `GuiClientManager.mouseClickedOutside(2)`: its class isn't translated (it's generic) (team/creative/creativecore/client/gui/manager/GuiClientManager.java:21)

### `GuiClientManagerItem`: it extends `GuiClientManager`

- [ ] `GuiClientManagerItem.freeSpace(2)`: it uses Minecraft's `Slot` (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:30)
- [ ] `GuiClientManagerItem.GuiClientManagerItem(1)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:43)
- [ ] `GuiClientManagerItem.getHand(0)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:47)
- [ ] `GuiClientManagerItem.setHandChanged(0)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:51)
- [ ] `GuiClientManagerItem.mouseClickedOutside(2)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:55)
- [ ] `GuiClientManagerItem.mouseReleased(3)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:62)
- [ ] `GuiClientManagerItem.renderOverlay(4)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:69)
- [ ] `GuiClientManagerItem.startDrag(3)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:99)
- [ ] `GuiClientManagerItem.addToDrag(1)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:109)
- [ ] `GuiClientManagerItem.modifyDrag(1)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:118)
- [ ] `GuiClientManagerItem.abortDrag(0)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:122)
- [ ] `GuiClientManagerItem.endDrag(0)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:133)
- [ ] `GuiClientManagerItem.isDragged(0)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:162)
- [ ] `GuiClientManagerItem.setHand(1)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:166)
- [ ] `GuiClientManagerItem.additionalDragCount(1)`: its class isn't translated (it extends `GuiClientManager`) (team/creative/creativecore/client/gui/manager/GuiClientManagerItem.java:170)

### `GuiClientRegistry`

- [ ] `GuiClientRegistry.CONTROL_FACTORY`: it uses PairList, which isn't translated (it's generic) (team/creative/creativecore/client/gui/registry/GuiClientRegistry.java:128)
- [ ] `GuiClientRegistry.BACKUP_CONTROL_FACTORY`: it uses `Object2ObjectMap` (it.unimi.dsi.fastutil.objects) (team/creative/creativecore/client/gui/registry/GuiClientRegistry.java:129)
- [ ] `GuiClientRegistry.CONTROL_SPECIAL_FACTORY`: it uses GuiControl, which isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/client/gui/registry/GuiClientRegistry.java:130)
- [ ] `GuiClientRegistry.MANAGER_FACTORY`: it uses `Object2ObjectMap` (it.unimi.dsi.fastutil.objects) (team/creative/creativecore/client/gui/registry/GuiClientRegistry.java:131)
- [ ] `GuiClientRegistry.create(1)`: it uses GuiClientManager, which isn't translated (it's generic) (team/creative/creativecore/client/gui/registry/GuiClientRegistry.java:133)
- [ ] `GuiClientRegistry.create(1)`: it uses GuiClientManager, which isn't translated (it's generic) (team/creative/creativecore/client/gui/registry/GuiClientRegistry.java:166)
- [ ] `GuiClientRegistry.register(2)`: it's generic (team/creative/creativecore/client/gui/registry/GuiClientRegistry.java:171)
- [ ] `GuiClientRegistry.registerManager(2)`: it's generic (team/creative/creativecore/client/gui/registry/GuiClientRegistry.java:175)
- [ ] `GuiClientRegistry.registerSpecial(1)`: it uses GuiControl, which isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/client/gui/registry/GuiClientRegistry.java:179)

### `ICreativeClientLoader`: it's an interface

- [ ] `ICreativeClientLoader.registerModifyTooltip(1)`: it has no body (team/creative/creativecore/client/ICreativeClientLoader.java:21)
- [ ] `ICreativeClientLoader.registerMenu(2)`: it has no body (team/creative/creativecore/client/ICreativeClientLoader.java:23)
- [ ] `ICreativeClientLoader.addScreenWidget(2)`: it has no body (team/creative/creativecore/client/ICreativeClientLoader.java:25)
- [ ] `ICreativeClientLoader.registerCategory(1)`: it has no body (team/creative/creativecore/client/ICreativeClientLoader.java:27)
- [ ] `ICreativeClientLoader.registerKeybind(1)`: it has no body (team/creative/creativecore/client/ICreativeClientLoader.java:29)

### `ICreativeClientLoader.TooltipModifier`: it's an interface

- [ ] `ICreativeClientLoader.TooltipModifier.modifyTooltip(4)`: it has no body (team/creative/creativecore/client/ICreativeClientLoader.java:33)

### `ModMenuImpl`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `ModMenuImpl.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (team/creative/creativecore/client/ModMenuImpl.java:15)
- [ ] `ModMenuImpl.getConfig(1)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (team/creative/creativecore/client/ModMenuImpl.java:19)
- [ ] `ModMenuImpl.getProvidedConfigScreenFactories(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (team/creative/creativecore/client/ModMenuImpl.java:30)

### `QuadGeneratorContext`: it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))

- [ ] `QuadGeneratorContext.set(4)`: its class isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))) (team/creative/creativecore/client/render/box/QuadGeneratorContext.java:38)
- [ ] `QuadGeneratorContext.setQuad(3)`: its class isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))) (team/creative/creativecore/client/render/box/QuadGeneratorContext.java:63)
- [ ] `QuadGeneratorContext.setBounds(6)`: its class isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))) (team/creative/creativecore/client/render/box/QuadGeneratorContext.java:68)
- [ ] `QuadGeneratorContext.hasBounds(0)`: its class isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))) (team/creative/creativecore/client/render/box/QuadGeneratorContext.java:80)
- [ ] `QuadGeneratorContext.clear(0)`: its class isn't translated (it uses RenderBox, which isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box))) (team/creative/creativecore/client/render/box/QuadGeneratorContext.java:88)

### `RenderBox`: it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)

- [ ] `RenderBox.DOWN`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/box/RenderBox.java:16)
- [ ] `RenderBox.UP`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/box/RenderBox.java:19)
- [ ] `RenderBox.NORTH`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/box/RenderBox.java:22)
- [ ] `RenderBox.SOUTH`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/box/RenderBox.java:25)
- [ ] `RenderBox.WEST`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/box/RenderBox.java:28)
- [ ] `RenderBox.EAST`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/box/RenderBox.java:31)
- [ ] `RenderBox.RenderBox(1)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:55)
- [ ] `RenderBox.RenderBox(2)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:59)
- [ ] `RenderBox.RenderBox(2)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:71)
- [ ] `RenderBox.RenderBox(2)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:76)
- [ ] `RenderBox.RenderBox(7)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:80)
- [ ] `RenderBox.RenderBox(7)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:85)
- [ ] `RenderBox.setColor(1)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:89)
- [ ] `RenderBox.setKeepUV(1)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:94)
- [ ] `RenderBox.setQuad(2)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:99)
- [ ] `RenderBox.getQuad(1)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:122)
- [ ] `RenderBox.countQuads(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:141)
- [ ] `RenderBox.setFace(2)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:170)
- [ ] `RenderBox.getFace(1)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:192)
- [ ] `RenderBox.shouldRenderFace(1)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:203)
- [ ] `RenderBox.intersectsWithFace(3)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:214)
- [ ] `RenderBox.getRenderQuads(1)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:236)
- [ ] `RenderBox.getOffsetX(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:259)
- [ ] `RenderBox.getOffsetY(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:263)
- [ ] `RenderBox.getOffsetZ(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:267)
- [ ] `RenderBox.getOverallScale(1)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:271)
- [ ] `RenderBox.getScaleX(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:275)
- [ ] `RenderBox.getScaleY(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:279)
- [ ] `RenderBox.getScaleZ(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:283)
- [ ] `RenderBox.scaleAndOffsetQuads(1)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:287)
- [ ] `RenderBox.onlyScaleOnceNoOffset(1)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:291)
- [ ] `RenderBox.deleteQuadCache(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:295)
- [ ] `RenderBox.previewScalingAndOffset(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:305)
- [ ] `RenderBox.getPreviewOffX(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:309)
- [ ] `RenderBox.getPreviewOffY(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:313)
- [ ] `RenderBox.getPreviewOffZ(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:317)
- [ ] `RenderBox.getPreviewScaleX(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:321)
- [ ] `RenderBox.getPreviewScaleY(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:325)
- [ ] `RenderBox.getPreviewScaleZ(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:329)
- [ ] `RenderBox.isTranslucent(0)`: its class isn't translated (it extends `AlignedBox` (team.creative.creativecore.common.util.math.box)) (team/creative/creativecore/client/render/box/RenderBox.java:333)

### `RenderBox.VectorFanSimple`: it extends `VectorFan` (team.creative.creativecore.common.util.math.geo)

- [ ] `RenderBox.VectorFanSimple.VectorFanSimple(1)`: its class isn't translated (it extends `VectorFan` (team.creative.creativecore.common.util.math.geo)) (team/creative/creativecore/client/render/box/RenderBox.java:338)

### `CreativePlatformHooks`

- [x] `CreativePlatformHooks.backupRenderState(0)` (team/creative/creativecore/client/render/CreativePlatformHooks.java:4)
- [x] `CreativePlatformHooks.restoreRenderState(0)` (team/creative/creativecore/client/render/CreativePlatformHooks.java:7)

### `RenderBoxFace`

- [ ] `RenderBoxFace.RENDER`: its value couldn't be read: an anonymous class (team/creative/creativecore/client/render/face/RenderBoxFace.java:7)
- [ ] `RenderBoxFace.NOT_RENDER`: its value couldn't be read: an anonymous class (team/creative/creativecore/client/render/face/RenderBoxFace.java:13)
- [ ] `RenderBoxFace.shouldRender(0)`: it has no body (team/creative/creativecore/client/render/face/RenderBoxFace.java:20)
- [ ] `RenderBoxFace.hasCachedFans(0)`: it has no body (team/creative/creativecore/client/render/face/RenderBoxFace.java:22)
- [ ] `RenderBoxFace.getCachedFans(0)`: it has no body (team/creative/creativecore/client/render/face/RenderBoxFace.java:24)
- [ ] `RenderBoxFace.getScale(0)`: it has no body (team/creative/creativecore/client/render/face/RenderBoxFace.java:26)

### `RenderBoxFace.RenderBoxFaceStatic`: it extends `RenderBoxFace`

- [ ] `RenderBoxFace.RenderBoxFaceStatic.hasCachedFans(0)`: its class isn't translated (it extends `RenderBoxFace`) (team/creative/creativecore/client/render/face/RenderBoxFace.java:29)
- [ ] `RenderBoxFace.RenderBoxFaceStatic.getCachedFans(0)`: its class isn't translated (it extends `RenderBoxFace`) (team/creative/creativecore/client/render/face/RenderBoxFace.java:34)
- [ ] `RenderBoxFace.RenderBoxFaceStatic.getScale(0)`: its class isn't translated (it extends `RenderBoxFace`) (team/creative/creativecore/client/render/face/RenderBoxFace.java:39)

### `RenderBoxFaceSpecial`: it extends `RenderBoxFace`

- [ ] `RenderBoxFaceSpecial.RenderBoxFaceSpecial(2)`: its class isn't translated (it extends `RenderBoxFace`) (team/creative/creativecore/client/render/face/RenderBoxFaceSpecial.java:10)
- [ ] `RenderBoxFaceSpecial.shouldRender(0)`: its class isn't translated (it extends `RenderBoxFace`) (team/creative/creativecore/client/render/face/RenderBoxFaceSpecial.java:15)
- [ ] `RenderBoxFaceSpecial.hasCachedFans(0)`: its class isn't translated (it extends `RenderBoxFace`) (team/creative/creativecore/client/render/face/RenderBoxFaceSpecial.java:20)
- [ ] `RenderBoxFaceSpecial.getCachedFans(0)`: its class isn't translated (it extends `RenderBoxFace`) (team/creative/creativecore/client/render/face/RenderBoxFaceSpecial.java:25)
- [ ] `RenderBoxFaceSpecial.getScale(0)`: its class isn't translated (it extends `RenderBoxFace`) (team/creative/creativecore/client/render/face/RenderBoxFaceSpecial.java:30)

### `VectorFanClient`

- [ ] `VectorFanClient.setLineNormal(3)`: it uses Vec3f, which isn't translated (it extends `VecNf`) (team/creative/creativecore/client/render/face/VectorFanClient.java:16)
- [ ] `VectorFanClient.setLineNormal(7)`: it uses Vec3f, which isn't translated (it extends `VecNf`) (team/creative/creativecore/client/render/face/VectorFanClient.java:20)
- [ ] `VectorFanClient.generate(3)`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/face/VectorFanClient.java:31)
- [ ] `VectorFanClient.generate(7)`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/face/VectorFanClient.java:84)
- [ ] `VectorFanClient.renderLines(7)`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/face/VectorFanClient.java:87)
- [ ] `VectorFanClient.renderLines(9)`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/face/VectorFanClient.java:96)
- [ ] `VectorFanClient.renderLines(13)`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/face/VectorFanClient.java:148)
- [ ] `VectorFanClient.renderLines(15)`: it uses VectorFan, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/client/render/face/VectorFanClient.java:177)

### `ColorRect`: it implements Minecraft's `GuiElementRenderState`

- [ ] `ColorRect.ColorRect(9)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:11)
- [ ] `ColorRect.buildVertices(1)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:14)
- [ ] `ColorRect.bounds(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:21)
- [ ] `ColorRect.pipeline(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:11)
- [ ] `ColorRect.textureSetup(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:11)
- [ ] `ColorRect.pose(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:11)
- [ ] `ColorRect.x0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:11)
- [ ] `ColorRect.y0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:11)
- [ ] `ColorRect.x1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:11)
- [ ] `ColorRect.y1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:11)
- [ ] `ColorRect.color(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:11)
- [ ] `ColorRect.scissorArea(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRect.java:11)

### `ColorRectF`: it implements Minecraft's `GuiElementRenderState`

- [ ] `ColorRectF.ColorRectF(9)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:11)
- [ ] `ColorRectF.buildVertices(1)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:22)
- [ ] `ColorRectF.bounds(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:29)
- [ ] `ColorRectF.pipeline(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:11)
- [ ] `ColorRectF.textureSetup(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:11)
- [ ] `ColorRectF.pose(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:11)
- [ ] `ColorRectF.x0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:11)
- [ ] `ColorRectF.y0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:11)
- [ ] `ColorRectF.x1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:11)
- [ ] `ColorRectF.y1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:11)
- [ ] `ColorRectF.color(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:11)
- [ ] `ColorRectF.scissorArea(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/ColorRectF.java:11)

### `CreativeGuiGraphics`: it's an interface

- [ ] `CreativeGuiGraphics.as(1)`: it uses CreativeGuiGraphics, which isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:24)
- [ ] `CreativeGuiGraphics.drawStringCentered(5)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:28)
- [ ] `CreativeGuiGraphics.drawString(6)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:54)
- [ ] `CreativeGuiGraphics.horizontalGradientRect(6)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:61)
- [ ] `CreativeGuiGraphics.horizontalGradientRect(6)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:68)
- [ ] `CreativeGuiGraphics.verticalGradientRect(6)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:75)
- [ ] `CreativeGuiGraphics.verticalGradientRect(6)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:82)
- [ ] `CreativeGuiGraphics.horizontalGradientMaskRect(6)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:89)
- [ ] `CreativeGuiGraphics.horizontalGradientMaskRect(6)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:93)
- [ ] `CreativeGuiGraphics.colorRect(5)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:97)
- [ ] `CreativeGuiGraphics.colorRect(5)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:102)
- [ ] `CreativeGuiGraphics.textureRect(7)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:107)
- [ ] `CreativeGuiGraphics.textureRect(7)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:125)
- [ ] `CreativeGuiGraphics.textureRect(9)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:143)
- [ ] `CreativeGuiGraphics.textureRect(9)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:151)
- [ ] `CreativeGuiGraphics.textureRectColor(10)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:169)
- [ ] `CreativeGuiGraphics.textureRectStrechted(5)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:177)
- [ ] `CreativeGuiGraphics.textureRectColorStretched(6)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:181)
- [ ] `CreativeGuiGraphics.renderItemDecorations(3)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:185)
- [ ] `CreativeGuiGraphics.renderItemDecorations(4)`: its class isn't translated (it's an interface) (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:189)
- [ ] `CreativeGuiGraphics.blit(11)`: it has no body (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:193)
- [ ] `CreativeGuiGraphics.as(0)`: it has no body (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:195)
- [ ] `CreativeGuiGraphics.minecraft(0)`: it has no body (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:197)
- [ ] `CreativeGuiGraphics.pose(0)`: it has no body (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:199)
- [ ] `CreativeGuiGraphics.font(0)`: it has no body (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:201)
- [ ] `CreativeGuiGraphics.guiRenderState(0)`: it has no body (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:203)
- [ ] `CreativeGuiGraphics.peekScissor(0)`: it has no body (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:205)
- [ ] `CreativeGuiGraphics.setOverrideScissor(1)`: it has no body (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:207)
- [ ] `CreativeGuiGraphics.clearOverrideScissor(0)`: it has no body (team/creative/creativecore/client/render/gui/CreativeGuiGraphics.java:209)

### `HorizontalGradientRect`: it implements Minecraft's `GuiElementRenderState`

- [ ] `HorizontalGradientRect.HorizontalGradientRect(10)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:11)
- [ ] `HorizontalGradientRect.buildVertices(1)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:23)
- [ ] `HorizontalGradientRect.bounds(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:30)
- [ ] `HorizontalGradientRect.pipeline(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:11)
- [ ] `HorizontalGradientRect.textureSetup(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:11)
- [ ] `HorizontalGradientRect.pose(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:11)
- [ ] `HorizontalGradientRect.x0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:11)
- [ ] `HorizontalGradientRect.y0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:11)
- [ ] `HorizontalGradientRect.x1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:11)
- [ ] `HorizontalGradientRect.y1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:11)
- [ ] `HorizontalGradientRect.colorFrom(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:11)
- [ ] `HorizontalGradientRect.colorTo(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:11)
- [ ] `HorizontalGradientRect.scissorArea(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRect.java:11)

### `HorizontalGradientRectF`: it implements Minecraft's `GuiElementRenderState`

- [ ] `HorizontalGradientRectF.HorizontalGradientRectF(10)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:11)
- [ ] `HorizontalGradientRectF.buildVertices(1)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:23)
- [ ] `HorizontalGradientRectF.bounds(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:30)
- [ ] `HorizontalGradientRectF.pipeline(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:11)
- [ ] `HorizontalGradientRectF.textureSetup(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:11)
- [ ] `HorizontalGradientRectF.pose(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:11)
- [ ] `HorizontalGradientRectF.x0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:11)
- [ ] `HorizontalGradientRectF.y0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:11)
- [ ] `HorizontalGradientRectF.x1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:11)
- [ ] `HorizontalGradientRectF.y1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:11)
- [ ] `HorizontalGradientRectF.colorFrom(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:11)
- [ ] `HorizontalGradientRectF.colorTo(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:11)
- [ ] `HorizontalGradientRectF.scissorArea(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/HorizontalGradientRectF.java:11)

### `VerticalGradientRect`: it implements Minecraft's `GuiElementRenderState`

- [ ] `VerticalGradientRect.VerticalGradientRect(10)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:11)
- [ ] `VerticalGradientRect.buildVertices(1)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:23)
- [ ] `VerticalGradientRect.bounds(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:30)
- [ ] `VerticalGradientRect.pipeline(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:11)
- [ ] `VerticalGradientRect.textureSetup(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:11)
- [ ] `VerticalGradientRect.pose(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:11)
- [ ] `VerticalGradientRect.x0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:11)
- [ ] `VerticalGradientRect.y0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:11)
- [ ] `VerticalGradientRect.x1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:11)
- [ ] `VerticalGradientRect.y1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:11)
- [ ] `VerticalGradientRect.colorFrom(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:11)
- [ ] `VerticalGradientRect.colorTo(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:11)
- [ ] `VerticalGradientRect.scissorArea(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRect.java:11)

### `VerticalGradientRectF`: it implements Minecraft's `GuiElementRenderState`

- [ ] `VerticalGradientRectF.VerticalGradientRectF(10)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:11)
- [ ] `VerticalGradientRectF.buildVertices(1)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:23)
- [ ] `VerticalGradientRectF.bounds(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:30)
- [ ] `VerticalGradientRectF.pipeline(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:11)
- [ ] `VerticalGradientRectF.textureSetup(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:11)
- [ ] `VerticalGradientRectF.pose(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:11)
- [ ] `VerticalGradientRectF.x0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:11)
- [ ] `VerticalGradientRectF.y0(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:11)
- [ ] `VerticalGradientRectF.x1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:11)
- [ ] `VerticalGradientRectF.y1(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:11)
- [ ] `VerticalGradientRectF.colorFrom(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:11)
- [ ] `VerticalGradientRectF.colorTo(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:11)
- [ ] `VerticalGradientRectF.scissorArea(0)`: its class isn't translated (it implements Minecraft's `GuiElementRenderState`) (team/creative/creativecore/client/render/gui/VerticalGradientRectF.java:11)

### `ScissorStackExtender`: it's an interface

- [ ] `ScissorStackExtender.setOverrideScissor(1)`: it has no body (team/creative/creativecore/client/render/ScissorStackExtender.java:7)
- [ ] `ScissorStackExtender.clearOverrideScissor(0)`: it has no body (team/creative/creativecore/client/render/ScissorStackExtender.java:9)

### `CompiledText`: it uses Minecraft's `Component`

- [ ] `CompiledText.EMPTY`: its value couldn't be read: an anonymous class (team/creative/creativecore/client/render/text/CompiledText.java:29)
- [ ] `CompiledText.createAnySize(0)`: it uses CompiledText, which isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:71)
- [ ] `CompiledText.width(1)`: it uses Minecraft's `FormattedText` (team/creative/creativecore/client/render/text/CompiledText.java:75)
- [ ] `CompiledText.lineHeight(1)`: it uses Minecraft's `FormattedText` (team/creative/creativecore/client/render/text/CompiledText.java:81)
- [ ] `CompiledText.CompiledText(2)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:91)
- [ ] `CompiledText.setMaxHeight(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:97)
- [ ] `CompiledText.setDimension(2)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:102)
- [ ] `CompiledText.getMaxWidth(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:110)
- [ ] `CompiledText.getMaxHeight(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:114)
- [ ] `CompiledText.getDefaultColor(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:118)
- [ ] `CompiledText.setDefaultColor(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:122)
- [ ] `CompiledText.setShadow(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:126)
- [ ] `CompiledText.setAlign(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:130)
- [ ] `CompiledText.setVAlign(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:134)
- [ ] `CompiledText.setScale(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:138)
- [ ] `CompiledText.getScale(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:145)
- [ ] `CompiledText.setText(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:149)
- [ ] `CompiledText.setText(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:153)
- [ ] `CompiledText.compile(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:158)
- [ ] `CompiledText.compileNext(3)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:171)
- [ ] `CompiledText.compileNext(3)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:183)
- [ ] `CompiledText.compileNext(2)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:191)
- [ ] `CompiledText.getTotalHeight(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:211)
- [ ] `CompiledText.render(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:221)
- [ ] `CompiledText.splitByWidth(4)`: its body couldn't be read: an anonymous class (team/creative/creativecore/client/render/text/CompiledText.java:283)
- [ ] `CompiledText.getTotalWidth(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:376)
- [ ] `CompiledText.calculateWidth(3)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:380)
- [ ] `CompiledText.sameDimensions(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:393)
- [ ] `CompiledText.copy(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:404)
- [ ] `CompiledText.contains(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:424)
- [ ] `CompiledText.getUsedWidth(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:434)
- [ ] `CompiledText.getUsedHeight(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:438)
- [ ] `CompiledText.untrimmedContent(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/client/render/text/CompiledText.java:442)

### `CompiledText.CompiledLine`: it uses Minecraft's `FormattedText`

- [ ] `CompiledText.CompiledLine.contains(1)`: its class isn't translated (it uses Minecraft's `FormattedText`) (team/creative/creativecore/client/render/text/CompiledText.java:451)
- [ ] `CompiledText.CompiledLine.render(1)`: its class isn't translated (it uses Minecraft's `FormattedText`) (team/creative/creativecore/client/render/text/CompiledText.java:461)
- [ ] `CompiledText.CompiledLine.updateDimension(2)`: its class isn't translated (it uses Minecraft's `FormattedText`) (team/creative/creativecore/client/render/text/CompiledText.java:487)
- [ ] `CompiledText.CompiledLine.add(1)`: its class isn't translated (it uses Minecraft's `FormattedText`) (team/creative/creativecore/client/render/text/CompiledText.java:492)

### `CompiledText.FormattedTextSplit`: it uses Minecraft's `FormattedText`

- [ ] `CompiledText.FormattedTextSplit.FormattedTextSplit(2)`: its class isn't translated (it uses Minecraft's `FormattedText`) (team/creative/creativecore/client/render/text/CompiledText.java:521)
- [ ] `CompiledText.FormattedTextSplit.head(0)`: its class isn't translated (it uses Minecraft's `FormattedText`) (team/creative/creativecore/client/render/text/CompiledText.java:521)
- [ ] `CompiledText.FormattedTextSplit.tail(0)`: its class isn't translated (it uses Minecraft's `FormattedText`) (team/creative/creativecore/client/render/text/CompiledText.java:521)

### `DebugTextRenderer`

- [ ] `DebugTextRenderer.DECIMAL_FORMAT`: it uses `DecimalFormat` (java.text) (team/creative/creativecore/client/render/text/DebugTextRenderer.java:14)
- [ ] `DebugTextRenderer.format(1)`: it uses `Object` (team/creative/creativecore/client/render/text/DebugTextRenderer.java:17)
- [x] `DebugTextRenderer.DebugTextRenderer(0)` (team/creative/creativecore/client/render/text/DebugTextRenderer.java:21)
- [ ] `DebugTextRenderer.text(1)`: List.set (team/creative/creativecore/client/render/text/DebugTextRenderer.java:25)
- [ ] `DebugTextRenderer.detail(2)`: it uses `Object` (team/creative/creativecore/client/render/text/DebugTextRenderer.java:30)
- [ ] `DebugTextRenderer.newLine(0)`: `this` used as a value (team/creative/creativecore/client/render/text/DebugTextRenderer.java:46)
- [ ] `DebugTextRenderer.render(2)`: it uses Minecraft's `Font` (team/creative/creativecore/client/render/text/DebugTextRenderer.java:51)

### `Linebreaker`: its constants carry values

- [ ] `Linebreaker.Linebreaker(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/client/render/text/Linebreaker.java:17)
- [ ] `Linebreaker.Linebreaker(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/client/render/text/Linebreaker.java:21)
- [ ] `Linebreaker.Linebreaker(3)`: its class isn't translated (its constants carry values) (team/creative/creativecore/client/render/text/Linebreaker.java:25)
- [ ] `Linebreaker.Linebreaker(3)`: its class isn't translated (its constants carry values) (team/creative/creativecore/client/render/text/Linebreaker.java:29)

### `WidthLimitedCharSink`: it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)

- [ ] `WidthLimitedCharSink.WidthLimitedCharSink(2)`: its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)) (team/creative/creativecore/client/render/text/WidthLimitedCharSink.java:17)
- [ ] `WidthLimitedCharSink.accept(3)`: its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)) (team/creative/creativecore/client/render/text/WidthLimitedCharSink.java:23)
- [ ] `WidthLimitedCharSink.accept(2)`: its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)) (team/creative/creativecore/client/render/text/WidthLimitedCharSink.java:41)
- [ ] `WidthLimitedCharSink.getPosition(0)`: its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)) (team/creative/creativecore/client/render/text/WidthLimitedCharSink.java:47)
- [ ] `WidthLimitedCharSink.lastBreaker(0)`: its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)) (team/creative/creativecore/client/render/text/WidthLimitedCharSink.java:51)
- [ ] `WidthLimitedCharSink.lastBreakerPos(0)`: its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)) (team/creative/creativecore/client/render/text/WidthLimitedCharSink.java:61)
- [ ] `WidthLimitedCharSink.resetPosition(0)`: its class isn't translated (it implements `FormattedSingleSink` (team.creative.creativecore.common.util.text.content)) (team/creative/creativecore/client/render/text/WidthLimitedCharSink.java:71)

### `VertexFormatUtils`

- [x] `VertexFormatUtils.VERTEX_FORMAT_SIZE` (team/creative/creativecore/client/render/VertexFormatUtils.java:8)
- [x] `VertexFormatUtils.VERTEX_FORMAT_INT_SIZE` (team/creative/creativecore/client/render/VertexFormatUtils.java:9)
- [x] `VertexFormatUtils.POSITION_OFFSET` (team/creative/creativecore/client/render/VertexFormatUtils.java:10)
- [x] `VertexFormatUtils.UV_OFFSET` (team/creative/creativecore/client/render/VertexFormatUtils.java:11)
- [ ] `VertexFormatUtils.update(0)`: it uses Minecraft's `VertexFormat` (team/creative/creativecore/client/render/VertexFormatUtils.java:13)
- [x] `VertexFormatUtils.blockFormatSize(0)` (team/creative/creativecore/client/render/VertexFormatUtils.java:28)
- [x] `VertexFormatUtils.blockFormatIntSize(0)` (team/creative/creativecore/client/render/VertexFormatUtils.java:32)
- [x] `VertexFormatUtils.blockPositionOffset(0)` (team/creative/creativecore/client/render/VertexFormatUtils.java:36)
- [x] `VertexFormatUtils.blockUvOffset(0)` (team/creative/creativecore/client/render/VertexFormatUtils.java:40)

### `EntitySound`: it extends Minecraft's `AbstractTickableSoundInstance`

- [ ] `EntitySound.EntitySound(5)`: its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (team/creative/creativecore/client/sound/EntitySound.java:18)
- [ ] `EntitySound.tick(0)`: its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (team/creative/creativecore/client/sound/EntitySound.java:25)

### `SpecialSoundInstance`: it's an interface


### `VertexFormatExtender`: it's an interface

- [ ] `VertexFormatExtender.getOffset(1)`: it has no body (team/creative/creativecore/client/VertexFormatExtender.java:4)

### `CommonRegistry`: it uses Entry, which isn't translated (it's generic)

- [ ] `CommonRegistry.bindToNamespace(1)`: its class isn't translated (it uses Entry, which isn't translated (it's generic)) (team/creative/creativecore/CommonRegistry.java:21)
- [ ] `CommonRegistry.register(4)`: it's generic (team/creative/creativecore/CommonRegistry.java:25)
- [ ] `CommonRegistry.register(2)`: it's generic (team/creative/creativecore/CommonRegistry.java:33)
- [ ] `CommonRegistry.addEntries(1)`: its class isn't translated (it uses Entry, which isn't translated (it's generic)) (team/creative/creativecore/CommonRegistry.java:37)

### `CommonRegistry.BoundRegistry`: it's generic

- [ ] `CommonRegistry.BoundRegistry.BoundRegistry(3)`: its class isn't translated (it's generic) (team/creative/creativecore/CommonRegistry.java:48)
- [ ] `CommonRegistry.BoundRegistry.register(2)`: it's generic (team/creative/creativecore/CommonRegistry.java:54)
- [ ] `CommonRegistry.BoundRegistry.register(2)`: it's generic (team/creative/creativecore/CommonRegistry.java:58)
- [ ] `CommonRegistry.BoundRegistry.createTagKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/CommonRegistry.java:62)
- [ ] `CommonRegistry.BoundRegistry.register(2)`: it's generic (team/creative/creativecore/CommonRegistry.java:66)

### `CommonRegistry.BoundRegistry.Items`: it extends `BoundRegistry`

- [ ] `CommonRegistry.BoundRegistry.Items.Items(1)`: its class isn't translated (it extends `BoundRegistry`) (team/creative/creativecore/CommonRegistry.java:71)
- [ ] `CommonRegistry.BoundRegistry.Items.registerItem(2)`: it's generic (team/creative/creativecore/CommonRegistry.java:75)

### `CommonRegistry.DeferredHolder`: it's an interface

- [ ] `CommonRegistry.DeferredHolder.value(0)`: it has no body (team/creative/creativecore/CommonRegistry.java:82)
- [ ] `CommonRegistry.DeferredHolder.get(0)`: it has no body (team/creative/creativecore/CommonRegistry.java:85)

### `CommonRegistry.Entry`: it's generic

- [ ] `CommonRegistry.Entry.Entry(4)`: its class isn't translated (it's generic) (team/creative/creativecore/CommonRegistry.java:98)
- [ ] `CommonRegistry.Entry.bind(0)`: its class isn't translated (it's generic) (team/creative/creativecore/CommonRegistry.java:107)
- [ ] `CommonRegistry.Entry.computeValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/CommonRegistry.java:115)
- [ ] `CommonRegistry.Entry.value(0)`: its class isn't translated (it's generic) (team/creative/creativecore/CommonRegistry.java:119)
- [ ] `CommonRegistry.Entry.get(0)`: its class isn't translated (it's generic) (team/creative/creativecore/CommonRegistry.java:124)
- [ ] `CommonRegistry.Entry.toVanilla(0)`: its class isn't translated (it's generic) (team/creative/creativecore/CommonRegistry.java:129)

### `CommonRegistry.Holder`: it's an interface

- [ ] `CommonRegistry.Holder.value(0)`: it has no body (team/creative/creativecore/CommonRegistry.java:137)
- [ ] `CommonRegistry.Holder.toVanilla(0)`: it has no body (team/creative/creativecore/CommonRegistry.java:139)

### `CommonRegistry.NamespacedRegistry`

- [x] `CommonRegistry.NamespacedRegistry.NamespacedRegistry(1)` (team/creative/creativecore/CommonRegistry.java:145)
- [x] `CommonRegistry.NamespacedRegistry.namespace(0)` (team/creative/creativecore/CommonRegistry.java:149)
- [ ] `CommonRegistry.NamespacedRegistry.register(4)`: it's generic (team/creative/creativecore/CommonRegistry.java:153)
- [ ] `CommonRegistry.NamespacedRegistry.register(4)`: it's generic (team/creative/creativecore/CommonRegistry.java:161)
- [ ] `CommonRegistry.NamespacedRegistry.createItems(0)`: it uses Items, which isn't translated (it extends `BoundRegistry`) (team/creative/creativecore/CommonRegistry.java:167)
- [ ] `CommonRegistry.NamespacedRegistry.bindToRegistry(2)`: it's generic (team/creative/creativecore/CommonRegistry.java:171)
- [ ] `CommonRegistry.NamespacedRegistry.bindToRegistry(1)`: it's generic (team/creative/creativecore/CommonRegistry.java:175)
- [ ] `CommonRegistry.NamespacedRegistry.createTagKey(2)`: it's generic (team/creative/creativecore/CommonRegistry.java:179)

### `CommonRegistry.RegisterHelper`: it's an interface

- [ ] `CommonRegistry.RegisterHelper.register(3)`: it has no body (team/creative/creativecore/CommonRegistry.java:185)

### `CommonRegistry.UnitSupplier`: it's generic

- [ ] `CommonRegistry.UnitSupplier.UnitSupplier(1)`: its class isn't translated (it's generic) (team/creative/creativecore/CommonRegistry.java:188)
- [ ] `CommonRegistry.UnitSupplier.get(0)`: its class isn't translated (it's generic) (team/creative/creativecore/CommonRegistry.java:189)
- [ ] `CommonRegistry.UnitSupplier.instance(0)`: its class isn't translated (it's generic) (team/creative/creativecore/CommonRegistry.java:188)

### `CommonLoader`: it's an interface

- [ ] `CommonLoader.onInitialize(0)`: it has no body (team/creative/creativecore/common/CommonLoader.java:8)
- [ ] `CommonLoader.registerCommands(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/CommonLoader.java:10)

### `CreativeConfig`: it's an annotation

- [ ] `CreativeConfig.name(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:14)
- [ ] `CreativeConfig.type(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:16)
- [ ] `CreativeConfig.requiresRestart(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:18)
- [ ] `CreativeConfig.hideFromGUI(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:20)

### `CreativeConfig.DecimalRange`: it's an annotation

- [ ] `CreativeConfig.DecimalRange.min(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:25)
- [ ] `CreativeConfig.DecimalRange.max(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:27)
- [ ] `CreativeConfig.DecimalRange.slider(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:29)

### `CreativeConfig.DecimalRangeSupplier`: it's an annotation

- [ ] `CreativeConfig.DecimalRangeSupplier.supplier(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:35)
- [ ] `CreativeConfig.DecimalRangeSupplier.slider(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:37)

### `CreativeConfig.IntRange`: it's an annotation

- [ ] `CreativeConfig.IntRange.min(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:43)
- [ ] `CreativeConfig.IntRange.max(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:45)
- [ ] `CreativeConfig.IntRange.slider(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:47)

### `CreativeConfig.IntRangeSupplier`: it's an annotation

- [ ] `CreativeConfig.IntRangeSupplier.supplier(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:53)
- [ ] `CreativeConfig.IntRangeSupplier.slider(0)`: it has no body (team/creative/creativecore/common/config/api/CreativeConfig.java:55)

### `DecimalRangeSupplier`: it's an interface

- [ ] `DecimalRangeSupplier.getMin(0)`: it has no body (team/creative/creativecore/common/config/api/DecimalRangeSupplier.java:4)
- [ ] `DecimalRangeSupplier.getMax(0)`: it has no body (team/creative/creativecore/common/config/api/DecimalRangeSupplier.java:6)

### `IConfigObject`: it's an interface

- [ ] `IConfigObject.isDefault(1)`: it has no body (team/creative/creativecore/common/config/api/IConfigObject.java:6)
- [ ] `IConfigObject.restoreDefault(2)`: it has no body (team/creative/creativecore/common/config/api/IConfigObject.java:8)

### `ICreativeConfig`: it's an interface

- [ ] `ICreativeConfig.configured(1)`: it has no body (team/creative/creativecore/common/config/api/ICreativeConfig.java:6)

### `IntRangeSupplier`: it's an interface

- [ ] `IntRangeSupplier.getMin(0)`: it has no body (team/creative/creativecore/common/config/api/IntRangeSupplier.java:4)
- [ ] `IntRangeSupplier.getMax(0)`: it has no body (team/creative/creativecore/common/config/api/IntRangeSupplier.java:6)

### `ConfigTypeArray`: it extends `ConfigTypeConveration`

- [ ] `ConfigTypeArray.readElement(7)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeArray.java:17)
- [ ] `ConfigTypeArray.writeElement(6)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeArray.java:45)
- [ ] `ConfigTypeArray.createControls(4)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeArray.java:59)
- [ ] `ConfigTypeArray.loadValue(6)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeArray.java:64)
- [ ] `ConfigTypeArray.saveValue(4)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeArray.java:85)
- [ ] `ConfigTypeArray.set(2)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeArray.java:99)
- [ ] `ConfigTypeArray.areEqual(4)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeArray.java:104)

### `ConfigTypeConveration`: it's generic

- [ ] `ConfigTypeConveration.TYPE_CREATORS`: it uses `Class` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:69)
- [ ] `ConfigTypeConveration.TYPE_GETTERS`: it uses `Class` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:70)
- [ ] `ConfigTypeConveration.COLLECTION_CREATORS`: it uses `Class` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:71)
- [ ] `ConfigTypeConveration.TYPES`: it uses `Class` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:72)
- [ ] `ConfigTypeConveration.SPECIAL_TYPES`: it uses PairList, which isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:73)
- [ ] `ConfigTypeConveration.FAKE_PARENT`: its value couldn't be read: an anonymous class (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:74)
- [ ] `ConfigTypeConveration.isTypeCreator(1)`: it uses `Class` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:148)
- [ ] `ConfigTypeConveration.registerTypeCreator(2)`: it's generic (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:152)
- [ ] `ConfigTypeConveration.registerTypeCreator(2)`: it's generic (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:156)
- [ ] `ConfigTypeConveration.registerTypeGetter(2)`: it uses `Class` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:160)
- [ ] `ConfigTypeConveration.registerCollectionCreator(2)`: it's generic (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:164)
- [ ] `ConfigTypeConveration.registerType(2)`: it's generic (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:168)
- [ ] `ConfigTypeConveration.registerTypes(2)`: it's generic (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:173)
- [ ] `ConfigTypeConveration.registerSpecialType(2)`: it uses `Class` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:179)
- [ ] `ConfigTypeConveration.has(1)`: it uses `Class` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:183)
- [ ] `ConfigTypeConveration.get(1)`: it uses ConfigTypeConveration, which isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:201)
- [ ] `ConfigTypeConveration.getUnsafe(1)`: it uses ConfigTypeConveration, which isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:216)
- [ ] `ConfigTypeConveration.createObject(1)`: it uses `Object` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:231)
- [ ] `ConfigTypeConveration.getGenericType(1)`: it uses `Type` (java.lang.reflect) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:244)
- [ ] `ConfigTypeConveration.getGenericType(1)`: it uses `Type` (java.lang.reflect) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:260)
- [ ] `ConfigTypeConveration.createCollection(1)`: it uses `Object` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:264)
- [ ] `ConfigTypeConveration.createCollection(1)`: it uses `Object` (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:269)
- [ ] `ConfigTypeConveration.readElement(7)`: it has no body (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:273)
- [ ] `ConfigTypeConveration.writeElement(6)`: it has no body (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:275)
- [ ] `ConfigTypeConveration.createControls(4)`: it has no body (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:277)
- [ ] `ConfigTypeConveration.loadValue(6)`: it has no body (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:279)
- [ ] `ConfigTypeConveration.restoreDefault(5)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:281)
- [ ] `ConfigTypeConveration.shouldSave(5)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:285)
- [ ] `ConfigTypeConveration.saveValue(4)`: it has no body (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:289)
- [ ] `ConfigTypeConveration.set(2)`: it has no body (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:291)
- [ ] `ConfigTypeConveration.save(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:293)
- [ ] `ConfigTypeConveration.areEqual(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:298)

### `ConfigTypeConveration.SimpleConfigTypeConveration`: it's generic

- [ ] `ConfigTypeConveration.SimpleConfigTypeConveration.readElement(7)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:720)
- [ ] `ConfigTypeConveration.SimpleConfigTypeConveration.readElement(4)`: it has no body (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:725)
- [ ] `ConfigTypeConveration.SimpleConfigTypeConveration.writeElement(6)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:727)
- [ ] `ConfigTypeConveration.SimpleConfigTypeConveration.writeElement(3)`: it has no body (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:732)
- [ ] `ConfigTypeConveration.SimpleConfigTypeConveration.createControls(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:734)
- [ ] `ConfigTypeConveration.SimpleConfigTypeConveration.createControls(2)`: it has no body (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:739)
- [ ] `ConfigTypeConveration.SimpleConfigTypeConveration.loadValue(6)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:741)
- [ ] `ConfigTypeConveration.SimpleConfigTypeConveration.loadValue(2)`: it has no body (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:746)
- [ ] `ConfigTypeConveration.SimpleConfigTypeConveration.saveValue(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:748)
- [ ] `ConfigTypeConveration.SimpleConfigTypeConveration.saveValue(2)`: it has no body (team/creative/creativecore/common/config/converation/ConfigTypeConveration.java:753)

### `ConfigTypeConverationSided`

- [x] `ConfigTypeConverationSided.registerSide(0)` (team/creative/creativecore/common/config/converation/ConfigTypeConverationSided.java:4)

### `ConfigTypeList`: it extends `ConfigTypeConveration`

- [ ] `ConfigTypeList.readElement(7)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeList.java:19)
- [ ] `ConfigTypeList.writeElement(6)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeList.java:43)
- [ ] `ConfigTypeList.createControls(4)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeList.java:55)
- [ ] `ConfigTypeList.restoreDefault(5)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeList.java:72)
- [ ] `ConfigTypeList.loadValue(6)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeList.java:85)
- [ ] `ConfigTypeList.saveValue(4)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeList.java:105)
- [ ] `ConfigTypeList.set(2)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeList.java:118)
- [ ] `ConfigTypeList.createList(2)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeList.java:122)
- [ ] `ConfigTypeList.areEqual(4)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeList.java:126)

### `ConfigTypeNamedList`: it's generic

- [ ] `ConfigTypeNamedList.addToList(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeNamedList.java:19)
- [ ] `ConfigTypeNamedList.create(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeNamedList.java:23)
- [ ] `ConfigTypeNamedList.readElement(7)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeNamedList.java:28)
- [ ] `ConfigTypeNamedList.writeElement(6)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeNamedList.java:50)
- [ ] `ConfigTypeNamedList.createControls(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeNamedList.java:62)
- [ ] `ConfigTypeNamedList.restoreDefault(5)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeNamedList.java:79)
- [ ] `ConfigTypeNamedList.loadValue(6)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeNamedList.java:92)
- [ ] `ConfigTypeNamedList.saveValue(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeNamedList.java:118)
- [ ] `ConfigTypeNamedList.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeNamedList.java:131)
- [ ] `ConfigTypeNamedList.areEqual(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypeNamedList.java:135)

### `ConfigTypeNumber`

- [ ] `ConfigTypeNumber.NUMBER_FORMAT`: it uses `NumberFormat` (java.text) (team/creative/creativecore/common/config/converation/ConfigTypeNumber.java:28)
- [ ] `ConfigTypeNumber.INT_SUPPLIERS`: it uses `Class` (team/creative/creativecore/common/config/converation/ConfigTypeNumber.java:29)
- [ ] `ConfigTypeNumber.DECIMAL_SUPPLIERS`: it uses `Class` (team/creative/creativecore/common/config/converation/ConfigTypeNumber.java:30)
- [ ] `ConfigTypeNumber.getIntRangeSupplier(1)`: it uses IntRangeSupplier, which isn't translated (it's an interface) (team/creative/creativecore/common/config/converation/ConfigTypeNumber.java:32)
- [ ] `ConfigTypeNumber.getDecimalRangeSupplier(1)`: it uses DecimalRangeSupplier, which isn't translated (it's an interface) (team/creative/creativecore/common/config/converation/ConfigTypeNumber.java:45)
- [ ] `ConfigTypeNumber.createFormat(0)`: it uses `NumberFormat` (java.text) (team/creative/creativecore/common/config/converation/ConfigTypeNumber.java:58)
- [ ] `ConfigTypeNumber.init(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/config/converation/ConfigTypeNumber.java:65)

### `ConfigTypePermission`: it extends `ConfigTypeNamedList`

- [ ] `ConfigTypePermission.PERMISSION_DIALOG`: it uses GuiSyncGlobalLayer, which isn't translated (it's generic) (team/creative/creativecore/common/config/converation/ConfigTypePermission.java:17)
- [ ] `ConfigTypePermission.create(2)`: its class isn't translated (it extends `ConfigTypeNamedList`) (team/creative/creativecore/common/config/converation/ConfigTypePermission.java:20)
- [ ] `ConfigTypePermission.addToList(3)`: its class isn't translated (it extends `ConfigTypeNamedList`) (team/creative/creativecore/common/config/converation/ConfigTypePermission.java:25)
- [ ] `ConfigTypePermission.shouldSave(5)`: its class isn't translated (it extends `ConfigTypeNamedList`) (team/creative/creativecore/common/config/converation/ConfigTypePermission.java:29)
- [ ] `ConfigTypePermission.areEqual(4)`: its class isn't translated (it extends `ConfigTypeNamedList`) (team/creative/creativecore/common/config/converation/ConfigTypePermission.java:33)
- [ ] `ConfigTypePermission.createControls(4)`: its class isn't translated (it extends `ConfigTypeNamedList`) (team/creative/creativecore/common/config/converation/ConfigTypePermission.java:64)
- [ ] `ConfigTypePermission.loadValue(6)`: its class isn't translated (it extends `ConfigTypeNamedList`) (team/creative/creativecore/common/config/converation/ConfigTypePermission.java:69)
- [ ] `ConfigTypePermission.saveValue(4)`: its class isn't translated (it extends `ConfigTypeNamedList`) (team/creative/creativecore/common/config/converation/ConfigTypePermission.java:75)

### `ConfigTypePermission.GuiPermissionConfigButton`: it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)

- [ ] `ConfigTypePermission.GuiPermissionConfigButton.GuiPermissionConfigButton(6)`: its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/config/converation/ConfigTypePermission.java:87)
- [ ] `ConfigTypePermission.GuiPermissionConfigButton.setNewValue(1)`: its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/config/converation/ConfigTypePermission.java:103)

### `ConfigTypeToggleable`: it extends `ConfigTypeConveration`

- [ ] `ConfigTypeToggleable.readElement(7)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeToggleable.java:15)
- [ ] `ConfigTypeToggleable.writeElement(6)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeToggleable.java:29)
- [ ] `ConfigTypeToggleable.createControls(4)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeToggleable.java:38)
- [ ] `ConfigTypeToggleable.loadValue(6)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeToggleable.java:46)
- [ ] `ConfigTypeToggleable.saveValue(4)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeToggleable.java:53)
- [ ] `ConfigTypeToggleable.set(2)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeToggleable.java:59)
- [ ] `ConfigTypeToggleable.areEqual(4)`: its class isn't translated (it extends `ConfigTypeConveration`) (team/creative/creativecore/common/config/converation/ConfigTypeToggleable.java:63)

### `ConfigTypeRegistryObject`: it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)

- [ ] `ConfigTypeRegistryObject.readElement(7)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObject.java:15)
- [ ] `ConfigTypeRegistryObject.writeElement(6)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObject.java:23)
- [ ] `ConfigTypeRegistryObject.createControls(4)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObject.java:27)
- [ ] `ConfigTypeRegistryObject.loadValue(6)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObject.java:33)
- [ ] `ConfigTypeRegistryObject.saveValue(4)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObject.java:39)
- [ ] `ConfigTypeRegistryObject.set(2)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObject.java:44)

### `ConfigTypeRegistryObjectList`: it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)

- [ ] `ConfigTypeRegistryObjectList.readElement(7)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObjectList.java:23)
- [ ] `ConfigTypeRegistryObjectList.writeElement(6)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObjectList.java:40)
- [ ] `ConfigTypeRegistryObjectList.createControls(4)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObjectList.java:50)
- [ ] `ConfigTypeRegistryObjectList.loadValue(6)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObjectList.java:59)
- [ ] `ConfigTypeRegistryObjectList.saveValue(4)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObjectList.java:83)
- [ ] `ConfigTypeRegistryObjectList.set(2)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObjectList.java:94)
- [ ] `ConfigTypeRegistryObjectList.areEqual(4)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryObjectList.java:98)

### `ConfigTypeRegistryTag`: it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)

- [ ] `ConfigTypeRegistryTag.readElement(7)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTag.java:16)
- [ ] `ConfigTypeRegistryTag.writeElement(6)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTag.java:24)
- [ ] `ConfigTypeRegistryTag.createControls(4)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTag.java:28)
- [ ] `ConfigTypeRegistryTag.loadValue(6)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTag.java:34)
- [ ] `ConfigTypeRegistryTag.saveValue(4)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTag.java:38)
- [ ] `ConfigTypeRegistryTag.set(2)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTag.java:43)

### `ConfigTypeRegistryTagList`: it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)

- [ ] `ConfigTypeRegistryTagList.readElement(7)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTagList.java:24)
- [ ] `ConfigTypeRegistryTagList.writeElement(6)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTagList.java:41)
- [ ] `ConfigTypeRegistryTagList.createControls(4)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTagList.java:51)
- [ ] `ConfigTypeRegistryTagList.loadValue(6)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTagList.java:60)
- [ ] `ConfigTypeRegistryTagList.saveValue(4)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTagList.java:84)
- [ ] `ConfigTypeRegistryTagList.set(2)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTagList.java:95)
- [ ] `ConfigTypeRegistryTagList.areEqual(4)`: its class isn't translated (it extends `ConfigTypeConveration` (team.creative.creativecore.common.config.converation)) (team/creative/creativecore/common/config/converation/registry/ConfigTypeRegistryTagList.java:99)

### `GuiRegistryObjectHandler`

- [ ] `GuiRegistryObjectHandler.REGISTRY`: its value couldn't be read: an anonymous class (team/creative/creativecore/common/config/converation/registry/GuiRegistryObjectHandler.java:17)
- [ ] `GuiRegistryObjectHandler.createControls(2)`: it has no body (team/creative/creativecore/common/config/converation/registry/GuiRegistryObjectHandler.java:47)
- [ ] `GuiRegistryObjectHandler.loadValue(3)`: it has no body (team/creative/creativecore/common/config/converation/registry/GuiRegistryObjectHandler.java:49)
- [ ] `GuiRegistryObjectHandler.saveValue(2)`: it has no body (team/creative/creativecore/common/config/converation/registry/GuiRegistryObjectHandler.java:51)

### `GuiRegistryTagHandler`

- [ ] `GuiRegistryTagHandler.REGISTRY`: its value couldn't be read: an anonymous class (team/creative/creativecore/common/config/converation/registry/GuiRegistryTagHandler.java:30)
- [ ] `GuiRegistryTagHandler.createControls(2)`: it has no body (team/creative/creativecore/common/config/converation/registry/GuiRegistryTagHandler.java:61)
- [ ] `GuiRegistryTagHandler.loadValue(3)`: it has no body (team/creative/creativecore/common/config/converation/registry/GuiRegistryTagHandler.java:63)
- [ ] `GuiRegistryTagHandler.saveValue(2)`: it has no body (team/creative/creativecore/common/config/converation/registry/GuiRegistryTagHandler.java:65)

### `ConfigEqualChecker`: it uses `Class`

- [ ] `ConfigEqualChecker.register(2)`: its class isn't translated (it uses `Class`) (team/creative/creativecore/common/config/core/ConfigEqualChecker.java:19)
- [ ] `ConfigEqualChecker.equals(4)`: its class isn't translated (it uses `Class`) (team/creative/creativecore/common/config/core/ConfigEqualChecker.java:56)

### `IConfigRegistry`: it's an interface

- [ ] `IConfigRegistry.getEqualChecker(0)`: it has no body (team/creative/creativecore/common/config/core/IConfigRegistry.java:8)
- [ ] `IConfigRegistry.is(1)`: it has no body (team/creative/creativecore/common/config/core/IConfigRegistry.java:10)
- [ ] `IConfigRegistry.is(2)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/config/core/IConfigRegistry.java:12)
- [ ] `IConfigRegistry.equals(3)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/config/core/IConfigRegistry.java:21)

### `ConfigEventHandler`: it uses `DecimalFormat` (java.text)

- [ ] `ConfigEventHandler.client(0)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:50)
- [ ] `ConfigEventHandler.ConfigEventHandler(2)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:55)
- [ ] `ConfigEventHandler.loadClientFieldList(1)`: it uses ICreativeConfigHolder, which isn't translated (it's an interface) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:65)
- [ ] `ConfigEventHandler.loadClientFieldList(3)`: it uses ICreativeConfigHolder, which isn't translated (it's an interface) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:77)
- [ ] `ConfigEventHandler.saveClientFieldList(2)`: it uses ICreativeConfigHolder, which isn't translated (it's an interface) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:94)
- [ ] `ConfigEventHandler.saveClientFieldList(3)`: it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:102)
- [ ] `ConfigEventHandler.enable(1)`: it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:122)
- [ ] `ConfigEventHandler.generateFormat(0)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:133)
- [ ] `ConfigEventHandler.playerLoggedIn(3)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:139)
- [ ] `ConfigEventHandler.isOwner(1)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:146)
- [ ] `ConfigEventHandler.sync(2)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:150)
- [ ] `ConfigEventHandler.sync(2)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:154)
- [ ] `ConfigEventHandler.syncAll(1)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:158)
- [ ] `ConfigEventHandler.syncAll(1)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:162)
- [ ] `ConfigEventHandler.saveClientFields(0)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:166)
- [ ] `ConfigEventHandler.save(3)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/config/event/ConfigEventHandler.java:192)
- [ ] `ConfigEventHandler.save(2)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:230)
- [ ] `ConfigEventHandler.loadClientFields(0)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:236)
- [ ] `ConfigEventHandler.load(3)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:266)
- [ ] `ConfigEventHandler.load(2)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:295)
- [ ] `ConfigEventHandler.isSynchronizedWithServer(1)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:305)
- [ ] `ConfigEventHandler.modFileExist(2)`: its class isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/common/config/event/ConfigEventHandler.java:311)

### `ConfigField`: it's an interface

- [ ] `ConfigField.get(0)`: it has no body (team/creative/creativecore/common/config/field/ConfigField.java:7)
- [ ] `ConfigField.set(1)`: it has no body (team/creative/creativecore/common/config/field/ConfigField.java:9)
- [ ] `ConfigField.getAnnotation(1)`: it has no body (team/creative/creativecore/common/config/field/ConfigField.java:11)
- [ ] `ConfigField.getGenericType(0)`: it has no body (team/creative/creativecore/common/config/field/ConfigField.java:13)
- [ ] `ConfigField.getType(0)`: it has no body (team/creative/creativecore/common/config/field/ConfigField.java:15)

### `ConfigFieldDynamic`: it implements `ConfigField`

- [ ] `ConfigFieldDynamic.ConfigFieldDynamic(1)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldDynamic.java:9)
- [ ] `ConfigFieldDynamic.get(0)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldDynamic.java:13)
- [ ] `ConfigFieldDynamic.set(1)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldDynamic.java:18)
- [ ] `ConfigFieldDynamic.getAnnotation(1)`: it's generic (team/creative/creativecore/common/config/field/ConfigFieldDynamic.java:23)
- [ ] `ConfigFieldDynamic.getGenericType(0)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldDynamic.java:28)
- [ ] `ConfigFieldDynamic.getType(0)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldDynamic.java:33)

### `ConfigFieldTyped`: it implements `ConfigField`

- [ ] `ConfigFieldTyped.ConfigFieldTyped(3)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldTyped.java:11)
- [ ] `ConfigFieldTyped.get(0)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldTyped.java:17)
- [ ] `ConfigFieldTyped.set(1)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldTyped.java:22)
- [ ] `ConfigFieldTyped.getAnnotation(1)`: it's generic (team/creative/creativecore/common/config/field/ConfigFieldTyped.java:27)
- [ ] `ConfigFieldTyped.getGenericType(0)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldTyped.java:32)
- [ ] `ConfigFieldTyped.getType(0)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldTyped.java:37)

### `ConfigFieldWrapper`: it implements `ConfigField`

- [ ] `ConfigFieldWrapper.ConfigFieldWrapper(2)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldWrapper.java:7)
- [ ] `ConfigFieldWrapper.get(0)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldWrapper.java:8)
- [ ] `ConfigFieldWrapper.set(1)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldWrapper.java:17)
- [ ] `ConfigFieldWrapper.getAnnotation(1)`: it's generic (team/creative/creativecore/common/config/field/ConfigFieldWrapper.java:26)
- [ ] `ConfigFieldWrapper.getGenericType(0)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldWrapper.java:31)
- [ ] `ConfigFieldWrapper.getType(0)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldWrapper.java:36)
- [ ] `ConfigFieldWrapper.parent(0)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldWrapper.java:7)
- [ ] `ConfigFieldWrapper.field(0)`: its class isn't translated (it implements `ConfigField`) (team/creative/creativecore/common/config/field/ConfigFieldWrapper.java:7)

### `Usergroup`

- [x] `Usergroup.Usergroup(0)` (team/creative/creativecore/common/config/group/Usergroup.java:14)
- [x] `Usergroup.Usergroup(1)` (team/creative/creativecore/common/config/group/Usergroup.java:18)
- [ ] `Usergroup.is(1)`: it uses Minecraft's `Player` (team/creative/creativecore/common/config/group/Usergroup.java:22)

### `ClientSyncGuiLayer`: it extends `GuiLayer` (team.creative.creativecore.common.gui)

- [ ] `ClientSyncGuiLayer.ClientSyncGuiLayer(2)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/ClientSyncGuiLayer.java:40)
- [ ] `ClientSyncGuiLayer.create(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/ClientSyncGuiLayer.java:79)
- [ ] `ClientSyncGuiLayer.save(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/ClientSyncGuiLayer.java:84)
- [ ] `ClientSyncGuiLayer.load(1)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/ClientSyncGuiLayer.java:88)
- [ ] `ClientSyncGuiLayer.closeTopLayer(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/ClientSyncGuiLayer.java:143)

### `ClientSyncGuiLayer.GuiTreeCheckBox`: it extends `GuiCheckBox` (team.creative.creativecore.common.gui.control.simple)

- [ ] `ClientSyncGuiLayer.GuiTreeCheckBox.GuiTreeCheckBox(2)`: its class isn't translated (it extends `GuiCheckBox` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/config/gui/ClientSyncGuiLayer.java:166)

### `ConfigGuiLayer`: it extends `GuiLayer` (team.creative.creativecore.common.gui)

- [ ] `ConfigGuiLayer.ConfigGuiLayer(3)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/ConfigGuiLayer.java:35)
- [ ] `ConfigGuiLayer.create(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/ConfigGuiLayer.java:43)
- [ ] `ConfigGuiLayer.savePage(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/ConfigGuiLayer.java:48)
- [ ] `ConfigGuiLayer.loadHolder(1)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/ConfigGuiLayer.java:66)
- [ ] `ConfigGuiLayer.sendUpdate(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/ConfigGuiLayer.java:134)
- [ ] `ConfigGuiLayer.closeTopLayer(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/ConfigGuiLayer.java:143)

### `FullItemDialogGuiLayer`: it extends `GuiLayer` (team.creative.creativecore.common.gui)

- [x] `FullItemDialogGuiLayer.latest` (team/creative/creativecore/common/config/gui/FullItemDialogGuiLayer.java:22)
- [ ] `FullItemDialogGuiLayer.FullItemDialogGuiLayer(1)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/FullItemDialogGuiLayer.java:26)
- [ ] `FullItemDialogGuiLayer.create(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/FullItemDialogGuiLayer.java:38)

### `GuiConfigControl`: it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)

- [ ] `GuiConfigControl.GuiConfigControl(5)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/config/gui/GuiConfigControl.java:25)
- [ ] `GuiConfigControl.GuiConfigControl(5)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/config/gui/GuiConfigControl.java:29)
- [ ] `GuiConfigControl.GuiConfigControl(7)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/config/gui/GuiConfigControl.java:33)
- [ ] `GuiConfigControl.isDefault(0)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/config/gui/GuiConfigControl.java:61)
- [ ] `GuiConfigControl.updateButton(0)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/config/gui/GuiConfigControl.java:65)
- [ ] `GuiConfigControl.init(1)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/config/gui/GuiConfigControl.java:71)
- [ ] `GuiConfigControl.reset(0)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/config/gui/GuiConfigControl.java:90)
- [ ] `GuiConfigControl.changed(0)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/config/gui/GuiConfigControl.java:95)
- [ ] `GuiConfigControl.save(0)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/config/gui/GuiConfigControl.java:100)
- [ ] `GuiConfigControl.setCustomData(1)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/config/gui/GuiConfigControl.java:107)
- [ ] `GuiConfigControl.getCustomData(0)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/config/gui/GuiConfigControl.java:112)

### `GuiConfigSubControl`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiConfigSubControl.GuiConfigSubControl(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/GuiConfigSubControl.java:16)
- [ ] `GuiConfigSubControl.getName(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/GuiConfigSubControl.java:22)
- [ ] `GuiConfigSubControl.addNameUnmodifieable(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/GuiConfigSubControl.java:30)
- [ ] `GuiConfigSubControl.addNameTextfield(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/GuiConfigSubControl.java:35)

### `GuiConfigSubControlHolder`: it extends `GuiConfigSubControl`

- [ ] `GuiConfigSubControlHolder.GuiConfigSubControlHolder(6)`: its class isn't translated (it extends `GuiConfigSubControl`) (team/creative/creativecore/common/config/gui/GuiConfigSubControlHolder.java:20)
- [ ] `GuiConfigSubControlHolder.load(2)`: its class isn't translated (it extends `GuiConfigSubControl`) (team/creative/creativecore/common/config/gui/GuiConfigSubControlHolder.java:30)
- [ ] `GuiConfigSubControlHolder.createControls(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/config/gui/GuiConfigSubControlHolder.java:35)
- [ ] `GuiConfigSubControlHolder.save(0)`: its class isn't translated (it extends `GuiConfigSubControl`) (team/creative/creativecore/common/config/gui/GuiConfigSubControlHolder.java:60)

### `GuiConfigSubControlNested`: it extends `GuiConfigSubControl`

- [ ] `GuiConfigSubControlNested.GuiConfigSubControlNested(7)`: its class isn't translated (it extends `GuiConfigSubControl`) (team/creative/creativecore/common/config/gui/GuiConfigSubControlNested.java:27)
- [ ] `GuiConfigSubControlNested.load(2)`: its class isn't translated (it extends `GuiConfigSubControl`) (team/creative/creativecore/common/config/gui/GuiConfigSubControlNested.java:43)
- [ ] `GuiConfigSubControlNested.addNameUnmodifieable(1)`: its class isn't translated (it extends `GuiConfigSubControl`) (team/creative/creativecore/common/config/gui/GuiConfigSubControlNested.java:48)
- [ ] `GuiConfigSubControlNested.addNameTextfield(1)`: its class isn't translated (it extends `GuiConfigSubControl`) (team/creative/creativecore/common/config/gui/GuiConfigSubControlNested.java:54)
- [ ] `GuiConfigSubControlNested.createControls(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/config/gui/GuiConfigSubControlNested.java:59)
- [ ] `GuiConfigSubControlNested.save(0)`: its class isn't translated (it extends `GuiConfigSubControl`) (team/creative/creativecore/common/config/gui/GuiConfigSubControlNested.java:93)

### `GuiInfoStackButton`: it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)

- [ ] `GuiInfoStackButton.ITEM_DIALOG`: it uses GuiSyncGlobalLayer, which isn't translated (it's generic) (team/creative/creativecore/common/config/gui/GuiInfoStackButton.java:15)
- [ ] `GuiInfoStackButton.GuiInfoStackButton(3)`: its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/config/gui/GuiInfoStackButton.java:19)
- [ ] `GuiInfoStackButton.set(1)`: its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/config/gui/GuiInfoStackButton.java:30)
- [ ] `GuiInfoStackButton.getLabelText(1)`: it uses TextBuilder, which isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/config/gui/GuiInfoStackButton.java:36)
- [ ] `GuiInfoStackButton.get(0)`: its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/config/gui/GuiInfoStackButton.java:46)

### `GuiPlayerSelectorButton`: it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)

- [ ] `GuiPlayerSelectorButton.PLAYER_SELECTOR`: it uses GuiSyncGlobalLayer, which isn't translated (it's generic) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorButton.java:13)
- [ ] `GuiPlayerSelectorButton.GuiPlayerSelectorButton(3)`: its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorButton.java:17)
- [ ] `GuiPlayerSelectorButton.set(1)`: its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorButton.java:28)
- [ ] `GuiPlayerSelectorButton.getLabelText(1)`: it calls PlayerSelector.info, which isn't translated (team/creative/creativecore/common/config/gui/GuiPlayerSelectorButton.java:34)
- [ ] `GuiPlayerSelectorButton.get(0)`: its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorButton.java:38)

### `GuiPlayerSelectorHandler`: it's generic

- [ ] `GuiPlayerSelectorHandler.REGISTRY`: it uses NamedHandlerRegistry, which isn't translated (it's generic) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java:20)
- [ ] `GuiPlayerSelectorHandler.get(1)`: it uses GuiPlayerSelectorHandler, which isn't translated (it's generic) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java:22)
- [ ] `GuiPlayerSelectorHandler.getName(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java:26)
- [ ] `GuiPlayerSelectorHandler.createControls(2)`: it has no body (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java:30)
- [ ] `GuiPlayerSelectorHandler.parseSelector(1)`: it has no body (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java:32)
- [ ] `GuiPlayerSelectorHandler.onChanged(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java:34)

### `GuiPlayerSelectorHandler.GuiPlayerSelectorHandlerMultiple`: it's generic

- [ ] `GuiPlayerSelectorHandler.GuiPlayerSelectorHandlerMultiple.getChildren(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java:119)
- [ ] `GuiPlayerSelectorHandler.GuiPlayerSelectorHandlerMultiple.createControls(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java:127)
- [ ] `GuiPlayerSelectorHandler.GuiPlayerSelectorHandlerMultiple.parseSelector(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java:145)
- [ ] `GuiPlayerSelectorHandler.GuiPlayerSelectorHandlerMultiple.parseSelector(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/gui/GuiPlayerSelectorHandler.java:157)

### `IGuiConfigParent`: it's an interface

- [ ] `IGuiConfigParent.provider(0)`: it has no body (team/creative/creativecore/common/config/gui/IGuiConfigParent.java:7)
- [ ] `IGuiConfigParent.setCustomData(1)`: it has no body (team/creative/creativecore/common/config/gui/IGuiConfigParent.java:9)
- [ ] `IGuiConfigParent.getCustomData(0)`: it has no body (team/creative/creativecore/common/config/gui/IGuiConfigParent.java:11)
- [ ] `IGuiConfigParent.changed(0)`: it has no body (team/creative/creativecore/common/config/gui/IGuiConfigParent.java:13)

### `PermissionGuiLayer`: it extends `GuiLayer` (team.creative.creativecore.common.gui)

- [ ] `PermissionGuiLayer.PermissionGuiLayer(1)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:46)
- [ ] `PermissionGuiLayer.create(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:50)
- [ ] `PermissionGuiLayer.addGroupHeader(3)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:196)
- [ ] `PermissionGuiLayer.closeTopLayer(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:227)
- [ ] `PermissionGuiLayer.save(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:244)

### `PermissionGuiLayer.PermissionGuiGroup`: it uses GuiComboBox, which isn't translated (it's generic)

- [ ] `PermissionGuiLayer.PermissionGuiGroup.PermissionGuiGroup(1)`: its class isn't translated (it uses GuiComboBox, which isn't translated (it's generic)) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:259)
- [ ] `PermissionGuiLayer.PermissionGuiGroup.getTitle(0)`: its class isn't translated (it uses GuiComboBox, which isn't translated (it's generic)) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:263)
- [ ] `PermissionGuiLayer.PermissionGuiGroup.updateResetButton(0)`: its class isn't translated (it uses GuiComboBox, which isn't translated (it's generic)) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:267)
- [ ] `PermissionGuiLayer.PermissionGuiGroup.save(0)`: it has no body (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:273)
- [ ] `PermissionGuiLayer.PermissionGuiGroup.reset(0)`: its class isn't translated (it uses GuiComboBox, which isn't translated (it's generic)) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:275)
- [ ] `PermissionGuiLayer.PermissionGuiGroup.resetInternal(0)`: it has no body (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:284)
- [ ] `PermissionGuiLayer.PermissionGuiGroup.isDefault(0)`: it has no body (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:286)
- [ ] `PermissionGuiLayer.PermissionGuiGroup.isDefaultGroup(0)`: its class isn't translated (it uses GuiComboBox, which isn't translated (it's generic)) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:288)

### `PermissionGuiLayer.PermissionGuiGroupMulti`: it extends `PermissionGuiGroup`

- [ ] `PermissionGuiLayer.PermissionGuiGroupMulti.PermissionGuiGroupMulti(3)`: its class isn't translated (it extends `PermissionGuiGroup`) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:299)
- [ ] `PermissionGuiLayer.PermissionGuiGroupMulti.save(0)`: its class isn't translated (it extends `PermissionGuiGroup`) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:305)
- [ ] `PermissionGuiLayer.PermissionGuiGroupMulti.resetInternal(0)`: its class isn't translated (it extends `PermissionGuiGroup`) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:320)
- [ ] `PermissionGuiLayer.PermissionGuiGroupMulti.isDefault(0)`: its class isn't translated (it extends `PermissionGuiGroup`) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:327)

### `PermissionGuiLayer.PermissionGuiGroupSimple`: it extends `PermissionGuiGroup`

- [ ] `PermissionGuiLayer.PermissionGuiGroupSimple.PermissionGuiGroupSimple(2)`: its class isn't translated (it extends `PermissionGuiGroup`) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:338)
- [ ] `PermissionGuiLayer.PermissionGuiGroupSimple.resetInternal(0)`: its class isn't translated (it extends `PermissionGuiGroup`) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:343)
- [ ] `PermissionGuiLayer.PermissionGuiGroupSimple.isDefault(0)`: its class isn't translated (it extends `PermissionGuiGroup`) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:351)
- [ ] `PermissionGuiLayer.PermissionGuiGroupSimple.save(0)`: its class isn't translated (it extends `PermissionGuiGroup`) (team/creative/creativecore/common/config/gui/PermissionGuiLayer.java:358)

### `PlayerSelectorDialog`: it extends `GuiLayer` (team.creative.creativecore.common.gui)

- [ ] `PlayerSelectorDialog.PlayerSelectorDialog(1)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/PlayerSelectorDialog.java:18)
- [ ] `PlayerSelectorDialog.create(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/config/gui/PlayerSelectorDialog.java:34)

### `ConfigHolder`: it's generic

- [ ] `ConfigHolder.ConfigHolder(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:19)
- [ ] `ConfigHolder.ConfigHolder(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:25)
- [ ] `ConfigHolder.synchronization(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:31)
- [ ] `ConfigHolder.parent(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:36)
- [ ] `ConfigHolder.path(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:41)
- [ ] `ConfigHolder.getField(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:46)
- [ ] `ConfigHolder.fields(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:51)
- [ ] `ConfigHolder.names(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:56)
- [ ] `ConfigHolder.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:61)
- [ ] `ConfigHolder.isEmpty(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:67)
- [ ] `ConfigHolder.isEmptyWithoutForce(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:83)
- [ ] `ConfigHolder.isDefault(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:99)
- [ ] `ConfigHolder.restoreDefault(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:110)
- [ ] `ConfigHolder.load(5)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:120)
- [ ] `ConfigHolder.save(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:136)
- [ ] `ConfigHolder.getRegistry(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/holder/ConfigHolder.java:150)

### `ConfigHolderDynamic`: it extends `ConfigHolder`

- [ ] `ConfigHolderDynamic.ConfigHolderDynamic(3)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:13)
- [ ] `ConfigHolderDynamic.ConfigHolderDynamic(0)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:17)
- [ ] `ConfigHolderDynamic.clear(0)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:20)
- [ ] `ConfigHolderDynamic.registerFolder(1)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:24)
- [ ] `ConfigHolderDynamic.registerFolder(2)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:28)
- [ ] `ConfigHolderDynamic.registerValue(2)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:43)
- [ ] `ConfigHolderDynamic.registerValue(4)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:47)
- [ ] `ConfigHolderDynamic.registerValue(5)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:51)
- [ ] `ConfigHolderDynamic.registerField(3)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:71)
- [ ] `ConfigHolderDynamic.registerField(5)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:75)
- [ ] `ConfigHolderDynamic.registerField(6)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:79)
- [ ] `ConfigHolderDynamic.configured(1)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderDynamic.java:92)

### `ConfigHolderObject`: it extends `ConfigHolder`

- [ ] `ConfigHolderObject.createUnrelated(3)`: it uses ConfigHolderObject, which isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderObject.java:20)
- [ ] `ConfigHolderObject.createUnrelated(4)`: it uses ConfigHolderObject, which isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderObject.java:24)
- [ ] `ConfigHolderObject.collectFields(3)`: it uses `Field` (java.lang.reflect) (team/creative/creativecore/common/config/holder/ConfigHolderObject.java:35)
- [ ] `ConfigHolderObject.ConfigHolderObject(4)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderObject.java:51)
- [ ] `ConfigHolderObject.ConfigHolderObject(5)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderObject.java:55)
- [ ] `ConfigHolderObject.ConfigHolderObject(6)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderObject.java:59)
- [ ] `ConfigHolderObject.getRegistry(0)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderObject.java:96)
- [ ] `ConfigHolderObject.restoreDefault(2)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderObject.java:101)
- [ ] `ConfigHolderObject.load(5)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderObject.java:107)
- [ ] `ConfigHolderObject.configured(1)`: its class isn't translated (it extends `ConfigHolder`) (team/creative/creativecore/common/config/holder/ConfigHolderObject.java:113)

### `CreativeConfigRegistry`: it extends `ConfigHolderDynamic`

- [ ] `CreativeConfigRegistry.ROOT`: it uses CreativeConfigRegistry, which isn't translated (it extends `ConfigHolderDynamic`) (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:16)
- [ ] `CreativeConfigRegistry.FIELD_PREDICATE`: it uses `Field` (java.lang.reflect) (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:17)
- [ ] `CreativeConfigRegistry.EQUAL_CHECKER`: it uses ConfigEqualChecker, which isn't translated (it uses `Class`) (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:20)
- [ ] `CreativeConfigRegistry.registerValue(2)`: its class isn't translated (it extends `ConfigHolderDynamic`) (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:22)
- [ ] `CreativeConfigRegistry.registerValue(4)`: its class isn't translated (it extends `ConfigHolderDynamic`) (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:27)
- [ ] `CreativeConfigRegistry.followPath(1)`: its class isn't translated (it extends `ConfigHolderDynamic`) (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:32)
- [ ] `CreativeConfigRegistry.findKey(1)`: its class isn't translated (it extends `ConfigHolderDynamic`) (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:47)
- [ ] `CreativeConfigRegistry.removeField(1)`: its class isn't translated (it extends `ConfigHolderDynamic`) (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:66)
- [ ] `CreativeConfigRegistry.load(3)`: it uses Minecraft's `Provider` (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:70)
- [ ] `CreativeConfigRegistry.getEqualChecker(0)`: its class isn't translated (it extends `ConfigHolderDynamic`) (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:74)
- [ ] `CreativeConfigRegistry.is(1)`: its class isn't translated (it extends `ConfigHolderDynamic`) (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:79)
- [ ] `CreativeConfigRegistry.getRegistry(0)`: its class isn't translated (it extends `ConfigHolderDynamic`) (team/creative/creativecore/common/config/holder/CreativeConfigRegistry.java:84)

### `ICreativeConfigHolder`: it's an interface

- [ ] `ICreativeConfigHolder.read(6)`: it uses Minecraft's `Provider` (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:14)
- [ ] `ICreativeConfigHolder.write(5)`: it uses Minecraft's `Provider` (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:22)
- [ ] `ICreativeConfigHolder.parent(0)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:26)
- [ ] `ICreativeConfigHolder.getName(0)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:28)
- [ ] `ICreativeConfigHolder.path(0)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:32)
- [ ] `ICreativeConfigHolder.fields(0)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:34)
- [ ] `ICreativeConfigHolder.names(0)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:36)
- [ ] `ICreativeConfigHolder.get(1)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:38)
- [ ] `ICreativeConfigHolder.getField(1)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:40)
- [ ] `ICreativeConfigHolder.load(5)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:42)
- [ ] `ICreativeConfigHolder.save(4)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:44)
- [ ] `ICreativeConfigHolder.isEmpty(1)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:46)
- [ ] `ICreativeConfigHolder.isEmptyWithoutForce(1)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:48)
- [ ] `ICreativeConfigHolder.synchronization(0)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:50)
- [ ] `ICreativeConfigHolder.getRegistry(0)`: it has no body (team/creative/creativecore/common/config/holder/ICreativeConfigHolder.java:52)

### `ConfigKey`: it uses ConfigSynchronization, which isn't translated (its constants carry values)

- [ ] `ConfigKey.of(8)`: it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:29)
- [ ] `ConfigKey.of(7)`: it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:42)
- [ ] `ConfigKey.ofGenericType(2)`: it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:60)
- [ ] `ConfigKey.ofArrayType(2)`: it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:67)
- [ ] `ConfigKey.ofType(3)`: it uses ConfigKey, which isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:74)
- [ ] `ConfigKey.ConfigKey(5)`: its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:83)
- [ ] `ConfigKey.get(0)`: its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:91)
- [ ] `ConfigKey.hashCode(0)`: its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:95)
- [ ] `ConfigKey.equals(1)`: its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:100)
- [ ] `ConfigKey.toString(0)`: its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:105)
- [ ] `ConfigKey.checkEqual(3)`: its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:110)
- [ ] `ConfigKey.isFolder(0)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:114)
- [ ] `ConfigKey.holder(0)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:116)
- [ ] `ConfigKey.converation(0)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:118)
- [ ] `ConfigKey.is(1)`: its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:120)
- [ ] `ConfigKey.isWithoutForce(1)`: its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:124)
- [ ] `ConfigKey.triggerConfigured(1)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:128)
- [ ] `ConfigKey.isDefault(1)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:130)
- [ ] `ConfigKey.isDefault(2)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:132)
- [ ] `ConfigKey.restoreDefault(2)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:134)
- [ ] `ConfigKey.forceValue(2)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:136)
- [ ] `ConfigKey.read(5)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:138)
- [ ] `ConfigKey.write(4)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:140)
- [ ] `ConfigKey.copy(2)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:142)
- [ ] `ConfigKey.create(3)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:144)
- [ ] `ConfigKey.load(3)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:146)
- [ ] `ConfigKey.save(3)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:148)
- [ ] `ConfigKey.field(0)`: its class isn't translated (it uses ConfigSynchronization, which isn't translated (its constants carry values)) (team/creative/creativecore/common/config/key/ConfigKey.java:150)
- [ ] `ConfigKey.getRegistry(0)`: it has no body (team/creative/creativecore/common/config/key/ConfigKey.java:154)

### `ConfigKeyHolder`: it extends `ConfigKey`

- [ ] `ConfigKeyHolder.ConfigKeyHolder(6)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:19)
- [ ] `ConfigKeyHolder.triggerConfigured(1)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:26)
- [ ] `ConfigKeyHolder.isDefault(1)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:31)
- [ ] `ConfigKeyHolder.isDefault(2)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:36)
- [ ] `ConfigKeyHolder.restoreDefault(2)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:41)
- [ ] `ConfigKeyHolder.isFolder(0)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:46)
- [ ] `ConfigKeyHolder.holder(0)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:51)
- [ ] `ConfigKeyHolder.copy(2)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:56)
- [ ] `ConfigKeyHolder.read(5)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:64)
- [ ] `ConfigKeyHolder.write(4)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:69)
- [ ] `ConfigKeyHolder.forceValue(2)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:74)
- [ ] `ConfigKeyHolder.converation(0)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:80)
- [ ] `ConfigKeyHolder.create(3)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:85)
- [ ] `ConfigKeyHolder.load(3)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:90)
- [ ] `ConfigKeyHolder.save(3)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:97)
- [ ] `ConfigKeyHolder.getRegistry(0)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyHolder.java:104)

### `ConfigKeyType`: it extends `ConfigKey`

- [ ] `ConfigKeyType.ConfigKeyType(7)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:21)
- [ ] `ConfigKeyType.set(2)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:36)
- [ ] `ConfigKeyType.checkEqual(3)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:42)
- [ ] `ConfigKeyType.isDefault(1)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:47)
- [ ] `ConfigKeyType.isDefault(2)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:52)
- [ ] `ConfigKeyType.restoreDefault(2)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:57)
- [ ] `ConfigKeyType.read(5)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:66)
- [ ] `ConfigKeyType.write(4)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:71)
- [ ] `ConfigKeyType.copy(2)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:76)
- [ ] `ConfigKeyType.isFolder(0)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:81)
- [ ] `ConfigKeyType.holder(0)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:86)
- [ ] `ConfigKeyType.triggerConfigured(1)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:91)
- [ ] `ConfigKeyType.forceValue(2)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:98)
- [ ] `ConfigKeyType.converation(0)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:103)
- [ ] `ConfigKeyType.create(3)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:108)
- [ ] `ConfigKeyType.load(3)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:115)
- [ ] `ConfigKeyType.save(3)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:120)
- [ ] `ConfigKeyType.getRegistry(0)`: its class isn't translated (it extends `ConfigKey`) (team/creative/creativecore/common/config/key/ConfigKeyType.java:125)

### `Curve`: it's an interface

- [ ] `Curve.valueAt(1)`: it has no body (team/creative/creativecore/common/config/premade/curve/Curve.java:4)

### `DecimalCurve`: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)

- [ ] `DecimalCurve.DecimalCurve(4)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/curve/DecimalCurve.java:20)
- [ ] `DecimalCurve.DecimalCurve(3)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/curve/DecimalCurve.java:27)
- [ ] `DecimalCurve.DecimalCurve(3)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/curve/DecimalCurve.java:34)
- [ ] `DecimalCurve.valueAt(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/curve/DecimalCurve.java:41)
- [ ] `DecimalCurve.configured(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/curve/DecimalCurve.java:55)

### `IntCurve`: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)

- [ ] `IntCurve.IntCurve(4)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/curve/IntCurve.java:17)
- [ ] `IntCurve.valueAt(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/curve/IntCurve.java:24)
- [ ] `IntCurve.configured(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/curve/IntCurve.java:38)

### `LinearCurve`: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)

- [ ] `LinearCurve.LinearCurve(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/curve/LinearCurve.java:11)
- [ ] `LinearCurve.valueAt(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/curve/LinearCurve.java:15)
- [ ] `LinearCurve.configured(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/curve/LinearCurve.java:20)

### `DecimalMinMax`: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)

- [ ] `DecimalMinMax.DecimalMinMax(2)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/DecimalMinMax.java:14)
- [ ] `DecimalMinMax.next(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/DecimalMinMax.java:19)
- [ ] `DecimalMinMax.configured(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/DecimalMinMax.java:23)
- [ ] `DecimalMinMax.spanLength(0)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/DecimalMinMax.java:32)

### `IntMinMax`: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)

- [ ] `IntMinMax.IntMinMax(2)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/IntMinMax.java:14)
- [ ] `IntMinMax.next(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/IntMinMax.java:19)
- [ ] `IntMinMax.configured(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/IntMinMax.java:28)
- [ ] `IntMinMax.spanLength(0)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/creativecore/common/config/premade/IntMinMax.java:37)

### `MobEffectConfig`: it uses RegistryObjectConfig, which isn't translated (it's generic)

- [ ] `MobEffectConfig.MobEffectConfig(5)`: its class isn't translated (it uses RegistryObjectConfig, which isn't translated (it's generic)) (team/creative/creativecore/common/config/premade/MobEffectConfig.java:20)
- [ ] `MobEffectConfig.create(0)`: its class isn't translated (it uses RegistryObjectConfig, which isn't translated (it's generic)) (team/creative/creativecore/common/config/premade/MobEffectConfig.java:27)
- [ ] `MobEffectConfig.equals(1)`: its class isn't translated (it uses RegistryObjectConfig, which isn't translated (it's generic)) (team/creative/creativecore/common/config/premade/MobEffectConfig.java:31)

### `NamedList`: it's generic

- [ ] `NamedList.entrySet(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/NamedList.java:8)
- [ ] `NamedList.keySet(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/NamedList.java:13)

### `Permission`: it's generic

- [ ] `Permission.Permission(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/Permission.java:10)
- [ ] `Permission.getDefault(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/Permission.java:15)
- [ ] `Permission.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/Permission.java:19)
- [ ] `Permission.put(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/Permission.java:24)
- [ ] `Permission.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/Permission.java:32)
- [ ] `Permission.getDirect(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/Permission.java:42)
- [ ] `Permission.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/Permission.java:47)
- [ ] `Permission.getOrDefault(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/Permission.java:53)

### `RegistryObjectConfig`: it's generic

- [ ] `RegistryObjectConfig.RegistryObjectConfig(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryObjectConfig.java:15)
- [ ] `RegistryObjectConfig.configured(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryObjectConfig.java:21)
- [ ] `RegistryObjectConfig.getHolder(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryObjectConfig.java:26)
- [ ] `RegistryObjectConfig.equals(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryObjectConfig.java:30)

### `RegistryObjectListConfig`: it's generic

- [ ] `RegistryObjectListConfig.RegistryObjectListConfig(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java:14)
- [ ] `RegistryObjectListConfig.add(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java:18)
- [ ] `RegistryObjectListConfig.iterator(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java:24)
- [ ] `RegistryObjectListConfig.identifiers(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java:29)
- [ ] `RegistryObjectListConfig.contains(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java:33)
- [ ] `RegistryObjectListConfig.getLocation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java:43)
- [ ] `RegistryObjectListConfig.size(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java:47)

### `RegistryObjectListConfig.RegistryHolder`: it uses `T`

- [ ] `RegistryObjectListConfig.RegistryHolder.RegistryHolder(1)`: its class isn't translated (it uses `T`) (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java:55)
- [ ] `RegistryObjectListConfig.RegistryHolder.hashCode(0)`: its class isn't translated (it uses `T`) (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java:59)
- [ ] `RegistryObjectListConfig.RegistryHolder.equals(1)`: its class isn't translated (it uses `T`) (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java:64)
- [ ] `RegistryObjectListConfig.RegistryHolder.get(0)`: its class isn't translated (it uses `T`) (team/creative/creativecore/common/config/premade/registry/RegistryObjectListConfig.java:69)

### `RegistryTagConfig`: it's generic

- [ ] `RegistryTagConfig.RegistryTagConfig(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryTagConfig.java:11)
- [ ] `RegistryTagConfig.equals(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryTagConfig.java:16)

### `RegistryTagListConfig`: it's generic

- [ ] `RegistryTagListConfig.RegistryTagListConfig(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryTagListConfig.java:13)
- [ ] `RegistryTagListConfig.add(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryTagListConfig.java:17)
- [ ] `RegistryTagListConfig.iterator(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryTagListConfig.java:23)
- [ ] `RegistryTagListConfig.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryTagListConfig.java:28)
- [ ] `RegistryTagListConfig.size(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/registry/RegistryTagListConfig.java:32)

### `SelectableConfig`: it's generic

- [ ] `SelectableConfig.SelectableConfig(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/SelectableConfig.java:12)
- [ ] `SelectableConfig.reset(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/SelectableConfig.java:18)
- [ ] `SelectableConfig.select(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/SelectableConfig.java:22)
- [ ] `SelectableConfig.get(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/SelectableConfig.java:30)
- [ ] `SelectableConfig.getArray(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/SelectableConfig.java:34)
- [ ] `SelectableConfig.getSelected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/SelectableConfig.java:38)
- [ ] `SelectableConfig.updateArray(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/SelectableConfig.java:42)
- [ ] `SelectableConfig.isDefault(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/SelectableConfig.java:58)
- [ ] `SelectableConfig.restoreDefault(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/SelectableConfig.java:63)
- [ ] `SelectableConfig.configured(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/SelectableConfig.java:68)

### `SoundConfig`

- [x] `SoundConfig.SoundConfig(3)` (team/creative/creativecore/common/config/premade/SoundConfig.java:15)
- [x] `SoundConfig.SoundConfig(1)` (team/creative/creativecore/common/config/premade/SoundConfig.java:21)
- [ ] `SoundConfig.getEvent(0)`: it uses Minecraft's `SoundEvent` (team/creative/creativecore/common/config/premade/SoundConfig.java:25)
- [ ] `SoundConfig.play(2)`: it uses Minecraft's `Entity` (team/creative/creativecore/common/config/premade/SoundConfig.java:29)
- [ ] `SoundConfig.play(5)`: it uses Minecraft's `Level` (team/creative/creativecore/common/config/premade/SoundConfig.java:36)
- [ ] `SoundConfig.equals(1)`: it uses `Object` (team/creative/creativecore/common/config/premade/SoundConfig.java:43)

### `ToggleableConfig`: it's generic

- [ ] `ToggleableConfig.ToggleableConfig(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/ToggleableConfig.java:10)
- [ ] `ToggleableConfig.ToggleableConfig(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/ToggleableConfig.java:14)
- [ ] `ToggleableConfig.configured(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/ToggleableConfig.java:19)
- [ ] `ToggleableConfig.setEnabled(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/ToggleableConfig.java:26)
- [ ] `ToggleableConfig.isEnabled(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/ToggleableConfig.java:30)
- [ ] `ToggleableConfig.equals(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/config/premade/ToggleableConfig.java:34)

### `ConfigSynchronization`: its constants carry values

- [ ] `ConfigSynchronization.useFolder(2)`: it has no body (team/creative/creativecore/common/config/sync/ConfigSynchronization.java:40)
- [ ] `ConfigSynchronization.useValue(2)`: it has no body (team/creative/creativecore/common/config/sync/ConfigSynchronization.java:42)

### `ConfigurationChangePacket`: it extends `CreativePacket` (team.creative.creativecore.common.network)

- [ ] `ConfigurationChangePacket.ConfigurationChangePacket(2)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationChangePacket.java:17)
- [ ] `ConfigurationChangePacket.ConfigurationChangePacket(0)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationChangePacket.java:22)
- [ ] `ConfigurationChangePacket.executeClient(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationChangePacket.java:25)
- [ ] `ConfigurationChangePacket.executeServer(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationChangePacket.java:29)

### `ConfigurationClientPacket`: it extends `CreativePacket` (team.creative.creativecore.common.network)

- [ ] `ConfigurationClientPacket.ConfigurationClientPacket(2)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationClientPacket.java:20)
- [ ] `ConfigurationClientPacket.ConfigurationClientPacket(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationClientPacket.java:25)
- [ ] `ConfigurationClientPacket.buildClientFieldList(2)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationClientPacket.java:30)
- [ ] `ConfigurationClientPacket.ConfigurationClientPacket(0)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationClientPacket.java:52)
- [ ] `ConfigurationClientPacket.run(0)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationClientPacket.java:55)
- [ ] `ConfigurationClientPacket.executeClient(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationClientPacket.java:64)
- [ ] `ConfigurationClientPacket.executeServer(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationClientPacket.java:70)

### `ConfigurationPacket`: it extends `CreativePacket` (team.creative.creativecore.common.network)

- [ ] `ConfigurationPacket.ConfigurationPacket(3)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationPacket.java:21)
- [ ] `ConfigurationPacket.ConfigurationPacket(0)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationPacket.java:27)
- [ ] `ConfigurationPacket.executeClient(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationPacket.java:30)
- [ ] `ConfigurationPacket.executeServer(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/config/sync/ConfigurationPacket.java:40)
- [ ] `ConfigurationPacket.updateGui(1)`: it uses Minecraft's LocalPlayer.containerMenu (no Horizon helper for it yet) (team/creative/creativecore/common/config/sync/ConfigurationPacket.java:44)

### `Align`


### `GuiCheckList`: it's generic

- [ ] `GuiCheckList.GuiCheckList(5)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:28)
- [ ] `GuiCheckList.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:36)
- [ ] `GuiCheckList.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:40)
- [ ] `GuiCheckList.createControl(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:53)
- [ ] `GuiCheckList.removeControl(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:59)
- [ ] `GuiCheckList.addCustomControl(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:64)
- [ ] `GuiCheckList.addCustom(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:68)
- [ ] `GuiCheckList.add(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:73)
- [ ] `GuiCheckList.addHover(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:79)
- [ ] `GuiCheckList.removeItem(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:85)
- [ ] `GuiCheckList.isEmpty(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:94)
- [ ] `GuiCheckList.size(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:99)
- [ ] `GuiCheckList.getValue(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:104)
- [ ] `GuiCheckList.selectedItems(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:108)
- [ ] `GuiCheckList.allItems(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:112)
- [ ] `GuiCheckList.checked(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:116)
- [ ] `GuiCheckList.indexOf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:120)

### `GuiCheckList.GuiCheckListDist`: it's an interface

- [ ] `GuiCheckList.GuiCheckListDist.reflowInternal(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:131)

### `GuiCheckList.GuiCheckListRow`: it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)

- [ ] `GuiCheckList.GuiCheckListRow.GuiCheckListRow(4)`: its class isn't translated (it extends `GuiRow` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/collection/GuiCheckList.java:139)

### `GuiComboBox`: it's generic

- [ ] `GuiComboBox.GuiComboBox(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:10)
- [ ] `GuiComboBox.GuiComboBox(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:17)
- [ ] `GuiComboBox.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:24)
- [ ] `GuiComboBox.hasSearchbar(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:28)
- [ ] `GuiComboBox.setSearchbar(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:32)
- [ ] `GuiComboBox.setDirection(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:40)
- [ ] `GuiComboBox.set(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:48)
- [ ] `GuiComboBox.selected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:54)
- [ ] `GuiComboBox.selected(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:59)
- [ ] `GuiComboBox.selectSilent(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:63)
- [ ] `GuiComboBox.select(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:69)
- [ ] `GuiComboBox.selectSilent(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:75)
- [ ] `GuiComboBox.select(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:81)
- [ ] `GuiComboBox.indexOf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:87)
- [ ] `GuiComboBox.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:91)
- [ ] `GuiComboBox.previous(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:97)
- [ ] `GuiComboBox.selectedIndex(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:103)

### `GuiComboBox.GuiComboBoxDist`: it's an interface

- [ ] `GuiComboBox.GuiComboBoxDist.hasSearchbar(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:108)
- [ ] `GuiComboBox.GuiComboBoxDist.setSearchbar(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:110)
- [ ] `GuiComboBox.GuiComboBoxDist.setDirection(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:112)
- [ ] `GuiComboBox.GuiComboBoxDist.set(2)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:114)
- [ ] `GuiComboBox.GuiComboBoxDist.selected(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:116)
- [ ] `GuiComboBox.GuiComboBoxDist.selected(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:119)
- [ ] `GuiComboBox.GuiComboBoxDist.select(2)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:121)
- [ ] `GuiComboBox.GuiComboBoxDist.select(2)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:123)
- [ ] `GuiComboBox.GuiComboBoxDist.indexOf(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:125)
- [ ] `GuiComboBox.GuiComboBoxDist.next(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:127)
- [ ] `GuiComboBox.GuiComboBoxDist.previous(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:129)
- [ ] `GuiComboBox.GuiComboBoxDist.selectedIndex(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBox.java:131)

### `GuiComboBoxExtension`: it extends `GuiListBoxBase`

- [ ] `GuiComboBoxExtension.GuiComboBoxExtension(2)`: its class isn't translated (it extends `GuiListBoxBase`) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxExtension.java:8)

### `GuiComboBoxFlexible`: it's generic

- [ ] `GuiComboBoxFlexible.GuiComboBoxFlexible(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxFlexible.java:9)
- [ ] `GuiComboBoxFlexible.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxFlexible.java:16)
- [ ] `GuiComboBoxFlexible.forceSelect(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxFlexible.java:20)

### `GuiComboBoxFlexible.GuiComboBoxFlexibleDist`: it's an interface

- [ ] `GuiComboBoxFlexible.GuiComboBoxFlexibleDist.init(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBoxFlexible.java:27)
- [ ] `GuiComboBoxFlexible.GuiComboBoxFlexibleDist.forceSelect(2)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBoxFlexible.java:29)

### `GuiComboBoxTree`: it's generic

- [ ] `GuiComboBoxTree.GuiComboBoxTree(5)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:12)
- [ ] `GuiComboBoxTree.GuiComboBoxTree(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:19)
- [ ] `GuiComboBoxTree.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:27)
- [ ] `GuiComboBoxTree.hasSearchbar(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:31)
- [ ] `GuiComboBoxTree.setSearchbar(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:35)
- [ ] `GuiComboBoxTree.setDirection(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:43)
- [ ] `GuiComboBoxTree.set(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:51)
- [ ] `GuiComboBoxTree.selected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:57)
- [ ] `GuiComboBoxTree.selected(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:62)
- [ ] `GuiComboBoxTree.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:66)
- [ ] `GuiComboBoxTree.select(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:72)

### `GuiComboBoxTree.GuiComboBoxTreeDist`: it's an interface

- [ ] `GuiComboBoxTree.GuiComboBoxTreeDist.init(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:79)
- [ ] `GuiComboBoxTree.GuiComboBoxTreeDist.hasSearchbar(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:81)
- [ ] `GuiComboBoxTree.GuiComboBoxTreeDist.setSearchbar(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:83)
- [ ] `GuiComboBoxTree.GuiComboBoxTreeDist.setDirection(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:85)
- [ ] `GuiComboBoxTree.GuiComboBoxTreeDist.set(2)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:87)
- [ ] `GuiComboBoxTree.GuiComboBoxTreeDist.selected(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:89)
- [ ] `GuiComboBoxTree.GuiComboBoxTreeDist.selected(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:92)
- [ ] `GuiComboBoxTree.GuiComboBoxTreeDist.select(3)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:94)
- [ ] `GuiComboBoxTree.GuiComboBoxTreeDist.select(2)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiComboBoxTree.java:96)

### `GuiListBoxBase`: it's generic

- [ ] `GuiListBoxBase.GuiListBoxBase(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:23)
- [ ] `GuiListBoxBase.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:30)
- [ ] `GuiListBoxBase.createItems(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:34)
- [ ] `GuiListBoxBase.createControl(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:40)
- [ ] `GuiListBoxBase.removeControl(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:57)
- [ ] `GuiListBoxBase.addCustomControl(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:63)
- [ ] `GuiListBoxBase.add(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:67)
- [ ] `GuiListBoxBase.addHover(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:73)
- [ ] `GuiListBoxBase.removeItem(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:79)
- [ ] `GuiListBoxBase.clearItems(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:96)
- [ ] `GuiListBoxBase.addAllItems(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:102)
- [ ] `GuiListBoxBase.addItem(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:113)
- [ ] `GuiListBoxBase.isListEmpty(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:123)
- [ ] `GuiListBoxBase.itemSize(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:127)
- [ ] `GuiListBoxBase.getItem(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:131)
- [ ] `GuiListBoxBase.items(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:135)

### `GuiListBoxBase.GuiButtonRemove`: it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)

- [ ] `GuiListBoxBase.GuiButtonRemove.GuiButtonRemove(2)`: its class isn't translated (it extends `GuiButton` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:142)

### `GuiListBoxBase.GuiListBoxBaseDist`: it's an interface

- [ ] `GuiListBoxBase.GuiListBoxBaseDist.reflowInternal(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiListBoxBase.java:153)

### `GuiStackSelector`: it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)

- [ ] `GuiStackSelector.GuiStackSelector(4)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:21)
- [ ] `GuiStackSelector.GuiStackSelector(3)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:33)
- [ ] `GuiStackSelector.dist(0)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:37)
- [ ] `GuiStackSelector.setWidth(1)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:41)
- [ ] `GuiStackSelector.hasSearchbar(0)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:46)
- [ ] `GuiStackSelector.setSearchbar(1)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:50)
- [ ] `GuiStackSelector.selectFirst(0)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:58)
- [ ] `GuiStackSelector.updateCollectedStacks(0)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:62)
- [ ] `GuiStackSelector.setSelectedForce(1)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:68)
- [ ] `GuiStackSelector.setSelected(1)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:72)
- [ ] `GuiStackSelector.getStacks(0)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:76)
- [ ] `GuiStackSelector.getSelected(0)`: its class isn't translated (it extends `GuiLabel` (team.creative.creativecore.common.gui.control.simple)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:80)
- [ ] `GuiStackSelector.contains(2)`: it calls GuiStackSelector.getItemName, which isn't translated (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:84)
- [ ] `GuiStackSelector.getItemName(1)`: it calls Minecraft's ItemStack.getDisplayName (no Horizon helper for it yet) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:102)

### `GuiStackSelector.CreativeCollector`: it extends `InventoryCollector`

- [ ] `GuiStackSelector.CreativeCollector.CreativeCollector(1)`: its class isn't translated (it extends `InventoryCollector`) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:115)
- [ ] `GuiStackSelector.CreativeCollector.collect(1)`: its class isn't translated (it extends `InventoryCollector`) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:119)

### `GuiStackSelector.GuiBlockSelector`: it extends `SearchSelector`

- [ ] `GuiStackSelector.GuiBlockSelector.allow(1)`: its class isn't translated (it extends `SearchSelector`) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:137)

### `GuiStackSelector.GuiStackSelectorDist`: it's an interface

- [ ] `GuiStackSelector.GuiStackSelectorDist.setCollector(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:144)
- [ ] `GuiStackSelector.GuiStackSelectorDist.hasSearchbar(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:146)
- [ ] `GuiStackSelector.GuiStackSelectorDist.setSearchbar(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:148)
- [ ] `GuiStackSelector.GuiStackSelectorDist.selectFirst(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:150)
- [ ] `GuiStackSelector.GuiStackSelectorDist.updateCollectedStacks(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:152)
- [ ] `GuiStackSelector.GuiStackSelectorDist.setSelectedForce(2)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:154)
- [ ] `GuiStackSelector.GuiStackSelectorDist.setSelected(2)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:156)
- [ ] `GuiStackSelector.GuiStackSelectorDist.getStacks(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:158)
- [ ] `GuiStackSelector.GuiStackSelectorDist.getSelected(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:160)

### `GuiStackSelector.InventoryCollector`: it extends `StackCollector`

- [ ] `GuiStackSelector.InventoryCollector.InventoryCollector(1)`: its class isn't translated (it extends `StackCollector`) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:164)
- [ ] `GuiStackSelector.InventoryCollector.collect(1)`: its class isn't translated (it extends `StackCollector`) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:168)

### `GuiStackSelector.SearchSelector`: it extends `StackSelector`

- [ ] `GuiStackSelector.SearchSelector.allow(1)`: its class isn't translated (it extends `StackSelector`) (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:192)

### `GuiStackSelector.StackCollector`

- [x] `GuiStackSelector.StackCollector.StackCollector(1)` (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:201)
- [ ] `GuiStackSelector.StackCollector.collect(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:205)

### `GuiStackSelector.StackSelector`

- [ ] `GuiStackSelector.StackSelector.allow(1)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelector.java:209)

### `GuiStackSelectorExtension`: it extends `GuiScrollY` (team.creative.creativecore.common.gui.control.parent)

- [ ] `GuiStackSelectorExtension.GuiStackSelectorExtension(2)`: its class isn't translated (it extends `GuiScrollY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelectorExtension.java:8)
- [ ] `GuiStackSelectorExtension.dist(0)`: its class isn't translated (it extends `GuiScrollY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/collection/GuiStackSelectorExtension.java:13)

### `GuiStackSelectorExtension.GuiStackSelectorExtensionDist`: it's an interface

- [ ] `GuiStackSelectorExtension.GuiStackSelectorExtensionDist.reflowInternal(0)`: it has no body (team/creative/creativecore/common/gui/control/collection/GuiStackSelectorExtension.java:18)

### `GuiInventoryGrid`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiInventoryGrid.GuiInventoryGrid(3)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:23)
- [ ] `GuiInventoryGrid.GuiInventoryGrid(4)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:28)
- [ ] `GuiInventoryGrid.GuiInventoryGrid(5)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:32)
- [ ] `GuiInventoryGrid.GuiInventoryGrid(6)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:36)
- [ ] `GuiInventoryGrid.dist(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:48)
- [ ] `GuiInventoryGrid.createInventoryGrid(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:52)
- [ ] `GuiInventoryGrid.disableSlot(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:58)
- [ ] `GuiInventoryGrid.addListener(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:63)
- [ ] `GuiInventoryGrid.addSlot(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:72)
- [ ] `GuiInventoryGrid.add(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:81)
- [ ] `GuiInventoryGrid.addHover(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:87)
- [ ] `GuiInventoryGrid.getSlot(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:93)
- [ ] `GuiInventoryGrid.inventorySize(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:98)
- [ ] `GuiInventoryGrid.hasFixedSize(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:103)
- [ ] `GuiInventoryGrid.name(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:107)
- [ ] `GuiInventoryGrid.tick(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:112)
- [ ] `GuiInventoryGrid.setChanged(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:128)
- [ ] `GuiInventoryGrid.setChanged(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:138)

### `GuiInventoryGrid.GuiInventoryGridDist`: it's an interface

- [ ] `GuiInventoryGrid.GuiInventoryGridDist.setGridDim(2)`: it has no body (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGrid.java:152)

### `GuiInventoryGridPreview`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiInventoryGridPreview.GuiInventoryGridPreview(3)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGridPreview.java:13)
- [ ] `GuiInventoryGridPreview.GuiInventoryGridPreview(4)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGridPreview.java:18)
- [ ] `GuiInventoryGridPreview.GuiInventoryGridPreview(5)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGridPreview.java:22)
- [ ] `GuiInventoryGridPreview.dist(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGridPreview.java:37)
- [ ] `GuiInventoryGridPreview.hasFixedSize(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGridPreview.java:41)
- [ ] `GuiInventoryGridPreview.add(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGridPreview.java:45)
- [ ] `GuiInventoryGridPreview.addHover(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGridPreview.java:51)

### `GuiInventoryGridPreview.GuiInventoryGridPreviewDist`: it's an interface

- [ ] `GuiInventoryGridPreview.GuiInventoryGridPreviewDist.setGridDim(2)`: it has no body (team/creative/creativecore/common/gui/control/inventory/GuiInventoryGridPreview.java:58)

### `GuiPlayerInventoryGrid`: it extends `GuiInventoryGrid`

- [ ] `GuiPlayerInventoryGrid.GuiPlayerInventoryGrid(1)`: its class isn't translated (it extends `GuiInventoryGrid`) (team/creative/creativecore/common/gui/control/inventory/GuiPlayerInventoryGrid.java:10)
- [ ] `GuiPlayerInventoryGrid.createInventoryGrid(1)`: its class isn't translated (it extends `GuiInventoryGrid`) (team/creative/creativecore/common/gui/control/inventory/GuiPlayerInventoryGrid.java:15)
- [ ] `GuiPlayerInventoryGrid.moveInside(2)`: its class isn't translated (it extends `GuiInventoryGrid`) (team/creative/creativecore/common/gui/control/inventory/GuiPlayerInventoryGrid.java:26)

### `GuiSlot`: it extends `GuiSlotBase`

- [ ] `GuiSlot.GuiSlot(3)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:17)
- [ ] `GuiSlot.GuiSlot(4)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:21)
- [ ] `GuiSlot.GuiSlot(2)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:25)
- [ ] `GuiSlot.GuiSlot(3)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:29)
- [ ] `GuiSlot.dist(0)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:34)
- [ ] `GuiSlot.getStack(0)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:38)
- [ ] `GuiSlot.itemManager(0)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:43)
- [ ] `GuiSlot.inventory(0)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:47)
- [ ] `GuiSlot.tick(0)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:58)
- [ ] `GuiSlot.onSendUpdate(0)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:66)
- [ ] `GuiSlot.changed(0)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:71)
- [ ] `GuiSlot.insert(1)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:79)

### `GuiSlot.GuiSlotDist`: it's an interface

- [ ] `GuiSlot.GuiSlotDist.changed(0)`: it has no body (team/creative/creativecore/common/gui/control/inventory/GuiSlot.java:106)

### `GuiSlotBase`: it extends `GuiControl` (team.creative.creativecore.common.gui)

- [ ] `GuiSlotBase.GuiSlotBase(2)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiSlotBase.java:8)
- [ ] `GuiSlotBase.init(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiSlotBase.java:12)
- [ ] `GuiSlotBase.closed(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiSlotBase.java:16)
- [ ] `GuiSlotBase.tick(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/inventory/GuiSlotBase.java:20)
- [ ] `GuiSlotBase.getStack(0)`: it has no body (team/creative/creativecore/common/gui/control/inventory/GuiSlotBase.java:24)

### `GuiSlotViewer`: it extends `GuiSlotBase`

- [ ] `GuiSlotViewer.GuiSlotViewer(3)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlotViewer.java:9)
- [ ] `GuiSlotViewer.GuiSlotViewer(2)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlotViewer.java:14)
- [ ] `GuiSlotViewer.getStack(0)`: its class isn't translated (it extends `GuiSlotBase`) (team/creative/creativecore/common/gui/control/inventory/GuiSlotViewer.java:19)

### `IGuiInventory`: it's an interface

- [ ] `IGuiInventory.getSlot(1)`: it has no body (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:34)
- [ ] `IGuiInventory.inventorySize(0)`: it has no body (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:36)
- [ ] `IGuiInventory.name(0)`: it has no body (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:38)
- [ ] `IGuiInventory.setChanged(0)`: it has no body (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:40)
- [ ] `IGuiInventory.setChanged(1)`: it has no body (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:42)
- [ ] `IGuiInventory.moveInside(2)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:44)
- [ ] `IGuiInventory.insertClever(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:48)
- [ ] `IGuiInventory.insertClever(3)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:52)
- [ ] `IGuiInventory.insert(2)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:59)
- [ ] `IGuiInventory.insert(4)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:63)
- [ ] `IGuiInventory.extract(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:75)
- [ ] `IGuiInventory.sync(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:94)
- [ ] `IGuiInventory.syncAll(0)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/control/inventory/IGuiInventory.java:109)

### `GuiMenu`: it's generic

- [ ] `GuiMenu.GuiMenu(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:9)
- [ ] `GuiMenu.populateEntryTree(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:16)
- [ ] `GuiMenu.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:22)
- [ ] `GuiMenu.closed(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:26)
- [ ] `GuiMenu.isRoot(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:33)
- [ ] `GuiMenu.root(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:37)

### `GuiMenu.GuiMenuDist`: it's an interface

- [ ] `GuiMenu.GuiMenuDist.closed(0)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:42)
- [ ] `GuiMenu.GuiMenuDist.init(1)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:44)
- [ ] `GuiMenu.GuiMenuDist.isRoot(0)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:46)
- [ ] `GuiMenu.GuiMenuDist.root(0)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:48)
- [ ] `GuiMenu.GuiMenuDist.buildTree(0)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:50)
- [ ] `GuiMenu.GuiMenuDist.populateEntryTree(1)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:52)

### `GuiMenu.GuiMenuEntry`: it's generic

- [ ] `GuiMenu.GuiMenuEntry.GuiMenuEntry(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:58)
- [ ] `GuiMenu.GuiMenuEntry.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:64)
- [ ] `GuiMenu.GuiMenuEntry.setHighlighted(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:68)
- [ ] `GuiMenu.GuiMenuEntry.close(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:74)
- [ ] `GuiMenu.GuiMenuEntry.open(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:80)
- [ ] `GuiMenu.GuiMenuEntry.menu(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:86)

### `GuiMenu.GuiMenuEntryDist`: it's an interface

- [ ] `GuiMenu.GuiMenuEntryDist.setHighlighted(1)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:92)
- [ ] `GuiMenu.GuiMenuEntryDist.close(0)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:94)
- [ ] `GuiMenu.GuiMenuEntryDist.open(0)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenu.java:96)

### `GuiMenuRoot`: it's generic

- [ ] `GuiMenuRoot.GuiMenuRoot(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenuRoot.java:10)
- [ ] `GuiMenuRoot.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenuRoot.java:18)
- [ ] `GuiMenuRoot.translate(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenuRoot.java:22)
- [ ] `GuiMenuRoot.select(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenuRoot.java:26)
- [ ] `GuiMenuRoot.createEntryTree(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenuRoot.java:32)

### `GuiMenuRoot.GuiMenuRootDist`: it's an interface

- [ ] `GuiMenuRoot.GuiMenuRootDist.set(2)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenuRoot.java:39)
- [ ] `GuiMenuRoot.GuiMenuRootDist.translate(2)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenuRoot.java:41)
- [ ] `GuiMenuRoot.GuiMenuRootDist.select(2)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenuRoot.java:43)

### `GuiMenuSub`: it's generic

- [ ] `GuiMenuSub.GuiMenuSub(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenuSub.java:6)
- [ ] `GuiMenuSub.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/menu/GuiMenuSub.java:14)

### `GuiMenuSub.GuiMenuSubDist`: it's an interface

- [ ] `GuiMenuSub.GuiMenuSubDist.setRoot(1)`: it has no body (team/creative/creativecore/common/gui/control/menu/GuiMenuSub.java:19)

### `GuiColumn`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiColumn.GuiColumn(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiColumn.java:10)
- [ ] `GuiColumn.GuiColumn(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiColumn.java:14)
- [ ] `GuiColumn.GuiColumn(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiColumn.java:18)
- [ ] `GuiColumn.GuiColumn(3)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiColumn.java:22)

### `GuiColumn.GuiColumnHeader`: it extends `GuiColumn`

- [ ] `GuiColumn.GuiColumnHeader.GuiColumnHeader(1)`: its class isn't translated (it extends `GuiColumn`) (team/creative/creativecore/common/gui/control/parent/GuiColumn.java:28)
- [ ] `GuiColumn.GuiColumnHeader.GuiColumnHeader(2)`: its class isn't translated (it extends `GuiColumn`) (team/creative/creativecore/common/gui/control/parent/GuiColumn.java:33)
- [ ] `GuiColumn.GuiColumnHeader.GuiColumnHeader(2)`: its class isn't translated (it extends `GuiColumn`) (team/creative/creativecore/common/gui/control/parent/GuiColumn.java:37)
- [ ] `GuiColumn.GuiColumnHeader.GuiColumnHeader(3)`: its class isn't translated (it extends `GuiColumn`) (team/creative/creativecore/common/gui/control/parent/GuiColumn.java:41)

### `GuiLabeledControl`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiLabeledControl.GuiLabeledControl(3)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiLabeledControl.java:15)
- [ ] `GuiLabeledControl.GuiLabeledControl(3)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiLabeledControl.java:28)
- [ ] `GuiLabeledControl.GuiLabeledControl(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiLabeledControl.java:32)
- [ ] `GuiLabeledControl.getInnerControl(0)`: it's generic (team/creative/creativecore/common/gui/control/parent/GuiLabeledControl.java:36)
- [ ] `GuiLabeledControl.setTranslate(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiLabeledControl.java:40)
- [ ] `GuiLabeledControl.setTranslate(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiLabeledControl.java:45)
- [ ] `GuiLabeledControl.setTitle(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiLabeledControl.java:50)
- [ ] `GuiLabeledControl.setTitle(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiLabeledControl.java:55)
- [ ] `GuiLabeledControl.setTooltip(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiLabeledControl.java:60)
- [ ] `GuiLabeledControl.setTooltip(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiLabeledControl.java:65)

### `GuiLeftRightBox`: it extends `GuiRow`

- [ ] `GuiLeftRightBox.GuiLeftRightBox(1)`: its class isn't translated (it extends `GuiRow`) (team/creative/creativecore/common/gui/control/parent/GuiLeftRightBox.java:12)
- [ ] `GuiLeftRightBox.isExpandableX(0)`: its class isn't translated (it extends `GuiRow`) (team/creative/creativecore/common/gui/control/parent/GuiLeftRightBox.java:21)
- [ ] `GuiLeftRightBox.getLeft(0)`: its class isn't translated (it extends `GuiRow`) (team/creative/creativecore/common/gui/control/parent/GuiLeftRightBox.java:26)
- [ ] `GuiLeftRightBox.getRight(0)`: its class isn't translated (it extends `GuiRow`) (team/creative/creativecore/common/gui/control/parent/GuiLeftRightBox.java:30)
- [ ] `GuiLeftRightBox.addLeft(1)`: its class isn't translated (it extends `GuiRow`) (team/creative/creativecore/common/gui/control/parent/GuiLeftRightBox.java:34)
- [ ] `GuiLeftRightBox.addRight(1)`: its class isn't translated (it extends `GuiRow`) (team/creative/creativecore/common/gui/control/parent/GuiLeftRightBox.java:39)
- [ ] `GuiLeftRightBox.setVAlign(1)`: its class isn't translated (it extends `GuiRow`) (team/creative/creativecore/common/gui/control/parent/GuiLeftRightBox.java:44)
- [ ] `GuiLeftRightBox.addColumn(1)`: its class isn't translated (it extends `GuiRow`) (team/creative/creativecore/common/gui/control/parent/GuiLeftRightBox.java:53)

### `GuiPanel`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiPanel.GuiPanel(3)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiPanel.java:10)
- [ ] `GuiPanel.GuiPanel(4)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiPanel.java:14)
- [ ] `GuiPanel.GuiPanel(5)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiPanel.java:18)
- [ ] `GuiPanel.GuiPanel(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiPanel.java:22)
- [ ] `GuiPanel.GuiPanel(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiPanel.java:26)
- [ ] `GuiPanel.GuiPanel(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiPanel.java:30)

### `GuiRow`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiRow.GuiRow(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiRow.java:11)
- [ ] `GuiRow.GuiRow(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiRow.java:19)
- [ ] `GuiRow.addColumn(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiRow.java:27)
- [ ] `GuiRow.removeCol(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiRow.java:32)
- [ ] `GuiRow.getCol(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiRow.java:36)
- [ ] `GuiRow.colCount(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiRow.java:40)
- [ ] `GuiRow.add(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiRow.java:44)
- [ ] `GuiRow.addHover(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiRow.java:50)

### `GuiScrollX`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiScrollX.GuiScrollX(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java:9)
- [ ] `GuiScrollX.GuiScrollX(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java:13)
- [ ] `GuiScrollX.dist(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java:17)
- [ ] `GuiScrollX.setHovered(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java:21)
- [ ] `GuiScrollX.setHover(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java:29)
- [ ] `GuiScrollX.setDim(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java:37)
- [ ] `GuiScrollX.setDim(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java:41)
- [ ] `GuiScrollX.setExpandable(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java:45)

### `GuiScrollX.GuiScrollXDist`: it's an interface

- [ ] `GuiScrollX.GuiScrollXDist.setHovered(0)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java:50)
- [ ] `GuiScrollX.GuiScrollXDist.setHover(1)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiScrollX.java:52)

### `GuiScrollXY`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiScrollXY.GuiScrollXY(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollXY.java:8)
- [ ] `GuiScrollXY.GuiScrollXY(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollXY.java:12)
- [ ] `GuiScrollXY.GuiScrollXY(3)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollXY.java:16)
- [ ] `GuiScrollXY.dist(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollXY.java:20)

### `GuiScrollXY.GuiScrollXYDist`: it's an interface


### `GuiScrollY`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiScrollY.GuiScrollY(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:10)
- [ ] `GuiScrollY.GuiScrollY(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:14)
- [ ] `GuiScrollY.dist(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:18)
- [ ] `GuiScrollY.setHovered(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:22)
- [ ] `GuiScrollY.setHover(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:30)
- [ ] `GuiScrollY.setScrollWhenCTRL(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:38)
- [ ] `GuiScrollY.setFormatting(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:46)
- [ ] `GuiScrollY.setDim(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:50)
- [ ] `GuiScrollY.setDim(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:54)
- [ ] `GuiScrollY.setExpandable(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:58)

### `GuiScrollY.GuiScrollYDist`: it's an interface

- [ ] `GuiScrollY.GuiScrollYDist.setHovered(0)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:63)
- [ ] `GuiScrollY.GuiScrollYDist.setHover(1)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:65)
- [ ] `GuiScrollY.GuiScrollYDist.setScrollWhenCTRL(0)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiScrollY.java:67)

### `GuiTable`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiTable.GuiTable(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTable.java:9)
- [ ] `GuiTable.GuiTable(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTable.java:13)
- [ ] `GuiTable.GuiTable(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTable.java:17)
- [ ] `GuiTable.addRow(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTable.java:25)
- [ ] `GuiTable.setExpandable(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTable.java:30)
- [ ] `GuiTable.add(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTable.java:34)
- [ ] `GuiTable.addHover(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTable.java:40)

### `GuiTableScrollable`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiTableScrollable.GuiTableScrollable(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:16)
- [ ] `GuiTableScrollable.GuiTableScrollable(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:20)
- [ ] `GuiTableScrollable.dist(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:30)
- [ ] `GuiTableScrollable.add(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:34)
- [ ] `GuiTableScrollable.addHover(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:40)
- [ ] `GuiTableScrollable.setExpandable(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:46)
- [ ] `GuiTableScrollable.getTopRow(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:50)
- [ ] `GuiTableScrollable.addRow(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:54)
- [ ] `GuiTableScrollable.isExpandableX(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:64)
- [ ] `GuiTableScrollable.isExpandableY(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:69)
- [ ] `GuiTableScrollable.removeContentCol(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:74)
- [ ] `GuiTableScrollable.contentRows(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:82)
- [ ] `GuiTableScrollable.createDist(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:86)

### `GuiTableScrollable.GuiTableScrollableDist`: it's an interface

- [ ] `GuiTableScrollable.GuiTableScrollableDist.createScrollBox(1)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:106)
- [ ] `GuiTableScrollable.GuiTableScrollableDist.createTopRow(1)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:108)
- [ ] `GuiTableScrollable.GuiTableScrollableDist.createFirstCol(1)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiTableScrollable.java:110)

### `GuiTabs`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiTabs.GuiTabs(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:20)
- [ ] `GuiTabs.dist(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:29)
- [ ] `GuiTabs.createDist(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:33)
- [ ] `GuiTabs.createTab(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:40)
- [ ] `GuiTabs.getTabButton(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:47)
- [ ] `GuiTabs.getTab(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:51)
- [ ] `GuiTabs.select(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:55)
- [ ] `GuiTabs.index(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:61)

### `GuiTabs.GuiTabBar`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiTabs.GuiTabBar.GuiTabBar(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:69)
- [ ] `GuiTabs.GuiTabBar.highlight(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:74)
- [ ] `GuiTabs.GuiTabBar.addTab(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:84)
- [ ] `GuiTabs.GuiTabBar.getTab(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:89)
- [ ] `GuiTabs.GuiTabBar.removeTab(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:93)

### `GuiTabs.GuiTabSpecialParent`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiTabs.GuiTabSpecialParent.GuiTabSpecialParent(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:99)

### `GuiTabs.GuiTabsDist`: it's an interface

- [ ] `GuiTabs.GuiTabsDist.init(2)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:105)
- [ ] `GuiTabs.GuiTabsDist.index(0)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:107)
- [ ] `GuiTabs.GuiTabsDist.select(2)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:109)
- [ ] `GuiTabs.GuiTabsDist.createSpecialParent(1)`: it has no body (team/creative/creativecore/common/gui/control/parent/GuiTabs.java:111)

### `GuiTabsMapped`: it's generic

- [ ] `GuiTabsMapped.GuiTabsMapped(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/parent/GuiTabsMapped.java:12)
- [ ] `GuiTabsMapped.createTab(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/parent/GuiTabsMapped.java:16)
- [ ] `GuiTabsMapped.createTab(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/parent/GuiTabsMapped.java:22)
- [ ] `GuiTabsMapped.getSelected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/parent/GuiTabsMapped.java:27)
- [ ] `GuiTabsMapped.getSelected(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/parent/GuiTabsMapped.java:32)
- [ ] `GuiTabsMapped.select(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/parent/GuiTabsMapped.java:37)

### `GuiTopBottomBox`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiTopBottomBox.GuiTopBottomBox(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTopBottomBox.java:14)
- [ ] `GuiTopBottomBox.addTop(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTopBottomBox.java:26)
- [ ] `GuiTopBottomBox.addBottom(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTopBottomBox.java:31)
- [ ] `GuiTopBottomBox.add(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTopBottomBox.java:36)
- [ ] `GuiTopBottomBox.addHover(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/parent/GuiTopBottomBox.java:42)

### `GuiArraySlider`: it extends `GuiSteppedSlider`

- [ ] `GuiArraySlider.GuiArraySlider(4)`: its class isn't translated (it extends `GuiSteppedSlider`) (team/creative/creativecore/common/gui/control/simple/GuiArraySlider.java:7)
- [ ] `GuiArraySlider.GuiArraySlider(2)`: its class isn't translated (it extends `GuiSteppedSlider`) (team/creative/creativecore/common/gui/control/simple/GuiArraySlider.java:14)
- [ ] `GuiArraySlider.dist(0)`: its class isn't translated (it extends `GuiSteppedSlider`) (team/creative/creativecore/common/gui/control/simple/GuiArraySlider.java:18)
- [ ] `GuiArraySlider.select(1)`: its class isn't translated (it extends `GuiSteppedSlider`) (team/creative/creativecore/common/gui/control/simple/GuiArraySlider.java:22)
- [ ] `GuiArraySlider.setValues(1)`: its class isn't translated (it extends `GuiSteppedSlider`) (team/creative/creativecore/common/gui/control/simple/GuiArraySlider.java:28)
- [ ] `GuiArraySlider.get(0)`: its class isn't translated (it extends `GuiSteppedSlider`) (team/creative/creativecore/common/gui/control/simple/GuiArraySlider.java:34)

### `GuiArraySlider.GuiArraySliderDist`: it's an interface

- [ ] `GuiArraySlider.GuiArraySliderDist.select(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiArraySlider.java:39)
- [ ] `GuiArraySlider.GuiArraySliderDist.setValues(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiArraySlider.java:41)
- [ ] `GuiArraySlider.GuiArraySliderDist.get(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiArraySlider.java:43)

### `GuiButton`: it extends `GuiLabel`

- [ ] `GuiButton.GuiButton(3)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiButton.java:7)
- [ ] `GuiButton.dist(0)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiButton.java:12)
- [ ] `GuiButton.setPressed(1)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiButton.java:16)
- [ ] `GuiButton.setHoverEffect(1)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiButton.java:22)

### `GuiButton.GuiButtonDist`: it's an interface

- [ ] `GuiButton.GuiButtonDist.setPressed(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiButton.java:31)
- [ ] `GuiButton.GuiButtonDist.setHoverEffect(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiButton.java:33)

### `GuiButtonContext`: it extends `GuiButton`

- [ ] `GuiButtonContext.GuiButtonContext(3)`: its class isn't translated (it extends `GuiButton`) (team/creative/creativecore/common/gui/control/simple/GuiButtonContext.java:8)
- [ ] `GuiButtonContext.dist(0)`: its class isn't translated (it extends `GuiButton`) (team/creative/creativecore/common/gui/control/simple/GuiButtonContext.java:15)
- [ ] `GuiButtonContext.set(1)`: its class isn't translated (it extends `GuiButton`) (team/creative/creativecore/common/gui/control/simple/GuiButtonContext.java:19)

### `GuiButtonContext.GuiButtonContextDist`: it's an interface

- [ ] `GuiButtonContext.GuiButtonContextDist.set(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiButtonContext.java:28)

### `GuiButtonHold`: it's generic

- [ ] `GuiButtonHold.GuiButtonHold(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiButtonHold.java:7)
- [ ] `GuiButtonHold.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiButtonHold.java:11)
- [ ] `GuiButtonHold.tick(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiButtonHold.java:15)

### `GuiButtonHold.GuiButtonHoldDist`: it's an interface

- [ ] `GuiButtonHold.GuiButtonHoldDist.tick(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiButtonHold.java:23)

### `GuiButtonIcon`: it extends `GuiIcon`

- [ ] `GuiButtonIcon.GuiButtonIcon(4)`: its class isn't translated (it extends `GuiIcon`) (team/creative/creativecore/common/gui/control/simple/GuiButtonIcon.java:9)
- [ ] `GuiButtonIcon.dist(0)`: its class isn't translated (it extends `GuiIcon`) (team/creative/creativecore/common/gui/control/simple/GuiButtonIcon.java:16)
- [ ] `GuiButtonIcon.setIcon(1)`: its class isn't translated (it extends `GuiIcon`) (team/creative/creativecore/common/gui/control/simple/GuiButtonIcon.java:20)
- [ ] `GuiButtonIcon.setColor(1)`: its class isn't translated (it extends `GuiIcon`) (team/creative/creativecore/common/gui/control/simple/GuiButtonIcon.java:25)
- [ ] `GuiButtonIcon.setShadow(1)`: its class isn't translated (it extends `GuiIcon`) (team/creative/creativecore/common/gui/control/simple/GuiButtonIcon.java:30)
- [ ] `GuiButtonIcon.setSquared(1)`: its class isn't translated (it extends `GuiIcon`) (team/creative/creativecore/common/gui/control/simple/GuiButtonIcon.java:35)

### `GuiButtonIcon.GuiButtonIconDist`: it's an interface

- [ ] `GuiButtonIcon.GuiButtonIconDist.setPressed(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiButtonIcon.java:41)

### `GuiCheckBox`: it extends `GuiLabel`

- [ ] `GuiCheckBox.GuiCheckBox(3)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:9)
- [ ] `GuiCheckBox.dist(0)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:16)
- [ ] `GuiCheckBox.set(1)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:20)
- [ ] `GuiCheckBox.get(0)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:26)
- [ ] `GuiCheckBox.setPartial(1)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:30)
- [ ] `GuiCheckBox.getPartial(0)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:38)
- [ ] `GuiCheckBox.consumeChanged(1)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:42)
- [ ] `GuiCheckBox.setTranslate(1)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:47)
- [ ] `GuiCheckBox.setTitle(1)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:51)
- [ ] `GuiCheckBox.setTitle(1)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:55)

### `GuiCheckBox.GuiCheckBoxDist`: it's an interface

- [ ] `GuiCheckBox.GuiCheckBoxDist.consumeChanged(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:60)
- [ ] `GuiCheckBox.GuiCheckBoxDist.set(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:62)
- [ ] `GuiCheckBox.GuiCheckBoxDist.get(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:64)
- [ ] `GuiCheckBox.GuiCheckBoxDist.setPartial(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:66)
- [ ] `GuiCheckBox.GuiCheckBoxDist.getPartial(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiCheckBox.java:68)

### `GuiCheckButtonIcon`: it extends `GuiButtonIcon`

- [ ] `GuiCheckButtonIcon.GuiCheckButtonIcon(5)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:10)
- [ ] `GuiCheckButtonIcon.GuiCheckButtonIcon(6)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:14)
- [ ] `GuiCheckButtonIcon.dist(0)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:23)
- [ ] `GuiCheckButtonIcon.setOnIcon(1)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:27)
- [ ] `GuiCheckButtonIcon.setOffIcon(1)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:35)
- [ ] `GuiCheckButtonIcon.setColor(1)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:43)
- [ ] `GuiCheckButtonIcon.setShadow(1)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:47)
- [ ] `GuiCheckButtonIcon.setSquared(1)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:51)
- [ ] `GuiCheckButtonIcon.setDim(1)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:55)
- [ ] `GuiCheckButtonIcon.setDim(2)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:60)
- [ ] `GuiCheckButtonIcon.getState(0)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:65)
- [ ] `GuiCheckButtonIcon.setState(1)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:69)

### `GuiCheckButtonIcon.GuiCheckButtonIconDist`: it's an interface

- [ ] `GuiCheckButtonIcon.GuiCheckButtonIconDist.setOnIcon(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:76)
- [ ] `GuiCheckButtonIcon.GuiCheckButtonIconDist.setOffIcon(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:78)
- [ ] `GuiCheckButtonIcon.GuiCheckButtonIconDist.getState(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:80)
- [ ] `GuiCheckButtonIcon.GuiCheckButtonIconDist.setState(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiCheckButtonIcon.java:82)

### `GuiColoredSteppedSlider`: it extends `GuiSteppedSlider`

- [ ] `GuiColoredSteppedSlider.GuiColoredSteppedSlider(4)`: its class isn't translated (it extends `GuiSteppedSlider`) (team/creative/creativecore/common/gui/control/simple/GuiColoredSteppedSlider.java:7)
- [ ] `GuiColoredSteppedSlider.setValueSilent(1)`: its class isn't translated (it extends `GuiSteppedSlider`) (team/creative/creativecore/common/gui/control/simple/GuiColoredSteppedSlider.java:15)
- [ ] `GuiColoredSteppedSlider.dist(0)`: its class isn't translated (it extends `GuiSteppedSlider`) (team/creative/creativecore/common/gui/control/simple/GuiColoredSteppedSlider.java:21)

### `GuiColoredSteppedSlider.GuiColoredSteppedSliderDist`: it's an interface

- [ ] `GuiColoredSteppedSlider.GuiColoredSteppedSliderDist.setPicker(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiColoredSteppedSlider.java:26)
- [ ] `GuiColoredSteppedSlider.GuiColoredSteppedSliderDist.setPart(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiColoredSteppedSlider.java:28)
- [ ] `GuiColoredSteppedSlider.GuiColoredSteppedSliderDist.setValueSilent(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiColoredSteppedSlider.java:30)

### `GuiColorPicker`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiColorPicker.GuiColorPicker(5)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiColorPicker.java:25)
- [ ] `GuiColorPicker.setColor(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiColorPicker.java:72)
- [ ] `GuiColorPicker.onColorChanged(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiColorPicker.java:83)

### `GuiColorPicker.GuiHexButton`: it extends `GuiButton`

- [ ] `GuiColorPicker.GuiHexButton.GuiHexButton(4)`: its class isn't translated (it extends `GuiButton`) (team/creative/creativecore/common/gui/control/simple/GuiColorPicker.java:92)
- [ ] `GuiColorPicker.GuiHexButton.dist(0)`: its class isn't translated (it extends `GuiButton`) (team/creative/creativecore/common/gui/control/simple/GuiColorPicker.java:101)
- [ ] `GuiColorPicker.GuiHexButton.colorChanged(0)`: its class isn't translated (it extends `GuiButton`) (team/creative/creativecore/common/gui/control/simple/GuiColorPicker.java:105)

### `GuiColorPicker.GuiHexButtonDist`: it's an interface

- [ ] `GuiColorPicker.GuiHexButtonDist.setConsumer(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiColorPicker.java:113)
- [ ] `GuiColorPicker.GuiHexButtonDist.colorChanged(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiColorPicker.java:115)

### `GuiColorPlate`: it extends `GuiControl` (team.creative.creativecore.common.gui)

- [ ] `GuiColorPlate.GuiColorPlate(3)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiColorPlate.java:9)
- [ ] `GuiColorPlate.dist(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiColorPlate.java:16)
- [ ] `GuiColorPlate.setColor(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiColorPlate.java:20)
- [ ] `GuiColorPlate.getColor(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiColorPlate.java:26)
- [ ] `GuiColorPlate.init(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiColorPlate.java:30)
- [ ] `GuiColorPlate.closed(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiColorPlate.java:34)
- [ ] `GuiColorPlate.tick(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiColorPlate.java:38)

### `GuiColorPlate.GuiColorPlateDist`: it's an interface

- [ ] `GuiColorPlate.GuiColorPlateDist.setColor(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiColorPlate.java:43)
- [ ] `GuiColorPlate.GuiColorPlateDist.getColor(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiColorPlate.java:45)

### `GuiCounter`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiCounter.GuiCounter(3)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:19)
- [ ] `GuiCounter.GuiCounter(5)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:23)
- [ ] `GuiCounter.GuiCounter(6)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:27)
- [ ] `GuiCounter.createButtons(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:42)
- [ ] `GuiCounter.setSpacing(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:53)
- [ ] `GuiCounter.add(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:57)
- [ ] `GuiCounter.isExpandableX(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:61)
- [ ] `GuiCounter.getPlusButton(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:66)
- [ ] `GuiCounter.getMinusButton(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:70)
- [ ] `GuiCounter.resetTextfield(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:74)
- [ ] `GuiCounter.raiseEvent(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:78)
- [ ] `GuiCounter.stepUp(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:87)
- [ ] `GuiCounter.stepDown(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:91)
- [ ] `GuiCounter.getValue(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:95)
- [ ] `GuiCounter.setValue(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounter.java:99)

### `GuiCounterDecimal`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiCounterDecimal.FORMAT`: it uses `DecimalFormat` (java.text) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:15)
- [ ] `GuiCounterDecimal.GuiCounterDecimal(3)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:23)
- [ ] `GuiCounterDecimal.GuiCounterDecimal(5)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:27)
- [ ] `GuiCounterDecimal.GuiCounterDecimal(6)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:31)
- [ ] `GuiCounterDecimal.createButtons(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:47)
- [ ] `GuiCounterDecimal.setSpacing(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:60)
- [ ] `GuiCounterDecimal.add(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:64)
- [ ] `GuiCounterDecimal.isExpandableX(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:68)
- [ ] `GuiCounterDecimal.getPlusButton(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:73)
- [ ] `GuiCounterDecimal.getMinusButton(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:77)
- [ ] `GuiCounterDecimal.setStep(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:81)
- [ ] `GuiCounterDecimal.getFormat(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:86)
- [ ] `GuiCounterDecimal.resetTextfield(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:90)
- [ ] `GuiCounterDecimal.raiseEvent(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:94)
- [ ] `GuiCounterDecimal.stepUp(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:103)
- [ ] `GuiCounterDecimal.stepDown(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:107)
- [ ] `GuiCounterDecimal.getValue(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:111)
- [ ] `GuiCounterDecimal.setValue(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiCounterDecimal.java:115)

### `GuiDuration`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [x] `GuiDuration.TICKS_PER_SECOND` (team/creative/creativecore/common/gui/control/simple/GuiDuration.java:8)
- [x] `GuiDuration.TICKS_PER_MINUTE` (team/creative/creativecore/common/gui/control/simple/GuiDuration.java:9)
- [x] `GuiDuration.TICKS_PER_HOUR` (team/creative/creativecore/common/gui/control/simple/GuiDuration.java:10)
- [x] `GuiDuration.TICKS_PER_DAY` (team/creative/creativecore/common/gui/control/simple/GuiDuration.java:11)
- [ ] `GuiDuration.GuiDuration(7)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiDuration.java:17)
- [ ] `GuiDuration.setDuration(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiDuration.java:43)
- [ ] `GuiDuration.getDuration(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiDuration.java:65)

### `GuiIcon`: it extends `GuiControl` (team.creative.creativecore.common.gui)

- [ ] `GuiIcon.GuiIcon(3)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:11)
- [ ] `GuiIcon.dist(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:18)
- [ ] `GuiIcon.setIcon(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:22)
- [ ] `GuiIcon.setColor(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:30)
- [ ] `GuiIcon.setShadow(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:38)
- [ ] `GuiIcon.setSquared(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:46)
- [ ] `GuiIcon.setDim(2)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:54)
- [ ] `GuiIcon.setDim(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:59)
- [ ] `GuiIcon.init(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:64)
- [ ] `GuiIcon.closed(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:68)
- [ ] `GuiIcon.tick(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:72)

### `GuiIcon.GuiIconDist`: it's an interface

- [ ] `GuiIcon.GuiIconDist.setIcon(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:77)
- [ ] `GuiIcon.GuiIconDist.setColor(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:79)
- [ ] `GuiIcon.GuiIconDist.setShadow(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:81)
- [ ] `GuiIcon.GuiIconDist.setSquared(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiIcon.java:83)

### `GuiLabel`: it extends `GuiControl` (team.creative.creativecore.common.gui)

- [ ] `GuiLabel.GuiLabel(2)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:12)
- [ ] `GuiLabel.dist(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:16)
- [ ] `GuiLabel.setDefaultColor(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:20)
- [ ] `GuiLabel.setDropShadow(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:28)
- [ ] `GuiLabel.setAlign(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:36)
- [ ] `GuiLabel.setVAlign(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:44)
- [ ] `GuiLabel.setTranslate(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:52)
- [ ] `GuiLabel.setTranslate(2)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:56)
- [ ] `GuiLabel.setTitle(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:60)
- [ ] `GuiLabel.setTitle(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:72)
- [ ] `GuiLabel.setScale(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:84)
- [ ] `GuiLabel.init(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:92)
- [ ] `GuiLabel.closed(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:96)
- [ ] `GuiLabel.tick(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:100)

### `GuiLabel.GuiLabelDist`: it's an interface

- [ ] `GuiLabel.GuiLabelDist.setDefaultColor(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:105)
- [ ] `GuiLabel.GuiLabelDist.setDropShadow(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:107)
- [ ] `GuiLabel.GuiLabelDist.setAlign(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:109)
- [ ] `GuiLabel.GuiLabelDist.setVAlign(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:111)
- [ ] `GuiLabel.GuiLabelDist.setTitle(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:113)
- [ ] `GuiLabel.GuiLabelDist.setTitle(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:115)
- [ ] `GuiLabel.GuiLabelDist.setScale(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiLabel.java:117)

### `GuiLabelHover`: it extends `GuiLabel`

- [ ] `GuiLabelHover.GuiLabelHover(2)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiLabelHover.java:6)

### `GuiListEntry`: it extends `GuiLabel`

- [ ] `GuiListEntry.GuiListEntry(5)`: its class isn't translated (it extends `GuiLabel`) (team/creative/creativecore/common/gui/control/simple/GuiListEntry.java:11)

### `GuiProgressbar`: it extends `GuiControl` (team.creative.creativecore.common.gui)

- [ ] `GuiProgressbar.GuiProgressbar(4)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiProgressbar.java:13)
- [ ] `GuiProgressbar.GuiProgressbar(5)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiProgressbar.java:17)
- [ ] `GuiProgressbar.init(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiProgressbar.java:24)
- [ ] `GuiProgressbar.closed(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiProgressbar.java:28)
- [ ] `GuiProgressbar.tick(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiProgressbar.java:32)
- [ ] `GuiProgressbar.getPercentage(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiProgressbar.java:36)

### `GuiSeekBar`: it extends `GuiControl` (team.creative.creativecore.common.gui)

- [ ] `GuiSeekBar.GuiSeekBar(4)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:13)
- [ ] `GuiSeekBar.GuiSeekBar(5)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:17)
- [ ] `GuiSeekBar.dist(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:26)
- [ ] `GuiSeekBar.setOnTimeUpdate(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:30)
- [ ] `GuiSeekBar.setOnLastTimeUpdate(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:38)
- [ ] `GuiSeekBar.setPosition(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:46)
- [ ] `GuiSeekBar.setTooltip(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:52)
- [ ] `GuiSeekBar.init(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:57)
- [ ] `GuiSeekBar.closed(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:61)
- [ ] `GuiSeekBar.tick(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:65)

### `GuiSeekBar.GuiSeekBarDist`: it's an interface

- [ ] `GuiSeekBar.GuiSeekBarDist.init(3)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:73)
- [ ] `GuiSeekBar.GuiSeekBarDist.setOnTimeUpdate(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:75)
- [ ] `GuiSeekBar.GuiSeekBarDist.setOnLastTimeUpdate(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:77)
- [ ] `GuiSeekBar.GuiSeekBarDist.setPosition(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:79)
- [ ] `GuiSeekBar.GuiSeekBarDist.tick(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSeekBar.java:81)

### `GuiShowItem`: it extends `GuiControl` (team.creative.creativecore.common.gui)

- [ ] `GuiShowItem.GuiShowItem(2)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiShowItem.java:10)
- [ ] `GuiShowItem.GuiShowItem(3)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiShowItem.java:14)
- [ ] `GuiShowItem.init(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiShowItem.java:19)
- [ ] `GuiShowItem.closed(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiShowItem.java:23)
- [ ] `GuiShowItem.tick(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiShowItem.java:27)

### `GuiSlider`: it extends `GuiControl` (team.creative.creativecore.common.gui)

- [ ] `GuiSlider.GuiSlider(5)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:13)
- [ ] `GuiSlider.GuiSlider(6)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:17)
- [ ] `GuiSlider.dist(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:26)
- [ ] `GuiSlider.getTextByValue(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:30)
- [ ] `GuiSlider.getTextfieldValue(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:34)
- [ ] `GuiSlider.getPercentage(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:38)
- [ ] `GuiSlider.closeTextField(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:42)
- [ ] `GuiSlider.setMaxValue(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:48)
- [ ] `GuiSlider.setMinValue(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:54)
- [ ] `GuiSlider.setValue(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:60)
- [ ] `GuiSlider.getValue(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:66)
- [ ] `GuiSlider.getMinValue(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:70)
- [ ] `GuiSlider.getMaxValue(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:74)
- [ ] `GuiSlider.setSliderSize(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:78)
- [ ] `GuiSlider.setMinSlider(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:86)
- [ ] `GuiSlider.setMaxSlider(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:94)
- [ ] `GuiSlider.isContainer(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:102)
- [ ] `GuiSlider.closeTopLayer(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:107)
- [ ] `GuiSlider.toLayerRect(2)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:112)
- [ ] `GuiSlider.toScreenRect(2)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:117)
- [ ] `GuiSlider.init(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:122)
- [ ] `GuiSlider.closed(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:126)
- [ ] `GuiSlider.tick(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:130)
- [ ] `GuiSlider.closeLayer(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:134)
- [ ] `GuiSlider.createDist(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:139)
- [ ] `GuiSlider.createDist(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:144)

### `GuiSlider.GuiSliderDist`: it's an interface

- [ ] `GuiSlider.GuiSliderDist.setParser(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:150)
- [ ] `GuiSlider.GuiSliderDist.getTextByValue(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:152)
- [ ] `GuiSlider.GuiSliderDist.getTextfieldValue(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:154)
- [ ] `GuiSlider.GuiSliderDist.getPercentage(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:156)
- [ ] `GuiSlider.GuiSliderDist.closeTextField(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:158)
- [ ] `GuiSlider.GuiSliderDist.setBounds(3)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:160)
- [ ] `GuiSlider.GuiSliderDist.setMaxValue(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:162)
- [ ] `GuiSlider.GuiSliderDist.setMinValue(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:164)
- [ ] `GuiSlider.GuiSliderDist.setValue(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:166)
- [ ] `GuiSlider.GuiSliderDist.getValue(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:168)
- [ ] `GuiSlider.GuiSliderDist.getMinValue(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:170)
- [ ] `GuiSlider.GuiSliderDist.getMaxValue(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:172)
- [ ] `GuiSlider.GuiSliderDist.setSliderSize(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:174)
- [ ] `GuiSlider.GuiSliderDist.setMinSlider(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:176)
- [ ] `GuiSlider.GuiSliderDist.setMaxSlider(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSlider.java:178)

### `GuiStateButton`: it's generic

- [ ] `GuiStateButton.GuiStateButton(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:8)
- [ ] `GuiStateButton.GuiStateButton(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:12)
- [ ] `GuiStateButton.GuiStateButton(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:27)
- [ ] `GuiStateButton.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:32)
- [ ] `GuiStateButton.set(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:36)
- [ ] `GuiStateButton.selected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:42)
- [ ] `GuiStateButton.selected(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:47)
- [ ] `GuiStateButton.select(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:51)
- [ ] `GuiStateButton.select(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:57)
- [ ] `GuiStateButton.indexOf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:63)
- [ ] `GuiStateButton.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:67)
- [ ] `GuiStateButton.previous(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:73)

### `GuiStateButton.GuiStateButtonDist`: it's an interface

- [ ] `GuiStateButton.GuiStateButtonDist.set(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:80)
- [ ] `GuiStateButton.GuiStateButtonDist.selected(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:82)
- [ ] `GuiStateButton.GuiStateButtonDist.selected(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:84)
- [ ] `GuiStateButton.GuiStateButtonDist.select(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:86)
- [ ] `GuiStateButton.GuiStateButtonDist.select(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:88)
- [ ] `GuiStateButton.GuiStateButtonDist.indexOf(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:90)
- [ ] `GuiStateButton.GuiStateButtonDist.next(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:92)
- [ ] `GuiStateButton.GuiStateButtonDist.previous(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButton.java:94)

### `GuiStateButtonIcon`: it extends `GuiButtonIcon`

- [ ] `GuiStateButtonIcon.GuiStateButtonIcon(3)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:8)
- [ ] `GuiStateButtonIcon.setState(1)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:15)
- [ ] `GuiStateButtonIcon.dist(0)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:23)
- [ ] `GuiStateButtonIcon.getState(0)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:27)
- [ ] `GuiStateButtonIcon.setShadow(1)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:31)
- [ ] `GuiStateButtonIcon.setColor(1)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:35)
- [ ] `GuiStateButtonIcon.previousState(0)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:39)
- [ ] `GuiStateButtonIcon.nextState(0)`: its class isn't translated (it extends `GuiButtonIcon`) (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:45)

### `GuiStateButtonIcon.GuiStateButtonIconDist`: it's an interface

- [ ] `GuiStateButtonIcon.GuiStateButtonIconDist.getState(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:52)
- [ ] `GuiStateButtonIcon.GuiStateButtonIconDist.setState(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:54)
- [ ] `GuiStateButtonIcon.GuiStateButtonIconDist.setIcons(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:56)
- [ ] `GuiStateButtonIcon.GuiStateButtonIconDist.previousState(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:58)
- [ ] `GuiStateButtonIcon.GuiStateButtonIconDist.nextState(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiStateButtonIcon.java:60)

### `GuiSteppedSlider`: it extends `GuiSlider`

- [ ] `GuiSteppedSlider.GuiSteppedSlider(5)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:7)
- [ ] `GuiSteppedSlider.GuiSteppedSlider(6)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:11)
- [ ] `GuiSteppedSlider.dist(0)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:18)
- [ ] `GuiSteppedSlider.stepUp(0)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:22)
- [ ] `GuiSteppedSlider.stepDown(0)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:26)
- [ ] `GuiSteppedSlider.setValue(1)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:30)
- [ ] `GuiSteppedSlider.setMaxValue(1)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:35)
- [ ] `GuiSteppedSlider.setMinValue(1)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:40)
- [ ] `GuiSteppedSlider.getValue(0)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:45)
- [ ] `GuiSteppedSlider.getMinValue(0)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:50)
- [ ] `GuiSteppedSlider.getMaxValue(0)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:55)
- [ ] `GuiSteppedSlider.getIntValue(0)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:60)
- [ ] `GuiSteppedSlider.getIntMaxValue(0)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:64)
- [ ] `GuiSteppedSlider.getIntMinValue(0)`: its class isn't translated (it extends `GuiSlider`) (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:68)

### `GuiSteppedSlider.GuiSteppedSliderDist`: it's an interface

- [ ] `GuiSteppedSlider.GuiSteppedSliderDist.setSteppedParser(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:73)
- [ ] `GuiSteppedSlider.GuiSteppedSliderDist.getIntValue(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:75)
- [ ] `GuiSteppedSlider.GuiSteppedSliderDist.getIntMaxValue(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:77)
- [ ] `GuiSteppedSlider.GuiSteppedSliderDist.getIntMinValue(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiSteppedSlider.java:79)

### `GuiTabButton`: it's generic

- [ ] `GuiTabButton.BUTTON_ACTIVE`: it uses ControlFormatting, which isn't translated (it's an interface) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:11)
- [ ] `GuiTabButton.BUTTON_INACTIVE`: it uses ControlFormatting, which isn't translated (it's an interface) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:14)
- [ ] `GuiTabButton.GuiTabButton(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:18)
- [ ] `GuiTabButton.GuiTabButton(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:22)
- [ ] `GuiTabButton.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:31)
- [ ] `GuiTabButton.set(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:35)
- [ ] `GuiTabButton.selected(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:41)
- [ ] `GuiTabButton.selected(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:46)
- [ ] `GuiTabButton.select(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:50)
- [ ] `GuiTabButton.select(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:56)
- [ ] `GuiTabButton.indexOf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:62)
- [ ] `GuiTabButton.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:66)
- [ ] `GuiTabButton.previous(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:72)
- [ ] `GuiTabButton.index(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:78)

### `GuiTabButton.GuiTabButtonDist`: it's an interface

- [ ] `GuiTabButton.GuiTabButtonDist.set(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:83)
- [ ] `GuiTabButton.GuiTabButtonDist.selected(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:85)
- [ ] `GuiTabButton.GuiTabButtonDist.selected(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:88)
- [ ] `GuiTabButton.GuiTabButtonDist.select(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:90)
- [ ] `GuiTabButton.GuiTabButtonDist.select(2)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:92)
- [ ] `GuiTabButton.GuiTabButtonDist.indexOf(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:94)
- [ ] `GuiTabButton.GuiTabButtonDist.next(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:96)
- [ ] `GuiTabButton.GuiTabButtonDist.previous(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:98)
- [ ] `GuiTabButton.GuiTabButtonDist.index(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTabButton.java:100)

### `GuiTextfield`: it extends `GuiControl` (team.creative.creativecore.common.gui)

- [ ] `GuiTextfield.GuiTextfield(2)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:9)
- [ ] `GuiTextfield.GuiTextfield(3)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:14)
- [ ] `GuiTextfield.GuiTextfield(4)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:19)
- [ ] `GuiTextfield.dist(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:28)
- [ ] `GuiTextfield.setDim(2)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:32)
- [ ] `GuiTextfield.setDim(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:36)
- [ ] `GuiTextfield.setFloatOnly(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:40)
- [ ] `GuiTextfield.setNumbersIncludingNegativeOnly(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:48)
- [ ] `GuiTextfield.setNumbersOnly(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:56)
- [ ] `GuiTextfield.setHexOnly(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:64)
- [ ] `GuiTextfield.parseFloat(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:72)
- [ ] `GuiTextfield.parseDouble(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:80)
- [ ] `GuiTextfield.parseInteger(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:88)
- [ ] `GuiTextfield.parseLong(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:96)
- [ ] `GuiTextfield.init(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:104)
- [ ] `GuiTextfield.closed(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:108)
- [ ] `GuiTextfield.tick(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:112)
- [ ] `GuiTextfield.setTextSilent(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:116)
- [ ] `GuiTextfield.setText(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:122)
- [ ] `GuiTextfield.setText(2)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:130)
- [ ] `GuiTextfield.getText(0)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:138)
- [ ] `GuiTextfield.setValidator(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:142)
- [ ] `GuiTextfield.setMaxStringLength(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:148)
- [ ] `GuiTextfield.setSuggestion(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:156)
- [ ] `GuiTextfield.moveCursorToEnd(1)`: its class isn't translated (it extends `GuiControl` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:164)

### `GuiTextfield.GuiTextfieldDist`: it's an interface

- [ ] `GuiTextfield.GuiTextfieldDist.setFloatOnly(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:171)
- [ ] `GuiTextfield.GuiTextfieldDist.setNumbersIncludingNegativeOnly(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:173)
- [ ] `GuiTextfield.GuiTextfieldDist.setNumbersOnly(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:175)
- [ ] `GuiTextfield.GuiTextfieldDist.setHexOnly(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:177)
- [ ] `GuiTextfield.GuiTextfieldDist.setText(3)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:179)
- [ ] `GuiTextfield.GuiTextfieldDist.getText(0)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:181)
- [ ] `GuiTextfield.GuiTextfieldDist.setValidator(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:183)
- [ ] `GuiTextfield.GuiTextfieldDist.setSuggestion(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:185)
- [ ] `GuiTextfield.GuiTextfieldDist.setMaxStringLength(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:187)
- [ ] `GuiTextfield.GuiTextfieldDist.moveCursorToEnd(1)`: it has no body (team/creative/creativecore/common/gui/control/simple/GuiTextfield.java:189)

### `GuiAnimationHandler`: it's an interface

- [ ] `GuiAnimationHandler.loop(1)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiAnimationHandler.java:4)
- [ ] `GuiAnimationHandler.play(0)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiAnimationHandler.java:6)
- [ ] `GuiAnimationHandler.pause(0)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiAnimationHandler.java:8)
- [ ] `GuiAnimationHandler.stop(0)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiAnimationHandler.java:10)
- [ ] `GuiAnimationHandler.set(1)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiAnimationHandler.java:12)
- [ ] `GuiAnimationHandler.get(0)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiAnimationHandler.java:14)

### `GuiTimeline`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiTimeline.GuiTimeline(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:34)
- [ ] `GuiTimeline.channels(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:61)
- [ ] `GuiTimeline.getChannelParent(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:65)
- [ ] `GuiTimeline.dist(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:69)
- [ ] `GuiTimeline.getDuration(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:73)
- [ ] `GuiTimeline.setDuration(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:77)
- [ ] `GuiTimeline.selectKey(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:83)
- [ ] `GuiTimeline.deselect(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:93)
- [ ] `GuiTimeline.adjustKeyPositionX(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:101)
- [ ] `GuiTimeline.adjustKeysPositionX(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:107)
- [ ] `GuiTimeline.setSidebarWidth(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:113)
- [ ] `GuiTimeline.addGuiTimelineChannel(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:118)
- [ ] `GuiTimeline.removeChannel(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:128)
- [ ] `GuiTimeline.isExpandableX(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:135)

### `GuiTimeline.GuiTimelineDist`: it's an interface

- [ ] `GuiTimeline.GuiTimelineDist.afterDurationSet(0)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:141)
- [ ] `GuiTimeline.GuiTimelineDist.adjustKeyPositionX(1)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:143)
- [ ] `GuiTimeline.GuiTimelineDist.adjustKeysPositionX(0)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:145)

### `GuiTimeline.KeySelectedEvent`: it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event)

- [ ] `GuiTimeline.KeySelectedEvent.KeySelectedEvent(1)`: its class isn't translated (it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:149)
- [ ] `GuiTimeline.KeySelectedEvent.cancelable(0)`: its class isn't translated (it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:153)

### `GuiTimeline.NoKeySelectedEvent`: it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event)

- [ ] `GuiTimeline.NoKeySelectedEvent.NoKeySelectedEvent(1)`: its class isn't translated (it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:160)
- [ ] `GuiTimeline.NoKeySelectedEvent.cancelable(0)`: its class isn't translated (it extends `GuiControlEvent` (team.creative.creativecore.common.gui.event)) (team/creative/creativecore/common/gui/control/timeline/GuiTimeline.java:164)

### `GuiTimelineChannel`: it's generic

- [ ] `GuiTimelineChannel.GuiTimelineChannel(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:18)
- [ ] `GuiTimelineChannel.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:24)
- [ ] `GuiTimelineChannel.setTooltip(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:28)
- [ ] `GuiTimelineChannel.addKeyFixed(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:34)
- [ ] `GuiTimelineChannel.addKey(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:40)
- [ ] `GuiTimelineChannel.select(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:66)
- [ ] `GuiTimelineChannel.deslect(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:70)
- [ ] `GuiTimelineChannel.removeChannel(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:74)
- [ ] `GuiTimelineChannel.removeKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:78)
- [ ] `GuiTimelineChannel.movedKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:84)
- [ ] `GuiTimelineChannel.dragKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:89)
- [ ] `GuiTimelineChannel.isSpaceFor(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:95)
- [ ] `GuiTimelineChannel.getValueAt(1)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:114)
- [ ] `GuiTimelineChannel.isChannelEmpty(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:116)
- [ ] `GuiTimelineChannel.keys(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:120)
- [ ] `GuiTimelineChannel.getFirst(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:124)
- [ ] `GuiTimelineChannel.getLast(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:128)

### `GuiTimelineChannel.GuiTimelineChannelDist`: it's an interface

- [ ] `GuiTimelineChannel.GuiTimelineChannelDist.keyInitialFlow(1)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:133)
- [ ] `GuiTimelineChannel.GuiTimelineChannelDist.dragKey(1)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannel.java:135)

### `GuiTimelineChannelDouble`: it extends `GuiTimelineChannel`

- [ ] `GuiTimelineChannelDouble.GuiTimelineChannelDouble(1)`: its class isn't translated (it extends `GuiTimelineChannel`) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannelDouble.java:4)
- [ ] `GuiTimelineChannelDouble.getValueAt(1)`: its class isn't translated (it extends `GuiTimelineChannel`) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineChannelDouble.java:8)

### `GuiTimelineHeader`: it extends `GuiColumnHeader`

- [ ] `GuiTimelineHeader.GuiTimelineHeader(1)`: its class isn't translated (it extends `GuiColumnHeader`) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineHeader.java:8)

### `GuiTimelineKey`: it's generic

- [ ] `GuiTimelineKey.GuiTimelineKey(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineKey.java:12)
- [ ] `GuiTimelineKey.dist(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineKey.java:19)
- [ ] `GuiTimelineKey.setSelected(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineKey.java:23)
- [ ] `GuiTimelineKey.removeKey(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineKey.java:29)
- [ ] `GuiTimelineKey.compareTo(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineKey.java:33)
- [ ] `GuiTimelineKey.init(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineKey.java:37)
- [ ] `GuiTimelineKey.closed(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineKey.java:41)
- [ ] `GuiTimelineKey.tick(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/control/timeline/GuiTimelineKey.java:45)

### `GuiTimelineKey.GuiTimelineKeyDist`: it's an interface

- [ ] `GuiTimelineKey.GuiTimelineKeyDist.setSelected(1)`: it has no body (team/creative/creativecore/common/gui/control/timeline/GuiTimelineKey.java:50)

### `GuiTree`: it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)

- [ ] `GuiTree.GuiTree(2)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:22)
- [ ] `GuiTree.GuiTree(3)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:26)
- [ ] `GuiTree.dist(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:40)
- [ ] `GuiTree.setCheckboxes(2)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:44)
- [ ] `GuiTree.setKeepOrder(1)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:61)
- [ ] `GuiTree.keepOrder(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:69)
- [ ] `GuiTree.keepSelected(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:73)
- [ ] `GuiTree.canDeselect(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:78)
- [ ] `GuiTree.hasCheckboxes(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:82)
- [ ] `GuiTree.hasCheckboxesPartial(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:86)
- [ ] `GuiTree.allItems(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:90)
- [ ] `GuiTree.itemsChecked(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:94)
- [ ] `GuiTree.selected(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:98)
- [ ] `GuiTree.getFirst(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:102)
- [ ] `GuiTree.selectFirst(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:110)
- [ ] `GuiTree.select(1)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:125)
- [ ] `GuiTree.setLineThickness(1)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:139)
- [ ] `GuiTree.setRootVisibility(1)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:147)
- [ ] `GuiTree.isRootVisible(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:152)
- [ ] `GuiTree.root(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:156)
- [ ] `GuiTree.updateTree(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:160)
- [ ] `GuiTree.addItem(1)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:179)
- [ ] `GuiTree.performModication(2)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:188)
- [ ] `GuiTree.moveUp(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:208)
- [ ] `GuiTree.moveDown(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:224)
- [ ] `GuiTree.isDragged(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:240)
- [ ] `GuiTree.startDrag(1)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:244)
- [ ] `GuiTree.endDrag(0)`: its class isn't translated (it extends `GuiScrollXY` (team.creative.creativecore.common.gui.control.parent)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:250)

### `GuiTree.GuiTreeDist`: it's an interface

- [ ] `GuiTree.GuiTreeDist.reflowTree(0)`: it has no body (team/creative/creativecore/common/gui/control/tree/GuiTree.java:255)
- [ ] `GuiTree.GuiTreeDist.setLineThickness(1)`: it has no body (team/creative/creativecore/common/gui/control/tree/GuiTree.java:257)
- [ ] `GuiTree.GuiTreeDist.isDragged(0)`: it has no body (team/creative/creativecore/common/gui/control/tree/GuiTree.java:259)
- [ ] `GuiTree.GuiTreeDist.startDrag(1)`: it has no body (team/creative/creativecore/common/gui/control/tree/GuiTree.java:261)
- [ ] `GuiTree.GuiTreeDist.endDrag(0)`: it has no body (team/creative/creativecore/common/gui/control/tree/GuiTree.java:263)
- [ ] `GuiTree.GuiTreeDist.setKeepOrder(1)`: it has no body (team/creative/creativecore/common/gui/control/tree/GuiTree.java:265)
- [ ] `GuiTree.GuiTreeDist.keepOrder(0)`: it has no body (team/creative/creativecore/common/gui/control/tree/GuiTree.java:267)

### `GuiTree.GuiTreeSelectionChanged`: it extends `GuiControlChangedEvent` (team.creative.creativecore.common.gui.event)

- [ ] `GuiTree.GuiTreeSelectionChanged.GuiTreeSelectionChanged(3)`: its class isn't translated (it extends `GuiControlChangedEvent` (team.creative.creativecore.common.gui.event)) (team/creative/creativecore/common/gui/control/tree/GuiTree.java:274)

### `GuiTreeDragPosition`: it uses ItemPosition, which isn't translated (its constants carry values)

- [ ] `GuiTreeDragPosition.GuiTreeDragPosition(3)`: its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)) (team/creative/creativecore/common/gui/control/tree/GuiTreeDragPosition.java:6)
- [ ] `GuiTreeDragPosition.insert(1)`: its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)) (team/creative/creativecore/common/gui/control/tree/GuiTreeDragPosition.java:7)
- [ ] `GuiTreeDragPosition.toString(0)`: its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)) (team/creative/creativecore/common/gui/control/tree/GuiTreeDragPosition.java:11)
- [ ] `GuiTreeDragPosition.position(0)`: its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)) (team/creative/creativecore/common/gui/control/tree/GuiTreeDragPosition.java:6)
- [ ] `GuiTreeDragPosition.item(0)`: its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)) (team/creative/creativecore/common/gui/control/tree/GuiTreeDragPosition.java:6)
- [ ] `GuiTreeDragPosition.above(0)`: its class isn't translated (it uses ItemPosition, which isn't translated (its constants carry values)) (team/creative/creativecore/common/gui/control/tree/GuiTreeDragPosition.java:6)

### `GuiTreeDragPosition.ItemPosition`: its constants carry values

- [ ] `GuiTreeDragPosition.ItemPosition.insert(2)`: it has no body (team/creative/creativecore/common/gui/control/tree/GuiTreeDragPosition.java:43)

### `GuiTreeItem`: it extends `GuiParent` (team.creative.creativecore.common.gui)

- [ ] `GuiTreeItem.GuiTreeItem(3)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:29)
- [ ] `GuiTreeItem.setTranslate(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:41)
- [ ] `GuiTreeItem.setTitle(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:46)
- [ ] `GuiTreeItem.setTitle(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:51)
- [ ] `GuiTreeItem.resetCheckboxPartial(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:56)
- [ ] `GuiTreeItem.getOrCreateCheckbox(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:62)
- [ ] `GuiTreeItem.isMoving(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:72)
- [ ] `GuiTreeItem.setMoving(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:76)
- [ ] `GuiTreeItem.opened(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:80)
- [ ] `GuiTreeItem.toggle(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:84)
- [ ] `GuiTreeItem.isAtLeastPartiallyChecked(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:92)
- [ ] `GuiTreeItem.isChecked(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:96)
- [ ] `GuiTreeItem.setChecked(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:100)
- [ ] `GuiTreeItem.childCheckedChanged(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:113)
- [ ] `GuiTreeItem.updateControls(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:145)
- [ ] `GuiTreeItem.getParentItem(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:166)
- [ ] `GuiTreeItem.clearItems(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:170)
- [ ] `GuiTreeItem.removeItem(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:180)
- [ ] `GuiTreeItem.removed(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:191)
- [ ] `GuiTreeItem.insertItemAfter(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:194)
- [ ] `GuiTreeItem.insertItemAfter(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:198)
- [ ] `GuiTreeItem.insertItem(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:206)
- [ ] `GuiTreeItem.insertItem(2)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:210)
- [ ] `GuiTreeItem.addItem(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:218)
- [ ] `GuiTreeItem.added(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:226)
- [ ] `GuiTreeItem.updateLevel(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:229)
- [ ] `GuiTreeItem.getLevel(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:237)
- [ ] `GuiTreeItem.items(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:241)
- [ ] `GuiTreeItem.itemsChecked(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:245)
- [ ] `GuiTreeItem.getItem(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:249)
- [ ] `GuiTreeItem.indexOf(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:253)
- [ ] `GuiTreeItem.itemsCount(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:257)
- [ ] `GuiTreeItem.isChild(1)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:261)
- [ ] `GuiTreeItem.selected(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:271)
- [ ] `GuiTreeItem.select(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:275)
- [ ] `GuiTreeItem.deselect(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:280)
- [ ] `GuiTreeItem.updateColor(0)`: its class isn't translated (it extends `GuiParent` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/control/tree/GuiTreeItem.java:285)

### `BlockGuiCreator`: it's an interface

- [ ] `BlockGuiCreator.create(5)`: it has no body (team/creative/creativecore/common/gui/creator/BlockGuiCreator.java:11)

### `GuiCreator`: it uses Minecraft's `CompoundTag`

- [ ] `GuiCreator.REGISTRY`: it uses NamedHandlerRegistry, which isn't translated (it's generic) (team/creative/creativecore/common/gui/creator/GuiCreator.java:18)
- [ ] `GuiCreator.ITEM_OPENER`: it uses GuiCreatorItem, which isn't translated (it extends `GuiCreator`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:19)
- [ ] `GuiCreator.BLOCK_OPENER`: it uses GuiCreatorBlock, which isn't translated (it extends `GuiCreator`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:20)
- [ ] `GuiCreator.register(2)`: it's generic (team/creative/creativecore/common/gui/creator/GuiCreator.java:24)
- [ ] `GuiCreator.GuiCreator(1)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:30)
- [ ] `GuiCreator.getName(0)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:34)
- [ ] `GuiCreator.openGui(2)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:38)

### `GuiCreator.GuiCreatorBasic`: it extends `GuiCreator`

- [ ] `GuiCreator.GuiCreatorBasic.GuiCreatorBasic(1)`: its class isn't translated (it extends `GuiCreator`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:47)
- [ ] `GuiCreator.GuiCreatorBasic.open(1)`: its class isn't translated (it extends `GuiCreator`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:51)
- [ ] `GuiCreator.GuiCreatorBasic.open(2)`: its class isn't translated (it extends `GuiCreator`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:55)

### `GuiCreator.GuiCreatorBlock`: it extends `GuiCreator`

- [ ] `GuiCreator.GuiCreatorBlock.GuiCreatorBlock(0)`: its class isn't translated (it extends `GuiCreator`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:61)
- [ ] `GuiCreator.GuiCreatorBlock.open(2)`: its class isn't translated (it extends `GuiCreator`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:70)
- [ ] `GuiCreator.GuiCreatorBlock.open(3)`: its class isn't translated (it extends `GuiCreator`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:74)

### `GuiCreator.GuiCreatorItem`: it extends `GuiCreator`

- [ ] `GuiCreator.GuiCreatorItem.GuiCreatorItem(0)`: its class isn't translated (it extends `GuiCreator`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:81)
- [ ] `GuiCreator.GuiCreatorItem.open(2)`: its class isn't translated (it extends `GuiCreator`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:89)
- [ ] `GuiCreator.GuiCreatorItem.open(3)`: its class isn't translated (it extends `GuiCreator`) (team/creative/creativecore/common/gui/creator/GuiCreator.java:93)

### `ItemGuiCreator`: it's an interface

- [ ] `ItemGuiCreator.create(2)`: it has no body (team/creative/creativecore/common/gui/creator/ItemGuiCreator.java:8)

### `DialogGuiLayer`: it extends `GuiLayer` (team.creative.creativecore.common.gui)

- [ ] `DialogGuiLayer.DialogGuiLayer(5)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/dialog/DialogGuiLayer.java:18)
- [ ] `DialogGuiLayer.create(0)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/dialog/DialogGuiLayer.java:30)
- [ ] `DialogGuiLayer.closeDialog(1)`: its class isn't translated (it extends `GuiLayer` (team.creative.creativecore.common.gui)) (team/creative/creativecore/common/gui/dialog/DialogGuiLayer.java:42)

### `DialogGuiLayer.DialogButton`


### `GuiDialogHandler`

- [ ] `GuiDialogHandler.DIALOG_HANDLER`: it uses GuiSyncGlobalLayer, which isn't translated (it's generic) (team/creative/creativecore/common/gui/dialog/GuiDialogHandler.java:13)
- [x] `GuiDialogHandler.init(0)` (team/creative/creativecore/common/gui/dialog/GuiDialogHandler.java:24)
- [ ] `GuiDialogHandler.openDialog(4)`: it uses GuiLayer, which isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/dialog/GuiDialogHandler.java:27)
- [ ] `GuiDialogHandler.openDialog(5)`: it uses GuiLayer, which isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/dialog/GuiDialogHandler.java:33)

### `GuiControlChangedEvent`: it's generic

- [ ] `GuiControlChangedEvent.GuiControlChangedEvent(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/event/GuiControlChangedEvent.java:6)
- [ ] `GuiControlChangedEvent.cancelable(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/event/GuiControlChangedEvent.java:10)

### `GuiControlClickEvent`: it extends `GuiControlEvent`

- [ ] `GuiControlClickEvent.GuiControlClickEvent(3)`: its class isn't translated (it extends `GuiControlEvent`) (team/creative/creativecore/common/gui/event/GuiControlClickEvent.java:9)
- [ ] `GuiControlClickEvent.cancelable(0)`: its class isn't translated (it extends `GuiControlEvent`) (team/creative/creativecore/common/gui/event/GuiControlClickEvent.java:15)

### `GuiControlEvent`: it's generic

- [ ] `GuiControlEvent.GuiControlEvent(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/event/GuiControlEvent.java:8)

### `GuiEvent`

- [x] `GuiEvent.isCanceled(0)` (team/creative/creativecore/common/gui/event/GuiEvent.java:6)
- [ ] `GuiEvent.cancel(0)`: it calls GuiEvent.getClass, which isn't in the mod's source (team/creative/creativecore/common/gui/event/GuiEvent.java:10)
- [ ] `GuiEvent.cancelable(0)`: it has no body (team/creative/creativecore/common/gui/event/GuiEvent.java:18)

### `GuiEventManager`: it uses GuiEventHandler, which isn't translated (it's generic)

- [ ] `GuiEventManager.raiseEvent(1)`: its class isn't translated (it uses GuiEventHandler, which isn't translated (it's generic)) (team/creative/creativecore/common/gui/event/GuiEventManager.java:10)
- [ ] `GuiEventManager.registerEvent(2)`: it's generic (team/creative/creativecore/common/gui/event/GuiEventManager.java:19)
- [ ] `GuiEventManager.clear(0)`: its class isn't translated (it uses GuiEventHandler, which isn't translated (it's generic)) (team/creative/creativecore/common/gui/event/GuiEventManager.java:23)

### `GuiEventManager.GuiEventHandler`: it's generic

- [ ] `GuiEventManager.GuiEventHandler.GuiEventHandler(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/event/GuiEventManager.java:31)
- [ ] `GuiEventManager.GuiEventHandler.react(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/event/GuiEventManager.java:36)

### `GuiSliderUpdateEvent`: it extends `GuiControlChangedEvent`

- [ ] `GuiSliderUpdateEvent.GuiSliderUpdateEvent(1)`: its class isn't translated (it extends `GuiControlChangedEvent`) (team/creative/creativecore/common/gui/event/GuiSliderUpdateEvent.java:6)

### `GuiTextUpdateEvent`: it extends `GuiControlChangedEvent`

- [ ] `GuiTextUpdateEvent.GuiTextUpdateEvent(1)`: its class isn't translated (it extends `GuiControlChangedEvent`) (team/creative/creativecore/common/gui/event/GuiTextUpdateEvent.java:6)

### `GuiTooltipEvent`: it extends `GuiControlEvent`

- [ ] `GuiTooltipEvent.GuiTooltipEvent(3)`: its class isn't translated (it extends `GuiControlEvent`) (team/creative/creativecore/common/gui/event/GuiTooltipEvent.java:11)
- [ ] `GuiTooltipEvent.cancelable(0)`: its class isn't translated (it extends `GuiControlEvent`) (team/creative/creativecore/common/gui/event/GuiTooltipEvent.java:17)

### `GuiFlow`

- [ ] `GuiFlow.STACK_X`: it uses GuiStackX, which isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiFlow.java:9)
- [ ] `GuiFlow.FIT_X`: it uses GuiFlowFitX, which isn't translated (it extends `GuiStackX`) (team/creative/creativecore/common/gui/flow/GuiFlow.java:10)
- [ ] `GuiFlow.STACK_Y`: it uses GuiStackY, which isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiFlow.java:11)
- [ ] `GuiFlow.areChildrenExpandableX(1)`: it uses GuiControlRect, which isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/common/gui/flow/GuiFlow.java:13)
- [ ] `GuiFlow.areChildrenExpandableY(1)`: it uses GuiControlRect, which isn't translated (it uses GuiClientControl, which isn't translated (it's generic)) (team/creative/creativecore/common/gui/flow/GuiFlow.java:23)
- [ ] `GuiFlow.minWidth(3)`: it has no body (team/creative/creativecore/common/gui/flow/GuiFlow.java:33)
- [ ] `GuiFlow.preferredWidth(3)`: it has no body (team/creative/creativecore/common/gui/flow/GuiFlow.java:35)
- [ ] `GuiFlow.minHeight(4)`: it has no body (team/creative/creativecore/common/gui/flow/GuiFlow.java:37)
- [ ] `GuiFlow.preferredHeight(4)`: it has no body (team/creative/creativecore/common/gui/flow/GuiFlow.java:39)
- [ ] `GuiFlow.flowX(6)`: it has no body (team/creative/creativecore/common/gui/flow/GuiFlow.java:41)
- [ ] `GuiFlow.flowY(7)`: it has no body (team/creative/creativecore/common/gui/flow/GuiFlow.java:43)

### `GuiFlowFitX`: it extends `GuiStackX`

- [ ] `GuiFlowFitX.minWidth(3)`: its class isn't translated (it extends `GuiStackX`) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:10)
- [ ] `GuiFlowFitX.minHeight(4)`: its class isn't translated (it extends `GuiStackX`) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:28)
- [ ] `GuiFlowFitX.preferredHeight(4)`: its class isn't translated (it extends `GuiStackX`) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:47)
- [ ] `GuiFlowFitX.flowX(6)`: its class isn't translated (it extends `GuiStackX`) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:66)
- [ ] `GuiFlowFitX.flowY(7)`: its class isn't translated (it extends `GuiStackX`) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:97)

### `GuiFlowFitX.GuiRowControl`: it extends `GuiControlRect` (team.creative.creativecore.client.gui)

- [ ] `GuiFlowFitX.GuiRowControl.GuiRowControl(4)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:136)
- [ ] `GuiFlowFitX.GuiRowControl.getContentWidth(0)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:144)
- [ ] `GuiFlowFitX.GuiRowControl.getMinWidth(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:149)
- [ ] `GuiFlowFitX.GuiRowControl.getMaxWidth(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:154)
- [ ] `GuiFlowFitX.GuiRowControl.getPreferredWidth(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:159)
- [ ] `GuiFlowFitX.GuiRowControl.getContentHeight(0)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:164)
- [ ] `GuiFlowFitX.GuiRowControl.getMinHeight(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:169)
- [ ] `GuiFlowFitX.GuiRowControl.getMaxHeight(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:174)
- [ ] `GuiFlowFitX.GuiRowControl.getPreferredHeight(1)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:179)
- [ ] `GuiFlowFitX.GuiRowControl.flowX(0)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:184)
- [ ] `GuiFlowFitX.GuiRowControl.flowY(0)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:188)
- [ ] `GuiFlowFitX.GuiRowControl.isExpandableX(0)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:193)
- [ ] `GuiFlowFitX.GuiRowControl.isExpandableY(0)`: its class isn't translated (it extends `GuiControlRect` (team.creative.creativecore.client.gui)) (team/creative/creativecore/common/gui/flow/GuiFlowFitX.java:198)

### `GuiSizeRule`

- [ ] `GuiSizeRule.minWidth(2)`: it has no body (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:6)
- [ ] `GuiSizeRule.preferredWidth(2)`: it has no body (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:8)
- [ ] `GuiSizeRule.maxWidth(2)`: it has no body (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:10)
- [ ] `GuiSizeRule.minHeight(3)`: it has no body (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:12)
- [ ] `GuiSizeRule.preferredHeight(3)`: it has no body (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:14)
- [ ] `GuiSizeRule.maxHeight(3)`: it has no body (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:16)

### `GuiSizeRule.GuiFixedDimension`: it extends `GuiSizeRule`

- [ ] `GuiSizeRule.GuiFixedDimension.GuiFixedDimension(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:22)
- [ ] `GuiSizeRule.GuiFixedDimension.GuiFixedDimension(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:27)
- [ ] `GuiSizeRule.GuiFixedDimension.minWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:31)
- [ ] `GuiSizeRule.GuiFixedDimension.preferredWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:36)
- [ ] `GuiSizeRule.GuiFixedDimension.maxWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:41)
- [ ] `GuiSizeRule.GuiFixedDimension.minHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:46)
- [ ] `GuiSizeRule.GuiFixedDimension.preferredHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:51)
- [ ] `GuiSizeRule.GuiFixedDimension.maxHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:56)

### `GuiSizeRule.GuiRatioDimension`: it extends `GuiSizeRule`

- [ ] `GuiSizeRule.GuiRatioDimension.GuiRatioDimension(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:66)
- [ ] `GuiSizeRule.GuiRatioDimension.minWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:71)
- [ ] `GuiSizeRule.GuiRatioDimension.preferredWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:76)
- [ ] `GuiSizeRule.GuiRatioDimension.maxWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:81)
- [ ] `GuiSizeRule.GuiRatioDimension.minHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:86)
- [ ] `GuiSizeRule.GuiRatioDimension.preferredHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:91)
- [ ] `GuiSizeRule.GuiRatioDimension.maxHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:96)

### `GuiSizeRule.GuiSizeRatioRules`: it extends `GuiSizeRule`

- [ ] `GuiSizeRule.GuiSizeRatioRules.GuiSizeRatioRules(0)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:110)
- [ ] `GuiSizeRule.GuiSizeRatioRules.GuiSizeRatioRules(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:113)
- [ ] `GuiSizeRule.GuiSizeRatioRules.minWidth(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:118)
- [ ] `GuiSizeRule.GuiSizeRatioRules.widthRatio(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:123)
- [ ] `GuiSizeRule.GuiSizeRatioRules.maxWidth(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:128)
- [ ] `GuiSizeRule.GuiSizeRatioRules.minHeight(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:133)
- [ ] `GuiSizeRule.GuiSizeRatioRules.heightRatio(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:138)
- [ ] `GuiSizeRule.GuiSizeRatioRules.maxHeight(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:143)
- [ ] `GuiSizeRule.GuiSizeRatioRules.minWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:148)
- [ ] `GuiSizeRule.GuiSizeRatioRules.preferredWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:153)
- [ ] `GuiSizeRule.GuiSizeRatioRules.maxWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:158)
- [ ] `GuiSizeRule.GuiSizeRatioRules.minHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:163)
- [ ] `GuiSizeRule.GuiSizeRatioRules.preferredHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:168)
- [ ] `GuiSizeRule.GuiSizeRatioRules.maxHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:173)

### `GuiSizeRule.GuiSizeRules`: it extends `GuiSizeRule`

- [ ] `GuiSizeRule.GuiSizeRules.minWidth(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:187)
- [ ] `GuiSizeRule.GuiSizeRules.prefWidth(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:192)
- [ ] `GuiSizeRule.GuiSizeRules.maxWidth(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:197)
- [ ] `GuiSizeRule.GuiSizeRules.minHeight(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:202)
- [ ] `GuiSizeRule.GuiSizeRules.prefHeight(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:207)
- [ ] `GuiSizeRule.GuiSizeRules.maxHeight(1)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:212)
- [ ] `GuiSizeRule.GuiSizeRules.minWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:217)
- [ ] `GuiSizeRule.GuiSizeRules.preferredWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:222)
- [ ] `GuiSizeRule.GuiSizeRules.maxWidth(2)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:227)
- [ ] `GuiSizeRule.GuiSizeRules.minHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:232)
- [ ] `GuiSizeRule.GuiSizeRules.preferredHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:237)
- [ ] `GuiSizeRule.GuiSizeRules.maxHeight(3)`: its class isn't translated (it extends `GuiSizeRule`) (team/creative/creativecore/common/gui/flow/GuiSizeRule.java:242)

### `GuiStackX`: it extends `GuiFlow`

- [ ] `GuiStackX.minWidth(3)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackX.java:12)
- [ ] `GuiStackX.preferredWidth(3)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackX.java:30)
- [ ] `GuiStackX.minHeight(4)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackX.java:41)
- [ ] `GuiStackX.preferredHeight(4)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackX.java:52)
- [ ] `GuiStackX.flowX(6)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackX.java:63)
- [ ] `GuiStackX.flowY(7)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackX.java:217)

### `GuiStackY`: it extends `GuiFlow`

- [ ] `GuiStackY.minWidth(3)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackY.java:12)
- [ ] `GuiStackY.preferredWidth(3)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackY.java:23)
- [ ] `GuiStackY.minHeight(4)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackY.java:34)
- [ ] `GuiStackY.preferredHeight(4)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackY.java:52)
- [ ] `GuiStackY.flowX(6)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackY.java:63)
- [ ] `GuiStackY.flowY(7)`: its class isn't translated (it extends `GuiFlow`) (team/creative/creativecore/common/gui/flow/GuiStackY.java:105)

### `GuiControl`: it uses IGuiParent, which isn't translated (it's an interface)

- [ ] `GuiControl.setParent(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/gui/GuiControl.java:23)
- [ ] `GuiControl.setParent(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/gui/GuiControl.java:27)
- [ ] `GuiControl.GuiControl(2)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:31)
- [ ] `GuiControl.dist(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:37)
- [ ] `GuiControl.setVisible(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:42)
- [ ] `GuiControl.setFixed(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:50)
- [ ] `GuiControl.setFixedX(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:58)
- [ ] `GuiControl.setFixedY(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:66)
- [ ] `GuiControl.setExpandable(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:74)
- [ ] `GuiControl.setExpandableX(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:82)
- [ ] `GuiControl.setExpandableY(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:90)
- [ ] `GuiControl.isExpandableX(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:98)
- [ ] `GuiControl.isExpandableY(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:102)
- [ ] `GuiControl.setDim(2)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:106)
- [ ] `GuiControl.setDim(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:114)
- [ ] `GuiControl.setEnabled(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:122)
- [ ] `GuiControl.removeFormatting(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:130)
- [ ] `GuiControl.setFormatting(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:138)
- [ ] `GuiControl.setCustomFormatting(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:146)
- [ ] `GuiControl.reflow(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:150)
- [ ] `GuiControl.provider(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:156)
- [ ] `GuiControl.isClient(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:161)
- [ ] `GuiControl.setTooltip(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:165)
- [ ] `GuiControl.setTooltip(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:173)
- [ ] `GuiControl.hasGui(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:181)
- [ ] `GuiControl.isParent(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:185)
- [ ] `GuiControl.getParent(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:189)
- [ ] `GuiControl.getIntegratedParent(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:193)
- [ ] `GuiControl.getNestedName(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:197)
- [ ] `GuiControl.hasLayer(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:201)
- [ ] `GuiControl.getLayer(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:205)
- [ ] `GuiControl.init(0)`: it has no body (team/creative/creativecore/common/gui/GuiControl.java:213)
- [ ] `GuiControl.closed(0)`: it has no body (team/creative/creativecore/common/gui/GuiControl.java:215)
- [ ] `GuiControl.tick(0)`: it has no body (team/creative/creativecore/common/gui/GuiControl.java:217)
- [ ] `GuiControl.is(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:219)
- [ ] `GuiControl.is(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:223)
- [ ] `GuiControl.raiseEvent(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:233)
- [ ] `GuiControl.getPlayer(0)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:239)
- [ ] `GuiControl.toLayerRect(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:243)
- [ ] `GuiControl.toScreenRect(1)`: its class isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiControl.java:247)
- [ ] `GuiControl.translatable(1)`: it calls GuiControl.translate, which isn't translated (team/creative/creativecore/common/gui/GuiControl.java:251)
- [ ] `GuiControl.translatable(2)`: it uses `Object` (team/creative/creativecore/common/gui/GuiControl.java:255)
- [ ] `GuiControl.translate(1)`: it calls LanguageUtils.translate, which isn't translated (team/creative/creativecore/common/gui/GuiControl.java:259)
- [ ] `GuiControl.translate(2)`: it uses `Object` (team/creative/creativecore/common/gui/GuiControl.java:263)
- [ ] `GuiControl.translateOrDefault(2)`: it calls LanguageUtils.translateOr, which isn't translated (team/creative/creativecore/common/gui/GuiControl.java:267)

### `GuiControl.GuiFocusControlDist`: it's an interface

- [ ] `GuiControl.GuiFocusControlDist.isFocused(0)`: it has no body (team/creative/creativecore/common/gui/GuiControl.java:272)
- [ ] `GuiControl.GuiFocusControlDist.focus(0)`: it has no body (team/creative/creativecore/common/gui/GuiControl.java:274)

### `GuiControlDistHandler`: it's an interface

- [ ] `GuiControlDistHandler.setVisible(1)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:12)
- [ ] `GuiControlDistHandler.setFixed(0)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:14)
- [ ] `GuiControlDistHandler.setFixedX(0)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:16)
- [ ] `GuiControlDistHandler.setFixedY(0)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:18)
- [ ] `GuiControlDistHandler.setExpandable(0)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:20)
- [ ] `GuiControlDistHandler.setExpandableX(0)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:22)
- [ ] `GuiControlDistHandler.setExpandableY(0)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:24)
- [ ] `GuiControlDistHandler.isExpandableX(0)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:26)
- [ ] `GuiControlDistHandler.isExpandableY(0)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:28)
- [ ] `GuiControlDistHandler.setDim(2)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:30)
- [ ] `GuiControlDistHandler.setDim(1)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:32)
- [ ] `GuiControlDistHandler.setEnabled(1)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:34)
- [ ] `GuiControlDistHandler.setTooltip(1)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:36)
- [ ] `GuiControlDistHandler.setTooltip(1)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:38)
- [ ] `GuiControlDistHandler.removeFormatting(0)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:40)
- [ ] `GuiControlDistHandler.setFormatting(1)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:42)
- [ ] `GuiControlDistHandler.setCustomFormatting(0)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:44)
- [ ] `GuiControlDistHandler.playSound(1)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:46)
- [ ] `GuiControlDistHandler.playSound(1)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:48)
- [ ] `GuiControlDistHandler.playSound(3)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:50)
- [ ] `GuiControlDistHandler.playSound(3)`: it has no body (team/creative/creativecore/common/gui/GuiControlDistHandler.java:52)

### `GuiControls`: it implements `Iterable`

- [ ] `GuiControls.get(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:17)
- [ ] `GuiControls.remove(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:21)
- [ ] `GuiControls.get(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:26)
- [ ] `GuiControls.get(2)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:31)
- [ ] `GuiControls.insertControlBefore(2)`: it's generic (team/creative/creativecore/common/gui/GuiControls.java:53)
- [ ] `GuiControls.insertControlAfter(2)`: it's generic (team/creative/creativecore/common/gui/GuiControls.java:64)
- [ ] `GuiControls.add(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:81)
- [ ] `GuiControls.addHover(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:86)
- [ ] `GuiControls.contains(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:91)
- [ ] `GuiControls.remove(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:95)
- [ ] `GuiControls.replace(2)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:113)
- [ ] `GuiControls.remove(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:133)
- [ ] `GuiControls.removeExclude(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:157)
- [ ] `GuiControls.isEmpty(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:181)
- [ ] `GuiControls.clear(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:185)
- [ ] `GuiControls.controlSize(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:192)
- [ ] `GuiControls.hoverSize(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:196)
- [ ] `GuiControls.totalSize(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:200)
- [ ] `GuiControls.iterator(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:204)
- [ ] `GuiControls.distControls(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:209)
- [ ] `GuiControls.distHoverControls(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:213)
- [ ] `GuiControls.distAll(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:217)
- [ ] `GuiControls.streamDistControls(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:221)
- [ ] `GuiControls.streamDistHoverControls(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:225)
- [ ] `GuiControls.distListControls(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:229)
- [ ] `GuiControls.distHoverListControls(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/gui/GuiControls.java:233)

### `GuiLayer`: it extends `GuiParent`

- [ ] `GuiLayer.CLIENT`: its value couldn't be read: an anonymous class (team/creative/creativecore/common/gui/GuiLayer.java:21)
- [ ] `GuiLayer.SERVER`: its value couldn't be read: an anonymous class (team/creative/creativecore/common/gui/GuiLayer.java:37)
- [ ] `GuiLayer.collectInventories(2)`: it uses GuiControl, which isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/common/gui/GuiLayer.java:56)
- [ ] `GuiLayer.GuiLayer(2)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:66)
- [ ] `GuiLayer.GuiLayer(4)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:70)
- [ ] `GuiLayer.dist(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:75)
- [ ] `GuiLayer.setParent(1)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:79)
- [ ] `GuiLayer.inventoriesToInsert(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:83)
- [ ] `GuiLayer.inventoriesToExtract(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:89)
- [ ] `GuiLayer.has(1)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:96)
- [ ] `GuiLayer.getOrCreate(1)`: it's generic (team/creative/creativecore/common/gui/GuiLayer.java:100)
- [ ] `GuiLayer.itemManager(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:113)
- [ ] `GuiLayer.managers(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:117)
- [ ] `GuiLayer.getSyncHolder(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:121)
- [ ] `GuiLayer.init(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:125)
- [ ] `GuiLayer.reinit(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:132)
- [ ] `GuiLayer.hasMinimumOuterSpacing(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:137)
- [ ] `GuiLayer.create(0)`: it has no body (team/creative/creativecore/common/gui/GuiLayer.java:141)
- [ ] `GuiLayer.becameTopLayer(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:143)
- [ ] `GuiLayer.getLayer(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:146)
- [ ] `GuiLayer.closeLayerUsingEscape(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:151)
- [ ] `GuiLayer.hasLayer(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:155)
- [ ] `GuiLayer.tick(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:160)
- [ ] `GuiLayer.closed(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:169)
- [ ] `GuiLayer.reflow(0)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:178)
- [ ] `GuiLayer.toLayerRect(2)`: its class isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:185)

### `GuiLayer.GuiLayerDistHandler`: it's an interface

- [ ] `GuiLayer.GuiLayerDistHandler.reflow(0)`: it has no body (team/creative/creativecore/common/gui/GuiLayer.java:195)

### `GuiLayer.GuiLayerDistination`: it implements `IGuiParent`

- [ ] `GuiLayer.GuiLayerDistination.isContainer(0)`: its class isn't translated (it implements `IGuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:199)
- [ ] `GuiLayer.GuiLayerDistination.getPlayer(0)`: its class isn't translated (it implements `IGuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:204)
- [ ] `GuiLayer.GuiLayerDistination.closeTopLayer(0)`: its class isn't translated (it implements `IGuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:209)
- [ ] `GuiLayer.GuiLayerDistination.closeLayer(1)`: its class isn't translated (it implements `IGuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:213)
- [ ] `GuiLayer.GuiLayerDistination.raiseEvent(1)`: its class isn't translated (it implements `IGuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:217)
- [ ] `GuiLayer.GuiLayerDistination.reflow(0)`: its class isn't translated (it implements `IGuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:221)
- [ ] `GuiLayer.GuiLayerDistination.hasGui(0)`: its class isn't translated (it implements `IGuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:225)
- [ ] `GuiLayer.GuiLayerDistination.isParent(1)`: its class isn't translated (it implements `IGuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:230)
- [ ] `GuiLayer.GuiLayerDistination.toScreenRect(2)`: its class isn't translated (it implements `IGuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:235)
- [ ] `GuiLayer.GuiLayerDistination.toLayerRect(2)`: its class isn't translated (it implements `IGuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:240)
- [ ] `GuiLayer.GuiLayerDistination.getIntegratedParent(0)`: its class isn't translated (it implements `IGuiParent`) (team/creative/creativecore/common/gui/GuiLayer.java:245)

### `GuiParent`: it extends `GuiControl`

- [ ] `GuiParent.GuiParent(3)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:20)
- [ ] `GuiParent.GuiParent(4)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:28)
- [ ] `GuiParent.GuiParent(4)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:32)
- [ ] `GuiParent.GuiParent(5)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:36)
- [ ] `GuiParent.GuiParent(2)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:44)
- [ ] `GuiParent.GuiParent(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:48)
- [ ] `GuiParent.GuiParent(2)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:52)
- [ ] `GuiParent.dist(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:56)
- [ ] `GuiParent.isClient(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:61)
- [ ] `GuiParent.setScale(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:66)
- [ ] `GuiParent.setAlign(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:74)
- [ ] `GuiParent.setVAlign(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:82)
- [ ] `GuiParent.setSpacing(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:90)
- [ ] `GuiParent.setFlow(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:98)
- [ ] `GuiParent.isExpandableX(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:106)
- [ ] `GuiParent.isExpandableY(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:121)
- [ ] `GuiParent.get(1)`: it's generic (team/creative/creativecore/common/gui/GuiParent.java:136)
- [ ] `GuiParent.get(2)`: it's generic (team/creative/creativecore/common/gui/GuiParent.java:140)
- [ ] `GuiParent.has(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:145)
- [ ] `GuiParent.insertControlBefore(2)`: it's generic (team/creative/creativecore/common/gui/GuiParent.java:149)
- [ ] `GuiParent.insertControlAfter(2)`: it's generic (team/creative/creativecore/common/gui/GuiParent.java:153)
- [ ] `GuiParent.contains(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:157)
- [ ] `GuiParent.add(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:161)
- [ ] `GuiParent.add(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:167)
- [ ] `GuiParent.add(2)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:175)
- [ ] `GuiParent.addHover(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:179)
- [ ] `GuiParent.addHover(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:185)
- [ ] `GuiParent.addHover(2)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:193)
- [ ] `GuiParent.remove(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:197)
- [ ] `GuiParent.remove(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:201)
- [ ] `GuiParent.replace(2)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:205)
- [ ] `GuiParent.remove(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:209)
- [ ] `GuiParent.removeExclude(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:213)
- [ ] `GuiParent.isEmpty(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:217)
- [ ] `GuiParent.clear(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:221)
- [ ] `GuiParent.get(1)`: it's generic (team/creative/creativecore/common/gui/GuiParent.java:225)
- [ ] `GuiParent.totalSize(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:229)
- [ ] `GuiParent.size(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:233)
- [ ] `GuiParent.iterator(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:237)
- [ ] `GuiParent.isContainer(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:242)
- [ ] `GuiParent.init(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:247)
- [ ] `GuiParent.closed(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:254)
- [ ] `GuiParent.tick(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:261)
- [ ] `GuiParent.closeTopLayer(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:268)
- [ ] `GuiParent.closeThisLayer(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:273)
- [ ] `GuiParent.closeLayer(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:277)
- [ ] `GuiParent.raiseEvent(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:282)
- [ ] `GuiParent.registerEventClick(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:295)
- [ ] `GuiParent.registerEventChanged(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:299)
- [ ] `GuiParent.registerEvent(2)`: it's generic (team/creative/creativecore/common/gui/GuiParent.java:303)
- [ ] `GuiParent.clearEvents(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:311)
- [ ] `GuiParent.getNestedName(0)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:317)
- [ ] `GuiParent.toLayerRect(2)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:326)
- [ ] `GuiParent.toScreenRect(2)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:335)
- [ ] `GuiParent.createDist(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:344)
- [ ] `GuiParent.createDist(1)`: its class isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/gui/GuiParent.java:349)

### `GuiParent.GuiParentDistHandler`: it's an interface

- [ ] `GuiParent.GuiParentDistHandler.setScale(1)`: it has no body (team/creative/creativecore/common/gui/GuiParent.java:355)
- [ ] `GuiParent.GuiParentDistHandler.setAlign(1)`: it has no body (team/creative/creativecore/common/gui/GuiParent.java:357)
- [ ] `GuiParent.GuiParentDistHandler.setVAlign(1)`: it has no body (team/creative/creativecore/common/gui/GuiParent.java:359)
- [ ] `GuiParent.GuiParentDistHandler.setSpacing(1)`: it has no body (team/creative/creativecore/common/gui/GuiParent.java:361)
- [ ] `GuiParent.GuiParentDistHandler.setFlow(1)`: it has no body (team/creative/creativecore/common/gui/GuiParent.java:363)
- [ ] `GuiParent.GuiParentDistHandler.initControlList(1)`: it has no body (team/creative/creativecore/common/gui/GuiParent.java:365)
- [ ] `GuiParent.GuiParentDistHandler.applyOffset(2)`: it has no body (team/creative/creativecore/common/gui/GuiParent.java:367)
- [ ] `GuiParent.GuiParentDistHandler.controls(0)`: it has no body (team/creative/creativecore/common/gui/GuiParent.java:369)
- [ ] `GuiParent.GuiParentDistHandler.hoverControls(0)`: it has no body (team/creative/creativecore/common/gui/GuiParent.java:371)
- [ ] `GuiParent.GuiParentDistHandler.all(0)`: it has no body (team/creative/creativecore/common/gui/GuiParent.java:373)

### `IGuiParent`: it's an interface

- [ ] `IGuiParent.isContainer(0)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:11)
- [ ] `IGuiParent.isClient(0)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:13)
- [ ] `IGuiParent.getPlayer(0)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:15)
- [ ] `IGuiParent.closeTopLayer(0)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:17)
- [ ] `IGuiParent.closeLayer(1)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:19)
- [ ] `IGuiParent.raiseEvent(1)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:21)
- [ ] `IGuiParent.reflow(0)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:23)
- [ ] `IGuiParent.hasGui(0)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:25)
- [ ] `IGuiParent.isParent(1)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:27)
- [ ] `IGuiParent.toScreenRect(2)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:29)
- [ ] `IGuiParent.toLayerRect(2)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:31)
- [ ] `IGuiParent.getIntegratedParent(0)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:33)
- [ ] `IGuiParent.createDist(1)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:35)
- [ ] `IGuiParent.createDist(1)`: it has no body (team/creative/creativecore/common/gui/IGuiParent.java:37)

### `ContainerIntegration`: it extends Minecraft's `AbstractContainerMenu`

- [ ] `ContainerIntegration.EMPTY_CLIENT`: it uses GuiLayer, which isn't translated (it extends `GuiParent`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:23)
- [ ] `ContainerIntegration.EMPTY_SERVER`: its value couldn't be read: an anonymous class (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:24)
- [ ] `ContainerIntegration.ContainerIntegration(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:33)
- [ ] `ContainerIntegration.ContainerIntegration(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:40)
- [ ] `ContainerIntegration.setScreen(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:45)
- [ ] `ContainerIntegration.broadcastChanges(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:49)
- [ ] `ContainerIntegration.tick(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:54)
- [ ] `ContainerIntegration.getLayers(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:60)
- [ ] `ContainerIntegration.getTopLayer(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:65)
- [ ] `ContainerIntegration.isContainer(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:74)
- [ ] `ContainerIntegration.removed(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:79)
- [ ] `ContainerIntegration.stillValid(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:87)
- [ ] `ContainerIntegration.getPlayer(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:91)
- [ ] `ContainerIntegration.isClient(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:96)
- [ ] `ContainerIntegration.openLayer(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:101)
- [ ] `ContainerIntegration.closeTopLayer(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:108)
- [ ] `ContainerIntegration.closeLayer(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:116)
- [ ] `ContainerIntegration.closeLayer(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:124)
- [ ] `ContainerIntegration.send(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:143)
- [ ] `ContainerIntegration.quickMoveStack(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:152)
- [ ] `ContainerIntegration.provider(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:156)
- [ ] `ContainerIntegration.createDist(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:161)
- [ ] `ContainerIntegration.createDist(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:166)
- [ ] `ContainerIntegration.toScreenRect(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (team/creative/creativecore/common/gui/integration/ContainerIntegration.java:171)

### `GuiEventHandler`

- [x] `GuiEventHandler.displayWidth` (team/creative/creativecore/common/gui/integration/GuiEventHandler.java:9)
- [x] `GuiEventHandler.displayHeight` (team/creative/creativecore/common/gui/integration/GuiEventHandler.java:10)
- [x] `GuiEventHandler.defaultScale` (team/creative/creativecore/common/gui/integration/GuiEventHandler.java:11)
- [x] `GuiEventHandler.changed` (team/creative/creativecore/common/gui/integration/GuiEventHandler.java:12)
- [ ] `GuiEventHandler.displayScreen`: it uses Minecraft's `Screen` (team/creative/creativecore/common/gui/integration/GuiEventHandler.java:13)
- [ ] `GuiEventHandler.queueScreen(1)`: it uses Minecraft's `Screen` (team/creative/creativecore/common/gui/integration/GuiEventHandler.java:15)
- [ ] `GuiEventHandler.onTick(1)`: it uses GuiEventHandler.displayScreen, which isn't translated (it uses Minecraft's `Screen`) (team/creative/creativecore/common/gui/integration/GuiEventHandler.java:19)

### `IGuiIntegratedParent`: it's an interface

- [ ] `IGuiIntegratedParent.provider(0)`: it has no body (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:13)
- [ ] `IGuiIntegratedParent.getLayers(0)`: it has no body (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:15)
- [ ] `IGuiIntegratedParent.getTopLayer(0)`: it has no body (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:17)
- [ ] `IGuiIntegratedParent.isOpen(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:19)
- [ ] `IGuiIntegratedParent.raiseEvent(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:29)
- [ ] `IGuiIntegratedParent.reflow(0)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:33)
- [ ] `IGuiIntegratedParent.openLayer(1)`: it has no body (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:37)
- [ ] `IGuiIntegratedParent.closeLayer(1)`: it has no body (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:39)
- [ ] `IGuiIntegratedParent.isParent(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:41)
- [ ] `IGuiIntegratedParent.hasGui(0)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:46)
- [ ] `IGuiIntegratedParent.get(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:51)
- [ ] `IGuiIntegratedParent.toLayerRect(2)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:65)
- [ ] `IGuiIntegratedParent.getIntegratedParent(0)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:70)
- [ ] `IGuiIntegratedParent.send(1)`: it has no body (team/creative/creativecore/common/gui/integration/IGuiIntegratedParent.java:75)

### `IScaleableGuiScreen`: it's an interface

- [ ] `IScaleableGuiScreen.getWidth(0)`: it has no body (team/creative/creativecore/common/gui/IScaleableGuiScreen.java:4)
- [ ] `IScaleableGuiScreen.getHeight(0)`: it has no body (team/creative/creativecore/common/gui/IScaleableGuiScreen.java:6)
- [ ] `IScaleableGuiScreen.getMaxScale(2)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/gui/IScaleableGuiScreen.java:8)
- [ ] `IScaleableGuiScreen.clientTick(0)`: it has no body (team/creative/creativecore/common/gui/IScaleableGuiScreen.java:20)

### `GuiManager`: it's generic

- [ ] `GuiManager.REGISTRY`: it uses NamedHandlerRegistry, which isn't translated (it's generic) (team/creative/creativecore/common/gui/manager/GuiManager.java:8)
- [ ] `GuiManager.ITEM`: it uses GuiManagerType, which isn't translated (it's generic) (team/creative/creativecore/common/gui/manager/GuiManager.java:9)
- [ ] `GuiManager.register(3)`: it's generic (team/creative/creativecore/common/gui/manager/GuiManager.java:13)
- [ ] `GuiManager.GuiManager(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/manager/GuiManager.java:19)
- [ ] `GuiManager.tick(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/manager/GuiManager.java:24)
- [ ] `GuiManager.closed(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/manager/GuiManager.java:27)

### `GuiManager.GuiManagerType`: it's generic

- [ ] `GuiManager.GuiManagerType.GuiManagerType(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/manager/GuiManager.java:30)
- [ ] `GuiManager.GuiManagerType.name(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/manager/GuiManager.java:30)
- [ ] `GuiManager.GuiManagerType.managerClass(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/manager/GuiManager.java:30)
- [ ] `GuiManager.GuiManagerType.factory(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/manager/GuiManager.java:30)

### `GuiManagerDist`: it's an interface


### `GuiManagerItem`: it extends `GuiManager`

- [ ] `GuiManagerItem.GuiManagerItem(1)`: its class isn't translated (it extends `GuiManager`) (team/creative/creativecore/common/gui/manager/GuiManagerItem.java:25)
- [ ] `GuiManagerItem.tick(0)`: its class isn't translated (it extends `GuiManager`) (team/creative/creativecore/common/gui/manager/GuiManagerItem.java:29)
- [ ] `GuiManagerItem.closed(0)`: its class isn't translated (it extends `GuiManager`) (team/creative/creativecore/common/gui/manager/GuiManagerItem.java:41)
- [ ] `GuiManagerItem.getHand(0)`: its class isn't translated (it extends `GuiManager`) (team/creative/creativecore/common/gui/manager/GuiManagerItem.java:50)
- [ ] `GuiManagerItem.setHandChanged(0)`: its class isn't translated (it extends `GuiManager`) (team/creative/creativecore/common/gui/manager/GuiManagerItem.java:54)
- [ ] `GuiManagerItem.setHand(1)`: its class isn't translated (it extends `GuiManager`) (team/creative/creativecore/common/gui/manager/GuiManagerItem.java:58)

### `ControlSyncPacket`: it extends `LayerPacket`

- [ ] `ControlSyncPacket.ControlSyncPacket(3)`: its class isn't translated (it extends `LayerPacket`) (team/creative/creativecore/common/gui/packet/ControlSyncPacket.java:15)
- [ ] `ControlSyncPacket.ControlSyncPacket(0)`: its class isn't translated (it extends `LayerPacket`) (team/creative/creativecore/common/gui/packet/ControlSyncPacket.java:21)
- [ ] `ControlSyncPacket.execute(2)`: its class isn't translated (it extends `LayerPacket`) (team/creative/creativecore/common/gui/packet/ControlSyncPacket.java:24)

### `ImmediateItemStackPacket`: it extends `CreativePacket` (team.creative.creativecore.common.network)

- [ ] `ImmediateItemStackPacket.ImmediateItemStackPacket(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/gui/packet/ImmediateItemStackPacket.java:13)
- [ ] `ImmediateItemStackPacket.ImmediateItemStackPacket(2)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/gui/packet/ImmediateItemStackPacket.java:17)
- [ ] `ImmediateItemStackPacket.ImmediateItemStackPacket(0)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/gui/packet/ImmediateItemStackPacket.java:22)
- [ ] `ImmediateItemStackPacket.executeClient(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/gui/packet/ImmediateItemStackPacket.java:25)
- [ ] `ImmediateItemStackPacket.executeServer(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/gui/packet/ImmediateItemStackPacket.java:30)

### `LayerClosePacket`: it extends `LayerPacket`

- [ ] `LayerClosePacket.LayerClosePacket(1)`: its class isn't translated (it extends `LayerPacket`) (team/creative/creativecore/common/gui/packet/LayerClosePacket.java:9)
- [ ] `LayerClosePacket.LayerClosePacket(0)`: its class isn't translated (it extends `LayerPacket`) (team/creative/creativecore/common/gui/packet/LayerClosePacket.java:13)
- [ ] `LayerClosePacket.execute(2)`: its class isn't translated (it extends `LayerPacket`) (team/creative/creativecore/common/gui/packet/LayerClosePacket.java:16)

### `LayerPacket`: it extends `CreativePacket` (team.creative.creativecore.common.network)

- [ ] `LayerPacket.executeClient(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/gui/packet/LayerPacket.java:10)
- [ ] `LayerPacket.executeServer(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/gui/packet/LayerPacket.java:19)
- [ ] `LayerPacket.execute(2)`: it has no body (team/creative/creativecore/common/gui/packet/LayerPacket.java:26)

### `OpenGuiPacket`: it extends `CreativePacket` (team.creative.creativecore.common.network)

- [ ] `OpenGuiPacket.OpenGuiPacket(0)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/gui/packet/OpenGuiPacket.java:19)
- [ ] `OpenGuiPacket.OpenGuiPacket(2)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/gui/packet/OpenGuiPacket.java:22)
- [ ] `OpenGuiPacket.executeClient(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/gui/packet/OpenGuiPacket.java:27)
- [ ] `OpenGuiPacket.executeServer(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/creativecore/common/gui/packet/OpenGuiPacket.java:34)
- [ ] `OpenGuiPacket.openGuiOnServer(3)`: it uses GuiCreator, which isn't translated (it uses Minecraft's `CompoundTag`) (team/creative/creativecore/common/gui/packet/OpenGuiPacket.java:39)

### `SyncPacket`: it extends `LayerPacket`

- [ ] `SyncPacket.SyncPacket(2)`: its class isn't translated (it extends `LayerPacket`) (team/creative/creativecore/common/gui/packet/SyncPacket.java:14)
- [ ] `SyncPacket.SyncPacket(0)`: its class isn't translated (it extends `LayerPacket`) (team/creative/creativecore/common/gui/packet/SyncPacket.java:19)
- [ ] `SyncPacket.execute(2)`: its class isn't translated (it extends `LayerPacket`) (team/creative/creativecore/common/gui/packet/SyncPacket.java:22)

### `DoubleValueParser`: it's an interface

- [ ] `DoubleValueParser.parse(2)`: it has no body (team/creative/creativecore/common/gui/parser/DoubleValueParser.java:13)

### `IntValueParser`: it's an interface

- [ ] `IntValueParser.parse(2)`: it has no body (team/creative/creativecore/common/gui/parser/IntValueParser.java:11)

### `LongValueParser`: it's an interface

- [ ] `LongValueParser.parse(2)`: it has no body (team/creative/creativecore/common/gui/parser/LongValueParser.java:14)

### `ShortValueParser`: it's an interface

- [ ] `ShortValueParser.parse(2)`: it has no body (team/creative/creativecore/common/gui/parser/ShortValueParser.java:5)

### `ControlFormatting`: it's an interface

- [ ] `ControlFormatting.border(0)`: it has no body (team/creative/creativecore/common/gui/style/ControlFormatting.java:53)
- [ ] `ControlFormatting.padding(0)`: it has no body (team/creative/creativecore/common/gui/style/ControlFormatting.java:55)
- [ ] `ControlFormatting.face(0)`: it has no body (team/creative/creativecore/common/gui/style/ControlFormatting.java:57)
- [ ] `ControlFormatting.hasDisabledEffect(0)`: it has no body (team/creative/creativecore/common/gui/style/ControlFormatting.java:59)

### `ControlFormatting.ControlFormattingStatic`: it implements `ControlFormatting`

- [ ] `ControlFormatting.ControlFormattingStatic.ControlFormattingStatic(3)`: its class isn't translated (it implements `ControlFormatting`) (team/creative/creativecore/common/gui/style/ControlFormatting.java:67)
- [ ] `ControlFormatting.ControlFormattingStatic.ControlFormattingStatic(4)`: its class isn't translated (it implements `ControlFormatting`) (team/creative/creativecore/common/gui/style/ControlFormatting.java:74)
- [ ] `ControlFormatting.ControlFormattingStatic.border(0)`: its class isn't translated (it implements `ControlFormatting`) (team/creative/creativecore/common/gui/style/ControlFormatting.java:83)
- [ ] `ControlFormatting.ControlFormattingStatic.padding(0)`: its class isn't translated (it implements `ControlFormatting`) (team/creative/creativecore/common/gui/style/ControlFormatting.java:88)
- [ ] `ControlFormatting.ControlFormattingStatic.face(0)`: its class isn't translated (it implements `ControlFormatting`) (team/creative/creativecore/common/gui/style/ControlFormatting.java:93)
- [ ] `ControlFormatting.ControlFormattingStatic.hasDisabledEffect(0)`: its class isn't translated (it implements `ControlFormatting`) (team/creative/creativecore/common/gui/style/ControlFormatting.java:98)

### `ControlFormatting.ControlStyleBorder`


### `ControlFormatting.ControlStyleFace`


### `ControlFormattingFlexible`: it's an interface

- [ ] `ControlFormattingFlexible.setBorder(1)`: it has no body (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:4)
- [ ] `ControlFormattingFlexible.setPadding(1)`: it has no body (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:6)
- [ ] `ControlFormattingFlexible.setFace(1)`: it has no body (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:8)
- [ ] `ControlFormattingFlexible.setDisabledEffect(1)`: it has no body (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:10)

### `ControlFormattingFlexible.ControlFormattingFlexibleEmpty`: it implements `ControlFormattingFlexible`

- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleEmpty.border(0)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:13)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleEmpty.padding(0)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:18)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleEmpty.face(0)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:23)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleEmpty.hasDisabledEffect(0)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:28)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleEmpty.setBorder(1)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:33)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleEmpty.setPadding(1)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:38)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleEmpty.setFace(1)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:43)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleEmpty.setDisabledEffect(1)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:48)

### `ControlFormattingFlexible.ControlFormattingFlexibleImpl`: it implements `ControlFormattingFlexible`

- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleImpl.ControlFormattingFlexibleImpl(1)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:61)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleImpl.setBorder(1)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:65)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleImpl.setPadding(1)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:71)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleImpl.setFace(1)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:77)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleImpl.setDisabledEffect(1)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:83)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleImpl.border(0)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:89)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleImpl.padding(0)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:94)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleImpl.face(0)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:99)
- [ ] `ControlFormattingFlexible.ControlFormattingFlexibleImpl.hasDisabledEffect(0)`: its class isn't translated (it implements `ControlFormattingFlexible`) (team/creative/creativecore/common/gui/style/ControlFormattingFlexible.java:104)

### `DisplayColor`: it extends `StyleDisplay`

- [ ] `DisplayColor.DisplayColor(0)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayColor.java:10)
- [ ] `DisplayColor.DisplayColor(1)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayColor.java:14)
- [ ] `DisplayColor.DisplayColor(4)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayColor.java:18)
- [ ] `DisplayColor.DisplayColor(1)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayColor.java:22)
- [ ] `DisplayColor.set(1)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayColor.java:26)
- [ ] `DisplayColor.DisplayColor(4)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayColor.java:30)
- [ ] `DisplayColor.set(4)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayColor.java:34)
- [ ] `DisplayColor.set(1)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayColor.java:38)
- [ ] `DisplayColor.render(5)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayColor.java:42)

### `DisplayTexture`: it extends `StyleDisplay`

- [ ] `DisplayTexture.DisplayTexture(0)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayTexture.java:12)
- [ ] `DisplayTexture.DisplayTexture(3)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayTexture.java:16)
- [ ] `DisplayTexture.render(5)`: its class isn't translated (it extends `StyleDisplay`) (team/creative/creativecore/common/gui/style/display/DisplayTexture.java:22)

### `DisplayTextureRepeat`: it extends `DisplayTexture`

- [ ] `DisplayTextureRepeat.DisplayTextureRepeat(0)`: its class isn't translated (it extends `DisplayTexture`) (team/creative/creativecore/common/gui/style/display/DisplayTextureRepeat.java:11)
- [ ] `DisplayTextureRepeat.DisplayTextureRepeat(5)`: its class isn't translated (it extends `DisplayTexture`) (team/creative/creativecore/common/gui/style/display/DisplayTextureRepeat.java:14)
- [ ] `DisplayTextureRepeat.render(5)`: its class isn't translated (it extends `DisplayTexture`) (team/creative/creativecore/common/gui/style/display/DisplayTextureRepeat.java:20)

### `DisplayTextureStretch`: it extends `DisplayTexture`

- [ ] `DisplayTextureStretch.DisplayTextureStretch(0)`: its class isn't translated (it extends `DisplayTexture`) (team/creative/creativecore/common/gui/style/display/DisplayTextureStretch.java:11)
- [ ] `DisplayTextureStretch.DisplayTextureStretch(5)`: its class isn't translated (it extends `DisplayTexture`) (team/creative/creativecore/common/gui/style/display/DisplayTextureStretch.java:14)
- [ ] `DisplayTextureStretch.render(5)`: its class isn't translated (it extends `DisplayTexture`) (team/creative/creativecore/common/gui/style/display/DisplayTextureStretch.java:20)

### `StyleDisplay`

- [ ] `StyleDisplay.NONE`: its value couldn't be read: an anonymous class (team/creative/creativecore/common/gui/style/display/StyleDisplay.java:16)
- [ ] `StyleDisplay.types`: it uses `Class` (team/creative/creativecore/common/gui/style/display/StyleDisplay.java:21)
- [ ] `StyleDisplay.registerType(2)`: it uses `Class` (team/creative/creativecore/common/gui/style/display/StyleDisplay.java:23)
- [ ] `StyleDisplay.render(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/gui/style/display/StyleDisplay.java:31)
- [ ] `StyleDisplay.render(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/gui/style/display/StyleDisplay.java:35)
- [ ] `StyleDisplay.render(2)`: it uses Minecraft's `GuiGraphicsExtractor` (team/creative/creativecore/common/gui/style/display/StyleDisplay.java:39)
- [ ] `StyleDisplay.render(5)`: it has no body (team/creative/creativecore/common/gui/style/display/StyleDisplay.java:43)

### `StyleDisplay.StyleDisplayDeserializer`: it implements `JsonDeserializer` (com.google.gson)

- [ ] `StyleDisplay.StyleDisplayDeserializer.deserialize(3)`: its class isn't translated (it implements `JsonDeserializer` (com.google.gson)) (team/creative/creativecore/common/gui/style/display/StyleDisplay.java:52)

### `GuiStyle`

- [ ] `GuiStyle.mc`: it uses Minecraft's `Minecraft` (team/creative/creativecore/common/gui/style/GuiStyle.java:27)
- [x] `GuiStyle.GSON` (team/creative/creativecore/common/gui/style/GuiStyle.java:28)
- [ ] `GuiStyle.REGISTRY`: it uses NamedHandlerRegistry, which isn't translated (it's generic) (team/creative/creativecore/common/gui/style/GuiStyle.java:29)
- [ ] `GuiStyle.LOADED`: it uses `Object2BooleanMap` (it.unimi.dsi.fastutil.objects) (team/creative/creativecore/common/gui/style/GuiStyle.java:30)
- [ ] `GuiStyle.reload(0)`: it uses GuiStyle.LOADED, which isn't translated (it uses `Object2BooleanMap` (it.unimi.dsi.fastutil.objects)) (team/creative/creativecore/common/gui/style/GuiStyle.java:63)
- [ ] `GuiStyle.getStyle(1)`: it uses GuiStyle.REGISTRY, which isn't translated (it uses NamedHandlerRegistry, which isn't translated (it's generic)) (team/creative/creativecore/common/gui/style/GuiStyle.java:83)
- [ ] `GuiStyle.get(1)`: it uses StyleDisplay.NONE, which isn't translated (its value couldn't be read: an anonymous class) (team/creative/creativecore/common/gui/style/GuiStyle.java:115)
- [ ] `GuiStyle.get(2)`: it uses StyleDisplay.NONE, which isn't translated (its value couldn't be read: an anonymous class) (team/creative/creativecore/common/gui/style/GuiStyle.java:123)
- [x] `GuiStyle.getBorder(1)` (team/creative/creativecore/common/gui/style/GuiStyle.java:137)
- [ ] `GuiStyle.getContentOffset(1)`: it uses ControlFormatting, which isn't translated (it's an interface) (team/creative/creativecore/common/gui/style/GuiStyle.java:145)

### `GuiStyleUtils`

- [x] `GuiStyleUtils.DEFAULT_STYLE_LOCATION` (team/creative/creativecore/common/gui/style/GuiStyleUtils.java:6)
- [x] `GuiStyleUtils.GUI_ASSETS` (team/creative/creativecore/common/gui/style/GuiStyleUtils.java:7)

### `Icon`

- [x] `Icon.EMPTY` (team/creative/creativecore/common/gui/style/Icon.java:6)
- [x] `Icon.ARROW_RIGHT` (team/creative/creativecore/common/gui/style/Icon.java:7)
- [x] `Icon.ARROW_UP` (team/creative/creativecore/common/gui/style/Icon.java:8)
- [x] `Icon.ARROW_LEFT` (team/creative/creativecore/common/gui/style/Icon.java:9)
- [x] `Icon.ARROW_DOWN` (team/creative/creativecore/common/gui/style/Icon.java:10)
- [x] `Icon.MIRROR` (team/creative/creativecore/common/gui/style/Icon.java:11)
- [x] `Icon.ARROW_OUT` (team/creative/creativecore/common/gui/style/Icon.java:12)
- [x] `Icon.ARROW_IN` (team/creative/creativecore/common/gui/style/Icon.java:13)
- [x] `Icon.COORDS` (team/creative/creativecore/common/gui/style/Icon.java:14)
- [x] `Icon.HOUSE` (team/creative/creativecore/common/gui/style/Icon.java:15)
- [x] `Icon.PAUSE` (team/creative/creativecore/common/gui/style/Icon.java:16)
- [x] `Icon.PLAY` (team/creative/creativecore/common/gui/style/Icon.java:17)
- [x] `Icon.STOP` (team/creative/creativecore/common/gui/style/Icon.java:18)
- [x] `Icon.DUPLICATE` (team/creative/creativecore/common/gui/style/Icon.java:19)
- [x] `Icon.MERGE` (team/creative/creativecore/common/gui/style/Icon.java:20)
- [x] `Icon.MOVE` (team/creative/creativecore/common/gui/style/Icon.java:21)
- [x] `Icon.CAMERA` (team/creative/creativecore/common/gui/style/Icon.java:22)
- [x] `Icon.Icon(5)` (team/creative/creativecore/common/gui/style/Icon.java:5)
- [x] `Icon.identifier(0)` (team/creative/creativecore/common/gui/style/Icon.java:5)
- [x] `Icon.minX(0)` (team/creative/creativecore/common/gui/style/Icon.java:5)
- [x] `Icon.minY(0)` (team/creative/creativecore/common/gui/style/Icon.java:5)
- [x] `Icon.width(0)` (team/creative/creativecore/common/gui/style/Icon.java:5)
- [x] `Icon.height(0)` (team/creative/creativecore/common/gui/style/Icon.java:5)

### `GuiSync`: it's generic

- [ ] `GuiSync.GuiSync(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSync.java:10)
- [ ] `GuiSync.receive(2)`: it has no body (team/creative/creativecore/common/gui/sync/GuiSync.java:15)
- [ ] `GuiSync.syncPath(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSync.java:17)

### `GuiSyncControl`: it's generic

- [ ] `GuiSyncControl.GuiSyncControl(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncControl.java:10)
- [ ] `GuiSyncControl.receive(2)`: it has no body (team/creative/creativecore/common/gui/sync/GuiSyncControl.java:15)
- [ ] `GuiSyncControl.syncPath(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncControl.java:17)

### `GuiSyncGlobal`: it's generic

- [ ] `GuiSyncGlobal.GuiSyncGlobal(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncGlobal.java:11)
- [ ] `GuiSyncGlobal.receive(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncGlobal.java:16)
- [ ] `GuiSyncGlobal.send(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncGlobal.java:21)
- [ ] `GuiSyncGlobal.sendAndExecute(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncGlobal.java:25)

### `GuiSyncGlobalLayer`: it's generic

- [ ] `GuiSyncGlobalLayer.GuiSyncGlobalLayer(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncGlobalLayer.java:13)
- [ ] `GuiSyncGlobalLayer.receive(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncGlobalLayer.java:18)
- [ ] `GuiSyncGlobalLayer.open(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncGlobalLayer.java:22)

### `GuiSyncHolder`

- [ ] `GuiSyncHolder.GLOBAL`: it uses GuiSyncHolderGlobal, which isn't translated (it extends `GuiSyncHolder`) (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:16)
- [ ] `GuiSyncHolder.followPath(2)`: it uses GuiSync, which isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:18)
- [ ] `GuiSyncHolder.followPathControl(2)`: it uses GuiSyncControl, which isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:47)
- [ ] `GuiSyncHolder.path(0)`: it has no body (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:76)
- [ ] `GuiSyncHolder.getSync(1)`: it has no body (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:78)
- [ ] `GuiSyncHolder.getControlSync(1)`: it has no body (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:80)

### `GuiSyncHolder.GuiSyncHolderGlobal`: it extends `GuiSyncHolder`

- [ ] `GuiSyncHolder.GuiSyncHolderGlobal.register(2)`: it's generic (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:86)
- [ ] `GuiSyncHolder.GuiSyncHolderGlobal.layer(2)`: it's generic (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:92)
- [ ] `GuiSyncHolder.GuiSyncHolderGlobal.path(0)`: its class isn't translated (it extends `GuiSyncHolder`) (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:98)
- [ ] `GuiSyncHolder.GuiSyncHolderGlobal.getSync(1)`: its class isn't translated (it extends `GuiSyncHolder`) (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:103)
- [ ] `GuiSyncHolder.GuiSyncHolderGlobal.getControlSync(1)`: its class isn't translated (it extends `GuiSyncHolder`) (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:108)

### `GuiSyncHolder.GuiSyncHolderLayer`: it extends `GuiSyncHolder`

- [ ] `GuiSyncHolder.GuiSyncHolderLayer.GuiSyncHolderLayer(1)`: its class isn't translated (it extends `GuiSyncHolder`) (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:118)
- [ ] `GuiSyncHolder.GuiSyncHolderLayer.register(2)`: it's generic (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:122)
- [ ] `GuiSyncHolder.GuiSyncHolderLayer.layer(2)`: it's generic (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:128)
- [ ] `GuiSyncHolder.GuiSyncHolderLayer.path(0)`: its class isn't translated (it extends `GuiSyncHolder`) (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:134)
- [ ] `GuiSyncHolder.GuiSyncHolderLayer.getSync(1)`: its class isn't translated (it extends `GuiSyncHolder`) (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:139)
- [ ] `GuiSyncHolder.GuiSyncHolderLayer.getControlSync(1)`: its class isn't translated (it extends `GuiSyncHolder`) (team/creative/creativecore/common/gui/sync/GuiSyncHolder.java:144)

### `GuiSyncLocal`: it's generic

- [ ] `GuiSyncLocal.GuiSyncLocal(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncLocal.java:12)
- [ ] `GuiSyncLocal.receive(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncLocal.java:17)
- [ ] `GuiSyncLocal.send(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncLocal.java:21)
- [ ] `GuiSyncLocal.sendAndExecute(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncLocal.java:26)

### `GuiSyncLocalLayer`: it's generic

- [ ] `GuiSyncLocalLayer.GuiSyncLocalLayer(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncLocalLayer.java:13)
- [ ] `GuiSyncLocalLayer.receive(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncLocalLayer.java:18)
- [ ] `GuiSyncLocalLayer.open(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/gui/sync/GuiSyncLocalLayer.java:22)

### `VAlign`


### `BlockGetterFake`: it implements Minecraft's `BlockGetter`

- [ ] `BlockGetterFake.set(3)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (team/creative/creativecore/common/level/BlockGetterFake.java:14)
- [ ] `BlockGetterFake.getBlockEntity(1)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (team/creative/creativecore/common/level/BlockGetterFake.java:20)
- [ ] `BlockGetterFake.getBlockState(1)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (team/creative/creativecore/common/level/BlockGetterFake.java:24)
- [ ] `BlockGetterFake.getFluidState(1)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (team/creative/creativecore/common/level/BlockGetterFake.java:28)
- [ ] `BlockGetterFake.getHeight(0)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (team/creative/creativecore/common/level/BlockGetterFake.java:32)
- [ ] `BlockGetterFake.getMinY(0)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (team/creative/creativecore/common/level/BlockGetterFake.java:36)

### `IOrientatedLevel`: it's an interface

- [ ] `IOrientatedLevel.getOrigin(0)`: it has no body (team/creative/creativecore/common/level/IOrientatedLevel.java:9)
- [ ] `IOrientatedLevel.setOrigin(1)`: it has no body (team/creative/creativecore/common/level/IOrientatedLevel.java:11)
- [ ] `IOrientatedLevel.getHolder(0)`: it has no body (team/creative/creativecore/common/level/IOrientatedLevel.java:13)
- [ ] `IOrientatedLevel.setHolder(1)`: it has no body (team/creative/creativecore/common/level/IOrientatedLevel.java:15)
- [ ] `IOrientatedLevel.unload(0)`: it has no body (team/creative/creativecore/common/level/IOrientatedLevel.java:17)

### `ISubLevel`: it's an interface

- [ ] `ISubLevel.getParent(0)`: it has no body (team/creative/creativecore/common/level/ISubLevel.java:7)
- [ ] `ISubLevel.getRealLevel(0)`: it has no body (team/creative/creativecore/common/level/ISubLevel.java:9)
- [ ] `ISubLevel.transformToRealWorld(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/level/ISubLevel.java:11)

### `LevelAccessorFake`: it implements Minecraft's `LevelAccessor`

- [ ] `LevelAccessorFake.set(3)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:46)
- [ ] `LevelAccessorFake.registryAccess(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:52)
- [ ] `LevelAccessorFake.getEntities(3)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:56)
- [ ] `LevelAccessorFake.getEntities(3)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:60)
- [ ] `LevelAccessorFake.players(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:64)
- [ ] `LevelAccessorFake.getChunk(4)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:68)
- [ ] `LevelAccessorFake.getHeight(3)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:72)
- [ ] `LevelAccessorFake.getSkyDarken(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:76)
- [ ] `LevelAccessorFake.getBiomeManager(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:80)
- [ ] `LevelAccessorFake.getUncachedNoiseBiome(3)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:84)
- [ ] `LevelAccessorFake.isClientSide(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:88)
- [ ] `LevelAccessorFake.getSeaLevel(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:92)
- [ ] `LevelAccessorFake.dimensionType(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:96)
- [ ] `LevelAccessorFake.getLightEngine(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:100)
- [ ] `LevelAccessorFake.getBlockEntity(1)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:104)
- [ ] `LevelAccessorFake.getBlockState(1)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:108)
- [ ] `LevelAccessorFake.getFluidState(1)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:112)
- [ ] `LevelAccessorFake.getWorldBorder(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:116)
- [ ] `LevelAccessorFake.isStateAtPosition(2)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:120)
- [ ] `LevelAccessorFake.isFluidAtPosition(2)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:124)
- [ ] `LevelAccessorFake.setBlock(4)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:128)
- [ ] `LevelAccessorFake.removeBlock(2)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:132)
- [ ] `LevelAccessorFake.destroyBlock(4)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:136)
- [ ] `LevelAccessorFake.nextSubTickCount(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:140)
- [ ] `LevelAccessorFake.getBlockTicks(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:144)
- [ ] `LevelAccessorFake.getFluidTicks(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:148)
- [ ] `LevelAccessorFake.getLevelData(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:152)
- [ ] `LevelAccessorFake.getServer(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:156)
- [ ] `LevelAccessorFake.getChunkSource(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:160)
- [ ] `LevelAccessorFake.getRandom(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:164)
- [ ] `LevelAccessorFake.playSound(6)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:168)
- [ ] `LevelAccessorFake.addParticle(7)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:172)
- [ ] `LevelAccessorFake.levelEvent(4)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:176)
- [ ] `LevelAccessorFake.enabledFeatures(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:180)
- [ ] `LevelAccessorFake.gameEvent(3)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:184)
- [ ] `LevelAccessorFake.environmentAttributes(0)`: its class isn't translated (it implements Minecraft's `LevelAccessor`) (team/creative/creativecore/common/level/LevelAccessorFake.java:188)

### `NeighborUpdateCollector`: it uses HashMapList, which isn't translated (it's generic)

- [ ] `NeighborUpdateCollector.NeighborUpdateCollector(2)`: its class isn't translated (it uses HashMapList, which isn't translated (it's generic)) (team/creative/creativecore/common/level/NeighborUpdateCollector.java:17)
- [ ] `NeighborUpdateCollector.NeighborUpdateCollector(0)`: its class isn't translated (it uses HashMapList, which isn't translated (it's generic)) (team/creative/creativecore/common/level/NeighborUpdateCollector.java:21)
- [ ] `NeighborUpdateCollector.add(2)`: its class isn't translated (it uses HashMapList, which isn't translated (it's generic)) (team/creative/creativecore/common/level/NeighborUpdateCollector.java:24)
- [ ] `NeighborUpdateCollector.add(1)`: its class isn't translated (it uses HashMapList, which isn't translated (it's generic)) (team/creative/creativecore/common/level/NeighborUpdateCollector.java:28)
- [ ] `NeighborUpdateCollector.add(2)`: its class isn't translated (it uses HashMapList, which isn't translated (it's generic)) (team/creative/creativecore/common/level/NeighborUpdateCollector.java:32)
- [ ] `NeighborUpdateCollector.processPosition(3)`: its class isn't translated (it uses HashMapList, which isn't translated (it's generic)) (team/creative/creativecore/common/level/NeighborUpdateCollector.java:36)
- [ ] `NeighborUpdateCollector.process(1)`: its class isn't translated (it uses HashMapList, which isn't translated (it's generic)) (team/creative/creativecore/common/level/NeighborUpdateCollector.java:48)
- [ ] `NeighborUpdateCollector.process(0)`: its class isn't translated (it uses HashMapList, which isn't translated (it's generic)) (team/creative/creativecore/common/level/NeighborUpdateCollector.java:56)

### `FabricLoaderUtils`: it implements `ILoaderUtils`

- [ ] `FabricLoaderUtils.getLifeSpan(1)`: its class isn't translated (it implements `ILoaderUtils`) (team/creative/creativecore/common/loader/FabricLoaderUtils.java:8)
- [ ] `FabricLoaderUtils.fireItemPickupPre(2)`: its class isn't translated (it implements `ILoaderUtils`) (team/creative/creativecore/common/loader/FabricLoaderUtils.java:13)
- [ ] `FabricLoaderUtils.fireItemPickupPost(3)`: its class isn't translated (it implements `ILoaderUtils`) (team/creative/creativecore/common/loader/FabricLoaderUtils.java:18)

### `ILoaderUtils`: it's an interface

- [ ] `ILoaderUtils.getLifeSpan(1)`: it has no body (team/creative/creativecore/common/loader/ILoaderUtils.java:8)
- [ ] `ILoaderUtils.fireItemPickupPre(2)`: it has no body (team/creative/creativecore/common/loader/ILoaderUtils.java:10)
- [ ] `ILoaderUtils.fireItemPickupPost(3)`: it has no body (team/creative/creativecore/common/loader/ILoaderUtils.java:12)

### `UniEvent`: it's an interface

- [ ] `UniEvent.cancelable(0)`: it has no body (team/creative/creativecore/common/loader/UniEvent.java:4)
- [ ] `UniEvent.cancel(0)`: it has no body (team/creative/creativecore/common/loader/UniEvent.java:6)

### `BundlePacketWrapper`: it's generic

- [ ] `BundlePacketWrapper.BundlePacketWrapper(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/BundlePacketWrapper.java:9)
- [ ] `BundlePacketWrapper.handle(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/BundlePacketWrapper.java:13)
- [ ] `BundlePacketWrapper.type(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/BundlePacketWrapper.java:19)

### `CanBeNull`: it's an annotation


### `CreativeByteBuf`: it extends Minecraft's `RegistryFriendlyByteBuf`

- [ ] `CreativeByteBuf.CreativeByteBuf(3)`: its class isn't translated (it extends Minecraft's `RegistryFriendlyByteBuf`) (team/creative/creativecore/common/network/CreativeByteBuf.java:11)

### `CreativeNetwork`: it uses `Class`

- [ ] `CreativeNetwork.isSideOnlyPresent(1)`: it uses `Field` (java.lang.reflect) (team/creative/creativecore/common/network/CreativeNetwork.java:35)
- [ ] `CreativeNetwork.CreativeNetwork(3)`: its class isn't translated (it uses `Class`) (team/creative/creativecore/common/network/CreativeNetwork.java:39)
- [ ] `CreativeNetwork.registerType(2)`: it's generic (team/creative/creativecore/common/network/CreativeNetwork.java:46)
- [ ] `CreativeNetwork.getPacketType(1)`: its class isn't translated (it uses `Class`) (team/creative/creativecore/common/network/CreativeNetwork.java:77)
- [ ] `CreativeNetwork.prepare(2)`: it's generic (team/creative/creativecore/common/network/CreativeNetwork.java:81)
- [ ] `CreativeNetwork.sendToServer(1)`: its class isn't translated (it uses `Class`) (team/creative/creativecore/common/network/CreativeNetwork.java:86)
- [ ] `CreativeNetwork.sendToClient(2)`: its class isn't translated (it uses `Class`) (team/creative/creativecore/common/network/CreativeNetwork.java:90)
- [ ] `CreativeNetwork.sendToClient(3)`: its class isn't translated (it uses `Class`) (team/creative/creativecore/common/network/CreativeNetwork.java:94)
- [ ] `CreativeNetwork.sendToClient(2)`: its class isn't translated (it uses `Class`) (team/creative/creativecore/common/network/CreativeNetwork.java:102)
- [ ] `CreativeNetwork.sendToClientTracking(2)`: its class isn't translated (it uses `Class`) (team/creative/creativecore/common/network/CreativeNetwork.java:110)
- [ ] `CreativeNetwork.sendToClientTrackingAndSelf(2)`: its class isn't translated (it uses `Class`) (team/creative/creativecore/common/network/CreativeNetwork.java:121)
- [ ] `CreativeNetwork.sendToClientAll(2)`: its class isn't translated (it uses `Class`) (team/creative/creativecore/common/network/CreativeNetwork.java:132)

### `CreativeNetworkClient`

- [ ] `CreativeNetworkClient.registerClientType(1)`: it's generic (team/creative/creativecore/common/network/CreativeNetworkClient.java:10)

### `CreativeNetworkField`: it uses `Field` (java.lang.reflect)

- [ ] `CreativeNetworkField.CreativeNetworkField(2)`: its class isn't translated (it uses `Field` (java.lang.reflect)) (team/creative/creativecore/common/network/CreativeNetworkField.java:14)
- [ ] `CreativeNetworkField.write(3)`: its class isn't translated (it uses `Field` (java.lang.reflect)) (team/creative/creativecore/common/network/CreativeNetworkField.java:20)
- [ ] `CreativeNetworkField.read(3)`: its class isn't translated (it uses `Field` (java.lang.reflect)) (team/creative/creativecore/common/network/CreativeNetworkField.java:35)
- [ ] `CreativeNetworkField.create(1)`: it uses CreativeNetworkField, which isn't translated (it uses `Field` (java.lang.reflect)) (team/creative/creativecore/common/network/CreativeNetworkField.java:50)

### `CreativeNetworkPacket`: it's generic

- [ ] `CreativeNetworkPacket.CreativeNetworkPacket(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/CreativeNetworkPacket.java:20)
- [ ] `CreativeNetworkPacket.write(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/CreativeNetworkPacket.java:39)
- [ ] `CreativeNetworkPacket.read(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/CreativeNetworkPacket.java:45)

### `CreativeNetworkUtils`

- [ ] `CreativeNetworkUtils.flatten(1)`: it's generic (team/creative/creativecore/common/network/CreativeNetworkUtils.java:10)

### `CreativePacket`: it implements Minecraft's `CustomPacketPayload`

- [ ] `CreativePacket.execute(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (team/creative/creativecore/common/network/CreativePacket.java:11)
- [ ] `CreativePacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (team/creative/creativecore/common/network/CreativePacket.java:19)
- [ ] `CreativePacket.setType(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (team/creative/creativecore/common/network/CreativePacket.java:23)
- [ ] `CreativePacket.executeClient(1)`: it has no body (team/creative/creativecore/common/network/CreativePacket.java:27)
- [ ] `CreativePacket.executeServer(1)`: it has no body (team/creative/creativecore/common/network/CreativePacket.java:29)
- [ ] `CreativePacket.requiresClient(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (team/creative/creativecore/common/network/CreativePacket.java:31)
- [ ] `CreativePacket.requiresServer(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (team/creative/creativecore/common/network/CreativePacket.java:37)

### `InvalidSideException`: it extends `RuntimeException`

- [ ] `InvalidSideException.InvalidSideException(1)`: its class isn't translated (it extends `RuntimeException`) (team/creative/creativecore/common/network/InvalidSideException.java:8)
- [ ] `InvalidSideException.InvalidSideException(1)`: its class isn't translated (it extends `RuntimeException`) (team/creative/creativecore/common/network/InvalidSideException.java:12)
- [ ] `InvalidSideException.InvalidSideException(1)`: its class isn't translated (it extends `RuntimeException`) (team/creative/creativecore/common/network/InvalidSideException.java:16)
- [ ] `InvalidSideException.InvalidSideException(1)`: its class isn't translated (it extends `RuntimeException`) (team/creative/creativecore/common/network/InvalidSideException.java:20)

### `NetworkFieldType`: it's generic

- [ ] `NetworkFieldType.write(5)`: it has no body (team/creative/creativecore/common/network/type/NetworkFieldType.java:9)
- [ ] `NetworkFieldType.read(4)`: it has no body (team/creative/creativecore/common/network/type/NetworkFieldType.java:11)

### `NetworkFieldTypeClass`: it's generic

- [ ] `NetworkFieldTypeClass.writeContent(2)`: it has no body (team/creative/creativecore/common/network/type/NetworkFieldTypeClass.java:10)
- [ ] `NetworkFieldTypeClass.write(5)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/type/NetworkFieldTypeClass.java:12)
- [ ] `NetworkFieldTypeClass.readContent(1)`: it has no body (team/creative/creativecore/common/network/type/NetworkFieldTypeClass.java:17)
- [ ] `NetworkFieldTypeClass.read(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/type/NetworkFieldTypeClass.java:19)

### `NetworkFieldTypeCodec`: it's generic

- [ ] `NetworkFieldTypeCodec.NetworkFieldTypeCodec(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/type/NetworkFieldTypeCodec.java:13)
- [ ] `NetworkFieldTypeCodec.write(5)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/type/NetworkFieldTypeCodec.java:17)
- [ ] `NetworkFieldTypeCodec.read(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/type/NetworkFieldTypeCodec.java:22)

### `NetworkFieldTypes`

- [x] `NetworkFieldTypes.GSON` (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:63)
- [ ] `NetworkFieldTypes.SPECIAL_PARSERS`: it uses NetworkFieldTypeSpecial, which isn't translated (it's generic) (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:64)
- [ ] `NetworkFieldTypes.PARSERS`: it uses `Class` (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:65)
- [ ] `NetworkFieldTypes.register(2)`: it's generic (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:67)
- [ ] `NetworkFieldTypes.register(2)`: it's generic (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:71)
- [ ] `NetworkFieldTypes.registerAndCodec(2)`: it's generic (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:75)
- [ ] `NetworkFieldTypes.register(2)`: it's generic (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:80)
- [ ] `NetworkFieldTypes.register(1)`: it's generic (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:86)
- [ ] `NetworkFieldTypes.get(1)`: it uses NetworkFieldType, which isn't translated (it's generic) (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:90)
- [ ] `NetworkFieldTypes.get(1)`: it uses NetworkFieldType, which isn't translated (it's generic) (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:94)
- [ ] `NetworkFieldTypes.get(2)`: it uses NetworkFieldType, which isn't translated (it's generic) (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:107)
- [ ] `NetworkFieldTypes.write(4)`: it's generic (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:126)
- [ ] `NetworkFieldTypes.writeMany(4)`: it's generic (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:130)
- [ ] `NetworkFieldTypes.writeMany(4)`: it's generic (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:139)
- [ ] `NetworkFieldTypes.writeMany(4)`: it's generic (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:148)
- [ ] `NetworkFieldTypes.read(3)`: it's generic (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:157)
- [ ] `NetworkFieldTypes.readMany(3)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:161)
- [ ] `NetworkFieldTypes.writeIntArray(2)`: it uses Minecraft's `RegistryFriendlyByteBuf` (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:180)
- [ ] `NetworkFieldTypes.readIntArray(1)`: it uses Minecraft's `RegistryFriendlyByteBuf` (team/creative/creativecore/common/network/type/NetworkFieldTypes.java:188)

### `NetworkFieldTypeSpecial`: it's generic

- [ ] `NetworkFieldTypeSpecial.NetworkFieldTypeSpecial(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/network/type/NetworkFieldTypeSpecial.java:9)

### `CreativeTest`

- [ ] `CreativeTest.test(1)`: it has no body (team/creative/creativecore/common/test/CreativeTest.java:4)

### `CreativeTestArgument`: it implements Minecraft's `ArgumentType`

- [ ] `CreativeTestArgument.test(0)`: it uses CreativeTestArgument, which isn't translated (it implements Minecraft's `ArgumentType`) (team/creative/creativecore/common/test/CreativeTestArgument.java:17)
- [ ] `CreativeTestArgument.getTest(2)`: it calls Minecraft's FabricClientCommandSource.getArgument (no Horizon helper for it yet) (team/creative/creativecore/common/test/CreativeTestArgument.java:21)
- [ ] `CreativeTestArgument.parse(1)`: its class isn't translated (it implements Minecraft's `ArgumentType`) (team/creative/creativecore/common/test/CreativeTestArgument.java:26)
- [ ] `CreativeTestArgument.listSuggestions(2)`: it's generic (team/creative/creativecore/common/test/CreativeTestArgument.java:37)
- [ ] `CreativeTestArgument.getExamples(0)`: its class isn't translated (it implements Minecraft's `ArgumentType`) (team/creative/creativecore/common/test/CreativeTestArgument.java:41)
- [ ] `CreativeTestArgument.getAll(0)`: its class isn't translated (it implements Minecraft's `ArgumentType`) (team/creative/creativecore/common/test/CreativeTestArgument.java:45)
- [ ] `CreativeTestArgument.isAllowed(1)`: its class isn't translated (it implements Minecraft's `ArgumentType`) (team/creative/creativecore/common/test/CreativeTestArgument.java:49)

### `CreativeTestException`: it extends `Exception`

- [ ] `CreativeTestException.CreativeTestException(1)`: its class isn't translated (it extends `Exception`) (team/creative/creativecore/common/test/CreativeTestException.java:4)

### `CreativeTestHelper`: it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)

- [ ] `CreativeTestHelper.trackTime(2)`: it's generic (team/creative/creativecore/common/test/CreativeTestHelper.java:15)
- [ ] `CreativeTestHelper.incrementCount(0)`: its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)) (team/creative/creativecore/common/test/CreativeTestHelper.java:23)
- [ ] `CreativeTestHelper.assertTrue(3)`: its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)) (team/creative/creativecore/common/test/CreativeTestHelper.java:27)
- [ ] `CreativeTestHelper.assertFalse(3)`: its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)) (team/creative/creativecore/common/test/CreativeTestHelper.java:34)
- [ ] `CreativeTestHelper.results(0)`: its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)) (team/creative/creativecore/common/test/CreativeTestHelper.java:41)

### `CreativeTestRegistry`

- [ ] `CreativeTestRegistry.REGISTRY`: it uses NamedHandlerRegistry, which isn't translated (it's generic) (team/creative/creativecore/common/test/CreativeTestRegistry.java:7)

### `TestVectorFan`: it extends `CreativeTest` (team.creative.creativecore.common.test)

- [ ] `TestVectorFan.test(1)`: its class isn't translated (it extends `CreativeTest` (team.creative.creativecore.common.test)) (team/creative/creativecore/common/test/geo/TestVectorFan.java:15)
- [ ] `TestVectorFan.generateVecs(5)`: its class isn't translated (it extends `CreativeTest` (team.creative.creativecore.common.test)) (team/creative/creativecore/common/test/geo/TestVectorFan.java:92)

### `StringArrayArgumentType`: it implements Minecraft's `ArgumentType`

- [x] `StringArrayArgumentType.EXAMPLES` (team/creative/creativecore/common/util/argument/StringArrayArgumentType.java:12)
- [ ] `StringArrayArgumentType.getStringArray(2)`: it calls Minecraft's FabricClientCommandSource.getArgument (no Horizon helper for it yet) (team/creative/creativecore/common/util/argument/StringArrayArgumentType.java:14)
- [ ] `StringArrayArgumentType.stringArray(0)`: it uses StringArrayArgumentType, which isn't translated (it implements Minecraft's `ArgumentType`) (team/creative/creativecore/common/util/argument/StringArrayArgumentType.java:18)
- [ ] `StringArrayArgumentType.parse(1)`: its class isn't translated (it implements Minecraft's `ArgumentType`) (team/creative/creativecore/common/util/argument/StringArrayArgumentType.java:22)
- [ ] `StringArrayArgumentType.getExamples(0)`: its class isn't translated (it implements Minecraft's `ArgumentType`) (team/creative/creativecore/common/util/argument/StringArrayArgumentType.java:26)

### `CompoundSerializer`: it's an interface

- [ ] `CompoundSerializer.write(0)`: it has no body (team/creative/creativecore/common/util/CompoundSerializer.java:6)

### `BiFilterSerializer`: it uses NamedTypeRegistry, which isn't translated (it's generic)

- [ ] `BiFilterSerializer.register(2)`: it's generic (team/creative/creativecore/common/util/filter/BiFilterSerializer.java:12)
- [ ] `BiFilterSerializer.write(1)`: its class isn't translated (it uses NamedTypeRegistry, which isn't translated (it's generic)) (team/creative/creativecore/common/util/filter/BiFilterSerializer.java:17)
- [ ] `BiFilterSerializer.read(1)`: its class isn't translated (it uses NamedTypeRegistry, which isn't translated (it's generic)) (team/creative/creativecore/common/util/filter/BiFilterSerializer.java:56)

### `FilterSerializer`: it uses NamedTypeRegistry, which isn't translated (it's generic)

- [ ] `FilterSerializer.register(2)`: it's generic (team/creative/creativecore/common/util/filter/FilterSerializer.java:12)
- [ ] `FilterSerializer.write(1)`: its class isn't translated (it uses NamedTypeRegistry, which isn't translated (it's generic)) (team/creative/creativecore/common/util/filter/FilterSerializer.java:17)
- [ ] `FilterSerializer.read(1)`: its class isn't translated (it uses NamedTypeRegistry, which isn't translated (it's generic)) (team/creative/creativecore/common/util/filter/FilterSerializer.java:56)

### `BlockFilters`

- [ ] `BlockFilters.block(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:16)
- [ ] `BlockFilters.blocks(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:20)
- [ ] `BlockFilters.instance(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:24)
- [ ] `BlockFilters.and(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:28)
- [ ] `BlockFilters.or(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:32)
- [ ] `BlockFilters.not(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:36)
- [ ] `BlockFilters.property(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:40)
- [ ] `BlockFilters.tag(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:44)

### `BlockFilters.BlockClassFilter`: it implements `Filter` (team.creative.creativecore.common.util.filter)

- [ ] `BlockFilters.BlockClassFilter.BlockClassFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:57)
- [ ] `BlockFilters.BlockClassFilter.BlockClassFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:61)
- [ ] `BlockFilters.BlockClassFilter.is(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:72)
- [ ] `BlockFilters.BlockClassFilter.write(0)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:76)

### `BlockFilters.BlockFilter`: it implements `Filter` (team.creative.creativecore.common.util.filter)

- [ ] `BlockFilters.BlockFilter.BlockFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:87)
- [ ] `BlockFilters.BlockFilter.BlockFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:91)
- [ ] `BlockFilters.BlockFilter.is(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:95)
- [ ] `BlockFilters.BlockFilter.write(0)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:99)

### `BlockFilters.BlockPropertyFilter`: it implements `Filter` (team.creative.creativecore.common.util.filter)

- [ ] `BlockFilters.BlockPropertyFilter.BlockPropertyFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:107)
- [ ] `BlockFilters.BlockPropertyFilter.is(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:108)
- [ ] `BlockFilters.BlockPropertyFilter.property(0)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:107)

### `BlockFilters.BlockTagFilter`: it implements `Filter` (team.creative.creativecore.common.util.filter)

- [ ] `BlockFilters.BlockTagFilter.BlockTagFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:113)
- [ ] `BlockFilters.BlockTagFilter.is(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:114)
- [ ] `BlockFilters.BlockTagFilter.tag(0)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:113)

### `BlockFilters.BlocksFilter`: it implements `Filter` (team.creative.creativecore.common.util.filter)

- [ ] `BlockFilters.BlocksFilter.BlocksFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:122)
- [ ] `BlockFilters.BlocksFilter.BlocksFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:126)
- [ ] `BlockFilters.BlocksFilter.is(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:135)
- [ ] `BlockFilters.BlocksFilter.write(0)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/BlockFilters.java:139)

### `ItemFilters`

- [ ] `ItemFilters.item(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:14)
- [ ] `ItemFilters.items(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:18)
- [ ] `ItemFilters.instance(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:22)
- [ ] `ItemFilters.and(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:26)
- [ ] `ItemFilters.or(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:30)
- [ ] `ItemFilters.not(1)`: it uses `Filter` (team.creative.creativecore.common.util.filter) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:34)

### `ItemFilters.ItemClassFilter`: it implements `Filter` (team.creative.creativecore.common.util.filter)

- [ ] `ItemFilters.ItemClassFilter.ItemClassFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:47)
- [ ] `ItemFilters.ItemClassFilter.ItemClassFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:51)
- [ ] `ItemFilters.ItemClassFilter.is(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:62)
- [ ] `ItemFilters.ItemClassFilter.write(0)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:66)

### `ItemFilters.ItemFilter`: it implements `Filter` (team.creative.creativecore.common.util.filter)

- [ ] `ItemFilters.ItemFilter.ItemFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:77)
- [ ] `ItemFilters.ItemFilter.ItemFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:81)
- [ ] `ItemFilters.ItemFilter.is(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:85)
- [ ] `ItemFilters.ItemFilter.write(0)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:89)

### `ItemFilters.ItemsFilter`: it implements `Filter` (team.creative.creativecore.common.util.filter)

- [ ] `ItemFilters.ItemsFilter.ItemsFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:100)
- [ ] `ItemFilters.ItemsFilter.ItemsFilter(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:104)
- [ ] `ItemFilters.ItemsFilter.is(1)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:113)
- [ ] `ItemFilters.ItemsFilter.write(0)`: its class isn't translated (it implements `Filter` (team.creative.creativecore.common.util.filter)) (team/creative/creativecore/common/util/filter/premade/ItemFilters.java:117)

### `CreativeIngredient`

- [ ] `CreativeIngredient.REGISTRY`: it uses NamedTypeRegistry, which isn't translated (it's generic) (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:22)
- [ ] `CreativeIngredient.OBJECT_PARSERS`: it uses `Object` (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:23)
- [ ] `CreativeIngredient.EMPTY`: its value couldn't be read: an anonymous class (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:24)
- [ ] `CreativeIngredient.registerType(3)`: it's generic (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:69)
- [ ] `CreativeIngredient.parse(1)`: it uses `Object` (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:76)
- [ ] `CreativeIngredient.load(2)`: it uses Minecraft's `Provider` (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:98)
- [ ] `CreativeIngredient.save(1)`: it uses Minecraft's `CompoundTag` (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:114)
- [ ] `CreativeIngredient.saveExtra(2)`: it has no body (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:121)
- [ ] `CreativeIngredient.loadExtra(2)`: it has no body (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:123)
- [ ] `CreativeIngredient.is(2)`: it has no body (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:125)
- [ ] `CreativeIngredient.is(1)`: it has no body (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:127)
- [ ] `CreativeIngredient.getExample(0)`: it has no body (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:129)
- [ ] `CreativeIngredient.copy(0)`: it has no body (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:131)
- [ ] `CreativeIngredient.equals(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:133)
- [ ] `CreativeIngredient.equals(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:138)
- [ ] `CreativeIngredient.description(0)`: it has no body (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:140)
- [ ] `CreativeIngredient.descriptionDetail(0)`: it has no body (team/creative/creativecore/common/util/ingredient/CreativeIngredient.java:142)

### `CreativeIngredientBlock`: it extends `CreativeIngredient`

- [ ] `CreativeIngredientBlock.CreativeIngredientBlock(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java:19)
- [ ] `CreativeIngredientBlock.CreativeIngredientBlock(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java:23)
- [ ] `CreativeIngredientBlock.loadExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java:26)
- [ ] `CreativeIngredientBlock.saveExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java:34)
- [ ] `CreativeIngredientBlock.is(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java:39)
- [ ] `CreativeIngredientBlock.is(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java:48)
- [ ] `CreativeIngredientBlock.equals(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java:53)
- [ ] `CreativeIngredientBlock.getExample(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java:58)
- [ ] `CreativeIngredientBlock.copy(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java:63)
- [ ] `CreativeIngredientBlock.description(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java:68)
- [ ] `CreativeIngredientBlock.descriptionDetail(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlock.java:73)

### `CreativeIngredientBlockTag`: it extends `CreativeIngredient`

- [ ] `CreativeIngredientBlockTag.CreativeIngredientBlockTag(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java:22)
- [ ] `CreativeIngredientBlockTag.CreativeIngredientBlockTag(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java:26)
- [ ] `CreativeIngredientBlockTag.saveExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java:29)
- [ ] `CreativeIngredientBlockTag.loadExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java:34)
- [ ] `CreativeIngredientBlockTag.is(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java:39)
- [ ] `CreativeIngredientBlockTag.is(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java:45)
- [ ] `CreativeIngredientBlockTag.getExample(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java:50)
- [ ] `CreativeIngredientBlockTag.equals(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java:56)
- [ ] `CreativeIngredientBlockTag.copy(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java:61)
- [ ] `CreativeIngredientBlockTag.description(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java:66)
- [ ] `CreativeIngredientBlockTag.descriptionDetail(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientBlockTag.java:71)

### `CreativeIngredientFuel`: it extends `CreativeIngredient`

- [ ] `CreativeIngredientFuel.saveExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientFuel.java:12)
- [ ] `CreativeIngredientFuel.loadExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientFuel.java:16)
- [ ] `CreativeIngredientFuel.getExample(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientFuel.java:20)
- [ ] `CreativeIngredientFuel.is(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientFuel.java:25)
- [ ] `CreativeIngredientFuel.is(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientFuel.java:30)
- [ ] `CreativeIngredientFuel.equals(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientFuel.java:35)
- [ ] `CreativeIngredientFuel.copy(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientFuel.java:40)
- [ ] `CreativeIngredientFuel.description(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientFuel.java:45)
- [ ] `CreativeIngredientFuel.descriptionDetail(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientFuel.java:50)

### `CreativeIngredientItem`: it extends `CreativeIngredient`

- [ ] `CreativeIngredientItem.CreativeIngredientItem(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java:20)
- [ ] `CreativeIngredientItem.CreativeIngredientItem(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java:24)
- [ ] `CreativeIngredientItem.saveExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java:27)
- [ ] `CreativeIngredientItem.loadExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java:32)
- [ ] `CreativeIngredientItem.getExample(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java:40)
- [ ] `CreativeIngredientItem.is(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java:45)
- [ ] `CreativeIngredientItem.is(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java:54)
- [ ] `CreativeIngredientItem.equals(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java:59)
- [ ] `CreativeIngredientItem.copy(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java:64)
- [ ] `CreativeIngredientItem.description(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java:69)
- [ ] `CreativeIngredientItem.descriptionDetail(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItem.java:74)

### `CreativeIngredientItemStack`: it extends `CreativeIngredient`

- [ ] `CreativeIngredientItemStack.CreativeIngredientItemStack(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:23)
- [ ] `CreativeIngredientItemStack.CreativeIngredientItemStack(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:27)
- [ ] `CreativeIngredientItemStack.CreativeIngredientItemStack(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:32)
- [ ] `CreativeIngredientItemStack.getIncluded(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:35)
- [ ] `CreativeIngredientItemStack.saveExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:39)
- [ ] `CreativeIngredientItemStack.loadExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:47)
- [ ] `CreativeIngredientItemStack.is(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:60)
- [ ] `CreativeIngredientItemStack.is(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:65)
- [ ] `CreativeIngredientItemStack.getExample(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:80)
- [ ] `CreativeIngredientItemStack.equals(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:85)
- [ ] `CreativeIngredientItemStack.copy(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:90)
- [ ] `CreativeIngredientItemStack.description(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:95)
- [ ] `CreativeIngredientItemStack.descriptionDetail(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemStack.java:100)

### `CreativeIngredientItemTag`: it extends `CreativeIngredient`

- [ ] `CreativeIngredientItemTag.CreativeIngredientItemTag(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java:21)
- [ ] `CreativeIngredientItemTag.CreativeIngredientItemTag(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java:25)
- [ ] `CreativeIngredientItemTag.saveExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java:28)
- [ ] `CreativeIngredientItemTag.loadExtra(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java:33)
- [ ] `CreativeIngredientItemTag.is(2)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java:38)
- [ ] `CreativeIngredientItemTag.is(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java:43)
- [ ] `CreativeIngredientItemTag.getExample(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java:48)
- [ ] `CreativeIngredientItemTag.equals(1)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java:54)
- [ ] `CreativeIngredientItemTag.copy(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java:59)
- [ ] `CreativeIngredientItemTag.description(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java:64)
- [ ] `CreativeIngredientItemTag.descriptionDetail(0)`: its class isn't translated (it extends `CreativeIngredient`) (team/creative/creativecore/common/util/ingredient/CreativeIngredientItemTag.java:69)

### `GuiCreativeIngredientHandler`

- [ ] `GuiCreativeIngredientHandler.REGISTRY`: it uses NamedHandlerRegistry, which isn't translated (it's generic) (team/creative/creativecore/common/util/ingredient/GuiCreativeIngredientHandler.java:40)
- [ ] `GuiCreativeIngredientHandler.find(1)`: it uses GuiCreativeIngredientHandler.REGISTRY, which isn't translated (it uses NamedHandlerRegistry, which isn't translated (it's generic)) (team/creative/creativecore/common/util/ingredient/GuiCreativeIngredientHandler.java:42)
- [ ] `GuiCreativeIngredientHandler.canHandle(1)`: it has no body (team/creative/creativecore/common/util/ingredient/GuiCreativeIngredientHandler.java:54)
- [ ] `GuiCreativeIngredientHandler.createControls(2)`: it has no body (team/creative/creativecore/common/util/ingredient/GuiCreativeIngredientHandler.java:56)
- [ ] `GuiCreativeIngredientHandler.parseControls(1)`: it has no body (team/creative/creativecore/common/util/ingredient/GuiCreativeIngredientHandler.java:58)
- [ ] `GuiCreativeIngredientHandler.onChanged(2)`: it uses GuiParent, which isn't translated (it extends `GuiControl`) (team/creative/creativecore/common/util/ingredient/GuiCreativeIngredientHandler.java:60)

### `GuiCreativeIngredientHandler.GuiDataCheckList`: it extends `GuiCheckList` (team.creative.creativecore.common.gui.control.collection)

- [ ] `GuiCreativeIngredientHandler.GuiDataCheckList.GuiDataCheckList(5)`: its class isn't translated (it extends `GuiCheckList` (team.creative.creativecore.common.gui.control.collection)) (team/creative/creativecore/common/util/ingredient/GuiCreativeIngredientHandler.java:284)
- [ ] `GuiCreativeIngredientHandler.GuiDataCheckList.includes(1)`: its class isn't translated (it extends `GuiCheckList` (team.creative.creativecore.common.gui.control.collection)) (team/creative/creativecore/common/util/ingredient/GuiCreativeIngredientHandler.java:289)
- [ ] `GuiCreativeIngredientHandler.GuiDataCheckList.getConfiguredIncluded(0)`: its class isn't translated (it extends `GuiCheckList` (team.creative.creativecore.common.gui.control.collection)) (team/creative/creativecore/common/util/ingredient/GuiCreativeIngredientHandler.java:293)

### `ContainerSlotView`: it uses Minecraft's `Player`

- [ ] `ContainerSlotView.EMPTY`: its value couldn't be read: an anonymous class (team/creative/creativecore/common/util/inventory/ContainerSlotView.java:12)
- [ ] `ContainerSlotView.mainHand(1)`: it uses ContainerSlotView, which isn't translated (it uses Minecraft's `Player`) (team/creative/creativecore/common/util/inventory/ContainerSlotView.java:30)
- [ ] `ContainerSlotView.offHand(1)`: it uses ContainerSlotView, which isn't translated (it uses Minecraft's `Player`) (team/creative/creativecore/common/util/inventory/ContainerSlotView.java:34)
- [ ] `ContainerSlotView.ContainerSlotView(3)`: its class isn't translated (it uses Minecraft's `Player`) (team/creative/creativecore/common/util/inventory/ContainerSlotView.java:38)
- [ ] `ContainerSlotView.get(0)`: its class isn't translated (it uses Minecraft's `Player`) (team/creative/creativecore/common/util/inventory/ContainerSlotView.java:44)
- [ ] `ContainerSlotView.set(1)`: its class isn't translated (it uses Minecraft's `Player`) (team/creative/creativecore/common/util/inventory/ContainerSlotView.java:48)
- [ ] `ContainerSlotView.changed(0)`: its class isn't translated (it uses Minecraft's `Player`) (team/creative/creativecore/common/util/inventory/ContainerSlotView.java:53)

### `InventoryUtils`

- [ ] `InventoryUtils.save(2)`: it uses Minecraft's `CompoundTag` (team/creative/creativecore/common/util/inventory/InventoryUtils.java:19)
- [ ] `InventoryUtils.load(2)`: it uses Minecraft's `SimpleContainer` (team/creative/creativecore/common/util/inventory/InventoryUtils.java:32)
- [ ] `InventoryUtils.load(3)`: it uses Minecraft's `SimpleContainer` (team/creative/creativecore/common/util/inventory/InventoryUtils.java:36)
- [ ] `InventoryUtils.isItemStackEqual(2)`: it calls Minecraft's `ItemStack`.isSameItemSameComponents (team/creative/creativecore/common/util/inventory/InventoryUtils.java:50)
- [ ] `InventoryUtils.consumeItemStack(2)`: it uses Minecraft's `Container` (team/creative/creativecore/common/util/inventory/InventoryUtils.java:60)
- [ ] `InventoryUtils.addItemStackToInventory(2)`: it uses Minecraft's `Container` (team/creative/creativecore/common/util/inventory/InventoryUtils.java:78)
- [ ] `InventoryUtils.getAmount(2)`: it uses Minecraft's `Container` (team/creative/creativecore/common/util/inventory/InventoryUtils.java:104)
- [ ] `InventoryUtils.cleanInventory(1)`: it uses Minecraft's `Container` (team/creative/creativecore/common/util/inventory/InventoryUtils.java:116)
- [ ] `InventoryUtils.consume(3)`: it uses Minecraft's `Level` (team/creative/creativecore/common/util/inventory/InventoryUtils.java:125)
- [ ] `InventoryUtils.consume(4)`: it uses Minecraft's `Level` (team/creative/creativecore/common/util/inventory/InventoryUtils.java:129)
- [ ] `InventoryUtils.sortInventory(2)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/inventory/InventoryUtils.java:155)
- [ ] `InventoryUtils.toString(1)`: it uses Minecraft's `Container` (team/creative/creativecore/common/util/inventory/InventoryUtils.java:221)
- [ ] `InventoryUtils.asContent(1)`: it uses Minecraft's `ItemContainerContents` (team/creative/creativecore/common/util/inventory/InventoryUtils.java:241)

### `Axis`: its constants carry values

- [ ] `Axis.get(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:370)
- [ ] `Axis.third(2)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:379)
- [ ] `Axis.getMirrorAxis(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:387)
- [ ] `Axis.Axis(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:395)
- [ ] `Axis.getMatrix(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:399)
- [ ] `Axis.one(0)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:403)
- [ ] `Axis.two(0)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:405)
- [ ] `Axis.facing(1)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:407)
- [ ] `Axis.get(3)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:409)
- [ ] `Axis.get(3)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:411)
- [ ] `Axis.get(3)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:413)
- [ ] `Axis.get(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:415)
- [ ] `Axis.set(2)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:417)
- [ ] `Axis.set(2)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:419)
- [ ] `Axis.set(2)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:421)
- [ ] `Axis.set(2)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:423)
- [ ] `Axis.get(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:425)
- [ ] `Axis.set(2)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:427)
- [ ] `Axis.get(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:429)
- [ ] `Axis.set(2)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:431)
- [ ] `Axis.get(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:433)
- [ ] `Axis.set(2)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:435)
- [ ] `Axis.get(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:437)
- [ ] `Axis.set(2)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:439)
- [ ] `Axis.get(3)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:441)
- [ ] `Axis.toVanilla(0)`: it has no body (team/creative/creativecore/common/util/math/base/Axis.java:443)
- [ ] `Axis.mirror(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:445)
- [ ] `Axis.mirror(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:449)
- [ ] `Axis.mirror(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:453)
- [ ] `Axis.mirror(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:455)
- [ ] `Axis.mirror(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:457)
- [ ] `Axis.mirror(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:461)
- [ ] `Axis.transform(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Axis.java:465)

### `Facing`: its constants carry values

- [ ] `Facing.VALUES`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:260)
- [ ] `Facing.HORIZONTA_VALUES`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:261)
- [x] `Facing.FACING_NAMES` (team/creative/creativecore/common/util/math/base/Facing.java:262)
- [x] `Facing.HORIZONTAL_FACING_NAMES` (team/creative/creativecore/common/util/math/base/Facing.java:263)
- [ ] `Facing.get(1)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:271)
- [ ] `Facing.get(1)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:275)
- [ ] `Facing.get(2)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:291)
- [ ] `Facing.getHorizontal(1)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:300)
- [ ] `Facing.of(1)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:304)
- [ ] `Facing.ofNormal(1)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:317)
- [ ] `Facing.direction(2)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:335)
- [ ] `Facing.nearest(1)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:347)
- [ ] `Facing.nearest(3)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:351)
- [ ] `Facing.Facing(4)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:371)
- [ ] `Facing.offset(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:379)
- [ ] `Facing.offset(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:383)
- [ ] `Facing.translate(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:387)
- [ ] `Facing.opposite(0)`: it has no body (team/creative/creativecore/common/util/math/base/Facing.java:391)
- [ ] `Facing.toVanilla(0)`: it has no body (team/creative/creativecore/common/util/math/base/Facing.java:393)
- [ ] `Facing.one(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:395)
- [ ] `Facing.two(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:399)
- [ ] `Facing.getUAxis(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:403)
- [ ] `Facing.getVAxis(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:412)
- [ ] `Facing.getU(3)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:421)
- [ ] `Facing.getV(3)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:430)
- [ ] `Facing.get(1)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:439)
- [ ] `Facing.set(2)`: it has no body (team/creative/creativecore/common/util/math/base/Facing.java:441)
- [ ] `Facing.rotation(0)`: it has no body (team/creative/creativecore/common/util/math/base/Facing.java:443)
- [ ] `Facing.get(6)`: it has no body (team/creative/creativecore/common/util/math/base/Facing.java:445)
- [ ] `Facing.get(6)`: it has no body (team/creative/creativecore/common/util/math/base/Facing.java:447)
- [ ] `Facing.get(6)`: it has no body (team/creative/creativecore/common/util/math/base/Facing.java:449)
- [ ] `Facing.transform(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/base/Facing.java:451)

### `ABB`

- [ ] `ABB.min(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:23)
- [ ] `ABB.max(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:32)
- [ ] `ABB.min(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:41)
- [ ] `ABB.max(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:49)
- [ ] `ABB.get(2)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/ABB.java:57)
- [x] `ABB.createEmptyBox(0)` (team/creative/creativecore/common/util/math/box/ABB.java:68)
- [ ] `ABB.clip(4)`: it uses Minecraft's `BlockHitResult` (team/creative/creativecore/common/util/math/box/ABB.java:79)
- [x] `ABB.ABB(6)` (team/creative/creativecore/common/util/math/box/ABB.java:98)
- [ ] `ABB.ABB(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:107)
- [ ] `ABB.ABB(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:111)
- [ ] `ABB.get(1)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/ABB.java:115)
- [ ] `ABB.min(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:126)
- [ ] `ABB.max(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:135)
- [ ] `ABB.min(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:144)
- [ ] `ABB.max(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:152)
- [ ] `ABB.corner(1)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/box/ABB.java:160)
- [ ] `ABB.cornerValue(3)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/ABB.java:164)
- [ ] `ABB.cornerX(1)`: it uses BoxCorner, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/ABB.java:168)
- [ ] `ABB.cornerY(1)`: it uses BoxCorner, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/ABB.java:172)
- [ ] `ABB.cornerZ(1)`: it uses BoxCorner, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/ABB.java:176)
- [ ] `ABB.intersectsWithAxis(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:180)
- [ ] `ABB.intersectsWithAxis(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:188)
- [ ] `ABB.intersectsWithAxis(4)`: it uses Minecraft's `Axis` (team/creative/creativecore/common/util/math/box/ABB.java:196)
- [ ] `ABB.calculateAxisOffset(5)`: it uses Minecraft's `Axis` (team/creative/creativecore/common/util/math/box/ABB.java:202)
- [ ] `ABB.copy(0)`: it creates a ABB with a constructor that isn't translated (team/creative/creativecore/common/util/math/box/ABB.java:226)
- [x] `ABB.move(3)` (team/creative/creativecore/common/util/math/box/ABB.java:230)
- [ ] `ABB.moveCopy(3)`: it calls ABB.copy, which isn't translated (team/creative/creativecore/common/util/math/box/ABB.java:239)
- [ ] `ABB.getCorners(0)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/box/ABB.java:245)
- [ ] `ABB.getRotatedCorners(1)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/box/ABB.java:255)
- [ ] `ABB.getOuterCorner(6)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/box/ABB.java:265)
- [ ] `ABB.include(1)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/box/ABB.java:289)
- [ ] `ABB.include(2)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/ABB.java:298)
- [ ] `ABB.toVanilla(0)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/ABB.java:320)
- [ ] `ABB.rayTrace(3)`: it uses Minecraft's `BlockHitResult` (team/creative/creativecore/common/util/math/box/ABB.java:324)
- [ ] `ABB.clipFacing(7)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/ABB.java:333)
- [ ] `ABB.clipPoint(8)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/ABB.java:344)
- [ ] `ABB.createRotatedSurrounding(1)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/box/ABB.java:361)
- [ ] `ABB.createRotatedSurroundingInverseInternal(1)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/box/ABB.java:383)
- [ ] `ABB.contains(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:405)
- [ ] `ABB.contains(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:409)
- [ ] `ABB.intersectsPrecise(1)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/ABB.java:413)
- [ ] `ABB.intersects(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:417)
- [ ] `ABB.intersects(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:421)
- [x] `ABB.intersects(6)` (team/creative/creativecore/common/util/math/box/ABB.java:425)
- [ ] `ABB.intersects(2)`: it uses Minecraft's `Vec3` (team/creative/creativecore/common/util/math/box/ABB.java:429)
- [x] `ABB.contains(3)` (team/creative/creativecore/common/util/math/box/ABB.java:440)
- [ ] `ABB.getCenter(0)`: it uses Minecraft's `Vec3` (team/creative/creativecore/common/util/math/box/ABB.java:444)
- [x] `ABB.toString(0)` (team/creative/creativecore/common/util/math/box/ABB.java:448)
- [ ] `ABB.equals(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:453)
- [ ] `ABB.equals(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/ABB.java:457)
- [ ] `ABB.combine(1)`: `this` used as a value (team/creative/creativecore/common/util/math/box/ABB.java:464)
- [x] `ABB.deflate(1)` (team/creative/creativecore/common/util/math/box/ABB.java:505)
- [x] `ABB.deflate(3)` (team/creative/creativecore/common/util/math/box/ABB.java:509)
- [x] `ABB.inflate(1)` (team/creative/creativecore/common/util/math/box/ABB.java:513)
- [x] `ABB.inflate(3)` (team/creative/creativecore/common/util/math/box/ABB.java:517)

### `ABBs`: it implements `Iterable`

- [ ] `ABBs.cutOut(8)`: a `continue` outside a loop (team/creative/creativecore/common/util/math/box/ABBs.java:12)
- [ ] `ABBs.ABBs(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:107)
- [ ] `ABBs.ABBs(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:111)
- [ ] `ABBs.ABBs(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:115)
- [ ] `ABBs.addShape(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:119)
- [ ] `ABBs.cutOutVanilla(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:131)
- [ ] `ABBs.cutOut(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:137)
- [ ] `ABBs.cutOut(6)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:143)
- [ ] `ABBs.intersectionVanilla(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:154)
- [ ] `ABBs.intersection(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:160)
- [ ] `ABBs.isEmpty(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:182)
- [ ] `ABBs.getBoxes(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:186)
- [ ] `ABBs.addNonOverlappingVanilla(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:190)
- [ ] `ABBs.addNonOverlapping(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:196)
- [ ] `ABBs.addNonOverlapping(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:202)
- [ ] `ABBs.optimize(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:222)
- [ ] `ABBs.iterator(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/math/box/ABBs.java:251)

### `AlignedBox`

- [x] `AlignedBox.AlignedBox(6)` (team/creative/creativecore/common/util/math/box/AlignedBox.java:24)
- [ ] `AlignedBox.AlignedBox(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/AlignedBox.java:33)
- [x] `AlignedBox.AlignedBox(0)` (team/creative/creativecore/common/util/math/box/AlignedBox.java:37)
- [ ] `AlignedBox.AlignedBox(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/AlignedBox.java:41)
- [x] `AlignedBox.add(3)` (team/creative/creativecore/common/util/math/box/AlignedBox.java:45)
- [x] `AlignedBox.sub(3)` (team/creative/creativecore/common/util/math/box/AlignedBox.java:54)
- [ ] `AlignedBox.add(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/AlignedBox.java:63)
- [ ] `AlignedBox.sub(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/AlignedBox.java:67)
- [ ] `AlignedBox.add(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/AlignedBox.java:71)
- [ ] `AlignedBox.sub(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/AlignedBox.java:75)
- [x] `AlignedBox.scale(1)` (team/creative/creativecore/common/util/math/box/AlignedBox.java:79)
- [ ] `AlignedBox.getSize(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/AlignedBox.java:88)
- [ ] `AlignedBox.getSize(0)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/box/AlignedBox.java:96)
- [ ] `AlignedBox.getCenter(0)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/box/AlignedBox.java:100)
- [x] `AlignedBox.toString(0)` (team/creative/creativecore/common/util/math/box/AlignedBox.java:104)
- [ ] `AlignedBox.getCorner(1)`: it uses Vec3f, which isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/box/AlignedBox.java:109)
- [ ] `AlignedBox.getBB(0)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/AlignedBox.java:113)
- [ ] `AlignedBox.getBB(1)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/AlignedBox.java:117)
- [ ] `AlignedBox.voxelShape(0)`: it uses Minecraft's `VoxelShape` (team/creative/creativecore/common/util/math/box/AlignedBox.java:123)
- [ ] `AlignedBox.voxelShape(1)`: it uses Minecraft's `VoxelShape` (team/creative/creativecore/common/util/math/box/AlignedBox.java:127)
- [ ] `AlignedBox.rotate(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/AlignedBox.java:133)
- [ ] `AlignedBox.rotate(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/AlignedBox.java:145)
- [x] `AlignedBox.set(6)` (team/creative/creativecore/common/util/math/box/AlignedBox.java:157)
- [ ] `AlignedBox.getOffset(0)`: it uses Minecraft's `BlockPos` (team/creative/creativecore/common/util/math/box/AlignedBox.java:166)
- [ ] `AlignedBox.get(1)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/AlignedBox.java:170)
- [ ] `AlignedBox.set(2)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/AlignedBox.java:181)
- [ ] `AlignedBox.setMin(2)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/AlignedBox.java:203)
- [ ] `AlignedBox.getMin(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/AlignedBox.java:216)
- [ ] `AlignedBox.setMax(2)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/AlignedBox.java:224)
- [ ] `AlignedBox.getMax(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/AlignedBox.java:237)
- [x] `AlignedBox.grow(1)` (team/creative/creativecore/common/util/math/box/AlignedBox.java:245)
- [ ] `AlignedBox.grow(2)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/AlignedBox.java:254)
- [ ] `AlignedBox.shrink(2)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/AlignedBox.java:260)

### `BoxCorner`: its constants carry values

- [ ] `BoxCorner.LOOKUP`: it uses BoxCorner, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:21)
- [ ] `BoxCorner.FACING_CORNERS`: it uses BoxCorner, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:22)
- [ ] `BoxCorner.BoxCorner(3)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:32)
- [ ] `BoxCorner.lookup(3)`: it uses BoxCorner, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:38)
- [ ] `BoxCorner.getCornerUnsorted(3)`: it uses BoxCorner, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:43)
- [ ] `BoxCorner.getCorner(3)`: it uses BoxCorner, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:50)
- [ ] `BoxCorner.faceCorners(1)`: it uses BoxCorner, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:55)
- [ ] `BoxCorner.isFacing(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:59)
- [ ] `BoxCorner.isFacingPositive(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:63)
- [ ] `BoxCorner.getFacing(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:67)
- [ ] `BoxCorner.transform(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:75)
- [ ] `BoxCorner.mirror(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:84)
- [ ] `BoxCorner.rotate(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:92)
- [ ] `BoxCorner.get(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:100)
- [ ] `BoxCorner.set(2)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:104)
- [ ] `BoxCorner.get(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:110)
- [ ] `BoxCorner.set(2)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:114)
- [ ] `BoxCorner.facingTo(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxCorner.java:120)

### `BoxesVoxelShape`: it extends Minecraft's `SliceShape`

- [ ] `BoxesVoxelShape.DISCRETE_SHAPE`: its value couldn't be read: an anonymous class (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:27)
- [ ] `BoxesVoxelShape.create(1)`: it uses BoxesVoxelShape, which isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:50)
- [ ] `BoxesVoxelShape.createVanilla(1)`: it uses BoxesVoxelShape, which isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:57)
- [ ] `BoxesVoxelShape.create(1)`: it uses BoxesVoxelShape, which isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:67)
- [ ] `BoxesVoxelShape.BoxesVoxelShape(0)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:71)
- [ ] `BoxesVoxelShape.move(3)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:75)
- [ ] `BoxesVoxelShape.getCoords(1)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:85)
- [ ] `BoxesVoxelShape.get(2)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:97)
- [ ] `BoxesVoxelShape.min(1)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:101)
- [ ] `BoxesVoxelShape.max(1)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:111)
- [ ] `BoxesVoxelShape.forAllEdges(1)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:121)
- [ ] `BoxesVoxelShape.forAllEdgesCorner(4)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:132)
- [ ] `BoxesVoxelShape.forAllBoxes(1)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:136)
- [ ] `BoxesVoxelShape.toAabbs(0)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:142)
- [ ] `BoxesVoxelShape.min(3)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:152)
- [ ] `BoxesVoxelShape.max(3)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:166)
- [ ] `BoxesVoxelShape.findIndex(2)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:180)
- [ ] `BoxesVoxelShape.clip(3)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:184)
- [ ] `BoxesVoxelShape.closestPointTo(1)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:189)
- [ ] `BoxesVoxelShape.getFaceShape(1)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:204)
- [ ] `BoxesVoxelShape.isEmpty(0)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:208)
- [ ] `BoxesVoxelShape.optimize(0)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:212)
- [ ] `BoxesVoxelShape.requiresAdvancedEntityStep(0)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:216)
- [ ] `BoxesVoxelShape.intersectsWith(1)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:226)
- [ ] `BoxesVoxelShape.onlyKeepIntersecting(1)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:236)
- [ ] `BoxesVoxelShape.collide(3)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:241)
- [ ] `BoxesVoxelShape.collideX(3)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:265)
- [ ] `BoxesVoxelShape.bounds(0)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:269)
- [ ] `BoxesVoxelShape.toString(0)`: its class isn't translated (it extends Minecraft's `SliceShape`) (team/creative/creativecore/common/util/math/box/BoxesVoxelShape.java:289)

### `BoxFace`: its constants carry values

- [ ] `BoxFace.BoxFace(4)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:27)
- [ ] `BoxFace.getCornerInQuestion(2)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:40)
- [ ] `BoxFace.getTriangle(2)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:48)
- [ ] `BoxFace.getTexUAxis(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:56)
- [ ] `BoxFace.getTexVAxis(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:60)
- [ ] `BoxFace.getTexU(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:64)
- [ ] `BoxFace.getTexV(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:68)
- [ ] `BoxFace.getTriangleFirst(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:72)
- [ ] `BoxFace.getTriangleSecond(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:76)
- [ ] `BoxFace.getCorners(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:80)
- [ ] `BoxFace.getFacing(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:84)
- [ ] `BoxFace.getOne(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:88)
- [ ] `BoxFace.getTwo(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:92)
- [ ] `BoxFace.first(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:96)
- [ ] `BoxFace.normal(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:100)
- [ ] `BoxFace.isFacingOutwards(3)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:109)
- [ ] `BoxFace.getTraingleNormal(2)`: it uses Vec3f, which isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/box/BoxFace.java:127)
- [ ] `BoxFace.ensureSameLength(2)`: it uses Vec3f, which isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/box/BoxFace.java:137)
- [ ] `BoxFace.get(1)`: it uses BoxFace, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:144)
- [ ] `BoxFace.get(2)`: it uses BoxFace, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxFace.java:155)
- [ ] `BoxFace.getVecArray(2)`: it uses Vec3f, which isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/box/BoxFace.java:163)

### `BoxPlane`: it uses BoxFace, which isn't translated (its constants carry values)

- [ ] `BoxPlane.BoxPlane(2)`: its class isn't translated (it uses BoxFace, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/box/BoxPlane.java:11)
- [ ] `BoxPlane.getIntersectingScale(2)`: its class isn't translated (it uses BoxFace, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/box/BoxPlane.java:17)
- [ ] `BoxPlane.linePlaneIntersection(4)`: it calls Minecraft's Vec3.dot (no Horizon helper for it yet) (team/creative/creativecore/common/util/math/box/BoxPlane.java:22)
- [ ] `BoxPlane.createPlane(3)`: it uses BoxPlane, which isn't translated (it uses BoxFace, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/box/BoxPlane.java:27)
- [ ] `BoxPlane.createOppositePlane(3)`: it uses BoxPlane, which isn't translated (it uses BoxFace, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/box/BoxPlane.java:32)

### `BoxUtils`

- [x] `BoxUtils.equals(3)` (team/creative/creativecore/common/util/math/box/BoxUtils.java:13)
- [x] `BoxUtils.greaterEquals(3)` (team/creative/creativecore/common/util/math/box/BoxUtils.java:17)
- [x] `BoxUtils.insideRect(6)` (team/creative/creativecore/common/util/math/box/BoxUtils.java:21)
- [ ] `BoxUtils.lengthIgnoreAxis(3)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxUtils.java:25)
- [ ] `BoxUtils.includeMaxRotationInBox(4)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxUtils.java:38)
- [ ] `BoxUtils.includeMaxRotationInBoxInverse(4)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxUtils.java:45)
- [ ] `BoxUtils.includeMaxRotationInBox(7)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxUtils.java:60)
- [ ] `BoxUtils.get(2)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/BoxUtils.java:99)
- [ ] `BoxUtils.min(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/BoxUtils.java:110)
- [ ] `BoxUtils.max(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/BoxUtils.java:119)
- [ ] `BoxUtils.min(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/BoxUtils.java:128)
- [ ] `BoxUtils.max(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/box/BoxUtils.java:136)
- [ ] `BoxUtils.corner(2)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/BoxUtils.java:144)
- [ ] `BoxUtils.cornerValue(3)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/BoxUtils.java:148)
- [ ] `BoxUtils.cornerX(2)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/BoxUtils.java:152)
- [ ] `BoxUtils.cornerY(2)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/BoxUtils.java:156)
- [ ] `BoxUtils.cornerZ(2)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/BoxUtils.java:160)
- [ ] `BoxUtils.getCorners(1)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/BoxUtils.java:164)
- [ ] `BoxUtils.getRotatedCorners(2)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/BoxUtils.java:174)
- [ ] `BoxUtils.intersectsWithAxis(4)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/BoxUtils.java:184)
- [ ] `BoxUtils.intersectsWithAxis(5)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/BoxUtils.java:191)
- [ ] `BoxUtils.calculateAxisOffset(6)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/box/BoxUtils.java:198)
- [ ] `BoxUtils.getIntersectionVolume(2)`: it uses Minecraft's `AABB` (team/creative/creativecore/common/util/math/box/BoxUtils.java:216)

### `OBB`: it extends `ABB`

- [x] `OBB.getCornerOffset(3)` (team/creative/creativecore/common/util/math/box/OBB.java:17)
- [ ] `OBB.calculateDistanceFromPlane(6)`: it uses Vec2d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/box/OBB.java:25)
- [ ] `OBB.OBB(2)`: its class isn't translated (it extends `ABB`) (team/creative/creativecore/common/util/math/box/OBB.java:32)
- [ ] `OBB.OBB(2)`: its class isn't translated (it extends `ABB`) (team/creative/creativecore/common/util/math/box/OBB.java:37)
- [ ] `OBB.copy(0)`: its class isn't translated (it extends `ABB`) (team/creative/creativecore/common/util/math/box/OBB.java:42)
- [ ] `OBB.intersectsPrecise(1)`: its class isn't translated (it extends `ABB`) (team/creative/creativecore/common/util/math/box/OBB.java:46)
- [ ] `OBB.calculateDistanceRotated(3)`: its body couldn't be read: a labelled statement (team/creative/creativecore/common/util/math/box/OBB.java:51)
- [ ] `OBB.calculateAxisOffset(5)`: its class isn't translated (it extends `ABB`) (team/creative/creativecore/common/util/math/box/OBB.java:140)
- [ ] `OBB.toString(0)`: its class isn't translated (it extends `ABB`) (team/creative/creativecore/common/util/math/box/OBB.java:160)

### `CollidingPlane`: it uses Minecraft's `AABB`

- [x] `CollidingPlane.accuracySteps` (team/creative/creativecore/common/util/math/collision/CollidingPlane.java:11)
- [ ] `CollidingPlane.CollidingPlane(5)`: its class isn't translated (it uses Minecraft's `AABB`) (team/creative/creativecore/common/util/math/collision/CollidingPlane.java:18)
- [ ] `CollidingPlane.isInFront(1)`: its class isn't translated (it uses Minecraft's `AABB`) (team/creative/creativecore/common/util/math/collision/CollidingPlane.java:32)
- [ ] `CollidingPlane.binarySearch(5)`: its class isn't translated (it uses Minecraft's `AABB`) (team/creative/creativecore/common/util/math/collision/CollidingPlane.java:41)
- [ ] `CollidingPlane.searchBetweenSimple(8)`: its class isn't translated (it uses Minecraft's `AABB`) (team/creative/creativecore/common/util/math/collision/CollidingPlane.java:89)
- [ ] `CollidingPlane.intersects(5)`: its class isn't translated (it uses Minecraft's `AABB`) (team/creative/creativecore/common/util/math/collision/CollidingPlane.java:130)
- [ ] `CollidingPlane.getDirection(3)`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/collision/CollidingPlane.java:167)

### `CollisionCoordinator`: a Map or Set of floating-point numbers

- [ ] `CollisionCoordinator.CollisionCoordinator(7)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:51)
- [ ] `CollisionCoordinator.hasOnlyTranslation(0)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:89)
- [ ] `CollisionCoordinator.getRotationDegree(1)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:93)
- [ ] `CollisionCoordinator.getRotationMatrix(1)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:102)
- [ ] `CollisionCoordinator.getRotationMatrixInv(1)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:129)
- [ ] `CollisionCoordinator.original(0)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:156)
- [ ] `CollisionCoordinator.moved(0)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:160)
- [ ] `CollisionCoordinator.computeSurroundingBox(1)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:164)
- [ ] `CollisionCoordinator.computeInverseSurroundingBoxInternal(1)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:168)
- [ ] `CollisionCoordinator.isSimple(0)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:172)
- [ ] `CollisionCoordinator.getNumberOfHalfRotations(0)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:186)
- [ ] `CollisionCoordinator.getInverted(1)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:203)
- [ ] `CollisionCoordinator.get(1)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:223)
- [ ] `CollisionCoordinator.transform(2)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:243)
- [ ] `CollisionCoordinator.transformInverted(2)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:256)
- [ ] `CollisionCoordinator.transform(2)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:269)
- [ ] `CollisionCoordinator.finish(0)`: its class isn't translated (a Map or Set of floating-point numbers) (team/creative/creativecore/common/util/math/collision/CollisionCoordinator.java:279)

### `IntersectionHelper`

- [ ] `IntersectionHelper.cutMinMax(5)`: it uses Vec2d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:15)
- [ ] `IntersectionHelper.cutMinMax(7)`: it uses Vec2f, which isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:119)
- [ ] `IntersectionHelper.add(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:225)
- [ ] `IntersectionHelper.iterateLines(15)`: it uses Edge, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:231)
- [ ] `IntersectionHelper.findIntersection(13)`: it uses Edge, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:388)
- [ ] `IntersectionHelper.addCornersBetween(8)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:436)
- [ ] `IntersectionHelper.add(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:453)
- [ ] `IntersectionHelper.iterateLines(13)`: it uses Edge, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:459)
- [ ] `IntersectionHelper.findIntersection(13)`: it uses Edge, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:612)
- [ ] `IntersectionHelper.addCornersBetween(8)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:660)

### `IntersectionHelper.Edge`: its constants carry values

- [ ] `IntersectionHelper.Edge.one(0)`: it has no body (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:770)
- [ ] `IntersectionHelper.Edge.positive(0)`: it has no body (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:772)
- [ ] `IntersectionHelper.Edge.clockwise(0)`: it has no body (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:774)
- [ ] `IntersectionHelper.Edge.counterClockwise(0)`: it has no body (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:776)

### `IntersectionHelper.EdgeStatus`: its constants carry values

- [ ] `IntersectionHelper.EdgeStatus.of(3)`: it uses EdgeStatus, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:816)
- [ ] `IntersectionHelper.EdgeStatus.of(3)`: it uses EdgeStatus, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:828)
- [ ] `IntersectionHelper.EdgeStatus.positive(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:840)
- [ ] `IntersectionHelper.EdgeStatus.edge(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:844)
- [ ] `IntersectionHelper.EdgeStatus.outside(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:848)

### `IntersectionHelper.InsideStatus`: it uses EdgeStatus, which isn't translated (its constants carry values)

- [ ] `IntersectionHelper.InsideStatus.CACHE`: it uses InsideStatus, which isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:854)
- [ ] `IntersectionHelper.InsideStatus.of(2)`: it uses InsideStatus, which isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:858)
- [ ] `IntersectionHelper.InsideStatus.get(6)`: it uses InsideStatus, which isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:862)
- [ ] `IntersectionHelper.InsideStatus.get(6)`: it uses InsideStatus, which isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:866)
- [ ] `IntersectionHelper.InsideStatus.InsideStatus(2)`: its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:870)
- [ ] `IntersectionHelper.InsideStatus.isInside(0)`: its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:875)
- [ ] `IntersectionHelper.InsideStatus.isInsideOrOnEdge(0)`: its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:879)
- [ ] `IntersectionHelper.InsideStatus.isOutside(0)`: its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:884)
- [ ] `IntersectionHelper.InsideStatus.oneEdge(0)`: its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:888)
- [ ] `IntersectionHelper.InsideStatus.twoEdge(0)`: its class isn't translated (it uses EdgeStatus, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/collision/IntersectionHelper.java:896)

### `MatrixUtils`

- [x] `MatrixUtils.createIdentityMatrix(0)` (team/creative/creativecore/common/util/math/collision/MatrixUtils.java:7)
- [ ] `MatrixUtils.createRotationMatrix(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/collision/MatrixUtils.java:13)
- [x] `MatrixUtils.mul(2)` (team/creative/creativecore/common/util/math/collision/MatrixUtils.java:20)
- [x] `MatrixUtils.createRotationMatrixAndTranslationRadians(6)` (team/creative/creativecore/common/util/math/collision/MatrixUtils.java:41)
- [x] `MatrixUtils.createRotationMatrixAndTranslation(6)` (team/creative/creativecore/common/util/math/collision/MatrixUtils.java:63)
- [x] `MatrixUtils.createRotationMatrixX(1)` (team/creative/creativecore/common/util/math/collision/MatrixUtils.java:85)
- [x] `MatrixUtils.createRotationMatrixXRadians(1)` (team/creative/creativecore/common/util/math/collision/MatrixUtils.java:91)
- [x] `MatrixUtils.createRotationMatrixY(1)` (team/creative/creativecore/common/util/math/collision/MatrixUtils.java:97)
- [x] `MatrixUtils.createRotationMatrixYRadians(1)` (team/creative/creativecore/common/util/math/collision/MatrixUtils.java:103)
- [x] `MatrixUtils.createRotationMatrixZ(1)` (team/creative/creativecore/common/util/math/collision/MatrixUtils.java:109)
- [x] `MatrixUtils.createRotationMatrixZRadians(1)` (team/creative/creativecore/common/util/math/collision/MatrixUtils.java:115)

### `PlaneCache`: it uses CollidingPlane, which isn't translated (it uses Minecraft's `AABB`)

- [ ] `PlaneCache.PlaneCache(2)`: its class isn't translated (it uses CollidingPlane, which isn't translated (it uses Minecraft's `AABB`)) (team/creative/creativecore/common/util/math/collision/PlaneCache.java:17)
- [ ] `PlaneCache.calculatePlanes(1)`: its class isn't translated (it uses CollidingPlane, which isn't translated (it uses Minecraft's `AABB`)) (team/creative/creativecore/common/util/math/collision/PlaneCache.java:27)
- [ ] `PlaneCache.getPushOutScale(3)`: its class isn't translated (it uses CollidingPlane, which isn't translated (it uses Minecraft's `AABB`)) (team/creative/creativecore/common/util/math/collision/PlaneCache.java:85)

### `NormalPlaneD`: it uses Vec3d, which isn't translated (it extends `VecNd`)

- [ ] `NormalPlaneD.NormalPlaneD(2)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:13)
- [ ] `NormalPlaneD.NormalPlaneD(2)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:19)
- [ ] `NormalPlaneD.NormalPlaneD(1)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:25)
- [ ] `NormalPlaneD.NormalPlaneD(3)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:31)
- [ ] `NormalPlaneD.isInvalid(0)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:38)
- [ ] `NormalPlaneD.isInFront(1)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:42)
- [ ] `NormalPlaneD.isInFront(2)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:46)
- [ ] `NormalPlaneD.isInFront(1)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:53)
- [ ] `NormalPlaneD.isInFront(2)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:57)
- [ ] `NormalPlaneD.cuts(1)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:64)
- [ ] `NormalPlaneD.intersect(2)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:90)
- [ ] `NormalPlaneD.intersect(1)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:106)
- [ ] `NormalPlaneD.project(5)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:118)
- [ ] `NormalPlaneD.toString(0)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:132)
- [ ] `NormalPlaneD.toFloat(0)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneD.java:137)

### `NormalPlaneF`: it uses Vec3f, which isn't translated (it extends `VecNf`)

- [ ] `NormalPlaneF.NormalPlaneF(2)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:13)
- [ ] `NormalPlaneF.NormalPlaneF(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:19)
- [ ] `NormalPlaneF.NormalPlaneF(3)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:25)
- [ ] `NormalPlaneF.isInvalid(0)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:32)
- [ ] `NormalPlaneF.isInFront(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:36)
- [ ] `NormalPlaneF.isInFront(2)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:40)
- [ ] `NormalPlaneF.cuts(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:47)
- [ ] `NormalPlaneF.intersect(2)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:73)
- [ ] `NormalPlaneF.intersect(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:89)
- [ ] `NormalPlaneF.project(5)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:101)
- [ ] `NormalPlaneF.toString(0)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:115)
- [ ] `NormalPlaneF.isFacing(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/NormalPlaneF.java:120)

### `Ray2d`: it uses Axis, which isn't translated (its constants carry values)

- [ ] `Ray2d.Ray2d(5)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:19)
- [ ] `Ray2d.Ray2d(6)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:28)
- [ ] `Ray2d.getOrigin(1)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:32)
- [ ] `Ray2d.getDirection(1)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:36)
- [ ] `Ray2d.getOther(1)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:40)
- [ ] `Ray2d.set(6)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:44)
- [ ] `Ray2d.set(4)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:53)
- [ ] `Ray2d.getT(2)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:62)
- [ ] `Ray2d.get(2)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:66)
- [ ] `Ray2d.get(1)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:71)
- [ ] `Ray2d.getFloat(1)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:75)
- [ ] `Ray2d.getWithLimits(2)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:79)
- [ ] `Ray2d.getWithLimits(4)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:83)
- [ ] `Ray2d.isCoordinateOnLine(2)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:89)
- [ ] `Ray2d.isCoordinateOnLine(2)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:93)
- [ ] `Ray2d.isCoordinateToTheRight(2)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:101)
- [ ] `Ray2d.isCoordinateToTheRight(2)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:107)
- [ ] `Ray2d.isParallel(1)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:114)
- [ ] `Ray2d.intersect(3)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:118)
- [ ] `Ray2d.intersectWhen(1)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:135)
- [ ] `Ray2d.intersect(2)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:148)
- [ ] `Ray2d.toString(0)`: its class isn't translated (it uses Axis, which isn't translated (its constants carry values)) (team/creative/creativecore/common/util/math/geo/Ray2d.java:161)

### `Ray3d`: it uses Vec3d, which isn't translated (it extends `VecNd`)

- [ ] `Ray3d.Ray3d(2)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/Ray3d.java:12)
- [ ] `Ray3d.Ray3d(3)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/Ray3d.java:19)
- [ ] `Ray3d.Ray3d(2)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/Ray3d.java:28)
- [ ] `Ray3d.set(6)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/Ray3d.java:34)
- [ ] `Ray3d.getT(2)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/Ray3d.java:40)
- [ ] `Ray3d.get(1)`: its class isn't translated (it uses Vec3d, which isn't translated (it extends `VecNd`)) (team/creative/creativecore/common/util/math/geo/Ray3d.java:44)

### `Ray3f`: it uses Vec3f, which isn't translated (it extends `VecNf`)

- [ ] `Ray3f.Ray3f(2)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/Ray3f.java:11)
- [ ] `Ray3f.Ray3f(2)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/Ray3f.java:18)
- [ ] `Ray3f.set(6)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/Ray3f.java:24)
- [ ] `Ray3f.getT(2)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/Ray3f.java:30)
- [ ] `Ray3f.get(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/Ray3f.java:34)

### `Rect`

- [x] `Rect.Rect(4)` (team/creative/creativecore/common/util/math/geo/Rect.java:12)
- [x] `Rect.intersects(1)` (team/creative/creativecore/common/util/math/geo/Rect.java:19)
- [x] `Rect.intersection(1)` (team/creative/creativecore/common/util/math/geo/Rect.java:23)
- [x] `Rect.move(2)` (team/creative/creativecore/common/util/math/geo/Rect.java:35)
- [x] `Rect.shrink(1)` (team/creative/creativecore/common/util/math/geo/Rect.java:42)
- [x] `Rect.grow(1)` (team/creative/creativecore/common/util/math/geo/Rect.java:49)
- [x] `Rect.getWidth(0)` (team/creative/creativecore/common/util/math/geo/Rect.java:56)
- [x] `Rect.getHeight(0)` (team/creative/creativecore/common/util/math/geo/Rect.java:60)
- [ ] `Rect.getSize(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/geo/Rect.java:64)
- [x] `Rect.copy(0)` (team/creative/creativecore/common/util/math/geo/Rect.java:72)
- [ ] `Rect.child(4)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/geo/Rect.java:76)
- [ ] `Rect.child(4)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/geo/Rect.java:80)
- [ ] `Rect.child(4)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/geo/Rect.java:89)
- [x] `Rect.inside(2)` (team/creative/creativecore/common/util/math/geo/Rect.java:98)
- [x] `Rect.toString(0)` (team/creative/creativecore/common/util/math/geo/Rect.java:102)
- [x] `Rect.scale(1)` (team/creative/creativecore/common/util/math/geo/Rect.java:107)

### `VectorFan`: it uses Vec3f, which isn't translated (it extends `VecNf`)

- [x] `VectorFan.EPSILON` (team/creative/creativecore/common/util/math/geo/VectorFan.java:15)
- [ ] `VectorFan.isInside(6)`: it uses NormalPlaneF, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:18)
- [ ] `VectorFan.isInside(3)`: it uses NormalPlaneF, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:42)
- [ ] `VectorFan.isPointBetween(3)`: it uses Vec3f, which isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/geo/VectorFan.java:52)
- [ ] `VectorFan.VectorFan(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:60)
- [ ] `VectorFan.getCoords(0)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:64)
- [ ] `VectorFan.get(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:68)
- [ ] `VectorFan.count(0)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:72)
- [ ] `VectorFan.cutMinMax(7)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:76)
- [ ] `VectorFan.doMinMaxLate(0)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:87)
- [ ] `VectorFan.doSimpleRendering(0)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:91)
- [ ] `VectorFan.add(2)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:95)
- [ ] `VectorFan.set(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:105)
- [ ] `VectorFan.set(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:109)
- [ ] `VectorFan.cutWithoutCopy(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:114)
- [ ] `VectorFan.cutWithoutCopy(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:125)
- [ ] `VectorFan.cutWithoutCopy(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:136)
- [ ] `VectorFan.cutWithoutCopy(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:140)
- [ ] `VectorFan.isEmpty(0)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:144)
- [ ] `VectorFan.cutInternal(2)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:148)
- [ ] `VectorFan.cut(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:242)
- [ ] `VectorFan.move(3)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:246)
- [ ] `VectorFan.scale(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:254)
- [ ] `VectorFan.divide(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:260)
- [ ] `VectorFan.intersects(2)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:264)
- [ ] `VectorFan.copy(0)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:292)
- [ ] `VectorFan.equals(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:302)
- [ ] `VectorFan.equals(4)`: it uses Vec3f, which isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/geo/VectorFan.java:330)
- [ ] `VectorFan.equalsIgnoreOrder(2)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:344)
- [ ] `VectorFan.calculateIntercept(4)`: it uses Ray3f, which isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:370)
- [ ] `VectorFan.calculateIntercept(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:401)
- [ ] `VectorFan.createNormal(0)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:415)
- [ ] `VectorFan.createPlane(0)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:425)
- [ ] `VectorFan.createPlane(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:429)
- [ ] `VectorFan.forAllEdges(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:461)
- [ ] `VectorFan.isInside(1)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:480)
- [ ] `VectorFan.intersect2d(5)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:531)
- [ ] `VectorFan.isInside2d(4)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:621)
- [ ] `VectorFan.cut2d(5)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:660)
- [ ] `VectorFan.cut2d(5)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:678)
- [ ] `VectorFan.cut2d(5)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:706)
- [ ] `VectorFan.combine(2)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:788)
- [ ] `VectorFan.checkConvexAndShrink(2)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/geo/VectorFan.java:838)
- [ ] `VectorFan.toString(0)`: its class isn't translated (it uses Vec3f, which isn't translated (it extends `VecNf`)) (team/creative/creativecore/common/util/math/geo/VectorFan.java:872)

### `VectorFan.ParallelException`: it extends `Exception`


### `CosineInterpolation`: it's generic

- [ ] `CosineInterpolation.CosineInterpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CosineInterpolation.java:7)
- [ ] `CosineInterpolation.CosineInterpolation(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CosineInterpolation.java:11)
- [ ] `CosineInterpolation.CosineInterpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CosineInterpolation.java:15)
- [ ] `CosineInterpolation.CosineInterpolation(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CosineInterpolation.java:19)
- [ ] `CosineInterpolation.estimateDistance(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CosineInterpolation.java:23)
- [ ] `CosineInterpolation.valueAt(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CosineInterpolation.java:36)

### `CubicInterpolation`: it's generic

- [ ] `CubicInterpolation.CubicInterpolation(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CubicInterpolation.java:10)
- [ ] `CubicInterpolation.CubicInterpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CubicInterpolation.java:20)
- [ ] `CubicInterpolation.CubicInterpolation(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CubicInterpolation.java:24)
- [ ] `CubicInterpolation.CubicInterpolation(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CubicInterpolation.java:43)
- [ ] `CubicInterpolation.CubicInterpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CubicInterpolation.java:62)
- [ ] `CubicInterpolation.getValue(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CubicInterpolation.java:72)
- [ ] `CubicInterpolation.estimateDistance(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CubicInterpolation.java:81)
- [ ] `CubicInterpolation.valueAt(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/CubicInterpolation.java:108)

### `HermiteInterpolation`: it's generic

- [ ] `HermiteInterpolation.HermiteInterpolation(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:10)
- [ ] `HermiteInterpolation.HermiteInterpolation(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:16)
- [ ] `HermiteInterpolation.HermiteInterpolation(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:20)
- [ ] `HermiteInterpolation.HermiteInterpolation(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:24)
- [ ] `HermiteInterpolation.HermiteInterpolation(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:30)
- [ ] `HermiteInterpolation.HermiteInterpolation(5)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:36)
- [ ] `HermiteInterpolation.HermiteInterpolation(6)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:42)
- [ ] `HermiteInterpolation.HermiteInterpolation(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:48)
- [ ] `HermiteInterpolation.HermiteInterpolation(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:52)
- [ ] `HermiteInterpolation.HermiteInterpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:56)
- [ ] `HermiteInterpolation.HermiteInterpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:60)
- [ ] `HermiteInterpolation.HermiteInterpolation(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:64)
- [ ] `HermiteInterpolation.HermiteInterpolation(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:68)
- [ ] `HermiteInterpolation.valueAt(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:72)

### `HermiteInterpolation.Tension`: its constants carry values

- [ ] `HermiteInterpolation.Tension.Tension(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/interpolation/HermiteInterpolation.java:98)

### `Interpolation`: it's generic

- [ ] `Interpolation.Interpolation(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/Interpolation.java:13)
- [ ] `Interpolation.Interpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/Interpolation.java:29)
- [ ] `Interpolation.Interpolation(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/Interpolation.java:38)
- [ ] `Interpolation.Interpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/Interpolation.java:54)
- [ ] `Interpolation.Interpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/Interpolation.java:69)
- [ ] `Interpolation.getValue(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/Interpolation.java:84)
- [ ] `Interpolation.valueAt(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/Interpolation.java:88)
- [ ] `Interpolation.createVec(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/Interpolation.java:126)
- [ ] `Interpolation.valueAt(4)`: it has no body (team/creative/creativecore/common/util/math/interpolation/Interpolation.java:130)
- [ ] `Interpolation.estimateDistance(0)`: it has no body (team/creative/creativecore/common/util/math/interpolation/Interpolation.java:132)

### `LinearInterpolation`: it's generic

- [ ] `LinearInterpolation.LinearInterpolation(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/LinearInterpolation.java:8)
- [ ] `LinearInterpolation.LinearInterpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/LinearInterpolation.java:12)
- [ ] `LinearInterpolation.LinearInterpolation(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/LinearInterpolation.java:16)
- [ ] `LinearInterpolation.LinearInterpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/LinearInterpolation.java:20)
- [ ] `LinearInterpolation.LinearInterpolation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/LinearInterpolation.java:24)
- [ ] `LinearInterpolation.estimateDistance(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/LinearInterpolation.java:28)
- [ ] `LinearInterpolation.valueAt(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/interpolation/LinearInterpolation.java:41)

### `Maths`

- [x] `Maths.EPSILON` (team/creative/creativecore/common/util/math/Maths.java:6)
- [x] `Maths.EPSILON_UP` (team/creative/creativecore/common/util/math/Maths.java:7)
- [x] `Maths.EPSILON_DOUBLE` (team/creative/creativecore/common/util/math/Maths.java:8)
- [x] `Maths.EPSILON_UP_DOUBLE` (team/creative/creativecore/common/util/math/Maths.java:9)
- [x] `Maths.EPSILON_PRECISE` (team/creative/creativecore/common/util/math/Maths.java:10)
- [x] `Maths.EPSILON_PRECISE_UP` (team/creative/creativecore/common/util/math/Maths.java:11)
- [x] `Maths.EPSILON_PRECISE_DOUBLE` (team/creative/creativecore/common/util/math/Maths.java:12)
- [x] `Maths.EPSILON_PRECISE_UP_DOUBLE` (team/creative/creativecore/common/util/math/Maths.java:13)
- [ ] `Maths.within(3)`: it calls Maths.within, which isn't translated (team/creative/creativecore/common/util/math/Maths.java:15)
- [ ] `Maths.within(4)`: it calls Maths.greaterThanOrEquals, which isn't translated (team/creative/creativecore/common/util/math/Maths.java:19)
- [ ] `Maths.smallerThanOrEquals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:23)
- [ ] `Maths.smallerThanOrEquals(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:27)
- [ ] `Maths.greaterThanOrEquals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:31)
- [ ] `Maths.greaterThanOrEquals(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:35)
- [ ] `Maths.equals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:39)
- [ ] `Maths.equals(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:43)
- [ ] `Maths.equals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:47)
- [ ] `Maths.equals(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:51)
- [x] `Maths.tickToMs(1)` (team/creative/creativecore/common/util/math/Maths.java:55)
- [x] `Maths.msToTick(1)` (team/creative/creativecore/common/util/math/Maths.java:59)
- [ ] `Maths.round(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:63)
- [ ] `Maths.round(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:67)
- [x] `Maths.safeDivide(2)` (team/creative/creativecore/common/util/math/Maths.java:71)
- [x] `Maths.safePercent(2)` (team/creative/creativecore/common/util/math/Maths.java:75)
- [x] `Maths.safeRound(1)` (team/creative/creativecore/common/util/math/Maths.java:79)
- [ ] `Maths.min(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:83)
- [ ] `Maths.min(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:87)
- [ ] `Maths.min(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:91)
- [ ] `Maths.min(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:95)
- [ ] `Maths.max(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:99)
- [ ] `Maths.max(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:103)
- [ ] `Maths.max(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:107)
- [ ] `Maths.max(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/Maths.java:111)

### `ChildVecOrigin`: it extends `VecOrigin`

- [ ] `ChildVecOrigin.ChildVecOrigin(2)`: its class isn't translated (it extends `VecOrigin`) (team/creative/creativecore/common/util/math/matrix/ChildVecOrigin.java:9)
- [ ] `ChildVecOrigin.onlyRotateWithoutCenter(1)`: its class isn't translated (it extends `VecOrigin`) (team/creative/creativecore/common/util/math/matrix/ChildVecOrigin.java:14)
- [ ] `ChildVecOrigin.transformPointToWorld(1)`: its class isn't translated (it extends `VecOrigin`) (team/creative/creativecore/common/util/math/matrix/ChildVecOrigin.java:20)
- [ ] `ChildVecOrigin.transformPointToFakeWorld(1)`: its class isn't translated (it extends `VecOrigin`) (team/creative/creativecore/common/util/math/matrix/ChildVecOrigin.java:26)
- [ ] `ChildVecOrigin.translationCombined(1)`: its class isn't translated (it extends `VecOrigin`) (team/creative/creativecore/common/util/math/matrix/ChildVecOrigin.java:32)
- [ ] `ChildVecOrigin.hasChanged(0)`: its class isn't translated (it extends `VecOrigin`) (team/creative/creativecore/common/util/math/matrix/ChildVecOrigin.java:37)
- [ ] `ChildVecOrigin.getParent(0)`: its class isn't translated (it extends `VecOrigin`) (team/creative/creativecore/common/util/math/matrix/ChildVecOrigin.java:42)
- [ ] `ChildVecOrigin.createInternalCopy(0)`: its class isn't translated (it extends `VecOrigin`) (team/creative/creativecore/common/util/math/matrix/ChildVecOrigin.java:47)

### `IntMatrix3`: it implements `IntMatrix3c`

- [x] `IntMatrix3.fma(3)` (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:20)
- [ ] `IntMatrix3.IntMatrix3(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:24)
- [ ] `IntMatrix3.IntMatrix3(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:28)
- [ ] `IntMatrix3.IntMatrix3(9)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:44)
- [ ] `IntMatrix3.IntMatrix3(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:56)
- [ ] `IntMatrix3.IntMatrix3(2)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:68)
- [ ] `IntMatrix3.getX(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:80)
- [ ] `IntMatrix3.getX(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:85)
- [ ] `IntMatrix3.getX(3)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:90)
- [ ] `IntMatrix3.getX(3)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:95)
- [ ] `IntMatrix3.getY(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:100)
- [ ] `IntMatrix3.getY(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:105)
- [ ] `IntMatrix3.getY(3)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:110)
- [ ] `IntMatrix3.getY(3)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:115)
- [ ] `IntMatrix3.getZ(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:120)
- [ ] `IntMatrix3.getZ(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:125)
- [ ] `IntMatrix3.getZ(3)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:130)
- [ ] `IntMatrix3.getZ(3)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:135)
- [ ] `IntMatrix3.transform(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:140)
- [ ] `IntMatrix3.transform(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:148)
- [ ] `IntMatrix3.transform(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:156)
- [ ] `IntMatrix3.transform(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:164)
- [ ] `IntMatrix3.transform(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:172)
- [ ] `IntMatrix3.mul(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:180)
- [ ] `IntMatrix3.mul(2)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:184)
- [ ] `IntMatrix3.equals(1)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:206)
- [ ] `IntMatrix3.toString(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:221)
- [ ] `IntMatrix3.m00(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:243)
- [ ] `IntMatrix3.m01(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:248)
- [ ] `IntMatrix3.m02(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:253)
- [ ] `IntMatrix3.m10(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:258)
- [ ] `IntMatrix3.m11(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:263)
- [ ] `IntMatrix3.m12(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:268)
- [ ] `IntMatrix3.m20(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:273)
- [ ] `IntMatrix3.m21(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:278)
- [ ] `IntMatrix3.m22(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:283)
- [ ] `IntMatrix3.invertedX(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:288)
- [ ] `IntMatrix3.getX(3)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:293)
- [ ] `IntMatrix3.invertedY(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:298)
- [ ] `IntMatrix3.getY(3)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:303)
- [ ] `IntMatrix3.invertedZ(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:308)
- [ ] `IntMatrix3.getZ(3)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:313)
- [ ] `IntMatrix3.getAsArray(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:318)
- [ ] `IntMatrix3.isIdentity(0)`: its class isn't translated (it implements `IntMatrix3c`) (team/creative/creativecore/common/util/math/matrix/IntMatrix3.java:323)

### `IntMatrix3c`: it's an interface

- [ ] `IntMatrix3c.m00(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:12)
- [ ] `IntMatrix3c.m01(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:14)
- [ ] `IntMatrix3c.m02(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:16)
- [ ] `IntMatrix3c.m10(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:18)
- [ ] `IntMatrix3c.m11(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:20)
- [ ] `IntMatrix3c.m12(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:22)
- [ ] `IntMatrix3c.m20(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:24)
- [ ] `IntMatrix3c.m21(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:26)
- [ ] `IntMatrix3c.m22(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:28)
- [ ] `IntMatrix3c.getX(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:30)
- [ ] `IntMatrix3c.getX(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:32)
- [ ] `IntMatrix3c.getX(3)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:34)
- [ ] `IntMatrix3c.getX(3)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:36)
- [ ] `IntMatrix3c.getY(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:38)
- [ ] `IntMatrix3c.getY(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:40)
- [ ] `IntMatrix3c.getY(3)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:42)
- [ ] `IntMatrix3c.getY(3)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:44)
- [ ] `IntMatrix3c.getZ(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:46)
- [ ] `IntMatrix3c.getZ(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:48)
- [ ] `IntMatrix3c.getZ(3)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:50)
- [ ] `IntMatrix3c.getZ(3)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:52)
- [ ] `IntMatrix3c.transform(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:54)
- [ ] `IntMatrix3c.transform(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:56)
- [ ] `IntMatrix3c.transform(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:58)
- [ ] `IntMatrix3c.transform(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:60)
- [ ] `IntMatrix3c.transform(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:62)
- [ ] `IntMatrix3c.invertedX(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:64)
- [ ] `IntMatrix3c.getX(3)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:66)
- [ ] `IntMatrix3c.invertedY(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:68)
- [ ] `IntMatrix3c.getY(3)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:70)
- [ ] `IntMatrix3c.invertedZ(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:72)
- [ ] `IntMatrix3c.getZ(3)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:74)
- [ ] `IntMatrix3c.getAsArray(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:76)
- [ ] `IntMatrix3c.isIdentity(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IntMatrix3c.java:78)

### `IVecOrigin`: it's an interface

- [ ] `IVecOrigin.offX(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:13)
- [ ] `IVecOrigin.offY(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:15)
- [ ] `IVecOrigin.offZ(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:17)
- [ ] `IVecOrigin.rotX(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:19)
- [ ] `IVecOrigin.rotY(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:21)
- [ ] `IVecOrigin.rotZ(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:23)
- [ ] `IVecOrigin.offXLast(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:25)
- [ ] `IVecOrigin.offYLast(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:27)
- [ ] `IVecOrigin.offZLast(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:29)
- [ ] `IVecOrigin.rotXLast(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:31)
- [ ] `IVecOrigin.rotYLast(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:33)
- [ ] `IVecOrigin.rotZLast(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:35)
- [ ] `IVecOrigin.isRotated(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:37)
- [ ] `IVecOrigin.offX(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:39)
- [ ] `IVecOrigin.offY(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:41)
- [ ] `IVecOrigin.offZ(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:43)
- [ ] `IVecOrigin.off(3)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:45)
- [ ] `IVecOrigin.rotX(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:47)
- [ ] `IVecOrigin.rotY(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:49)
- [ ] `IVecOrigin.rotZ(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:51)
- [ ] `IVecOrigin.rot(3)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:53)
- [ ] `IVecOrigin.center(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:55)
- [ ] `IVecOrigin.setCenter(1)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:57)
- [ ] `IVecOrigin.rotation(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:59)
- [ ] `IVecOrigin.rotationInv(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:61)
- [ ] `IVecOrigin.translation(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:63)
- [ ] `IVecOrigin.tick(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:65)
- [ ] `IVecOrigin.getParent(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:67)
- [ ] `IVecOrigin.set(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:69)
- [ ] `IVecOrigin.translationCombined(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:77)
- [ ] `IVecOrigin.onlyRotateWithoutCenter(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:81)
- [ ] `IVecOrigin.transformPointToWorld(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:85)
- [ ] `IVecOrigin.transformPointToFakeWorld(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:91)
- [ ] `IVecOrigin.transformPointToWorld(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:97)
- [ ] `IVecOrigin.transformPointToFakeWorld(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:104)
- [ ] `IVecOrigin.transformPointToWorld(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:111)
- [ ] `IVecOrigin.transformPointToFakeWorld(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:117)
- [ ] `IVecOrigin.transformPointToWorld(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:123)
- [ ] `IVecOrigin.transformPointToFakeWorld(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:129)
- [ ] `IVecOrigin.getAABB(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:135)
- [ ] `IVecOrigin.getOBB(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:153)
- [ ] `IVecOrigin.getAABB(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:171)
- [ ] `IVecOrigin.getOBB(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:189)
- [ ] `IVecOrigin.hasChanged(0)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:207)
- [ ] `IVecOrigin.copy(0)`: it has no body (team/creative/creativecore/common/util/math/matrix/IVecOrigin.java:216)

### `Matrix3`

- [x] `Matrix3.EPS` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:16)
- [x] `Matrix3.Matrix3(0)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:18)
- [x] `Matrix3.Matrix3(9)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:22)
- [ ] `Matrix3.Matrix3(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix3.java:34)
- [x] `Matrix3.toString(0)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:38)
- [x] `Matrix3.setIdentity(0)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:60)
- [ ] `Matrix3.setScale(1)`: it calls Matrix3.getScaleRotate, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix3.java:72)
- [ ] `Matrix3.setElement(3)`: a switch case that falls through (team/creative/creativecore/common/util/math/matrix/Matrix3.java:87)
- [ ] `Matrix3.getElement(2)`: a switch case that falls through (team/creative/creativecore/common/util/math/matrix/Matrix3.java:136)
- [ ] `Matrix3.getScale(0)`: it calls Matrix3.getScaleRotate, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix3.java:174)
- [ ] `Matrix3.add(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix3.java:181)
- [ ] `Matrix3.add(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix3.java:193)
- [x] `Matrix3.sub(1)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:205)
- [x] `Matrix3.transpose(0)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:217)
- [ ] `Matrix3.set(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix3.java:229)
- [ ] `Matrix3.invert(0)`: it calls Matrix3.luDecomposition, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix3.java:241)
- [ ] `Matrix3.invert(1)`: it calls Matrix3.luDecomposition, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix3.java:268)
- [ ] `Matrix3.luDecomposition(2)`: `--` inside an expression (team/creative/creativecore/common/util/math/matrix/Matrix3.java:295)
- [x] `Matrix3.luBacksubstitution(3)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:395)
- [x] `Matrix3.determinant(0)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:428)
- [ ] `Matrix3.set(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix3.java:434)
- [x] `Matrix3.rotX(1)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:446)
- [x] `Matrix3.rotY(1)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:460)
- [x] `Matrix3.rotZ(1)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:474)
- [ ] `Matrix3.mul(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix3.java:488)
- [ ] `Matrix3.mul(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix3.java:500)
- [ ] `Matrix3.mulNormalize(1)`: it calls Matrix3.compute_svd, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix3.java:521)
- [ ] `Matrix3.normalize(0)`: it calls Matrix3.getScaleRotate, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix3.java:546)
- [x] `Matrix3.normalizeCP(0)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:561)
- [ ] `Matrix3.equals(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix3.java:575)
- [ ] `Matrix3.equals(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix3.java:591)
- [ ] `Matrix3.transform(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix3.java:609)
- [ ] `Matrix3.transform(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix3.java:616)
- [ ] `Matrix3.getScaleRotate(2)`: it calls Matrix3.compute_svd, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix3.java:623)
- [ ] `Matrix3.compute_svd(3)`: it changes an array or object through a second name (tmp) (team/creative/creativecore/common/util/math/matrix/Matrix3.java:628)
- [x] `Matrix3.svdReorder(6)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:876)
- [x] `Matrix3.compute_qr(4)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1008)
- [x] `Matrix3.max(2)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1145)
- [x] `Matrix3.min(2)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1149)
- [x] `Matrix3.d_sign(2)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1153)
- [x] `Matrix3.compute_shift(3)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1158)
- [x] `Matrix3.compute_2X2(9)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1195)
- [x] `Matrix3.compute_rot(6)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1362)
- [x] `Matrix3.print_mat(1)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1432)
- [x] `Matrix3.print_det(1)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1438)
- [x] `Matrix3.mat_mul(3)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1448)
- [x] `Matrix3.transpose_mat(2)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1466)
- [x] `Matrix3.max3(1)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1478)
- [x] `Matrix3.almostEqual(2)` (team/creative/creativecore/common/util/math/matrix/Matrix3.java:1486)

### `Matrix4`

- [x] `Matrix4.Matrix4(16)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:25)
- [ ] `Matrix4.Matrix4(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:61)
- [ ] `Matrix4.Matrix4(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:80)
- [ ] `Matrix4.Matrix4(3)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/matrix/Matrix4.java:99)
- [x] `Matrix4.Matrix4(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:118)
- [x] `Matrix4.toString(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:137)
- [x] `Matrix4.setIdentity(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:173)
- [ ] `Matrix4.setElement(3)`: a switch case that falls through (team/creative/creativecore/common/util/math/matrix/Matrix4.java:192)
- [ ] `Matrix4.getElement(2)`: a switch case that falls through (team/creative/creativecore/common/util/math/matrix/Matrix4.java:267)
- [ ] `Matrix4.getRow(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:309)
- [ ] `Matrix4.getRow(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:337)
- [ ] `Matrix4.getColumn(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:365)
- [ ] `Matrix4.getColumn(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:393)
- [ ] `Matrix4.get(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:421)
- [ ] `Matrix4.get(2)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/matrix/Matrix4.java:436)
- [ ] `Matrix4.get(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:455)
- [x] `Matrix4.getRotationScale(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:461)
- [ ] `Matrix4.getScale(0)`: it calls Matrix4.getScaleRotate, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix4.java:473)
- [x] `Matrix4.setRotationScale(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:480)
- [ ] `Matrix4.setScale(1)`: it calls Matrix4.getScaleRotate, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix4.java:492)
- [x] `Matrix4.setRow(5)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:507)
- [ ] `Matrix4.setRow(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:538)
- [ ] `Matrix4.setRow(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:569)
- [x] `Matrix4.setColumn(5)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:600)
- [ ] `Matrix4.setColumn(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:631)
- [ ] `Matrix4.setColumn(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:662)
- [ ] `Matrix4.add(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:693)
- [ ] `Matrix4.add(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:712)
- [ ] `Matrix4.add(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:731)
- [ ] `Matrix4.add(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:750)
- [x] `Matrix4.sub(2)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:769)
- [x] `Matrix4.sub(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:788)
- [x] `Matrix4.transpose(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:807)
- [ ] `Matrix4.transpose(1)`: `this` used as a value (team/creative/creativecore/common/util/math/matrix/Matrix4.java:828)
- [ ] `Matrix4.set(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:851)
- [ ] `Matrix4.set(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:870)
- [ ] `Matrix4.set(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:889)
- [ ] `Matrix4.invert(1)`: it calls Matrix4.invertGeneral, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix4.java:908)
- [ ] `Matrix4.invert(0)`: `this` used as a value (team/creative/creativecore/common/util/math/matrix/Matrix4.java:912)
- [ ] `Matrix4.invertGeneral(1)`: it calls Matrix4.luDecomposition, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix4.java:916)
- [ ] `Matrix4.luDecomposition(2)`: `--` inside an expression (team/creative/creativecore/common/util/math/matrix/Matrix4.java:968)
- [x] `Matrix4.luBacksubstitution(3)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1068)
- [x] `Matrix4.determinant(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1103)
- [ ] `Matrix4.set(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1143)
- [ ] `Matrix4.set(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1162)
- [ ] `Matrix4.set(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1181)
- [ ] `Matrix4.set(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1200)
- [ ] `Matrix4.set(3)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1219)
- [ ] `Matrix4.setTranslation(1)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1238)
- [x] `Matrix4.rotX(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1244)
- [x] `Matrix4.rotY(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1265)
- [x] `Matrix4.rotZ(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1286)
- [ ] `Matrix4.mul(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1307)
- [ ] `Matrix4.mul(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1326)
- [ ] `Matrix4.mul(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1345)
- [ ] `Matrix4.mul(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1380)
- [ ] `Matrix4.mulTransposeBoth(2)`: `this` used as a value (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1434)
- [ ] `Matrix4.mulTransposeRight(2)`: `this` used as a value (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1488)
- [ ] `Matrix4.mulTransposeLeft(2)`: `this` used as a value (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1542)
- [ ] `Matrix4.equals(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1596)
- [ ] `Matrix4.equals(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1619)
- [ ] `Matrix4.epsilonEquals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1644)
- [ ] `Matrix4.epsilonEquals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1648)
- [ ] `Matrix4.hashCode(0)`: it calls `Double`.doubleToLongBits (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1728)
- [ ] `Matrix4.transform(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1750)
- [ ] `Matrix4.transform(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1760)
- [ ] `Matrix4.transform(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1770)
- [ ] `Matrix4.transform(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1778)
- [ ] `Matrix4.transform(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1786)
- [ ] `Matrix4.transform(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1794)
- [ ] `Matrix4.setRotation(1)`: it calls Matrix4.getScaleRotate, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1802)
- [x] `Matrix4.setZero(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1817)
- [x] `Matrix4.negate(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1836)
- [x] `Matrix4.negate(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1855)
- [ ] `Matrix4.getScaleRotate(2)`: it calls Matrix3.compute_svd, which isn't translated (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1874)
- [ ] `Matrix4.clone(0)`: it uses `Object` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1879)
- [x] `Matrix4.getM00(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1890)
- [x] `Matrix4.setM00(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1894)
- [x] `Matrix4.getM01(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1898)
- [x] `Matrix4.setM01(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1902)
- [x] `Matrix4.getM02(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1906)
- [x] `Matrix4.setM02(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1910)
- [x] `Matrix4.getM10(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1914)
- [x] `Matrix4.setM10(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1918)
- [x] `Matrix4.getM11(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1922)
- [x] `Matrix4.setM11(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1926)
- [x] `Matrix4.getM12(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1930)
- [x] `Matrix4.setM12(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1934)
- [x] `Matrix4.getM20(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1938)
- [x] `Matrix4.setM20(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1942)
- [x] `Matrix4.getM21(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1946)
- [x] `Matrix4.setM21(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1950)
- [x] `Matrix4.getM22(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1954)
- [x] `Matrix4.setM22(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1958)
- [x] `Matrix4.getM03(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1962)
- [x] `Matrix4.setM03(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1966)
- [x] `Matrix4.getM13(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1970)
- [x] `Matrix4.setM13(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1974)
- [x] `Matrix4.getM23(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1978)
- [x] `Matrix4.setM23(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1982)
- [x] `Matrix4.getM30(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1986)
- [x] `Matrix4.setM30(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1990)
- [x] `Matrix4.getM31(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1994)
- [x] `Matrix4.setM31(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:1998)
- [x] `Matrix4.getM32(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:2002)
- [x] `Matrix4.setM32(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:2006)
- [x] `Matrix4.getM33(0)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:2010)
- [x] `Matrix4.setM33(1)` (team/creative/creativecore/common/util/math/matrix/Matrix4.java:2014)

### `VecOrigin`: it implements `IVecOrigin`

- [ ] `VecOrigin.VecOrigin(1)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:25)
- [ ] `VecOrigin.offX(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:29)
- [ ] `VecOrigin.offY(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:34)
- [ ] `VecOrigin.offZ(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:39)
- [ ] `VecOrigin.rotX(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:44)
- [ ] `VecOrigin.rotY(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:49)
- [ ] `VecOrigin.rotZ(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:54)
- [ ] `VecOrigin.isRotated(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:59)
- [ ] `VecOrigin.updateRotated(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:64)
- [ ] `VecOrigin.updateTranslation(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:70)
- [ ] `VecOrigin.offX(1)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:74)
- [ ] `VecOrigin.offY(1)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:80)
- [ ] `VecOrigin.offZ(1)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:86)
- [ ] `VecOrigin.off(3)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:92)
- [ ] `VecOrigin.rotX(1)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:100)
- [ ] `VecOrigin.rotY(1)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:106)
- [ ] `VecOrigin.rotZ(1)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:112)
- [ ] `VecOrigin.rot(3)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:118)
- [ ] `VecOrigin.center(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:126)
- [ ] `VecOrigin.rotation(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:131)
- [ ] `VecOrigin.rotationInv(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:136)
- [ ] `VecOrigin.translation(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:141)
- [ ] `VecOrigin.setCenter(1)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:146)
- [ ] `VecOrigin.offXLast(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:151)
- [ ] `VecOrigin.offYLast(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:156)
- [ ] `VecOrigin.offZLast(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:161)
- [ ] `VecOrigin.rotXLast(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:166)
- [ ] `VecOrigin.rotYLast(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:171)
- [ ] `VecOrigin.rotZLast(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:176)
- [ ] `VecOrigin.tick(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:181)
- [ ] `VecOrigin.createInternalCopy(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:191)
- [ ] `VecOrigin.copy(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:195)
- [ ] `VecOrigin.getParent(0)`: its class isn't translated (it implements `IVecOrigin`) (team/creative/creativecore/common/util/math/matrix/VecOrigin.java:217)

### `TimeMath`

- [ ] `TimeMath.FORMAT`: it uses `DateFormat` (java.text) (team/creative/creativecore/common/util/math/TimeMath.java:9)
- [ ] `TimeMath.timestamp(1)`: it calls `MILLISECONDS`.toMinutes (team/creative/creativecore/common/util/math/TimeMath.java:11)

### `BooleanRotation`: its constants carry values

- [ ] `BooleanRotation.ROTATIONS`: it uses BooleanRotation, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java:21)
- [ ] `BooleanRotation.BooleanRotation(4)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java:27)
- [ ] `BooleanRotation.one(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java:34)
- [ ] `BooleanRotation.two(1)`: it uses Axis, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java:42)
- [ ] `BooleanRotation.clockwise(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java:50)
- [ ] `BooleanRotation.counterClockwise(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java:54)
- [ ] `BooleanRotation.clockwiseMaxFacing(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java:58)
- [ ] `BooleanRotation.counterMaxClockwiseFacing(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java:62)
- [ ] `BooleanRotation.getFacingInBetween(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java:66)
- [ ] `BooleanRotation.is(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java:76)
- [ ] `BooleanRotation.get(2)`: it uses BooleanRotation, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/BooleanRotation.java:80)

### `Mirror`

- [ ] `Mirror.mirror(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/transformation/Mirror.java:11)
- [ ] `Mirror.mirror(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/transformation/Mirror.java:15)
- [ ] `Mirror.mirror(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/transformation/Mirror.java:24)
- [ ] `Mirror.mirror(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/transformation/Mirror.java:33)
- [ ] `Mirror.mirror(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/transformation/Mirror.java:46)

### `Rotation`: its constants carry values

- [ ] `Rotation.FACING_ROTATION`: it uses Facing, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:52)
- [ ] `Rotation.getRotation(2)`: it uses Rotation, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:59)
- [ ] `Rotation.ofNormal(1)`: it uses Rotation, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:67)
- [ ] `Rotation.getRotation(1)`: it uses Rotation, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:83)
- [ ] `Rotation.getRotationCount(1)`: it uses `Rotation` (team/creative/creativecore/common/util/math/transformation/Rotation.java:92)
- [ ] `Rotation.Rotation(3)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:96)
- [ ] `Rotation.getMatrix(0)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:104)
- [ ] `Rotation.opposite(0)`: it has no body (team/creative/creativecore/common/util/math/transformation/Rotation.java:108)
- [ ] `Rotation.rotate(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:110)
- [ ] `Rotation.rotate(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:114)
- [ ] `Rotation.mirror(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:126)
- [ ] `Rotation.rotate(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:130)
- [ ] `Rotation.transform(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:134)
- [ ] `Rotation.transform(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:138)
- [ ] `Rotation.transform(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:142)
- [ ] `Rotation.transform(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:146)
- [ ] `Rotation.transform(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:150)
- [ ] `Rotation.transform(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/math/transformation/Rotation.java:154)

### `BooleanUtils`

- [x] `BooleanUtils.toInt(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:6)
- [x] `BooleanUtils.asArray(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:10)
- [x] `BooleanUtils.isTrue(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:14)
- [x] `BooleanUtils.isFalse(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:18)
- [x] `BooleanUtils.explicitOneTrue(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:22)
- [x] `BooleanUtils.explicitOneFalse(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:38)
- [x] `BooleanUtils.countTrue(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:54)
- [x] `BooleanUtils.set(2)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:66)
- [x] `BooleanUtils.reset(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:76)
- [x] `BooleanUtils.any(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:80)
- [x] `BooleanUtils.or(2)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:90)
- [x] `BooleanUtils.equals(2)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:96)
- [x] `BooleanUtils.boolToInt(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:106)
- [x] `BooleanUtils.intToBool(2)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:120)
- [ ] `BooleanUtils.print(1)`: it calls `Arrays` (java.util).toString (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:130)
- [x] `BooleanUtils.getRequiredBandwidth(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:134)
- [x] `BooleanUtils.toBits(2)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:145)
- [x] `BooleanUtils.toNumber(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:155)
- [x] `BooleanUtils.copy(1)` (team/creative/creativecore/common/util/math/utils/BooleanUtils.java:165)

### `IntegerUtils`

- [x] `IntegerUtils.bitIs(2)` (team/creative/creativecore/common/util/math/utils/IntegerUtils.java:4)
- [x] `IntegerUtils.getMask(1)` (team/creative/creativecore/common/util/math/utils/IntegerUtils.java:8)
- [x] `IntegerUtils.set(3)` (team/creative/creativecore/common/util/math/utils/IntegerUtils.java:12)
- [x] `IntegerUtils.unset(2)` (team/creative/creativecore/common/util/math/utils/IntegerUtils.java:16)
- [x] `IntegerUtils.set(2)` (team/creative/creativecore/common/util/math/utils/IntegerUtils.java:20)
- [x] `IntegerUtils.print(1)` (team/creative/creativecore/common/util/math/utils/IntegerUtils.java:24)

### `RangedBitSet`: it uses `BitSet` (java.util)

- [ ] `RangedBitSet.RangedBitSet(2)`: its class isn't translated (it uses `BitSet` (java.util)) (team/creative/creativecore/common/util/math/vec/RangedBitSet.java:12)
- [ ] `RangedBitSet.getMin(0)`: its class isn't translated (it uses `BitSet` (java.util)) (team/creative/creativecore/common/util/math/vec/RangedBitSet.java:22)
- [ ] `RangedBitSet.getMax(0)`: its class isn't translated (it uses `BitSet` (java.util)) (team/creative/creativecore/common/util/math/vec/RangedBitSet.java:26)
- [ ] `RangedBitSet.add(1)`: its class isn't translated (it uses `BitSet` (java.util)) (team/creative/creativecore/common/util/math/vec/RangedBitSet.java:30)
- [ ] `RangedBitSet.getRanges(0)`: its class isn't translated (it uses `BitSet` (java.util)) (team/creative/creativecore/common/util/math/vec/RangedBitSet.java:36)

### `RangedBitSet.BitRange`

- [x] `RangedBitSet.BitRange.BitRange(2)` (team/creative/creativecore/common/util/math/vec/RangedBitSet.java:53)
- [x] `RangedBitSet.BitRange.min(0)` (team/creative/creativecore/common/util/math/vec/RangedBitSet.java:53)
- [x] `RangedBitSet.BitRange.max(0)` (team/creative/creativecore/common/util/math/vec/RangedBitSet.java:53)

### `SmoothValue`

- [x] `SmoothValue.SmoothValue(2)` (team/creative/creativecore/common/util/math/vec/SmoothValue.java:10)
- [x] `SmoothValue.SmoothValue(1)` (team/creative/creativecore/common/util/math/vec/SmoothValue.java:15)
- [x] `SmoothValue.setStart(1)` (team/creative/creativecore/common/util/math/vec/SmoothValue.java:19)
- [x] `SmoothValue.add(1)` (team/creative/creativecore/common/util/math/vec/SmoothValue.java:26)
- [x] `SmoothValue.set(1)` (team/creative/creativecore/common/util/math/vec/SmoothValue.java:30)
- [x] `SmoothValue.tick(0)` (team/creative/creativecore/common/util/math/vec/SmoothValue.java:36)
- [x] `SmoothValue.current(0)` (team/creative/creativecore/common/util/math/vec/SmoothValue.java:48)
- [x] `SmoothValue.aimed(0)` (team/creative/creativecore/common/util/math/vec/SmoothValue.java:52)

### `Vec1d`: it extends `VecNd`

- [ ] `Vec1d.Vec1d(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:6)
- [ ] `Vec1d.Vec1d(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:9)
- [ ] `Vec1d.Vec1d(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:13)
- [ ] `Vec1d.set(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:17)
- [ ] `Vec1d.get(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:21)
- [ ] `Vec1d.set(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:26)
- [ ] `Vec1d.dimensions(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:33)
- [ ] `Vec1d.copy(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:38)
- [ ] `Vec1d.add(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:42)
- [ ] `Vec1d.sub(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:46)
- [ ] `Vec1d.scale(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:50)
- [ ] `Vec1d.equals(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:55)
- [ ] `Vec1d.epsilonEquals(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:60)
- [ ] `Vec1d.distance(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:65)
- [ ] `Vec1d.distanceSqr(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:69)
- [ ] `Vec1d.length(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:74)
- [ ] `Vec1d.lengthSquared(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:79)
- [ ] `Vec1d.angle(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:84)
- [ ] `Vec1d.dot(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec1d.java:88)

### `Vec1f`: it extends `VecNf`

- [ ] `Vec1f.Vec1f(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:6)
- [ ] `Vec1f.Vec1f(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:9)
- [ ] `Vec1f.Vec1f(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:13)
- [ ] `Vec1f.set(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:17)
- [ ] `Vec1f.get(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:21)
- [ ] `Vec1f.set(2)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:26)
- [ ] `Vec1f.dimensions(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:33)
- [ ] `Vec1f.copy(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:38)
- [ ] `Vec1f.add(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:42)
- [ ] `Vec1f.sub(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:46)
- [ ] `Vec1f.scale(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:50)
- [ ] `Vec1f.equals(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:55)
- [ ] `Vec1f.epsilonEquals(2)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:60)
- [ ] `Vec1f.distance(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:65)
- [ ] `Vec1f.distanceSqr(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:69)
- [ ] `Vec1f.length(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:74)
- [ ] `Vec1f.lengthSquared(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:79)
- [ ] `Vec1f.angle(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:84)
- [ ] `Vec1f.dot(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec1f.java:88)

### `Vec2d`: it extends `VecNd`

- [ ] `Vec2d.Vec2d(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:7)
- [ ] `Vec2d.Vec2d(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:10)
- [ ] `Vec2d.Vec2d(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:15)
- [ ] `Vec2d.set(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:19)
- [ ] `Vec2d.get(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:24)
- [ ] `Vec2d.set(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:33)
- [ ] `Vec2d.set(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:38)
- [ ] `Vec2d.dimensions(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:47)
- [ ] `Vec2d.copy(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:52)
- [ ] `Vec2d.add(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:56)
- [ ] `Vec2d.add(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:61)
- [ ] `Vec2d.sub(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:66)
- [ ] `Vec2d.sub(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:71)
- [ ] `Vec2d.scale(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:76)
- [ ] `Vec2d.equals(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:82)
- [ ] `Vec2d.epsilonEquals(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:87)
- [ ] `Vec2d.distance(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:101)
- [ ] `Vec2d.distanceSqr(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:107)
- [ ] `Vec2d.length(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:113)
- [ ] `Vec2d.lengthSquared(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:118)
- [ ] `Vec2d.angle(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:123)
- [ ] `Vec2d.dot(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec2d.java:136)

### `Vec2f`: it extends `VecNf`

- [ ] `Vec2f.Vec2f(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:7)
- [ ] `Vec2f.Vec2f(2)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:10)
- [ ] `Vec2f.Vec2f(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:15)
- [ ] `Vec2f.set(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:19)
- [ ] `Vec2f.get(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:24)
- [ ] `Vec2f.set(2)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:33)
- [ ] `Vec2f.dimensions(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:42)
- [ ] `Vec2f.copy(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:47)
- [ ] `Vec2f.add(2)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:51)
- [ ] `Vec2f.add(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:56)
- [ ] `Vec2f.sub(2)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:61)
- [ ] `Vec2f.sub(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:66)
- [ ] `Vec2f.scale(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:71)
- [ ] `Vec2f.equals(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:77)
- [ ] `Vec2f.epsilonEquals(2)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:82)
- [ ] `Vec2f.distance(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:96)
- [ ] `Vec2f.distanceSqr(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:102)
- [ ] `Vec2f.length(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:108)
- [ ] `Vec2f.lengthSquared(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:113)
- [ ] `Vec2f.angle(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:118)
- [ ] `Vec2f.dot(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec2f.java:131)

### `Vec3d`: it extends `VecNd`

- [ ] `Vec3d.Vec3d(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:15)
- [ ] `Vec3d.Vec3d(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:18)
- [ ] `Vec3d.Vec3d(3)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:22)
- [ ] `Vec3d.Vec3d(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:28)
- [ ] `Vec3d.Vec3d(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:32)
- [ ] `Vec3d.Vec3d(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:36)
- [ ] `Vec3d.Vec3d(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:40)
- [ ] `Vec3d.Vec3d(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:44)
- [ ] `Vec3d.toVanilla(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:48)
- [ ] `Vec3d.toBlockPos(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:52)
- [ ] `Vec3d.set(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:56)
- [ ] `Vec3d.set(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:62)
- [ ] `Vec3d.get(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:68)
- [ ] `Vec3d.get(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:78)
- [ ] `Vec3d.set(3)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:88)
- [ ] `Vec3d.set(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:94)
- [ ] `Vec3d.set(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:108)
- [ ] `Vec3d.dimensions(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:122)
- [ ] `Vec3d.copy(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:127)
- [ ] `Vec3d.add(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:131)
- [ ] `Vec3d.sub(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:137)
- [ ] `Vec3d.scale(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:143)
- [ ] `Vec3d.equals(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:150)
- [ ] `Vec3d.epsilonEquals(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:155)
- [ ] `Vec3d.distance(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:178)
- [ ] `Vec3d.distance(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:182)
- [ ] `Vec3d.distanceSqr(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:189)
- [ ] `Vec3d.distance(3)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:196)
- [ ] `Vec3d.distanceSqr(3)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:203)
- [ ] `Vec3d.length(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:210)
- [ ] `Vec3d.lengthSquared(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:215)
- [ ] `Vec3d.angle(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:220)
- [ ] `Vec3d.cross(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:233)
- [ ] `Vec3d.dot(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec3d.java:239)

### `Vec3f`: it extends `VecNf`

- [ ] `Vec3f.Vec3f(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:12)
- [ ] `Vec3f.Vec3f(3)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:15)
- [ ] `Vec3f.Vec3f(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:21)
- [ ] `Vec3f.Vec3f(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:25)
- [ ] `Vec3f.Vec3f(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:29)
- [ ] `Vec3f.toVanilla(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:33)
- [ ] `Vec3f.set(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:37)
- [ ] `Vec3f.get(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:43)
- [ ] `Vec3f.get(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:53)
- [ ] `Vec3f.set(3)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:63)
- [ ] `Vec3f.set(2)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:69)
- [ ] `Vec3f.set(2)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:83)
- [ ] `Vec3f.dimensions(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:97)
- [ ] `Vec3f.copy(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:102)
- [ ] `Vec3f.add(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:106)
- [ ] `Vec3f.sub(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:112)
- [ ] `Vec3f.scale(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:118)
- [ ] `Vec3f.equals(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:126)
- [ ] `Vec3f.epsilonEquals(2)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:131)
- [ ] `Vec3f.distance(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:154)
- [ ] `Vec3f.distanceSqr(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:161)
- [ ] `Vec3f.length(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:168)
- [ ] `Vec3f.lengthSquared(0)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:173)
- [ ] `Vec3f.angle(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:178)
- [ ] `Vec3f.cross(2)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:191)
- [ ] `Vec3f.dot(1)`: its class isn't translated (it extends `VecNf`) (team/creative/creativecore/common/util/math/vec/Vec3f.java:197)

### `Vec4d`: it extends `VecNd`

- [ ] `Vec4d.Vec4d(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:11)
- [ ] `Vec4d.Vec4d(4)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:14)
- [ ] `Vec4d.Vec4d(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:21)
- [ ] `Vec4d.Vec4d(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:25)
- [ ] `Vec4d.set(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:29)
- [ ] `Vec4d.get(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:36)
- [ ] `Vec4d.get(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:47)
- [ ] `Vec4d.set(4)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:57)
- [ ] `Vec4d.set(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:64)
- [ ] `Vec4d.set(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:78)
- [ ] `Vec4d.dimensions(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:95)
- [ ] `Vec4d.copy(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:100)
- [ ] `Vec4d.add(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:104)
- [ ] `Vec4d.sub(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:111)
- [ ] `Vec4d.scale(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:118)
- [ ] `Vec4d.equals(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:126)
- [ ] `Vec4d.epsilonEquals(2)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:131)
- [ ] `Vec4d.distance(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:163)
- [ ] `Vec4d.distanceSqr(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:171)
- [ ] `Vec4d.length(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:179)
- [ ] `Vec4d.lengthSquared(0)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:184)
- [ ] `Vec4d.angle(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:189)
- [ ] `Vec4d.dot(1)`: its class isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/math/vec/Vec4d.java:202)

### `VecNd`: it's generic

- [ ] `VecNd.VecNd(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:9)
- [ ] `VecNd.VecNd(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:12)
- [ ] `VecNd.set(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:16)
- [ ] `VecNd.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:18)
- [ ] `VecNd.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:22)
- [ ] `VecNd.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:26)
- [ ] `VecNd.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:28)
- [ ] `VecNd.dimensions(0)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:30)
- [ ] `VecNd.copy(0)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:32)
- [ ] `VecNd.add(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:34)
- [ ] `VecNd.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:36)
- [ ] `VecNd.sub(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:41)
- [ ] `VecNd.sub(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:43)
- [ ] `VecNd.scale(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:48)
- [ ] `VecNd.invert(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:50)
- [ ] `VecNd.equals(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:54)
- [ ] `VecNd.epsilonEquals(2)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:57)
- [ ] `VecNd.epsilonEquals(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:59)
- [ ] `VecNd.distance(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:63)
- [ ] `VecNd.distanceSqr(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:65)
- [ ] `VecNd.length(0)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:67)
- [ ] `VecNd.lengthSquared(0)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:69)
- [ ] `VecNd.normalize(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:71)
- [ ] `VecNd.angle(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:75)
- [ ] `VecNd.dot(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNd.java:77)
- [ ] `VecNd.toLong(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:79)
- [ ] `VecNd.toString(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:89)
- [ ] `VecNd.load(1)`: it uses VecNd, which isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNd.java:106)
- [ ] `VecNd.createEmptyVec(1)`: it's generic (team/creative/creativecore/common/util/math/vec/VecNd.java:118)

### `VecNf`: it's generic

- [ ] `VecNf.VecNf(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:6)
- [ ] `VecNf.VecNf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:9)
- [ ] `VecNf.set(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:13)
- [ ] `VecNf.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:15)
- [ ] `VecNf.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:19)
- [ ] `VecNf.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:23)
- [ ] `VecNf.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:25)
- [ ] `VecNf.dimensions(0)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:27)
- [ ] `VecNf.copy(0)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:29)
- [ ] `VecNf.add(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:31)
- [ ] `VecNf.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:33)
- [ ] `VecNf.sub(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:38)
- [ ] `VecNf.sub(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:40)
- [ ] `VecNf.scale(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:45)
- [ ] `VecNf.invert(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:47)
- [ ] `VecNf.equals(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:51)
- [ ] `VecNf.epsilonEquals(2)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:54)
- [ ] `VecNf.epsilonEquals(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:56)
- [ ] `VecNf.distance(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:60)
- [ ] `VecNf.distanceSqr(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:62)
- [ ] `VecNf.length(0)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:64)
- [ ] `VecNf.lengthSquared(0)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:66)
- [ ] `VecNf.normalize(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:68)
- [ ] `VecNf.angle(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:72)
- [ ] `VecNf.dot(1)`: it has no body (team/creative/creativecore/common/util/math/vec/VecNf.java:74)
- [ ] `VecNf.toString(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/math/vec/VecNf.java:76)

### `VectorUtils`

- [ ] `VectorUtils.set(3)`: it uses `Vector3d` (org.joml) (team/creative/creativecore/common/util/math/vec/VectorUtils.java:12)
- [ ] `VectorUtils.set(3)`: it uses `Vector3d` (org.joml) (team/creative/creativecore/common/util/math/vec/VectorUtils.java:27)
- [ ] `VectorUtils.set(3)`: it uses `Vector3d` (org.joml) (team/creative/creativecore/common/util/math/vec/VectorUtils.java:35)
- [ ] `VectorUtils.set(3)`: it uses `Vector3d` (org.joml) (team/creative/creativecore/common/util/math/vec/VectorUtils.java:50)
- [ ] `VectorUtils.set(3)`: it uses `Vector3d` (org.joml) (team/creative/creativecore/common/util/math/vec/VectorUtils.java:58)
- [ ] `VectorUtils.get(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:66)
- [ ] `VectorUtils.get(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:70)
- [ ] `VectorUtils.get(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:74)
- [ ] `VectorUtils.get(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:78)
- [ ] `VectorUtils.get(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:82)
- [ ] `VectorUtils.get(4)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:86)
- [ ] `VectorUtils.get(4)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:94)
- [ ] `VectorUtils.get(4)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:102)
- [ ] `VectorUtils.get(4)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:110)
- [ ] `VectorUtils.get(4)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:119)
- [ ] `VectorUtils.get(4)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:128)
- [ ] `VectorUtils.isZero(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:137)
- [ ] `VectorUtils.isZero(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:141)
- [ ] `VectorUtils.equals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:145)
- [ ] `VectorUtils.equals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:149)
- [ ] `VectorUtils.within(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:153)
- [ ] `VectorUtils.within(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:157)
- [ ] `VectorUtils.greaterEquals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:161)
- [ ] `VectorUtils.smallerEquals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:165)
- [ ] `VectorUtils.greaterEquals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:169)
- [ ] `VectorUtils.smallerEquals(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/math/vec/VectorUtils.java:173)

### `ColorUtils`

- [ ] `ColorUtils.HEX`: it uses `HexFormat` (java.util) (team/creative/creativecore/common/util/mc/ColorUtils.java:9)
- [x] `ColorUtils.WHITE` (team/creative/creativecore/common/util/mc/ColorUtils.java:10)
- [x] `ColorUtils.RED` (team/creative/creativecore/common/util/mc/ColorUtils.java:11)
- [x] `ColorUtils.GREEN` (team/creative/creativecore/common/util/mc/ColorUtils.java:12)
- [x] `ColorUtils.BLUE` (team/creative/creativecore/common/util/mc/ColorUtils.java:13)
- [x] `ColorUtils.LIGHT_BLUE` (team/creative/creativecore/common/util/mc/ColorUtils.java:14)
- [x] `ColorUtils.ORANGE` (team/creative/creativecore/common/util/mc/ColorUtils.java:15)
- [x] `ColorUtils.YELLOW` (team/creative/creativecore/common/util/mc/ColorUtils.java:16)
- [x] `ColorUtils.CYAN` (team/creative/creativecore/common/util/mc/ColorUtils.java:17)
- [x] `ColorUtils.MAGENTA` (team/creative/creativecore/common/util/mc/ColorUtils.java:18)
- [x] `ColorUtils.BLACK` (team/creative/creativecore/common/util/mc/ColorUtils.java:19)
- [x] `ColorUtils.GRAY` (team/creative/creativecore/common/util/mc/ColorUtils.java:20)
- [x] `ColorUtils.DARK_GRAY` (team/creative/creativecore/common/util/mc/ColorUtils.java:21)
- [x] `ColorUtils.alpha(1)` (team/creative/creativecore/common/util/mc/ColorUtils.java:23)
- [x] `ColorUtils.red(1)` (team/creative/creativecore/common/util/mc/ColorUtils.java:27)
- [x] `ColorUtils.green(1)` (team/creative/creativecore/common/util/mc/ColorUtils.java:31)
- [x] `ColorUtils.blue(1)` (team/creative/creativecore/common/util/mc/ColorUtils.java:35)
- [x] `ColorUtils.alphaF(1)` (team/creative/creativecore/common/util/mc/ColorUtils.java:39)
- [x] `ColorUtils.redF(1)` (team/creative/creativecore/common/util/mc/ColorUtils.java:43)
- [x] `ColorUtils.greenF(1)` (team/creative/creativecore/common/util/mc/ColorUtils.java:47)
- [x] `ColorUtils.blueF(1)` (team/creative/creativecore/common/util/mc/ColorUtils.java:51)
- [ ] `ColorUtils.setAlpha(2)`: unknown static ColorPart.ALPHA (team/creative/creativecore/common/util/mc/ColorUtils.java:55)
- [ ] `ColorUtils.rgba(4)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/ColorUtils.java:59)
- [ ] `ColorUtils.rgba(4)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/ColorUtils.java:63)
- [ ] `ColorUtils.rgb(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/ColorUtils.java:67)
- [ ] `ColorUtils.rgb(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/ColorUtils.java:71)
- [ ] `ColorUtils.toVec(1)`: it uses `Vector3d` (org.joml) (team/creative/creativecore/common/util/mc/ColorUtils.java:75)
- [x] `ColorUtils.isDefault(1)` (team/creative/creativecore/common/util/mc/ColorUtils.java:82)
- [x] `ColorUtils.isTransparent(1)` (team/creative/creativecore/common/util/mc/ColorUtils.java:86)
- [x] `ColorUtils.isInvisible(1)` (team/creative/creativecore/common/util/mc/ColorUtils.java:90)
- [ ] `ColorUtils.substract(2)`: it calls ColorUtils.rgba, which isn't translated (team/creative/creativecore/common/util/mc/ColorUtils.java:94)
- [x] `ColorUtils.blend(2)` (team/creative/creativecore/common/util/mc/ColorUtils.java:106)
- [x] `ColorUtils.blend(3)` (team/creative/creativecore/common/util/mc/ColorUtils.java:110)
- [ ] `ColorUtils.toHex(1)`: it uses ColorUtils.HEX, which isn't translated (it uses `HexFormat` (java.util)) (team/creative/creativecore/common/util/mc/ColorUtils.java:133)
- [ ] `ColorUtils.fromHex(1)`: it calls `HexFormat` (java.util).fromHexDigits (team/creative/creativecore/common/util/mc/ColorUtils.java:137)

### `ColorUtils.ColorPart`: its constants carry values

- [ ] `ColorUtils.ColorPart.ColorPart(1)`: its class isn't translated (its constants carry values) (team/creative/creativecore/common/util/mc/ColorUtils.java:189)
- [ ] `ColorUtils.ColorPart.get(1)`: it has no body (team/creative/creativecore/common/util/mc/ColorUtils.java:193)
- [ ] `ColorUtils.ColorPart.set(2)`: it has no body (team/creative/creativecore/common/util/mc/ColorUtils.java:195)

### `ConnectionUtils`

- [ ] `ConnectionUtils.getProtocolInfo(1)`: it uses Minecraft's `ProtocolInfo` (team/creative/creativecore/common/util/mc/ConnectionUtils.java:8)

### `JsonUtils`

- [ ] `JsonUtils.set(4)`: it calls JsonUtils.get, which isn't translated (team/creative/creativecore/common/util/mc/JsonUtils.java:26)
- [ ] `JsonUtils.tryGet(2)`: it calls `Arrays` (java.util).toString (team/creative/creativecore/common/util/mc/JsonUtils.java:30)
- [ ] `JsonUtils.get(2)`: it calls `Arrays` (java.util).toString (team/creative/creativecore/common/util/mc/JsonUtils.java:49)
- [ ] `JsonUtils.cleanUp(1)`: .iterator on a map's entries (team/creative/creativecore/common/util/mc/JsonUtils.java:70)
- [ ] `JsonUtils.of(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/JsonUtils.java:83)
- [ ] `JsonUtils.of(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/JsonUtils.java:93)

### `LanguageUtils`

- [ ] `LanguageUtils.can(1)`: it calls Minecraft's `Language`.getInstance (team/creative/creativecore/common/util/mc/LanguageUtils.java:9)
- [ ] `LanguageUtils.translate(1)`: it calls CreativeCore.loader, which isn't translated (team/creative/creativecore/common/util/mc/LanguageUtils.java:13)
- [ ] `LanguageUtils.translateOr(2)`: it calls LanguageUtils.translate, which isn't translated (team/creative/creativecore/common/util/mc/LanguageUtils.java:17)
- [ ] `LanguageUtils.translate(2)`: it uses `Object` (team/creative/creativecore/common/util/mc/LanguageUtils.java:22)

### `LevelUtils`

- [ ] `LevelUtils.dropItem(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/LevelUtils.java:12)
- [ ] `LevelUtils.dropItem(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/LevelUtils.java:18)
- [ ] `LevelUtils.dropItem(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/LevelUtils.java:29)
- [ ] `LevelUtils.dropItem(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/LevelUtils.java:35)

### `NBTUtils`

- [ ] `NBTUtils.mergeNotOverwrite(2)`: it uses Minecraft's `CompoundTag` (team/creative/creativecore/common/util/mc/NBTUtils.java:23)
- [ ] `NBTUtils.of(1)`: it uses Minecraft's `Tag` (team/creative/creativecore/common/util/mc/NBTUtils.java:39)
- [ ] `NBTUtils.of(2)`: it uses Minecraft's `CompoundTag` (team/creative/creativecore/common/util/mc/NBTUtils.java:43)
- [ ] `NBTUtils.of(1)`: it uses Minecraft's `Tag` (team/creative/creativecore/common/util/mc/NBTUtils.java:51)

### `PlatformResourceUtils`

- [ ] `PlatformResourceUtils.length(1)`: Path.toFile (team/creative/creativecore/common/util/mc/PlatformResourceUtils.java:11)
- [ ] `PlatformResourceUtils.resolvePath(4)`: it uses Minecraft's `PackType` (team/creative/creativecore/common/util/mc/PlatformResourceUtils.java:15)

### `PlayerUtils`

- [ ] `PlayerUtils.getPersistentData(1)`: it uses Minecraft's `CompoundTag` (team/creative/creativecore/common/util/mc/PlayerUtils.java:15)
- [ ] `PlayerUtils.getGameTypeClient(1)`: it uses Minecraft's `GameType` (team/creative/creativecore/common/util/mc/PlayerUtils.java:19)
- [ ] `PlayerUtils.isAdventure(1)`: it calls PlayerUtils.getGameType, which isn't translated (team/creative/creativecore/common/util/mc/PlayerUtils.java:24)
- [ ] `PlayerUtils.getGameType(1)`: it uses Minecraft's `GameType` (team/creative/creativecore/common/util/mc/PlayerUtils.java:28)
- [ ] `PlayerUtils.getReach(1)`: it calls Minecraft's LocalPlayer.blockInteractionRange (no Horizon helper for it yet) (team/creative/creativecore/common/util/mc/PlayerUtils.java:32)
- [ ] `PlayerUtils.addOrDrop(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/PlayerUtils.java:37)
- [ ] `PlayerUtils.addOrDrop(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/PlayerUtils.java:43)

### `ResourceUtils`

- [ ] `ResourceUtils.length(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/ResourceUtils.java:21)
- [ ] `ResourceUtils.length(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/ResourceUtils.java:25)
- [ ] `ResourceUtils.resolve(3)`: it calls Minecraft's `FileUtil`.resolvePath (team/creative/creativecore/common/util/mc/ResourceUtils.java:52)

### `StackUtils`

- [x] `StackUtils.collect(3)` in game_side.rs (team/creative/creativecore/common/util/mc/StackUtils.java:9)

### `TickUtils`

- [ ] `TickUtils.getFrameTimeClient(0)`: it calls CreativeCoreClient.getFrameTime, which isn't translated (team/creative/creativecore/common/util/mc/TickUtils.java:9)
- [ ] `TickUtils.getFrameTime(1)`: it uses Minecraft's `LevelAccessor` (team/creative/creativecore/common/util/mc/TickUtils.java:14)

### `TooltipUtils`

- [ ] `TooltipUtils.NUMBER_FORMAT`: it uses `NumberFormat` (java.text) (team/creative/creativecore/common/util/mc/TooltipUtils.java:9)
- [ ] `TooltipUtils.DECIMAL_FORMAT`: it uses `DecimalFormat` (java.text) (team/creative/creativecore/common/util/mc/TooltipUtils.java:10)
- [ ] `TooltipUtils.printColor(1)`: it uses Minecraft's `ChatFormatting`.RED (team/creative/creativecore/common/util/mc/TooltipUtils.java:12)
- [ ] `TooltipUtils.print(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/TooltipUtils.java:20)
- [ ] `TooltipUtils.print(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/TooltipUtils.java:24)
- [ ] `TooltipUtils.print(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/mc/TooltipUtils.java:28)
- [x] `TooltipUtils.printRoman(1)` (team/creative/creativecore/common/util/mc/TooltipUtils.java:32)

### `PlayerSelector`

- [ ] `PlayerSelector.REGISTRY`: it uses NamedTypeRegistry, which isn't translated (it's generic) (team/creative/creativecore/common/util/player/PlayerSelector.java:27)
- [ ] `PlayerSelector.read(1)`: it uses Minecraft's `CompoundTag` (team/creative/creativecore/common/util/player/PlayerSelector.java:29)
- [ ] `PlayerSelector.is(1)`: it has no body (team/creative/creativecore/common/util/player/PlayerSelector.java:44)
- [ ] `PlayerSelector.readFromNBT(1)`: it has no body (team/creative/creativecore/common/util/player/PlayerSelector.java:46)
- [ ] `PlayerSelector.write(1)`: it has no body (team/creative/creativecore/common/util/player/PlayerSelector.java:48)
- [ ] `PlayerSelector.info(0)`: it has no body (team/creative/creativecore/common/util/player/PlayerSelector.java:50)
- [ ] `PlayerSelector.writeToNBT(1)`: it uses Minecraft's `CompoundTag` (team/creative/creativecore/common/util/player/PlayerSelector.java:52)

### `PlayerSelector.PlayerSelectorAnd`: it extends `PlayerSelector`

- [ ] `PlayerSelector.PlayerSelectorAnd.PlayerSelectorAnd(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:109)
- [ ] `PlayerSelector.PlayerSelectorAnd.PlayerSelectorAnd(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:112)
- [ ] `PlayerSelector.PlayerSelectorAnd.is(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:116)
- [ ] `PlayerSelector.PlayerSelectorAnd.readFromNBT(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:127)
- [ ] `PlayerSelector.PlayerSelectorAnd.write(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:137)
- [ ] `PlayerSelector.PlayerSelectorAnd.info(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:148)

### `PlayerSelector.PlayerSelectorCommandSelector`: it extends `PlayerSelector`

- [ ] `PlayerSelector.PlayerSelectorCommandSelector.PlayerSelectorCommandSelector(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:167)
- [ ] `PlayerSelector.PlayerSelectorCommandSelector.PlayerSelectorCommandSelector(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:170)
- [ ] `PlayerSelector.PlayerSelectorCommandSelector.is(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:174)
- [ ] `PlayerSelector.PlayerSelectorCommandSelector.readFromNBT(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:188)
- [ ] `PlayerSelector.PlayerSelectorCommandSelector.write(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:193)
- [ ] `PlayerSelector.PlayerSelectorCommandSelector.info(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:198)

### `PlayerSelector.PlayerSelectorGamemode`: it extends `PlayerSelector`

- [ ] `PlayerSelector.PlayerSelectorGamemode.PlayerSelectorGamemode(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:207)
- [ ] `PlayerSelector.PlayerSelectorGamemode.PlayerSelectorGamemode(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:210)
- [ ] `PlayerSelector.PlayerSelectorGamemode.is(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:214)
- [ ] `PlayerSelector.PlayerSelectorGamemode.readFromNBT(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:219)
- [ ] `PlayerSelector.PlayerSelectorGamemode.write(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:224)
- [ ] `PlayerSelector.PlayerSelectorGamemode.info(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:229)

### `PlayerSelector.PlayerSelectorLevel`: it extends `PlayerSelector`

- [ ] `PlayerSelector.PlayerSelectorLevel.PlayerSelectorLevel(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:238)
- [ ] `PlayerSelector.PlayerSelectorLevel.PlayerSelectorLevel(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:241)
- [ ] `PlayerSelector.PlayerSelectorLevel.is(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:245)
- [ ] `PlayerSelector.PlayerSelectorLevel.readFromNBT(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:257)
- [ ] `PlayerSelector.PlayerSelectorLevel.write(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:262)
- [ ] `PlayerSelector.PlayerSelectorLevel.info(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:267)

### `PlayerSelector.PlayerSelectorNot`: it extends `PlayerSelector`

- [ ] `PlayerSelector.PlayerSelectorNot.PlayerSelectorNot(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:276)
- [ ] `PlayerSelector.PlayerSelectorNot.PlayerSelectorNot(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:279)
- [ ] `PlayerSelector.PlayerSelectorNot.is(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:283)
- [ ] `PlayerSelector.PlayerSelectorNot.readFromNBT(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:288)
- [ ] `PlayerSelector.PlayerSelectorNot.write(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:293)
- [ ] `PlayerSelector.PlayerSelectorNot.info(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:298)

### `PlayerSelector.PlayerSelectorOr`: it extends `PlayerSelector`

- [ ] `PlayerSelector.PlayerSelectorOr.PlayerSelectorOr(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:307)
- [ ] `PlayerSelector.PlayerSelectorOr.PlayerSelectorOr(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:310)
- [ ] `PlayerSelector.PlayerSelectorOr.is(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:314)
- [ ] `PlayerSelector.PlayerSelectorOr.readFromNBT(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:325)
- [ ] `PlayerSelector.PlayerSelectorOr.write(1)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:335)
- [ ] `PlayerSelector.PlayerSelectorOr.info(0)`: its class isn't translated (it extends `PlayerSelector`) (team/creative/creativecore/common/util/player/PlayerSelector.java:346)

### `ConfigTypeRegistry`: it's generic

- [ ] `ConfigTypeRegistry.DEFAULT_FIELD_PREDICATE`: it uses `Field` (java.lang.reflect) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:21)
- [ ] `ConfigTypeRegistry.setFieldPredicate(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:31)
- [ ] `ConfigTypeRegistry.setKeepUnrelatedData(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:36)
- [ ] `ConfigTypeRegistry.register(4)`: it's generic (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:41)
- [ ] `ConfigTypeRegistry.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:51)
- [ ] `ConfigTypeRegistry.contains(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:55)
- [ ] `ConfigTypeRegistry.getId(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:59)
- [ ] `ConfigTypeRegistry.load(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:63)
- [ ] `ConfigTypeRegistry.loadOrCreateDefault(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:68)
- [ ] `ConfigTypeRegistry.save(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:72)
- [ ] `ConfigTypeRegistry.create(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:77)
- [ ] `ConfigTypeRegistry.createDefault(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:82)
- [ ] `ConfigTypeRegistry.getEqualChecker(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:86)
- [ ] `ConfigTypeRegistry.is(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:91)

### `ConfigTypeRegistry.ConfigurationType`: it uses `T`

- [ ] `ConfigTypeRegistry.ConfigurationType.ConfigurationType(4)`: its class isn't translated (it uses `T`) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:101)
- [ ] `ConfigTypeRegistry.ConfigurationType.create(2)`: its class isn't translated (it uses `T`) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:107)
- [ ] `ConfigTypeRegistry.ConfigurationType.save(4)`: its class isn't translated (it uses `T`) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:111)
- [ ] `ConfigTypeRegistry.ConfigurationType.load(3)`: its class isn't translated (it uses `T`) (team/creative/creativecore/common/util/registry/ConfigTypeRegistry.java:117)

### `IdNotFoundException`: it extends `RegistryException`

- [ ] `IdNotFoundException.IdNotFoundException(1)`: its class isn't translated (it extends `RegistryException`) (team/creative/creativecore/common/util/registry/exception/IdNotFoundException.java:4)

### `RegistryException`: it extends `Exception`

- [ ] `RegistryException.RegistryException(1)`: its class isn't translated (it extends `Exception`) (team/creative/creativecore/common/util/registry/exception/RegistryException.java:4)

### `FilteredHandlerRegistry`: it's generic

- [ ] `FilteredHandlerRegistry.FilteredHandlerRegistry(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/FilteredHandlerRegistry.java:17)
- [ ] `FilteredHandlerRegistry.allowOverwrite(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/FilteredHandlerRegistry.java:21)
- [ ] `FilteredHandlerRegistry.getDefault(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/FilteredHandlerRegistry.java:26)
- [ ] `FilteredHandlerRegistry.register(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/FilteredHandlerRegistry.java:30)
- [ ] `FilteredHandlerRegistry.register(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/FilteredHandlerRegistry.java:38)
- [ ] `FilteredHandlerRegistry.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/FilteredHandlerRegistry.java:42)
- [ ] `FilteredHandlerRegistry.handlers(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/FilteredHandlerRegistry.java:57)

### `ICreativeRegistry`: it's an interface

- [ ] `ICreativeRegistry.get(1)`: it has no body (team/creative/creativecore/common/util/registry/ICreativeRegistry.java:4)
- [ ] `ICreativeRegistry.name(1)`: it has no body (team/creative/creativecore/common/util/registry/ICreativeRegistry.java:6)
- [ ] `ICreativeRegistry.names(0)`: it has no body (team/creative/creativecore/common/util/registry/ICreativeRegistry.java:8)
- [ ] `ICreativeRegistry.values(0)`: it has no body (team/creative/creativecore/common/util/registry/ICreativeRegistry.java:10)
- [ ] `ICreativeRegistry.next(1)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/registry/ICreativeRegistry.java:12)

### `LocatedHandlerRegistry`: it's generic

- [ ] `LocatedHandlerRegistry.LocatedHandlerRegistry(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:17)
- [ ] `LocatedHandlerRegistry.allowOverwrite(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:21)
- [ ] `LocatedHandlerRegistry.getDefault(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:26)
- [ ] `LocatedHandlerRegistry.register(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:30)
- [ ] `LocatedHandlerRegistry.registerDefault(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:39)
- [ ] `LocatedHandlerRegistry.getLocation(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:44)
- [ ] `LocatedHandlerRegistry.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:48)
- [ ] `LocatedHandlerRegistry.getOrThrow(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:52)
- [ ] `LocatedHandlerRegistry.contains(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:61)
- [ ] `LocatedHandlerRegistry.keys(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:65)
- [ ] `LocatedHandlerRegistry.entrySet(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:69)
- [ ] `LocatedHandlerRegistry.values(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:73)
- [ ] `LocatedHandlerRegistry.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:77)
- [ ] `LocatedHandlerRegistry.name(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:82)
- [ ] `LocatedHandlerRegistry.names(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/LocatedHandlerRegistry.java:87)

### `NamedClassBoundHandlerRegistry`: it's generic

- [ ] `NamedClassBoundHandlerRegistry.clearRegistry(1)`: it uses NamedClassBoundHandlerRegistry, which isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:16)
- [ ] `NamedClassBoundHandlerRegistry.allowOverwrite(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:22)
- [ ] `NamedClassBoundHandlerRegistry.getDefault(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:27)
- [ ] `NamedClassBoundHandlerRegistry.register(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:31)
- [ ] `NamedClassBoundHandlerRegistry.registerDefault(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:41)
- [ ] `NamedClassBoundHandlerRegistry.getId(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:46)
- [ ] `NamedClassBoundHandlerRegistry.getId(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:50)
- [ ] `NamedClassBoundHandlerRegistry.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:54)
- [ ] `NamedClassBoundHandlerRegistry.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:58)
- [ ] `NamedClassBoundHandlerRegistry.getOrThrow(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:62)
- [ ] `NamedClassBoundHandlerRegistry.getOrThrow(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:71)
- [ ] `NamedClassBoundHandlerRegistry.contains(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:80)
- [ ] `NamedClassBoundHandlerRegistry.contains(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:84)
- [ ] `NamedClassBoundHandlerRegistry.keys(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:88)
- [ ] `NamedClassBoundHandlerRegistry.entrySet(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:92)
- [ ] `NamedClassBoundHandlerRegistry.values(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:96)
- [ ] `NamedClassBoundHandlerRegistry.classes(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedClassBoundHandlerRegistry.java:100)

### `NamedHandlerRegistry`: it's generic

- [ ] `NamedHandlerRegistry.clearRegistry(1)`: it uses NamedHandlerRegistry, which isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:15)
- [ ] `NamedHandlerRegistry.NamedHandlerRegistry(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:21)
- [ ] `NamedHandlerRegistry.allowOverwrite(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:25)
- [ ] `NamedHandlerRegistry.getDefault(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:30)
- [ ] `NamedHandlerRegistry.register(2)`: it's generic (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:34)
- [ ] `NamedHandlerRegistry.registerDefault(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:44)
- [ ] `NamedHandlerRegistry.getId(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:49)
- [ ] `NamedHandlerRegistry.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:53)
- [ ] `NamedHandlerRegistry.getOrThrow(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:58)
- [ ] `NamedHandlerRegistry.contains(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:67)
- [ ] `NamedHandlerRegistry.keys(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:71)
- [ ] `NamedHandlerRegistry.names(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:75)
- [ ] `NamedHandlerRegistry.entrySet(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:79)
- [ ] `NamedHandlerRegistry.values(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:83)
- [ ] `NamedHandlerRegistry.name(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedHandlerRegistry.java:87)

### `NamedTypeRegistry`: it's generic

- [ ] `NamedTypeRegistry.addConstructorPattern(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:22)
- [ ] `NamedTypeRegistry.allowOverwrite(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:27)
- [ ] `NamedTypeRegistry.printConstructors(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:32)
- [ ] `NamedTypeRegistry.checkConstructor(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:53)
- [ ] `NamedTypeRegistry.checkConstructor(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:79)
- [ ] `NamedTypeRegistry.register(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:95)
- [ ] `NamedTypeRegistry.getId(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:104)
- [ ] `NamedTypeRegistry.getIdOrDefault(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:108)
- [ ] `NamedTypeRegistry.getId(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:112)
- [ ] `NamedTypeRegistry.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:116)
- [ ] `NamedTypeRegistry.create(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:120)
- [ ] `NamedTypeRegistry.createSafe(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:140)
- [ ] `NamedTypeRegistry.keys(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:158)
- [ ] `NamedTypeRegistry.entrySet(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:162)
- [ ] `NamedTypeRegistry.values(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:166)
- [ ] `NamedTypeRegistry.contains(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:170)
- [ ] `NamedTypeRegistry.toString(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:174)
- [ ] `NamedTypeRegistry.toString(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:199)

### `NamedTypeRegistry.ConstructorForbiddenException`: it extends `RegistryException` (team.creative.creativecore.common.util.registry.exception)

- [ ] `NamedTypeRegistry.ConstructorForbiddenException.ConstructorForbiddenException(2)`: its class isn't translated (it extends `RegistryException` (team.creative.creativecore.common.util.registry.exception)) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:225)

### `NamedTypeRegistry.ConstructorNotFoundException`: it extends `RegistryException` (team.creative.creativecore.common.util.registry.exception)

- [ ] `NamedTypeRegistry.ConstructorNotFoundException.ConstructorNotFoundException(1)`: its class isn't translated (it extends `RegistryException` (team.creative.creativecore.common.util.registry.exception)) (team/creative/creativecore/common/util/registry/NamedTypeRegistry.java:231)

### `AdvancedComponentHelper`

- [ ] `AdvancedComponentHelper.SPLITTER`: it uses AdvancedStringSplitter, which isn't translated (it extends Minecraft's `StringSplitter`) (team/creative/creativecore/common/util/text/AdvancedComponentHelper.java:16)
- [ ] `AdvancedComponentHelper.STOP_ITERATION`: it uses `Object` (team/creative/creativecore/common/util/text/AdvancedComponentHelper.java:17)
- [ ] `AdvancedComponentHelper.copy(1)`: Component.getContents (team/creative/creativecore/common/util/text/AdvancedComponentHelper.java:19)
- [ ] `AdvancedComponentHelper.iterateFormatted(3)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/text/AdvancedComponentHelper.java:30)
- [ ] `AdvancedComponentHelper.visit(3)`: it's generic (team/creative/creativecore/common/util/text/AdvancedComponentHelper.java:44)
- [ ] `AdvancedComponentHelper.visit(3)`: it's generic (team/creative/creativecore/common/util/text/AdvancedComponentHelper.java:61)

### `AdvancedStringSplitter`: it extends Minecraft's `StringSplitter`

- [ ] `AdvancedStringSplitter.AdvancedStringSplitter(1)`: its class isn't translated (it extends Minecraft's `StringSplitter`) (team/creative/creativecore/common/util/text/AdvancedStringSplitter.java:18)
- [ ] `AdvancedStringSplitter.stringWidth(1)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/text/AdvancedStringSplitter.java:24)

### `AdvancedContent`: it's an interface

- [ ] `AdvancedContent.visit(2)`: it's generic (team/creative/creativecore/common/util/text/content/AdvancedContent.java:10)
- [ ] `AdvancedContent.width(2)`: it has no body (team/creative/creativecore/common/util/text/content/AdvancedContent.java:14)
- [ ] `AdvancedContent.height(0)`: it has no body (team/creative/creativecore/common/util/text/content/AdvancedContent.java:16)
- [ ] `AdvancedContent.asText(0)`: it has no body (team/creative/creativecore/common/util/text/content/AdvancedContent.java:18)

### `AdvancedContentConsumer`: it's an interface

- [ ] `AdvancedContentConsumer.accept(2)`: it has no body (team/creative/creativecore/common/util/text/content/AdvancedContentConsumer.java:8)
- [ ] `AdvancedContentConsumer.accept(2)`: it has no body (team/creative/creativecore/common/util/text/content/AdvancedContentConsumer.java:10)

### `AdvancedFormattedText`: it's an interface

- [ ] `AdvancedFormattedText.visit(1)`: it's generic (team/creative/creativecore/common/util/text/content/AdvancedFormattedText.java:12)
- [ ] `AdvancedFormattedText.visit(2)`: it's generic (team/creative/creativecore/common/util/text/content/AdvancedFormattedText.java:16)
- [ ] `AdvancedFormattedText.width(2)`: it has no body (team/creative/creativecore/common/util/text/content/AdvancedFormattedText.java:20)
- [ ] `AdvancedFormattedText.height(0)`: it has no body (team/creative/creativecore/common/util/text/content/AdvancedFormattedText.java:22)
- [ ] `AdvancedFormattedText.render(2)`: it has no body (team/creative/creativecore/common/util/text/content/AdvancedFormattedText.java:24)

### `ContentItemStack`: it implements `AdvancedContent`

- [ ] `ContentItemStack.CODEC`: it uses Minecraft's `MapCodec` (team/creative/creativecore/common/util/text/content/ContentItemStack.java:17)
- [ ] `ContentItemStack.ContentItemStack(1)`: its class isn't translated (it implements `AdvancedContent`) (team/creative/creativecore/common/util/text/content/ContentItemStack.java:16)
- [ ] `ContentItemStack.visit(2)`: it's generic (team/creative/creativecore/common/util/text/content/ContentItemStack.java:21)
- [ ] `ContentItemStack.visit(1)`: it's generic (team/creative/creativecore/common/util/text/content/ContentItemStack.java:25)
- [ ] `ContentItemStack.width(2)`: its class isn't translated (it implements `AdvancedContent`) (team/creative/creativecore/common/util/text/content/ContentItemStack.java:29)
- [ ] `ContentItemStack.height(0)`: its class isn't translated (it implements `AdvancedContent`) (team/creative/creativecore/common/util/text/content/ContentItemStack.java:34)
- [ ] `ContentItemStack.asText(0)`: its class isn't translated (it implements `AdvancedContent`) (team/creative/creativecore/common/util/text/content/ContentItemStack.java:39)
- [ ] `ContentItemStack.codec(0)`: its class isn't translated (it implements `AdvancedContent`) (team/creative/creativecore/common/util/text/content/ContentItemStack.java:44)
- [ ] `ContentItemStack.stack(0)`: its class isn't translated (it implements `AdvancedContent`) (team/creative/creativecore/common/util/text/content/ContentItemStack.java:16)

### `ContentItemStack.ContentItemStackText`: it implements `AdvancedFormattedText`

- [ ] `ContentItemStack.ContentItemStackText.ContentItemStackText(1)`: its class isn't translated (it implements `AdvancedFormattedText`) (team/creative/creativecore/common/util/text/content/ContentItemStack.java:48)
- [ ] `ContentItemStack.ContentItemStackText.width(2)`: its class isn't translated (it implements `AdvancedFormattedText`) (team/creative/creativecore/common/util/text/content/ContentItemStack.java:49)
- [ ] `ContentItemStack.ContentItemStackText.height(0)`: its class isn't translated (it implements `AdvancedFormattedText`) (team/creative/creativecore/common/util/text/content/ContentItemStack.java:54)
- [ ] `ContentItemStack.ContentItemStackText.render(2)`: its class isn't translated (it implements `AdvancedFormattedText`) (team/creative/creativecore/common/util/text/content/ContentItemStack.java:59)
- [ ] `ContentItemStack.ContentItemStackText.content(0)`: its class isn't translated (it implements `AdvancedFormattedText`) (team/creative/creativecore/common/util/text/content/ContentItemStack.java:48)

### `FormattedSingleSink`: it's an interface

- [ ] `FormattedSingleSink.accept(3)`: it has no body (team/creative/creativecore/common/util/text/content/FormattedSingleSink.java:7)
- [ ] `FormattedSingleSink.accept(2)`: it has no body (team/creative/creativecore/common/util/text/content/FormattedSingleSink.java:9)

### `IComponentMap`: it's an interface

- [ ] `IComponentMap.entrySet(0)`: it has no body (team/creative/creativecore/common/util/text/IComponentMap.java:12)
- [ ] `IComponentMap.values(0)`: it has no body (team/creative/creativecore/common/util/text/IComponentMap.java:14)
- [ ] `IComponentMap.keys(0)`: it has no body (team/creative/creativecore/common/util/text/IComponentMap.java:16)
- [ ] `IComponentMap.build(0)`: it has no body (team/creative/creativecore/common/util/text/IComponentMap.java:18)

### `TextBuilder`: it uses Minecraft's `Component`

- [ ] `TextBuilder.format`: it uses `NumberFormat` (java.text) (team/creative/creativecore/common/util/text/TextBuilder.java:17)
- [ ] `TextBuilder.TextBuilder(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:20)
- [ ] `TextBuilder.TextBuilder(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:23)
- [ ] `TextBuilder.add(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:27)
- [ ] `TextBuilder.TextBuilder(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:37)
- [ ] `TextBuilder.translateIfCan(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:41)
- [ ] `TextBuilder.translate(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:50)
- [ ] `TextBuilder.translate(2)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:55)
- [ ] `TextBuilder.getLastStyle(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:60)
- [ ] `TextBuilder.text(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:74)
- [ ] `TextBuilder.number(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:92)
- [ ] `TextBuilder.number(2)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:96)
- [ ] `TextBuilder.number(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:106)
- [ ] `TextBuilder.number(2)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:110)
- [ ] `TextBuilder.newLine(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:120)
- [ ] `TextBuilder.bool(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:125)
- [ ] `TextBuilder.color(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:135)
- [ ] `TextBuilder.stack(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:144)
- [ ] `TextBuilder.build(0)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:149)
- [ ] `TextBuilder.textColor(1)`: its class isn't translated (it uses Minecraft's `Component`) (team/creative/creativecore/common/util/text/TextBuilder.java:153)

### `TextMapBuilder`: it's generic

- [ ] `TextMapBuilder.setFilter(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:20)
- [ ] `TextMapBuilder.addNewLine(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:25)
- [ ] `TextMapBuilder.addNewLine(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:31)
- [ ] `TextMapBuilder.addComponent(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:47)
- [ ] `TextMapBuilder.addComponent(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:52)
- [ ] `TextMapBuilder.addComponent(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:60)
- [ ] `TextMapBuilder.addComponent(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:68)
- [ ] `TextMapBuilder.addEntrySet(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:79)
- [ ] `TextMapBuilder.addComponents(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:87)
- [ ] `TextMapBuilder.size(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:95)
- [ ] `TextMapBuilder.first(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:99)
- [ ] `TextMapBuilder.build(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:103)
- [ ] `TextMapBuilder.entrySet(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:116)
- [ ] `TextMapBuilder.values(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:121)
- [ ] `TextMapBuilder.keys(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/text/TextMapBuilder.java:126)

### `Bunch`: it's an interface

- [ ] `Bunch.size(0)`: it has no body (team/creative/creativecore/common/util/type/Bunch.java:4)

### `Color`

- [ ] `Color.NONE`: it creates a Color with a constructor that isn't translated (team/creative/creativecore/common/util/type/Color.java:9)
- [ ] `Color.WHITE`: it creates a Color with a constructor that isn't translated (team/creative/creativecore/common/util/type/Color.java:10)
- [ ] `Color.RED`: it creates a Color with a constructor that isn't translated (team/creative/creativecore/common/util/type/Color.java:11)
- [ ] `Color.GREEN`: it creates a Color with a constructor that isn't translated (team/creative/creativecore/common/util/type/Color.java:12)
- [ ] `Color.BLUE`: it creates a Color with a constructor that isn't translated (team/creative/creativecore/common/util/type/Color.java:13)
- [ ] `Color.LIGHT_BLUE`: it creates a Color with a constructor that isn't translated (team/creative/creativecore/common/util/type/Color.java:14)
- [ ] `Color.ORANGE`: it creates a Color with a constructor that isn't translated (team/creative/creativecore/common/util/type/Color.java:15)
- [ ] `Color.YELLOW`: it creates a Color with a constructor that isn't translated (team/creative/creativecore/common/util/type/Color.java:16)
- [ ] `Color.CYAN`: it creates a Color with a constructor that isn't translated (team/creative/creativecore/common/util/type/Color.java:17)
- [ ] `Color.MAGENTA`: it creates a Color with a constructor that isn't translated (team/creative/creativecore/common/util/type/Color.java:18)
- [ ] `Color.BLACK`: it creates a Color with a constructor that isn't translated (team/creative/creativecore/common/util/type/Color.java:19)
- [ ] `Color.Color(0)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:25)
- [ ] `Color.Color(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:29)
- [ ] `Color.Color(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:33)
- [ ] `Color.Color(3)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:40)
- [ ] `Color.Color(3)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:44)
- [ ] `Color.Color(3)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:48)
- [ ] `Color.Color(4)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:52)
- [ ] `Color.Color(4)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:56)
- [ ] `Color.Color(4)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:63)
- [ ] `Color.Color(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:70)
- [ ] `Color.Color(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:77)
- [ ] `Color.set(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:84)
- [ ] `Color.set(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:91)
- [ ] `Color.set(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:98)
- [ ] `Color.set(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:102)
- [ ] `Color.setAlpha(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:118)
- [ ] `Color.setAlpha(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:122)
- [ ] `Color.setRed(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:126)
- [ ] `Color.setRed(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:130)
- [ ] `Color.setGreen(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:134)
- [ ] `Color.setGreen(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:138)
- [ ] `Color.setBlue(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:142)
- [ ] `Color.setBlue(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:146)
- [ ] `Color.get(1)`: it uses ColorType, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/type/Color.java:150)
- [x] `Color.getAlpha(0)` (team/creative/creativecore/common/util/type/Color.java:160)
- [x] `Color.getRed(0)` (team/creative/creativecore/common/util/type/Color.java:164)
- [x] `Color.getGreen(0)` (team/creative/creativecore/common/util/type/Color.java:168)
- [x] `Color.getBlue(0)` (team/creative/creativecore/common/util/type/Color.java:172)
- [ ] `Color.getDecimal(1)`: it uses ColorType, which isn't translated (its constants carry values) (team/creative/creativecore/common/util/type/Color.java:176)
- [x] `Color.getAlphaDecimal(0)` (team/creative/creativecore/common/util/type/Color.java:186)
- [x] `Color.getRedDecimal(0)` (team/creative/creativecore/common/util/type/Color.java:190)
- [x] `Color.getGreenDecimal(0)` (team/creative/creativecore/common/util/type/Color.java:194)
- [x] `Color.getBlueDecimal(0)` (team/creative/creativecore/common/util/type/Color.java:198)
- [x] `Color.toInt(0)` (team/creative/creativecore/common/util/type/Color.java:202)
- [x] `Color.isDefault(0)` (team/creative/creativecore/common/util/type/Color.java:206)
- [x] `Color.isWhite(0)` (team/creative/creativecore/common/util/type/Color.java:210)
- [x] `Color.isTransparent(0)` (team/creative/creativecore/common/util/type/Color.java:214)
- [x] `Color.isInvisible(0)` (team/creative/creativecore/common/util/type/Color.java:218)
- [ ] `Color.blend(1)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:222)
- [ ] `Color.blend(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:226)
- [ ] `Color.toVec(0)`: it uses Vec3d, which isn't translated (it extends `VecNd`) (team/creative/creativecore/common/util/type/Color.java:240)
- [x] `Color.isWhite(1)` (team/creative/creativecore/common/util/type/Color.java:244)
- [x] `Color.isTransparent(1)` (team/creative/creativecore/common/util/type/Color.java:251)
- [x] `Color.isInvisible(1)` (team/creative/creativecore/common/util/type/Color.java:256)
- [ ] `Color.blend(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:261)
- [ ] `Color.blend(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:265)
- [ ] `Color.blend(2)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:280)
- [ ] `Color.blend(3)`: it's overloaded with the same number of parameters (team/creative/creativecore/common/util/type/Color.java:284)
- [ ] `Color.hex(0)`: it calls ColorUtils.toHex, which isn't translated (team/creative/creativecore/common/util/type/Color.java:307)

### `ColorType`: its constants carry values

- [ ] `ColorType.getBrightest(0)`: it has no body (team/creative/creativecore/common/util/type/ColorType.java:29)

### `ArrayIterator`: it's generic

- [ ] `ArrayIterator.ArrayIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ArrayIterator.java:7)
- [ ] `ArrayIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ArrayIterator.java:12)
- [ ] `ArrayIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ArrayIterator.java:17)

### `ArrayOffsetIterator`: it's generic

- [ ] `ArrayOffsetIterator.ArrayOffsetIterator(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ArrayOffsetIterator.java:8)
- [ ] `ArrayOffsetIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ArrayOffsetIterator.java:14)
- [ ] `ArrayOffsetIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ArrayOffsetIterator.java:19)

### `ComputeNextIterator`: it's generic

- [ ] `ComputeNextIterator.computeNext(0)`: it has no body (team/creative/creativecore/common/util/type/itr/ComputeNextIterator.java:8)
- [ ] `ComputeNextIterator.end(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ComputeNextIterator.java:10)
- [ ] `ComputeNextIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ComputeNextIterator.java:15)
- [ ] `ComputeNextIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ComputeNextIterator.java:25)

### `ComputeNextListIterator`: it's generic

- [ ] `ComputeNextListIterator.computeNext(0)`: it has no body (team/creative/creativecore/common/util/type/itr/ComputeNextListIterator.java:13)
- [ ] `ComputeNextListIterator.endNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ComputeNextListIterator.java:15)
- [ ] `ComputeNextListIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ComputeNextListIterator.java:20)
- [ ] `ComputeNextListIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ComputeNextListIterator.java:30)
- [ ] `ComputeNextListIterator.computePrevious(0)`: it has no body (team/creative/creativecore/common/util/type/itr/ComputeNextListIterator.java:36)
- [ ] `ComputeNextListIterator.endPrevious(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ComputeNextListIterator.java:38)
- [ ] `ComputeNextListIterator.hasPrevious(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ComputeNextListIterator.java:43)
- [ ] `ComputeNextListIterator.previous(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ComputeNextListIterator.java:53)

### `ConsecutiveIterator`: it's generic

- [ ] `ConsecutiveIterator.ConsecutiveIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveIterator.java:9)
- [ ] `ConsecutiveIterator.ConsecutiveIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveIterator.java:14)
- [ ] `ConsecutiveIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveIterator.java:23)
- [ ] `ConsecutiveIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveIterator.java:32)

### `ConsecutiveListIterator`: it's generic

- [ ] `ConsecutiveListIterator.ConsecutiveListIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:10)
- [ ] `ConsecutiveListIterator.ConsecutiveListIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:14)
- [ ] `ConsecutiveListIterator.goEnd(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:22)
- [ ] `ConsecutiveListIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:30)
- [ ] `ConsecutiveListIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:43)
- [ ] `ConsecutiveListIterator.hasPrevious(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:48)
- [ ] `ConsecutiveListIterator.previous(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:57)
- [ ] `ConsecutiveListIterator.nextIndex(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:62)
- [ ] `ConsecutiveListIterator.previousIndex(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:67)
- [ ] `ConsecutiveListIterator.remove(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:72)
- [ ] `ConsecutiveListIterator.set(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:77)
- [ ] `ConsecutiveListIterator.add(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/ConsecutiveListIterator.java:82)

### `FilterIterator`: it's generic

- [ ] `FilterIterator.skipNull(1)`: it's generic (team/creative/creativecore/common/util/type/itr/FilterIterator.java:11)
- [ ] `FilterIterator.FilterIterator(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterIterator.java:15)
- [ ] `FilterIterator.FilterIterator(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterIterator.java:19)
- [ ] `FilterIterator.FilterIterator(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterIterator.java:23)
- [ ] `FilterIterator.FilterIterator(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterIterator.java:27)
- [ ] `FilterIterator.computeNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterIterator.java:32)

### `FilterListIterator`: it's generic

- [ ] `FilterListIterator.skipNull(1)`: it's generic (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:12)
- [ ] `FilterListIterator.FilterListIterator(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:16)
- [ ] `FilterListIterator.FilterListIterator(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:20)
- [ ] `FilterListIterator.FilterListIterator(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:24)
- [ ] `FilterListIterator.FilterListIterator(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:28)
- [ ] `FilterListIterator.computeNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:33)
- [ ] `FilterListIterator.computePrevious(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:45)
- [ ] `FilterListIterator.nextIndex(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:57)
- [ ] `FilterListIterator.previousIndex(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:62)
- [ ] `FilterListIterator.remove(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:67)
- [ ] `FilterListIterator.set(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:72)
- [ ] `FilterListIterator.add(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FilterListIterator.java:77)

### `FunctionIterator`: it's generic

- [ ] `FunctionIterator.FunctionIterator(2)`: it's generic (team/creative/creativecore/common/util/type/itr/FunctionIterator.java:11)
- [ ] `FunctionIterator.FunctionIterator(2)`: it's generic (team/creative/creativecore/common/util/type/itr/FunctionIterator.java:18)
- [ ] `FunctionIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FunctionIterator.java:25)
- [ ] `FunctionIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FunctionIterator.java:30)

### `FunctionNonNullIterator`: it's generic

- [ ] `FunctionNonNullIterator.FunctionNonNullIterator(2)`: it's generic (team/creative/creativecore/common/util/type/itr/FunctionNonNullIterator.java:11)
- [ ] `FunctionNonNullIterator.FunctionNonNullIterator(2)`: it's generic (team/creative/creativecore/common/util/type/itr/FunctionNonNullIterator.java:18)
- [ ] `FunctionNonNullIterator.computeNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/FunctionNonNullIterator.java:25)

### `InverseArrayIterator`: it's generic

- [ ] `InverseArrayIterator.InverseArrayIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/InverseArrayIterator.java:7)
- [ ] `InverseArrayIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/InverseArrayIterator.java:12)
- [ ] `InverseArrayIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/InverseArrayIterator.java:17)

### `InverseConsecutiveIterator`: it's generic

- [ ] `InverseConsecutiveIterator.InverseConsecutiveIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/InverseConsecutiveIterator.java:9)
- [ ] `InverseConsecutiveIterator.InverseConsecutiveIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/InverseConsecutiveIterator.java:14)
- [ ] `InverseConsecutiveIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/InverseConsecutiveIterator.java:22)
- [ ] `InverseConsecutiveIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/InverseConsecutiveIterator.java:31)

### `InverseListIterator`: it's generic

- [ ] `InverseListIterator.InverseListIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/InverseListIterator.java:9)
- [ ] `InverseListIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/InverseListIterator.java:14)
- [ ] `InverseListIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/InverseListIterator.java:19)

### `IterableIterator`: it's an interface

- [ ] `IterableIterator.iterator(0)`: its class isn't translated (it's an interface) (team/creative/creativecore/common/util/type/itr/IterableIterator.java:6)

### `NestedFunctionIterator`: it's generic

- [ ] `NestedFunctionIterator.NestedFunctionIterator(2)`: it's generic (team/creative/creativecore/common/util/type/itr/NestedFunctionIterator.java:12)
- [ ] `NestedFunctionIterator.NestedFunctionIterator(2)`: it's generic (team/creative/creativecore/common/util/type/itr/NestedFunctionIterator.java:19)
- [ ] `NestedFunctionIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/NestedFunctionIterator.java:26)
- [ ] `NestedFunctionIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/NestedFunctionIterator.java:35)

### `NestedIterator`: it's generic

- [ ] `NestedIterator.NestedIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/NestedIterator.java:10)
- [ ] `NestedIterator.NestedIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/NestedIterator.java:15)
- [ ] `NestedIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/NestedIterator.java:20)
- [ ] `NestedIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/NestedIterator.java:29)

### `SingleIterator`: it's generic

- [ ] `SingleIterator.SingleIterator(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/SingleIterator.java:10)
- [ ] `SingleIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/SingleIterator.java:14)
- [ ] `SingleIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/SingleIterator.java:19)
- [ ] `SingleIterator.remove(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/SingleIterator.java:30)
- [ ] `SingleIterator.forEachRemaining(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/SingleIterator.java:35)

### `TreeIterator`: it's generic

- [ ] `TreeIterator.TreeIterator(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/TreeIterator.java:12)
- [ ] `TreeIterator.hasNext(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/TreeIterator.java:18)
- [ ] `TreeIterator.next(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/itr/TreeIterator.java:27)

### `BucketList`: it's generic

- [ ] `BucketList.BucketList(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:16)
- [ ] `BucketList.BucketList(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:19)
- [ ] `BucketList.BucketList(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:23)
- [ ] `BucketList.ensureBucketSize(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:27)
- [ ] `BucketList.recountSize(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:33)
- [ ] `BucketList.getBucket(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:41)
- [ ] `BucketList.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:52)
- [ ] `BucketList.addAll(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:58)
- [ ] `BucketList.remove(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:64)
- [ ] `BucketList.removeAll(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:70)
- [ ] `BucketList.removeBucket(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:76)
- [ ] `BucketList.clear(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:80)
- [ ] `BucketList.removeBuckets(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:86)
- [ ] `BucketList.buckets(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:90)
- [ ] `BucketList.bucketCount(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:94)
- [ ] `BucketList.size(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:98)
- [ ] `BucketList.iterator(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:102)
- [ ] `BucketList.inverseIterator(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/BucketList.java:107)

### `CopyArrayCollection`: it's generic

- [x] `CopyArrayCollection.DEFAULT_CAPACITY` (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:12)
- [ ] `CopyArrayCollection.EMPTY_ELEMENTDATA`: it uses `Object` (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:13)
- [ ] `CopyArrayCollection.DEFAULTCAPACITY_EMPTY_ELEMENTDATA`: it uses `Object` (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:14)
- [x] `CopyArrayCollection.SOFT_MAX_ARRAY_LENGTH` (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:15)
- [x] `CopyArrayCollection.newLength(3)` (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:20)
- [x] `CopyArrayCollection.hugeLength(2)` (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:25)
- [ ] `CopyArrayCollection.CopyArrayCollection(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:34)
- [ ] `CopyArrayCollection.CopyArrayCollection(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:39)
- [ ] `CopyArrayCollection.CopyArrayCollection(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:53)
- [ ] `CopyArrayCollection.CopyArrayCollection(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:59)
- [ ] `CopyArrayCollection.CopyArrayCollection(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:69)
- [ ] `CopyArrayCollection.CopyArrayCollection(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:74)
- [ ] `CopyArrayCollection.trimToSize(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:79)
- [ ] `CopyArrayCollection.ensureCapacity(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:86)
- [ ] `CopyArrayCollection.grow(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:92)
- [ ] `CopyArrayCollection.grow(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:104)
- [ ] `CopyArrayCollection.size(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:108)
- [ ] `CopyArrayCollection.isEmpty(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:113)
- [ ] `CopyArrayCollection.first(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:118)
- [ ] `CopyArrayCollection.contains(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:129)
- [ ] `CopyArrayCollection.indexOfRange(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:141)
- [ ] `CopyArrayCollection.clone(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:159)
- [ ] `CopyArrayCollection.toArray(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:164)
- [ ] `CopyArrayCollection.toArray(1)`: it's generic (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:176)
- [ ] `CopyArrayCollection.add(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:197)
- [ ] `CopyArrayCollection.clear(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:212)
- [ ] `CopyArrayCollection.addAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:230)
- [ ] `CopyArrayCollection.containsAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:266)
- [ ] `CopyArrayCollection.iterator(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:284)
- [ ] `CopyArrayCollection.remove(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:316)
- [ ] `CopyArrayCollection.remove(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:325)
- [ ] `CopyArrayCollection.fastRemove(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:343)
- [ ] `CopyArrayCollection.removeAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:352)
- [ ] `CopyArrayCollection.retainAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:357)
- [ ] `CopyArrayCollection.removeIf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:362)
- [ ] `CopyArrayCollection.bulkRemove(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:368)
- [x] `CopyArrayCollection.nBits(1)` (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:374)
- [x] `CopyArrayCollection.setBit(2)` (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:378)
- [x] `CopyArrayCollection.isClear(2)` (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:382)
- [ ] `CopyArrayCollection.bulkRemove(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:386)
- [ ] `CopyArrayCollection.equals(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:431)
- [ ] `CopyArrayCollection.hashCodeOfRange(3)`: it uses `Object` (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:450)
- [ ] `CopyArrayCollection.hashCode(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:461)
- [ ] `CopyArrayCollection.toString(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/CopyArrayCollection.java:473)

### `IndexedCollector`: it's generic

- [ ] `IndexedCollector.PREDICATE`: a raw Predicate (team/creative/creativecore/common/util/type/list/IndexedCollector.java:12)
- [ ] `IndexedCollector.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/IndexedCollector.java:18)
- [ ] `IndexedCollector.add(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/IndexedCollector.java:32)
- [ ] `IndexedCollector.startSection(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/IndexedCollector.java:41)
- [ ] `IndexedCollector.endSection(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/IndexedCollector.java:50)
- [ ] `IndexedCollector.isEmpty(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/IndexedCollector.java:58)
- [ ] `IndexedCollector.iterator(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/IndexedCollector.java:62)
- [ ] `IndexedCollector.clear(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/IndexedCollector.java:71)
- [ ] `IndexedCollector.sectionIterator(1)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/type/list/IndexedCollector.java:77)

### `MarkIterator`: it's an interface

- [ ] `MarkIterator.mark(0)`: it has no body (team/creative/creativecore/common/util/type/list/MarkIterator.java:6)

### `MarkList`: it's generic

- [ ] `MarkList.EMPTY`: it uses MarkList, which isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/MarkList.java:8)
- [ ] `MarkList.MarkList(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/MarkList.java:13)
- [ ] `MarkList.mark(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/MarkList.java:19)
- [ ] `MarkList.reset(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/MarkList.java:27)
- [ ] `MarkList.is(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/MarkList.java:35)
- [ ] `MarkList.remaing(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/MarkList.java:39)
- [ ] `MarkList.isEmpty(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/MarkList.java:43)
- [ ] `MarkList.iterator(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/type/list/MarkList.java:47)
- [ ] `MarkList.clear(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/MarkList.java:85)
- [ ] `MarkList.allIgnoreMark(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/MarkList.java:90)
- [ ] `MarkList.getIgnoreMark(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/MarkList.java:94)
- [ ] `MarkList.sizeIgnoreMark(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/MarkList.java:98)

### `Pair`: it's generic

- [ ] `Pair.Pair(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Pair.java:9)
- [ ] `Pair.getKey(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Pair.java:14)
- [ ] `Pair.getValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Pair.java:19)
- [ ] `Pair.setValue(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Pair.java:24)
- [ ] `Pair.hashCode(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Pair.java:30)
- [ ] `Pair.is(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Pair.java:35)
- [ ] `Pair.equals(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Pair.java:39)
- [ ] `Pair.toString(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Pair.java:44)

### `PairList`: it's generic

- [ ] `PairList.PairList(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:19)
- [ ] `PairList.PairList(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:22)
- [ ] `PairList.updateEntireMap(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:27)
- [ ] `PairList.add(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:37)
- [ ] `PairList.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:50)
- [ ] `PairList.addAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:60)
- [ ] `PairList.addAll(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:85)
- [ ] `PairList.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:104)
- [ ] `PairList.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:108)
- [ ] `PairList.remove(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:115)
- [ ] `PairList.remove(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:125)
- [ ] `PairList.removeAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:135)
- [ ] `PairList.removeIf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:145)
- [ ] `PairList.retainAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:155)
- [ ] `PairList.clear(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:165)
- [ ] `PairList.set(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:172)
- [ ] `PairList.sort(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:189)
- [ ] `PairList.containsKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:195)
- [ ] `PairList.indexOfKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:199)
- [ ] `PairList.removeKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:203)
- [ ] `PairList.values(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:208)
- [ ] `PairList.keys(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:212)
- [ ] `PairList.getFirst(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:216)
- [ ] `PairList.getLast(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:220)
- [ ] `PairList.getValue(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:224)
- [ ] `PairList.getPair(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:230)
- [ ] `PairList.setKey(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/PairList.java:236)

### `SingletonList`: it's generic

- [x] `SingletonList.serialVersionUID` (team/creative/creativecore/common/util/type/list/SingletonList.java:16)
- [ ] `SingletonList.SingletonList(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/SingletonList.java:19)
- [ ] `SingletonList.setElement(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/SingletonList.java:23)
- [ ] `SingletonList.iterator(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/SingletonList.java:28)
- [ ] `SingletonList.size(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/SingletonList.java:33)
- [ ] `SingletonList.contains(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/SingletonList.java:38)
- [ ] `SingletonList.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/SingletonList.java:43)
- [ ] `SingletonList.forEach(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/SingletonList.java:52)
- [ ] `SingletonList.removeIf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/SingletonList.java:57)
- [ ] `SingletonList.replaceAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/SingletonList.java:62)
- [ ] `SingletonList.sort(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/SingletonList.java:67)
- [ ] `SingletonList.spliterator(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/type/list/SingletonList.java:71)

### `SortingBlockList`: it uses RegistryObjectListConfig, which isn't translated (it's generic)

- [ ] `SortingBlockList.add(1)`: its class isn't translated (it uses RegistryObjectListConfig, which isn't translated (it's generic)) (team/creative/creativecore/common/util/type/list/SortingBlockList.java:18)
- [ ] `SortingBlockList.add(1)`: its class isn't translated (it uses RegistryObjectListConfig, which isn't translated (it's generic)) (team/creative/creativecore/common/util/type/list/SortingBlockList.java:23)
- [ ] `SortingBlockList.is(1)`: its class isn't translated (it uses RegistryObjectListConfig, which isn't translated (it's generic)) (team/creative/creativecore/common/util/type/list/SortingBlockList.java:28)
- [ ] `SortingBlockList.is(1)`: its class isn't translated (it uses RegistryObjectListConfig, which isn't translated (it's generic)) (team/creative/creativecore/common/util/type/list/SortingBlockList.java:32)

### `SortingList`

- [ ] `SortingList.SortingList(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/list/SortingList.java:24)
- [ ] `SortingList.SortingList(0)`: it calls a constructor that isn't translated (team/creative/creativecore/common/util/type/list/SortingList.java:29)
- [ ] `SortingList.SortingList(1)`: its constructors are overloaded with the same number of parameters (team/creative/creativecore/common/util/type/list/SortingList.java:33)
- [x] `SortingList.isBlacklist(0)` (team/creative/creativecore/common/util/type/list/SortingList.java:37)
- [x] `SortingList.isWhitelist(0)` (team/creative/creativecore/common/util/type/list/SortingList.java:41)
- [x] `SortingList.setListType(1)` (team/creative/creativecore/common/util/type/list/SortingList.java:45)
- [x] `SortingList.setWhitelist(0)` (team/creative/creativecore/common/util/type/list/SortingList.java:49)
- [x] `SortingList.setBlacklist(0)` (team/creative/creativecore/common/util/type/list/SortingList.java:53)
- [ ] `SortingList.addSortingObjects(1)`: it uses `Object` (team/creative/creativecore/common/util/type/list/SortingList.java:57)
- [ ] `SortingList.addSortingObject(1)`: it uses `Object` (team/creative/creativecore/common/util/type/list/SortingList.java:63)
- [ ] `SortingList.addSortingByBlock(1)`: it uses Minecraft's `Block` (team/creative/creativecore/common/util/type/list/SortingList.java:74)
- [ ] `SortingList.addSortingByItem(1)`: it uses Minecraft's `Item` (team/creative/creativecore/common/util/type/list/SortingList.java:78)
- [ ] `SortingList.addSortingByItemStack(1)`: it uses Minecraft's `ItemStack` (team/creative/creativecore/common/util/type/list/SortingList.java:82)
- [ ] `SortingList.addSortingByBlockTag(1)`: it uses Minecraft's `TagKey` (team/creative/creativecore/common/util/type/list/SortingList.java:86)
- [ ] `SortingList.addSortingByItemTag(1)`: it uses Minecraft's `TagKey` (team/creative/creativecore/common/util/type/list/SortingList.java:90)
- [ ] `SortingList.canBeFoundInList(1)`: it uses `Object` (team/creative/creativecore/common/util/type/list/SortingList.java:94)
- [ ] `SortingList.canBeFoundInList(2)`: it uses Minecraft's `Level` (team/creative/creativecore/common/util/type/list/SortingList.java:109)
- [ ] `SortingList.canPass(1)`: it uses `Object` (team/creative/creativecore/common/util/type/list/SortingList.java:119)
- [ ] `SortingList.canPass(2)`: it uses Minecraft's `Level` (team/creative/creativecore/common/util/type/list/SortingList.java:123)

### `Tuple`: it's generic

- [ ] `Tuple.Tuple(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Tuple.java:9)
- [ ] `Tuple.getKey(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Tuple.java:14)
- [ ] `Tuple.getValue(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Tuple.java:19)
- [ ] `Tuple.setValue(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Tuple.java:24)
- [ ] `Tuple.hashCode(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Tuple.java:30)
- [ ] `Tuple.is(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Tuple.java:35)
- [ ] `Tuple.equals(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Tuple.java:39)
- [ ] `Tuple.toString(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/Tuple.java:44)

### `TupleList`: it's generic

- [ ] `TupleList.TupleList(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/TupleList.java:9)
- [ ] `TupleList.TupleList(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/TupleList.java:12)
- [ ] `TupleList.TupleList(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/TupleList.java:16)
- [ ] `TupleList.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/TupleList.java:20)
- [ ] `TupleList.containsKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/TupleList.java:24)
- [ ] `TupleList.indexOfKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/TupleList.java:28)
- [ ] `TupleList.values(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/TupleList.java:38)
- [ ] `TupleList.keys(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/TupleList.java:42)
- [ ] `TupleList.findValue(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/TupleList.java:46)
- [ ] `TupleList.findTuple(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/list/TupleList.java:52)

### `HashMapDouble`: it's generic

- [ ] `HashMapDouble.HashMapDouble(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapDouble.java:9)
- [ ] `HashMapDouble.HashMapDouble(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapDouble.java:12)
- [ ] `HashMapDouble.scale(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapDouble.java:16)
- [ ] `HashMapDouble.put(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapDouble.java:22)
- [ ] `HashMapDouble.putAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapDouble.java:33)
- [ ] `HashMapDouble.toString(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapDouble.java:40)
- [ ] `HashMapDouble.toString(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapDouble.java:59)

### `HashMapFloat`: it's generic

- [ ] `HashMapFloat.HashMapFloat(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapFloat.java:9)
- [ ] `HashMapFloat.HashMapFloat(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapFloat.java:12)
- [ ] `HashMapFloat.scale(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapFloat.java:16)
- [ ] `HashMapFloat.put(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapFloat.java:22)
- [ ] `HashMapFloat.putAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapFloat.java:33)
- [ ] `HashMapFloat.toString(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapFloat.java:40)
- [ ] `HashMapFloat.toString(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapFloat.java:59)

### `HashMapInteger`: it's generic

- [ ] `HashMapInteger.HashMapInteger(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapInteger.java:8)
- [ ] `HashMapInteger.HashMapInteger(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapInteger.java:11)
- [ ] `HashMapInteger.scale(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapInteger.java:15)
- [ ] `HashMapInteger.put(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapInteger.java:21)
- [ ] `HashMapInteger.putAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapInteger.java:32)

### `HashMapList`: it's generic

- [ ] `HashMapList.HashMapList(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:16)
- [ ] `HashMapList.HashMapList(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:19)
- [ ] `HashMapList.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:27)
- [ ] `HashMapList.tryGet(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:31)
- [ ] `HashMapList.keyOf(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:36)
- [ ] `HashMapList.keySet(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:46)
- [ ] `HashMapList.values(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:50)
- [ ] `HashMapList.entrySet(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:54)
- [ ] `HashMapList.containsKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:58)
- [ ] `HashMapList.contains(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:62)
- [ ] `HashMapList.contains(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:72)
- [ ] `HashMapList.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:77)
- [ ] `HashMapList.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:81)
- [ ] `HashMapList.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:90)
- [ ] `HashMapList.removeKey(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:101)
- [ ] `HashMapList.removeValue(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:105)
- [ ] `HashMapList.removeValue(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:118)
- [ ] `HashMapList.sizeOfValues(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:135)
- [ ] `HashMapList.size(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:145)
- [ ] `HashMapList.clear(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:149)
- [ ] `HashMapList.toString(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:153)
- [ ] `HashMapList.isEmpty(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:158)
- [ ] `HashMapList.getFirst(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/HashMapList.java:162)
- [ ] `HashMapList.iterator(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/type/map/HashMapList.java:174)

### `LinkedHashMapDouble`: it's generic

- [ ] `LinkedHashMapDouble.LinkedHashMapDouble(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapDouble.java:9)
- [ ] `LinkedHashMapDouble.LinkedHashMapDouble(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapDouble.java:12)
- [ ] `LinkedHashMapDouble.put(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapDouble.java:16)
- [ ] `LinkedHashMapDouble.putAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapDouble.java:27)
- [ ] `LinkedHashMapDouble.toString(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapDouble.java:34)
- [ ] `LinkedHashMapDouble.toString(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapDouble.java:53)

### `LinkedHashMapFloat`: it's generic

- [ ] `LinkedHashMapFloat.LinkedHashMapFloat(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapFloat.java:9)
- [ ] `LinkedHashMapFloat.LinkedHashMapFloat(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapFloat.java:12)
- [ ] `LinkedHashMapFloat.put(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapFloat.java:16)
- [ ] `LinkedHashMapFloat.putAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapFloat.java:27)
- [ ] `LinkedHashMapFloat.toString(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapFloat.java:34)
- [ ] `LinkedHashMapFloat.toString(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapFloat.java:53)

### `LinkedHashMapInteger`: it's generic

- [ ] `LinkedHashMapInteger.LinkedHashMapInteger(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapInteger.java:8)
- [ ] `LinkedHashMapInteger.LinkedHashMapInteger(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapInteger.java:11)
- [ ] `LinkedHashMapInteger.put(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapInteger.java:15)
- [ ] `LinkedHashMapInteger.putAll(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/map/LinkedHashMapInteger.java:26)

### `RunnableReturn`: it's an interface

- [ ] `RunnableReturn.run(0)`: it has no body (team/creative/creativecore/common/util/type/RunnableReturn.java:5)

### `CubeBitSet`: it implements `Iterable`

- [x] `CubeBitSet.CHUNK_SIZE` (team/creative/creativecore/common/util/type/set/CubeBitSet.java:12)
- [x] `CubeBitSet.CHUNK_SIZE_SQUARE` (team/creative/creativecore/common/util/type/set/CubeBitSet.java:13)
- [x] `CubeBitSet.CHUNK_BITS` (team/creative/creativecore/common/util/type/set/CubeBitSet.java:14)
- [x] `CubeBitSet.chunkIndex(1)` (team/creative/creativecore/common/util/type/set/CubeBitSet.java:21)
- [x] `CubeBitSet.index(3)` (team/creative/creativecore/common/util/type/set/CubeBitSet.java:25)
- [ ] `CubeBitSet.load(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:29)
- [ ] `CubeBitSet.save(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:55)
- [ ] `CubeBitSet.init(3)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:78)
- [ ] `CubeBitSet.ensureCapacity(3)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:89)
- [ ] `CubeBitSet.flip(3)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:173)
- [ ] `CubeBitSet.set(3)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:193)
- [ ] `CubeBitSet.set(4)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:210)
- [ ] `CubeBitSet.clear(3)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:218)
- [ ] `CubeBitSet.clearIncludingSize(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:235)
- [ ] `CubeBitSet.clear(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:242)
- [ ] `CubeBitSet.get(3)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:258)
- [ ] `CubeBitSet.isEmpty(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:287)
- [ ] `CubeBitSet.count(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:291)
- [ ] `CubeBitSet.iterator(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/type/set/CubeBitSet.java:295)
- [ ] `CubeBitSet.toString(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/CubeBitSet.java:349)

### `LineBitSet`: it implements `Iterable`

- [x] `LineBitSet.CHUNK_SIZE` (team/creative/creativecore/common/util/type/set/LineBitSet.java:9)
- [x] `LineBitSet.CHUNK_BITS` (team/creative/creativecore/common/util/type/set/LineBitSet.java:10)
- [x] `LineBitSet.chunkIndex(1)` (team/creative/creativecore/common/util/type/set/LineBitSet.java:15)
- [ ] `LineBitSet.load(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:19)
- [ ] `LineBitSet.save(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:34)
- [ ] `LineBitSet.init(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:45)
- [ ] `LineBitSet.ensureCapacity(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:54)
- [ ] `LineBitSet.flip(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:73)
- [ ] `LineBitSet.set(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:87)
- [ ] `LineBitSet.set(2)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:98)
- [ ] `LineBitSet.clear(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:106)
- [ ] `LineBitSet.clearIncludingSize(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:117)
- [ ] `LineBitSet.clear(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:123)
- [ ] `LineBitSet.get(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:132)
- [ ] `LineBitSet.isEmpty(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:147)
- [ ] `LineBitSet.count(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:151)
- [ ] `LineBitSet.iterator(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/type/set/LineBitSet.java:155)
- [ ] `LineBitSet.toString(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/LineBitSet.java:191)

### `QuadBitSet`: it implements `Iterable`

- [x] `QuadBitSet.CHUNK_SIZE` (team/creative/creativecore/common/util/type/set/QuadBitSet.java:12)
- [x] `QuadBitSet.CHUNK_BITS` (team/creative/creativecore/common/util/type/set/QuadBitSet.java:13)
- [x] `QuadBitSet.chunkIndex(1)` (team/creative/creativecore/common/util/type/set/QuadBitSet.java:19)
- [x] `QuadBitSet.index(2)` (team/creative/creativecore/common/util/type/set/QuadBitSet.java:23)
- [ ] `QuadBitSet.load(1)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:27)
- [ ] `QuadBitSet.save(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:48)
- [ ] `QuadBitSet.init(2)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:65)
- [ ] `QuadBitSet.ensureCapacity(2)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:75)
- [ ] `QuadBitSet.flip(2)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:124)
- [ ] `QuadBitSet.set(2)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:141)
- [ ] `QuadBitSet.set(3)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:155)
- [ ] `QuadBitSet.clear(2)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:163)
- [ ] `QuadBitSet.clearIncludingSize(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:177)
- [ ] `QuadBitSet.clear(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:184)
- [ ] `QuadBitSet.get(2)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:196)
- [ ] `QuadBitSet.isEmpty(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:218)
- [ ] `QuadBitSet.count(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:222)
- [ ] `QuadBitSet.iterator(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/type/set/QuadBitSet.java:226)
- [ ] `QuadBitSet.toString(0)`: its class isn't translated (it implements `Iterable`) (team/creative/creativecore/common/util/type/set/QuadBitSet.java:269)

### `CheckTree`: it's generic

- [ ] `CheckTree.CheckTree(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/CheckTree.java:15)
- [ ] `CheckTree.CheckTree(4)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/CheckTree.java:22)
- [ ] `CheckTree.reload(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/CheckTree.java:29)
- [ ] `CheckTree.reload(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/CheckTree.java:33)
- [ ] `CheckTree.apply(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/CheckTree.java:45)
- [ ] `CheckTree.apply(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/CheckTree.java:49)

### `CheckTree.CheckTreeEntry`: it holds an object of its own class (a linked structure)

- [ ] `CheckTree.CheckTreeEntry.CheckTreeEntry(1)`: its body couldn't be read: an inner class creation (team/creative/creativecore/common/util/type/tree/CheckTree.java:67)
- [ ] `CheckTree.CheckTreeEntry.CheckTreeEntry(2)`: its body couldn't be read: an inner class creation (team/creative/creativecore/common/util/type/tree/CheckTree.java:78)
- [ ] `CheckTree.CheckTreeEntry.isEnabled(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (team/creative/creativecore/common/util/type/tree/CheckTree.java:95)
- [ ] `CheckTree.CheckTreeEntry.isChildEnabled(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (team/creative/creativecore/common/util/type/tree/CheckTree.java:113)
- [ ] `CheckTree.CheckTreeEntry.enable(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (team/creative/creativecore/common/util/type/tree/CheckTree.java:125)
- [ ] `CheckTree.CheckTreeEntry.disable(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (team/creative/creativecore/common/util/type/tree/CheckTree.java:132)
- [ ] `CheckTree.CheckTreeEntry.enableInteral(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (team/creative/creativecore/common/util/type/tree/CheckTree.java:139)
- [ ] `CheckTree.CheckTreeEntry.disableInteral(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (team/creative/creativecore/common/util/type/tree/CheckTree.java:149)
- [ ] `CheckTree.CheckTreeEntry.toggle(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (team/creative/creativecore/common/util/type/tree/CheckTree.java:159)

### `NamedTree`: it's generic

- [ ] `NamedTree.NamedTree(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:21)
- [ ] `NamedTree.NamedTree(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:26)
- [ ] `NamedTree.name(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:31)
- [ ] `NamedTree.parent(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:35)
- [ ] `NamedTree.add(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:40)
- [ ] `NamedTree.children(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:46)
- [ ] `NamedTree.entries(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:50)
- [ ] `NamedTree.keySet(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:54)
- [ ] `NamedTree.get(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:58)
- [ ] `NamedTree.folder(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:63)
- [ ] `NamedTree.folder(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:67)
- [ ] `NamedTree.folderForce(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:77)
- [ ] `NamedTree.folderForce(2)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:81)
- [ ] `NamedTree.folderForce(3)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:85)
- [ ] `NamedTree.path(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:96)
- [ ] `NamedTree.findPath(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:108)
- [ ] `NamedTree.toString(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:123)
- [ ] `NamedTree.allValues(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:128)
- [ ] `NamedTree.allNames(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:134)
- [ ] `NamedTree.collect(1)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:138)
- [ ] `NamedTree.all(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:148)
- [ ] `NamedTree.first(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:154)
- [ ] `NamedTree.firstKey(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:169)
- [ ] `NamedTree.hasChildren(0)`: its class isn't translated (it's generic) (team/creative/creativecore/common/util/type/tree/NamedTree.java:173)

### `TriPredicate`: it's an interface

- [ ] `TriPredicate.test(3)`: it has no body (team/creative/creativecore/common/util/type/TriPredicate.java:5)

### `CreativeHackery`

- [ ] `CreativeHackery.UNSAFE`: it uses `Unsafe` (sun.misc) (team/creative/creativecore/common/util/unsafe/CreativeHackery.java:10)
- [ ] `CreativeHackery.getUnsafe(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/common/util/unsafe/CreativeHackery.java:12)
- [ ] `CreativeHackery.allocateInstance(1)`: it's generic (team/creative/creativecore/common/util/unsafe/CreativeHackery.java:38)

### `CreativeCore`: it implements Fabric's `ModInitializer`

- [ ] `CreativeCore.LOADER`: it uses ICreativeLoader, which isn't translated (it's an interface) (team/creative/creativecore/CreativeCore.java:40)
- [ ] `CreativeCore.UTILS`: it uses ILoaderUtils, which isn't translated (it's an interface) (team/creative/creativecore/CreativeCore.java:41)
- [x] `CreativeCore.MODID` (team/creative/creativecore/CreativeCore.java:42)
- [x] `CreativeCore.LOGGER` (team/creative/creativecore/CreativeCore.java:43)
- [ ] `CreativeCore.NETWORK`: it uses CreativeNetwork, which isn't translated (it uses `Class`) (team/creative/creativecore/CreativeCore.java:44)
- [ ] `CreativeCore.CONFIG`: it uses CreativeCoreConfig, which isn't translated (it uses NamedList, which isn't translated (it's generic)) (team/creative/creativecore/CreativeCore.java:45)
- [ ] `CreativeCore.CONFIG_OPEN`: it uses GuiCreatorBasic, which isn't translated (it extends `GuiCreator`) (team/creative/creativecore/CreativeCore.java:46)
- [ ] `CreativeCore.CONFIG_CLIENT_OPEN`: it uses GuiCreatorBasic, which isn't translated (it extends `GuiCreator`) (team/creative/creativecore/CreativeCore.java:49)
- [ ] `CreativeCore.CONFIG_CLIENT_SYNC_OPEN`: it uses GuiCreatorBasic, which isn't translated (it extends `GuiCreator`) (team/creative/creativecore/CreativeCore.java:53)
- [ ] `CreativeCore.CONFIG_HANDLER`: it uses ConfigEventHandler, which isn't translated (it uses `DecimalFormat` (java.text)) (team/creative/creativecore/CreativeCore.java:56)
- [ ] `CreativeCore.GUI_CONTAINER`: it uses Minecraft's `MenuType` (team/creative/creativecore/CreativeCore.java:57)
- [ ] `CreativeCore.CreativeCore(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (team/creative/creativecore/CreativeCore.java:59)
- [ ] `CreativeCore.server(1)`: its class isn't translated (it implements Fabric's `ModInitializer`) (team/creative/creativecore/CreativeCore.java:64)
- [ ] `CreativeCore.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (team/creative/creativecore/CreativeCore.java:71)
- [ ] `CreativeCore.loader(0)`: it uses ICreativeLoader, which isn't translated (it's an interface) (team/creative/creativecore/CreativeCore.java:87)
- [ ] `CreativeCore.utils(0)`: it uses ILoaderUtils, which isn't translated (it's an interface) (team/creative/creativecore/CreativeCore.java:91)

### `CreativeCoreConfig`: it uses NamedList, which isn't translated (it's generic)

- [ ] `CreativeCoreConfig.CreativeCoreConfig(0)`: its class isn't translated (it uses NamedList, which isn't translated (it's generic)) (team/creative/creativecore/CreativeCoreConfig.java:17)
- [ ] `CreativeCoreConfig.is(2)`: its class isn't translated (it uses NamedList, which isn't translated (it's generic)) (team/creative/creativecore/CreativeCoreConfig.java:23)

### `CreativeCoreGuiRegistry`

- [ ] `CreativeCoreGuiRegistry.HAND`: it uses GuiSyncGlobal, which isn't translated (it's generic) (team/creative/creativecore/CreativeCoreGuiRegistry.java:24)
- [ ] `CreativeCoreGuiRegistry.DROP`: it uses GuiSyncGlobal, which isn't translated (it's generic) (team/creative/creativecore/CreativeCoreGuiRegistry.java:26)
- [ ] `CreativeCoreGuiRegistry.DROP_HAND`: it uses GuiSyncGlobal, which isn't translated (it's generic) (team/creative/creativecore/CreativeCoreGuiRegistry.java:37)
- [ ] `CreativeCoreGuiRegistry.INSERT`: it uses GuiSyncGlobal, which isn't translated (it's generic) (team/creative/creativecore/CreativeCoreGuiRegistry.java:45)
- [ ] `CreativeCoreGuiRegistry.EXTRACT`: it uses GuiSyncGlobal, which isn't translated (it's generic) (team/creative/creativecore/CreativeCoreGuiRegistry.java:68)
- [ ] `CreativeCoreGuiRegistry.DUPLICATE`: it uses GuiSyncGlobal, which isn't translated (it's generic) (team/creative/creativecore/CreativeCoreGuiRegistry.java:90)
- [ ] `CreativeCoreGuiRegistry.SWAP`: it uses GuiSyncGlobal, which isn't translated (it's generic) (team/creative/creativecore/CreativeCoreGuiRegistry.java:99)
- [ ] `CreativeCoreGuiRegistry.SPREAD`: it uses GuiSyncGlobal, which isn't translated (it's generic) (team/creative/creativecore/CreativeCoreGuiRegistry.java:121)
- [x] `CreativeCoreGuiRegistry.init(0)` (team/creative/creativecore/CreativeCoreGuiRegistry.java:184)

### `CreativeFabricLoader`: it implements `ICreativeLoader`

- [ ] `CreativeFabricLoader.currentServer`: it uses Minecraft's `MinecraftServer` (team/creative/creativecore/CreativeFabricLoader.java:81)
- [ ] `CreativeFabricLoader.FINISH_CONSUMING`: it uses Fabric's `Event` (team/creative/creativecore/CreativeFabricLoader.java:82)
- [ ] `CreativeFabricLoader.register(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:92)
- [ ] `CreativeFabricLoader.registerClient(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:97)
- [ ] `CreativeFabricLoader.registerClientTick(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:104)
- [ ] `CreativeFabricLoader.registerHudElement(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:109)
- [ ] `CreativeFabricLoader.registerModifyCreativeTab(2)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:118)
- [ ] `CreativeFabricLoader.registerClientRenderGui(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:123)
- [ ] `CreativeFabricLoader.registerClientRenderStart(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:130)
- [ ] `CreativeFabricLoader.registerReloadListener(2)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:135)
- [ ] `CreativeFabricLoader.registerLevelTick(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:144)
- [ ] `CreativeFabricLoader.registerLevelTickStart(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:149)
- [ ] `CreativeFabricLoader.registerLoadLevel(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:154)
- [ ] `CreativeFabricLoader.registerUnloadLevel(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:159)
- [ ] `CreativeFabricLoader.registerListener(1)`: it's generic (team/creative/creativecore/CreativeFabricLoader.java:164)
- [ ] `CreativeFabricLoader.getFluidViscosityMultiplier(2)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:168)
- [ ] `CreativeFabricLoader.getFriction(3)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:173)
- [ ] `CreativeFabricLoader.registerClientStarted(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:178)
- [ ] `CreativeFabricLoader.postForge(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:183)
- [ ] `CreativeFabricLoader.isModLoaded(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:187)
- [ ] `CreativeFabricLoader.getOverallSide(0)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:192)
- [ ] `CreativeFabricLoader.getEffectiveSide(0)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:197)
- [ ] `CreativeFabricLoader.forge(0)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:202)
- [ ] `CreativeFabricLoader.fabric(0)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:207)
- [ ] `CreativeFabricLoader.getCurrentServer(0)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:212)
- [ ] `CreativeFabricLoader.registerPlayerJoin(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:217)
- [ ] `CreativeFabricLoader.registerPlayerCopy(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:222)
- [ ] `CreativeFabricLoader.registerPlayerDimensionChange(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:227)
- [ ] `CreativeFabricLoader.registerPlayerRespawn(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:233)
- [ ] `CreativeFabricLoader.registerItemUsed(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:238)
- [ ] `CreativeFabricLoader.publishItemUsed(2)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:243)
- [ ] `CreativeFabricLoader.registerAttachment(3)`: its body couldn't be read: an anonymous class (team/creative/creativecore/CreativeFabricLoader.java:248)
- [ ] `CreativeFabricLoader.registerRemoveEffectCallback(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:264)
- [ ] `CreativeFabricLoader.openTransaction(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:269)
- [ ] `CreativeFabricLoader.registerItemStorage(2)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:274)
- [ ] `CreativeFabricLoader.getItemStorage(2)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:282)
- [ ] `CreativeFabricLoader.getItemStorage(3)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:288)
- [ ] `CreativeFabricLoader.register(1)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:294)
- [ ] `CreativeFabricLoader.onItemToss(2)`: its class isn't translated (it implements `ICreativeLoader`) (team/creative/creativecore/CreativeFabricLoader.java:304)

### `CreativeFabricLoader.CreativeHudElement`: it implements Fabric's `HudElement`

- [ ] `CreativeFabricLoader.CreativeHudElement.extractRenderState(2)`: its class isn't translated (it implements Fabric's `HudElement`) (team/creative/creativecore/CreativeFabricLoader.java:316)

### `CreativeFabricLoader.FabricItemResource`: it implements `CommonItemResource`

- [ ] `CreativeFabricLoader.FabricItemResource.FabricItemResource(1)`: its class isn't translated (it implements `CommonItemResource`) (team/creative/creativecore/CreativeFabricLoader.java:326)
- [ ] `CreativeFabricLoader.FabricItemResource.isEmpty(0)`: its class isn't translated (it implements `CommonItemResource`) (team/creative/creativecore/CreativeFabricLoader.java:330)
- [ ] `CreativeFabricLoader.FabricItemResource.getMaxStackSize(0)`: its class isn't translated (it implements `CommonItemResource`) (team/creative/creativecore/CreativeFabricLoader.java:335)
- [ ] `CreativeFabricLoader.FabricItemResource.getItem(0)`: its class isn't translated (it implements `CommonItemResource`) (team/creative/creativecore/CreativeFabricLoader.java:340)
- [ ] `CreativeFabricLoader.FabricItemResource.toStack(1)`: its class isn't translated (it implements `CommonItemResource`) (team/creative/creativecore/CreativeFabricLoader.java:345)
- [ ] `CreativeFabricLoader.FabricItemResource.toStack(0)`: its class isn't translated (it implements `CommonItemResource`) (team/creative/creativecore/CreativeFabricLoader.java:350)
- [ ] `CreativeFabricLoader.FabricItemResource.typeHolder(0)`: its class isn't translated (it implements `CommonItemResource`) (team/creative/creativecore/CreativeFabricLoader.java:355)
- [ ] `CreativeFabricLoader.FabricItemResource.getComponents(0)`: its class isn't translated (it implements `CommonItemResource`) (team/creative/creativecore/CreativeFabricLoader.java:359)

### `CreativeFabricLoader.FabricItemStorage`: it implements `CommonItemStorage`

- [ ] `CreativeFabricLoader.FabricItemStorage.FabricItemStorage(1)`: its class isn't translated (it implements `CommonItemStorage`) (team/creative/creativecore/CreativeFabricLoader.java:367)
- [ ] `CreativeFabricLoader.FabricItemStorage.insert(3)`: its class isn't translated (it implements `CommonItemStorage`) (team/creative/creativecore/CreativeFabricLoader.java:371)
- [ ] `CreativeFabricLoader.FabricItemStorage.extract(3)`: its class isn't translated (it implements `CommonItemStorage`) (team/creative/creativecore/CreativeFabricLoader.java:377)
- [ ] `CreativeFabricLoader.FabricItemStorage.iterator(0)`: its body couldn't be read: an anonymous class (team/creative/creativecore/CreativeFabricLoader.java:383)

### `CreativeFabricLoader.FabricItemStorageView`: it implements `CommonItemStorageView`

- [ ] `CreativeFabricLoader.FabricItemStorageView.FabricItemStorageView(2)`: its class isn't translated (it implements `CommonItemStorageView`) (team/creative/creativecore/CreativeFabricLoader.java:404)
- [ ] `CreativeFabricLoader.FabricItemStorageView.getCapacityAsInt(0)`: its class isn't translated (it implements `CommonItemStorageView`) (team/creative/creativecore/CreativeFabricLoader.java:409)
- [ ] `CreativeFabricLoader.FabricItemStorageView.getResource(0)`: its class isn't translated (it implements `CommonItemStorageView`) (team/creative/creativecore/CreativeFabricLoader.java:414)
- [ ] `CreativeFabricLoader.FabricItemStorageView.getAmountAsInt(0)`: its class isn't translated (it implements `CommonItemStorageView`) (team/creative/creativecore/CreativeFabricLoader.java:419)
- [ ] `CreativeFabricLoader.FabricItemStorageView.insert(3)`: its class isn't translated (it implements `CommonItemStorageView`) (team/creative/creativecore/CreativeFabricLoader.java:424)
- [ ] `CreativeFabricLoader.FabricItemStorageView.extract(3)`: its class isn't translated (it implements `CommonItemStorageView`) (team/creative/creativecore/CreativeFabricLoader.java:434)

### `CreativeFabricLoader.FabricTransaction`: it implements `CommonTransaction`

- [ ] `CreativeFabricLoader.FabricTransaction.FabricTransaction(1)`: its class isn't translated (it implements `CommonTransaction`) (team/creative/creativecore/CreativeFabricLoader.java:444)
- [ ] `CreativeFabricLoader.FabricTransaction.commit(0)`: its class isn't translated (it implements `CommonTransaction`) (team/creative/creativecore/CreativeFabricLoader.java:452)
- [ ] `CreativeFabricLoader.FabricTransaction.close(0)`: its class isn't translated (it implements `CommonTransaction`) (team/creative/creativecore/CreativeFabricLoader.java:457)

### `ICreativeAttachmentType`: it's an interface

- [ ] `ICreativeAttachmentType.get(1)`: it has no body (team/creative/creativecore/ICreativeAttachmentType.java:6)
- [ ] `ICreativeAttachmentType.set(2)`: it has no body (team/creative/creativecore/ICreativeAttachmentType.java:8)

### `ICreativeLoader`: it's an interface

- [ ] `ICreativeLoader.loadCommon(0)`: its class isn't translated (it's an interface) (team/creative/creativecore/ICreativeLoader.java:38)
- [ ] `ICreativeLoader.getCurrentServer(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:43)
- [ ] `ICreativeLoader.forge(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:45)
- [ ] `ICreativeLoader.fabric(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:47)
- [ ] `ICreativeLoader.getOverallSide(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:49)
- [ ] `ICreativeLoader.getEffectiveSide(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:51)
- [ ] `ICreativeLoader.register(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:53)
- [ ] `ICreativeLoader.registerClient(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:55)
- [ ] `ICreativeLoader.registerClientTick(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:57)
- [ ] `ICreativeLoader.registerClientRenderStart(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:59)
- [ ] `ICreativeLoader.registerClientRenderGui(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:61)
- [ ] `ICreativeLoader.registerClientStarted(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:63)
- [ ] `ICreativeLoader.registerReloadListener(2)`: it has no body (team/creative/creativecore/ICreativeLoader.java:65)
- [ ] `ICreativeLoader.registerLevelTick(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:67)
- [ ] `ICreativeLoader.registerLevelTickStart(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:69)
- [ ] `ICreativeLoader.registerLoadLevel(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:71)
- [ ] `ICreativeLoader.registerUnloadLevel(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:73)
- [ ] `ICreativeLoader.registerModifyCreativeTab(2)`: it has no body (team/creative/creativecore/ICreativeLoader.java:75)
- [ ] `ICreativeLoader.registerListener(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:77)
- [ ] `ICreativeLoader.getFluidViscosityMultiplier(2)`: it has no body (team/creative/creativecore/ICreativeLoader.java:79)
- [ ] `ICreativeLoader.getFriction(3)`: it has no body (team/creative/creativecore/ICreativeLoader.java:81)
- [ ] `ICreativeLoader.postForge(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:83)
- [ ] `ICreativeLoader.registerPlayerJoin(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:85)
- [ ] `ICreativeLoader.registerPlayerCopy(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:87)
- [ ] `ICreativeLoader.registerAttachment(3)`: it has no body (team/creative/creativecore/ICreativeLoader.java:89)
- [ ] `ICreativeLoader.openTransaction(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:91)
- [ ] `ICreativeLoader.registerItemStorage(2)`: it has no body (team/creative/creativecore/ICreativeLoader.java:93)
- [ ] `ICreativeLoader.getItemStorage(2)`: it has no body (team/creative/creativecore/ICreativeLoader.java:95)
- [ ] `ICreativeLoader.getItemStorage(3)`: it has no body (team/creative/creativecore/ICreativeLoader.java:97)
- [ ] `ICreativeLoader.registerRemoveEffectCallback(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:99)
- [ ] `ICreativeLoader.publishItemUsed(2)`: it has no body (team/creative/creativecore/ICreativeLoader.java:101)
- [ ] `ICreativeLoader.registerItemUsed(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:103)
- [ ] `ICreativeLoader.registerPlayerRespawn(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:105)
- [ ] `ICreativeLoader.registerPlayerDimensionChange(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:107)
- [ ] `ICreativeLoader.onItemToss(2)`: it has no body (team/creative/creativecore/ICreativeLoader.java:109)
- [ ] `ICreativeLoader.register(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:111)
- [ ] `ICreativeLoader.isModLoaded(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:113)

### `ICreativeLoader.CommonItemResource`: it's an interface

- [ ] `ICreativeLoader.CommonItemResource.isEmpty(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:116)
- [ ] `ICreativeLoader.CommonItemResource.getItem(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:118)
- [ ] `ICreativeLoader.CommonItemResource.getMaxStackSize(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:120)
- [ ] `ICreativeLoader.CommonItemResource.toStack(1)`: it has no body (team/creative/creativecore/ICreativeLoader.java:122)
- [ ] `ICreativeLoader.CommonItemResource.toStack(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:124)

### `ICreativeLoader.CommonItemStorage`: it's an interface

- [ ] `ICreativeLoader.CommonItemStorage.insert(3)`: it has no body (team/creative/creativecore/ICreativeLoader.java:128)
- [ ] `ICreativeLoader.CommonItemStorage.extract(3)`: it has no body (team/creative/creativecore/ICreativeLoader.java:130)

### `ICreativeLoader.CommonItemStorageView`: it's an interface

- [ ] `ICreativeLoader.CommonItemStorageView.getCapacityAsInt(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:134)
- [ ] `ICreativeLoader.CommonItemStorageView.getResource(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:136)
- [ ] `ICreativeLoader.CommonItemStorageView.getAmountAsInt(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:138)
- [ ] `ICreativeLoader.CommonItemStorageView.insert(3)`: it has no body (team/creative/creativecore/ICreativeLoader.java:140)
- [ ] `ICreativeLoader.CommonItemStorageView.extract(3)`: it has no body (team/creative/creativecore/ICreativeLoader.java:142)

### `ICreativeLoader.CommonTransaction`: it's an interface

- [ ] `ICreativeLoader.CommonTransaction.commit(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:146)
- [ ] `ICreativeLoader.CommonTransaction.close(0)`: it has no body (team/creative/creativecore/ICreativeLoader.java:148)

### `ICreativeLoader.CopyConsumer`: it's an interface

- [ ] `ICreativeLoader.CopyConsumer.accept(3)`: it has no body (team/creative/creativecore/ICreativeLoader.java:154)

### `ICreativeLoader.FinishConsuming`: it's an interface

- [ ] `ICreativeLoader.FinishConsuming.accept(2)`: it has no body (team/creative/creativecore/ICreativeLoader.java:159)

### `ICreativeLoader.PlayerDimensionChangeConsumer`: it's an interface

- [ ] `ICreativeLoader.PlayerDimensionChangeConsumer.accept(3)`: it has no body (team/creative/creativecore/ICreativeLoader.java:164)

### `ICreativeLoader.RemoveEffect`: it's an interface

- [ ] `ICreativeLoader.RemoveEffect.shouldCancel(2)`: it has no body (team/creative/creativecore/ICreativeLoader.java:169)
- [ ] `AbstractContainerScreenMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:13)
- [ ] `ArgumentTypeInfosAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:10)
- [ ] `ClientConfigurationPacketListenerImplMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:18)
- [ ] `ComponentSerializationMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:14)
- [ ] `ConnectionMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:13)
- [ ] `FilePackResourcesAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:12)
- [ ] `FixedPathPackResourcesAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:12)
- [ ] `GameRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:12)
- [ ] `GuiGraphicsExtractorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:23)
- [ ] `LivingEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:17)
- [ ] `ModNioResourcePackAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:11)
- [ ] `MouseHandlerAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:8)
- [ ] `PathPackResourcesAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:9)
- [ ] `PlayerListMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:16)
- [ ] `ScissorStackMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:12)
- [ ] `ServerCommonPacketListenerImplAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:9)
- [ ] `ServerConfigurationPacketListenerImplMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:18)
- [ ] `ShapesMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:20)
- [ ] `SharedZipFileAccessAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:9)
- [ ] `StringSplitterAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:9)
- [ ] `VoxelShapeAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/creativecore/ICreativeLoader.java:10)

### `ReflectionHelper`

- [ ] `ReflectionHelper.findFieldByType(2)`: it's generic (team/creative/creativecore/reflection/ReflectionHelper.java:8)
- [ ] `ReflectionHelper.findFieldByType(2)`: it's generic (team/creative/creativecore/reflection/ReflectionHelper.java:16)
- [ ] `ReflectionHelper.findField(3)`: it's generic (team/creative/creativecore/reflection/ReflectionHelper.java:34)
- [ ] `ReflectionHelper.findField(2)`: it's generic (team/creative/creativecore/reflection/ReflectionHelper.java:52)
- [ ] `ReflectionHelper.findMethod(3)`: it uses `Method` (java.lang.reflect) (team/creative/creativecore/reflection/ReflectionHelper.java:62)

### `GuiServerControl`: it's generic

- [ ] `GuiServerControl.GuiServerControl(1)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:16)
- [ ] `GuiServerControl.setVisible(1)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:20)
- [ ] `GuiServerControl.setFixed(0)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:24)
- [ ] `GuiServerControl.setFixedX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:28)
- [ ] `GuiServerControl.setFixedY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:32)
- [ ] `GuiServerControl.setExpandable(0)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:36)
- [ ] `GuiServerControl.setExpandableX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:40)
- [ ] `GuiServerControl.setExpandableY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:44)
- [ ] `GuiServerControl.setDim(2)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:48)
- [ ] `GuiServerControl.setDim(1)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:52)
- [ ] `GuiServerControl.setEnabled(1)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:56)
- [ ] `GuiServerControl.setTooltip(1)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:60)
- [ ] `GuiServerControl.setTooltip(1)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:64)
- [ ] `GuiServerControl.isExpandableX(0)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:68)
- [ ] `GuiServerControl.isExpandableY(0)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:73)
- [ ] `GuiServerControl.removeFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:78)
- [ ] `GuiServerControl.setFormatting(1)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:82)
- [ ] `GuiServerControl.setCustomFormatting(0)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:86)
- [ ] `GuiServerControl.playSound(1)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:91)
- [ ] `GuiServerControl.playSound(1)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:95)
- [ ] `GuiServerControl.playSound(3)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:99)
- [ ] `GuiServerControl.playSound(3)`: its class isn't translated (it's generic) (team/creative/creativecore/server/gui/GuiServerControl.java:103)

### `GuiServerManager`: it implements `GuiManagerDist` (team.creative.creativecore.common.gui.manager)


### `GuiServerRegistry`

- [ ] `GuiServerRegistry.CONTROL_FACTORY`: it uses `Object2ObjectMap` (it.unimi.dsi.fastutil.objects) (team/creative/creativecore/server/gui/registry/GuiServerRegistry.java:14)
- [ ] `GuiServerRegistry.CONTROL_SPECIAL_FACTORY`: it uses GuiControl, which isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/server/gui/registry/GuiServerRegistry.java:15)
- [ ] `GuiServerRegistry.MANAGER_FACTORY`: it uses `Object2ObjectMap` (it.unimi.dsi.fastutil.objects) (team/creative/creativecore/server/gui/registry/GuiServerRegistry.java:16)
- [ ] `GuiServerRegistry.create(1)`: it uses GuiServerManager, which isn't translated (it implements `GuiManagerDist` (team.creative.creativecore.common.gui.manager)) (team/creative/creativecore/server/gui/registry/GuiServerRegistry.java:18)
- [ ] `GuiServerRegistry.create(1)`: it uses GuiServerManager, which isn't translated (it implements `GuiManagerDist` (team.creative.creativecore.common.gui.manager)) (team/creative/creativecore/server/gui/registry/GuiServerRegistry.java:34)
- [ ] `GuiServerRegistry.register(2)`: it's generic (team/creative/creativecore/server/gui/registry/GuiServerRegistry.java:39)
- [ ] `GuiServerRegistry.registerManager(2)`: it's generic (team/creative/creativecore/server/gui/registry/GuiServerRegistry.java:43)
- [ ] `GuiServerRegistry.registerSpecial(1)`: it uses GuiControl, which isn't translated (it uses IGuiParent, which isn't translated (it's an interface)) (team/creative/creativecore/server/gui/registry/GuiServerRegistry.java:47)

### `Side`: its constants carry values

- [ ] `Side.isClient(0)`: it has no body (team/creative/creativecore/Side.java:27)
- [ ] `Side.isServer(0)`: it has no body (team/creative/creativecore/Side.java:29)
