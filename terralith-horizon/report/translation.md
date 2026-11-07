# Java translated to Rust

Translated 5 of 39 methods (1 more is done another way in Horizon) and 4 of 11 static fields to Rust; 0 of 1 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 27 | `ConfigListBuilder.build(1)`: its class isn't translated (it's an interface) |
| Other Java this translator doesn't handle yet | 4 | `ConfigListBuilder.addCategory(2)`: it has no body |
| Done another way in Horizon | 1 | `TerralithModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |
| Shares one object between names (Java references) | 1 | `ConfigState.ConfigState(2)`: it stores a second reference to an object (Rust would need it copied or shared) |
| Uses translated text (Component.translatable) | 1 | `TerralithConfigScreen.text(1)`: a translated text with a computed key |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| net/stardustlabs/terralith/TerralithFabric.java:9 | `init` | not translated: it calls ConfigHandler.load, which isn't translated |

## Every class


### `ConfigListBuilder`: it's an interface

- [ ] `ConfigListBuilder.addCategory(2)`: it has no body (net/stardustlabs/terralith/client/ConfigListBuilder.java:10)
- [ ] `ConfigListBuilder.addText(2)`: it has no body (net/stardustlabs/terralith/client/ConfigListBuilder.java:12)
- [ ] `ConfigListBuilder.addBoolean(4)`: it has no body (net/stardustlabs/terralith/client/ConfigListBuilder.java:14)
- [ ] `ConfigListBuilder.build(1)`: its class isn't translated (it's an interface) (net/stardustlabs/terralith/client/ConfigListBuilder.java:16)

### `TerralithModMenu`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `TerralithModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (net/stardustlabs/terralith/compat/TerralithModMenu.java:8)

### `ConfigResourceCondition`: it implements Fabric's `ResourceCondition`

- [ ] `ConfigResourceCondition.CODEC`: it uses Minecraft's `MapCodec` (net/stardustlabs/terralith/ConfigResourceCondition.java:12)
- [ ] `ConfigResourceCondition.TYPE`: it uses Fabric's `ResourceConditionType` (net/stardustlabs/terralith/ConfigResourceCondition.java:19)
- [ ] `ConfigResourceCondition.ConfigResourceCondition(2)`: its class isn't translated (it implements Fabric's `ResourceCondition`) (net/stardustlabs/terralith/ConfigResourceCondition.java:11)
- [ ] `ConfigResourceCondition.getType(0)`: its class isn't translated (it implements Fabric's `ResourceCondition`) (net/stardustlabs/terralith/ConfigResourceCondition.java:21)
- [ ] `ConfigResourceCondition.test(1)`: its class isn't translated (it implements Fabric's `ResourceCondition`) (net/stardustlabs/terralith/ConfigResourceCondition.java:25)
- [ ] `ConfigResourceCondition.key(0)`: its class isn't translated (it implements Fabric's `ResourceCondition`) (net/stardustlabs/terralith/ConfigResourceCondition.java:11)
- [ ] `ConfigResourceCondition.invert(0)`: its class isn't translated (it implements Fabric's `ResourceCondition`) (net/stardustlabs/terralith/ConfigResourceCondition.java:11)

### `ConfigHandler`

- [ ] `ConfigHandler.LOADED_STATE`: it uses ConfigState.DEFAULT_CONFIG, which isn't translated (it creates a ConfigState with a constructor that isn't translated) (net/stardustlabs/terralith/config/ConfigHandler.java:19)
- [ ] `ConfigHandler.PATH`: a Path with no value (null in Java) (net/stardustlabs/terralith/config/ConfigHandler.java:20)
- [ ] `ConfigHandler.getState(0)`: it uses ConfigHandler.LOADED_STATE, which isn't translated (it stores a second reference to an object (Rust would need it copied or shared)) (net/stardustlabs/terralith/config/ConfigHandler.java:22)
- [ ] `ConfigHandler.load(1)`: it uses ConfigHandler.PATH, which isn't translated (a Path with no value (null in Java)) (net/stardustlabs/terralith/config/ConfigHandler.java:26)
- [ ] `ConfigHandler.save(0)`: a catch block that uses its exception (net/stardustlabs/terralith/config/ConfigHandler.java:49)

### `ConfigState`

- [x] `ConfigState.DEFAULT_MODULES` (net/stardustlabs/terralith/config/ConfigState.java:11)
- [ ] `ConfigState.DEFAULT_CONFIG`: it creates a ConfigState with a constructor that isn't translated (net/stardustlabs/terralith/config/ConfigState.java:29)
- [ ] `ConfigState.CODEC`: it uses Minecraft's `Codec` (net/stardustlabs/terralith/config/ConfigState.java:30)
- [ ] `ConfigState.ConfigState(2)`: it stores a second reference to an object (Rust would need it copied or shared) (net/stardustlabs/terralith/config/ConfigState.java:40)
- [x] `ConfigState.test(1)` (net/stardustlabs/terralith/config/ConfigState.java:45)

### `ConfigState.Modules`

- [ ] `ConfigState.Modules.CODEC`: it uses Minecraft's `Codec` (net/stardustlabs/terralith/config/ConfigState.java:50)
- [x] `ConfigState.Modules.Modules(1)` (net/stardustlabs/terralith/config/ConfigState.java:54)
- [x] `ConfigState.Modules.validateAndCreate(1)` (net/stardustlabs/terralith/config/ConfigState.java:58)
- [ ] `ConfigState.Modules.setModule(2)`: it calls ConfigHandler.save, which isn't translated (net/stardustlabs/terralith/config/ConfigState.java:70)
- [x] `ConfigState.Modules.getModule(1)` (net/stardustlabs/terralith/config/ConfigState.java:75)

### `TerralithConfigList`: it extends Minecraft's `ContainerObjectSelectionList`

- [ ] `TerralithConfigList.TerralithConfigList(3)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (net/stardustlabs/terralith/gui/TerralithConfigList.java:25)
- [ ] `TerralithConfigList.addCategory(2)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (net/stardustlabs/terralith/gui/TerralithConfigList.java:29)
- [ ] `TerralithConfigList.addText(2)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (net/stardustlabs/terralith/gui/TerralithConfigList.java:34)
- [ ] `TerralithConfigList.addBoolean(4)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (net/stardustlabs/terralith/gui/TerralithConfigList.java:44)
- [ ] `TerralithConfigList.addEntry(1)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (net/stardustlabs/terralith/gui/TerralithConfigList.java:56)
- [ ] `TerralithConfigList.getRowWidth(0)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (net/stardustlabs/terralith/gui/TerralithConfigList.java:64)
- [ ] `TerralithConfigList.updateSize(2)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (net/stardustlabs/terralith/gui/TerralithConfigList.java:68)

### `TerralithConfigList.Entry`: it extends `Entry`

- [ ] `TerralithConfigList.Entry.Entry(1)`: its class isn't translated (it extends `Entry`) (net/stardustlabs/terralith/gui/TerralithConfigList.java:76)
- [ ] `TerralithConfigList.Entry.children(0)`: its class isn't translated (it extends `Entry`) (net/stardustlabs/terralith/gui/TerralithConfigList.java:80)
- [ ] `TerralithConfigList.Entry.narratables(0)`: its class isn't translated (it extends `Entry`) (net/stardustlabs/terralith/gui/TerralithConfigList.java:84)
- [ ] `TerralithConfigList.Entry.extractContent(5)`: its class isn't translated (it extends `Entry`) (net/stardustlabs/terralith/gui/TerralithConfigList.java:88)

### `TerralithConfigScreen`: it extends Minecraft's `Screen`

- [ ] `TerralithConfigScreen.TerralithConfigScreen(1)`: its class isn't translated (it extends Minecraft's `Screen`) (net/stardustlabs/terralith/gui/TerralithConfigScreen.java:16)
- [ ] `TerralithConfigScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (net/stardustlabs/terralith/gui/TerralithConfigScreen.java:21)
- [ ] `TerralithConfigScreen.repositionElements(0)`: its class isn't translated (it extends Minecraft's `Screen`) (net/stardustlabs/terralith/gui/TerralithConfigScreen.java:31)
- [ ] `TerralithConfigScreen.onDone(0)`: its class isn't translated (it extends Minecraft's `Screen`) (net/stardustlabs/terralith/gui/TerralithConfigScreen.java:38)
- [ ] `TerralithConfigScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (net/stardustlabs/terralith/gui/TerralithConfigScreen.java:43)
- [ ] `TerralithConfigScreen.text(1)`: a translated text with a computed key (net/stardustlabs/terralith/gui/TerralithConfigScreen.java:47)
- [ ] `TerralithConfigScreen.option(1)`: it calls TerralithConfigScreen.text, which isn't translated (net/stardustlabs/terralith/gui/TerralithConfigScreen.java:51)

### `Terralith`

- [x] `Terralith.MOD_ID` (net/stardustlabs/terralith/Terralith.java:8)
- [x] `Terralith.LOGGER` (net/stardustlabs/terralith/Terralith.java:9)
- [x] `Terralith.CONFIG_VERSION` (net/stardustlabs/terralith/Terralith.java:10)
- [x] `Terralith.id(1)` (net/stardustlabs/terralith/Terralith.java:12)

### `TerralithFabric`: it implements Fabric's `ModInitializer`

- [ ] `TerralithFabric.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (net/stardustlabs/terralith/TerralithFabric.java:9)
