// Sound Physics Remastered, ported to Horizon by horizon-port from sound-physics-remastered-fabric-1.5.2+26.3.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_sound_physics_remastered). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "sound_physics_remastered";
pub const NAME: &str = "Sound Physics Remastered";
pub const VERSION: &str = "1.5.2+26.3";
pub const DESCRIPTION: &str = "Provides realistic sound attenuation, reverberation, and absorption through blocks.";

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

    /// `BlockDefinition` (com/sonicether/soundphysics/config/blocksound/BlockDefinition.java)
    pub mod block_definition {
        use super::*;
        // The class itself isn't translated: it implements `Comparable`. Its static parts are below.
        // BlockDefinition.getConfigString(0): not translated, it has no body.
        // BlockDefinition.getConfigComment(0): not translated, it has no body.
        // BlockDefinition.getName(0): not translated, it has no body.
        // BlockDefinition.compareTo(1): not translated, its class isn't translated (it implements `Comparable`).
    }

    /// `BlockIdDefinition` (com/sonicether/soundphysics/config/blocksound/BlockIdDefinition.java)
    pub mod block_id_definition {
        use super::*;
        // The class itself isn't translated: it extends `BlockDefinition`. Its static parts are below.
        // BlockIdDefinition.BlockIdDefinition(1): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockIdDefinition.getConfigString(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockIdDefinition.getConfigComment(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockIdDefinition.getName(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockIdDefinition.getBlock(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockIdDefinition.fromConfigString(1): not translated, it uses BlockIdDefinition, which isn't translated (it extends `BlockDefinition`).
        // BlockIdDefinition.equals(1): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockIdDefinition.hashCode(0): not translated, its class isn't translated (it extends `BlockDefinition`).
    }

    /// `BlockSoundConfigBase` (com/sonicether/soundphysics/config/blocksound/BlockSoundConfigBase.java)
    pub mod block_sound_config_base {
        use super::*;
        // The class itself isn't translated: it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder). Its static parts are below.
        // BlockSoundConfigBase.BlockSoundConfigBase(1): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // BlockSoundConfigBase.load(0): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // BlockSoundConfigBase.loadBlockDefinition(1): not translated, it uses BlockDefinition, which isn't translated (it implements `Comparable`).
        // BlockSoundConfigBase.saveSync(0): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // BlockSoundConfigBase.getBlockDefinitions(0): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // BlockSoundConfigBase.getBlockDefinitionValue(1): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // BlockSoundConfigBase.isTagIn(2): not translated, it's generic.
        // BlockSoundConfigBase.invalidateCaches(0): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // BlockSoundConfigBase.getBlockTags(0): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // BlockSoundConfigBase.getBlocks(0): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // BlockSoundConfigBase.getSoundTypes(0): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // BlockSoundConfigBase.setBlockDefinitionValue(2): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // BlockSoundConfigBase.addDefaults(1): not translated, it has no body.
        // BlockSoundConfigBase.getDefaultValue(0): not translated, it has no body.
        // BlockSoundConfigBase.putSoundType(3): not translated, it uses BlockDefinition, which isn't translated (it implements `Comparable`).
    }

    /// `BlockSoundTypeDefinition` (com/sonicether/soundphysics/config/blocksound/BlockSoundTypeDefinition.java)
    pub mod block_sound_type_definition {
        use super::*;
        // The class itself isn't translated: it extends `BlockDefinition`. Its static parts are below.
        // BlockSoundTypeDefinition.BlockSoundTypeDefinition(1): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockSoundTypeDefinition.getConfigString(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockSoundTypeDefinition.getConfigComment(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockSoundTypeDefinition.getName(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockSoundTypeDefinition.getSoundType(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockSoundTypeDefinition.fromConfigString(1): not translated, it uses BlockSoundTypeDefinition, which isn't translated (it extends `BlockDefinition`).
        // BlockSoundTypeDefinition.equals(1): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockSoundTypeDefinition.hashCode(0): not translated, its class isn't translated (it extends `BlockDefinition`).
    }

    /// `BlockTagDefinition` (com/sonicether/soundphysics/config/blocksound/BlockTagDefinition.java)
    pub mod block_tag_definition {
        use super::*;
        // The class itself isn't translated: it extends `BlockDefinition`. Its static parts are below.
        // BlockTagDefinition.BlockTagDefinition(1): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockTagDefinition.getConfigString(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockTagDefinition.getConfigComment(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockTagDefinition.getName(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockTagDefinition.getBlockTag(0): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockTagDefinition.fromConfigString(1): not translated, it uses BlockTagDefinition, which isn't translated (it extends `BlockDefinition`).
        // BlockTagDefinition.equals(1): not translated, its class isn't translated (it extends `BlockDefinition`).
        // BlockTagDefinition.hashCode(0): not translated, its class isn't translated (it extends `BlockDefinition`).
    }

    /// `ConfigUtils` (com/sonicether/soundphysics/config/ConfigUtils.java)
    pub mod config_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConfigUtils {
        }
        impl ConfigUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ConfigUtils.sortMap(1): not translated, it's generic.
    }

    /// `OcclusionConfig` (com/sonicether/soundphysics/config/OcclusionConfig.java)
    pub mod occlusion_config {
        use super::*;
        // The class itself isn't translated: it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound). Its static parts are below.
        // OcclusionConfig.OcclusionConfig(1): not translated, its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)).
        // OcclusionConfig.addDefaults(1): not translated, its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)).
        // OcclusionConfig.getDefaultValue(0): not translated, its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)).
    }

    /// `ReflectivityConfig` (com/sonicether/soundphysics/config/ReflectivityConfig.java)
    pub mod reflectivity_config {
        use super::*;
        // The class itself isn't translated: it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound). Its static parts are below.
        // ReflectivityConfig.ReflectivityConfig(1): not translated, its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)).
        // ReflectivityConfig.addDefaults(1): not translated, its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)).
        // ReflectivityConfig.getDefaultValue(0): not translated, its class isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)).
    }

    /// `ReverbParams` (com/sonicether/soundphysics/config/ReverbParams.java)
    pub mod reverb_params {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ReverbParams {
            pub decay_time: f32,
            pub density: f32,
            pub diffusion: f32,
            pub gain: f32,
            pub gain_hf: f32,
            pub decay_hf_ratio: f32,
            pub reflections_gain: f32,
            pub reflections_delay: f32,
            pub late_reverb_gain: f32,
            pub late_reverb_delay: f32,
            pub air_absorption_gain_hf: f32,
            pub room_rolloff_factor: f32,
        }
        impl ReverbParams {
            pub fn new() -> Self {
                let mut s = Self { decay_time: 0.0f32, density: 0.0f32, diffusion: 0.0f32, gain: 0.0f32, gain_hf: 0.0f32, decay_hf_ratio: 0.0f32, reflections_gain: 0.0f32, reflections_delay: 0.0f32, late_reverb_gain: 0.0f32, late_reverb_delay: 0.0f32, air_absorption_gain_hf: 0.0f32, room_rolloff_factor: 0.0f32 };
                s
            }
        }
        // ReverbParams.getReverb0(0): not translated, a nullable object kept in a changing static.
        // ReverbParams.getReverb1(0): not translated, a nullable object kept in a changing static.
        // ReverbParams.getReverb2(0): not translated, a nullable object kept in a changing static.
        // ReverbParams.getReverb3(0): not translated, a nullable object kept in a changing static.
        // ReverbParams.globalReverbMultiplier(0): not translated, a nullable object kept in a changing static.
    }

    /// `SoundPhysicsConfig` (com/sonicether/soundphysics/config/SoundPhysicsConfig.java)
    pub mod sound_physics_config {
        use super::*;
        // The class itself isn't translated: it uses ConfigEntry, which isn't translated (it's an interface). Its static parts are below.
        // SoundPhysicsConfig.SoundPhysicsConfig(1): not translated, its class isn't translated (it uses ConfigEntry, which isn't translated (it's an interface)).
        // SoundPhysicsConfig.reloadClient(0): not translated, its class isn't translated (it uses ConfigEntry, which isn't translated (it's an interface)).
    }

    /// `SoundRateConfig` (com/sonicether/soundphysics/config/SoundRateConfig.java)
    pub mod sound_rate_config {
        use super::*;
        // The class itself isn't translated: it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder). Its static parts are below.
        // SoundRateConfig.SoundRateConfig(1): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // SoundRateConfig.load(0): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // SoundRateConfig.logIfUnknownSound(1): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // SoundRateConfig.saveSync(0): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // SoundRateConfig.getSoundRateConfig(0): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // SoundRateConfig.getMaxCount(1): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        // SoundRateConfig.createDefaultMap(0): not translated, its class isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
    }

    /// `SoundTypes` (com/sonicether/soundphysics/config/SoundTypes.java)
    pub mod sound_types {
        use super::*;
        // TRANSLATION_MAP: not translated, it uses Minecraft's `SoundType`.
        #[derive(Clone, Debug)]
        pub struct SoundTypes {
        }
        impl SoundTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SoundTypes.getTranslationMap(0): not translated, it uses Minecraft's `SoundType`.
        // SoundTypes.getName(1): not translated, it uses Minecraft's `SoundType`.
        // SoundTypes.getSoundType(1): not translated, it uses Minecraft's `SoundType`.
        // SoundTypes.getNameComponent(1): not translated, it uses Minecraft's `MutableComponent`.
    }

    /// `RaycastRenderer` (com/sonicether/soundphysics/debug/RaycastRenderer.java)
    pub mod raycast_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `SimpleDebugRenderer`. Its static parts are below.
        // rays: not translated, it uses Ray, which isn't translated (it uses Minecraft's `Vec3`).
        // mc: not translated, it uses Minecraft's `Minecraft`.
        // RaycastRenderer.emitGizmos(6): not translated, its class isn't translated (it implements Minecraft's `SimpleDebugRenderer`).
        // RaycastRenderer.color(1): not translated, it uses Minecraft's `TextColor`.
        // RaycastRenderer.addSoundBounceRay(3): not translated, it uses Minecraft's `Vec3`.
        // RaycastRenderer.addOcclusionRay(3): not translated, it uses Minecraft's `Vec3`.
        // RaycastRenderer.addRay(4): not translated, it uses Minecraft's `Vec3`.
    }

    /// `RaycastRenderer.Ray` (com/sonicether/soundphysics/debug/RaycastRenderer.java)
    pub mod raycast_renderer_ray {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // RaycastRenderer.Ray.Ray(4): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `FabricSoundPhysicsMod` (com/sonicether/soundphysics/FabricSoundPhysicsMod.java)
    pub mod fabric_sound_physics_mod {
        use super::*;
        // The class itself isn't translated: it extends `SoundPhysicsMod`. Its static parts are below.
        // FabricSoundPhysicsMod.onInitialize(0): not translated, its class isn't translated (it extends `SoundPhysicsMod`).
        // FabricSoundPhysicsMod.onInitializeClient(0): not translated, its class isn't translated (it extends `SoundPhysicsMod`).
        // FabricSoundPhysicsMod.getConfigFolder(0): not translated, its class isn't translated (it extends `SoundPhysicsMod`).
    }

    /// `ClothConfigIntegration` (com/sonicether/soundphysics/integration/ClothConfigIntegration.java)
    pub mod cloth_config_integration {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClothConfigIntegration {
        }
        impl ClothConfigIntegration {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClothConfigIntegration.createConfigScreen(1): not translated, it uses Minecraft's `Screen`.
        // ClothConfigIntegration.fromConfigEntry(4): not translated, it's generic.
    }

    /// `ModMenuIntegration` (com/sonicether/soundphysics/integration/ModMenuIntegration.java)
    pub mod mod_menu_integration {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // ModMenuIntegration.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
        pub fn is_cloth_config_loaded() -> bool {
            // (`if` always false in Horizon: only the else part)
            return false;
        }
    }

    /// `AudioChannel` (com/sonicether/soundphysics/integration/voicechat/AudioChannel.java)
    pub mod audio_channel {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        pub const VOICECHAT: &str = "voicechat";
        pub static VOICECHAT_SOUND: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("voicechat", "voicechat"));
        // AudioChannel.AudioChannel(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // AudioChannel.onSound(4): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // AudioChannel.getChannelId(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // AudioChannel.canBeRemoved(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        pub fn is_voicechat_sound(sound: &JIdentifier) -> bool {
            return j_eq(&sound.namespace.clone(), "voicechat");
        }
    }

    /// `SimpleVoiceChatPlugin` (com/sonicether/soundphysics/integration/voicechat/SimpleVoiceChatPlugin.java)
    pub mod simple_voice_chat_plugin {
        use super::*;
        // The class itself isn't translated: it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api). Its static parts are below.
        // OWN_VOICE_ID: not translated, it uses `UUID` (java.util).
        pub static OWN_VOICE_CATEGORY: LazyLock<JCell<String>> = LazyLock::new(|| JCell::new("own_voice".to_string()));
        // SimpleVoiceChatPlugin.getPluginId(0): not translated, its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)).
        // SimpleVoiceChatPlugin.initialize(1): not translated, its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)).
        // SimpleVoiceChatPlugin.registerEvents(1): not translated, its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)).
        // SimpleVoiceChatPlugin.onServerStarted(1): not translated, its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)).
        // SimpleVoiceChatPlugin.onClientSound(1): not translated, its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)).
        // SimpleVoiceChatPlugin.onCreateALContext(1): not translated, its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)).
        // SimpleVoiceChatPlugin.onConnection(1): not translated, its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)).
        // SimpleVoiceChatPlugin.onOpenALSound(1): not translated, its class isn't translated (it implements `VoicechatPlugin` (de.maxhenkel.voicechat.api)).
        // SimpleVoiceChatPlugin.getDirectGainScale(2): not translated, it uses `VoicechatClientApi` (de.maxhenkel.voicechat.api).
    }

    /// `Loggers` (com/sonicether/soundphysics/Loggers.java)
    pub mod loggers {
        use super::*;
        pub const LOG_PREFIX: &str = "Sound Physics - %s";
        // DEBUG_LOGGER: the logger. Log calls become comments.
        // PROFILING_LOGGER: the logger. Log calls become comments.
        // ENVIRONMENT_LOGGER: the logger. Log calls become comments.
        // OCCLUSION_LOGGER: the logger. Log calls become comments.
        // LOGGER: the logger. Log calls become comments.
        #[derive(Clone, Debug)]
        pub struct Loggers {
        }
        impl Loggers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Loggers.log(2): not translated, it uses `Object`.
        // Loggers.warn(2): not translated, it uses `Object`.
        // Loggers.error(2): not translated, it uses `Object`.
        // Loggers.logProfiling(2): not translated, it uses `Object`.
        // Loggers.logDebug(2): not translated, it uses `Object`.
        // Loggers.logOcclusion(2): not translated, it uses `Object`.
        // Loggers.logEnvironment(2): not translated, it uses `Object`.
        // Loggers.logALError(1): not translated, it calls `AL11` (org.lwjgl.openal).alGetError.
    }

    /// `TaskProfiler` (com/sonicether/soundphysics/profiling/TaskProfiler.java)
    pub mod task_profiler {
        use super::*;
        // The class itself isn't translated: it keeps an atomic value in an object's field. Its static parts are below.
        pub const TASK_RING_BUFFER_SIZE: i32 = 100;
        pub const TASK_RING_TALLY_SIZE: i32 = 100;
        // TaskProfiler.TaskProfiler(1): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // TaskProfiler.profile(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // TaskProfiler.addDuration(1): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // TaskProfiler.getTotalDuration(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // TaskProfiler.getAverageDuration(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // TaskProfiler.getMinDuration(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // TaskProfiler.getMaxDuration(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // TaskProfiler.logResults(0): not translated, its class isn't translated (it keeps an atomic value in an object's field).
        // TaskProfiler.onTally(1): not translated, its class isn't translated (it keeps an atomic value in an object's field).
    }

    /// `TaskProfiler.TaskProfilerHandle` (com/sonicether/soundphysics/profiling/TaskProfiler.java)
    pub mod task_profiler_task_profiler_handle {
        use super::*;
        // The class itself isn't translated: it uses `WeakReference` (java.lang.ref). Its static parts are below.
        // TaskProfiler.TaskProfilerHandle.TaskProfilerHandle(0): not translated, its class isn't translated (it uses `WeakReference` (java.lang.ref)).
        // TaskProfiler.TaskProfilerHandle.finish(0): not translated, its class isn't translated (it uses `WeakReference` (java.lang.ref)).
        // TaskProfiler.TaskProfilerHandle.getDuration(0): not translated, its class isn't translated (it uses `WeakReference` (java.lang.ref)).
    }

    /// `ReflectedAudio` (com/sonicether/soundphysics/ReflectedAudio.java)
    pub mod reflected_audio {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // ReflectedAudio.ReflectedAudio(2): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ReflectedAudio.shouldEvaluateDirection(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ReflectedAudio.getSharedAirspaces(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ReflectedAudio.addDirectAirspace(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ReflectedAudio.addSharedAirspace(2): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ReflectedAudio.evaluateSoundPosition(2): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `SoundPhysics` (com/sonicether/soundphysics/SoundPhysics.java)
    pub mod sound_physics {
        use super::*;
        pub const PHI: f32 = 1.618034f32;
        pub static AMBIENT_PATTERN: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("^[a-zA-Z0-9_\\-\\.]+:ambient\\..*$"));
        pub static BLOCK_PATTERN: LazyLock<JRegex> = LazyLock::new(|| JRegex::new(".*block..*"));
        pub static REFERENCE_DISTANCE_BUG_VERSION: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("ALSOFT 1\\.(22|23)\\."));
        pub static AUX_FX_SLOT0: JCell<i32> = JCell::new(0i32);
        pub static AUX_FX_SLOT1: JCell<i32> = JCell::new(0i32);
        pub static AUX_FX_SLOT2: JCell<i32> = JCell::new(0i32);
        pub static AUX_FX_SLOT3: JCell<i32> = JCell::new(0i32);
        pub static REVERB0: JCell<i32> = JCell::new(0i32);
        pub static REVERB1: JCell<i32> = JCell::new(0i32);
        pub static REVERB2: JCell<i32> = JCell::new(0i32);
        pub static REVERB3: JCell<i32> = JCell::new(0i32);
        pub static DIRECT_FILTER0: JCell<i32> = JCell::new(0i32);
        pub static SEND_FILTER0: JCell<i32> = JCell::new(0i32);
        pub static SEND_FILTER1: JCell<i32> = JCell::new(0i32);
        pub static SEND_FILTER2: JCell<i32> = JCell::new(0i32);
        pub static SEND_FILTER3: JCell<i32> = JCell::new(0i32);
        // minecraft: not translated, it uses Minecraft's `Minecraft`.
        // profiler: not translated, it uses TaskProfiler, which isn't translated (it keeps an atomic value in an object's field).
        // lastSoundCategory: not translated, it uses Minecraft's `SoundSource`.
        pub static LAST_SOUND: LazyLock<JCell<Option<JIdentifier>>> = LazyLock::new(|| JCell::new(None));
        pub static REFERENCE_DISTANCE_BUG: LazyLock<JCell<Option<bool>>> = LazyLock::new(|| JCell::new(None));
        pub static MAX_AUX_SENDS: JCell<i32> = JCell::new(0i32);
        #[derive(Clone, Debug)]
        pub struct SoundPhysics {
        }
        impl SoundPhysics {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SoundPhysics.init(0): not translated, it calls Loggers.log, which isn't translated.
        // SoundPhysics.syncReverbParams(0): not translated, it calls ReverbParams.getReverb0, which isn't translated.
        // SoundPhysics.setupEFX(0): not translated, it calls `ALC10` (org.lwjgl.openal).alcGetCurrentContext.
        // SoundPhysics.applyReferenceDistanceWorkaround(2): not translated, it calls `AL10` (org.lwjgl.openal).alSourcef.
        // SoundPhysics.hasReferenceDistanceBug(0): not translated, it calls `AL10` (org.lwjgl.openal).alGetString.
        // SoundPhysics.setLastSoundCategoryAndName(2): not translated, it uses Minecraft's `SoundSource`.
        // SoundPhysics.onPlaySound(4): not translated, it's overloaded with the same number of parameters.
        // SoundPhysics.onPlayReverb(4): not translated, it's overloaded with the same number of parameters.
        // SoundPhysics.processSound(6): not translated, it uses Minecraft's `Vec3`.
        // SoundPhysics.processSound(7): not translated, it uses Minecraft's `Vec3`.
        // SoundPhysics.processSound(7): not translated, it uses Minecraft's `Vec3`.
        // SoundPhysics.evaluateEnvironment(7): not translated, its body couldn't be read: an assert.
        pub fn is_ambient_sound(sound: &JIdentifier) -> bool {
            return (JMatcher::new((&(*sound_physics::AMBIENT_PATTERN)), &sound.to_string())).matches();
        }
        // SoundPhysics.getBlockReflectivity(1): not translated, it uses Minecraft's `BlockPos`.
        // SoundPhysics.reflect(2): not translated, it uses Minecraft's `Vec3`.
        // SoundPhysics.calculateOcclusion(4): not translated, it uses Minecraft's `Vec3`.
        // SoundPhysics.runOcclusion(2): not translated, it uses Minecraft's `Vec3`.
        // SoundPhysics.getLevelProxy(0): not translated, it uses ClientLevelProxy, which isn't translated (it's an interface).
        // SoundPhysics.getSharedAirspace(2): not translated, it uses Minecraft's `Vec3`.
        // SoundPhysics.getSharedAirspace(2): not translated, it uses Minecraft's `Vec3`.
        // SoundPhysics.setDefaultEnvironment(1): not translated, it calls SoundPhysics.setDefaultEnvironment, which isn't translated.
        // SoundPhysics.setDefaultEnvironment(2): not translated, it calls SoundPhysics.setEnvironment, which isn't translated.
        // SoundPhysics.setEnvironment(11): not translated, it uses SoundPhysicsConfig, which isn't translated (it uses ConfigEntry, which isn't translated (it's an interface)).
        // SoundPhysics.setSoundPos(2): not translated, it uses Minecraft's `Vec3`.
        // SoundPhysics.setReverbParams(3): not translated, it calls `EXTEfx` (org.lwjgl.openal).alEffectf.
    }

    /// `SoundPhysicsMod` (com/sonicether/soundphysics/SoundPhysicsMod.java)
    pub mod sound_physics_mod {
        use super::*;
        pub const MODID: &str = "sound_physics_remastered";
        // CONFIG: not translated, it uses SoundPhysicsConfig, which isn't translated (it uses ConfigEntry, which isn't translated (it's an interface)).
        // REFLECTIVITY_CONFIG: not translated, it uses ReflectivityConfig, which isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)).
        // OCCLUSION_CONFIG: not translated, it uses OcclusionConfig, which isn't translated (it extends `BlockSoundConfigBase` (com.sonicether.soundphysics.config.blocksound)).
        // SOUND_RATE_CONFIG: not translated, it uses SoundRateConfig, which isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder)).
        #[derive(Clone, Debug)]
        pub struct SoundPhysicsMod {
        }
        impl SoundPhysicsMod {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // SoundPhysicsMod.init(0): not translated, it calls SoundPhysicsMod.initConfig, which isn't translated.
            // SoundPhysicsMod.initClient(0): not translated, it uses SoundPhysicsMod.CONFIG, which isn't translated (it uses SoundPhysicsConfig, which isn't translated (it uses ConfigEntry, which isn't translated (it's an interface))).
            // SoundPhysicsMod.initConfig(0): not translated, it uses SoundPhysicsMod.CONFIG, which isn't translated (it uses SoundPhysicsConfig, which isn't translated (it uses ConfigEntry, which isn't translated (it's an interface))).
            // SoundPhysicsMod.renameAllowedSounds(0): not translated, a try/catch (NoSuchFileException, FileAlreadyExistsException, IOException).
            // SoundPhysicsMod.getConfigFolder(0): not translated, it has no body.
        }
    }

    /// `LevelAccessUtils` (com/sonicether/soundphysics/utils/LevelAccessUtils.java)
    pub mod level_access_utils {
        use super::*;
        // PROFILER: not translated, it uses TaskProfiler, which isn't translated (it keeps an atomic value in an object's field).
        // MC: not translated, it uses Minecraft's `Minecraft`.
        #[derive(Clone, Debug)]
        pub struct LevelAccessUtils {
        }
        impl LevelAccessUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LevelAccessUtils.onLoadLevel(1): not translated, it uses Minecraft's `ClientLevel`.
        // LevelAccessUtils.onUnloadLevel(1): not translated, it uses Minecraft's `ClientLevel`.
        // LevelAccessUtils.tickLevelCache(1): not translated, it uses Minecraft's `ClientLevel`.
        // LevelAccessUtils.updateLevelCache(3): not translated, it uses Minecraft's `ClientLevel`.
        // LevelAccessUtils.getClientLevelProxy(1): not translated, it uses ClientLevelProxy, which isn't translated (it's an interface).
        // LevelAccessUtils.levelOriginFromPlayer(0): not translated, it uses Minecraft's `BlockPos`.
    }

    /// `RaycastUtils` (com/sonicether/soundphysics/utils/RaycastUtils.java)
    pub mod raycast_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RaycastUtils {
        }
        impl RaycastUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RaycastUtils.rayCast(4): not translated, it uses Minecraft's `BlockHitResult`.
    }

    /// `SoundRateManager` (com/sonicether/soundphysics/utils/SoundRateManager.java)
    pub mod sound_rate_manager {
        use super::*;
        pub static SOUND_COUNTS: LazyLock<JCell<JMap<JIdentifier, i32>>> = LazyLock::new(|| JCell::new(JMap::new()));
        pub static WORLD_INITIALIZED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct SoundRateManager {
        }
        impl SoundRateManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SoundRateManager.incrementAndCheckLimit(1): not translated, it uses SoundPhysicsMod.SOUND_RATE_CONFIG, which isn't translated (it uses SoundRateConfig, which isn't translated (it extends `CommentedPropertyConfig` (de.maxhenkel.sound_physics_remastered.configbuilder))).
        pub fn is_world_initialized() -> bool {
            return sound_rate_manager::WORLD_INITIALIZED.get();
        }
        // SoundRateManager.onClientTick(1): not translated, it uses Minecraft's `ClientLevel`.
        pub fn clear() {
            sound_rate_manager::SOUND_COUNTS.with(|o| o.clear());
        }
        // SoundRateManager.onLoadLevel(1): not translated, it uses Minecraft's `ClientLevel`.
        // SoundRateManager.onUnloadLevel(1): not translated, it uses Minecraft's `ClientLevel`.
    }

    /// `SoundUtils` (com/sonicether/soundphysics/utils/SoundUtils.java)
    pub mod sound_utils {
        use super::*;
        pub static STEP_PATTERN: LazyLock<JRegex> = LazyLock::new(|| JRegex::new(".*step.*"));
        #[derive(Clone, Debug)]
        pub struct SoundUtils {
        }
        impl SoundUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SoundUtils.calculateEntitySoundYOffset(2): not translated, it uses Minecraft's `SoundEvent`.
    }

    /// `CachingClientLevel` (com/sonicether/soundphysics/world/CachingClientLevel.java)
    pub mod caching_client_level {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CachingClientLevel.sound_physics_remastered$getCachedClone(0): not translated, it has no body.
        // CachingClientLevel.sound_physics_remastered$setCachedClone(1): not translated, it has no body.
    }

    /// `ClientLevelProxy` (com/sonicether/soundphysics/world/ClientLevelProxy.java)
    pub mod client_level_proxy {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `ClonedClientLevel` (com/sonicether/soundphysics/world/ClonedClientLevel.java)
    pub mod cloned_client_level {
        use super::*;
        // The class itself isn't translated: it implements `ClientLevelProxy`. Its static parts are below.
        // ClonedClientLevel.ClonedClientLevel(4): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // ClonedClientLevel.getOrigin(0): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // ClonedClientLevel.getTick(0): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // ClonedClientLevel.getChunk(2): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // ClonedClientLevel.getBlockState(1): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // ClonedClientLevel.getFluidState(1): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // ClonedClientLevel.getHeight(0): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // ClonedClientLevel.getMinY(0): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // ClonedClientLevel.getBlockEntity(1): not translated, its class isn't translated (it implements `ClientLevelProxy`).
    }

    /// `ClonedLevelChunk` (com/sonicether/soundphysics/world/ClonedLevelChunk.java)
    pub mod cloned_level_chunk {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ChunkAccess`. Its static parts are below.
        // ClonedLevelChunk.ClonedLevelChunk(3): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.getBlockEntity(1): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.getBlockState(1): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.withLevelChunkSectionAtPosition(2): not translated, it's generic.
        // ClonedLevelChunk.getBlockTicks(0): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.getFluidTicks(0): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.getTicksForSerialization(1): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.getPersistedStatus(0): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.addEntity(1): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.getBlockEntityNbtForSaving(2): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.removeBlockEntity(1): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.setBlockState(3): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
        // ClonedLevelChunk.setBlockEntity(1): not translated, its class isn't translated (it extends Minecraft's `ChunkAccess`).
    }

    /// `ClonedLevelHeightAccessor` (com/sonicether/soundphysics/world/ClonedLevelHeightAccessor.java)
    pub mod cloned_level_height_accessor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `LevelHeightAccessor`. Its static parts are below.
        // ClonedLevelHeightAccessor.ClonedLevelHeightAccessor(1): not translated, its class isn't translated (it implements Minecraft's `LevelHeightAccessor`).
        // ClonedLevelHeightAccessor.getHeight(0): not translated, its class isn't translated (it implements Minecraft's `LevelHeightAccessor`).
        // ClonedLevelHeightAccessor.getMinY(0): not translated, its class isn't translated (it implements Minecraft's `LevelHeightAccessor`).
        // ClonedLevelHeightAccessor.getMinBuildHeight(0): not translated, its class isn't translated (it implements Minecraft's `LevelHeightAccessor`).
    }

    /// `UnsafeClientLevel` (com/sonicether/soundphysics/world/UnsafeClientLevel.java)
    pub mod unsafe_client_level {
        use super::*;
        // The class itself isn't translated: it implements `ClientLevelProxy`. Its static parts are below.
        // UnsafeClientLevel.UnsafeClientLevel(1): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // UnsafeClientLevel.getBlockEntity(1): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // UnsafeClientLevel.getBlockState(1): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // UnsafeClientLevel.getFluidState(1): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // UnsafeClientLevel.getHeight(0): not translated, its class isn't translated (it implements `ClientLevelProxy`).
        // UnsafeClientLevel.getMinY(0): not translated, its class isn't translated (it implements `ClientLevelProxy`).
    }

    /// `CommentedProperties` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java)
    pub mod commented_properties {
        use super::*;
        // The class itself isn't translated: it implements `Map` (java.util). Its static parts are below.
        // CommentedProperties.CommentedProperties(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.CommentedProperties(0): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.addHeaderComment(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.setHeaderComments(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.sort(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.get(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.getComments(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.setComments(2): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.set(3): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.load(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.readLine(1): not translated, it uses `StringReader` (java.io).
        // CommentedProperties.readEscapedCharacter(2): not translated, it uses `StringReader` (java.io).
        pub fn is_whitespace(c: i32) -> bool {
            return ((((c == 32) || (c == 9)) || (c == 13)) || (c == 12)) || c.is_whitespace();
        }
        pub fn is_separator(c: i32) -> bool {
            return ((((c == 61) || (c == 58)) || (c == 32)) || (c == 9)) || (c == 12);
        }
        // CommentedProperties.save(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        pub fn remove_new_lines(comments: Option<Vec<String>>) -> Vec<String> {
            let mut new_comments: Vec<String> = Vec::new();
            for comment in (comments.as_ref().expect("null")).iter() {
                new_comments.extend(({ static RE: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("\\r?\\n")); &*RE }.split(comment, 0)).iter().cloned().collect::<Vec<_>>());
            }
            return new_comments.to_vec();
        }
        // CommentedProperties.escapeKey(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.escapeValue(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.escape(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.size(0): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.isEmpty(0): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.containsKey(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.containsValue(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.get(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.put(2): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.remove(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.putAll(1): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.clear(0): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.keySet(0): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.values(0): not translated, its class isn't translated (it implements `Map` (java.util)).
        // CommentedProperties.entrySet(0): not translated, its class isn't translated (it implements `Map` (java.util)).
    }

    /// `CommentedProperties.LineReader` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java)
    pub mod commented_properties_line_reader {
        use super::*;
        // The class itself isn't translated: it implements `Closeable` (java.io). Its static parts are below.
        // CommentedProperties.LineReader.LineReader(1): not translated, its class isn't translated (it implements `Closeable` (java.io)).
        // CommentedProperties.LineReader.fromInputStream(1): not translated, it uses LineReader, which isn't translated (it implements `Closeable` (java.io)).
        // CommentedProperties.LineReader.nextLine(0): not translated, its class isn't translated (it implements `Closeable` (java.io)).
        // CommentedProperties.LineReader.close(0): not translated, its class isn't translated (it implements `Closeable` (java.io)).
    }

    /// `CommentedProperties.Pair` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java)
    pub mod commented_properties_pair {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Pair {
            pub key: String,
            pub value: String,
        }
        impl Pair {
            pub fn new(key: &str, value: &str) -> Self {
                let mut s = Self { key: String::new(), value: String::new() };
                s.key = key.to_string();
                s.value = value.to_string();
                s
            }
        }
    }

    /// `CommentedProperties.Property` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedProperties.java)
    pub mod commented_properties_property {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Property {
            pub comments: Option<Vec<String>>,
            pub value: Option<String>,
        }
        impl Property {
            pub fn new_2(comments: &[String], value: &str) -> Self {
                let mut s = Self { comments: Some(comments.to_vec()), value: Some(value.to_string()) };
                s.comments = Some(comments.to_vec());
                s.value = Some(value.to_string());
                s
            }
            pub fn new_1(value: &str) -> Self {
                let mut s = Self::new_2(&Vec::new(), &value);
                s
            }
        }
    }

    /// `CommentedPropertyConfig` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java)
    pub mod commented_property_config {
        use super::*;
        // The class itself isn't translated: it implements `Config`. Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        // SAVE_EXECUTOR_SERVICE: not translated, it uses `ExecutorService` (java.util.concurrent).
        // CommentedPropertyConfig.CommentedPropertyConfig(1): not translated, its class isn't translated (it implements `Config`).
        // CommentedPropertyConfig.builder(0): not translated, it creates a Builder with a constructor that isn't translated.
        // CommentedPropertyConfig.get(1): not translated, its class isn't translated (it implements `Config`).
        // CommentedPropertyConfig.set(3): not translated, its class isn't translated (it implements `Config`).
        // CommentedPropertyConfig.getProperties(0): not translated, its class isn't translated (it implements `Config`).
        // CommentedPropertyConfig.load(0): not translated, its class isn't translated (it implements `Config`).
        // CommentedPropertyConfig.reload(0): not translated, its class isn't translated (it implements `Config`).
        // CommentedPropertyConfig.saveSync(0): not translated, its class isn't translated (it implements `Config`).
        // CommentedPropertyConfig.save(0): not translated, its class isn't translated (it implements `Config`).
        // CommentedPropertyConfig.getEntries(0): not translated, its class isn't translated (it implements `Config`).
    }

    /// `CommentedPropertyConfig.Builder` (de/maxhenkel/sound_physics_remastered/configbuilder/CommentedPropertyConfig.java)
    pub mod commented_property_config_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
            pub path: std::path::PathBuf,
            pub strict: bool,
        }
        impl Builder {
            // CommentedPropertyConfig.Builder.Builder(0): not translated, a Path with no value (null in Java).
            // CommentedPropertyConfig.Builder.path(1): not translated, `this` used as a value.
            // CommentedPropertyConfig.Builder.strict(1): not translated, `this` used as a value.
            // CommentedPropertyConfig.Builder.build(0): not translated, it uses CommentedPropertyConfig, which isn't translated (it implements `Config`).
        }
    }

    /// `Config` (de/maxhenkel/sound_physics_remastered/configbuilder/Config.java)
    pub mod config {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Config.getEntries(0): not translated, it has no body.
    }

    /// `ConfigBuilder` (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java)
    pub mod config_builder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ConfigBuilder.header(1): not translated, it has no body.
        // ConfigBuilder.booleanEntry(3): not translated, it has no body.
        // ConfigBuilder.integerEntry(5): not translated, it has no body.
        // ConfigBuilder.integerEntry(3): not translated, its class isn't translated (it's an interface).
        // ConfigBuilder.longEntry(5): not translated, it has no body.
        // ConfigBuilder.longEntry(3): not translated, its class isn't translated (it's an interface).
        // ConfigBuilder.doubleEntry(5): not translated, it has no body.
        // ConfigBuilder.doubleEntry(3): not translated, its class isn't translated (it's an interface).
        // ConfigBuilder.floatEntry(5): not translated, it has no body.
        // ConfigBuilder.floatEntry(3): not translated, its class isn't translated (it's an interface).
        // ConfigBuilder.stringEntry(3): not translated, it has no body.
        // ConfigBuilder.enumEntry(3): not translated, it has no body.
        // ConfigBuilder.entry(3): not translated, it has no body.
        // ConfigBuilder.builder(1): not translated, it's generic.
    }

    /// `ConfigBuilder.Builder` (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilder.java)
    pub mod config_builder_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ConfigBuilder.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // ConfigBuilder.Builder.path(1): not translated, its class isn't translated (it's generic).
        // ConfigBuilder.Builder.addValueSerializer(2): not translated, it's generic.
        // ConfigBuilder.Builder.removeUnused(1): not translated, its class isn't translated (it's generic).
        // ConfigBuilder.Builder.strict(1): not translated, its class isn't translated (it's generic).
        // ConfigBuilder.Builder.keepOrder(1): not translated, its class isn't translated (it's generic).
        // ConfigBuilder.Builder.saveAfterBuild(1): not translated, its class isn't translated (it's generic).
        // ConfigBuilder.Builder.saveSyncAfterBuild(1): not translated, its class isn't translated (it's generic).
        // ConfigBuilder.Builder.build(0): not translated, its class isn't translated (it's generic).
    }

    /// `ConfigBuilderImpl` (de/maxhenkel/sound_physics_remastered/configbuilder/ConfigBuilderImpl.java)
    pub mod config_builder_impl {
        use super::*;
        // The class itself isn't translated: it implements `ConfigBuilder`. Its static parts are below.
        // ConfigBuilderImpl.ConfigBuilderImpl(1): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.ConfigBuilderImpl(2): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.removeUnused(0): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.sortEntries(0): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.reloadFromDisk(0): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.freeze(0): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.checkFrozen(0): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.header(1): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.booleanEntry(3): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.integerEntry(5): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.longEntry(5): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.doubleEntry(5): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.floatEntry(5): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.stringEntry(3): not translated, its class isn't translated (it implements `ConfigBuilder`).
        // ConfigBuilderImpl.enumEntry(3): not translated, it's generic.
        // ConfigBuilderImpl.entry(3): not translated, it's generic.
        // ConfigBuilderImpl.entryInternal(3): not translated, it's generic.
        // ConfigBuilderImpl.getDefaultValueSerializers(0): not translated, it uses `Class`.
    }

    /// `AbstractValueList` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueList.java)
    pub mod abstract_value_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractValueList.AbstractValueList(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.AbstractValueList(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.size(0): not translated, its class isn't translated (it's generic).
        // AbstractValueList.isEmpty(0): not translated, its class isn't translated (it's generic).
        // AbstractValueList.contains(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.iterator(0): not translated, its class isn't translated (it's generic).
        // AbstractValueList.toArray(0): not translated, its class isn't translated (it's generic).
        // AbstractValueList.toArray(1): not translated, it's generic.
        // AbstractValueList.add(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.remove(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.containsAll(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.addAll(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.addAll(2): not translated, its class isn't translated (it's generic).
        // AbstractValueList.removeAll(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.retainAll(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.clear(0): not translated, its class isn't translated (it's generic).
        // AbstractValueList.get(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.set(2): not translated, its class isn't translated (it's generic).
        // AbstractValueList.add(2): not translated, its class isn't translated (it's generic).
        // AbstractValueList.remove(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.indexOf(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.lastIndexOf(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.listIterator(0): not translated, its class isn't translated (it's generic).
        // AbstractValueList.listIterator(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.subList(2): not translated, its class isn't translated (it's generic).
        // AbstractValueList.equals(1): not translated, its class isn't translated (it's generic).
        // AbstractValueList.hashCode(0): not translated, its class isn't translated (it's generic).
        // AbstractValueList.throwException(0): not translated, it's generic.
    }

    /// `AbstractValueMap` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java)
    pub mod abstract_value_map {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractValueMap.AbstractValueMap(1): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.size(0): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.isEmpty(0): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.containsKey(1): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.containsValue(1): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.get(1): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.put(2): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.remove(1): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.putAll(1): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.clear(0): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.keySet(0): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.values(0): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.entrySet(0): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.equals(1): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.hashCode(0): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.throwException(0): not translated, it's generic.
    }

    /// `AbstractValueMap.Builder` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/AbstractValueMap.java)
    pub mod abstract_value_map_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractValueMap.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.Builder.put(2): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.Builder.putAll(1): not translated, its class isn't translated (it's generic).
        // AbstractValueMap.Builder.build(0): not translated, it has no body.
    }

    /// `IntegerList` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/IntegerList.java)
    pub mod integer_list {
        use super::*;
        // The class itself isn't translated: it extends `AbstractValueList`. Its static parts are below.
        // IntegerList.IntegerList(1): not translated, its class isn't translated (it extends `AbstractValueList`).
        // IntegerList.IntegerList(1): not translated, its class isn't translated (it extends `AbstractValueList`).
        // IntegerList.of(1): not translated, it uses IntegerList, which isn't translated (it extends `AbstractValueList`).
        // IntegerList.of(1): not translated, it uses IntegerList, which isn't translated (it extends `AbstractValueList`).
    }

    /// `IntegerListValueSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/IntegerListValueSerializer.java)
    pub mod integer_list_value_serializer {
        use super::*;
        // The class itself isn't translated: it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer). Its static parts are below.
        // INSTANCE: not translated, it uses IntegerListValueSerializer, which isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
        // IntegerListValueSerializer.deserialize(1): not translated, its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
        // IntegerListValueSerializer.serialize(1): not translated, its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
    }

    /// `StringListValueSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/StringListValueSerializer.java)
    pub mod string_list_value_serializer {
        use super::*;
        // The class itself isn't translated: it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer). Its static parts are below.
        // INSTANCE: not translated, it uses StringListValueSerializer, which isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
        // StringListValueSerializer.deserialize(1): not translated, its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
        // StringListValueSerializer.serialize(1): not translated, its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
    }

    /// `StringMapValueSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/StringMapValueSerializer.java)
    pub mod string_map_value_serializer {
        use super::*;
        // The class itself isn't translated: it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer). Its static parts are below.
        // INSTANCE: not translated, it uses StringMapValueSerializer, which isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
        pub static QUOTE_ESCAPE_PATTERN: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("\"((?:(?![\"\\\\]).|\\\\.)*)\"\\s*=\\s*\"((?:(?![\"\\\\]).|\\\\.)*)\""));
        // StringMapValueSerializer.deserialize(1): not translated, its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
        // StringMapValueSerializer.serialize(1): not translated, its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
        pub fn escape(input: &str) -> String {
            return (input.replace("\\", "\\\\")).replace("\"", "\\\"");
        }
        pub fn unescape(input: &str) -> String {
            return (input.replace("\\\"", "\"")).replace("\\\\", "\\");
        }
    }

    /// `UUIDSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/serializer/UUIDSerializer.java)
    pub mod uuid_serializer {
        use super::*;
        // The class itself isn't translated: it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer). Its static parts are below.
        // INSTANCE: not translated, it uses UUIDSerializer, which isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
        // UUIDSerializer.deserialize(1): not translated, its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
        // UUIDSerializer.serialize(1): not translated, its class isn't translated (it implements `ValueSerializer` (de.maxhenkel.sound_physics_remastered.configbuilder.entry.serializer)).
    }

    /// `StringList` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringList.java)
    pub mod string_list {
        use super::*;
        // The class itself isn't translated: it extends `AbstractValueList`. Its static parts are below.
        // StringList.StringList(1): not translated, its class isn't translated (it extends `AbstractValueList`).
        // StringList.StringList(1): not translated, its class isn't translated (it extends `AbstractValueList`).
        // StringList.of(1): not translated, it uses StringList, which isn't translated (it extends `AbstractValueList`).
        // StringList.of(1): not translated, it uses StringList, which isn't translated (it extends `AbstractValueList`).
    }

    /// `StringMap` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringMap.java)
    pub mod string_map {
        use super::*;
        // The class itself isn't translated: it extends `AbstractValueMap`. Its static parts are below.
        // StringMap.StringMap(1): not translated, its class isn't translated (it extends `AbstractValueMap`).
        // StringMap.of(1): not translated, it uses StringMap, which isn't translated (it extends `AbstractValueMap`).
        // StringMap.of(0): not translated, it uses StringMap, which isn't translated (it extends `AbstractValueMap`).
        // StringMap.builder(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `StringMap.Builder` (de/maxhenkel/sound_physics_remastered/configbuilder/custom/StringMap.java)
    pub mod string_map_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // StringMap.Builder.build(0): not translated, its class isn't translated (it extends `Builder`).
    }

    /// `AbstractConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractConfigEntry.java)
    pub mod abstract_config_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractConfigEntry.AbstractConfigEntry(5): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.reload(0): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.get(0): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.set(1): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.getKey(0): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.comment(1): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.getComments(0): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.reset(0): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.syncEntryToProperties(0): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.save(0): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.saveSync(0): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.fixValue(1): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.getDefault(0): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.getConfig(0): not translated, its class isn't translated (it's generic).
        // AbstractConfigEntry.getSerializer(0): not translated, its class isn't translated (it's generic).
    }

    /// `AbstractRangedConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/AbstractRangedConfigEntry.java)
    pub mod abstract_ranged_config_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractRangedConfigEntry.AbstractRangedConfigEntry(7): not translated, its class isn't translated (it's generic).
        // AbstractRangedConfigEntry.getMin(0): not translated, its class isn't translated (it's generic).
        // AbstractRangedConfigEntry.getMax(0): not translated, its class isn't translated (it's generic).
        // AbstractRangedConfigEntry.minimumPossibleValue(0): not translated, it has no body.
        // AbstractRangedConfigEntry.maximumPossibleValue(0): not translated, it has no body.
    }

    /// `BooleanConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/BooleanConfigEntry.java)
    pub mod boolean_config_entry {
        use super::*;
        // The class itself isn't translated: it extends `AbstractConfigEntry`. Its static parts are below.
        // BooleanConfigEntry.BooleanConfigEntry(5): not translated, its class isn't translated (it extends `AbstractConfigEntry`).
    }

    /// `ConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/ConfigEntry.java)
    pub mod config_entry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ConfigEntry.comment(1): not translated, it has no body.
        // ConfigEntry.getComments(0): not translated, it has no body.
        // ConfigEntry.get(0): not translated, it has no body.
        // ConfigEntry.set(1): not translated, it has no body.
        // ConfigEntry.getKey(0): not translated, it has no body.
        // ConfigEntry.reset(0): not translated, it has no body.
        // ConfigEntry.save(0): not translated, it has no body.
        // ConfigEntry.saveSync(0): not translated, it has no body.
        // ConfigEntry.getDefault(0): not translated, it has no body.
        // ConfigEntry.getConfig(0): not translated, it has no body.
    }

    /// `DoubleConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/DoubleConfigEntry.java)
    pub mod double_config_entry {
        use super::*;
        // The class itself isn't translated: it extends `AbstractRangedConfigEntry`. Its static parts are below.
        // DoubleConfigEntry.DoubleConfigEntry(7): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // DoubleConfigEntry.minimumPossibleValue(0): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // DoubleConfigEntry.maximumPossibleValue(0): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // DoubleConfigEntry.fixValue(1): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
    }

    /// `EnumConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/EnumConfigEntry.java)
    pub mod enum_config_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EnumConfigEntry.EnumConfigEntry(5): not translated, its class isn't translated (it's generic).
    }

    /// `FloatConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/FloatConfigEntry.java)
    pub mod float_config_entry {
        use super::*;
        // The class itself isn't translated: it extends `AbstractRangedConfigEntry`. Its static parts are below.
        // FloatConfigEntry.FloatConfigEntry(7): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // FloatConfigEntry.minimumPossibleValue(0): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // FloatConfigEntry.maximumPossibleValue(0): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // FloatConfigEntry.fixValue(1): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
    }

    /// `GenericConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/GenericConfigEntry.java)
    pub mod generic_config_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GenericConfigEntry.GenericConfigEntry(5): not translated, its class isn't translated (it's generic).
    }

    /// `IntegerConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/IntegerConfigEntry.java)
    pub mod integer_config_entry {
        use super::*;
        // The class itself isn't translated: it extends `AbstractRangedConfigEntry`. Its static parts are below.
        // IntegerConfigEntry.IntegerConfigEntry(7): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // IntegerConfigEntry.minimumPossibleValue(0): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // IntegerConfigEntry.maximumPossibleValue(0): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // IntegerConfigEntry.fixValue(1): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
    }

    /// `LongConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/LongConfigEntry.java)
    pub mod long_config_entry {
        use super::*;
        // The class itself isn't translated: it extends `AbstractRangedConfigEntry`. Its static parts are below.
        // LongConfigEntry.LongConfigEntry(7): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // LongConfigEntry.minimumPossibleValue(0): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // LongConfigEntry.maximumPossibleValue(0): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
        // LongConfigEntry.fixValue(1): not translated, its class isn't translated (it extends `AbstractRangedConfigEntry`).
    }

    /// `RangedConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/RangedConfigEntry.java)
    pub mod ranged_config_entry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RangedConfigEntry.getMin(0): not translated, it has no body.
        // RangedConfigEntry.getMax(0): not translated, it has no body.
    }

    /// `BooleanSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/BooleanSerializer.java)
    pub mod boolean_serializer {
        use super::*;
        // The class itself isn't translated: it implements `ValueSerializer`. Its static parts are below.
        // INSTANCE: not translated, it uses BooleanSerializer, which isn't translated (it implements `ValueSerializer`).
        // BooleanSerializer.deserialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
        // BooleanSerializer.serialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
    }

    /// `DoubleSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/DoubleSerializer.java)
    pub mod double_serializer {
        use super::*;
        // The class itself isn't translated: it implements `ValueSerializer`. Its static parts are below.
        // INSTANCE: not translated, it uses DoubleSerializer, which isn't translated (it implements `ValueSerializer`).
        // DoubleSerializer.deserialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
        // DoubleSerializer.serialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
    }

    /// `EnumSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/EnumSerializer.java)
    pub mod enum_serializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EnumSerializer.EnumSerializer(1): not translated, its class isn't translated (it's generic).
        // EnumSerializer.deserialize(1): not translated, its class isn't translated (it's generic).
        // EnumSerializer.serialize(1): not translated, its class isn't translated (it's generic).
    }

    /// `FloatSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/FloatSerializer.java)
    pub mod float_serializer {
        use super::*;
        // The class itself isn't translated: it implements `ValueSerializer`. Its static parts are below.
        // INSTANCE: not translated, it uses FloatSerializer, which isn't translated (it implements `ValueSerializer`).
        // FloatSerializer.deserialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
        // FloatSerializer.serialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
    }

    /// `IntegerSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/IntegerSerializer.java)
    pub mod integer_serializer {
        use super::*;
        // The class itself isn't translated: it implements `ValueSerializer`. Its static parts are below.
        // INSTANCE: not translated, it uses IntegerSerializer, which isn't translated (it implements `ValueSerializer`).
        // IntegerSerializer.deserialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
        // IntegerSerializer.serialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
    }

    /// `LongSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/LongSerializer.java)
    pub mod long_serializer {
        use super::*;
        // The class itself isn't translated: it implements `ValueSerializer`. Its static parts are below.
        // INSTANCE: not translated, it uses LongSerializer, which isn't translated (it implements `ValueSerializer`).
        // LongSerializer.deserialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
        // LongSerializer.serialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
    }

    /// `StringSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/StringSerializer.java)
    pub mod string_serializer {
        use super::*;
        // The class itself isn't translated: it implements `ValueSerializer`. Its static parts are below.
        // INSTANCE: not translated, it uses StringSerializer, which isn't translated (it implements `ValueSerializer`).
        // StringSerializer.deserialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
        // StringSerializer.serialize(1): not translated, its class isn't translated (it implements `ValueSerializer`).
    }

    /// `ValueSerializable` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/ValueSerializable.java)
    pub mod value_serializable {
        use super::*;
        // The class itself isn't translated: it's an annotation. Its static parts are below.
        // ValueSerializable.value(0): not translated, it has no body.
    }

    /// `ValueSerializer` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/serializer/ValueSerializer.java)
    pub mod value_serializer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ValueSerializer.deserialize(1): not translated, it has no body.
        // ValueSerializer.serialize(1): not translated, it has no body.
    }

    /// `StringConfigEntry` (de/maxhenkel/sound_physics_remastered/configbuilder/entry/StringConfigEntry.java)
    pub mod string_config_entry {
        use super::*;
        // The class itself isn't translated: it extends `AbstractConfigEntry`. Its static parts are below.
        // StringConfigEntry.StringConfigEntry(5): not translated, its class isn't translated (it extends `AbstractConfigEntry`).
    }

    
    // ---- Java regular expressions (java.util.regex), a small backtracking engine ----
    // Bundles can't add the regex crate, so this covers what mods use: literals, `.`, classes with ranges and
    // \d \s \w, groups (capturing, `(?:`, `(?<name>`), lookahead, alternation, greedy/lazy quantifiers, ^ $ \b.
    // Positions are in chars, like the translated String methods.
    
    #[derive(Clone, Debug)]
    enum JClassItem {
        Range(char, char),
        Digit(bool),
        Space(bool),
        Word(bool),
    }
    
    #[derive(Clone, Debug)]
    enum JNode {
        Char(char),
        Any,
        Class(Vec<JClassItem>, bool),
        Start,
        End,
        WordB(bool),
        Group(Box<JNode>, Option<usize>),
        Look(Box<JNode>, bool),
        Alt(Vec<JNode>),
        Concat(Vec<JNode>),
        Repeat(Box<JNode>, u32, u32, bool),
        Backref(usize),
    }
    
    type JCaps = Vec<Option<(usize, usize)>>;
    
    /// A compiled `Pattern`.
    #[derive(Clone, Debug)]
    pub struct JRegex {
        node: JNode,
        groups: usize,
        names: Vec<(String, usize)>,
        ci: bool,
    }
    
    fn j_is_word(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
    
    fn j_is_space(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r')
    }
    
    struct JRegexParser<'a> {
        p: &'a [char],
        i: usize,
        groups: usize,
        names: Vec<(String, usize)>,
    }
    
    impl JRegexParser<'_> {
        fn peek(&self) -> Option<char> {
            self.p.get(self.i).copied()
        }
        fn eat(&mut self, c: char) -> bool {
            if self.peek() == Some(c) {
                self.i += 1;
                true
            } else {
                false
            }
        }
        fn alt(&mut self) -> JNode {
            let mut arms = vec![self.concat()];
            while self.eat('|') {
                arms.push(self.concat());
            }
            if arms.len() == 1 { arms.pop().unwrap() } else { JNode::Alt(arms) }
        }
        fn concat(&mut self) -> JNode {
            let mut items = Vec::new();
            while let Some(c) = self.peek() {
                if c == '|' || c == ')' {
                    break;
                }
                let atom = self.atom();
                items.push(self.quantified(atom));
            }
            JNode::Concat(items)
        }
        fn number(&mut self) -> Option<u32> {
            let start = self.i;
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.i += 1;
            }
            if start == self.i { None } else { self.p[start..self.i].iter().collect::<String>().parse().ok() }
        }
        fn quantified(&mut self, atom: JNode) -> JNode {
            let (min, max) = match self.peek() {
                Some('*') => (0, u32::MAX),
                Some('+') => (1, u32::MAX),
                Some('?') => (0, 1),
                Some('{') => {
                    let save = self.i;
                    self.i += 1;
                    let Some(lo) = self.number() else {
                        self.i = save;
                        return atom;
                    };
                    let hi = if self.eat(',') { self.number().unwrap_or(u32::MAX) } else { lo };
                    if !self.eat('}') {
                        panic!("bad repetition in regex");
                    }
                    self.i -= 1;
                    (lo, hi)
                }
                _ => return atom,
            };
            self.i += 1;
            let greedy = !self.eat('?');
            if greedy {
                self.eat('+');
            }
            self.quantified(JNode::Repeat(Box::new(atom), min, max, greedy))
        }
        fn escape(&mut self, in_class: bool) -> JClassItem {
            let c = self.peek().expect("regex ends with a backslash");
            self.i += 1;
            let lit = |c: char| JClassItem::Range(c, c);
            match c {
                'd' => JClassItem::Digit(true),
                'D' => JClassItem::Digit(false),
                's' => JClassItem::Space(true),
                'S' => JClassItem::Space(false),
                'w' => JClassItem::Word(true),
                'W' => JClassItem::Word(false),
                't' => lit('\t'),
                'n' => lit('\n'),
                'r' => lit('\r'),
                'f' => lit('\u{0C}'),
                'e' => lit('\u{1B}'),
                '0' => lit('\0'),
                'u' => {
                    let hex: String = self.p[self.i..self.i + 4].iter().collect();
                    self.i += 4;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\u escape")).unwrap_or('\u{FFFD}'))
                }
                'x' => {
                    let hex: String = self.p[self.i..self.i + 2].iter().collect();
                    self.i += 2;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\x escape")).unwrap_or('\u{FFFD}'))
                }
                c if !in_class && c.is_ascii_alphabetic() => panic!("unsupported regex escape \\{c}"),
                c => lit(c),
            }
        }
        fn class(&mut self) -> JNode {
            let negated = self.eat('^');
            let mut items = Vec::new();
            let mut first = true;
            loop {
                let c = self.peek().expect("unclosed [ in regex");
                if c == ']' && !first {
                    self.i += 1;
                    break;
                }
                first = false;
                if c == '[' {
                    panic!("nested character classes aren't supported");
                }
                self.i += 1;
                let item = if c == '\\' { self.escape(true) } else { JClassItem::Range(c, c) };
                // a-z
                if let JClassItem::Range(lo, _) = item
                    && self.peek() == Some('-')
                    && self.p.get(self.i + 1).is_some_and(|&n| n != ']')
                {
                    self.i += 1;
                    let hc = self.peek().unwrap();
                    self.i += 1;
                    let hi = if hc == '\\' {
                        match self.escape(true) {
                            JClassItem::Range(h, _) => h,
                            _ => panic!("bad range in regex"),
                        }
                    } else {
                        hc
                    };
                    items.push(JClassItem::Range(lo, hi));
                    continue;
                }
                items.push(item);
            }
            JNode::Class(items, negated)
        }
        fn atom(&mut self) -> JNode {
            let c = self.peek().unwrap();
            self.i += 1;
            match c {
                '.' => JNode::Any,
                '^' => JNode::Start,
                '$' => JNode::End,
                '[' => self.class(),
                '(' => {
                    let mut index = None;
                    let mut look = None;
                    if self.eat('?') {
                        if self.eat(':') {
                        } else if self.eat('=') {
                            look = Some(false);
                        } else if self.eat('!') {
                            look = Some(true);
                        } else if self.eat('<') {
                            let start = self.i;
                            while self.peek().is_some_and(|c| c != '>') {
                                self.i += 1;
                            }
                            let name: String = self.p[start..self.i].iter().collect();
                            self.i += 1;
                            self.groups += 1;
                            self.names.push((name, self.groups));
                            index = Some(self.groups);
                        } else {
                            panic!("unsupported regex group (?{:?}", self.peek());
                        }
                    } else {
                        self.groups += 1;
                        index = Some(self.groups);
                    }
                    let inner = self.alt();
                    if !self.eat(')') {
                        panic!("unclosed ( in regex");
                    }
                    match look {
                        Some(neg) => JNode::Look(Box::new(inner), neg),
                        None => JNode::Group(Box::new(inner), index),
                    }
                }
                '\\' => match self.peek() {
                    Some('b') => {
                        self.i += 1;
                        JNode::WordB(true)
                    }
                    Some('B') => {
                        self.i += 1;
                        JNode::WordB(false)
                    }
                    Some('A') => {
                        self.i += 1;
                        JNode::Start
                    }
                    Some('z') | Some('Z') => {
                        self.i += 1;
                        JNode::End
                    }
                    Some(d) if d.is_ascii_digit() && d != '0' => {
                        self.i += 1;
                        JNode::Backref(d as usize - '0' as usize)
                    }
                    Some('Q') => {
                        self.i += 1;
                        let mut lits = Vec::new();
                        while self.i < self.p.len() && !(self.p[self.i] == '\\' && self.p.get(self.i + 1) == Some(&'E')) {
                            lits.push(JNode::Char(self.p[self.i]));
                            self.i += 1;
                        }
                        self.i = (self.i + 2).min(self.p.len());
                        JNode::Concat(lits)
                    }
                    _ => match self.escape(false) {
                        JClassItem::Range(c, _) => JNode::Char(c),
                        other => JNode::Class(vec![other], false),
                    },
                },
                c => JNode::Char(c),
            }
        }
    }
    
    impl JRegex {
        /// `Pattern.compile(p)`. Java throws PatternSyntaxException on a bad pattern; this panics.
        pub fn new(p: &str) -> Self {
            Self::with_flags(p, 0)
        }
        /// `Pattern.compile(p, flags)`: CASE_INSENSITIVE (2) is honoured.
        pub fn with_flags(p: &str, flags: i32) -> Self {
            let chars: Vec<char> = p.chars().collect();
            let (chars, ci) = match chars.strip_prefix(&['(', '?', 'i', ')']) {
                Some(rest) => (rest.to_vec(), true),
                None => (chars, flags & 2 != 0),
            };
            let mut parser = JRegexParser { p: &chars, i: 0, groups: 0, names: Vec::new() };
            let node = parser.alt();
            if parser.i != chars.len() {
                panic!("unmatched ) in regex {p:?}");
            }
            JRegex { node, groups: parser.groups, names: parser.names, ci }
        }
    
        fn same(&self, a: char, b: char) -> bool {
            a == b || self.ci && (j_lower_char(a) == j_lower_char(b) || j_upper_char(a) == j_upper_char(b))
        }
    
        fn in_class(&self, items: &[JClassItem], c: char) -> bool {
            items.iter().any(|it| match *it {
                JClassItem::Range(lo, hi) => {
                    (lo..=hi).contains(&c) || self.ci && ((lo..=hi).contains(&j_lower_char(c)) || (lo..=hi).contains(&j_upper_char(c)))
                }
                JClassItem::Digit(yes) => c.is_ascii_digit() == yes,
                JClassItem::Space(yes) => j_is_space(c) == yes,
                JClassItem::Word(yes) => j_is_word(c) == yes,
            })
        }
    
        fn m(&self, n: &JNode, s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match n {
                JNode::Char(c) => i < s.len() && self.same(s[i], *c) && k(i + 1, caps),
                JNode::Any => i < s.len() && !matches!(s[i], '\n' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}') && k(i + 1, caps),
                JNode::Class(items, neg) => i < s.len() && self.in_class(items, s[i]) != *neg && k(i + 1, caps),
                JNode::Start => i == 0 && k(i, caps),
                JNode::End => (i == s.len() || i + 1 == s.len() && s[i] == '\n') && k(i, caps),
                JNode::WordB(yes) => {
                    let before = i > 0 && j_is_word(s[i - 1]);
                    let after = i < s.len() && j_is_word(s[i]);
                    (before != after) == *yes && k(i, caps)
                }
                JNode::Group(inner, index) => match *index {
                    None => self.m(inner, s, i, caps, k),
                    Some(g) => self.m(inner, s, i, caps, &mut |j, caps: &mut JCaps| {
                        let prev = caps[g];
                        caps[g] = Some((i, j));
                        if k(j, caps) {
                            return true;
                        }
                        caps[g] = prev;
                        false
                    }),
                },
                JNode::Look(inner, neg) => {
                    let mut probe = caps.clone();
                    let found = self.m(inner, s, i, &mut probe, &mut |_, _| true);
                    if found == *neg {
                        return false;
                    }
                    if !neg {
                        *caps = probe;
                    }
                    k(i, caps)
                }
                JNode::Alt(arms) => arms.iter().any(|a| self.m(a, s, i, caps, k)),
                JNode::Concat(items) => self.seq(items, s, i, caps, k),
                JNode::Repeat(inner, min, max, greedy) => self.rep(inner, *min, *max, *greedy, 0, s, i, caps, k),
                JNode::Backref(g) => match caps.get(*g).copied().flatten() {
                    Some((a, b)) => {
                        let len = b - a;
                        i + len <= s.len() && (0..len).all(|d| self.same(s[a + d], s[i + d])) && k(i + len, caps)
                    }
                    None => false,
                },
            }
        }
    
        fn seq(&self, items: &[JNode], s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match items.split_first() {
                None => k(i, caps),
                Some((first, rest)) => self.m(first, s, i, caps, &mut |j, caps: &mut JCaps| self.seq(rest, s, j, caps, k)),
            }
        }
    
        #[allow(clippy::too_many_arguments)]
        fn rep(&self, n: &JNode, min: u32, max: u32, greedy: bool, count: u32, s: &[char], i: usize, caps: &mut JCaps,
               k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            let more = |caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool| {
                count < max
                    && self.m(n, s, i, caps, &mut |j, caps: &mut JCaps| {
                        // An empty pass can't make progress: stop repeating once the minimum is met.
                        (j != i || count < min) && self.rep(n, min, max, greedy, count + 1, s, j, caps, k)
                    })
            };
            if count < min {
                return more(caps, k);
            }
            if greedy { more(caps, k) || k(i, caps) } else { k(i, caps) || more(caps, k) }
        }
    
        /// A match starting exactly at `i`; `whole`: it must also end at the end of the text.
        fn at(&self, s: &[char], i: usize, whole: bool) -> Option<JCaps> {
            let mut caps: JCaps = vec![None; self.groups + 1];
            let mut end = None;
            let ok = self.m(&self.node, s, i, &mut caps, &mut |j, _| {
                if whole && j != s.len() {
                    return false;
                }
                end = Some(j);
                true
            });
            if !ok {
                return None;
            }
            caps[0] = Some((i, end.unwrap()));
            Some(caps)
        }
    
        fn find_from(&self, s: &[char], from: usize) -> Option<JCaps> {
            (from..=s.len()).find_map(|i| self.at(s, i, false))
        }
    
        /// `String.matches` / `Pattern.matches`: the whole text.
        pub fn matches(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.at(&cs, 0, true).is_some()
        }
    
        /// `Pattern.matcher(s).find()` as a test.
        pub fn is_found(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.find_from(&cs, 0).is_some()
        }
    
        fn expand(&self, s: &[char], caps: &JCaps, rep: &str, out: &mut String) {
            let r: Vec<char> = rep.chars().collect();
            let mut i = 0;
            while i < r.len() {
                match r[i] {
                    '\\' if i + 1 < r.len() => {
                        out.push(r[i + 1]);
                        i += 2;
                    }
                    '$' if i + 1 < r.len() && r[i + 1].is_ascii_digit() => {
                        let g = r[i + 1] as usize - '0' as usize;
                        if let Some((a, b)) = caps.get(g).copied().flatten() {
                            out.extend(&s[a..b]);
                        } else if g > self.groups {
                            panic!("No group {g}");
                        }
                        i += 2;
                    }
                    '$' if r.get(i + 1) == Some(&'{') => {
                        let close = r[i..].iter().position(|&c| c == '}').expect("bad ${name} in replacement") + i;
                        let name: String = r[i + 2..close].iter().collect();
                        let g = self.names.iter().find(|(n, _)| *n == name).map(|(_, g)| *g).expect("no group with that name");
                        if let Some((a, b)) = caps[g] {
                            out.extend(&s[a..b]);
                        }
                        i = close + 1;
                    }
                    c => {
                        out.push(c);
                        i += 1;
                    }
                }
            }
        }
    
        fn replace(&self, s: &str, rep: &str, all: bool) -> String {
            let cs: Vec<char> = s.chars().collect();
            let mut out = String::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                out.extend(&cs[pos..a]);
                self.expand(&cs, &caps, rep, &mut out);
                pos = b;
                from = if b == a { b + 1 } else { b };
                if b == a && a < cs.len() {
                    out.push(cs[a]);
                    pos = a + 1;
                }
                if !all {
                    break;
                }
            }
            if pos <= cs.len() {
                out.extend(&cs[pos..]);
            }
            out
        }
    
        /// `String.replaceAll` / `Matcher.replaceAll`, with Java's `$1` and `\$` in the replacement.
        pub fn replace_all(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, true)
        }
    
        pub fn replace_first(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, false)
        }
    
        /// `String.split(regex, limit)` / `Pattern.split`, with Java's rules for empty parts.
        pub fn split(&self, s: &str, limit: i32) -> Vec<String> {
            let cs: Vec<char> = s.chars().collect();
            let mut parts: Vec<String> = Vec::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                if limit > 0 && parts.len() as i32 == limit - 1 {
                    break;
                }
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                if b == 0 {
                    // A zero-width match at the start makes no leading empty part.
                    from = 1;
                    continue;
                }
                if a == b && a >= cs.len() {
                    break;
                }
                parts.push(cs[pos..a].iter().collect());
                pos = b;
                from = if a == b { b + 1 } else { b };
            }
            parts.push(cs[pos.min(cs.len())..].iter().collect());
            if limit == 0 {
                while parts.len() > 1 && parts.last().is_some_and(|p| p.is_empty()) {
                    parts.pop();
                }
                if parts.len() == 1 && parts[0].is_empty() && !s.is_empty() {
                    parts.clear();
                }
            }
            parts
        }
    }
    
    /// A `Matcher`: a regex over one text, with the last match.
    #[derive(Clone, Debug)]
    pub struct JMatcher {
        re: JRegex,
        text: Vec<char>,
        from: usize,
        caps: Option<JCaps>,
    }
    
    impl JMatcher {
        pub fn new(re: &JRegex, s: &str) -> Self {
            JMatcher { re: re.clone(), text: s.chars().collect(), from: 0, caps: None }
        }
        pub fn find(&mut self) -> bool {
            if self.from > self.text.len() {
                self.caps = None;
                return false;
            }
            self.caps = self.re.find_from(&self.text, self.from);
            if let Some((a, b)) = self.caps.as_ref().and_then(|c| c[0]) {
                self.from = if a == b { b + 1 } else { b };
                true
            } else {
                self.from = self.text.len() + 1;
                false
            }
        }
        pub fn find_at(&mut self, start: i32) -> bool {
            self.from = start as usize;
            self.find()
        }
        pub fn matches(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, true);
            self.caps.is_some()
        }
        pub fn looking_at(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, false);
            self.caps.is_some()
        }
        fn span(&self, g: usize) -> Option<(usize, usize)> {
            let caps = self.caps.as_ref().expect("No match found");
            assert!(g < caps.len(), "No group {g}");
            caps[g]
        }
        /// `group(n)`: null (None) when that group took no part in the match.
        pub fn group(&self, g: i32) -> Option<String> {
            self.span(g as usize).map(|(a, b)| self.text[a..b].iter().collect())
        }
        pub fn group_named(&self, name: &str) -> Option<String> {
            let g = self.re.names.iter().find(|(n, _)| n == name).map(|(_, g)| *g).expect("No group with that name");
            self.group(g as i32)
        }
        pub fn start(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(a, _)| a as i32)
        }
        pub fn end(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(_, b)| b as i32)
        }
        pub fn group_count(&self) -> i32 {
            self.re.groups as i32
        }
        pub fn replace_all(&self, rep: &str) -> String {
            self.re.replace_all(&self.text.iter().collect::<String>(), rep)
        }
    }
    
    #[cfg(test)]
    mod jregex_tests {
        use super::*;
    
        #[test]
        fn groups_and_lazy() {
            let re = JRegex::new("^(?:\\S )?(?:VERY SPECIAL|COMMON|RARE|EPIC)(?: DUNGEON)?(?: (.+?))?(?: \\S)?$");
            let mut m = JMatcher::new(&re, "EPIC DUNGEON SWORD");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("SWORD"));
            let mut m = JMatcher::new(&re, "a RARE FISHING ROD b");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("FISHING ROD"));
            let mut m = JMatcher::new(&re, "COMMON");
            assert!(m.matches());
            assert_eq!(m.group(1), None);
        }
    
        #[test]
        fn classes_counts_find() {
            let re = JRegex::new("server/[0-9A-Fa-f]{8}/(.+)");
            let mut m = JMatcher::new(&re, "server/0aF3b2c1/pack.zip");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("pack.zip"));
            assert!(!JRegex::new("server/[0-9A-Fa-f]{8}/(.+)").matches("server/xyz/a"));
            let re = JRegex::new("\"id\"\\s*:\\s*\"([a-z0-9_.-]+:[a-z0-9_./-]+)\"");
            let mut m = JMatcher::new(&re, "{\"id\": \"minecraft:item/a\", \"id\":\"x:y\"}");
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("minecraft:item/a"));
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("x:y"));
            assert!(!m.find());
        }
    
        #[test]
        fn split_and_replace() {
            assert_eq!(JRegex::new("\\s*,\\s*").split("a , b,c,,", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new(",").split("a,b,c", 2), vec!["a", "b,c"]);
            assert_eq!(JRegex::new("").split("abc", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new("(\\w+)@(\\w+)").replace_all("x a@b y c@d", "$2.$1"), "x b.a y d.c");
            assert_eq!(JRegex::new("a*").replace_all("baaac", "-"), "-b--c-");
            assert!(JRegex::with_flags("abc", 2).matches("AbC"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "sound_physics_remastered");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
