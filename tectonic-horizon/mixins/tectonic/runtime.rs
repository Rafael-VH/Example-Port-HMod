// Tectonic, ported to Horizon by horizon-port from tectonic-3.0.31-fabric-26.3.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_tectonic). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "tectonic";
pub const NAME: &str = "Tectonic";
pub const VERSION: &str = "3.0.31";
pub const DESCRIPTION: &str = "Terrain shaping brought to new heights, grander and more varied than ever before!";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[];

/// The mod's own Java that doesn't need the game, translated to Rust by horizon-port.
/// Each Java method is here whole or not at all; the ones left out say why.
#[allow(dead_code, unused_variables, unused_mut, unused_parens, unused_imports, unreachable_code,
    non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
pub mod java {
    use std::sync::{LazyLock, Mutex};

    /// A Java static that changes: one value behind a lock.
    pub struct JCell<T>(Mutex<T>);
    impl<T: Clone> JCell<T> {
        pub const fn new(v: T) -> Self {
            Self(Mutex::new(v))
        }
        pub fn get(&self) -> T {
            self.0.lock().unwrap_or_else(|e| e.into_inner()).clone()
        }
        pub fn set(&self, v: T) {
            *self.0.lock().unwrap_or_else(|e| e.into_inner()) = v;
        }
        pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            f(&mut self.0.lock().unwrap_or_else(|e| e.into_inner()))
        }
    }

    /// Java's HashMap, LinkedHashMap and TreeMap. Keeps insertion order (LinkedHashMap's; HashMap's
    /// order is unspecified), or key order when made with `new_sorted` (TreeMap's).
    #[derive(Clone, Debug)]
    pub struct JMap<K, V> {
        index: std::collections::HashMap<K, usize>,
        entries: Vec<(K, V)>,
        sorted: bool,
    }
    /// Java's Map.equals: the same keys with equal values, in any order.
    impl<K: std::hash::Hash + Eq, V: PartialEq> PartialEq for JMap<K, V> {
        fn eq(&self, o: &Self) -> bool {
            self.entries.len() == o.entries.len()
                && self.entries.iter().all(|(k, v)| o.index.get(k).is_some_and(|&i| o.entries[i].1 == *v))
        }
    }
    impl<K, V> Default for JMap<K, V> {
        fn default() -> Self {
            Self { index: std::collections::HashMap::new(), entries: Vec::new(), sorted: false }
        }
    }
    impl<K: std::hash::Hash + Ord + Clone, V> JMap<K, V> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self { sorted: true, ..Self::default() }
        }
        pub fn from_iter(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn from_iter_sorted(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new_sorted();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn len(&self) -> usize {
            self.entries.len()
        }
        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
        pub fn clear(&mut self) {
            self.index.clear();
            self.entries.clear();
        }
        pub fn contains_key<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> bool
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.contains_key(k)
        }
        pub fn get<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> Option<&V>
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.get(k).map(|&i| &self.entries[i].1)
        }
        fn reindex(&mut self) {
            self.index.clear();
            for (i, (k, _)) in self.entries.iter().enumerate() {
                self.index.insert(k.clone(), i);
            }
        }
        fn slot(&mut self, k: K, v: V) -> usize {
            if self.sorted {
                let at = self.entries.partition_point(|e| e.0 < k);
                self.entries.insert(at, (k, v));
                self.reindex();
                at
            } else {
                self.index.insert(k.clone(), self.entries.len());
                self.entries.push((k, v));
                self.entries.len() - 1
            }
        }
        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            if let Some(&i) = self.index.get(&k) {
                return Some(std::mem::replace(&mut self.entries[i].1, v));
            }
            self.slot(k, v);
            None
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: std::borrow::Borrow<Q>,
        {
            let i = *self.index.get(k)?;
            let (_, v) = self.entries.remove(i);
            self.reindex();
            Some(v)
        }
        /// computeIfAbsent and putIfAbsent: the value, made first when the key is missing.
        pub fn or_insert_with(&mut self, k: K, f: impl FnOnce() -> V) -> &mut V {
            let i = match self.index.get(&k) {
                Some(&i) => i,
                None => self.slot(k, f()),
            };
            &mut self.entries[i].1
        }
        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.entries.iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &K> {
            self.entries.iter().map(|e| &e.0)
        }
        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|e| &e.1)
        }
        pub fn clone_pairs(&self) -> Vec<(K, V)>
        where
            V: Clone,
        {
            self.entries.clone()
        }
    }

    /// Java's HashSet, LinkedHashSet and TreeSet, kept like JMap's keys.
    #[derive(Clone, Debug)]
    pub struct JSet<T>(JMap<T, ()>);
    impl<T: std::hash::Hash + Eq> PartialEq for JSet<T> {
        fn eq(&self, o: &Self) -> bool {
            self.0 == o.0
        }
    }
    impl<T> Default for JSet<T> {
        fn default() -> Self {
            Self(JMap::default())
        }
    }
    impl<T: std::hash::Hash + Ord + Clone> JSet<T> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self(JMap::new_sorted())
        }
        pub fn from_iter(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn from_iter_sorted(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new_sorted();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn clear(&mut self) {
            self.0.clear();
        }
        /// True when it wasn't there yet, as Java's add.
        pub fn insert(&mut self, v: T) -> bool {
            self.0.insert(v, ()).is_none()
        }
        pub fn contains<Q: std::hash::Hash + Eq + ?Sized>(&self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.contains_key(v)
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.remove(v).is_some()
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.0.keys()
        }
        pub fn clone_pairs(&self) -> Vec<T> {
            self.0.keys().cloned().collect()
        }
    }

    /// The mod's data folder in Horizon, set by the hooks (game_side.rs `ensure_init`) before the mod's code runs.
    pub static J_DATA_DIR: JCell<Option<std::path::PathBuf>> = JCell::new(None);
    /// FabricLoader's config folder: the mod's data folder in Horizon (./config until the game sets it).
    pub fn j_config_dir() -> std::path::PathBuf {
        J_DATA_DIR.get().unwrap_or_else(|| std::path::PathBuf::from("config"))
    }

    /// Minecraft's RandomSource / java.util.Random: other numbers than Java's, the same ranges.
    #[derive(Clone, Debug)]
    pub struct JRandom(u64);
    impl JRandom {
        pub fn new() -> Self {
            let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0x9E37_79B9);
            Self(t | 1)
        }
        fn next(&mut self) -> u64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            v
        }
        /// 0 inclusive to n exclusive; Java throws on n <= 0.
        pub fn next_int(&mut self, n: i32) -> i32 {
            assert!(n > 0, "bound must be positive");
            (self.next() % n as u64) as i32
        }
        pub fn next_int_between(&mut self, lo: i32, hi: i32) -> i32 {
            lo + self.next_int(hi - lo)
        }
        pub fn next_float(&mut self) -> f32 {
            (self.next() >> 40) as f32 / (1u64 << 24) as f32
        }
        pub fn next_double(&mut self) -> f64 {
            (self.next() >> 11) as f64 / (1u64 << 53) as f64
        }
        pub fn next_boolean(&mut self) -> bool {
            self.next() & 1 == 1
        }
        pub fn next_gaussian(&mut self) -> f64 {
            let (u, v) = (self.next_double().max(1e-12), self.next_double());
            (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
        }
    }

    /// Minecraft's Identifier (`namespace:path`), which is only text.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct JIdentifier {
        pub namespace: String,
        pub path: String,
    }
    impl JIdentifier {
        pub fn new(namespace: &str, path: &str) -> Self {
            Self { namespace: namespace.to_string(), path: path.to_string() }
        }
        /// `a:b`, or `minecraft:b` without a namespace.
        pub fn parse(s: &str) -> Self {
            match s.split_once(':') {
                Some((n, p)) => Self::new(n, p),
                None => Self::new("minecraft", s),
            }
        }
        /// Identifier.tryParse: None (null) when it has characters an identifier can't.
        pub fn try_parse(s: &str) -> Option<Self> {
            let id = Self::parse(s);
            Self::try_build(&id.namespace, &id.path)
        }
        pub fn try_build(namespace: &str, path: &str) -> Option<Self> {
            let ns_ok = namespace.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'));
            let path_ok = path.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/'));
            if ns_ok && path_ok { Some(Self::new(namespace, path)) } else { None }
        }
    }
    impl std::fmt::Display for JIdentifier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.namespace, self.path)
        }
    }

    /// Java's String.equals.
    pub fn j_eq(a: &str, b: &str) -> bool {
        a == b
    }
    pub fn j_eq_ignore_case(a: &str, b: &str) -> bool {
        a.to_lowercase() == b.to_lowercase()
    }
    /// Java's String.length (in chars).
    pub fn j_len(s: &str) -> i32 {
        s.chars().count() as i32
    }
    pub fn j_char_at(s: &str, i: i32) -> char {
        s.chars().nth(i as usize).expect("charAt out of range")
    }
    /// Java's substring, by char.
    pub fn j_substring(s: &str, from: i32, to: Option<i32>) -> String {
        let n = s.chars().count() as i32;
        let to = to.unwrap_or(n);
        assert!(0 <= from && from <= to && to <= n, "substring out of range");
        s.chars().skip(from as usize).take((to - from) as usize).collect()
    }
    pub fn j_index_of(s: &str, sub: &str) -> i32 {
        s.find(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    pub fn j_index_of_char(s: &str, c: char) -> i32 {
        s.chars().position(|x| x == c).map_or(-1, |i| i as i32)
    }
    pub fn j_last_index_of(s: &str, sub: &str) -> i32 {
        s.rfind(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    /// Java's String.split on plain text: trailing empty parts dropped.
    pub fn j_split(s: &str, sep: &str) -> Vec<String> {
        let mut v: Vec<String> = s.split(sep).map(str::to_string).collect();
        while v.len() > 1 && v.last().is_some_and(|p| p.is_empty()) {
            v.pop();
        }
        v
    }
    /// Java's String.split(sep, limit) on plain text.
    pub fn j_split_limit(s: &str, sep: &str, limit: i32) -> Vec<String> {
        if limit == 0 {
            return j_split(s, sep);
        }
        if limit > 0 {
            return s.splitn(limit as usize, sep).map(str::to_string).collect();
        }
        s.split(sep).map(str::to_string).collect()
    }
    /// Java's String.compareTo: by UTF-16 unit, then by length.
    pub fn j_compare(a: &str, b: &str) -> i32 {
        let (x, y): (Vec<u16>, Vec<u16>) = (a.encode_utf16().collect(), b.encode_utf16().collect());
        for (p, q) in x.iter().zip(y.iter()) {
            if p != q {
                return *p as i32 - *q as i32;
            }
        }
        x.len() as i32 - y.len() as i32
    }
    /// Java's String.regionMatches, by char.
    pub fn j_region_matches(s: &str, ignore_case: bool, from: i32, other: &str, other_from: i32, len: i32) -> bool {
        let (a, b): (Vec<char>, Vec<char>) = (s.chars().collect(), other.chars().collect());
        if from < 0 || other_from < 0 || from as i64 + len as i64 > a.len() as i64 || other_from as i64 + len as i64 > b.len() as i64 {
            return false;
        }
        (0..len.max(0) as usize).all(|i| {
            let (p, q) = (a[from as usize + i], b[other_from as usize + i]);
            p == q || ignore_case && (j_upper_char(p) == j_upper_char(q) || j_lower_char(p) == j_lower_char(q))
        })
    }
    /// Java's Character.toLowerCase / toUpperCase (one char).
    pub fn j_lower_char(c: char) -> char {
        let mut l = c.to_lowercase();
        if l.len() == 1 { l.next().unwrap_or(c) } else { c }
    }
    pub fn j_upper_char(c: char) -> char {
        let mut u = c.to_uppercase();
        if u.len() == 1 { u.next().unwrap_or(c) } else { c }
    }
    /// Java's Arrays.copyOfRange: past the end is filled with defaults.
    pub fn j_copy_of_range<T: Clone + Default>(a: &[T], from: usize, to: usize) -> Vec<T> {
        assert!(from <= to && from <= a.len(), "copyOfRange out of range");
        let mut v: Vec<T> = a[from..to.min(a.len())].to_vec();
        v.resize(to - from, T::default());
        v
    }
    pub fn j_hash(s: &str) -> i32 {
        s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
    }
    /// Java's Boolean.parseBoolean.
    pub fn j_parse_bool(s: &str) -> bool {
        s.eq_ignore_ascii_case("true")
    }
    /// A float as Java prints it in "..." + x (1.0, not 1).
    pub fn j_float(x: f64) -> String {
        if x.is_finite() && x == x.trunc() && x.abs() < 1e7 { format!("{x:.1}") } else { format!("{x}") }
    }
    /// Java's Math.round.
    pub fn j_round_f(x: f32) -> i32 {
        (x + 0.5).floor() as i32
    }
    pub fn j_round_d(x: f64) -> i64 {
        (x + 0.5).floor() as i64
    }
    /// Java's Math.clamp / Mth.clamp (no panic when min > max).
    pub fn j_clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
        if v < min { min } else if v > max { max } else { v }
    }
    pub fn j_lerp<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>>(d: T, a: T, b: T) -> T {
        a + d * (b - a)
    }
    pub fn j_signum_f(x: f32) -> f32 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_signum_d(x: f64) -> f64 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_wrap_degrees(x: f64) -> f64 {
        let mut v = x % 360.0;
        if v >= 180.0 {
            v -= 360.0;
        }
        if v < -180.0 {
            v += 360.0;
        }
        v
    }
    pub fn j_frac(x: f64) -> f64 {
        x - x.floor()
    }
    pub fn j_compare_f(a: f64, b: f64) -> i32 {
        a.partial_cmp(&b).map_or(0, |o| o as i32)
    }
    pub fn j_copy_of<T: Clone + Default>(a: &[T], n: usize) -> Vec<T> {
        let mut v: Vec<T> = a.iter().take(n).cloned().collect();
        v.resize(n, T::default());
        v
    }
    /// Java's System.currentTimeMillis.
    pub fn j_millis() -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
    }
    /// Java's System.nanoTime: only differences mean anything.
    pub fn j_nanos() -> i64 {
        static START: LazyLock<std::time::Instant> = LazyLock::new(std::time::Instant::now);
        START.elapsed().as_nanos() as i64
    }

    /// `ConfigScreenBuilder` (dev/worldgen/tectonic/client/ConfigScreenBuilder.java)
    pub mod config_screen_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConfigScreenBuilder {
        }
        impl ConfigScreenBuilder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ConfigScreenBuilder.build(1): not translated, it uses Minecraft's `Screen`.
        // ConfigScreenBuilder.addButtons(1): not translated, it uses `ApollibRowHelper` (dev.worldgen.apollib.client.gui.element).
        // ConfigScreenBuilder.addNoise(4): not translated, it uses `ApollibRowHelper` (dev.worldgen.apollib.client.gui.element).
        // ConfigScreenBuilder.slider(9): not translated, it uses `ApollibSlider` (dev.worldgen.apollib.client.gui.element).
        // ConfigScreenBuilder.alignLeft(1): not translated, it uses `Builder` (dev.worldgen.apollib.client.gui.element.ApollibRowHelper).
    }

    /// `PresetSelectorScreen` (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java)
    pub mod preset_selector_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // PresetSelectorScreen.PresetSelectorScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // PresetSelectorScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // PresetSelectorScreen.repositionElements(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // PresetSelectorScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // PresetSelectorScreen.text(1): not translated, it uses Minecraft's `Component`.
    }

    /// `PresetSelectorScreen.PresetList` (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java)
    pub mod preset_selector_screen_preset_list {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ContainerObjectSelectionList`. Its static parts are below.
        // PresetSelectorScreen.PresetList.PresetList(3): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // PresetSelectorScreen.PresetList.addEntry(3): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // PresetSelectorScreen.PresetList.getRowWidth(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // PresetSelectorScreen.PresetList.updateSize(2): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
    }

    /// `PresetSelectorScreen.PresetList.Entry` (dev/worldgen/tectonic/client/gui/PresetSelectorScreen.java)
    pub mod preset_selector_screen_preset_list_entry {
        use super::*;
        // The class itself isn't translated: it extends `Entry`. Its static parts are below.
        // PresetSelectorScreen.PresetList.Entry.Entry(3): not translated, its class isn't translated (it extends `Entry`).
        // PresetSelectorScreen.PresetList.Entry.select(1): not translated, its class isn't translated (it extends `Entry`).
        // PresetSelectorScreen.PresetList.Entry.extractContent(5): not translated, its class isn't translated (it extends `Entry`).
        // PresetSelectorScreen.PresetList.Entry.children(0): not translated, its class isn't translated (it extends `Entry`).
        // PresetSelectorScreen.PresetList.Entry.narratables(0): not translated, its class isn't translated (it extends `Entry`).
    }

    /// `TectonicCommand` (dev/worldgen/tectonic/command/TectonicCommand.java)
    pub mod tectonic_command {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TectonicCommand {
        }
        impl TectonicCommand {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TectonicCommand.register(1): not translated, it uses Minecraft's `CommandDispatcher`.
        // TectonicCommand.addTarget(4): not translated, it uses Minecraft's `LiteralArgumentBuilder`.
        // TectonicCommand.debugOutput(1): not translated, it uses Minecraft's `CommandContext`.
        // TectonicCommand.locate(4): not translated, it uses Minecraft's `CommandContext`.
        pub fn dist(x1: i32, z1: i32, x2: i32, z2: i32) -> f32 {
            let i: i32 = x2 - x1;
            let j: i32 = z2 - z1;
            return (((i * i) + (j * j)) as f32).sqrt();
        }
        // TectonicCommand.getRandomState(1): not translated, it uses Minecraft's `RandomState`.
        // TectonicCommand.message(2): needs the game, so it's in game_side.rs.
        // TectonicCommand.failure(2): needs the game, so it's in game_side.rs.
        // TectonicCommand.get(4): not translated, it uses Minecraft's `Holder`.
        // TectonicCommand.getRegion(2): not translated, it uses Minecraft's `Component`.
        // TectonicCommand.key(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `TectonicCommand.LocateTargets` (dev/worldgen/tectonic/command/TectonicCommand.java)
    pub mod tectonic_command_locate_targets {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TectonicCommand.LocateTargets.range(2): not translated, it uses Minecraft's `InclusiveRange`.
    }

    /// `TectonicModMenuCompat` (dev/worldgen/tectonic/compat/TectonicModMenuCompat.java)
    pub mod tectonic_mod_menu_compat {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // TectonicModMenuCompat.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `ConfigPresets` (dev/worldgen/tectonic/config/ConfigPresets.java)
    pub mod config_presets {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ConfigPresets.acceptPresets(1): not translated, it uses `TriConsumer` (org.apache.logging.log4j.util).
    }

    /// `ConfigState` (dev/worldgen/tectonic/config/ConfigState.java)
    pub mod config_state {
        use super::*;
        // The class itself isn't translated: it implements `ApollibCopyable` (dev.worldgen.apollib.config). Its static parts are below.
        // DEFAULT_STATE: not translated, it uses ConfigState, which isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
        pub const MINOR_VERSION: i32 = 1;
        // BASE_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // ConfigState.create(9): not translated, it uses ConfigState, which isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
        // ConfigState.ConfigState(9): not translated, its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
        // ConfigState.enabled(0): not translated, its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
        // ConfigState.getValue(1): not translated, its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
        // ConfigState.getNoiseState(1): not translated, its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
        // ConfigState.test(1): not translated, its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
        // ConfigState.copy(0): not translated, its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
    }

    /// `ConfigState.Biomes` (dev/worldgen/tectonic/config/ConfigState.java)
    pub mod config_state_biomes {
        use super::*;
        // DEFAULT: not translated, it creates a Biomes with a constructor that isn't translated.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct Biomes {
            pub temperature: noise_state::NoiseState,
            pub vegetation: noise_state::NoiseState,
        }
        impl Biomes {
            // ConfigState.Biomes.Biomes(2): not translated, a NoiseState with no value (null in Java).
        }
    }

    /// `ConfigState.Caves` (dev/worldgen/tectonic/config/ConfigState.java)
    pub mod config_state_caves {
        use super::*;
        pub const DEPTH_CUTOFF_START: f32 = 0.1f32;
        pub const DEPTH_CUTOFF_SIZE: f32 = 0.1f32;
        pub const CHEESE_ENABLED: bool = true;
        pub const CHEESE_ADDITIVE: f32 = 0.27f32;
        pub const NOODLE_ENABLED: bool = true;
        pub const NOODLE_ADDITIVE: f32 = -0.075f32;
        pub const SPAGHETTI_ENABLED: bool = true;
        pub const CARVERS_ENABLED: bool = true;
        pub const ORE_FIX: bool = false;
        pub static DEFAULT: LazyLock<config_state_caves::Caves> = LazyLock::new(|| config_state_caves::Caves::new(0.1f32, 0.1f32, true, 0.27f32, true, -0.075f32, true, true, false));
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct Caves {
            pub depth_cutoff_start: f32,
            pub depth_cutoff_size: f32,
            pub cheese_enabled: bool,
            pub cheese_additive: f32,
            pub noodle_enabled: bool,
            pub noodle_additive: f32,
            pub spaghetti_enabled: bool,
            pub carvers_enabled: bool,
            pub ore_fix: bool,
        }
        impl Caves {
            pub fn new(depth_cutoff_start: f32, depth_cutoff_size: f32, cheese_enabled: bool, cheese_additive: f32, noodle_enabled: bool, noodle_additive: f32, spaghetti_enabled: bool, carvers_enabled: bool, ore_fix: bool) -> Self {
                let mut s = Self { depth_cutoff_start: depth_cutoff_start, depth_cutoff_size: depth_cutoff_size, cheese_enabled: cheese_enabled, cheese_additive: cheese_additive, noodle_enabled: noodle_enabled, noodle_additive: noodle_additive, spaghetti_enabled: spaghetti_enabled, carvers_enabled: carvers_enabled, ore_fix: ore_fix };
                s.depth_cutoff_start = depth_cutoff_start;
                s.depth_cutoff_size = depth_cutoff_size;
                s.cheese_enabled = cheese_enabled;
                s.cheese_additive = cheese_additive;
                s.noodle_enabled = noodle_enabled;
                s.noodle_additive = noodle_additive;
                s.spaghetti_enabled = spaghetti_enabled;
                s.carvers_enabled = carvers_enabled;
                s.ore_fix = ore_fix;
                s
            }
        }
    }

    /// `ConfigState.Continents` (dev/worldgen/tectonic/config/ConfigState.java)
    pub mod config_state_continents {
        use super::*;
        pub const OCEAN_OFFSET: f32 = -0.8f32;
        pub const CONTINENTS_SCALE: f32 = 0.13f32;
        pub const EROSION_SCALE: f32 = 0.25f32;
        pub const RIDGE_SCALE: f32 = 0.25f32;
        pub const UNDERGROUND_RIVERS: bool = true;
        pub const RIVER_LANTERNS: bool = true;
        pub const RIVER_ICE: bool = false;
        pub const FLAT_TERRAIN_SKEW: f32 = 0.1f32;
        pub const ROLLING_HILLS: bool = true;
        pub const JUNGLE_PILLARS: bool = true;
        pub static DEFAULT: LazyLock<config_state_continents::Continents> = LazyLock::new(|| config_state_continents::Continents::new(-0.8f32, 0.13f32, 0.25f32, 0.25f32, true, true, false, 0.1f32, true, true));
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct Continents {
            pub ocean_offset: f32,
            pub continents_scale: f32,
            pub erosion_scale: f32,
            pub ridge_scale: f32,
            pub underground_rivers: bool,
            pub river_lanterns: bool,
            pub river_ice: bool,
            pub flat_terrain_skew: f32,
            pub rolling_hills: bool,
            pub jungle_pillars: bool,
        }
        impl Continents {
            pub fn new(ocean_offset: f32, continents_scale: f32, erosion_scale: f32, ridge_scale: f32, underground_rivers: bool, river_lanterns: bool, river_ice: bool, flat_terrain_skew: f32, rolling_hills: bool, jungle_pillars: bool) -> Self {
                let mut s = Self { ocean_offset: ocean_offset, continents_scale: continents_scale, erosion_scale: erosion_scale, ridge_scale: ridge_scale, underground_rivers: underground_rivers, river_lanterns: river_lanterns, river_ice: river_ice, flat_terrain_skew: flat_terrain_skew, rolling_hills: rolling_hills, jungle_pillars: jungle_pillars };
                s.ocean_offset = ocean_offset;
                s.continents_scale = continents_scale;
                s.erosion_scale = erosion_scale;
                s.ridge_scale = ridge_scale;
                s.underground_rivers = underground_rivers;
                s.river_lanterns = river_lanterns;
                s.river_ice = river_ice;
                s.flat_terrain_skew = flat_terrain_skew;
                s.rolling_hills = rolling_hills;
                s.jungle_pillars = jungle_pillars;
                s
            }
        }
    }

    /// `ConfigState.Experimental` (dev/worldgen/tectonic/config/ConfigState.java)
    pub mod config_state_experimental {
        use super::*;
        pub const ALTERNATE_EROSION_SCALING: bool = false;
        pub const ALTERNATE_CONTINENTS_SCALING: bool = false;
        pub static DEFAULT: LazyLock<config_state_experimental::Experimental> = LazyLock::new(|| config_state_experimental::Experimental::new(false, false));
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct Experimental {
            pub alternate_erosion_scaling: bool,
            pub alternate_continents_scaling: bool,
        }
        impl Experimental {
            pub fn new(alternate_erosion_scaling: bool, alternate_continents_scaling: bool) -> Self {
                let mut s = Self { alternate_erosion_scaling: alternate_erosion_scaling, alternate_continents_scaling: alternate_continents_scaling };
                s.alternate_erosion_scaling = alternate_erosion_scaling;
                s.alternate_continents_scaling = alternate_continents_scaling;
                s
            }
        }
    }

    /// `ConfigState.General` (dev/worldgen/tectonic/config/ConfigState.java)
    pub mod config_state_general {
        use super::*;
        pub const MOD_ENABLED: bool = true;
        pub const SNOW_START_OFFSET: i32 = 128;
        pub static DEFAULT: LazyLock<config_state_general::General> = LazyLock::new(|| config_state_general::General::new(true, 128));
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct General {
            pub mod_enabled: bool,
            pub snow_start_offset: i32,
        }
        impl General {
            pub fn new(mod_enabled: bool, snow_start_offset: i32) -> Self {
                let mut s = Self { mod_enabled: mod_enabled, snow_start_offset: snow_start_offset };
                s.mod_enabled = mod_enabled;
                s.snow_start_offset = snow_start_offset;
                s
            }
        }
    }

    /// `ConfigState.GlobalTerrain` (dev/worldgen/tectonic/config/ConfigState.java)
    pub mod config_state_global_terrain {
        use super::*;
        pub const VERTICAL_SCALE: f32 = 1.125f32;
        pub const ELEVATION_BOOST: f32 = 0.0f32;
        // HEIGHT_LIMITS: not translated, it stores a second reference to an object (Rust would need it copied or shared).
        pub const LAVA_TUNNELS: bool = true;
        pub const ULTRASMOOTH: bool = false;
        // DEFAULT: not translated, it creates a GlobalTerrain with a constructor that isn't translated.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct GlobalTerrain {
            pub vertical_scale: f32,
            pub elevation_boost: f32,
            pub height_limits: height_limits::HeightLimits,
            pub lava_tunnels: bool,
            pub ultrasmooth: bool,
        }
        impl GlobalTerrain {
            // ConfigState.GlobalTerrain.GlobalTerrain(5): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        }
    }

    /// `ConfigState.Islands` (dev/worldgen/tectonic/config/ConfigState.java)
    pub mod config_state_islands {
        use super::*;
        pub const ENABLED: bool = true;
        pub static NOISE: LazyLock<noise_state::NoiseState> = LazyLock::new(|| noise_state::NoiseState::new_3(0.11f32, 1.0f32, 0.0f32));
        // DEFAULT: not translated, it creates a Islands with a constructor that isn't translated.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct Islands {
            pub enabled: bool,
            pub noise: noise_state::NoiseState,
        }
        impl Islands {
            // ConfigState.Islands.Islands(2): not translated, a NoiseState with no value (null in Java).
        }
    }

    /// `ConfigState.Oceans` (dev/worldgen/tectonic/config/ConfigState.java)
    pub mod config_state_oceans {
        use super::*;
        pub const OCEAN_DEPTH: f32 = -0.22f32;
        pub const DEEP_OCEAN_DEPTH: f32 = -0.45f32;
        pub const MONUMENT_OFFSET: i32 = -30;
        pub const REMOVE_FROZEN_OCEAN_ICE: bool = false;
        pub static DEFAULT: LazyLock<config_state_oceans::Oceans> = LazyLock::new(|| config_state_oceans::Oceans::new(-0.22f32, -0.45f32, -30, false));
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct Oceans {
            pub ocean_depth: f32,
            pub deep_ocean_depth: f32,
            pub monument_offset: i32,
            pub remove_frozen_ocean_ice: bool,
        }
        impl Oceans {
            pub fn new(ocean_depth: f32, deep_ocean_depth: f32, monument_offset: i32, remove_frozen_ocean_ice: bool) -> Self {
                let mut s = Self { ocean_depth: ocean_depth, deep_ocean_depth: deep_ocean_depth, monument_offset: monument_offset, remove_frozen_ocean_ice: remove_frozen_ocean_ice };
                s.ocean_depth = ocean_depth;
                s.deep_ocean_depth = deep_ocean_depth;
                s.monument_offset = monument_offset;
                s.remove_frozen_ocean_ice = remove_frozen_ocean_ice;
                s
            }
        }
    }

    /// `NewConfigState` (dev/worldgen/tectonic/config/NewConfigState.java)
    pub mod new_config_state {
        use super::*;
        pub const VERSION: i32 = 4;
        #[derive(Clone, Debug)]
        pub struct NewConfigState {
            pub version: i32,
        }
        impl NewConfigState {
            pub fn new() -> Self {
                let mut s = Self { version: 0i32 };
                s
            }
        }
    }

    /// `NewConfigState.Features` (dev/worldgen/tectonic/config/NewConfigState.java)
    pub mod new_config_state_features {
        use super::*;
        pub const UNDERGROUND_RIVERS: bool = true;
        pub const RIVER_ICE: bool = true;
        pub const LAVA_TUNNELS: bool = true;
        pub const JUNGLE_PILLARS: bool = true;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct Features {
            pub underground_rivers: bool,
            pub river_ice: bool,
            pub lava_tunnels: bool,
            pub jungle_pillars: bool,
        }
        impl Features {
            pub fn new(underground_rivers: bool, river_ice: bool, lava_tunnels: bool, jungle_pillars: bool) -> Self {
                let mut s = Self { underground_rivers: underground_rivers, river_ice: river_ice, lava_tunnels: lava_tunnels, jungle_pillars: jungle_pillars };
                s.underground_rivers = underground_rivers;
                s.river_ice = river_ice;
                s.lava_tunnels = lava_tunnels;
                s.jungle_pillars = jungle_pillars;
                s
            }
        }
    }

    /// `NewConfigState.Noises` (dev/worldgen/tectonic/config/NewConfigState.java)
    pub mod new_config_state_noises {
        use super::*;
        pub static TEMPERATURE: LazyLock<noise_state::NoiseState> = LazyLock::new(|| noise_state::create_0());
        pub static VEGETATION: LazyLock<noise_state::NoiseState> = LazyLock::new(|| noise_state::create_0());
        pub static CONTINENTS: LazyLock<noise_state::NoiseState> = LazyLock::new(|| noise_state::create_1(0.13f32));
        pub static EROSION: LazyLock<noise_state::NoiseState> = LazyLock::new(|| noise_state::create_0());
        pub static RIDGES: LazyLock<noise_state::NoiseState> = LazyLock::new(|| noise_state::create_0());
        pub static ISLANDS: LazyLock<noise_state::NoiseState> = LazyLock::new(|| noise_state::create_1(0.11f32));
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct Noises {
            pub temperature: noise_state::NoiseState,
            pub vegetation: noise_state::NoiseState,
            pub continents: noise_state::NoiseState,
            pub erosion: noise_state::NoiseState,
            pub ridges: noise_state::NoiseState,
            pub islands: noise_state::NoiseState,
        }
        impl Noises {
            // NewConfigState.Noises.Noises(6): not translated, a NoiseState with no value (null in Java).
        }
    }

    /// `NewConfigState.TerrainScales` (dev/worldgen/tectonic/config/NewConfigState.java)
    pub mod new_config_state_terrain_scales {
        use super::*;
        pub const VERTICAL_SCALE: f64 = 1.0f64;
        pub const OCEAN_DEPTH: f64 = 2.0f64;
        pub const DEEP_OCEAN_DEPTH: f64 = 2.0f64;
        #[derive(Clone, Debug)]
        pub struct TerrainScales {
        }
        impl TerrainScales {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `HeightLimits` (dev/worldgen/tectonic/config/object/HeightLimits.java)
    pub mod height_limits {
        use super::*;
        pub static DEFAULT: LazyLock<height_limits::HeightLimits> = LazyLock::new(|| height_limits::HeightLimits::new(-64, 320));
        pub static INCREASED_HEIGHT: LazyLock<height_limits::HeightLimits> = LazyLock::new(|| height_limits::HeightLimits::new(-64, 640));
        // OLD_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NEW_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // FULL_CODEC: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug)]
        pub struct HeightLimits {
            pub min_y: i32,
            pub max_y: i32,
        }
        impl HeightLimits {
            pub fn new(min_y: i32, max_y: i32) -> Self {
                let mut s = Self { min_y: min_y, max_y: max_y };
                if (min_y % 16) != 0 {
                    panic!("{}", "min_y should be a multiple of 16!");
                }
                if min_y > 0 {
                    panic!("{}", "min_y should be greater than 0!");
                }
                if (max_y % 16) != 0 {
                    panic!("{}", "max_y should be a multiple of 16!");
                }
                if max_y < 256 {
                    panic!("{}", "max_y should be less than 256!");
                }
                s.min_y = min_y;
                s.max_y = max_y;
                s
            }
            pub fn copy(&self) -> height_limits::HeightLimits {
                return height_limits::HeightLimits::new(self.min_y, self.max_y);
            }
            pub fn is_vanilla(&self) -> bool {
                return (self.min_y == (-64)) && (self.max_y == 320);
            }
            pub fn get_height(&self) -> i32 {
                return self.max_y - self.min_y;
            }
        }
        // HeightLimits.defaultLimits(1): not translated, it stores a second reference to an object (Rust would need it copied or shared).
    }

    /// `NoiseState` (dev/worldgen/tectonic/config/object/NoiseState.java)
    pub mod noise_state {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct NoiseState {
            pub scale: f32,
            pub multiplier: f32,
            pub offset: f32,
            pub smoother_scaling: bool,
        }
        impl NoiseState {
            pub fn new_3(scale: f32, multiplier: f32, offset: f32) -> Self {
                let mut s = Self { scale: scale, multiplier: multiplier, offset: offset, smoother_scaling: false };
                s.scale = scale;
                s.multiplier = multiplier;
                s.offset = offset;
                s.smoother_scaling = false;
                s
            }
            pub fn new_4(scale: f32, multiplier: f32, offset: f32, smoother_scaling: bool) -> Self {
                let mut s = Self { scale: scale, multiplier: multiplier, offset: offset, smoother_scaling: smoother_scaling };
                s.scale = scale;
                s.multiplier = multiplier;
                s.offset = offset;
                s.smoother_scaling = smoother_scaling;
                s
            }
            pub fn copy(&self) -> noise_state::NoiseState {
                return noise_state::NoiseState::new_4(self.scale, self.multiplier, self.offset, self.smoother_scaling);
            }
        }
        pub fn create_0() -> noise_state::NoiseState {
            return noise_state::NoiseState::new_3(0.25f32, 1.0f32, 0.0f32);
        }
        pub fn create_1(scale: f32) -> noise_state::NoiseState {
            return noise_state::NoiseState::new_3(scale, 1.0f32, 0.0f32);
        }
        // NoiseState.legacyCodec(1): not translated, it uses Minecraft's `MapCodec`.
        // NoiseState.codec(2): not translated, it uses Minecraft's `MapCodec`.
    }

    /// `V1ConfigState` (dev/worldgen/tectonic/config/V1ConfigState.java)
    pub mod v1_config_state {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct V1ConfigState {
            pub mod_enabled: bool,
            pub features: v1_config_state_features::Features,
            pub experimental: v1_config_state_experimental::Experimental,
        }
        impl V1ConfigState {
            pub fn new(mod_enabled: bool, features: &v1_config_state_features::Features, experimental: &v1_config_state_experimental::Experimental) -> Self {
                let mut s = Self { mod_enabled: mod_enabled, features: features.clone(), experimental: experimental.clone() };
                s.mod_enabled = mod_enabled;
                s.features = features.clone();
                s.experimental = experimental.clone();
                s
            }
            // V1ConfigState.upgrade(0): not translated, it creates a V2ConfigState with a constructor that isn't translated.
            pub fn mod_enabled(&self) -> bool {
                return self.mod_enabled;
            }
            pub fn features(&self) -> v1_config_state_features::Features {
                return self.features.clone();
            }
            pub fn experimental(&self) -> v1_config_state_experimental::Experimental {
                return self.experimental.clone();
            }
        }
    }

    /// `V1ConfigState.Experimental` (dev/worldgen/tectonic/config/V1ConfigState.java)
    pub mod v1_config_state_experimental {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        pub static DEFAULT: LazyLock<v1_config_state_experimental::Experimental> = LazyLock::new(|| v1_config_state_experimental::Experimental::new(0.25f32, false, 1.125f32));
        #[derive(Clone, Debug, PartialEq)]
        pub struct Experimental {
            pub horizontal_mountain_scale: f32,
            pub increased_height: bool,
            pub terrain_scale: f32,
        }
        impl Experimental {
            pub fn new(horizontal_mountain_scale: f32, increased_height: bool, terrain_scale: f32) -> Self {
                let mut s = Self { horizontal_mountain_scale: horizontal_mountain_scale, increased_height: increased_height, terrain_scale: terrain_scale };
                s.horizontal_mountain_scale = horizontal_mountain_scale;
                s.increased_height = increased_height;
                s.terrain_scale = terrain_scale;
                s
            }
            pub fn horizontal_mountain_scale(&self) -> f32 {
                return self.horizontal_mountain_scale;
            }
            pub fn increased_height(&self) -> bool {
                return self.increased_height;
            }
            pub fn terrain_scale(&self) -> f32 {
                return self.terrain_scale;
            }
        }
    }

    /// `V1ConfigState.Features` (dev/worldgen/tectonic/config/V1ConfigState.java)
    pub mod v1_config_state_features {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        pub static DEFAULT: LazyLock<v1_config_state_features::Features> = LazyLock::new(|| v1_config_state_features::Features::new(true, true, true, 128, true));
        #[derive(Clone, Debug, PartialEq)]
        pub struct Features {
            pub deeper_oceans: bool,
            pub desert_dunes: bool,
            pub lava_rivers: bool,
            pub snow_start_offset: i32,
            pub underground_rivers: bool,
        }
        impl Features {
            pub fn new(deeper_oceans: bool, desert_dunes: bool, lava_rivers: bool, snow_start_offset: i32, underground_rivers: bool) -> Self {
                let mut s = Self { deeper_oceans: deeper_oceans, desert_dunes: desert_dunes, lava_rivers: lava_rivers, snow_start_offset: snow_start_offset, underground_rivers: underground_rivers };
                s.deeper_oceans = deeper_oceans;
                s.desert_dunes = desert_dunes;
                s.lava_rivers = lava_rivers;
                s.snow_start_offset = snow_start_offset;
                s.underground_rivers = underground_rivers;
                s
            }
            pub fn deeper_oceans(&self) -> bool {
                return self.deeper_oceans;
            }
            pub fn desert_dunes(&self) -> bool {
                return self.desert_dunes;
            }
            pub fn lava_rivers(&self) -> bool {
                return self.lava_rivers;
            }
            pub fn snow_start_offset(&self) -> i32 {
                return self.snow_start_offset;
            }
            pub fn underground_rivers(&self) -> bool {
                return self.underground_rivers;
            }
        }
    }

    /// `V2ConfigState` (dev/worldgen/tectonic/config/V2ConfigState.java)
    pub mod v2_config_state {
        use super::*;
        // BASE_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct V2ConfigState {
            pub enabled: bool,
            pub toggles: v2_config_state_toggles::Toggles,
            pub scales: v2_config_state_scales::Scales,
            pub snow_offset: i32,
        }
        impl V2ConfigState {
            // V2ConfigState.V2ConfigState(4): not translated, it stores a second reference to an object (Rust would need it copied or shared).
            // V2ConfigState.upgrade(0): not translated, it uses ConfigState, which isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
        }
    }

    /// `V2ConfigState.Scales` (dev/worldgen/tectonic/config/V2ConfigState.java)
    pub mod v2_config_state_scales {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        pub static DEFAULT: LazyLock<v2_config_state_scales::Scales> = LazyLock::new(|| v2_config_state_scales::Scales::new(1.125f32, 0.25f32, -0.15f32, -0.45f32));
        #[derive(Clone, Debug)]
        pub struct Scales {
            pub final_multiplier: f32,
            pub erosion_scale: f32,
            pub ocean: f32,
            pub deep_ocean: f32,
        }
        impl Scales {
            pub fn new(final_multiplier: f32, erosion_scale: f32, ocean: f32, deep_ocean: f32) -> Self {
                let mut s = Self { final_multiplier: final_multiplier, erosion_scale: erosion_scale, ocean: ocean, deep_ocean: deep_ocean };
                s.final_multiplier = final_multiplier;
                s.erosion_scale = erosion_scale;
                s.ocean = ocean;
                s.deep_ocean = deep_ocean;
                s
            }
        }
    }

    /// `V2ConfigState.Toggles` (dev/worldgen/tectonic/config/V2ConfigState.java)
    pub mod v2_config_state_toggles {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        pub static DEFAULT: LazyLock<v2_config_state_toggles::Toggles> = LazyLock::new(|| v2_config_state_toggles::Toggles::new(true, true, true, true, false));
        #[derive(Clone, Debug)]
        pub struct Toggles {
            pub dunes: bool,
            pub underground_rivers: bool,
            pub lava_rivers: bool,
            pub monument_offset: bool,
            pub increased_height: bool,
        }
        impl Toggles {
            pub fn new(dunes: bool, underground_rivers: bool, lava_rivers: bool, monument_offset: bool, increased_height: bool) -> Self {
                let mut s = Self { dunes: dunes, underground_rivers: underground_rivers, lava_rivers: lava_rivers, monument_offset: monument_offset, increased_height: increased_height };
                s.dunes = dunes;
                s.underground_rivers = underground_rivers;
                s.lava_rivers = lava_rivers;
                s.monument_offset = monument_offset;
                s.increased_height = increased_height;
                s
            }
        }
    }

    /// `ConfigLoadPredicate` (dev/worldgen/tectonic/lithostitched/ConfigLoadPredicate.java)
    pub mod config_load_predicate {
        use super::*;
        // The class itself isn't translated: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ConfigLoadPredicate.ConfigLoadPredicate(1): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // ConfigLoadPredicate.test(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // ConfigLoadPredicate.codec(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // ConfigLoadPredicate.key(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
    }

    /// `SetHeightLimitsModifier` (dev/worldgen/tectonic/lithostitched/SetHeightLimitsModifier.java)
    pub mod set_height_limits_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetHeightLimitsModifier.SetHeightLimitsModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetHeightLimitsModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetHeightLimitsModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetHeightLimitsModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetHeightLimitsModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetHeightLimitsModifier.dimensionType(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetHeightLimitsModifier.noiseSettings(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `ConfigResourceCondition` (dev/worldgen/tectonic/platform/fabric/ConfigResourceCondition.java)
    pub mod config_resource_condition {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ResourceCondition`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Fabric's `ResourceConditionType`.
        // ConfigResourceCondition.ConfigResourceCondition(1): not translated, its class isn't translated (it implements Fabric's `ResourceCondition`).
        // ConfigResourceCondition.getType(0): not translated, its class isn't translated (it implements Fabric's `ResourceCondition`).
        // ConfigResourceCondition.test(1): not translated, its class isn't translated (it implements Fabric's `ResourceCondition`).
        // ConfigResourceCondition.key(0): not translated, its class isn't translated (it implements Fabric's `ResourceCondition`).
    }

    /// `TectonicFabric` (dev/worldgen/tectonic/platform/fabric/TectonicFabric.java)
    pub mod tectonic_fabric {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        // TectonicFabric.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
    }

    /// `TectonicRegistrations` (dev/worldgen/tectonic/registry/TectonicRegistrations.java)
    pub mod tectonic_registrations {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TectonicRegistrations {
        }
        impl TectonicRegistrations {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TectonicRegistrations.init(0): not translated, it uses Minecraft's `Registry`.
        // TectonicRegistrations.register(3): not translated, it's generic.
    }

    /// `Tectonic` (dev/worldgen/tectonic/Tectonic.java)
    pub mod tectonic {
        use super::*;
        pub const MOD_ID: &str = "tectonic";
        // LOGGER: the logger. Log calls become comments.
        pub static BLENDING_VERSION: JCell<i32> = JCell::new(1);
        pub static BLENDING_KEY: LazyLock<JCell<String>> = LazyLock::new(|| JCell::new("tectonic:blending_version".to_string()));
        // CONFIG: not translated, it uses `ApollibConfigHolder` (dev.worldgen.apollib.config).
        // REGISTRAR: not translated, it uses `ApollibRegistrar` (dev.worldgen.apollib.registry).
        #[derive(Clone, Debug)]
        pub struct Tectonic {
        }
        impl Tectonic {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Tectonic.init(0): not translated, it uses Tectonic.CONFIG, which isn't translated (it uses `ApollibConfigHolder` (dev.worldgen.apollib.config)).
        pub fn id(name: &str) -> JIdentifier {
            return JIdentifier::new("tectonic", &name);
        }
    }

    /// `TectonicTags` (dev/worldgen/tectonic/TectonicTags.java)
    pub mod tectonic_tags {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `ConfigClamp` (dev/worldgen/tectonic/worldgen/densityfunction/ConfigClamp.java)
    pub mod config_clamp {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensityFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ConfigClamp.ConfigClamp(3): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigClamp.compileSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigClamp.rewriteChildren(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigClamp.range(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigClamp.domainAxes(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigClamp.codec(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigClamp.input(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigClamp.min(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigClamp.max(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
    }

    /// `ConfigConstant` (dev/worldgen/tectonic/worldgen/densityfunction/ConfigConstant.java)
    pub mod config_constant {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensityFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ConfigConstant.ConfigConstant(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigConstant.create(1): not translated, it uses ConfigConstant, which isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigConstant.compileSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigConstant.rewriteChildren(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigConstant.range(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigConstant.domainAxes(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigConstant.codec(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigConstant.value(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
    }

    /// `ConfigNoise` (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java)
    pub mod config_noise {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensityFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ConfigNoise.ConfigNoise(7): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.create(4): not translated, it uses ConfigNoise, which isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.compileSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.rewriteChildren(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.range(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.domainAxes(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.codec(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.noise(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.shiftX(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.shiftZ(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.scale(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.multiplier(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.offset(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ConfigNoise.smootherScaling(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
    }

    /// `ConfigNoise.NormalSampler` (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java)
    pub mod config_noise_normal_sampler {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensitySampler`. Its static parts are below.
        // ConfigNoise.NormalSampler.NormalSampler(6): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.NormalSampler.sampleVolume(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.NormalSampler.sampleValue(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.NormalSampler.noise(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.NormalSampler.shiftX(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.NormalSampler.shiftZ(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.NormalSampler.scale(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.NormalSampler.multiplier(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.NormalSampler.offset(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
    }

    /// `ConfigNoise.SmoothSampler` (dev/worldgen/tectonic/worldgen/densityfunction/ConfigNoise.java)
    pub mod config_noise_smooth_sampler {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensitySampler`. Its static parts are below.
        // ConfigNoise.SmoothSampler.SmoothSampler(6): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.SmoothSampler.sampleVolume(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.SmoothSampler.sampleValue(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.SmoothSampler.noise(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.SmoothSampler.shiftX(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.SmoothSampler.shiftZ(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.SmoothSampler.scale(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.SmoothSampler.multiplier(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ConfigNoise.SmoothSampler.offset(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
    }

    /// `HeightStabilizedCount` (dev/worldgen/tectonic/worldgen/placementmodifier/HeightStabilizedCount.java)
    pub mod height_stabilized_count {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `PlacementModifier`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // HeightStabilizedCount.HeightStabilizedCount(3): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // HeightStabilizedCount.modify(4): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // HeightStabilizedCount.codec(0): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // HeightStabilizedCount.maxY(0): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // HeightStabilizedCount.countPerSection(0): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // HeightStabilizedCount.biasedToBottom(0): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "tectonic");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
