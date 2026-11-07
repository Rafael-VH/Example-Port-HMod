// Terralith, ported to Horizon by horizon-port from Terralith_26.3_v2.6.5+26.3.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_terralith). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "terralith";
pub const NAME: &str = "Terralith";
pub const VERSION: &str = "2.6.5";
pub const DESCRIPTION: &str = "Explore almost 100 new biomes consisting of both realism and light fantasy, using just Vanilla blocks. Complete with several immersive structures to compliment the overhauled terrain.";

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

    /// `ConfigListBuilder` (net/stardustlabs/terralith/client/ConfigListBuilder.java)
    pub mod config_list_builder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ConfigListBuilder.addCategory(2): not translated, it has no body.
        // ConfigListBuilder.addText(2): not translated, it has no body.
        // ConfigListBuilder.addBoolean(4): not translated, it has no body.
        // ConfigListBuilder.build(1): not translated, its class isn't translated (it's an interface).
    }

    /// `TerralithModMenu` (net/stardustlabs/terralith/compat/TerralithModMenu.java)
    pub mod terralith_mod_menu {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // TerralithModMenu.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `ConfigResourceCondition` (net/stardustlabs/terralith/ConfigResourceCondition.java)
    pub mod config_resource_condition {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ResourceCondition`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Fabric's `ResourceConditionType`.
        // ConfigResourceCondition.ConfigResourceCondition(2): not translated, its class isn't translated (it implements Fabric's `ResourceCondition`).
        // ConfigResourceCondition.getType(0): not translated, its class isn't translated (it implements Fabric's `ResourceCondition`).
        // ConfigResourceCondition.test(1): not translated, its class isn't translated (it implements Fabric's `ResourceCondition`).
        // ConfigResourceCondition.key(0): not translated, its class isn't translated (it implements Fabric's `ResourceCondition`).
        // ConfigResourceCondition.invert(0): not translated, its class isn't translated (it implements Fabric's `ResourceCondition`).
    }

    /// `ConfigHandler` (net/stardustlabs/terralith/config/ConfigHandler.java)
    pub mod config_handler {
        use super::*;
        // LOADED_STATE: not translated, it uses ConfigState.DEFAULT_CONFIG, which isn't translated (it creates a ConfigState with a constructor that isn't translated).
        // PATH: not translated, a Path with no value (null in Java).
        #[derive(Clone, Debug)]
        pub struct ConfigHandler {
        }
        impl ConfigHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ConfigHandler.getState(0): not translated, it uses ConfigHandler.LOADED_STATE, which isn't translated (it stores a second reference to an object (Rust would need it copied or shared)).
        // ConfigHandler.load(1): not translated, it uses ConfigHandler.PATH, which isn't translated (a Path with no value (null in Java)).
        // ConfigHandler.save(0): not translated, a catch block that uses its exception.
    }

    /// `ConfigState` (net/stardustlabs/terralith/config/ConfigState.java)
    pub mod config_state {
        use super::*;
        pub static DEFAULT_MODULES: LazyLock<config_state_modules::Modules> = LazyLock::new(|| config_state_modules::Modules::new(&(JMap::from_iter([("custom_structures".to_string(), true), ("fog_tweaks".to_string(), true), ("intro_message".to_string(), true), ("skylands".to_string(), true), ("terrain_slabs".to_string(), true), ("vanilla_stone_gen".to_string(), false), ("recipe_changes".to_string(), false)]))));
        // DEFAULT_CONFIG: not translated, it creates a ConfigState with a constructor that isn't translated.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct ConfigState {
            pub format_version: i32,
            pub modules: config_state_modules::Modules,
        }
        impl ConfigState {
            // ConfigState.ConfigState(2): not translated, it stores a second reference to an object (Rust would need it copied or shared).
            pub fn test(&self, key: &str) -> bool {
                return self.modules.get_module(&key);
            }
        }
    }

    /// `ConfigState.Modules` (net/stardustlabs/terralith/config/ConfigState.java)
    pub mod config_state_modules {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug)]
        pub struct Modules {
            pub modules: JMap<String, bool>,
        }
        impl Modules {
            pub fn new(modules: &JMap<String, bool>) -> Self {
                let mut s = Self { modules: JMap::new() };
                s.modules = JMap::from_iter(modules.clone_pairs());
                s
            }
            // ConfigState.Modules.setModule(2): not translated, it calls ConfigHandler.save, which isn't translated.
            pub fn get_module(&self, key: &str) -> bool {
                if !(((*config_state::DEFAULT_MODULES).modules).contains_key(&*key)) {
                    // log: "Unknown module: {}"
                    return false;
                } else {
                    return self.modules.get(&*key).cloned().unwrap_or_else(|| (((*config_state::DEFAULT_MODULES).modules).get(&*key).cloned()).expect("null"));
                }
                #[allow(unreachable_code)]
                unreachable!()
            }
        }
        pub fn validate_and_create(modules: &JMap<String, bool>) -> config_state_modules::Modules {
            let mut filtered_modules: JMap<String, bool> = JMap::new();
            for entry in modules.iter() {
                if ((*config_state::DEFAULT_MODULES).modules).contains_key(&*entry.0) {
                    filtered_modules.insert(entry.0.to_string(), *entry.1);
                }
            }
            return config_state_modules::Modules::new(&filtered_modules);
        }
    }

    /// `TerralithConfigList` (net/stardustlabs/terralith/gui/TerralithConfigList.java)
    pub mod terralith_config_list {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ContainerObjectSelectionList`. Its static parts are below.
        // TerralithConfigList.TerralithConfigList(3): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // TerralithConfigList.addCategory(2): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // TerralithConfigList.addText(2): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // TerralithConfigList.addBoolean(4): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // TerralithConfigList.addEntry(1): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // TerralithConfigList.getRowWidth(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // TerralithConfigList.updateSize(2): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
    }

    /// `TerralithConfigList.Entry` (net/stardustlabs/terralith/gui/TerralithConfigList.java)
    pub mod terralith_config_list_entry {
        use super::*;
        // The class itself isn't translated: it extends `Entry`. Its static parts are below.
        // TerralithConfigList.Entry.Entry(1): not translated, its class isn't translated (it extends `Entry`).
        // TerralithConfigList.Entry.children(0): not translated, its class isn't translated (it extends `Entry`).
        // TerralithConfigList.Entry.narratables(0): not translated, its class isn't translated (it extends `Entry`).
        // TerralithConfigList.Entry.extractContent(5): not translated, its class isn't translated (it extends `Entry`).
    }

    /// `TerralithConfigScreen` (net/stardustlabs/terralith/gui/TerralithConfigScreen.java)
    pub mod terralith_config_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // TerralithConfigScreen.TerralithConfigScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // TerralithConfigScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // TerralithConfigScreen.repositionElements(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // TerralithConfigScreen.onDone(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // TerralithConfigScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // TerralithConfigScreen.text(1): not translated, it uses Minecraft's `Component`.
        // TerralithConfigScreen.option(1): not translated, it uses Minecraft's `Component`.
    }

    /// `Terralith` (net/stardustlabs/terralith/Terralith.java)
    pub mod terralith {
        use super::*;
        pub const MOD_ID: &str = "terralith";
        // LOGGER: the logger. Log calls become comments.
        pub const CONFIG_VERSION: i32 = 1;
        #[derive(Clone, Debug)]
        pub struct Terralith {
        }
        impl Terralith {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn id(name: &str) -> JIdentifier {
            return JIdentifier::new("terralith", &name);
        }
    }

    /// `TerralithFabric` (net/stardustlabs/terralith/TerralithFabric.java)
    pub mod terralith_fabric {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        // TerralithFabric.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "terralith");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
