// The Graveyard, ported to Horizon by horizon-port from graveyard-FABRIC-4.0.0+mc26.3.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_graveyard). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "graveyard";
pub const NAME: &str = "The Graveyard";
pub const VERSION: &str = "4.0.0+mc26.3";
pub const DESCRIPTION: &str = "Adds new bosses, structures, mobs and blocks themed around the graveyard.";

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

    /// `PlatformMethods` (architectury_inject_graveyard_common_b80d6c58b4ee4764bbf9e4837d6706ef_60a3283155ee0c308a142f8b9590e1717e056ecc29148714b8674935119ff19cgraveyard400mc263devjar/PlatformMethods.java)
    pub mod platform_methods {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlatformMethods {
        }
        impl PlatformMethods {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn get_current_target() -> String {
            return "fabric".to_string();
        }
    }

    /// `TGAdvancementTrigger` (com/lion/graveyard/advancements/TGAdvancementTrigger.java)
    pub mod tg_advancement_trigger {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleCriterionTrigger`. Its static parts are below.
        // TGAdvancementTrigger.trigger(1): not translated, its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // TGAdvancementTrigger.codec(0): not translated, its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
    }

    /// `TGAdvancementTrigger.Condition` (com/lion/graveyard/advancements/TGAdvancementTrigger.java)
    pub mod tg_advancement_trigger_condition {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `SimpleInstance`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // TGAdvancementTrigger.Condition.Condition(1): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
        // TGAdvancementTrigger.Condition.player(0): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
    }

    /// `SarcophagusLidAnimator` (com/lion/graveyard/blockentities/animation/SarcophagusLidAnimator.java)
    pub mod sarcophagus_lid_animator {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SarcophagusLidAnimator {
            pub open: bool,
            pub progress: f32,
            pub last_progress: f32,
        }
        impl SarcophagusLidAnimator {
            pub fn new() -> Self {
                let mut s = Self { open: false, progress: 0.0f32, last_progress: 0.0f32 };
                s
            }
            pub fn step(&mut self) {
                self.last_progress = self.progress;
                let f: f32 = 0.1f32;
                if (!self.open) && (self.progress > 0.0f32) {
                    self.progress = (self.progress - 0.1f32).max(0.0f32);
                } else if self.open && (self.progress < 1.0f32) {
                    self.progress = (self.progress + 0.1f32).min(1.0f32);
                }
            }
            pub fn get_progress(&self, delta: f32) -> f32 {
                return j_lerp(delta, self.last_progress, self.progress);
            }
            pub fn set_open(&mut self, open: bool) {
                self.open = open;
            }
        }
    }

    /// `BrazierBlockEntity` (com/lion/graveyard/blockentities/BrazierBlockEntity.java)
    pub mod brazier_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        // BrazierBlockEntity.BrazierBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BrazierBlockEntity.registerControllers(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BrazierBlockEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
    }

    /// `SarcophagusPart` (com/lion/graveyard/blockentities/enums/SarcophagusPart.java)
    pub mod sarcophagus_part {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // SarcophagusPart.SarcophagusPart(1): not translated, its class isn't translated (its constants carry values).
        // SarcophagusPart.toString(0): not translated, its class isn't translated (its constants carry values).
        // SarcophagusPart.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `GravestoneBlockEntity` (com/lion/graveyard/blockentities/GravestoneBlockEntity.java)
    pub mod gravestone_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        pub static LEGACY_LINE_KEYS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["messages".to_string(), "filtered_messages".to_string()]);
        // GravestoneBlockEntity.GravestoneBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.GravestoneBlockEntity(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.createText(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.getText(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.getTextLineHeight(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.getMaxTextWidth(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.convertLegacyText(3): not translated, it uses Minecraft's `CompoundTag`.
        // GravestoneBlockEntity.convertLegacyLines(2): not translated, it uses Minecraft's `CompoundTag`.
        // GravestoneBlockEntity.convertLegacyLine(2): not translated, it uses Minecraft's `Tag`.
        // GravestoneBlockEntity.parseLines(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.parseLine(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.tryChangeText(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.changeText(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.getTextWithMessages(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.setText(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.setFrontText(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.runCommandClickEvent(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.createCommandSource(3): not translated, it uses Minecraft's `CommandSourceStack`.
        // GravestoneBlockEntity.getUpdatePacket(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.getUpdateTag(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.setEditor(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.getEditor(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.updateListeners(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.isWaxed(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.setWaxed(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.isPlayerTooFarToEdit(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntity.tick(4): not translated, it uses Minecraft's `Level`.
        // GravestoneBlockEntity.tryClearInvalidEditor(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
    }

    /// `LoreBookBlockEntity` (com/lion/graveyard/blockentities/LoreBookBlockEntity.java)
    pub mod lore_book_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        pub const PAGE_KEY: &str = "page";
        // LoreBookBlockEntity.LoreBookBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // LoreBookBlockEntity.getPage(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // LoreBookBlockEntity.setPage(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // LoreBookBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // LoreBookBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // LoreBookBlockEntity.getUpdateTag(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // LoreBookBlockEntity.getUpdatePacket(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
    }

    /// `BrazierModel` (com/lion/graveyard/blockentities/models/BrazierModel.java)
    pub mod brazier_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // BrazierModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // BrazierModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // BrazierModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `OssuaryModel` (com/lion/graveyard/blockentities/models/OssuaryModel.java)
    pub mod ossuary_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // OssuaryModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // OssuaryModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // OssuaryModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `OssuaryBlockEntity` (com/lion/graveyard/blockentities/OssuaryBlockEntity.java)
    pub mod ossuary_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        // OssuaryBlockEntity.OssuaryBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // OssuaryBlockEntity.registerControllers(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // OssuaryBlockEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // OssuaryBlockEntity.tick(4): not translated, it uses Minecraft's `Level`.
    }

    /// `BrazierBlockEntityRenderer` (com/lion/graveyard/blockentities/renders/BrazierBlockEntityRenderer.java)
    pub mod brazier_block_entity_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // BrazierBlockEntityRenderer.BrazierBlockEntityRenderer(1): not translated, its class isn't translated (it's generic).
        // BrazierBlockEntityRenderer.getViewDistance(0): not translated, its class isn't translated (it's generic).
        // BrazierBlockEntityRenderer.getRenderType(2): not translated, its class isn't translated (it's generic).
    }

    /// `GravestoneBlockEntityRenderer` (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java)
    pub mod gravestone_block_entity_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockEntityRenderer`. Its static parts are below.
        pub static RENDER_DISTANCE: LazyLock<i32> = LazyLock::new(|| 16 * 16);
        // GravestoneBlockEntityRenderer.GravestoneBlockEntityRenderer(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // GravestoneBlockEntityRenderer.createRenderState(0): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // GravestoneBlockEntityRenderer.extractRenderState(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // GravestoneBlockEntityRenderer.submit(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // GravestoneBlockEntityRenderer.submitGrave(3): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // GravestoneBlockEntityRenderer.shouldRender(2): not translated, it uses GravestoneBlockEntity, which isn't translated (it extends Minecraft's `BlockEntity`).
        // GravestoneBlockEntityRenderer.getColor(1): not translated, it uses GravestoneBlockEntity, which isn't translated (it extends Minecraft's `BlockEntity`).
    }

    /// `GravestoneBlockEntityRenderer.CrumblingItemCollector` (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java)
    pub mod gravestone_block_entity_renderer_crumbling_item_collector {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `SubmitNodeCollector`. Its static parts are below.
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.CrumblingItemCollector(2): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.wrap(2): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitItem(8): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.order(1): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitShadow(3): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitNameTag(7): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitText(10): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitTextBackground(8): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitFlame(3): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitLeash(2): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitModel(9): not translated, it's generic.
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitCrumblingOverlay(8): not translated, it's generic.
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitMovingBlock(3): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitBlockModel(7): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitBreakingBlockModel(4): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitShapeOutline(6): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitCustomGeometry(3): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitQuadParticleGroup(1): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
        // GravestoneBlockEntityRenderer.CrumblingItemCollector.submitGizmoPrimitives(3): not translated, its class isn't translated (it implements Minecraft's `SubmitNodeCollector`).
    }

    /// `GravestoneBlockEntityRenderer.QuadModel` (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java)
    pub mod gravestone_block_entity_renderer_quad_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Model`. Its static parts are below.
        // GravestoneBlockEntityRenderer.QuadModel.QuadModel(2): not translated, its class isn't translated (it extends Minecraft's `Model`).
        // GravestoneBlockEntityRenderer.QuadModel.createRoot(1): not translated, it uses Minecraft's `ModelPart`.
    }

    /// `GravestoneBlockEntityRenderer.State` (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java)
    pub mod gravestone_block_entity_renderer_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntityRenderState`. Its static parts are below.
    }

    /// `OssuaryBlockEntityRenderer` (com/lion/graveyard/blockentities/renders/OssuaryBlockEntityRenderer.java)
    pub mod ossuary_block_entity_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // OssuaryBlockEntityRenderer.OssuaryBlockEntityRenderer(1): not translated, its class isn't translated (it's generic).
        // OssuaryBlockEntityRenderer.getRenderType(2): not translated, its class isn't translated (it's generic).
    }

    /// `SarcophagusBlockEntityRenderer` (com/lion/graveyard/blockentities/renders/SarcophagusBlockEntityRenderer.java)
    pub mod sarcophagus_block_entity_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SarcophagusBlockEntityRenderer.SarcophagusBlockEntityRenderer(1): not translated, its class isn't translated (it's generic).
        // SarcophagusBlockEntityRenderer.createRenderState(0): not translated, its class isn't translated (it's generic).
        // SarcophagusBlockEntityRenderer.getViewDistance(0): not translated, its class isn't translated (it's generic).
        // SarcophagusBlockEntityRenderer.getRenderBoundingBox(1): not translated, its class isn't translated (it's generic).
        // SarcophagusBlockEntityRenderer.extractRenderState(5): not translated, its class isn't translated (it's generic).
        // SarcophagusBlockEntityRenderer.submit(4): not translated, its class isn't translated (it's generic).
        // SarcophagusBlockEntityRenderer.submitPart(5): not translated, its class isn't translated (it's generic).
        // SarcophagusBlockEntityRenderer.updateModel(3): not translated, its class isn't translated (it's generic).
    }

    /// `SarcophagusBlockEntityRenderer.State` (com/lion/graveyard/blockentities/renders/SarcophagusBlockEntityRenderer.java)
    pub mod sarcophagus_block_entity_renderer_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntityRenderState`. Its static parts are below.
    }

    /// `SarcophagusBlockEntity` (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java)
    pub mod sarcophagus_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RandomizableContainerBlockEntity`. Its static parts are below.
        // SarcophagusBlockEntity.SarcophagusBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.getName(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.getCustomName(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.applyImplicitComponents(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.collectImplicitComponents(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.getContainerSize(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.getItems(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.setItems(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.getDefaultName(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.createMenu(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.startOpen(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.stopOpen(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.onScheduledTick(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.onInvOpenOrClose(5): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.playSound(4): not translated, it uses Minecraft's `Level`.
        // SarcophagusBlockEntity.clientTick(4): not translated, it uses Minecraft's `Level`.
        // SarcophagusBlockEntity.triggerEvent(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.getOpenNess(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // SarcophagusBlockEntity.isCoffin(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
    }

    /// `SmolderingDebrisBlockEntity` (com/lion/graveyard/blockentities/SmolderingDebrisBlockEntity.java)
    pub mod smoldering_debris_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        // SmolderingDebrisBlockEntity.SmolderingDebrisBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // SmolderingDebrisBlockEntity.clientTick(4): not translated, it uses Minecraft's `Level`.
    }

    /// `UrnBlockEntity` (com/lion/graveyard/blockentities/UrnBlockEntity.java)
    pub mod urn_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RandomizableContainerBlockEntity`. Its static parts are below.
        // UrnBlockEntity.UrnBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.getName(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.getCustomName(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.applyImplicitComponents(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.collectImplicitComponents(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.getContainerSize(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.getItems(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.setItems(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.getDefaultName(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.createMenu(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.startOpen(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.stopOpen(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.onScheduledTick(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.onInvOpenOrClose(5): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // UrnBlockEntity.playSound(4): not translated, it uses Minecraft's `Level`.
    }

    /// `AltarBlock` (com/lion/graveyard/blocks/AltarBlock.java)
    pub mod altar_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BLOODY: not translated, it uses Minecraft's `BooleanProperty`.
        // COMPLETED_ALTAR: not translated, it uses Minecraft's `BlockPattern`.
        // ANY_ORDER_ALTAR: not translated, it uses Minecraft's `BlockPattern`.
        // AltarBlock.AltarBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // AltarBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // AltarBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // AltarBlock.getCompletedFramePattern(0): not translated, it uses Minecraft's `BlockPattern`.
        // AltarBlock.getAnyOrderFramePattern(0): not translated, it uses Minecraft's `BlockPattern`.
        // AltarBlock.findSummonFailure(3): not translated, it uses Minecraft's `Level`.
        // AltarBlock.hasLivingLich(2): not translated, it uses Minecraft's `Level`.
        // AltarBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `AltarDirectionBlock` (com/lion/graveyard/blocks/AltarDirectionBlock.java)
    pub mod altar_direction_block {
        use super::*;
        // The class itself isn't translated: it extends `AltarParticleBlock`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // AltarDirectionBlock.AltarDirectionBlock(1): not translated, its class isn't translated (it extends `AltarParticleBlock`).
        // AltarDirectionBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `AltarParticleBlock`).
        // AltarDirectionBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends `AltarParticleBlock`).
        // AltarDirectionBlock.rotate(2): not translated, its class isn't translated (it extends `AltarParticleBlock`).
    }

    /// `AltarParticleBlock` (com/lion/graveyard/blocks/AltarParticleBlock.java)
    pub mod altar_particle_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // AltarParticleBlock.AltarParticleBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // AltarParticleBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockItemTooltip` (com/lion/graveyard/blocks/BlockItemTooltip.java)
    pub mod block_item_tooltip {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BlockItemTooltip.appendItemTooltip(1): not translated, it has no body.
    }

    /// `BoneDisplayBlock` (com/lion/graveyard/blocks/BoneDisplayBlock.java)
    pub mod bone_display_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // BoneDisplayBlock.BoneDisplayBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BoneDisplayBlock.propagatesSkylightDown(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BoneDisplayBlock.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BoneDisplayBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BoneDisplayBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BoneDisplayBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BoneDisplayBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BoneDisplayBlock.isCollisionShapeFullBlock(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BrazierBlock` (com/lion/graveyard/blocks/BrazierBlock.java)
    pub mod brazier_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // LIT: not translated, it uses Minecraft's `BooleanProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // STATE_TO_LUMINANCE: not translated, it uses Minecraft's `BlockState`.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BrazierBlock.BrazierBlock(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.isPathfindable(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.placeLiquid(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.onProjectileHit(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BrazierBlock.extinguish(4): not translated, it uses Minecraft's `Player`.
        // BrazierBlock.spawnSmokeParticle(3): not translated, it uses Minecraft's `Level`.
        // BrazierBlock.setLit(4): not translated, it uses Minecraft's `LevelAccessor`.
    }

    /// `CandleHolderBlock` (com/lion/graveyard/blocks/CandleHolderBlock.java)
    pub mod candle_holder_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE_NORTH: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_SOUTH: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_EAST: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_WEST: not translated, it uses Minecraft's `VoxelShape`.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // CandleHolderBlock.CandleHolderBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CandleHolderBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CandleHolderBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CandleHolderBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CandleHolderBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CandleHolderBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // CandleHolderBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `DarkIronBars` (com/lion/graveyard/blocks/DarkIronBars.java)
    pub mod dark_iron_bars {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `IronBarsBlock`. Its static parts are below.
        // DarkIronBars.DarkIronBars(1): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
    }

    /// `GravestoneBlock` (com/lion/graveyard/blocks/GravestoneBlock.java)
    pub mod gravestone_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `StandingSignBlock`. Its static parts are below.
        // ROTATION: not translated, it uses Minecraft's `IntegerProperty`.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // FLOOR: not translated, it uses Minecraft's `BooleanProperty`.
        // SHAPE_FACING_EW: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_FACING_NS: not translated, it uses Minecraft's `VoxelShape`.
        // GravestoneBlock.GravestoneBlock(2): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.useDyeOnSign(3): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.useInkOnSign(2): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.useGlowInkOnSign(2): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.useWaxOnSign(2): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.isTextLiteralOrEmpty(2): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.openEditScreen(5): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.openScreen(3): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.isOtherPlayerEditing(2): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.getDrops(2): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.getTexture(0): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.getCollisionShape(4): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
        // GravestoneBlock.isPathfindable(2): not translated, its class isn't translated (it extends Minecraft's `StandingSignBlock`).
    }

    /// `LongDecoBlock` (com/lion/graveyard/blocks/LongDecoBlock.java)
    pub mod long_deco_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // PART: not translated, it uses Minecraft's `EnumProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // LongDecoBlock.LongDecoBlock(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LongDecoBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LongDecoBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LongDecoBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LongDecoBlock.otherPartPos(2): not translated, it uses Minecraft's `BlockPos`.
        // LongDecoBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LongDecoBlock.getDrops(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LongDecoBlock.playerWillDestroy(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LongDecoBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LongDecoBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LongDecoBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // LongDecoBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `LoreBookBlock` (com/lion/graveyard/blocks/LoreBookBlock.java)
    pub mod lore_book_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // VARIANT: not translated, it uses Minecraft's `EnumProperty`.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // LoreBookBlock.LoreBookBlock(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.getDrops(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // LoreBookBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
    }

    /// `LoreBookBlock.Variant` (com/lion/graveyard/blocks/LoreBookBlock.java)
    pub mod lore_book_block_variant {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // LoreBookBlock.Variant.Variant(1): not translated, its class isn't translated (its constants carry values).
        // LoreBookBlock.Variant.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // LoreBookBlock.Variant.next(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `OminousBoneStaffFragment` (com/lion/graveyard/blocks/OminousBoneStaffFragment.java)
    pub mod ominous_bone_staff_fragment {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // OminousBoneStaffFragment.OminousBoneStaffFragment(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OminousBoneStaffFragment.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OminousBoneStaffFragment.appendItemTooltip(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OminousBoneStaffFragment.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OminousBoneStaffFragment.propagatesSkylightDown(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OminousBoneStaffFragment.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OminousBoneStaffFragment.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OminousBoneStaffFragment.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OminousBoneStaffFragment.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OminousBoneStaffFragment.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // OminousBoneStaffFragment.isCollisionShapeFullBlock(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `OssuaryBlock` (com/lion/graveyard/blocks/OssuaryBlock.java)
    pub mod ossuary_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        // TITLE: a game value, so it's in game_side.rs.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // OPEN: not translated, it uses Minecraft's `BooleanProperty`.
        // OssuaryBlock.appendItemTooltip(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // OssuaryBlock.OssuaryBlock(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // OssuaryBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // OssuaryBlock.getTicker(3): not translated, it's generic.
        // OssuaryBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // OssuaryBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // OssuaryBlock.getMenuProvider(3): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // OssuaryBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // OssuaryBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // OssuaryBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // OssuaryBlock.isPathfindable(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // OssuaryBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // OssuaryBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
    }

    /// `PillarBlock` (com/lion/graveyard/blocks/PillarBlock.java)
    pub mod pillar_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // PillarBlock.PillarBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PillarBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PillarBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PillarBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PillarBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PillarBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // PillarBlock.isPathfindable(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `SarcophagusBlock` (com/lion/graveyard/blocks/SarcophagusBlock.java)
    pub mod sarcophagus_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // OPEN: not translated, it uses Minecraft's `BooleanProperty`.
        // PLAYER_PLACED: not translated, it uses Minecraft's `BooleanProperty`.
        // IS_COFFIN: not translated, it uses Minecraft's `BooleanProperty`.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // DOUBLE_NORTH_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // DOUBLE_SOUTH_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // DOUBLE_WEST_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // DOUBLE_EAST_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // PART: not translated, it uses Minecraft's `EnumProperty`.
        // SarcophagusBlock.SarcophagusBlock(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.getDirectionTowardsOtherPart(2): not translated, it uses Minecraft's `Direction`.
        // SarcophagusBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.spawnGhost(5): not translated, it uses Minecraft's `BlockState`.
        // SarcophagusBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.getSeed(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.getSarcophagusPart(1): not translated, it uses Minecraft's `BlockType`.
        // SarcophagusBlock.playerWillDestroy(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.affectNeighborsAfterRemoval(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.tick(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.isPathfindable(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.getOppositePartDirection(1): not translated, it uses Minecraft's `Direction`.
        // SarcophagusBlock.getAnimationProgressRetriever(1): not translated, its body couldn't be read: an anonymous class.
        // SarcophagusBlock.getTicker(3): not translated, it's generic.
        // SarcophagusBlock.getLid(0): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SarcophagusBlock.getBase(0): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
    }

    /// `SmolderingDebrisBlock` (com/lion/graveyard/blocks/SmolderingDebrisBlock.java)
    pub mod smoldering_debris_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        // SmolderingDebrisBlock.SmolderingDebrisBlock(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SmolderingDebrisBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SmolderingDebrisBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // SmolderingDebrisBlock.getTicker(3): not translated, it's generic.
        // SmolderingDebrisBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
    }

    /// `SmolderingDoorBlock` (com/lion/graveyard/blocks/SmolderingDoorBlock.java)
    pub mod smoldering_door_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `DoorBlock`. Its static parts are below.
        // SmolderingDoorBlock.SmolderingDoorBlock(2): not translated, its class isn't translated (it extends Minecraft's `DoorBlock`).
        // SmolderingDoorBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `DoorBlock`).
        // SmolderingDoorBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `DoorBlock`).
        // SmolderingDoorBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `DoorBlock`).
    }

    /// `SmolderingFenceBlock` (com/lion/graveyard/blocks/SmolderingFenceBlock.java)
    pub mod smoldering_fence_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `FenceBlock`. Its static parts are below.
        // SmolderingFenceBlock.SmolderingFenceBlock(1): not translated, its class isn't translated (it extends Minecraft's `FenceBlock`).
        // SmolderingFenceBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `FenceBlock`).
        // SmolderingFenceBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `FenceBlock`).
        // SmolderingFenceBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `FenceBlock`).
    }

    /// `SmolderingFenceGateBlock` (com/lion/graveyard/blocks/SmolderingFenceGateBlock.java)
    pub mod smoldering_fence_gate_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `FenceGateBlock`. Its static parts are below.
        // SmolderingFenceGateBlock.SmolderingFenceGateBlock(2): not translated, its class isn't translated (it extends Minecraft's `FenceGateBlock`).
        // SmolderingFenceGateBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `FenceGateBlock`).
        // SmolderingFenceGateBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `FenceGateBlock`).
        // SmolderingFenceGateBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `FenceGateBlock`).
    }

    /// `SmolderingPillarBlock` (com/lion/graveyard/blocks/SmolderingPillarBlock.java)
    pub mod smoldering_pillar_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RotatedPillarBlock`. Its static parts are below.
        // SmolderingPillarBlock.SmolderingPillarBlock(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // SmolderingPillarBlock.SmolderingPillarBlock(2): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // SmolderingPillarBlock.strippedState(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // SmolderingPillarBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // SmolderingPillarBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // SmolderingPillarBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
    }

    /// `SmolderingSlabBlock` (com/lion/graveyard/blocks/SmolderingSlabBlock.java)
    pub mod smoldering_slab_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SlabBlock`. Its static parts are below.
        // SmolderingSlabBlock.SmolderingSlabBlock(1): not translated, its class isn't translated (it extends Minecraft's `SlabBlock`).
        // SmolderingSlabBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `SlabBlock`).
        // SmolderingSlabBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `SlabBlock`).
        // SmolderingSlabBlock.placeLiquid(4): not translated, its class isn't translated (it extends Minecraft's `SlabBlock`).
        // SmolderingSlabBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `SlabBlock`).
    }

    /// `SmolderingStairsBlock` (com/lion/graveyard/blocks/SmolderingStairsBlock.java)
    pub mod smoldering_stairs_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `StairBlock`. Its static parts are below.
        // SmolderingStairsBlock.SmolderingStairsBlock(2): not translated, its class isn't translated (it extends Minecraft's `StairBlock`).
        // SmolderingStairsBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `StairBlock`).
        // SmolderingStairsBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `StairBlock`).
        // SmolderingStairsBlock.placeLiquid(4): not translated, its class isn't translated (it extends Minecraft's `StairBlock`).
        // SmolderingStairsBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `StairBlock`).
    }

    /// `SmolderingTrapdoorBlock` (com/lion/graveyard/blocks/SmolderingTrapdoorBlock.java)
    pub mod smoldering_trapdoor_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `TrapDoorBlock`. Its static parts are below.
        // SmolderingTrapdoorBlock.SmolderingTrapdoorBlock(2): not translated, its class isn't translated (it extends Minecraft's `TrapDoorBlock`).
        // SmolderingTrapdoorBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `TrapDoorBlock`).
        // SmolderingTrapdoorBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `TrapDoorBlock`).
        // SmolderingTrapdoorBlock.placeLiquid(4): not translated, its class isn't translated (it extends Minecraft's `TrapDoorBlock`).
        // SmolderingTrapdoorBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `TrapDoorBlock`).
    }

    /// `SmolderingWood` (com/lion/graveyard/blocks/SmolderingWood.java)
    pub mod smoldering_wood {
        use super::*;
        // SMOLDERING: not translated, it uses Minecraft's `BooleanProperty`.
        pub const ASH_CONFIG_KEY: &str = "ashParticles";
        pub const LIGHT_LEVEL: i32 = 3;
        pub const ASH_REACH: i32 = 6;
        pub const ASH_PER_CALL: i32 = 3;
        pub const WHITE_ASH_ONE_IN: i32 = 8;
        #[derive(Clone, Debug)]
        pub struct SmolderingWood {
        }
        impl SmolderingWood {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SmolderingWood.light(1): not translated, it uses Minecraft's `BlockState`.
        // SmolderingWood.isFlooded(1): not translated, it uses Minecraft's `BlockState`.
        // SmolderingWood.use(6): not translated, it uses Minecraft's `InteractionResult`.
        // SmolderingWood.setLit(4): not translated, it uses Minecraft's `Level`.
        // SmolderingWood.extinguish(3): not translated, it uses Minecraft's `Level`.
        // SmolderingWood.displayTick(4): not translated, it uses Minecraft's `BlockState`.
        // SmolderingWood.ash(3): not translated, it uses Minecraft's `Level`.
    }

    /// `SmolderingWoodBlock` (com/lion/graveyard/blocks/SmolderingWoodBlock.java)
    pub mod smoldering_wood_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SmolderingWoodBlock.SmolderingWoodBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SmolderingWoodBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SmolderingWoodBlock.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // SmolderingWoodBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `TallDeco` (com/lion/graveyard/blocks/TallDeco.java)
    pub mod tall_deco {
        use super::*;
        // HALF: not translated, it uses Minecraft's `EnumProperty`.
        #[derive(Clone, Debug)]
        pub struct TallDeco {
        }
        impl TallDeco {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TallDeco.placementState(2): not translated, it uses Minecraft's `BlockState`.
        // TallDeco.placeUpperHalf(3): not translated, it uses Minecraft's `Level`.
        // TallDeco.canPlaceAt(3): not translated, it uses Minecraft's `BlockState`.
        // TallDeco.hangingPlacementState(2): not translated, it uses Minecraft's `BlockState`.
        // TallDeco.placeLowerHalf(3): not translated, it uses Minecraft's `Level`.
        // TallDeco.canHangAt(3): not translated, it uses Minecraft's `BlockState`.
        // TallDeco.onNeighborUpdate(5): not translated, it uses Minecraft's `BlockState`.
        // TallDeco.onBreak(4): not translated, it uses Minecraft's `Level`.
        // TallDeco.isLower(1): not translated, it uses Minecraft's `BlockState`.
    }

    /// `TallDecoBlock` (com/lion/graveyard/blocks/TallDecoBlock.java)
    pub mod tall_deco_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // TallDecoBlock.TallDecoBlock(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.TallDecoBlock(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.TallDecoBlock(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.playerWillDestroy(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.getDrops(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TallDecoBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `TGDeepslateBlock` (com/lion/graveyard/blocks/TGDeepslateBlock.java)
    pub mod tg_deepslate_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RotatedPillarBlock`. Its static parts are below.
        // TGDeepslateBlock.TGDeepslateBlock(2): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // TGDeepslateBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
    }

    /// `TGDoorBlock` (com/lion/graveyard/blocks/TGDoorBlock.java)
    pub mod tg_door_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `DoorBlock`. Its static parts are below.
        // TGDoorBlock.TGDoorBlock(1): not translated, its class isn't translated (it extends Minecraft's `DoorBlock`).
        // TGDoorBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `DoorBlock`).
    }

    /// `TGGrassBlock` (com/lion/graveyard/blocks/TGGrassBlock.java)
    pub mod tg_grass_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `GrassBlock`. Its static parts are below.
        // TGGrassBlock.TGGrassBlock(1): not translated, its class isn't translated (it extends Minecraft's `GrassBlock`).
        // TGGrassBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `GrassBlock`).
    }

    /// `TGMossBlock` (com/lion/graveyard/blocks/TGMossBlock.java)
    pub mod tg_moss_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BonemealableFeaturePlacerBlock`. Its static parts are below.
        pub const FOG_SHEET_RARITY: i32 = 2;
        // TGMossBlock.TGMossBlock(1): not translated, its class isn't translated (it extends Minecraft's `BonemealableFeaturePlacerBlock`).
        // TGMossBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `BonemealableFeaturePlacerBlock`).
        // TGMossBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `BonemealableFeaturePlacerBlock`).
    }

    /// `TGStairsBlock` (com/lion/graveyard/blocks/TGStairsBlock.java)
    pub mod tg_stairs_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `StairBlock`. Its static parts are below.
        // TGStairsBlock.TGStairsBlock(2): not translated, its class isn't translated (it extends Minecraft's `StairBlock`).
    }

    /// `TGStoneBlock` (com/lion/graveyard/blocks/TGStoneBlock.java)
    pub mod tg_stone_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // TGStoneBlock.TGStoneBlock(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TGStoneBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `TGTrapDoorBlock` (com/lion/graveyard/blocks/TGTrapDoorBlock.java)
    pub mod tg_trap_door_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `TrapDoorBlock`. Its static parts are below.
        // TGTrapDoorBlock.TGTrapDoorBlock(1): not translated, its class isn't translated (it extends Minecraft's `TrapDoorBlock`).
        // TGTrapDoorBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `TrapDoorBlock`).
    }

    /// `UrnBlock` (com/lion/graveyard/blocks/UrnBlock.java)
    pub mod urn_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // OPEN: not translated, it uses Minecraft's `BooleanProperty`.
        // LARGE_URN: not translated, it uses Minecraft's `VoxelShape`.
        // SMALL_URN: not translated, it uses Minecraft's `VoxelShape`.
        // UrnBlock.UrnBlock(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.propagatesSkylightDown(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.getCollisionShape(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.isCollisionShapeFullBlock(3): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.tick(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.isPathfindable(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.rotate(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.mirror(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // UrnBlock.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
    }

    /// `VariantBoneDisplayBlock` (com/lion/graveyard/blocks/VariantBoneDisplayBlock.java)
    pub mod variant_bone_display_block {
        use super::*;
        // The class itself isn't translated: it extends `BoneDisplayBlock`. Its static parts are below.
        // VARIANT: not translated, it uses Minecraft's `IntegerProperty`.
        // VariantBoneDisplayBlock.VariantBoneDisplayBlock(1): not translated, its class isn't translated (it extends `BoneDisplayBlock`).
        // VariantBoneDisplayBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BoneDisplayBlock`).
        // VariantBoneDisplayBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends `BoneDisplayBlock`).
        // VariantBoneDisplayBlock.useItemOn(7): not translated, its class isn't translated (it extends `BoneDisplayBlock`).
    }

    /// `VaseBlock` (com/lion/graveyard/blocks/VaseBlock.java)
    pub mod vase_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // VASES: not translated, it uses Minecraft's `IntegerProperty`.
        // WATERLOGGED: not translated, it uses Minecraft's `BooleanProperty`.
        // VASE_SHAPE_ONE: not translated, it uses Minecraft's `VoxelShape`.
        // VASE_SHAPE_TWO: not translated, it uses Minecraft's `VoxelShape`.
        // VASE_SHAPE_THREE: not translated, it uses Minecraft's `VoxelShape`.
        // VASE_SHAPE_FOUR: not translated, it uses Minecraft's `VoxelShape`.
        // VaseBlock.VaseBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VaseBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VaseBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VaseBlock.getFluidState(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VaseBlock.canBeReplaced(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VaseBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // VaseBlock.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `Chronicle` (com/lion/graveyard/chronicle/Chronicle.java)
    pub mod chronicle {
        use super::*;
        // SIGNED_BOOKS: not translated, it calls `Map` (java.util).entry.
        pub static UNSIGNED_BOOKS: LazyLock<JMap<String, String>> = LazyLock::new(|| JMap::from_iter([("Day 112:".to_string(), "day_112".to_string()), ("Day 132:".to_string(), "day_132".to_string()), ("Day 140:".to_string(), "day_140".to_string()), ("It is done.".to_string(), "the_last_book".to_string())]));
        pub static ACT_ONE: LazyLock<Vec<String>> = LazyLock::new(|| vec!["on_bells".to_string(), "the_shroud".to_string(), "the_gravedigger".to_string(), "the_red_moon".to_string(), "fog".to_string(), "the_quiet".to_string(), "plague_rain".to_string(), "on_the_scarecrow".to_string(), "letter_unsent_1".to_string(), "letter_unsent_2".to_string(), "letter_unsent_3".to_string(), "roberts_diary_last".to_string(), "notes_64".to_string(), "the_mycelium_1".to_string(), "the_mycelium_2".to_string(), "the_tree".to_string(), "the_column".to_string()]);
        pub static ACT_TWO: LazyLock<Vec<String>> = LazyLock::new(|| vec!["the_arrival".to_string(), "the_first_omen".to_string(), "the_spreading_1".to_string(), "the_spreading_2".to_string(), "the_spreading_3".to_string(), "day_112".to_string(), "day_115".to_string(), "day_119".to_string(), "day_124".to_string(), "the_bell_ringer".to_string(), "day_127".to_string(), "day_132".to_string(), "tome_of_rituals".to_string(), "day_136".to_string(), "day_140".to_string(), "roberts_diary".to_string(), "the_consequence".to_string(), "notes_63".to_string(), "the_last_book".to_string()]);
        pub static PAGE_ID: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("[a-z0-9_]{1,64}"));
        #[derive(Clone, Debug)]
        pub struct Chronicle {
        }
        impl Chronicle {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn is_valid_id(page: Option<String>) -> bool {
            return page.is_some() && ((JMatcher::new((&(*chronicle::PAGE_ID)), &(page.as_deref().expect("null")))).matches());
        }
        pub fn all_pages() -> Vec<String> {
            let mut pages: Vec<String> = (*chronicle::ACT_ONE).to_vec();
            pages.extend((*chronicle::ACT_TWO).iter().cloned().collect::<Vec<_>>());
            return pages.to_vec();
        }
        // Chronicle.find(2): not translated, it uses Minecraft's `ServerPlayer`.
        // Chronicle.syncAll(1): not translated, it uses Minecraft's `ServerPlayer`.
        // Chronicle.pageOf(1): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `ChronicleOpener` (com/lion/graveyard/chronicle/ChronicleOpener.java)
    pub mod chronicle_opener {
        use super::*;
        // opener: not translated, a function kept in a variable or field (only parameters can be).
        #[derive(Clone, Debug)]
        pub struct ChronicleOpener {
        }
        impl ChronicleOpener {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ChronicleOpener.install(1): not translated, it uses ChronicleOpener.opener, which isn't translated (a function kept in a variable or field (only parameters can be)).
        // ChronicleOpener.open(1): not translated, it uses ChronicleOpener.opener, which isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `ChronicleState` (com/lion/graveyard/chronicle/ChronicleState.java)
    pub mod chronicle_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SavedData`. Its static parts are below.
        pub const LEGACY_FILE: &str = "graveyard_chronicle.dat";
        pub const MAX_PAGES_PER_PLAYER: i32 = 512;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // TYPE: not translated, it uses Minecraft's `SavedDataType`.
        // ChronicleState.get(1): not translated, it uses ChronicleState, which isn't translated (it extends Minecraft's `SavedData`).
        // ChronicleState.importLegacy(1): not translated, it uses ChronicleState, which isn't translated (it extends Minecraft's `SavedData`).
        // ChronicleState.found(1): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // ChronicleState.add(2): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // ChronicleState.remove(2): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // ChronicleState.clear(1): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // ChronicleState.toNbt(0): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // ChronicleState.fromNbt(1): not translated, it uses ChronicleState, which isn't translated (it extends Minecraft's `SavedData`).
    }

    /// `ChronicleButton` (com/lion/graveyard/chronicle/client/ChronicleButton.java)
    pub mod chronicle_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Button`. Its static parts are below.
        // ChronicleButton.ChronicleButton(1): not translated, its class isn't translated (it extends Minecraft's `Button`).
        // ChronicleButton.addTo(2): not translated, it uses Minecraft's `Screen`.
        // ChronicleButton.extractContents(4): not translated, its class isn't translated (it extends Minecraft's `Button`).
        // ChronicleButton.followInventory(0): not translated, its class isn't translated (it extends Minecraft's `Button`).
        // ChronicleButton.hasStatusEffects(0): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `ChronicleChapters` (com/lion/graveyard/chronicle/client/ChronicleChapters.java)
    pub mod chronicle_chapters {
        use super::*;
        pub const DIRECTORY: &str = "chronicle";
        #[derive(Clone, Debug)]
        pub struct ChronicleChapters {
        }
        impl ChronicleChapters {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ChronicleChapters.load(0): not translated, it uses Minecraft's `Resource`.
    }

    /// `ChronicleChapters.Chapter` (com/lion/graveyard/chronicle/client/ChronicleChapters.java)
    pub mod chronicle_chapters_chapter {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Chapter {
            pub id: String,
            pub act: i32,
            pub order: i32,
            pub source: String,
            pub pages: Vec<String>,
        }
        impl Chapter {
            pub fn new(id: &str, act: i32, order: i32, source: &str, pages: &[String]) -> Self {
                let mut s = Self { id: String::new(), act: act, order: order, source: String::new(), pages: Vec::new() };
                s.id = id.to_string();
                s.act = act;
                s.order = order;
                s.source = source.to_string();
                s.pages = pages.to_vec();
                s
            }
            pub fn title_key(&self) -> String {
                return format!("chronicle.graveyard.page.{}.title", self.id);
            }
            pub fn id(&self) -> String {
                return self.id.to_string();
            }
            pub fn act(&self) -> i32 {
                return self.act;
            }
            pub fn order(&self) -> i32 {
                return self.order;
            }
            pub fn source(&self) -> String {
                return self.source.to_string();
            }
            pub fn pages(&self) -> Vec<String> {
                return self.pages.to_vec();
            }
        }
    }

    /// `ChronicleScreen` (com/lion/graveyard/chronicle/client/ChronicleScreen.java)
    pub mod chronicle_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub static BUTTON_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/gui/chronicle_button.png"));
        pub static BOOK_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "textures/gui/book.png"));
        pub const BOOK_WIDTH: i32 = 192;
        pub const BOOK_HEIGHT: i32 = 192;
        pub const TEXT_WIDTH: i32 = 114;
        pub const TEXT_TOP: i32 = 38;
        pub const LINES_PER_PAGE: i32 = 13;
        pub const LIST_WIDTH: i32 = 120;
        pub const ROW_HEIGHT: i32 = 11;
        // ChronicleScreen.ChronicleScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.ChronicleScreen(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.select(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.paginate(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.turnPage(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.updateButtons(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.extractBackground(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.extractList(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.progressY(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.extractBook(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.mouseScrolled(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ChronicleScreen.isPauseScreen(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `ChronicleToast` (com/lion/graveyard/chronicle/client/ChronicleToast.java)
    pub mod chronicle_toast {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Toast`. Its static parts are below.
        pub static BACKGROUND: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "toast/advancement"));
        pub const DURATION: i64 = 5000i64;
        // ChronicleToast.ChronicleToast(1): not translated, its class isn't translated (it implements Minecraft's `Toast`).
        // ChronicleToast.getWantedVisibility(0): not translated, its class isn't translated (it implements Minecraft's `Toast`).
        // ChronicleToast.update(2): not translated, its class isn't translated (it implements Minecraft's `Toast`).
        // ChronicleToast.extractRenderState(3): not translated, its class isn't translated (it implements Minecraft's `Toast`).
    }

    /// `ClientChronicle` (com/lion/graveyard/chronicle/client/ClientChronicle.java)
    pub mod client_chronicle {
        use super::*;
        pub static FOUND: LazyLock<JCell<JSet<String>>> = LazyLock::new(|| JCell::new(JSet::new()));
        #[derive(Clone, Debug)]
        pub struct ClientChronicle {
        }
        impl ClientChronicle {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientChronicle.receive(2): not translated, it uses Minecraft's `Minecraft`.
        pub fn found() -> JSet<String> {
            return client_chronicle::FOUND.with(|o| o.clone());
        }
        pub fn is_found(page: &str) -> bool {
            return { let a_0 = page.to_string(); client_chronicle::FOUND.with(|o| o.contains(&a_0)) };
        }
    }

    /// `FragmentSpotHints` (com/lion/graveyard/client/FragmentSpotHints.java)
    pub mod fragment_spot_hints {
        use super::*;
        pub const RANGE: i32 = 24;
        pub const RANGE_UP_DOWN: i32 = 12;
        pub const SCAN_INTERVAL: i32 = 20;
        pub const PARTICLE_INTERVAL: i32 = 4;
        pub const ROW_DISTANCE: i32 = 7;
        pub const END_OFFSET: i32 = 3;
        // ALTARS: not translated, it uses Minecraft's `BlockPos`.
        pub static TICKS: JCell<i32> = JCell::new(0i32);
        #[derive(Clone, Debug)]
        pub struct FragmentSpotHints {
        }
        impl FragmentSpotHints {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FragmentSpotHints.clientTick(1): not translated, it uses Minecraft's `Minecraft`.
        // FragmentSpotHints.holdsFragment(1): not translated, it uses Minecraft's `LocalPlayer`.
        // FragmentSpotHints.isFragment(1): not translated, it uses Minecraft's `ItemStack`.
        // FragmentSpotHints.scanAltars(2): not translated, it uses Minecraft's `ClientLevel`.
        // FragmentSpotHints.spawnParticles(2): not translated, it uses Minecraft's `ClientLevel`.
        // FragmentSpotHints.isUsableRow(2): not translated, it uses Minecraft's `ClientLevel`.
    }

    /// `GraveyardDataTickets` (com/lion/graveyard/client/GraveyardDataTickets.java)
    pub mod graveyard_data_tickets {
        use super::*;
        // GHOUL_VARIANT: not translated, it uses `DataTicket` (com.geckolib.constant.dataticket).
        // GHOULING_VARIANT: not translated, it uses `DataTicket` (com.geckolib.constant.dataticket).
        // GHOULING_HAS_COFFIN: not translated, it uses `DataTicket` (com.geckolib.constant.dataticket).
        // GHOULING_OFFHAND: not translated, it uses `DataTicket` (com.geckolib.constant.dataticket).
        // ILLAGER_TEXTURE: not translated, it uses `DataTicket` (com.geckolib.constant.dataticket).
        // ILLAGER_LEFT_CLAW: not translated, it uses `DataTicket` (com.geckolib.constant.dataticket).
        // ILLAGER_RIGHT_CLAW: not translated, it uses `DataTicket` (com.geckolib.constant.dataticket).
        // CORVID_SPECIES: not translated, it uses `DataTicket` (com.geckolib.constant.dataticket).
        // CORVID_LOOK_WEIGHT: not translated, it uses `DataTicket` (com.geckolib.constant.dataticket).
        // LICH_PHASE: not translated, it uses `DataTicket` (com.geckolib.constant.dataticket).
        #[derive(Clone, Debug)]
        pub struct GraveyardDataTickets {
        }
        impl GraveyardDataTickets {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `VialChargedProperty` (com/lion/graveyard/client/VialChargedProperty.java)
    pub mod vial_charged_property {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RangeSelectItemModelProperty`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // VialChargedProperty.VialChargedProperty(0): not translated, its class isn't translated (it implements Minecraft's `RangeSelectItemModelProperty`).
        // VialChargedProperty.get(4): not translated, its class isn't translated (it implements Minecraft's `RangeSelectItemModelProperty`).
        // VialChargedProperty.type(0): not translated, its class isn't translated (it implements Minecraft's `RangeSelectItemModelProperty`).
    }

    /// `ConfigFiles` (com/lion/graveyard/config/ConfigFiles.java)
    pub mod config_files {
        use super::*;
        pub const FILE_NAME: &str = "graveyard.json5";
        pub const LEGACY_FILE_NAME: &str = "the-graveyard-3.0-config.json5";
        pub const MAX_LISTED: i32 = 10;
        #[derive(Clone, Debug)]
        pub struct ConfigFiles {
        }
        impl ConfigFiles {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ConfigFiles.load(2): not translated, it uses ConfigLog, which isn't translated (it's an interface).
        // ConfigFiles.loadCurrent(4): not translated, it uses ConfigLog, which isn't translated (it's an interface).
        // ConfigFiles.migrate(4): not translated, it uses ConfigLog, which isn't translated (it's an interface).
        // ConfigFiles.write(4): not translated, it uses ConfigLog, which isn't translated (it's an interface).
        // ConfigFiles.deleteTemporary(2): not translated, it uses ConfigLog, which isn't translated (it's an interface).
        pub fn delete_quietly(file: &std::path::Path) {
            let result_0 = (|| -> std::io::Result<Option<()>> {
                std::fs::remove_file(&file).is_ok();
                Ok(None)
            })();
            match result_0 {
                Ok(_) => {}
                Err(_) => {
                }
            }
        }
        // ConfigFiles.backup(3): not translated, it uses ConfigLog, which isn't translated (it's an interface).
        pub fn r#move(temporary: &std::path::Path, target: &std::path::Path) -> std::io::Result<()> {
            {
                std::fs::rename(&temporary, &target)?;
            }
            Ok(())
        }
        // ConfigFiles.reportUnreadable(3): not translated, it uses ConfigLog, which isn't translated (it's an interface).
        // ConfigFiles.reportProblems(4): not translated, it uses ConfigLog, which isn't translated (it's an interface).
        // ConfigFiles.listed(1): not translated, List.subList.
        pub fn normalize_line_ends(text: &str) -> String {
            return text.replace("\r\n", "\n");
        }
    }

    /// `ConfigLog` (com/lion/graveyard/config/ConfigLog.java)
    pub mod config_log {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ConfigLog.info(1): not translated, it has no body.
        // ConfigLog.warn(1): not translated, it has no body.
        // ConfigLog.error(1): not translated, it has no body.
    }

    /// `ConfigReader` (com/lion/graveyard/config/ConfigReader.java)
    pub mod config_reader {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConfigReader {
        }
        impl ConfigReader {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ConfigReader.read(1): not translated, unknown name `exception`.
        // ConfigReader.parseObject(2): not translated, it uses `JsonReader` (com.google.gson.stream).
        // ConfigReader.hasContentAfter(1): not translated, it uses `JsonReader` (com.google.gson.stream).
        // ConfigReader.readSection(3): not translated, it uses Section, which isn't translated (it extends `Node`).
        // ConfigReader.hasChild(2): not translated, it uses Section, which isn't translated (it extends `Node`).
        // ConfigReader.readNested(3): not translated, it uses Section, which isn't translated (it extends `Node`).
    }

    /// `ConfigReader.Result` (com/lion/graveyard/config/ConfigReader.java)
    pub mod config_reader_result {
        use super::*;
        // The class itself isn't translated: it uses ConfigValues, which isn't translated (it uses `Object`). Its static parts are below.
        // ConfigReader.Result.Result(2): not translated, its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)).
        // ConfigReader.Result.values(0): not translated, its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)).
        // ConfigReader.Result.failed(0): not translated, its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)).
        // ConfigReader.Result.failure(0): not translated, its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)).
        // ConfigReader.Result.reset(0): not translated, its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)).
        // ConfigReader.Result.dropped(0): not translated, its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)).
        // ConfigReader.Result.hasTrailingContent(0): not translated, its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)).
        // ConfigReader.Result.hasProblems(0): not translated, its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)).
    }

    /// `ConfigRules` (com/lion/graveyard/config/ConfigRules.java)
    pub mod config_rules {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConfigRules {
        }
        impl ConfigRules {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ConfigRules.apply(2): not translated, it uses ConfigValues, which isn't translated (it uses `Object`).
        // ConfigRules.raise(4): not translated, it uses `Object`.
    }

    /// `ConfigSchema` (com/lion/graveyard/config/ConfigSchema.java)
    pub mod config_schema {
        use super::*;
        // The class itself isn't translated: it uses Section, which isn't translated (it extends `Node`). Its static parts are below.
        pub static STRUCTURE_IDS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["altar".to_string(), "crypt".to_string(), "dead_tree".to_string(), "giant_mushroom".to_string(), "haunted_house".to_string(), "large_graveyard".to_string(), "lich_prison".to_string(), "medium_graveyard".to_string(), "memorial_tree".to_string(), "mushroom_grave".to_string(), "overrun_village".to_string(), "overrun_desert_village".to_string(), "overrun_plains_village".to_string(), "overrun_savanna_village".to_string(), "overrun_snowy_village".to_string(), "overrun_taiga_village".to_string(), "ruins".to_string(), "small_desert_grave".to_string(), "small_desert_graveyard".to_string(), "small_grave".to_string(), "small_graveyard".to_string(), "small_mountain_grave".to_string(), "small_savanna_grave".to_string()]);
        pub const NAMELESS_HANGED: &str = "nameless_hanged";
        pub const MIN_MULTIPLIER: f64 = 0.05f64;
        pub static MOBS: LazyLock<Vec<config_schema_mob_defaults::MobDefaults>> = LazyLock::new(|| vec![config_schema_mob_defaults::MobDefaults::new("acolyte", true, 0, 0, 0, false), config_schema_mob_defaults::MobDefaults::new("corrupted_evoker", true, 0, 0, 0, true), config_schema_mob_defaults::MobDefaults::new("corrupted_pillager", true, 0, 0, 0, true), config_schema_mob_defaults::MobDefaults::new("corrupted_vindicator", true, 0, 0, 0, true), config_schema_mob_defaults::MobDefaults::new("crow", true, 10, 4, 9, false), config_schema_mob_defaults::MobDefaults::new("ghoul", true, 10, 1, 4, true), config_schema_mob_defaults::MobDefaults::new("magpie", true, 8, 2, 3, false), config_schema_mob_defaults::MobDefaults::new("nameless_hanged", true, 0, 0, 0, false), config_schema_mob_defaults::MobDefaults::new("nightmare", true, 10, 1, 1, false), config_schema_mob_defaults::MobDefaults::new("raven", true, 8, 2, 4, false), config_schema_mob_defaults::MobDefaults::new("reaper", false, 5, 2, 3, true), config_schema_mob_defaults::MobDefaults::new("revenant", true, 25, 5, 8, true), config_schema_mob_defaults::MobDefaults::new("skeleton_creeper", true, 25, 1, 4, true), config_schema_mob_defaults::MobDefaults::new("tormented_villager", true, 0, 1, 1, false), config_schema_mob_defaults::MobDefaults::new("wraith", true, 0, 0, 0, true)]);
        pub static FULL_MOB_IDS: LazyLock<Vec<String>> = LazyLock::new(|| (*config_schema::MOBS).iter().cloned().map(|p_a: config_schema_mob_defaults::MobDefaults| p_a.id()).filter(|id: &String| !j_eq(id, "nameless_hanged")).collect::<Vec<_>>());
        pub static ALL_MOB_IDS: LazyLock<Vec<String>> = LazyLock::new(|| (*config_schema::MOBS).iter().cloned().map(|p_a: config_schema_mob_defaults::MobDefaults| p_a.id()).collect::<Vec<_>>());
        // INSTANCE: not translated, it uses ConfigSchema, which isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.ConfigSchema(0): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.root(0): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.leaves(0): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.leaf(1): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.leafPaths(0): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        pub fn is_full_mob(id: &str) -> bool {
            return (*config_schema::FULL_MOB_IDS).iter().any(|e| e == &id);
        }
        // ConfigSchema.collect(1): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.buildGeneral(0): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.buildStructures(0): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.buildMobs(0): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.buildHordes(0): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.buildChampion(0): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.buildChronicle(0): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // ConfigSchema.buildBehaviour(0): not translated, its class isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        pub fn sentence(id: &str) -> String {
            let text: String = id.replace('_', &' '.to_string());
            return format!("{}{}.", j_char_at(&text, 0).to_ascii_uppercase(), j_substring(&text, 1, None));
        }
    }

    /// `ConfigSchema.Coerced` (com/lion/graveyard/config/ConfigSchema.java)
    pub mod config_schema_coerced {
        use super::*;
        // The class itself isn't translated: it uses `Object`. Its static parts are below.
        // ConfigSchema.Coerced.Coerced(2): not translated, its class isn't translated (it uses `Object`).
        // ConfigSchema.Coerced.value(0): not translated, its class isn't translated (it uses `Object`).
        // ConfigSchema.Coerced.exact(0): not translated, its class isn't translated (it uses `Object`).
    }

    /// `ConfigSchema.Kind` (com/lion/graveyard/config/ConfigSchema.java)
    pub mod config_schema_kind {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Kind {
            Bool,
            Int,
            Double,
            Float,
            Multiplier,
            StringList,
        }
        impl Kind {
            pub const VALUES: [Kind; 6] = [Kind::Bool, Kind::Int, Kind::Double, Kind::Float, Kind::Multiplier, Kind::StringList, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Kind::Bool => "BOOL",
                    Kind::Int => "INT",
                    Kind::Double => "DOUBLE",
                    Kind::Float => "FLOAT",
                    Kind::Multiplier => "MULTIPLIER",
                    Kind::StringList => "STRING_LIST",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "BOOL" => Kind::Bool,
                    "INT" => Kind::Int,
                    "DOUBLE" => Kind::Double,
                    "FLOAT" => Kind::Float,
                    "MULTIPLIER" => Kind::Multiplier,
                    "STRING_LIST" => Kind::StringList,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ConfigSchema.Leaf` (com/lion/graveyard/config/ConfigSchema.java)
    pub mod config_schema_leaf {
        use super::*;
        // The class itself isn't translated: it extends `Node`. Its static parts are below.
        // ConfigSchema.Leaf.Leaf(9): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Leaf.kind(0): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Leaf.defaultValue(0): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Leaf.coerce(1): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Leaf.coerceBool(1): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Leaf.coerceNumber(2): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Leaf.coerceFloored(1): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Leaf.bounded(1): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Leaf.box(1): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Leaf.coerceMultiplier(1): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Leaf.coerceStrings(1): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Leaf.invalid(0): not translated, its class isn't translated (it extends `Node`).
    }

    /// `ConfigSchema.MobDefaults` (com/lion/graveyard/config/ConfigSchema.java)
    pub mod config_schema_mob_defaults {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct MobDefaults {
            pub id: String,
            pub enabled: bool,
            pub weight: i32,
            pub min_group: i32,
            pub max_group: i32,
            pub burns: bool,
        }
        impl MobDefaults {
            pub fn new(id: &str, enabled: bool, weight: i32, min_group: i32, max_group: i32, burns: bool) -> Self {
                let mut s = Self { id: String::new(), enabled: enabled, weight: weight, min_group: min_group, max_group: max_group, burns: burns };
                s.id = id.to_string();
                s.enabled = enabled;
                s.weight = weight;
                s.min_group = min_group;
                s.max_group = max_group;
                s.burns = burns;
                s
            }
            pub fn id(&self) -> String {
                return self.id.to_string();
            }
            pub fn enabled(&self) -> bool {
                return self.enabled;
            }
            pub fn weight(&self) -> i32 {
                return self.weight;
            }
            pub fn min_group(&self) -> i32 {
                return self.min_group;
            }
            pub fn max_group(&self) -> i32 {
                return self.max_group;
            }
            pub fn burns(&self) -> bool {
                return self.burns;
            }
        }
    }

    /// `ConfigSchema.Node` (com/lion/graveyard/config/ConfigSchema.java)
    pub mod config_schema_node {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Node {
            pub name: String,
            pub path: String,
            pub comment: String,
        }
        impl Node {
            pub fn new(name: &str, path: &str, comment: &str) -> Self {
                let mut s = Self { name: String::new(), path: String::new(), comment: String::new() };
                s.name = name.to_string();
                s.path = path.to_string();
                s.comment = comment.to_string();
                s
            }
            pub fn name(&self) -> String {
                return self.name.to_string();
            }
            pub fn path(&self) -> String {
                return self.path.to_string();
            }
            pub fn comment(&self) -> String {
                return self.comment.to_string();
            }
        }
    }

    /// `ConfigSchema.Section` (com/lion/graveyard/config/ConfigSchema.java)
    pub mod config_schema_section {
        use super::*;
        // The class itself isn't translated: it extends `Node`. Its static parts are below.
        // ConfigSchema.Section.Section(4): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.children(0): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.isTable(0): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.section(2): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.table(1): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.bool(3): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.integer(5): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.real(5): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.realAtLeast(3): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.floating(5): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.floatAbove(3): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.multiplier(2): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.strings(3): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.add(1): not translated, its class isn't translated (it extends `Node`).
        // ConfigSchema.Section.childPath(1): not translated, its class isn't translated (it extends `Node`).
    }

    /// `ConfigValues` (com/lion/graveyard/config/ConfigValues.java)
    pub mod config_values {
        use super::*;
        // The class itself isn't translated: it uses `Object`. Its static parts are below.
        // ConfigValues.defaults(0): not translated, it uses ConfigValues, which isn't translated (it uses `Object`).
        // ConfigValues.copy(0): not translated, its class isn't translated (it uses `Object`).
        // ConfigValues.has(1): not translated, its class isn't translated (it uses `Object`).
        // ConfigValues.bool(1): not translated, its class isn't translated (it uses `Object`).
        // ConfigValues.integer(1): not translated, its class isn't translated (it uses `Object`).
        // ConfigValues.real(1): not translated, its class isn't translated (it uses `Object`).
        // ConfigValues.floating(1): not translated, its class isn't translated (it uses `Object`).
        // ConfigValues.strings(1): not translated, its class isn't translated (it uses `Object`).
        // ConfigValues.get(1): not translated, its class isn't translated (it uses `Object`).
        // ConfigValues.set(2): not translated, its class isn't translated (it uses `Object`).
        // ConfigValues.asMap(0): not translated, its class isn't translated (it uses `Object`).
        // ConfigValues.equals(1): not translated, its class isn't translated (it uses `Object`).
        // ConfigValues.hashCode(0): not translated, its class isn't translated (it uses `Object`).
    }

    /// `ConfigWriter` (com/lion/graveyard/config/ConfigWriter.java)
    pub mod config_writer {
        use super::*;
        pub const HEADER: &str = "// The Graveyard config. Comments are rewritten at start. Values are kept.";
        pub const INDENT: &str = "  ";
        #[derive(Clone, Debug)]
        pub struct ConfigWriter {
        }
        impl ConfigWriter {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ConfigWriter.write(1): not translated, it uses ConfigValues, which isn't translated (it uses `Object`).
        // ConfigWriter.writeChildren(6): not translated, it uses Section, which isn't translated (it extends `Node`).
        // ConfigWriter.writeSection(8): not translated, it uses Section, which isn't translated (it extends `Node`).
        // ConfigWriter.writeLeaf(6): not translated, it uses Leaf, which isn't translated (it extends `Node`).
        pub fn line(out: &mut String, depth: i32, text: &str) {
            out.push_str(&("  ".repeat(depth as usize)));
            out.push_str(&text);
            out.push_str(&(format!("{}", '\n')));
        }
        // ConfigWriter.format(2): not translated, it uses Leaf, which isn't translated (it extends `Node`).
        // ConfigWriter.decimal(1): not translated, it calls `BigDecimal` (java.math).valueOf.
        // ConfigWriter.list(1): not translated, it uses `Object`.
    }

    /// `CorruptedChampionEntry` (com/lion/graveyard/config/entries/CorruptedChampionEntry.java)
    pub mod corrupted_champion_entry {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CorruptedChampionEntry {
            pub health_in_casting_phase: f32,
            pub health_in_hunting_phase: f32,
            pub damage_casting_phase: f64,
            pub damage_hunting_phase_addition: f64,
            pub armor: f64,
            pub armor_toughness: f64,
            pub speed_in_hunt_phase: f64,
            pub duration_hunting_phase: i32,
            pub duration_falling_corpse_spell: i32,
            pub duration_healing_spell: i32,
            pub duration_levitation_spell: i32,
            pub max_amount_skulls_in_shoot_skull_spell: i32,
            pub max_summoned_mobs: i32,
            pub max_group_size_summoned_mobs: i32,
            pub ghoul_spawn_timer_in_fight: i32,
            pub is_blood_collectable_entity: Vec<String>,
            pub is_boss_summonable_item: Vec<String>,
            pub summoning_needs_staff_fragments: bool,
            pub is_multiphase_fight: bool,
            pub is_invulnerable_during_spells: bool,
            pub cooldown_corpse_spell: i32,
            pub cooldown_teleport_player_and_heal: i32,
            pub cooldown_levitation_spell: i32,
            pub player_teleport_y_offset: i32,
            pub boss_bar_darken_sky: Option<bool>,
            pub boss_bar_thicken_fog: Option<bool>,
            pub boss_music_volume: Option<f32>,
            pub spawn_invulnerable_ticks: Option<i32>,
            pub phase_invulnerable_ticks: Option<i32>,
            pub cooldown_hunt: Option<i32>,
            pub cooldown_charge: Option<i32>,
            pub cooldown_pounce: Option<i32>,
            pub heal_threshold_health: Option<f32>,
            pub skull_explosion_radius: Option<f32>,
            pub skulls_destroy_blocks: Option<bool>,
        }
        impl CorruptedChampionEntry {
            pub fn boss_bar_darken_sky(&self) -> bool {
                return self.boss_bar_darken_sky.is_none() || (self.boss_bar_darken_sky.expect("null"));
            }
            pub fn boss_bar_thicken_fog(&self) -> bool {
                return self.boss_bar_thicken_fog.is_none() || (self.boss_bar_thicken_fog.expect("null"));
            }
            // CorruptedChampionEntry.bossMusicVolume(0): not translated, arithmetic on float (or null) and float.
            // CorruptedChampionEntry.spawnInvulnerableTicks(0): not translated, arithmetic on int and int (or null).
            // CorruptedChampionEntry.phaseInvulnerableTicks(0): not translated, arithmetic on int and int (or null).
            // CorruptedChampionEntry.cooldownHunt(0): not translated, arithmetic on int and int (or null).
            // CorruptedChampionEntry.cooldownCharge(0): not translated, arithmetic on int and int (or null).
            // CorruptedChampionEntry.cooldownPounce(0): not translated, arithmetic on int and int (or null).
            pub fn heal_threshold_health(&self) -> f32 {
                return if self.heal_threshold_health.is_some() && self.heal_threshold_health.is_finite() { self.heal_threshold_health.expect("null") } else { 300.0f32 };
            }
            // CorruptedChampionEntry.skullExplosionRadius(0): not translated, arithmetic on float (or null) and float.
            pub fn skulls_destroy_blocks(&self) -> bool {
                return self.skulls_destroy_blocks.is_some() && (self.skulls_destroy_blocks.expect("null"));
            }
            pub fn new(health_in_casting_phase: f32, health_in_hunting_phase: f32, damage_casting_phase: f64, damage_hunting_phase_addition: f64, armor: f64, armor_toughness: f64, speed_in_hunt_phase: f64, duration_hunting_phase: i32, duration_falling_corpse_spell: i32, duration_healing_spell: i32, duration_levitation_spell: i32, max_amount_skulls_in_shoot_skull_spell: i32, max_summoned_mobs: i32, max_group_size_summoned_mobs: i32, ghoul_spawn_timer_in_fight: i32, is_blood_collectable_entity: &[String], is_boss_summonable_item: &[String], is_multiphase_fight: bool, is_invulnerable_during_spells: bool, summoning_needs_staff_fragments: bool, cooldown_corpse_spell: i32, cooldown_teleport_player_and_heal: i32, cooldown_levitation_spell: i32, player_teleport_y_offset: i32) -> Self {
                let mut s = Self { health_in_casting_phase: health_in_casting_phase, health_in_hunting_phase: health_in_hunting_phase, damage_casting_phase: damage_casting_phase, damage_hunting_phase_addition: damage_hunting_phase_addition, armor: armor, armor_toughness: armor_toughness, speed_in_hunt_phase: speed_in_hunt_phase, duration_hunting_phase: duration_hunting_phase, duration_falling_corpse_spell: duration_falling_corpse_spell, duration_healing_spell: duration_healing_spell, duration_levitation_spell: duration_levitation_spell, max_amount_skulls_in_shoot_skull_spell: max_amount_skulls_in_shoot_skull_spell, max_summoned_mobs: max_summoned_mobs, max_group_size_summoned_mobs: max_group_size_summoned_mobs, ghoul_spawn_timer_in_fight: ghoul_spawn_timer_in_fight, is_blood_collectable_entity: Vec::new(), is_boss_summonable_item: Vec::new(), summoning_needs_staff_fragments: summoning_needs_staff_fragments, is_multiphase_fight: is_multiphase_fight, is_invulnerable_during_spells: is_invulnerable_during_spells, cooldown_corpse_spell: cooldown_corpse_spell, cooldown_teleport_player_and_heal: cooldown_teleport_player_and_heal, cooldown_levitation_spell: cooldown_levitation_spell, player_teleport_y_offset: player_teleport_y_offset, boss_bar_darken_sky: None, boss_bar_thicken_fog: None, boss_music_volume: None, spawn_invulnerable_ticks: None, phase_invulnerable_ticks: None, cooldown_hunt: None, cooldown_charge: None, cooldown_pounce: None, heal_threshold_health: None, skull_explosion_radius: None, skulls_destroy_blocks: None };
                s.health_in_casting_phase = health_in_casting_phase;
                s.health_in_hunting_phase = health_in_hunting_phase;
                s.damage_casting_phase = damage_casting_phase;
                s.damage_hunting_phase_addition = damage_hunting_phase_addition;
                s.armor = armor;
                s.armor_toughness = armor_toughness;
                s.speed_in_hunt_phase = speed_in_hunt_phase;
                s.duration_hunting_phase = duration_hunting_phase;
                s.duration_falling_corpse_spell = duration_falling_corpse_spell;
                s.duration_healing_spell = duration_healing_spell;
                s.duration_levitation_spell = duration_levitation_spell;
                s.max_amount_skulls_in_shoot_skull_spell = max_amount_skulls_in_shoot_skull_spell;
                s.max_summoned_mobs = max_summoned_mobs;
                s.max_group_size_summoned_mobs = max_group_size_summoned_mobs;
                s.ghoul_spawn_timer_in_fight = ghoul_spawn_timer_in_fight;
                s.is_blood_collectable_entity = is_blood_collectable_entity.to_vec();
                s.is_boss_summonable_item = is_boss_summonable_item.to_vec();
                s.is_multiphase_fight = is_multiphase_fight;
                s.is_invulnerable_during_spells = is_invulnerable_during_spells;
                s.summoning_needs_staff_fragments = summoning_needs_staff_fragments;
                s.cooldown_corpse_spell = cooldown_corpse_spell;
                s.cooldown_teleport_player_and_heal = cooldown_teleport_player_and_heal;
                s.cooldown_levitation_spell = cooldown_levitation_spell;
                s.player_teleport_y_offset = player_teleport_y_offset;
                s
            }
        }
        pub fn of(health_in_casting_phase: f32, health_in_hunting_phase: f32, damage_casting_phase: f64, damage_hunting_phase: f64, armor: f64, armor_toughness: f64, speed_in_hunt_phase: f64, duration_hunting_phase: i32, duration_falling_corpse_spell: i32, duration_healing_spell: i32, duration_levitation_spell: i32, amount_shoot_skulls_spell: i32, max_summoned_mobs: i32, chance_summoned_mobs: i32, ghoul_spawn_timer_in_fight: i32, is_blood_collectable_entity: &[String], is_boss_summonable_item: &[String], is_multiphase_fight: bool, is_invulnerable_during_spells: bool, summoning_needs_staff_fragments: bool, cooldown_corpse_spell: i32, cooldown_teleport_player_and_heal: i32, cooldown_levitation_spell: i32, player_teleport_y_offset: i32) -> corrupted_champion_entry::CorruptedChampionEntry {
            return corrupted_champion_entry::CorruptedChampionEntry::new(health_in_casting_phase, health_in_hunting_phase, damage_casting_phase, damage_hunting_phase, armor, armor_toughness, speed_in_hunt_phase, duration_hunting_phase, duration_falling_corpse_spell, duration_healing_spell, duration_levitation_spell, amount_shoot_skulls_spell, max_summoned_mobs, chance_summoned_mobs, ghoul_spawn_timer_in_fight, is_blood_collectable_entity, is_boss_summonable_item, is_multiphase_fight, is_invulnerable_during_spells, summoning_needs_staff_fragments, cooldown_corpse_spell, cooldown_teleport_player_and_heal, cooldown_levitation_spell, player_teleport_y_offset);
        }
    }

    /// `HordeConfigEntry` (com/lion/graveyard/config/entries/HordeConfigEntry.java)
    pub mod horde_config_entry {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct HordeConfigEntry {
            pub enabled: bool,
            pub mob_spawn_attempts: i32,
            pub ticks_until_next_spawn: i32,
            pub additional_randomized_ticks: i32,
            pub chance_per_check: Option<f64>,
            pub min_distance: Option<i32>,
            pub max_distance: Option<i32>,
            pub night_only: Option<bool>,
        }
        impl HordeConfigEntry {
            pub fn new_3(mob_spawn_attempts: i32, ticks_until_next_spawn: i32, additional_randomized_ticks: i32) -> Self {
                let mut s = Self::new_8(true, mob_spawn_attempts, ticks_until_next_spawn, additional_randomized_ticks, 0.2f64, 24, 48, true);
                s
            }
            pub fn new_8(enabled: bool, mob_spawn_attempts: i32, ticks_until_next_spawn: i32, additional_randomized_ticks: i32, chance_per_check: f64, min_distance: i32, max_distance: i32, night_only: bool) -> Self {
                let mut s = Self { enabled: enabled, mob_spawn_attempts: mob_spawn_attempts, ticks_until_next_spawn: ticks_until_next_spawn, additional_randomized_ticks: additional_randomized_ticks, chance_per_check: Some(chance_per_check), min_distance: Some(min_distance), max_distance: Some(max_distance), night_only: Some(night_only) };
                s.enabled = enabled;
                s.mob_spawn_attempts = mob_spawn_attempts;
                s.ticks_until_next_spawn = ticks_until_next_spawn;
                s.additional_randomized_ticks = additional_randomized_ticks;
                s.chance_per_check = Some(chance_per_check);
                s.min_distance = Some(min_distance);
                s.max_distance = Some(max_distance);
                s.night_only = Some(night_only);
                s
            }
            // HordeConfigEntry.chance(0): not translated, arithmetic on double and double (or null).
            // HordeConfigEntry.nearDistance(0): not translated, arithmetic on int and int (or null).
            // HordeConfigEntry.farDistance(0): not translated, it calls HordeConfigEntry.nearDistance, which isn't translated.
            pub fn only_at_night(&self) -> bool {
                return self.night_only.is_none() || (self.night_only.expect("null"));
            }
        }
        pub fn of_8(enabled: bool, mob_spawn_attempts: i32, ticks_until_next_spawn: i32, additional_randomized_ticks: i32, chance_per_check: f64, min_distance: i32, max_distance: i32, night_only: bool) -> horde_config_entry::HordeConfigEntry {
            return horde_config_entry::HordeConfigEntry::new_8(enabled, mob_spawn_attempts, ticks_until_next_spawn, additional_randomized_ticks, chance_per_check, min_distance, max_distance, night_only);
        }
        pub fn of_3(mob_spawn_attempts: i32, ticks_until_next_spawn: i32, additional_randomized_ticks: i32) -> horde_config_entry::HordeConfigEntry {
            return horde_config_entry::HordeConfigEntry::new_3(mob_spawn_attempts, ticks_until_next_spawn, additional_randomized_ticks);
        }
    }

    /// `MobConfigEntry` (com/lion/graveyard/config/entries/MobConfigEntry.java)
    pub mod mob_config_entry {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MobConfigEntry {
            pub enabled: bool,
            pub weight: i32,
            pub min_group: i32,
            pub max_group: i32,
            pub can_burn_in_sunlight: bool,
            pub can_be_withered: bool,
            pub health_multiplier: f64,
            pub damage_multiplier: f64,
            pub speed_multiplier: f64,
        }
        impl MobConfigEntry {
            pub fn new(enabled: bool, weight: i32, min_group: i32, max_group: i32, can_burn_in_sunlight: bool, can_be_withered: bool, health_multiplier: f64, damage_multiplier: f64, speed_multiplier: f64) -> Self {
                let mut s = Self { enabled: enabled, weight: weight, min_group: min_group, max_group: max_group, can_burn_in_sunlight: can_burn_in_sunlight, can_be_withered: can_be_withered, health_multiplier: health_multiplier, damage_multiplier: damage_multiplier, speed_multiplier: speed_multiplier };
                s.enabled = enabled;
                s.weight = weight;
                s.min_group = min_group;
                s.max_group = max_group;
                s.can_burn_in_sunlight = can_burn_in_sunlight;
                s.can_be_withered = can_be_withered;
                s.health_multiplier = health_multiplier;
                s.damage_multiplier = damage_multiplier;
                s.speed_multiplier = speed_multiplier;
                s
            }
            // MobConfigEntry.health(0): not translated, it calls MobConfigEntry.multiplier, which isn't translated.
            // MobConfigEntry.damage(0): not translated, it calls MobConfigEntry.multiplier, which isn't translated.
            // MobConfigEntry.speed(0): not translated, it calls MobConfigEntry.multiplier, which isn't translated.
        }
        // MobConfigEntry.multiplier(1): not translated, arithmetic on double and double (or null).
        pub fn of_9(enabled: bool, weight: i32, min_group: i32, max_group: i32, can_burn_in_sunlight: bool, can_be_withered: bool, health_multiplier: f64, damage_multiplier: f64, speed_multiplier: f64) -> mob_config_entry::MobConfigEntry {
            return mob_config_entry::MobConfigEntry::new(enabled, weight, min_group, max_group, can_burn_in_sunlight, can_be_withered, health_multiplier, damage_multiplier, speed_multiplier);
        }
        pub fn of_3(enabled: bool, can_burn_in_sunlight: bool, can_be_withered: bool) -> mob_config_entry::MobConfigEntry {
            return mob_config_entry::of_6(enabled, 0, 0, 0, can_burn_in_sunlight, can_be_withered);
        }
        pub fn of_6(enabled: bool, weight: i32, min_group: i32, max_group: i32, can_burn_in_sunlight: bool, can_be_withered: bool) -> mob_config_entry::MobConfigEntry {
            return mob_config_entry::MobConfigEntry::new(enabled, weight, min_group, max_group, can_burn_in_sunlight, can_be_withered, 1.0f64, 1.0f64, 1.0f64);
        }
    }

    /// `ParticleConfigEntry` (com/lion/graveyard/config/entries/ParticleConfigEntry.java)
    pub mod particle_config_entry {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ParticleConfigEntry {
            pub can_generate: bool,
            pub spawn_chance: i32,
        }
        impl ParticleConfigEntry {
            pub fn new_1(spawn_chance: i32) -> Self {
                let mut s = Self::new_2(true, spawn_chance);
                s
            }
            pub fn new_2(can_generate: bool, spawn_chance: i32) -> Self {
                let mut s = Self { can_generate: can_generate, spawn_chance: spawn_chance };
                s.can_generate = can_generate;
                s.spawn_chance = spawn_chance;
                s
            }
        }
        pub fn of_2(can_generate: bool, spawn_chance: i32) -> particle_config_entry::ParticleConfigEntry {
            return particle_config_entry::ParticleConfigEntry::new_2(can_generate, spawn_chance);
        }
        pub fn of_1(spawn_chance: i32) -> particle_config_entry::ParticleConfigEntry {
            return particle_config_entry::ParticleConfigEntry::new_1(spawn_chance);
        }
    }

    /// `StructureConfigEntry` (com/lion/graveyard/config/entries/StructureConfigEntry.java)
    pub mod structure_config_entry {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StructureConfigEntry {
            pub enabled: bool,
            pub chance: Option<f64>,
            pub min_y: Option<i32>,
            pub max_y: Option<i32>,
        }
        impl StructureConfigEntry {
            pub fn new(enabled: bool, chance: f64, min_y: Option<i32>, max_y: Option<i32>) -> Self {
                let mut s = Self { enabled: enabled, chance: Some(chance), min_y: min_y.clone(), max_y: max_y.clone() };
                s.enabled = enabled;
                s.chance = Some(chance);
                s.min_y = min_y.clone();
                s.max_y = max_y.clone();
                s
            }
            // StructureConfigEntry.spawnChance(0): not translated, arithmetic on double and double (or null).
            pub fn lowest_y(&self, fallback: i32) -> i32 {
                return if self.min_y.is_none() { fallback } else { self.min_y.expect("null") };
            }
            pub fn highest_y(&self, fallback: i32) -> i32 {
                return if self.max_y.is_none() { fallback } else { self.max_y.expect("null") };
            }
        }
        pub fn of_4(enabled: bool, chance: f64, min_y: i32, max_y: i32) -> structure_config_entry::StructureConfigEntry {
            return structure_config_entry::StructureConfigEntry::new(enabled, chance, Some(min_y), Some(max_y));
        }
        pub fn of_0() -> structure_config_entry::StructureConfigEntry {
            return structure_config_entry::StructureConfigEntry::new(true, 1.0f64, None, None);
        }
    }

    /// `GraveyardConfig` (com/lion/graveyard/config/GraveyardConfig.java)
    pub mod graveyard_config {
        use super::*;
        pub const CHAMPION_ID: &str = "corrupted_champion";
        pub const HORDE_ID: &str = "horde_spawn";
        pub const FOG_ID: &str = "graveyard_fog_particle";
        pub static BOOLEAN_KEYS: LazyLock<Vec<Vec<String>>> = LazyLock::new(|| vec![vec!["urnHasDoubleInventory".to_string(), "behaviour.urnDoubleInventory".to_string()], vec!["disableWitherSkeletonSpawner".to_string(), "behaviour.replaceWitherSkeletonSpawner".to_string()], vec!["enableBossMusic".to_string(), "general.bossMusic".to_string()], vec!["chroniclePagesInLoot".to_string(), "chronicle.pagesInLoot".to_string()], vec!["ashParticles".to_string(), "general.ashParticles".to_string()], vec!["structuresOnly".to_string(), "general.structuresOnly".to_string()], vec!["chronicleInventoryButton".to_string(), "chronicle.inventoryButton".to_string()], vec!["chronicleToasts".to_string(), "chronicle.toasts".to_string()], vec!["corvidsStealItems".to_string(), "behaviour.magpiesStealItems".to_string()], vec!["ghoulingCollectsItems".to_string(), "behaviour.ghoulingCollectsItems".to_string()], vec!["crowsEatCrops".to_string(), "behaviour.crowsEatCrops".to_string()], vec!["smolderingBlocksInVillages".to_string(), "behaviour.smolderingBlocksInVillages".to_string()], vec!["skeletonCreeperDestroysBlocks".to_string(), "behaviour.skeletonCreeperDestroysBlocks".to_string()], vec!["wraithGriefing".to_string(), "behaviour.wraithGriefing".to_string()], vec!["darkIronOpensByHand".to_string(), "behaviour.darkIronOpensByHand".to_string()]]);
        pub static NUMBER_KEYS: LazyLock<Vec<Vec<String>>> = LazyLock::new(|| vec![vec!["skeletonCreeperExplosionRadius".to_string(), "behaviour.skeletonCreeperExplosionRadius".to_string()], vec!["corvidTheftChance".to_string(), "behaviour.magpieTheftChance".to_string()], vec!["soundVolume".to_string(), "general.soundVolume".to_string()]]);
        #[derive(Clone, Debug)]
        pub struct GraveyardConfig {
            pub structure_config_entries: JMap<String, structure_config_entry::StructureConfigEntry>,
            pub mob_config_entries: JMap<String, mob_config_entry::MobConfigEntry>,
            pub horde_config_entries: JMap<String, horde_config_entry::HordeConfigEntry>,
            pub corrupted_champion_config_entries: JMap<String, corrupted_champion_entry::CorruptedChampionEntry>,
            pub boolean_entries: JMap<String, bool>,
            pub number_entries: JMap<String, f64>,
            pub particle_entries: JMap<String, particle_config_entry::ParticleConfigEntry>,
            pub mob_switches: JMap<String, bool>,
        }
        impl GraveyardConfig {
            // GraveyardConfig.GraveyardConfig(1): not translated, it uses ConfigValues, which isn't translated (it uses `Object`).
            pub fn enabled(&self, id: &JIdentifier) -> bool {
                let path: String = id.path.clone();
                return if !(self.structure_switch(&path)) { false } else { (!graveyard_config::is_overrun_variant(&path)) || (self.structure_switch("overrun_village")) };
            }
            pub fn structure_switch(&self, path: &str) -> bool {
                let entry: Option<structure_config_entry::StructureConfigEntry> = (self.structure_config_entries.get(&*path)).cloned();
                return entry.is_none() || ((entry.as_ref().expect("null")).enabled);
            }
            // GraveyardConfig.structureChance(1): not translated, it calls StructureConfigEntry.spawnChance, which isn't translated.
            pub fn flag(&self, key: &str, fallback: bool) -> bool {
                let value: Option<bool> = self.boolean_entries.get(&*key).cloned();
                return if value.is_none() { fallback } else { value.expect("null") };
            }
            pub fn structures_only(&self) -> bool {
                return self.flag("structuresOnly", false);
            }
            pub fn mob_may_appear(&self, mob_name: &str) -> bool {
                let value: Option<bool> = self.mob_switches.get(&*mob_name).cloned();
                return value.is_none() || (value.expect("null"));
            }
            pub fn number(&self, key: &str, fallback: f64) -> f64 {
                let value: Option<f64> = self.number_entries.get(&*key).cloned();
                return if value.is_none() { fallback } else { value.expect("null") };
            }
            pub fn mob_entry(&self, mob_name: &str) -> mob_config_entry::MobConfigEntry {
                let entry: Option<mob_config_entry::MobConfigEntry> = (self.mob_config_entries.get(&*mob_name)).cloned();
                return if entry.is_none() { mob_config_entry::of_6(true, 0, 0, 0, false, false) } else { (entry.as_ref().expect("null")).clone() };
            }
            pub fn fog_spawn(&self, id: &JIdentifier) -> bool {
                return (self.get_particle(id)).can_generate;
            }
            pub fn get_structure(&self, id: &JIdentifier) -> structure_config_entry::StructureConfigEntry {
                let entry: Option<structure_config_entry::StructureConfigEntry> = (self.structure_config_entries.get(&*id.path.clone())).cloned();
                return if entry.is_none() { structure_config_entry::of_0() } else { (entry.as_ref().expect("null")).clone() };
            }
            pub fn get_particle(&self, id: &JIdentifier) -> particle_config_entry::ParticleConfigEntry {
                let entry: Option<particle_config_entry::ParticleConfigEntry> = (self.particle_entries.get(&*id.path.clone())).cloned();
                return if entry.is_none() { particle_config_entry::of_1(50) } else { (entry.as_ref().expect("null")).clone() };
            }
            pub fn get_horde(&self, id: &JIdentifier) -> horde_config_entry::HordeConfigEntry {
                let entry: Option<horde_config_entry::HordeConfigEntry> = (self.horde_config_entries.get(&*id.path.clone())).cloned();
                return if entry.is_none() { horde_config_entry::of_3(40, 24000, 1200) } else { (entry.as_ref().expect("null")).clone() };
            }
        }
        pub fn is_overrun_variant(path: &str) -> bool {
            return ((path.starts_with("overrun_")) && (path.ends_with("_village"))) && (!j_eq(&path, "overrun_village"));
        }
        // GraveyardConfig.structure(2): not translated, it uses ConfigValues, which isn't translated (it uses `Object`).
        // GraveyardConfig.mob(2): not translated, it uses ConfigValues, which isn't translated (it uses `Object`).
        // GraveyardConfig.horde(1): not translated, it uses ConfigValues, which isn't translated (it uses `Object`).
        // GraveyardConfig.champion(1): not translated, it uses ConfigValues, which isn't translated (it uses `Object`).
    }

    /// `LegacyFile` (com/lion/graveyard/config/LegacyFile.java)
    pub mod legacy_file {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LegacyFile {
            pub structure_config_entries: Option<JMap<String, legacy_file_structure::Structure>>,
            pub particle_config_entries: Option<JMap<String, legacy_file_particle::Particle>>,
            pub mob_config_entries: Option<JMap<String, legacy_file_mob::Mob>>,
            pub horde_config_entries: Option<JMap<String, legacy_file_horde::Horde>>,
            pub corrupted_champion_config_entries: Option<JMap<String, legacy_file_champion::Champion>>,
            pub boolean_entries: Option<JMap<String, bool>>,
            pub mob_spawn_enabled: Option<JMap<String, bool>>,
            pub number_entries: Option<JMap<String, f64>>,
        }
        impl LegacyFile {
            pub fn new() -> Self {
                let mut s = Self { structure_config_entries: None, particle_config_entries: None, mob_config_entries: None, horde_config_entries: None, corrupted_champion_config_entries: None, boolean_entries: None, mob_spawn_enabled: None, number_entries: None };
                s
            }
            /// Gson.toJson: the instance fields by name.
            pub fn to_jjson(&self) -> JJson {
                let mut j = JJson::object();
                j.add("structureConfigEntries", match &self.structure_config_entries { Some(v) => { let mut o = JJson::object(); for (k, e) in v.iter() { o.add(&k.to_string(), e.to_jjson()); } o }, None => JJson::Null });
                j.add("particleConfigEntries", match &self.particle_config_entries { Some(v) => { let mut o = JJson::object(); for (k, e) in v.iter() { o.add(&k.to_string(), e.to_jjson()); } o }, None => JJson::Null });
                j.add("mobConfigEntries", match &self.mob_config_entries { Some(v) => { let mut o = JJson::object(); for (k, e) in v.iter() { o.add(&k.to_string(), e.to_jjson()); } o }, None => JJson::Null });
                j.add("hordeConfigEntries", match &self.horde_config_entries { Some(v) => { let mut o = JJson::object(); for (k, e) in v.iter() { o.add(&k.to_string(), e.to_jjson()); } o }, None => JJson::Null });
                j.add("corruptedChampionConfigEntries", match &self.corrupted_champion_config_entries { Some(v) => { let mut o = JJson::object(); for (k, e) in v.iter() { o.add(&k.to_string(), e.to_jjson()); } o }, None => JJson::Null });
                j.add("booleanEntries", match &self.boolean_entries { Some(v) => { let mut o = JJson::object(); for (k, e) in v.iter() { o.add(&k.to_string(), JJson::Bool(*e)); } o }, None => JJson::Null });
                j.add("mobSpawnEnabled", match &self.mob_spawn_enabled { Some(v) => { let mut o = JJson::object(); for (k, e) in v.iter() { o.add(&k.to_string(), JJson::Bool(*e)); } o }, None => JJson::Null });
                j.add("numberEntries", match &self.number_entries { Some(v) => { let mut o = JJson::object(); for (k, e) in v.iter() { o.add(&k.to_string(), JJson::double(*e)); } o }, None => JJson::Null });
                j
            }
            /// Gson.fromJson: a new one (its constructor's values), with the fields the JSON has.
            pub fn from_jjson(j: &JJson) -> Option<Self> {
                if j.is_json_null() {
                    return None;
                }
                let mut s = Self::new();
                if let Some(v) = j.get("structureConfigEntries") && !v.is_json_null() {
                    s.structure_config_entries = if v.is_json_null() { None } else { Some(JMap::from_iter(v.entries().map(|(k, e)| (k.clone(), legacy_file_structure::Structure::from_jjson(e).expect("null"))))) };
                }
                if let Some(v) = j.get("particleConfigEntries") && !v.is_json_null() {
                    s.particle_config_entries = if v.is_json_null() { None } else { Some(JMap::from_iter(v.entries().map(|(k, e)| (k.clone(), legacy_file_particle::Particle::from_jjson(e).expect("null"))))) };
                }
                if let Some(v) = j.get("mobConfigEntries") && !v.is_json_null() {
                    s.mob_config_entries = if v.is_json_null() { None } else { Some(JMap::from_iter(v.entries().map(|(k, e)| (k.clone(), legacy_file_mob::Mob::from_jjson(e).expect("null"))))) };
                }
                if let Some(v) = j.get("hordeConfigEntries") && !v.is_json_null() {
                    s.horde_config_entries = if v.is_json_null() { None } else { Some(JMap::from_iter(v.entries().map(|(k, e)| (k.clone(), legacy_file_horde::Horde::from_jjson(e).expect("null"))))) };
                }
                if let Some(v) = j.get("corruptedChampionConfigEntries") && !v.is_json_null() {
                    s.corrupted_champion_config_entries = if v.is_json_null() { None } else { Some(JMap::from_iter(v.entries().map(|(k, e)| (k.clone(), legacy_file_champion::Champion::from_jjson(e).expect("null"))))) };
                }
                if let Some(v) = j.get("booleanEntries") && !v.is_json_null() {
                    s.boolean_entries = if v.is_json_null() { None } else { Some(JMap::from_iter(v.entries().map(|(k, e)| (k.clone(), e.as_bool())))) };
                }
                if let Some(v) = j.get("mobSpawnEnabled") && !v.is_json_null() {
                    s.mob_spawn_enabled = if v.is_json_null() { None } else { Some(JMap::from_iter(v.entries().map(|(k, e)| (k.clone(), e.as_bool())))) };
                }
                if let Some(v) = j.get("numberEntries") && !v.is_json_null() {
                    s.number_entries = if v.is_json_null() { None } else { Some(JMap::from_iter(v.entries().map(|(k, e)| (k.clone(), e.as_double())))) };
                }
                Some(s)
            }
        }
    }

    /// `LegacyFile.Champion` (com/lion/graveyard/config/LegacyFile.java)
    pub mod legacy_file_champion {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Champion {
            pub health_in_casting_phase: f32,
            pub health_in_hunting_phase: f32,
            pub damage_casting_phase: f64,
            pub damage_hunting_phase_addition: f64,
            pub armor: f64,
            pub armor_toughness: f64,
            pub speed_in_hunt_phase: f64,
            pub duration_hunting_phase: i32,
            pub duration_falling_corpse_spell: i32,
            pub duration_healing_spell: i32,
            pub duration_levitation_spell: i32,
            pub max_amount_skulls_in_shoot_skull_spell: i32,
            pub max_summoned_mobs: i32,
            pub max_group_size_summoned_mobs: i32,
            pub ghoul_spawn_timer_in_fight: i32,
            pub is_blood_collectable_entity: Option<Vec<String>>,
            pub is_boss_summonable_item: Option<Vec<String>>,
            pub summoning_needs_staff_fragments: bool,
            pub is_multiphase_fight: bool,
            pub is_invulnerable_during_spells: bool,
            pub cooldown_corpse_spell: i32,
            pub cooldown_teleport_player_and_heal: i32,
            pub cooldown_levitation_spell: i32,
            pub player_teleport_y_offset: i32,
            pub boss_bar_darken_sky: Option<bool>,
            pub boss_bar_thicken_fog: Option<bool>,
            pub boss_music_volume: Option<f32>,
            pub spawn_invulnerable_ticks: Option<i32>,
            pub phase_invulnerable_ticks: Option<i32>,
            pub cooldown_hunt: Option<i32>,
            pub cooldown_charge: Option<i32>,
            pub cooldown_pounce: Option<i32>,
            pub heal_threshold_health: Option<f32>,
            pub skull_explosion_radius: Option<f32>,
            pub skulls_destroy_blocks: Option<bool>,
        }
        impl Champion {
            pub fn new() -> Self {
                let mut s = Self { health_in_casting_phase: 0.0f32, health_in_hunting_phase: 0.0f32, damage_casting_phase: 0.0f64, damage_hunting_phase_addition: 0.0f64, armor: 0.0f64, armor_toughness: 0.0f64, speed_in_hunt_phase: 0.0f64, duration_hunting_phase: 0i32, duration_falling_corpse_spell: 0i32, duration_healing_spell: 0i32, duration_levitation_spell: 0i32, max_amount_skulls_in_shoot_skull_spell: 0i32, max_summoned_mobs: 0i32, max_group_size_summoned_mobs: 0i32, ghoul_spawn_timer_in_fight: 0i32, is_blood_collectable_entity: None, is_boss_summonable_item: None, summoning_needs_staff_fragments: false, is_multiphase_fight: false, is_invulnerable_during_spells: false, cooldown_corpse_spell: 0i32, cooldown_teleport_player_and_heal: 0i32, cooldown_levitation_spell: 0i32, player_teleport_y_offset: 0i32, boss_bar_darken_sky: None, boss_bar_thicken_fog: None, boss_music_volume: None, spawn_invulnerable_ticks: None, phase_invulnerable_ticks: None, cooldown_hunt: None, cooldown_charge: None, cooldown_pounce: None, heal_threshold_health: None, skull_explosion_radius: None, skulls_destroy_blocks: None };
                s
            }
            /// Gson.toJson: the instance fields by name.
            pub fn to_jjson(&self) -> JJson {
                let mut j = JJson::object();
                j.add("healthInCastingPhase", JJson::float(self.health_in_casting_phase));
                j.add("healthInHuntingPhase", JJson::float(self.health_in_hunting_phase));
                j.add("damageCastingPhase", JJson::double(self.damage_casting_phase));
                j.add("damageHuntingPhaseAddition", JJson::double(self.damage_hunting_phase_addition));
                j.add("armor", JJson::double(self.armor));
                j.add("armorToughness", JJson::double(self.armor_toughness));
                j.add("speedInHuntPhase", JJson::double(self.speed_in_hunt_phase));
                j.add("durationHuntingPhase", JJson::int(self.duration_hunting_phase as i64));
                j.add("durationFallingCorpseSpell", JJson::int(self.duration_falling_corpse_spell as i64));
                j.add("durationHealingSpell", JJson::int(self.duration_healing_spell as i64));
                j.add("durationLevitationSpell", JJson::int(self.duration_levitation_spell as i64));
                j.add("maxAmountSkullsInShootSkullSpell", JJson::int(self.max_amount_skulls_in_shoot_skull_spell as i64));
                j.add("maxSummonedMobs", JJson::int(self.max_summoned_mobs as i64));
                j.add("maxGroupSizeSummonedMobs", JJson::int(self.max_group_size_summoned_mobs as i64));
                j.add("ghoulSpawnTimerInFight", JJson::int(self.ghoul_spawn_timer_in_fight as i64));
                j.add("isBloodCollectableEntity", match &self.is_blood_collectable_entity { Some(v) => JJson::Arr(v.iter().map(|e| JJson::str(&e)).collect()), None => JJson::Null });
                j.add("isBossSummonableItem", match &self.is_boss_summonable_item { Some(v) => JJson::Arr(v.iter().map(|e| JJson::str(&e)).collect()), None => JJson::Null });
                j.add("summoningNeedsStaffFragments", JJson::Bool(self.summoning_needs_staff_fragments));
                j.add("isMultiphaseFight", JJson::Bool(self.is_multiphase_fight));
                j.add("isInvulnerableDuringSpells", JJson::Bool(self.is_invulnerable_during_spells));
                j.add("cooldownCorpseSpell", JJson::int(self.cooldown_corpse_spell as i64));
                j.add("cooldownTeleportPlayerAndHeal", JJson::int(self.cooldown_teleport_player_and_heal as i64));
                j.add("cooldownLevitationSpell", JJson::int(self.cooldown_levitation_spell as i64));
                j.add("playerTeleportYOffset", JJson::int(self.player_teleport_y_offset as i64));
                j.add("bossBarDarkenSky", match &self.boss_bar_darken_sky { Some(v) => JJson::Bool(*v), None => JJson::Null });
                j.add("bossBarThickenFog", match &self.boss_bar_thicken_fog { Some(v) => JJson::Bool(*v), None => JJson::Null });
                j.add("bossMusicVolume", match &self.boss_music_volume { Some(v) => JJson::float(*v), None => JJson::Null });
                j.add("spawnInvulnerableTicks", match &self.spawn_invulnerable_ticks { Some(v) => JJson::int(*v as i64), None => JJson::Null });
                j.add("phaseInvulnerableTicks", match &self.phase_invulnerable_ticks { Some(v) => JJson::int(*v as i64), None => JJson::Null });
                j.add("cooldownHunt", match &self.cooldown_hunt { Some(v) => JJson::int(*v as i64), None => JJson::Null });
                j.add("cooldownCharge", match &self.cooldown_charge { Some(v) => JJson::int(*v as i64), None => JJson::Null });
                j.add("cooldownPounce", match &self.cooldown_pounce { Some(v) => JJson::int(*v as i64), None => JJson::Null });
                j.add("healThresholdHealth", match &self.heal_threshold_health { Some(v) => JJson::float(*v), None => JJson::Null });
                j.add("skullExplosionRadius", match &self.skull_explosion_radius { Some(v) => JJson::float(*v), None => JJson::Null });
                j.add("skullsDestroyBlocks", match &self.skulls_destroy_blocks { Some(v) => JJson::Bool(*v), None => JJson::Null });
                j
            }
            /// Gson.fromJson: a new one (its constructor's values), with the fields the JSON has.
            pub fn from_jjson(j: &JJson) -> Option<Self> {
                if j.is_json_null() {
                    return None;
                }
                let mut s = Self::new();
                if let Some(v) = j.get("healthInCastingPhase") && !v.is_json_null() {
                    s.health_in_casting_phase = v.as_float();
                }
                if let Some(v) = j.get("healthInHuntingPhase") && !v.is_json_null() {
                    s.health_in_hunting_phase = v.as_float();
                }
                if let Some(v) = j.get("damageCastingPhase") && !v.is_json_null() {
                    s.damage_casting_phase = v.as_double();
                }
                if let Some(v) = j.get("damageHuntingPhaseAddition") && !v.is_json_null() {
                    s.damage_hunting_phase_addition = v.as_double();
                }
                if let Some(v) = j.get("armor") && !v.is_json_null() {
                    s.armor = v.as_double();
                }
                if let Some(v) = j.get("armorToughness") && !v.is_json_null() {
                    s.armor_toughness = v.as_double();
                }
                if let Some(v) = j.get("speedInHuntPhase") && !v.is_json_null() {
                    s.speed_in_hunt_phase = v.as_double();
                }
                if let Some(v) = j.get("durationHuntingPhase") && !v.is_json_null() {
                    s.duration_hunting_phase = v.as_int() as i32;
                }
                if let Some(v) = j.get("durationFallingCorpseSpell") && !v.is_json_null() {
                    s.duration_falling_corpse_spell = v.as_int() as i32;
                }
                if let Some(v) = j.get("durationHealingSpell") && !v.is_json_null() {
                    s.duration_healing_spell = v.as_int() as i32;
                }
                if let Some(v) = j.get("durationLevitationSpell") && !v.is_json_null() {
                    s.duration_levitation_spell = v.as_int() as i32;
                }
                if let Some(v) = j.get("maxAmountSkullsInShootSkullSpell") && !v.is_json_null() {
                    s.max_amount_skulls_in_shoot_skull_spell = v.as_int() as i32;
                }
                if let Some(v) = j.get("maxSummonedMobs") && !v.is_json_null() {
                    s.max_summoned_mobs = v.as_int() as i32;
                }
                if let Some(v) = j.get("maxGroupSizeSummonedMobs") && !v.is_json_null() {
                    s.max_group_size_summoned_mobs = v.as_int() as i32;
                }
                if let Some(v) = j.get("ghoulSpawnTimerInFight") && !v.is_json_null() {
                    s.ghoul_spawn_timer_in_fight = v.as_int() as i32;
                }
                if let Some(v) = j.get("isBloodCollectableEntity") && !v.is_json_null() {
                    s.is_blood_collectable_entity = if v.is_json_null() { None } else { Some(v.elements().map(|e| e.as_string()).collect()) };
                }
                if let Some(v) = j.get("isBossSummonableItem") && !v.is_json_null() {
                    s.is_boss_summonable_item = if v.is_json_null() { None } else { Some(v.elements().map(|e| e.as_string()).collect()) };
                }
                if let Some(v) = j.get("summoningNeedsStaffFragments") && !v.is_json_null() {
                    s.summoning_needs_staff_fragments = v.as_bool();
                }
                if let Some(v) = j.get("isMultiphaseFight") && !v.is_json_null() {
                    s.is_multiphase_fight = v.as_bool();
                }
                if let Some(v) = j.get("isInvulnerableDuringSpells") && !v.is_json_null() {
                    s.is_invulnerable_during_spells = v.as_bool();
                }
                if let Some(v) = j.get("cooldownCorpseSpell") && !v.is_json_null() {
                    s.cooldown_corpse_spell = v.as_int() as i32;
                }
                if let Some(v) = j.get("cooldownTeleportPlayerAndHeal") && !v.is_json_null() {
                    s.cooldown_teleport_player_and_heal = v.as_int() as i32;
                }
                if let Some(v) = j.get("cooldownLevitationSpell") && !v.is_json_null() {
                    s.cooldown_levitation_spell = v.as_int() as i32;
                }
                if let Some(v) = j.get("playerTeleportYOffset") && !v.is_json_null() {
                    s.player_teleport_y_offset = v.as_int() as i32;
                }
                if let Some(v) = j.get("bossBarDarkenSky") && !v.is_json_null() {
                    s.boss_bar_darken_sky = if v.is_json_null() { None } else { Some(v.as_bool()) };
                }
                if let Some(v) = j.get("bossBarThickenFog") && !v.is_json_null() {
                    s.boss_bar_thicken_fog = if v.is_json_null() { None } else { Some(v.as_bool()) };
                }
                if let Some(v) = j.get("bossMusicVolume") && !v.is_json_null() {
                    s.boss_music_volume = if v.is_json_null() { None } else { Some(v.as_float()) };
                }
                if let Some(v) = j.get("spawnInvulnerableTicks") && !v.is_json_null() {
                    s.spawn_invulnerable_ticks = if v.is_json_null() { None } else { Some(v.as_int() as i32) };
                }
                if let Some(v) = j.get("phaseInvulnerableTicks") && !v.is_json_null() {
                    s.phase_invulnerable_ticks = if v.is_json_null() { None } else { Some(v.as_int() as i32) };
                }
                if let Some(v) = j.get("cooldownHunt") && !v.is_json_null() {
                    s.cooldown_hunt = if v.is_json_null() { None } else { Some(v.as_int() as i32) };
                }
                if let Some(v) = j.get("cooldownCharge") && !v.is_json_null() {
                    s.cooldown_charge = if v.is_json_null() { None } else { Some(v.as_int() as i32) };
                }
                if let Some(v) = j.get("cooldownPounce") && !v.is_json_null() {
                    s.cooldown_pounce = if v.is_json_null() { None } else { Some(v.as_int() as i32) };
                }
                if let Some(v) = j.get("healThresholdHealth") && !v.is_json_null() {
                    s.heal_threshold_health = if v.is_json_null() { None } else { Some(v.as_float()) };
                }
                if let Some(v) = j.get("skullExplosionRadius") && !v.is_json_null() {
                    s.skull_explosion_radius = if v.is_json_null() { None } else { Some(v.as_float()) };
                }
                if let Some(v) = j.get("skullsDestroyBlocks") && !v.is_json_null() {
                    s.skulls_destroy_blocks = if v.is_json_null() { None } else { Some(v.as_bool()) };
                }
                Some(s)
            }
        }
    }

    /// `LegacyFile.Horde` (com/lion/graveyard/config/LegacyFile.java)
    pub mod legacy_file_horde {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Horde {
            pub enabled: bool,
            pub mob_spawn_attempts: i32,
            pub ticks_until_next_spawn: i32,
            pub additional_randomized_ticks: i32,
            pub chance_per_check: Option<f64>,
            pub min_distance: Option<i32>,
            pub max_distance: Option<i32>,
            pub night_only: Option<bool>,
        }
        impl Horde {
            pub fn new() -> Self {
                let mut s = Self { enabled: false, mob_spawn_attempts: 0i32, ticks_until_next_spawn: 0i32, additional_randomized_ticks: 0i32, chance_per_check: None, min_distance: None, max_distance: None, night_only: None };
                s
            }
            /// Gson.toJson: the instance fields by name.
            pub fn to_jjson(&self) -> JJson {
                let mut j = JJson::object();
                j.add("enabled", JJson::Bool(self.enabled));
                j.add("mobSpawnAttempts", JJson::int(self.mob_spawn_attempts as i64));
                j.add("ticksUntilNextSpawn", JJson::int(self.ticks_until_next_spawn as i64));
                j.add("additionalRandomizedTicks", JJson::int(self.additional_randomized_ticks as i64));
                j.add("chancePerCheck", match &self.chance_per_check { Some(v) => JJson::double(*v), None => JJson::Null });
                j.add("minDistance", match &self.min_distance { Some(v) => JJson::int(*v as i64), None => JJson::Null });
                j.add("maxDistance", match &self.max_distance { Some(v) => JJson::int(*v as i64), None => JJson::Null });
                j.add("nightOnly", match &self.night_only { Some(v) => JJson::Bool(*v), None => JJson::Null });
                j
            }
            /// Gson.fromJson: a new one (its constructor's values), with the fields the JSON has.
            pub fn from_jjson(j: &JJson) -> Option<Self> {
                if j.is_json_null() {
                    return None;
                }
                let mut s = Self::new();
                if let Some(v) = j.get("enabled") && !v.is_json_null() {
                    s.enabled = v.as_bool();
                }
                if let Some(v) = j.get("mobSpawnAttempts") && !v.is_json_null() {
                    s.mob_spawn_attempts = v.as_int() as i32;
                }
                if let Some(v) = j.get("ticksUntilNextSpawn") && !v.is_json_null() {
                    s.ticks_until_next_spawn = v.as_int() as i32;
                }
                if let Some(v) = j.get("additionalRandomizedTicks") && !v.is_json_null() {
                    s.additional_randomized_ticks = v.as_int() as i32;
                }
                if let Some(v) = j.get("chancePerCheck") && !v.is_json_null() {
                    s.chance_per_check = if v.is_json_null() { None } else { Some(v.as_double()) };
                }
                if let Some(v) = j.get("minDistance") && !v.is_json_null() {
                    s.min_distance = if v.is_json_null() { None } else { Some(v.as_int() as i32) };
                }
                if let Some(v) = j.get("maxDistance") && !v.is_json_null() {
                    s.max_distance = if v.is_json_null() { None } else { Some(v.as_int() as i32) };
                }
                if let Some(v) = j.get("nightOnly") && !v.is_json_null() {
                    s.night_only = if v.is_json_null() { None } else { Some(v.as_bool()) };
                }
                Some(s)
            }
        }
    }

    /// `LegacyFile.Mob` (com/lion/graveyard/config/LegacyFile.java)
    pub mod legacy_file_mob {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Mob {
            pub enabled: bool,
            pub weight: i32,
            pub min_group: i32,
            pub max_group: i32,
            pub can_burn_in_sunlight: bool,
            pub can_be_withered: bool,
            pub health_multiplier: Option<f64>,
            pub damage_multiplier: Option<f64>,
            pub speed_multiplier: Option<f64>,
        }
        impl Mob {
            pub fn new() -> Self {
                let mut s = Self { enabled: false, weight: 0i32, min_group: 0i32, max_group: 0i32, can_burn_in_sunlight: false, can_be_withered: false, health_multiplier: None, damage_multiplier: None, speed_multiplier: None };
                s
            }
            /// Gson.toJson: the instance fields by name.
            pub fn to_jjson(&self) -> JJson {
                let mut j = JJson::object();
                j.add("enabled", JJson::Bool(self.enabled));
                j.add("weight", JJson::int(self.weight as i64));
                j.add("minGroup", JJson::int(self.min_group as i64));
                j.add("maxGroup", JJson::int(self.max_group as i64));
                j.add("canBurnInSunlight", JJson::Bool(self.can_burn_in_sunlight));
                j.add("canBeWithered", JJson::Bool(self.can_be_withered));
                j.add("healthMultiplier", match &self.health_multiplier { Some(v) => JJson::double(*v), None => JJson::Null });
                j.add("damageMultiplier", match &self.damage_multiplier { Some(v) => JJson::double(*v), None => JJson::Null });
                j.add("speedMultiplier", match &self.speed_multiplier { Some(v) => JJson::double(*v), None => JJson::Null });
                j
            }
            /// Gson.fromJson: a new one (its constructor's values), with the fields the JSON has.
            pub fn from_jjson(j: &JJson) -> Option<Self> {
                if j.is_json_null() {
                    return None;
                }
                let mut s = Self::new();
                if let Some(v) = j.get("enabled") && !v.is_json_null() {
                    s.enabled = v.as_bool();
                }
                if let Some(v) = j.get("weight") && !v.is_json_null() {
                    s.weight = v.as_int() as i32;
                }
                if let Some(v) = j.get("minGroup") && !v.is_json_null() {
                    s.min_group = v.as_int() as i32;
                }
                if let Some(v) = j.get("maxGroup") && !v.is_json_null() {
                    s.max_group = v.as_int() as i32;
                }
                if let Some(v) = j.get("canBurnInSunlight") && !v.is_json_null() {
                    s.can_burn_in_sunlight = v.as_bool();
                }
                if let Some(v) = j.get("canBeWithered") && !v.is_json_null() {
                    s.can_be_withered = v.as_bool();
                }
                if let Some(v) = j.get("healthMultiplier") && !v.is_json_null() {
                    s.health_multiplier = if v.is_json_null() { None } else { Some(v.as_double()) };
                }
                if let Some(v) = j.get("damageMultiplier") && !v.is_json_null() {
                    s.damage_multiplier = if v.is_json_null() { None } else { Some(v.as_double()) };
                }
                if let Some(v) = j.get("speedMultiplier") && !v.is_json_null() {
                    s.speed_multiplier = if v.is_json_null() { None } else { Some(v.as_double()) };
                }
                Some(s)
            }
        }
    }

    /// `LegacyFile.Particle` (com/lion/graveyard/config/LegacyFile.java)
    pub mod legacy_file_particle {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Particle {
            pub can_generate: bool,
            pub spawn_chance: i32,
        }
        impl Particle {
            pub fn new() -> Self {
                let mut s = Self { can_generate: false, spawn_chance: 0i32 };
                s
            }
            /// Gson.toJson: the instance fields by name.
            pub fn to_jjson(&self) -> JJson {
                let mut j = JJson::object();
                j.add("canGenerate", JJson::Bool(self.can_generate));
                j.add("spawnChance", JJson::int(self.spawn_chance as i64));
                j
            }
            /// Gson.fromJson: a new one (its constructor's values), with the fields the JSON has.
            pub fn from_jjson(j: &JJson) -> Option<Self> {
                if j.is_json_null() {
                    return None;
                }
                let mut s = Self::new();
                if let Some(v) = j.get("canGenerate") && !v.is_json_null() {
                    s.can_generate = v.as_bool();
                }
                if let Some(v) = j.get("spawnChance") && !v.is_json_null() {
                    s.spawn_chance = v.as_int() as i32;
                }
                Some(s)
            }
        }
    }

    /// `LegacyFile.Structure` (com/lion/graveyard/config/LegacyFile.java)
    pub mod legacy_file_structure {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Structure {
            pub enabled: bool,
            pub chance: Option<f64>,
            pub min_y: Option<i32>,
            pub max_y: Option<i32>,
        }
        impl Structure {
            pub fn new() -> Self {
                let mut s = Self { enabled: false, chance: None, min_y: None, max_y: None };
                s
            }
            /// Gson.toJson: the instance fields by name.
            pub fn to_jjson(&self) -> JJson {
                let mut j = JJson::object();
                j.add("enabled", JJson::Bool(self.enabled));
                j.add("chance", match &self.chance { Some(v) => JJson::double(*v), None => JJson::Null });
                j.add("minY", match &self.min_y { Some(v) => JJson::int(*v as i64), None => JJson::Null });
                j.add("maxY", match &self.max_y { Some(v) => JJson::int(*v as i64), None => JJson::Null });
                j
            }
            /// Gson.fromJson: a new one (its constructor's values), with the fields the JSON has.
            pub fn from_jjson(j: &JJson) -> Option<Self> {
                if j.is_json_null() {
                    return None;
                }
                let mut s = Self::new();
                if let Some(v) = j.get("enabled") && !v.is_json_null() {
                    s.enabled = v.as_bool();
                }
                if let Some(v) = j.get("chance") && !v.is_json_null() {
                    s.chance = if v.is_json_null() { None } else { Some(v.as_double()) };
                }
                if let Some(v) = j.get("minY") && !v.is_json_null() {
                    s.min_y = if v.is_json_null() { None } else { Some(v.as_int() as i32) };
                }
                if let Some(v) = j.get("maxY") && !v.is_json_null() {
                    s.max_y = if v.is_json_null() { None } else { Some(v.as_int() as i32) };
                }
                Some(s)
            }
        }
    }

    /// `LegacyKeys` (com/lion/graveyard/config/LegacyKeys.java)
    pub mod legacy_keys {
        use super::*;
        pub static BOOLEANS: LazyLock<Vec<Vec<String>>> = LazyLock::new(|| vec![vec!["urnHasDoubleInventory".to_string(), "behaviour.urnDoubleInventory".to_string()], vec!["disableWitherSkeletonSpawner".to_string(), "behaviour.replaceWitherSkeletonSpawner".to_string()], vec!["enableBossMusic".to_string(), "general.bossMusic".to_string()], vec!["chroniclePagesInLoot".to_string(), "chronicle.pagesInLoot".to_string()], vec!["ashParticles".to_string(), "general.ashParticles".to_string()], vec!["structuresOnly".to_string(), "general.structuresOnly".to_string()], vec!["chronicleInventoryButton".to_string(), "chronicle.inventoryButton".to_string()], vec!["chronicleToasts".to_string(), "chronicle.toasts".to_string()], vec!["corvidsStealItems".to_string(), "behaviour.magpiesStealItems".to_string()], vec!["ghoulingCollectsItems".to_string(), "behaviour.ghoulingCollectsItems".to_string()], vec!["crowsEatCrops".to_string(), "behaviour.crowsEatCrops".to_string()], vec!["smolderingBlocksInVillages".to_string(), "behaviour.smolderingBlocksInVillages".to_string()], vec!["skeletonCreeperDestroysBlocks".to_string(), "behaviour.skeletonCreeperDestroysBlocks".to_string()], vec!["wraithGriefing".to_string(), "behaviour.wraithGriefing".to_string()]]);
        pub static NUMBERS: LazyLock<Vec<Vec<String>>> = LazyLock::new(|| vec![vec!["skeletonCreeperExplosionRadius".to_string(), "behaviour.skeletonCreeperExplosionRadius".to_string()], vec!["corvidTheftChance".to_string(), "behaviour.magpieTheftChance".to_string()]]);
        pub static MOB_FIELDS: LazyLock<Vec<Vec<String>>> = LazyLock::new(|| vec![vec!["weight".to_string(), "spawn.weight".to_string()], vec!["minGroup".to_string(), "spawn.minGroup".to_string()], vec!["maxGroup".to_string(), "spawn.maxGroup".to_string()], vec!["canBurnInSunlight".to_string(), "burnsInSunlight".to_string()], vec!["canBeWithered".to_string(), "canBeWithered".to_string()], vec!["healthMultiplier".to_string(), "health".to_string()], vec!["damageMultiplier".to_string(), "damage".to_string()], vec!["speedMultiplier".to_string(), "speed".to_string()]]);
        pub static HORDE_FIELDS: LazyLock<Vec<Vec<String>>> = LazyLock::new(|| vec![vec!["enabled".to_string(), "enabled".to_string()], vec!["mobSpawnAttempts".to_string(), "spawnAttempts".to_string()], vec!["ticksUntilNextSpawn".to_string(), "intervalTicks".to_string()], vec!["additionalRandomizedTicks".to_string(), "randomExtraTicks".to_string()], vec!["chancePerCheck".to_string(), "chancePerCheck".to_string()], vec!["minDistance".to_string(), "minDistance".to_string()], vec!["maxDistance".to_string(), "maxDistance".to_string()], vec!["nightOnly".to_string(), "nightOnly".to_string()]]);
        pub static CHAMPION_FIELDS: LazyLock<Vec<Vec<String>>> = LazyLock::new(|| vec![vec!["summoningNeedsStaffFragments".to_string(), "summoning.needsStaffFragments".to_string()], vec!["isBossSummonableItem".to_string(), "summoning.summonItems".to_string()], vec!["isBloodCollectableEntity".to_string(), "summoning.bloodEntities".to_string()], vec!["spawnInvulnerableTicks".to_string(), "summoning.spawnInvulnerableTicks".to_string()], vec!["healthInCastingPhase".to_string(), "stats.castingHealth".to_string()], vec!["healthInHuntingPhase".to_string(), "stats.huntingHealth".to_string()], vec!["damageCastingPhase".to_string(), "stats.castingDamage".to_string()], vec!["damageHuntingPhaseAddition".to_string(), "stats.huntingDamageBonus".to_string()], vec!["armor".to_string(), "stats.armor".to_string()], vec!["armorToughness".to_string(), "stats.armorToughness".to_string()], vec!["speedInHuntPhase".to_string(), "stats.huntingSpeedBonus".to_string()], vec!["durationHuntingPhase".to_string(), "spells.huntDuration".to_string()], vec!["durationFallingCorpseSpell".to_string(), "spells.corpseDuration".to_string()], vec!["cooldownCorpseSpell".to_string(), "spells.corpseCooldown".to_string()], vec!["durationHealingSpell".to_string(), "spells.healingDuration".to_string()], vec!["cooldownTeleportPlayerAndHeal".to_string(), "spells.healingCooldown".to_string()], vec!["healThresholdHealth".to_string(), "spells.healThresholdHealth".to_string()], vec!["durationLevitationSpell".to_string(), "spells.levitationDuration".to_string()], vec!["cooldownLevitationSpell".to_string(), "spells.levitationCooldown".to_string()], vec!["playerTeleportYOffset".to_string(), "spells.playerTeleportYOffset".to_string()], vec!["maxAmountSkullsInShootSkullSpell".to_string(), "spells.maxSkulls".to_string()], vec!["skullExplosionRadius".to_string(), "spells.skullExplosionRadius".to_string()], vec!["skullsDestroyBlocks".to_string(), "spells.skullsDestroyBlocks".to_string()], vec!["isMultiphaseFight".to_string(), "fight.multiphase".to_string()], vec!["isInvulnerableDuringSpells".to_string(), "fight.invulnerableDuringSpells".to_string()], vec!["phaseInvulnerableTicks".to_string(), "fight.phaseInvulnerableTicks".to_string()], vec!["cooldownHunt".to_string(), "fight.huntCooldown".to_string()], vec!["cooldownCharge".to_string(), "fight.chargeCooldown".to_string()], vec!["cooldownPounce".to_string(), "fight.pounceCooldown".to_string()], vec!["maxSummonedMobs".to_string(), "fight.maxSummonedMobs".to_string()], vec!["maxGroupSizeSummonedMobs".to_string(), "fight.maxSummonGroupSize".to_string()], vec!["ghoulSpawnTimerInFight".to_string(), "fight.ghoulSpawnTimer".to_string()], vec!["bossBarDarkenSky".to_string(), "effects.darkenSky".to_string()], vec!["bossBarThickenFog".to_string(), "effects.thickenFog".to_string()], vec!["bossMusicVolume".to_string(), "effects.musicVolume".to_string()]]);
        // ENTRIES: not translated, its value reads other changing statics.
        #[derive(Clone, Debug)]
        pub struct LegacyKeys {
        }
        impl LegacyKeys {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LegacyKeys.entries(0): not translated, it uses LegacyKeys.ENTRIES, which isn't translated (its value reads other changing statics).
        pub fn build() -> Vec<legacy_keys_entry::Entry> {
            let mut entries: Vec<legacy_keys_entry::Entry> = Vec::new();
            for id in (*config_schema::STRUCTURE_IDS).iter() {
                legacy_keys::copy(&mut entries, &(format!("structureConfigEntries.{}.enabled", id)), &(format!("structures.{}.enabled", id)));
                legacy_keys::copy(&mut entries, &(format!("structureConfigEntries.{}.chance", id)), &(format!("structures.{}.chance", id)));
                if j_eq(id, "crypt") || j_eq(id, "lich_prison") {
                    legacy_keys::copy(&mut entries, &(format!("structureConfigEntries.{}.minY", id)), &(format!("structures.{}.minY", id)));
                    legacy_keys::copy(&mut entries, &(format!("structureConfigEntries.{}.maxY", id)), &(format!("structures.{}.maxY", id)));
                }
            }
            legacy_keys::copy(&mut entries, "particleConfigEntries.graveyard_fog_particle.canGenerate", "behaviour.fogParticles.enabled");
            legacy_keys::copy(&mut entries, "particleConfigEntries.graveyard_fog_particle.spawnChance", "behaviour.fogParticles.rarity");
            for mob in (*config_schema::FULL_MOB_IDS).iter() {
                entries.push(legacy_keys_entry::Entry::new(&(format!("mobConfigEntries.{}.enabled", mob)), &(format!("mobs.{}.enabled", mob)), legacy_keys_rule::Rule::MobSwitch));
                for field in (*legacy_keys::MOB_FIELDS).iter() {
                    legacy_keys::copy(&mut entries, &(format!("mobConfigEntries.{}.{}", mob, field[0])), &(format!("mobs.{}.{}", mob, field[1])));
                }
            }
            for field in (*legacy_keys::HORDE_FIELDS).iter() {
                legacy_keys::copy(&mut entries, &(format!("hordeConfigEntries.horde_spawn.{}", field[0])), &(format!("hordes.{}", field[1])));
            }
            for field in (*legacy_keys::CHAMPION_FIELDS).iter() {
                legacy_keys::copy(&mut entries, &(format!("corruptedChampionConfigEntries.corrupted_champion.{}", field[0])), &(format!("corruptedChampion.{}", field[1])));
            }
            entries.push(legacy_keys_entry::Entry::new("booleanEntries.tormentedVillagerLight", "", legacy_keys_rule::Rule::Skip));
            for key in (*legacy_keys::BOOLEANS).iter() {
                legacy_keys::copy(&mut entries, &(format!("booleanEntries.{}", key[0])), key[1]);
            }
            for mob in (*config_schema::ALL_MOB_IDS).iter() {
                entries.push(legacy_keys_entry::Entry::new(&(format!("mobSpawnEnabled.{}", mob)), &(format!("mobs.{}.enabled", mob)), legacy_keys_rule::Rule::MobSwitch));
            }
            for key in (*legacy_keys::NUMBERS).iter() {
                legacy_keys::copy(&mut entries, &(format!("numberEntries.{}", key[0])), key[1]);
            }
            entries.push(legacy_keys_entry::Entry::new("numberEntries.namelessHangedMinOffers", "behaviour.namelessHangedOffers.min", legacy_keys_rule::Rule::Floor));
            entries.push(legacy_keys_entry::Entry::new("numberEntries.namelessHangedMaxOffers", "behaviour.namelessHangedOffers.max", legacy_keys_rule::Rule::Floor));
            return entries.to_vec();
        }
        pub fn copy(entries: &mut Vec<legacy_keys_entry::Entry>, old_path: &str, new_path: &str) {
            entries.push(legacy_keys_entry::Entry::new(&old_path, &new_path, legacy_keys_rule::Rule::Copy));
        }
    }

    /// `LegacyKeys.Entry` (com/lion/graveyard/config/LegacyKeys.java)
    pub mod legacy_keys_entry {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Entry {
            pub old_path: String,
            pub new_path: String,
            pub rule: legacy_keys_rule::Rule,
        }
        impl Entry {
            pub fn new(old_path: &str, new_path: &str, rule: legacy_keys_rule::Rule) -> Self {
                let mut s = Self { old_path: String::new(), new_path: String::new(), rule: rule };
                s.old_path = old_path.to_string();
                s.new_path = new_path.to_string();
                s.rule = rule;
                s
            }
            pub fn old_path(&self) -> String {
                return self.old_path.to_string();
            }
            pub fn new_path(&self) -> String {
                return self.new_path.to_string();
            }
            pub fn rule(&self) -> legacy_keys_rule::Rule {
                return self.rule;
            }
        }
    }

    /// `LegacyKeys.Rule` (com/lion/graveyard/config/LegacyKeys.java)
    pub mod legacy_keys_rule {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Rule {
            Copy,
            Floor,
            MobSwitch,
            Skip,
        }
        impl Rule {
            pub const VALUES: [Rule; 4] = [Rule::Copy, Rule::Floor, Rule::MobSwitch, Rule::Skip, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Rule::Copy => "COPY",
                    Rule::Floor => "FLOOR",
                    Rule::MobSwitch => "MOB_SWITCH",
                    Rule::Skip => "SKIP",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "COPY" => Rule::Copy,
                    "FLOOR" => Rule::Floor,
                    "MOB_SWITCH" => Rule::MobSwitch,
                    "SKIP" => Rule::Skip,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `LegacyMigration` (com/lion/graveyard/config/LegacyMigration.java)
    pub mod legacy_migration {
        use super::*;
        pub static GSON: LazyLock<JGson> = LazyLock::new(|| JGson::default());
        pub static SWITCH_ONLY_MOBS: LazyLock<JSet<String>> = LazyLock::new(|| JSet::from_iter(["acolyte".to_string(), "wraith".to_string(), "corrupted_vindicator".to_string(), "corrupted_pillager".to_string(), "corrupted_evoker".to_string()]));
        // OLD_PATH_BY_NEW_PATH: not translated, it calls LegacyMigration.oldPathsByNewPath, which isn't translated.
        // KNOWN_PREFIXES: not translated, it calls LegacyMigration.knownPrefixes, which isn't translated.
        #[derive(Clone, Debug)]
        pub struct LegacyMigration {
        }
        impl LegacyMigration {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LegacyMigration.migrate(1): not translated, unknown name `exception`.
        // LegacyMigration.rejected(1): not translated, it uses ConfigValues, which isn't translated (it uses `Object`).
        // LegacyMigration.joinLines(1): not translated, it creates `BufferedReader` (java.io).
        // LegacyMigration.holdsNonFiniteNumber(1): not translated, `null` (Rust has no null: this needs an Option).
        // LegacyMigration.copyValue(3): not translated, it uses `Object`.
        // LegacyMigration.mergeMobSwitch(3): not translated, it uses ConfigSchema, which isn't translated (it uses Section, which isn't translated (it extends `Node`)).
        // LegacyMigration.rawValue(2): not translated, it uses `Object`.
        // LegacyMigration.entryMap(2): not translated, it uses `Object`.
        // LegacyMigration.fieldOf(3): not translated, it uses `Object`.
        // LegacyMigration.toJson(1): not translated, it uses `Object`.
        pub fn add_once(list: &mut Vec<String>, path: &str) {
            if !(list.iter().any(|e| e == &path)) {
                list.push(path.to_string());
            }
        }
        // LegacyMigration.collectUnknownKeys(2): not translated, it uses `JsonReader` (com.google.gson.stream).
        // LegacyMigration.collectUnknown(3): not translated, it uses LegacyMigration.KNOWN_PREFIXES, which isn't translated (its value reads other changing statics).
        // LegacyMigration.knownPrefixes(0): not translated, it calls LegacyKeys.entries, which isn't translated.
        // LegacyMigration.oldPathsByNewPath(0): not translated, it calls LegacyKeys.entries, which isn't translated.
    }

    /// `Slf4jConfigLog` (com/lion/graveyard/config/Slf4jConfigLog.java)
    pub mod slf4j_config_log {
        use super::*;
        // The class itself isn't translated: it implements `ConfigLog`. Its static parts are below.
        // Slf4jConfigLog.Slf4jConfigLog(1): not translated, its class isn't translated (it implements `ConfigLog`).
        // Slf4jConfigLog.info(1): not translated, its class isn't translated (it implements `ConfigLog`).
        // Slf4jConfigLog.warn(1): not translated, its class isn't translated (it implements `ConfigLog`).
        // Slf4jConfigLog.error(1): not translated, its class isn't translated (it implements `ConfigLog`).
    }

    /// `AcolyteEntity` (com/lion/graveyard/entities/AcolyteEntity.java)
    pub mod acolyte_entity {
        use super::*;
        // The class itself isn't translated: it extends `CorruptedIllager`. Its static parts are below.
        // AcolyteEntity.AcolyteEntity(2): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AcolyteEntity.populateDefaultEquipmentSlots(2): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AcolyteEntity.playAmbientSound(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AcolyteEntity.playHurtSound(1): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AcolyteEntity.die(1): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AcolyteEntity.createWitherRose(1): not translated, its class isn't translated (it extends `CorruptedIllager`).
    }

    /// `ChaseTarget` (com/lion/graveyard/entities/ai/ChaseTarget.java)
    pub mod chase_target {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ChaseTarget {
        }
        impl ChaseTarget {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ChaseTarget.isLive(1): not translated, it uses Minecraft's `LivingEntity`.
    }

    /// `CorvidMoveControl` (com/lion/graveyard/entities/ai/control/CorvidMoveControl.java)
    pub mod corvid_move_control {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `FlyingMoveControl`. Its static parts are below.
        pub const MAX_PITCH_CHANGE: f32 = 10.0f32;
        pub const VERTICAL_DEADZONE: f64 = 0.15f64;
        pub const VERTICAL_GAIN: f64 = 0.2f64;
        pub const CRUISE_SPEED: f64 = 0.25f64;
        pub const MAX_SINK: f64 = 0.3f64;
        pub const MAX_CLIMB: f64 = 0.2f64;
        pub const COAST_THRUST: f64 = 0.6f64;
        // CorvidMoveControl.CorvidMoveControl(1): not translated, its class isn't translated (it extends Minecraft's `FlyingMoveControl`).
        // CorvidMoveControl.tick(0): not translated, its class isn't translated (it extends Minecraft's `FlyingMoveControl`).
        // CorvidMoveControl.coast(0): not translated, its class isn't translated (it extends Minecraft's `FlyingMoveControl`).
    }

    /// `AbstractGraveyardMeleeGoal` (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java)
    pub mod abstract_graveyard_melee_goal {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractGraveyardMeleeGoal.AbstractGraveyardMeleeGoal(4): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.canUse(0): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.isInReach(1): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.isNewOrFarMoved(1): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.rememberChecked(1): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.canContinueToUse(0): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.start(0): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.stop(0): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.requiresUpdateEveryTick(0): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.tick(0): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.onTargetTick(0): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.attack(2): not translated, it has no body.
        // AbstractGraveyardMeleeGoal.getSquaredMaxAttackDistance(1): not translated, its class isn't translated (it's generic).
        // AbstractGraveyardMeleeGoal.resetCooldown(1): not translated, its class isn't translated (it's generic).
    }

    /// `AttackWithOwnerGoal` (com/lion/graveyard/entities/ai/goals/AttackWithOwnerGoal.java)
    pub mod attack_with_owner_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `TargetGoal`. Its static parts are below.
        // AttackWithOwnerGoal.AttackWithOwnerGoal(1): not translated, its class isn't translated (it extends Minecraft's `TargetGoal`).
        // AttackWithOwnerGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `TargetGoal`).
        // AttackWithOwnerGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `TargetGoal`).
    }

    /// `CircleAnchorGoal` (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java)
    pub mod circle_anchor_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        pub const SPEED: f64 = 1.0f64;
        pub const RING_SPEED: f32 = 0.2f32;
        pub const MIN_RADIUS: i32 = 8;
        pub const RADIUS_SPREAD: i32 = 6;
        pub const MIN_HEIGHT: i32 = 10;
        pub const HEIGHT_SPREAD: i32 = 6;
        pub const DURATION: i32 = 600;
        pub const START_CHANCE: i32 = 240;
        // CircleAnchorGoal.CircleAnchorGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // CircleAnchorGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // CircleAnchorGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // CircleAnchorGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // CircleAnchorGoal.findMaxTop(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // CircleAnchorGoal.requiresUpdateEveryTick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // CircleAnchorGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // CircleAnchorGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `FleeGraveyardMobsGoal` (com/lion/graveyard/entities/ai/goals/FleeGraveyardMobsGoal.java)
    pub mod flee_graveyard_mobs_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AvoidEntityGoal`. Its static parts are below.
        // FleeGraveyardMobsGoal.FleeGraveyardMobsGoal(3): not translated, its class isn't translated (it extends Minecraft's `AvoidEntityGoal`).
    }

    /// `FlockStartleGoal` (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java)
    pub mod flock_startle_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        pub const SENTINEL_RANGE: f64 = 6.0f64;
        pub const MEMBER_RANGE: f64 = 3.0f64;
        pub const SNEAK_FACTOR: f64 = 0.5f64;
        pub const SCAN_INTERVAL: i32 = 5;
        pub const SCARECROW_SCAN_INTERVAL: i32 = 20;
        pub const SCARE_FLOCK_SIZE: i32 = 6;
        pub const SCARECROW_WITNESS_RANGE: f64 = 16.0f64;
        pub const ESCAPE_DISTANCE: i32 = 14;
        pub const ESCAPE_HEIGHT: i32 = 12;
        pub const ESCAPE_SPEED: f64 = 1.3f64;
        // FlockStartleGoal.FlockStartleGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FlockStartleGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FlockStartleGoal.watch(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FlockStartleGoal.watchForScarecrows(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FlockStartleGoal.findEscape(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FlockStartleGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FlockStartleGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FlockStartleGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `FlyOntoPerchGoal` (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java)
    pub mod fly_onto_perch_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `WaterAvoidingRandomFlyingGoal`. Its static parts are below.
        pub const SEARCH_RADIUS: i32 = 8;
        pub const MAX_HOME_SEARCH_DISTANCE: f64 = 48.0f64;
        pub const SEARCH_HEIGHT: i32 = 6;
        pub const SEARCH_DEPTH: i32 = 12;
        pub const STAY_PERCHED_CHANCE: i32 = 6;
        pub const SOAR_CHANCE: f32 = 0.25f32;
        // FlyOntoPerchGoal.FlyOntoPerchGoal(2): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.getSearchCenter(0): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.isUsableHome(1): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.getPosition(0): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.getWanderFallbackTarget(0): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.getSoarTarget(0): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.getPerchTarget(0): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.isField(1): not translated, it uses Minecraft's `BlockState`.
        // FlyOntoPerchGoal.scorePerch(2): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.canPerchOn(2): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.isBonePerch(2): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.isRepelled(2): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
        // FlyOntoPerchGoal.isClear(2): not translated, its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`).
    }

    /// `FollowOwnerGoal` (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java)
    pub mod follow_owner_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        pub const TELEPORT_DISTANCE: i32 = 12;
        pub const HORIZONTAL_RANGE: i32 = 2;
        pub const HORIZONTAL_VARIATION: i32 = 3;
        pub const VERTICAL_VARIATION: i32 = 1;
        // FollowOwnerGoal.FollowOwnerGoal(5): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FollowOwnerGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FollowOwnerGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FollowOwnerGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FollowOwnerGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FollowOwnerGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FollowOwnerGoal.tryTeleport(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FollowOwnerGoal.tryTeleportTo(3): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FollowOwnerGoal.canTeleportTo(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // FollowOwnerGoal.getRandomInt(2): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `GhoulingCollectItemsGoal` (com/lion/graveyard/entities/ai/goals/GhoulingCollectItemsGoal.java)
    pub mod ghouling_collect_items_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        pub const COLLECT_RANGE: f64 = 1.5f64;
        pub const MAX_TICKS: i32 = 200;
        // GhoulingCollectItemsGoal.GhoulingCollectItemsGoal(2): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // GhoulingCollectItemsGoal.collect(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // GhoulingCollectItemsGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // GhoulingCollectItemsGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // GhoulingCollectItemsGoal.isValid(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // GhoulingCollectItemsGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // GhoulingCollectItemsGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `GhoulingMeleeAttackGoal` (com/lion/graveyard/entities/ai/goals/GhoulingMeleeAttackGoal.java)
    pub mod ghouling_melee_attack_goal {
        use super::*;
        // The class itself isn't translated: it extends `AbstractGraveyardMeleeGoal`. Its static parts are below.
        // GhoulingMeleeAttackGoal.GhoulingMeleeAttackGoal(3): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // GhoulingMeleeAttackGoal.attack(2): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
    }

    /// `GhoulMeleeAttackGoal` (com/lion/graveyard/entities/ai/goals/GhoulMeleeAttackGoal.java)
    pub mod ghoul_melee_attack_goal {
        use super::*;
        // The class itself isn't translated: it extends `AbstractGraveyardMeleeGoal`. Its static parts are below.
        // GhoulMeleeAttackGoal.GhoulMeleeAttackGoal(3): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // GhoulMeleeAttackGoal.attack(2): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
    }

    /// `HopAwayGoal` (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java)
    pub mod hop_away_goal {
        use super::*;
        // The class itself isn't translated: it extends `HopGoal`. Its static parts are below.
        pub const PLAYER_RANGE: f64 = 6.0f64;
        pub const COOLDOWN: i32 = 60;
        pub const FIND_ATTEMPTS: i32 = 4;
        pub const MIN_DISTANCE: f64 = 3.0f64;
        pub const DISTANCE_SPREAD: f64 = 2.0f64;
        pub const ANGLE_SPREAD_DEGREES: f64 = 60.0f64;
        // HopAwayGoal.HopAwayGoal(1): not translated, its class isn't translated (it extends `HopGoal`).
        // HopAwayGoal.canUse(0): not translated, its class isn't translated (it extends `HopGoal`).
        // HopAwayGoal.findTarget(0): not translated, its class isn't translated (it extends `HopGoal`).
        // HopAwayGoal.rollHopCount(0): not translated, its class isn't translated (it extends `HopGoal`).
        // HopAwayGoal.stop(0): not translated, its class isn't translated (it extends `HopGoal`).
    }

    /// `HopGoal` (com/lion/graveyard/entities/ai/goals/HopGoal.java)
    pub mod hop_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        pub const HOP_INTERVAL: i32 = 12;
        pub const HOP_UP: f64 = 0.32f64;
        pub const HOP_FORWARD: f64 = 0.22f64;
        pub const TIMEOUT: i32 = 120;
        pub const FIND_ATTEMPTS: i32 = 4;
        pub const MIN_DISTANCE: f64 = 2.0f64;
        pub const DISTANCE_SPREAD: f64 = 2.0f64;
        // HopGoal.HopGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.findTarget(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.isHopPathClear(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.isColumnClear(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.isEmptyShape(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.isLowHalf(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.getHopInterval(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.rollHopCount(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.requiresUpdateEveryTick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HopGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `JoinFlockGoal` (com/lion/graveyard/entities/ai/goals/JoinFlockGoal.java)
    pub mod join_flock_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        pub const SEARCH_INTERVAL: i32 = 40;
        pub const MOVE_INTERVAL: i32 = 10;
        // JoinFlockGoal.JoinFlockGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // JoinFlockGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // JoinFlockGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // JoinFlockGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // JoinFlockGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `LichMeleeGoal` (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java)
    pub mod lich_melee_goal {
        use super::*;
        // The class itself isn't translated: it extends `AbstractGraveyardMeleeGoal`. Its static parts are below.
        pub const DAMAGE_START_IN_ANIM: i32 = 18;
        pub const HIT_REACH_SQUARED: f64 = 36.0f64;
        // LichMeleeGoal.LichMeleeGoal(3): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // LichMeleeGoal.canContinueToUse(0): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // LichMeleeGoal.tick(0): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // LichMeleeGoal.isPhaseOneHold(1): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // LichMeleeGoal.isStraightBelowOrAbove(1): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // LichMeleeGoal.isDirectChase(1): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // LichMeleeGoal.onTargetTick(0): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // LichMeleeGoal.attack(2): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // LichMeleeGoal.getSquaredMaxAttackDistance(1): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
    }

    /// `NightmareMeleeAttackGoal` (com/lion/graveyard/entities/ai/goals/NightmareMeleeAttackGoal.java)
    pub mod nightmare_melee_attack_goal {
        use super::*;
        // The class itself isn't translated: it extends `AbstractGraveyardMeleeGoal`. Its static parts are below.
        // NightmareMeleeAttackGoal.NightmareMeleeAttackGoal(3): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // NightmareMeleeAttackGoal.attack(2): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
    }

    /// `PickTreasureGoal` (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java)
    pub mod pick_treasure_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        pub const RESCAN_INTERVAL: i32 = 10;
        pub const SPEED: f64 = 1.2f64;
        pub const GIVE_UP_DISTANCE: f64 = 16.0f64;
        pub const MAX_DURATION: i32 = 200;
        // PickTreasureGoal.PickTreasureGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // PickTreasureGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // PickTreasureGoal.findTreasure(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // PickTreasureGoal.requiresUpdateEveryTick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // PickTreasureGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // PickTreasureGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // PickTreasureGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // PickTreasureGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `RevenantMeleeAttackGoal` (com/lion/graveyard/entities/ai/goals/RevenantMeleeAttackGoal.java)
    pub mod revenant_melee_attack_goal {
        use super::*;
        // The class itself isn't translated: it extends `AbstractGraveyardMeleeGoal`. Its static parts are below.
        // RevenantMeleeAttackGoal.RevenantMeleeAttackGoal(3): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
        // RevenantMeleeAttackGoal.attack(2): not translated, its class isn't translated (it extends `AbstractGraveyardMeleeGoal`).
    }

    /// `SettleGoal` (com/lion/graveyard/entities/ai/goals/SettleGoal.java)
    pub mod settle_goal {
        use super::*;
        // The class itself isn't translated: it extends `FlyOntoPerchGoal`. Its static parts are below.
        pub const DEATH_LANDING_RANGE_SQ: i32 = 36;
        pub const DEATH_ANCHOR_RANGE_SQ: i32 = 576;
        // SettleGoal.SettleGoal(2): not translated, its class isn't translated (it extends `FlyOntoPerchGoal`).
        // SettleGoal.canUse(0): not translated, its class isn't translated (it extends `FlyOntoPerchGoal`).
        // SettleGoal.getSearchCenter(0): not translated, its class isn't translated (it extends `FlyOntoPerchGoal`).
        // SettleGoal.canPerchOn(2): not translated, its class isn't translated (it extends `FlyOntoPerchGoal`).
        // SettleGoal.isRepelled(2): not translated, its class isn't translated (it extends `FlyOntoPerchGoal`).
        // SettleGoal.scorePerch(2): not translated, its class isn't translated (it extends `FlyOntoPerchGoal`).
        // SettleGoal.isNearDeathSite(2): not translated, its class isn't translated (it extends `FlyOntoPerchGoal`).
    }

    /// `SitGoal` (com/lion/graveyard/entities/ai/goals/SitGoal.java)
    pub mod sit_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // SitGoal.SitGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SitGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SitGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SitGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // SitGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `StartleGoal` (com/lion/graveyard/entities/ai/goals/StartleGoal.java)
    pub mod startle_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // StartleGoal.StartleGoal(2): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // StartleGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // StartleGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // StartleGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // StartleGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `TrackOwnerAttackerGoal` (com/lion/graveyard/entities/ai/goals/TrackOwnerAttackerGoal.java)
    pub mod track_owner_attacker_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `TargetGoal`. Its static parts are below.
        // TrackOwnerAttackerGoal.TrackOwnerAttackerGoal(1): not translated, its class isn't translated (it extends Minecraft's `TargetGoal`).
        // TrackOwnerAttackerGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `TargetGoal`).
        // TrackOwnerAttackerGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `TargetGoal`).
    }

    /// `WalkGoal` (com/lion/graveyard/entities/ai/goals/WalkGoal.java)
    pub mod walk_goal {
        use super::*;
        // The class itself isn't translated: it extends `HopGoal`. Its static parts are below.
        pub const WALK_SPEED: f64 = 0.07f64;
        pub const ARRIVE_DISTANCE_SQ: f64 = 0.36f64;
        pub const TIMEOUT: i32 = 100;
        // WalkGoal.WalkGoal(1): not translated, its class isn't translated (it extends `HopGoal`).
        // WalkGoal.canUse(0): not translated, its class isn't translated (it extends `HopGoal`).
        // WalkGoal.start(0): not translated, its class isn't translated (it extends `HopGoal`).
        // WalkGoal.tick(0): not translated, its class isn't translated (it extends `HopGoal`).
        // WalkGoal.canContinueToUse(0): not translated, its class isn't translated (it extends `HopGoal`).
        // WalkGoal.stop(0): not translated, its class isn't translated (it extends `HopGoal`).
    }

    /// `AngerableGraveyardEntity` (com/lion/graveyard/entities/AngerableGraveyardEntity.java)
    pub mod angerable_graveyard_entity {
        use super::*;
        // The class itself isn't translated: it extends `HordeGraveyardEntity`. Its static parts are below.
        pub static ATTACKING_SPEED_BOOST_ID: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("attacking_speed_boost_id"));
        // ATTACKING_SPEED_BOOST: not translated, it uses Minecraft's `AttributeModifier`.
        // ANGRY: not translated, it uses Minecraft's `EntityDataAccessor`.
        // PROVOKED: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ANGER_TIME: not translated, it uses Minecraft's `UniformInt`.
        // AngerableGraveyardEntity.AngerableGraveyardEntity(3): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.aiStep(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.defineSynchedData(1): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.readPersistentAngerSaveData(2): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.isAngry(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.setPersistentAngerEndTime(1): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.getPersistentAngerEndTime(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.getPersistentAngerTarget(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.setPersistentAngerTarget(1): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.startPersistentAngerTimer(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // AngerableGraveyardEntity.setTarget(1): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
    }

    /// `AnimatedCorruptedIllager` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java)
    pub mod animated_corrupted_illager {
        use super::*;
        // The class itself isn't translated: it extends `CorruptedIllager`. Its static parts are below.
        pub const ANIMATION_PREFIX: &str = "animation.corrupted_illager.";
        pub const SPAWN_TICKS: i32 = 92;
        pub const DEATH_TICKS: i32 = 49;
        pub const GLITCH_TICKS: i32 = 49;
        pub const STRIKE_TICKS: i32 = 27;
        pub const BITE_TICKS: i32 = 21;
        pub const STRIKE_HIT_TICKS: i32 = 16;
        pub const BITE_HIT_TICKS: i32 = 13;
        pub const GLITCH_CHANCE: i32 = 2400;
        pub const GLITCH_COOLDOWN_TICKS: i32 = 600;
        pub const SOUND_RESTART_SLACK: i32 = 2;
        pub const ATTACK_STRIKE: i8 = 0i8;
        pub const ATTACK_BITE: i8 = 1i8;
        pub const DECAY_LEVELS: i32 = 2;
        pub const LEFT_CLAW: i32 = 1;
        pub const RIGHT_CLAW: i32 = 2;
        pub const WANDER_SPEED: f64 = 0.27f64;
        pub const MOVING_LIMB_SPEED: f32 = 0.05f32;
        pub const WANDER_STUCK_TICKS: i32 = 20;
        // DECAY: not translated, it uses Minecraft's `EntityDataAccessor`.
        // SPAWN_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ATTACK_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ATTACK_KIND: not translated, it uses Minecraft's `EntityDataAccessor`.
        // GLITCH_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // CLAWS: not translated, it uses Minecraft's `EntityDataAccessor`.
        // WANDERING: not translated, it uses Minecraft's `EntityDataAccessor`.
        // SPAWN: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // DEATH: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // STRIKE: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // BITE: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // GLITCH: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // ATTACK_RUN: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // WANDER: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // IDLE: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // AnimatedCorruptedIllager.AnimatedCorruptedIllager(3): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.hasSpawnAnimation(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.defineSynchedData(1): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.getDecay(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.getWanderSpeed(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.hasLeftClaw(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.hasRightClaw(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.getClaws(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.getSpawnTimer(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.finalizeSpawn(4): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.readAdditionalSaveData(1): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.tick(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.currentAnimationSound(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.tickAnimationSound(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.onAnimationKeyframeZero(2): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.startSound(1): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.getDeathSound(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.resolvePendingHit(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.isSwimmingInWater(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.tickDown(1): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.aiStep(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.customServerAiStep(1): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.doHurtTarget(2): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.isAttackPending(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.isImmobile(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.tickDeath(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.registerControllers(1): not translated, its class isn't translated (it extends `CorruptedIllager`).
        // AnimatedCorruptedIllager.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends `CorruptedIllager`).
    }

    /// `AnimatedCorruptedIllager.AnimationSound` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java)
    pub mod animated_corrupted_illager_animation_sound {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // AnimatedCorruptedIllager.AnimationSound.AnimationSound(4): not translated, its class isn't translated (its constants carry values).
        // AnimatedCorruptedIllager.AnimationSound.of(1): not translated, it uses AnimationSound, which isn't translated (its constants carry values).
    }

    /// `AnimationClock` (com/lion/graveyard/entities/animation/AnimationClock.java)
    pub mod animation_clock {
        use super::*;
        pub const RENDER_GRACE_TICKS: i32 = 2;
        pub const TICKS_PER_SECOND: f64 = 20.0f64;
        pub const LOOP_EPSILON: f64 = 1.0e-6f64;
        // LAST_FRAME: not translated, it uses Minecraft's `Entity`.
        // listener: not translated, it uses Listener, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct AnimationClock {
        }
        impl AnimationClock {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AnimationClock.setListener(1): not translated, it uses Listener, which isn't translated (it's an interface).
        // AnimationClock.isRendered(1): not translated, it uses Minecraft's `Entity`.
        // AnimationClock.playTracked(2): not translated, it uses Minecraft's `Entity`.
        // AnimationClock.playDetached(2): not translated, it uses Minecraft's `Entity`.
        // AnimationClock.restart(1): not translated, it uses `AnimationController` (com.geckolib.animation).
        // AnimationClock.controller(3): not translated, it's generic.
        // AnimationClock.controller(5): not translated, it's generic.
    }

    /// `AnimationClock.ClockController` (com/lion/graveyard/entities/animation/AnimationClock.java)
    pub mod animation_clock_clock_controller {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AnimationClock.ClockController.ClockController(5): not translated, its class isn't translated (it's generic).
        // AnimationClock.ClockController.guard(1): not translated, it's generic.
        // AnimationClock.ClockController.restart(0): not translated, its class isn't translated (it's generic).
        // AnimationClock.ClockController.resumeAfterStop(1): not translated, its class isn't translated (it's generic).
        // AnimationClock.ClockController.isRunningTriggeredClip(0): not translated, its class isn't translated (it's generic).
        // AnimationClock.ClockController.checkControllerState(4): not translated, its class isn't translated (it's generic).
        // AnimationClock.ClockController.progressExistingAnimation(4): not translated, its class isn't translated (it's generic).
        // AnimationClock.ClockController.extractControllerState(5): not translated, its class isn't translated (it's generic).
        // AnimationClock.ClockController.appendResetStage(0): not translated, its class isn't translated (it's generic).
        // AnimationClock.ClockController.currentAnimationStage(0): not translated, its class isn't translated (it's generic).
        // AnimationClock.ClockController.applyStartOffset(2): not translated, its class isn't translated (it's generic).
    }

    /// `AnimationClock.KeyframeZeroSink` (com/lion/graveyard/entities/animation/AnimationClock.java)
    pub mod animation_clock_keyframe_zero_sink {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AnimationClock.KeyframeZeroSink.keyframeZero(2): not translated, it has no body.
    }

    /// `AnimationClock.Listener` (com/lion/graveyard/entities/animation/AnimationClock.java)
    pub mod animation_clock_listener {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AnimationClock.Listener.keyframeZero(3): not translated, it has no body.
        // AnimationClock.Listener.playTracked(2): not translated, it has no body.
        // AnimationClock.Listener.playDetached(2): not translated, it has no body.
    }

    /// `AnimationPreview` (com/lion/graveyard/entities/animation/AnimationPreview.java)
    pub mod animation_preview {
        use super::*;
        pub const ALL: &str = "*";
        pub const PREFIX: &str = "anim:";
        pub const UNKNOWN_STEP_TICKS: f64 = 40.0f64;
        // LOOPS: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // PLAYS: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // STEPS: not translated, it uses `AnimationController` (com.geckolib.animation).
        #[derive(Clone, Debug)]
        pub struct AnimationPreview {
        }
        impl AnimationPreview {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AnimationPreview.forcedAnimation(1): not translated, it uses Minecraft's `Entity`.
        // AnimationPreview.frame(2): not translated, it uses `RawAnimation` (com.geckolib.animation).
        // AnimationPreview.nextIndex(3): not translated, it uses `AnimationController` (com.geckolib.animation).
        // AnimationPreview.isStalled(3): not translated, it uses `AnimationController` (com.geckolib.animation).
    }

    /// `AnimationPreview.Step` (com/lion/graveyard/entities/animation/AnimationPreview.java)
    pub mod animation_preview_step {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Step {
            pub index: i32,
            pub started: f64,
        }
        impl Step {
            pub fn new() -> Self {
                let mut s = Self { index: 0i32, started: -1.0f64 };
                s
            }
        }
    }

    /// `AnimationPreviewClient` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java)
    pub mod animation_preview_client {
        use super::*;
        pub const FILE: &str = "graveyard_anim_preview.json";
        // PLAYBACKS: not translated, it uses Minecraft's `Entity`.
        // TRACKED: not translated, it uses Minecraft's `Entity`.
        // DIGS: not translated, it uses Minecraft's `Entity`.
        pub static DIG_SPECS: LazyLock<JMap<String, animation_preview_client_dig_spec::DigSpec>> = LazyLock::new(|| JMap::from_iter([("graveyard:corrupted_vindicator|animation.corrupted_illager.spawn".to_string(), animation_preview_client_dig_spec::DigSpec::new(85, &(vec![6, 15, 26, 37, 50]), true)), ("graveyard:corrupted_pillager|animation.corrupted_illager.spawn".to_string(), animation_preview_client_dig_spec::DigSpec::new(85, &(vec![6, 15, 26, 37, 50]), true)), ("graveyard:corrupted_evoker|animation.corrupted_illager.spawn".to_string(), animation_preview_client_dig_spec::DigSpec::new(85, &(vec![6, 15, 26, 37, 50]), true)), ("graveyard:ghoul|animation.ghoul.spawn".to_string(), animation_preview_client_dig_spec::DigSpec::new(70, &(vec![2, 15, 26, 42, 62]), false)), ("graveyard:ghouling|spawn".to_string(), animation_preview_client_dig_spec::DigSpec::new(54, &(vec![14, 17, 22, 27, 32, 47]), false))]));
        // LISTENER: not translated, its value couldn't be read: an anonymous class.
        // cues: not translated, it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`).
        // lastWorld: not translated, it uses Minecraft's `ClientLevel`.
        #[derive(Clone, Debug)]
        pub struct AnimationPreviewClient {
        }
        impl AnimationPreviewClient {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AnimationPreviewClient.clientTick(1): not translated, it uses Minecraft's `Minecraft`.
        // AnimationPreviewClient.startDig(3): not translated, it uses Minecraft's `Entity`.
        // AnimationPreviewClient.startPreview(2): not translated, it uses Minecraft's `Entity`.
        // AnimationPreviewClient.play(4): not translated, it uses Minecraft's `SoundInstance`.
        // AnimationPreviewClient.stop(1): not translated, it uses Minecraft's `SoundInstance`.
        // AnimationPreviewClient.load(0): not translated, it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`).
    }

    /// `AnimationPreviewClient.Cue` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java)
    pub mod animation_preview_client_cue {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `SoundEvent`. Its static parts are below.
        // AnimationPreviewClient.Cue.Cue(5): not translated, its class isn't translated (it uses Minecraft's `SoundEvent`).
        // AnimationPreviewClient.Cue.parse(1): not translated, it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`).
        // AnimationPreviewClient.Cue.event(0): not translated, its class isn't translated (it uses Minecraft's `SoundEvent`).
        // AnimationPreviewClient.Cue.offsetTicks(0): not translated, its class isn't translated (it uses Minecraft's `SoundEvent`).
        // AnimationPreviewClient.Cue.repeatTicks(0): not translated, its class isn't translated (it uses Minecraft's `SoundEvent`).
        // AnimationPreviewClient.Cue.volume(0): not translated, its class isn't translated (it uses Minecraft's `SoundEvent`).
        // AnimationPreviewClient.Cue.pitch(0): not translated, its class isn't translated (it uses Minecraft's `SoundEvent`).
    }

    /// `AnimationPreviewClient.Dig` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java)
    pub mod animation_preview_client_dig {
        use super::*;
        pub const MIN_GAP: i32 = 16;
        pub const GAP_SPREAD: i32 = 8;
        #[derive(Clone, Debug)]
        pub struct Dig {
            pub spec: animation_preview_client_dig_spec::DigSpec,
            pub start: i64,
            pub particles: bool,
            pub stroke: i32,
            pub next_scrape: i64,
            pub last_tick: i64,
            pub started: bool,
        }
        impl Dig {
            pub fn new(spec: &animation_preview_client_dig_spec::DigSpec, start: i64, particles: bool) -> Self {
                let mut s = Self { spec: spec.clone(), start: start, particles: particles, stroke: 0i32, next_scrape: start, last_tick: i64::MIN, started: false };
                s.spec = spec.clone();
                s.start = start;
                s.particles = particles;
                s.next_scrape = start;
                s
            }
            // AnimationPreviewClient.Dig.tick(2): not translated, it uses Minecraft's `Entity`.
        }
        // AnimationPreviewClient.Dig.groundBelow(1): not translated, it uses Minecraft's `BlockState`.
        // AnimationPreviewClient.Dig.playEarth(3): not translated, it uses Minecraft's `Entity`.
        // AnimationPreviewClient.Dig.playDig(3): not translated, it uses Minecraft's `Entity`.
    }

    /// `AnimationPreviewClient.DigSpec` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java)
    pub mod animation_preview_client_dig_spec {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct DigSpec {
            pub end_tick: i32,
            pub stroke_ticks: Vec<i32>,
            pub particles: bool,
        }
        impl DigSpec {
            pub fn new(end_tick: i32, stroke_ticks: &[i32], particles: bool) -> Self {
                let mut s = Self { end_tick: end_tick, stroke_ticks: Vec::new(), particles: particles };
                s.end_tick = end_tick;
                s.stroke_ticks = stroke_ticks.to_vec();
                s.particles = particles;
                s
            }
            pub fn end_tick(&self) -> i32 {
                return self.end_tick;
            }
            pub fn stroke_ticks(&self) -> Vec<i32> {
                return self.stroke_ticks.to_vec();
            }
            pub fn particles(&self) -> bool {
                return self.particles;
            }
        }
    }

    /// `AnimationPreviewClient.Playback` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java)
    pub mod animation_preview_client_playback {
        use super::*;
        // The class itself isn't translated: it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`). Its static parts are below.
        // AnimationPreviewClient.Playback.Playback(2): not translated, its class isn't translated (it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`)).
        // AnimationPreviewClient.Playback.tick(2): not translated, its class isn't translated (it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`)).
        // AnimationPreviewClient.Playback.stopSounds(0): not translated, its class isn't translated (it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`)).
    }

    /// `AnimationTable` (com/lion/graveyard/entities/animation/AnimationTable.java)
    pub mod animation_table {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const TRANSITION_TICKS: i32 = 5;
        pub const MOVEMENT_THRESHOLD: f32 = 0.05f32;
        // AnimationTable.AnimationTable(0): not translated, its class isn't translated (it's generic).
        // AnimationTable.create(0): not translated, it's generic.
        // AnimationTable.when(2): not translated, its class isn't translated (it's generic).
        // AnimationTable.when(2): not translated, its class isn't translated (it's generic).
        // AnimationTable.stopWhen(1): not translated, its class isn't translated (it's generic).
        // AnimationTable.otherwise(1): not translated, its class isn't translated (it's generic).
        // AnimationTable.startAt(2): not translated, its class isn't translated (it's generic).
        // AnimationTable.speed(1): not translated, its class isn't translated (it's generic).
        // AnimationTable.transitionTicks(1): not translated, its class isn't translated (it's generic).
        // AnimationTable.build(1): not translated, its class isn't translated (it's generic).
        // AnimationTable.build(2): not translated, its class isn't translated (it's generic).
        // AnimationTable.animationNames(0): not translated, its class isn't translated (it's generic).
        // AnimationTable.addName(2): not translated, it uses `RawAnimation` (com.geckolib.animation).
        // AnimationTable.isMoving(1): not translated, it uses `AnimationTest` (com.geckolib.animation.state).
    }

    /// `AnimationTable.Entry` (com/lion/graveyard/entities/animation/AnimationTable.java)
    pub mod animation_table_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AnimationTable.Entry.Entry(2): not translated, its class isn't translated (it's generic).
        // AnimationTable.Entry.condition(0): not translated, its class isn't translated (it's generic).
        // AnimationTable.Entry.animation(0): not translated, its class isn't translated (it's generic).
    }

    /// `BoneReturn` (com/lion/graveyard/entities/animation/BoneReturn.java)
    pub mod bone_return {
        use super::*;
        pub const TICKS: i32 = 5;
        // RETURNS: not translated, it uses `DataTicket` (com.geckolib.constant.dataticket).
        #[derive(Clone, Debug)]
        pub struct BoneReturn {
        }
        impl BoneReturn {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BoneReturn.add(2): not translated, it uses `GeoRenderState` (com.geckolib.renderer.base).
        // BoneReturn.apply(2): not translated, it uses `RenderPassInfo` (com.geckolib.renderer.base).
        // BoneReturn.easeToBind(3): not translated, it uses `BoneSnapshot` (com.geckolib.animation.state).
        // BoneReturn.value(5): not translated, it uses `BoneSnapshot` (com.geckolib.animation.state).
    }

    /// `CorruptedEvoker` (com/lion/graveyard/entities/CorruptedEvoker.java)
    pub mod corrupted_evoker {
        use super::*;
        // The class itself isn't translated: it extends `CorruptedVindicator`. Its static parts are below.
        // CorruptedEvoker.CorruptedEvoker(2): not translated, its class isn't translated (it extends `CorruptedVindicator`).
    }

    /// `CorruptedIllager` (com/lion/graveyard/entities/CorruptedIllager.java)
    pub mod corrupted_illager {
        use super::*;
        // The class itself isn't translated: it extends `HordeGraveyardEntity`. Its static parts are below.
        pub const CHASE_END_TICKS: i32 = 20;
        // CHASING: not translated, it uses Minecraft's `EntityDataAccessor`.
        // CorruptedIllager.CorruptedIllager(3): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.defineSynchedData(1): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.registerGoals(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.getWanderSpeed(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.canAttack(1): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.customServerAiStep(1): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.trackChase(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.isChasing(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.setChasing(1): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.canLead(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.createCorruptedIllagerAttributes(0): not translated, it uses Minecraft's `Builder`.
        // CorruptedIllager.getState(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.finalizeSpawn(4): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.considersEntityAsAlly(1): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.isModelDamaged(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
        // CorruptedIllager.isAttackPending(0): not translated, its class isn't translated (it extends `HordeGraveyardEntity`).
    }

    /// `CorruptedIllager.AttackGoal` (com/lion/graveyard/entities/CorruptedIllager.java)
    pub mod corrupted_illager_attack_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MeleeAttackGoal`. Its static parts are below.
        // CorruptedIllager.AttackGoal.AttackGoal(1): not translated, its class isn't translated (it extends Minecraft's `MeleeAttackGoal`).
        // CorruptedIllager.AttackGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `MeleeAttackGoal`).
        // CorruptedIllager.AttackGoal.canPerformAttack(1): not translated, its class isn't translated (it extends Minecraft's `MeleeAttackGoal`).
    }

    /// `CorruptedIllager.State` (com/lion/graveyard/entities/CorruptedIllager.java)
    pub mod corrupted_illager_state {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum State {
            Crossed,
            Attacking,
            Spellcasting,
            BowAndArrow,
            CrossbowHold,
            CrossbowCharge,
            Celebrating,
            Undead,
            UndeadAttacking,
            Neutral,
        }
        impl State {
            pub const VALUES: [State; 10] = [State::Crossed, State::Attacking, State::Spellcasting, State::BowAndArrow, State::CrossbowHold, State::CrossbowCharge, State::Celebrating, State::Undead, State::UndeadAttacking, State::Neutral, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    State::Crossed => "CROSSED",
                    State::Attacking => "ATTACKING",
                    State::Spellcasting => "SPELLCASTING",
                    State::BowAndArrow => "BOW_AND_ARROW",
                    State::CrossbowHold => "CROSSBOW_HOLD",
                    State::CrossbowCharge => "CROSSBOW_CHARGE",
                    State::Celebrating => "CELEBRATING",
                    State::Undead => "UNDEAD",
                    State::UndeadAttacking => "UNDEAD_ATTACKING",
                    State::Neutral => "NEUTRAL",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "CROSSED" => State::Crossed,
                    "ATTACKING" => State::Attacking,
                    "SPELLCASTING" => State::Spellcasting,
                    "BOW_AND_ARROW" => State::BowAndArrow,
                    "CROSSBOW_HOLD" => State::CrossbowHold,
                    "CROSSBOW_CHARGE" => State::CrossbowCharge,
                    "CELEBRATING" => State::Celebrating,
                    "UNDEAD" => State::Undead,
                    "UNDEAD_ATTACKING" => State::UndeadAttacking,
                    "NEUTRAL" => State::Neutral,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `CorruptedPillager` (com/lion/graveyard/entities/CorruptedPillager.java)
    pub mod corrupted_pillager {
        use super::*;
        // The class itself isn't translated: it extends `AnimatedCorruptedIllager`. Its static parts are below.
        // CorruptedPillager.CorruptedPillager(2): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedPillager.createCorruptedPillagerAttributes(0): not translated, it uses Minecraft's `Builder`.
        // CorruptedPillager.hasSpawnAnimation(0): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedPillager.getState(0): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedPillager.playAmbientSound(0): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedPillager.playHurtSound(1): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedPillager.playStepSound(2): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
    }

    /// `CorruptedVindicator` (com/lion/graveyard/entities/CorruptedVindicator.java)
    pub mod corrupted_vindicator {
        use super::*;
        // The class itself isn't translated: it extends `AnimatedCorruptedIllager`. Its static parts are below.
        // CorruptedVindicator.CorruptedVindicator(2): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedVindicator.CorruptedVindicator(3): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedVindicator.hasSpawnAnimation(0): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedVindicator.isModelDamaged(0): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedVindicator.getState(0): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedVindicator.playAmbientSound(0): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedVindicator.playHurtSound(1): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
        // CorruptedVindicator.playStepSound(2): not translated, its class isn't translated (it extends `AnimatedCorruptedIllager`).
    }

    /// `CorvidEntity` (com/lion/graveyard/entities/CorvidEntity.java)
    pub mod corvid_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `PathfinderMob`. Its static parts are below.
        pub const WALK_MIN_SPEED_SQ: f64 = 0.0025f64;
        pub const GROUND_TURN_MIN_SPEED_SQ: f64 = 1.0e-4f64;
        pub const WALK_HOLD_TICKS: i32 = 3;
        // TAKEOFF: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // GLIDE: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // FLY: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // LAND: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // HOP: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // WATCH: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // IDLE: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // FLIGHT_STATE: not translated, it uses Minecraft's `EntityDataAccessor`.
        pub const GRAVESTONE_CHECK_INTERVAL: i32 = 20;
        pub const GRAVESTONE_WITNESS_RANGE: f64 = 4.0f64;
        pub const GLIDE_MIN: i32 = 6;
        pub const GLIDE_MAX: i32 = 70;
        pub const FLAP_BURST: i32 = 40;
        pub const HOME_RADIUS: i32 = 24;
        pub const DEATH_ANIMATION_DURATION: i32 = 37;
        pub const DEAD_FALL_LIMIT: i32 = 200;
        pub const IMPACT_REST_TICKS: i32 = 20;
        pub const FLAP_SOUND_INTERVAL: i32 = 36;
        pub const TAKEOFF_SOUND_COOLDOWN: i32 = 100;
        pub const FLAP_SOUND_VOLUME: f32 = 0.08f32;
        pub const TAKEOFF_SOUND_VOLUME: f32 = 0.25f32;
        pub const LAND_SOUND_VOLUME: f32 = 0.2f32;
        // CorvidEntity.CorvidEntity(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.createCorvidAttributes(0): not translated, it uses Minecraft's `Builder`.
        // CorvidEntity.getSpecies(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.tickHeadTurn(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getFlightState(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.finalizeSpawn(4): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getHome(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.setHome(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.restoreFrom(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isFarFromHome(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isPreparingTakeoff(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getPerchTag(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.registerGoals(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.createNavigation(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getFlyingSpeed(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.tick(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.aiStep(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.makeStuckInBlock(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.computeNearGround(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.takeOff(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.lift(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.playWingSound(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.tryPeck(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isPecking(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isOnFeedingGround(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isFlying(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isOnGravestone(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isPerched(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isNearGround(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isLanding(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isWatching(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isGliding(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isHopping(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isWalking(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.canWalk(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.canPeck(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.state(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isTouching(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isOnBlock(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.canSpawn(5): not translated, it uses Minecraft's `EntityType`.
        // CorvidEntity.checkSpawnRules(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getWalkTargetValue(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.causeFallDamage(3): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.checkFallDamage(4): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.playStepSound(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getAmbientSound(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getHurtSound(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getDeathSound(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getAmbientSoundInterval(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getSoundVolume(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.isPushable(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getMaxSpawnClusterSize(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.stepLookWeight(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.tickDeath(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.removeDeadBody(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.die(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.registerControllers(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // CorvidEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
    }

    /// `CorvidEntity.FlightState` (com/lion/graveyard/entities/CorvidEntity.java)
    pub mod corvid_entity_flight_state {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CorvidEntity.FlightState.FlightState(1): not translated, its class isn't translated (its constants carry values).
        // CorvidEntity.FlightState.byId(1): not translated, it uses FlightState, which isn't translated (its constants carry values).
    }

    /// `CorvidEntity.Species` (com/lion/graveyard/entities/CorvidEntity.java)
    pub mod corvid_entity_species {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CorvidEntity.Species.Species(2): not translated, its class isn't translated (its constants carry values).
        // CorvidEntity.Species.getTexture(0): not translated, its class isn't translated (its constants carry values).
        // CorvidEntity.Species.getWalkAnimation(0): not translated, its class isn't translated (its constants carry values).
        // CorvidEntity.Species.getPeckAnimation(0): not translated, its class isn't translated (its constants carry values).
        // CorvidEntity.Species.getDeathAnimation(0): not translated, its class isn't translated (its constants carry values).
        // CorvidEntity.Species.getModel(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `CrowEntity` (com/lion/graveyard/entities/CrowEntity.java)
    pub mod crow_entity {
        use super::*;
        // The class itself isn't translated: it extends `CorvidEntity`. Its static parts are below.
        pub const MAX_FLOCK: i32 = 12;
        pub const FLOCK_SEARCH_RADIUS: i32 = 12;
        pub const CLOSE_TO_LEADER_SQ: i32 = 144;
        pub const ALARM_TICKS: i32 = 120;
        pub const ALARM_SPREAD_RADIUS: i32 = 16;
        pub const ALARM_TICKS_PER_BLOCK: f32 = 0.5f32;
        pub const ANCHOR_INTERVAL: i32 = 100;
        // CrowEntity.CrowEntity(2): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.getSpecies(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.getPerchTag(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.getMaxSpawnClusterSize(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.registerGoals(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.tick(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.remove(1): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.hasLeader(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.getFlockHead(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.isSentinel(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.canAcceptMembers(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.isCloseToLeader(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.moveTowardLeader(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.findFlock(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.leaveFlock(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.getFlockSize(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.isAlarmed(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.raiseAlarm(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.isCircling(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.setCircling(1): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.getAnchor(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.updateAnchor(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.getAmbientSound(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.getAmbientSoundInterval(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.isGliding(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.setGliding(1): not translated, its class isn't translated (it extends `CorvidEntity`).
        // CrowEntity.canPeck(0): not translated, its class isn't translated (it extends `CorvidEntity`).
    }

    /// `FallingCorpse` (com/lion/graveyard/entities/FallingCorpse.java)
    pub mod falling_corpse {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Monster`. Its static parts are below.
        // IS_FALLING: not translated, it uses Minecraft's `EntityDataAccessor`.
        // HAS_COLLIDED: not translated, it uses Minecraft's `EntityDataAccessor`.
        // SINKING: not translated, it uses Minecraft's `EntityDataAccessor`.
        pub const LANDING_TICKS: i32 = 52;
        pub const SINK_START_TICKS: i32 = 26;
        pub const IMPACT_DELAY: i32 = 7;
        pub const LANDING_TRANSITION_TICKS: i32 = 2;
        // FallingCorpse.FallingCorpse(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.getRotation(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.causeFallDamage(3): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.hurtServer(3): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.resetFallDistance(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.registerControllers(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.createFallingCorpseAttributes(0): not translated, it uses Minecraft's `Builder`.
        // FallingCorpse.aiStep(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.customServerAiStep(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.impact(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.playerTouch(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.isFalling(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.setIsFalling(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.isSinking(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.setSinking(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.hasCollided(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // FallingCorpse.setHasCollided(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
    }

    /// `GhoulEntity` (com/lion/graveyard/entities/GhoulEntity.java)
    pub mod ghoul_entity {
        use super::*;
        // The class itself isn't translated: it extends `AngerableGraveyardEntity`. Its static parts are below.
        pub const EMERGE_WITNESS_INTERVAL: i32 = 10;
        pub const EMERGE_WITNESS_RANGE: f64 = 4.0f64;
        pub static SLOWNESS_ID: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("slowness_id"));
        // SLOWNESS_EFFECT: not translated, it uses Minecraft's `AttributeModifier`.
        // VARIANT: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ATTACK_ANIM_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // RAGE_ANIM_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ANIMATION: not translated, it uses Minecraft's `EntityDataAccessor`.
        // SPAWN_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // IS_RAGING: not translated, it uses Minecraft's `EntityDataAccessor`.
        // RUN_CRAWLING: not translated, it uses Minecraft's `EntityDataAccessor`.
        // NAVIGATING: not translated, it uses Minecraft's `EntityDataAccessor`.
        // RUNNING: not translated, it uses Minecraft's `EntityDataAccessor`.
        pub const ANIMATION_IDLE: i32 = 0;
        pub const ANIMATION_WALK: i32 = 1;
        pub const ANIMATION_RAGE: i32 = 2;
        pub const ANIMATION_DEATH: i32 = 3;
        pub const ANIMATION_RUNNING: i32 = 4;
        pub const ANIMATION_MELEE: i32 = 5;
        pub const ANIMATION_SPAWN: i32 = 6;
        pub const RUN_START_LIMB_SWING: f32 = 0.55f32;
        pub const RUN_STOP_LIMB_SWING: f32 = 0.3f32;
        pub const ATTACK_ANIM_SYNC_TICKS: i32 = 27;
        pub const SPAWN_ANIMATION_DURATION: i32 = 92;
        pub const CHASE_SOUND_COOLDOWN_TICKS: i32 = 200;
        pub const CHASE_END_TICKS: i32 = 20;
        pub static BIPED_SPEED_ID: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("biped_speed_id"));
        // BIPED_SPEED_BOOST: not translated, it uses Minecraft's `AttributeModifier`.
        pub const WALK_STEP_DISTANCE: f32 = 1.0f32;
        pub const CHASE_STEP_DISTANCE: f32 = 0.55f32;
        pub const DEATH_ANIMATION_DURATION: i32 = 49;
        // GhoulEntity.GhoulEntity(2): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.defineSynchedData(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.registerGoals(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.createGhoulAttributes(0): not translated, it uses Minecraft's `Builder`.
        // GhoulEntity.getAnimationState(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.setAnimationState(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.getAttackAnimTimer(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.setAttackAnimTimer(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.getSpawnTimer(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.setSpawnTimer(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.getRageAnimTimer(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.setRageAnimTimer(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.isRaging(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.setIsRaging(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.getVariant(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.setVariant(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.trackChase(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.customServerAiStep(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.aiStep(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.tick(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.restartAttackClipOnRearm(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.isImmobile(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.tickDeath(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.aggroMobs(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.registerControllers(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.isDying(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.isRunningAtSpeed(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.isRunning(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.playAmbientSound(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.nextStep(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.playStepSound(2): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.playHurtSound(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // GhoulEntity.die(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
    }

    /// `GhoulingBindings` (com/lion/graveyard/entities/GhoulingBindings.java)
    pub mod ghouling_bindings {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SavedData`. Its static parts are below.
        pub const LEGACY_DATA_FILE: &str = "graveyard_ghoulings.dat";
        pub const LEGACY_FILE: &str = "graveyardGhoulingUUIDmapping.txt";
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // TYPE: not translated, it uses Minecraft's `SavedDataType`.
        // GhoulingBindings.get(1): not translated, it uses GhoulingBindings, which isn't translated (it extends Minecraft's `SavedData`).
        // GhoulingBindings.get(1): not translated, it uses GhoulingBindings, which isn't translated (it extends Minecraft's `SavedData`).
        // GhoulingBindings.update(1): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // GhoulingBindings.remove(1): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // GhoulingBindings.toNbt(0): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // GhoulingBindings.fromNbt(1): not translated, it uses GhoulingBindings, which isn't translated (it extends Minecraft's `SavedData`).
        // GhoulingBindings.importLegacy(1): not translated, it uses GhoulingBindings, which isn't translated (it extends Minecraft's `SavedData`).
    }

    /// `GhoulingBindings.Binding` (com/lion/graveyard/entities/GhoulingBindings.java)
    pub mod ghouling_bindings_binding {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // GhoulingBindings.Binding.Binding(4): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // GhoulingBindings.Binding.owner(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // GhoulingBindings.Binding.dimension(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // GhoulingBindings.Binding.pos(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // GhoulingBindings.Binding.sitting(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
    }

    /// `GhoulingEntity` (com/lion/graveyard/entities/GhoulingEntity.java)
    pub mod ghouling_entity {
        use super::*;
        // The class itself isn't translated: it extends `GraveyardMinionEntity`. Its static parts are below.
        pub const BINDING_INTERVAL: i32 = 40;
        pub const SPAWN_ANIMATION_DURATION: i32 = 68;
        pub const PICKUP_INTERVAL: i32 = 5;
        pub const DEATH_ANIMATION_DURATION: i32 = 49;
        // ATTACK_ANIM_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ANIMATION: not translated, it uses Minecraft's `EntityDataAccessor`.
        // SPAWN_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // TELEPORT_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // STAFF: not translated, it uses Minecraft's `EntityDataAccessor`.
        // COFFIN: not translated, it uses Minecraft's `EntityDataAccessor`.
        // VARIANT: not translated, it uses Minecraft's `EntityDataAccessor`.
        pub const ANIMATION_SPAWN: i32 = 0;
        pub const ANIMATION_IDLE: i32 = 1;
        pub const ANIMATION_MELEE: i32 = 2;
        pub const ANIMATION_WALK: i32 = 3;
        pub const ANIMATION_DEATH: i32 = 4;
        // GhoulingEntity.GhoulingEntity(2): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.registerGoals(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.createGhoulingAttributes(0): not translated, it uses Minecraft's `Builder`.
        // GhoulingEntity.defineSynchedData(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.customServerAiStep(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.aiStep(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.tick(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.restartAttackClipOnRearm(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.mayTake(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.canPickUpPassingItems(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.getMaxFallDistance(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.isImmobile(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.storeInCoffin(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.collect(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.isCoffinFull(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.getAnimationState(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.setAnimationState(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.getAttackAnimTimer(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.setAttackAnimTimer(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.getTeleportTimer(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.setTeleportTimer(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.prepareSpawn(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.onSummoned(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.playSpawnSounds(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.removeWhenFarAway(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.registerControllers(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.mobInteract(2): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.toggleSitting(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.removeForStorage(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.tickDeath(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.getSpawnTimer(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.setSpawnTimer(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.getVariant(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.setVariant(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.getBondId(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.setBondId(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.acceptsStaff(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.getStaff(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.setStaff(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.hasCoffin(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.setHasCoffin(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.dropEquipment(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.ensureInventory(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.closeCoffinScreens(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.dropCustomDeathLoot(3): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.causeFallDamage(3): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.die(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.playAmbientSound(0): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.playStepSound(2): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.playHurtSound(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
        // GhoulingEntity.createMenu(3): not translated, its class isn't translated (it extends `GraveyardMinionEntity`).
    }

    /// `GhoulingEntity.CoffinInventory` (com/lion/graveyard/entities/GhoulingEntity.java)
    pub mod ghouling_entity_coffin_inventory {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleContainer`. Its static parts are below.
        // GhoulingEntity.CoffinInventory.CoffinInventory(2): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // GhoulingEntity.CoffinInventory.stillValid(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
    }

    /// `GhoulingEntity.GhoulingEscapeDangerGoal` (com/lion/graveyard/entities/GhoulingEntity.java)
    pub mod ghouling_entity_ghouling_escape_danger_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `PanicGoal`. Its static parts are below.
        // GhoulingEntity.GhoulingEscapeDangerGoal.GhoulingEscapeDangerGoal(1): not translated, its class isn't translated (it extends Minecraft's `PanicGoal`).
        // GhoulingEntity.GhoulingEscapeDangerGoal.shouldPanic(0): not translated, its class isn't translated (it extends Minecraft's `PanicGoal`).
    }

    /// `GraveyardMinionEntity` (com/lion/graveyard/entities/GraveyardMinionEntity.java)
    pub mod graveyard_minion_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `PathfinderMob`. Its static parts are below.
        // OWNER_UUID: not translated, it uses Minecraft's `EntityDataAccessor`.
        // TAMEABLE_FLAGS: not translated, it uses Minecraft's `EntityDataAccessor`.
        // GraveyardMinionEntity.GraveyardMinionEntity(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.getOwnerUuid(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.setOwnerUuid(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.setOwner(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.isSitting(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.setSitting(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.isInSittingPose(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.setInSittingPose(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.getOwner(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.canAttack(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.setTarget(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.sharesOwner(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.isOwner(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.considersEntityAsAlly(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.canAttackWithOwner(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // GraveyardMinionEntity.isPvpAllowed(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
    }

    /// `HordeGraveyardEntity` (com/lion/graveyard/entities/HordeGraveyardEntity.java)
    pub mod horde_graveyard_entity {
        use super::*;
        // The class itself isn't translated: it extends `HostileGraveyardEntity`. Its static parts are below.
        // HordeGraveyardEntity.HordeGraveyardEntity(3): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.registerGoals(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.canLead(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.finalizeSpawn(4): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.removeWhenFarAway(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.setPatrolTarget(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.setPatrolLeader(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.setHordeCaptain(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.isPatrolLeader(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.getPatrolTarget(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.hasPatrolTarget(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.setRandomPatrolTarget(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.setPatrolling(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.hasNoRaid(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.isRaidCenterSet(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // HordeGraveyardEntity.createWitherRose(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
    }

    /// `HordeGraveyardEntity.PatrolApproachGoal` (com/lion/graveyard/entities/HordeGraveyardEntity.java)
    pub mod horde_graveyard_entity_patrol_approach_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // HordeGraveyardEntity.PatrolApproachGoal.PatrolApproachGoal(2): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HordeGraveyardEntity.PatrolApproachGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HordeGraveyardEntity.PatrolApproachGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HordeGraveyardEntity.PatrolApproachGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HordeGraveyardEntity.PatrolApproachGoal.requiresUpdateEveryTick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // HordeGraveyardEntity.PatrolApproachGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `HordeGraveyardEntity.PatrolGoal` (com/lion/graveyard/entities/HordeGraveyardEntity.java)
    pub mod horde_graveyard_entity_patrol_goal {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // HordeGraveyardEntity.PatrolGoal.PatrolGoal(3): not translated, its class isn't translated (it's generic).
        // HordeGraveyardEntity.PatrolGoal.canUse(0): not translated, its class isn't translated (it's generic).
        // HordeGraveyardEntity.PatrolGoal.start(0): not translated, its class isn't translated (it's generic).
        // HordeGraveyardEntity.PatrolGoal.stop(0): not translated, its class isn't translated (it's generic).
        // HordeGraveyardEntity.PatrolGoal.tick(0): not translated, its class isn't translated (it's generic).
        // HordeGraveyardEntity.PatrolGoal.findPatrolTargets(0): not translated, its class isn't translated (it's generic).
        // HordeGraveyardEntity.PatrolGoal.wander(0): not translated, its class isn't translated (it's generic).
    }

    /// `GraveyardHordeSpawner` (com/lion/graveyard/entities/horde/GraveyardHordeSpawner.java)
    pub mod graveyard_horde_spawner {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomSpawner`. Its static parts are below.
        pub const CAPTAIN_SPOTS: i32 = 10;
        // GraveyardHordeSpawner.monstersAllowed(1): not translated, it uses Minecraft's `ServerLevel`.
        // GraveyardHordeSpawner.tick(2): not translated, its class isn't translated (it implements Minecraft's `CustomSpawner`).
        // GraveyardHordeSpawner.spawnFollowers(5): not translated, its class isn't translated (it implements Minecraft's `CustomSpawner`).
        // GraveyardHordeSpawner.spawnHordeEntity(5): not translated, its class isn't translated (it implements Minecraft's `CustomSpawner`).
    }

    /// `HostileGraveyardEntity` (com/lion/graveyard/entities/HostileGraveyardEntity.java)
    pub mod hostile_graveyard_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Monster`. Its static parts are below.
        // CAN_BURN_IN_SUNLIGHT: not translated, it uses Minecraft's `EntityDataAccessor`.
        // HostileGraveyardEntity.HostileGraveyardEntity(3): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // HostileGraveyardEntity.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // HostileGraveyardEntity.isSunBurnTick(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // HostileGraveyardEntity.burnsInDaylight(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // HostileGraveyardEntity.canBurnInSunlight(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // HostileGraveyardEntity.setCanBurnInSunlight(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // HostileGraveyardEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // HostileGraveyardEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // HostileGraveyardEntity.canBeAffected(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // HostileGraveyardEntity.aiStep(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // HostileGraveyardEntity.canSpawnInDarkness(5): not translated, it uses Minecraft's `EntityType`.
        // HostileGraveyardEntity.canSpawnInLight(5): not translated, it uses Minecraft's `EntityType`.
    }

    /// `LegacyEntityData` (com/lion/graveyard/entities/LegacyEntityData.java)
    pub mod legacy_entity_data {
        use super::*;
        // HAND_SLOTS: not translated, it uses Minecraft's `EquipmentSlot`.
        // ARMOR_SLOTS: not translated, it uses Minecraft's `EquipmentSlot`.
        // MODIFIER_IDS_BY_UUID: not translated, it uses `UUID` (java.util).
        pub static MODIFIER_IDS_BY_NAME: LazyLock<JMap<String, String>> = LazyLock::new(|| JMap::from_iter([("Random spawn bonus".to_string(), "minecraft:random_spawn_bonus".to_string()), ("Random zombie-spawn bonus".to_string(), "minecraft:zombie_random_spawn_bonus".to_string()), ("Leader zombie bonus".to_string(), "minecraft:leader_zombie_bonus".to_string()), ("Zombie reinforcement callee charge".to_string(), "minecraft:reinforcement_callee_charge".to_string()), ("Zombie reinforcement caller charge".to_string(), "minecraft:reinforcement_caller_charge".to_string())]));
        pub static ATTRIBUTE_PREFIXES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["generic.".to_string(), "horse.".to_string(), "player.".to_string(), "zombie.".to_string()]);
        pub static OPERATIONS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["add_value".to_string(), "add_multiplied_base".to_string(), "add_multiplied_total".to_string()]);
        pub static EFFECT_IDS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["speed".to_string(), "slowness".to_string(), "haste".to_string(), "mining_fatigue".to_string(), "strength".to_string(), "instant_health".to_string(), "instant_damage".to_string(), "jump_boost".to_string(), "nausea".to_string(), "regeneration".to_string(), "resistance".to_string(), "fire_resistance".to_string(), "water_breathing".to_string(), "invisibility".to_string(), "blindness".to_string(), "night_vision".to_string(), "hunger".to_string(), "weakness".to_string(), "poison".to_string(), "wither".to_string(), "health_boost".to_string(), "absorption".to_string(), "saturation".to_string(), "glowing".to_string(), "levitation".to_string(), "luck".to_string(), "unluck".to_string(), "slow_falling".to_string(), "conduit_power".to_string(), "dolphins_grace".to_string(), "bad_omen".to_string(), "hero_of_the_village".to_string(), "darkness".to_string()]);
        pub static EFFECT_RENAMES: LazyLock<JMap<String, String>> = LazyLock::new(|| JMap::from_iter([("Amplifier".to_string(), "amplifier".to_string()), ("Duration".to_string(), "duration".to_string()), ("Ambient".to_string(), "ambient".to_string()), ("ShowParticles".to_string(), "show_particles".to_string()), ("ShowIcon".to_string(), "show_icon".to_string())]));
        #[derive(Clone, Debug)]
        pub struct LegacyEntityData {
        }
        impl LegacyEntityData {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LegacyEntityData.write(1): not translated, it uses Minecraft's `ValueOutput`.
        // LegacyEntityData.read(2): not translated, it uses Minecraft's `ValueInput`.
        // LegacyEntityData.readLegacyAngerTarget(3): not translated, it's generic.
        // LegacyEntityData.storedCustomName(2): not translated, it uses Minecraft's `Component`.
        // LegacyEntityData.parseJson(2): not translated, it uses Minecraft's `Component`.
        // LegacyEntityData.readLeash(2): not translated, it uses Minecraft's `ValueInput`.
        // LegacyEntityData.convertLeash(1): not translated, it uses Minecraft's `Tag`.
        // LegacyEntityData.readCustomName(2): not translated, it uses Minecraft's `ValueInput`.
        // LegacyEntityData.readEquipment(2): not translated, it uses Minecraft's `ValueInput`.
        // LegacyEntityData.applyStacks(5): not translated, it uses Minecraft's `ValueInput`.
        // LegacyEntityData.readDropChances(4): not translated, it uses Minecraft's `ValueInput`.
        // LegacyEntityData.modifier(2): not translated, it uses `UUID` (java.util).
        // LegacyEntityData.readAttributes(2): not translated, it uses Minecraft's `ValueInput`.
        // LegacyEntityData.applyAttributes(3): not translated, it uses Minecraft's `ValueInput`.
        // LegacyEntityData.withUnprefixedIds(1): not translated, it uses Minecraft's `ListTag`.
        // LegacyEntityData.convertAttributes(1): not translated, it uses Minecraft's `ListTag`.
        pub fn attribute_id(name: &str) -> String {
            let mut id: String = legacy_entity_data::ensure_namespaced(&name);
            if j_eq("minecraft:horse.jump_strength", &id) {
                id = "minecraft:generic.jump_strength".to_string();
            }
            for prefix in (*legacy_entity_data::ATTRIBUTE_PREFIXES).iter() {
                let namespaced: String = format!("minecraft:{}", prefix);
                if id.starts_with(&namespaced) {
                    return format!("minecraft:{}", j_substring(&id, j_len(&namespaced), None));
                }
            }
            return id.to_string();
        }
        pub fn ensure_namespaced(id: &str) -> String {
            return if (j_index_of_char(&id, char::from_u32(58 as u32).unwrap_or('\0'))) < 0 { format!("minecraft:{}", id) } else { id.to_string() };
        }
        // LegacyEntityData.convertModifiers(1): not translated, it uses Minecraft's `ListTag`.
        pub fn operation(legacy: i32) -> String {
            return if (legacy >= 0) && (legacy < ((*legacy_entity_data::OPERATIONS).len() as i32)) { ((*legacy_entity_data::OPERATIONS)[legacy as usize]).to_string() } else { "invalid".to_string() };
        }
        // LegacyEntityData.readUuid(1): not translated, it uses `UUID` (java.util).
        // LegacyEntityData.readEffects(2): not translated, it uses Minecraft's `ValueInput`.
        // LegacyEntityData.convertEffect(1): not translated, it uses Minecraft's `CompoundTag`.
    }

    /// `LichEntity` (com/lion/graveyard/entities/LichEntity.java)
    pub mod lich_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Monster`. Its static parts are below.
        // HEAD_TARGET_PREDICATE: not translated, it uses Minecraft's `TargetingConditions`.
        // CAN_ATTACK_PREDICATE: not translated, it uses Minecraft's `Selector`.
        pub static ATTACKING_SPEED_BOOST_ID: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("attacking_speed_boost_id"));
        pub static ATTACKING_DMG_BOOST_ID: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("attacking_dmg_boost_id"));
        pub static CRAWL_SPEED_BOOST_ID: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("crawl_speed_boost_id"));
        // ATTACKING_SPEED_BOOST: not translated, it uses Minecraft's `AttributeModifier`.
        // CRAWL_SPEED_BOOST: not translated, it uses Minecraft's `AttributeModifier`.
        // DMG_BOOST: not translated, it uses Minecraft's `AttributeModifier`.
        pub const ANIMATION_SPAWN: i32 = 0;
        pub const ANIMATION_IDLE: i32 = 1;
        pub const ANIMATION_MELEE: i32 = 2;
        pub const ANIMATION_CORPSE_SPELL: i32 = 3;
        pub const ANIMATION_START_PHASE_2: i32 = 4;
        pub const ANIMATION_PHASE_2_IDLE: i32 = 5;
        pub const ANIMATION_PHASE_2_ATTACK: i32 = 6;
        pub const ANIMATION_SHOOT_SKULL: i32 = 7;
        pub const ANIMATION_START_PHASE_3: i32 = 8;
        pub const ANIMATION_PHASE_3_ATTACK: i32 = 9;
        pub const ANIMATION_STOP: i32 = 10;
        pub const ANIMATION_SUMMON: i32 = 11;
        pub const ANIMATION_CONJURE_FANG: i32 = 12;
        pub const ANIMATION_STUNNED: i32 = 13;
        pub const ANIMATION_CRAWL_IDLE: i32 = 14;
        pub const ANIMATION_PHASE_2_CHARGE: i32 = 15;
        pub const ANIMATION_CRAWL_POUNCE: i32 = 16;
        pub const ANIMATION_CRAWL_ATTACK: i32 = 17;
        // INVUL_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // SPAWN_DURATION: not translated, it uses Minecraft's `EntityDataAccessor`.
        // PHASE_INVUL_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ATTACK_ANIM_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // FIGHT_DURATION_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // PHASE_TWO_START_ANIM_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // PHASE_THREE_START_ANIM_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // PHASE: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ANIMATION: not translated, it uses Minecraft's `EntityDataAccessor`.
        // HUNT_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // CAN_HUNT_START: not translated, it uses Minecraft's `EntityDataAccessor`.
        // CAN_MOVE: not translated, it uses Minecraft's `EntityDataAccessor`.
        // MUSIC_DELAY: not translated, it uses Minecraft's `EntityDataAccessor`.
        // CONJURE_FANG_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // HEAL_DURATION_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // CORPSE_SPELL_DURATION_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // LEVITATION_DURATION_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        pub const MUSIC_PLAY_ID: i8 = 67i8;
        pub const MUSIC_STOP_ID: i8 = 68i8;
        pub const SPAWN_CLIP_TICKS: i32 = 490;
        // CRAWL_DIMENSIONS: not translated, it uses Minecraft's `EntityDimensions`.
        pub const DEATH_REMOVAL_TICKS: i32 = 165;
        pub static SUMMON_OFFSETS: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![0, 0], vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]]);
        pub const HELIX_POINT_SPACING: i32 = 4;
        pub const HELIX_SWIRL_SPEED: f64 = 0.08f64;
        pub const HELIX_FLAME_RISE: f64 = 0.03f64;
        pub const HELIX_SOUL_RISE: f64 = 0.06f64;
        pub const HELIX_POINTS: i32 = 40;
        pub const HELIX_SPIN_PER_TICK: f64 = 0.12f64;
        pub const HELIX_TURNS: f64 = 3.0f64;
        pub const HELIX_RADIUS: f64 = 2.4f64;
        pub const HELIX_HEIGHT: f64 = 6.0f64;
        pub const CRAWL_STRIKE_TICKS: i32 = 23;
        pub const CRAWL_STRIKE_HIT_TICK: i32 = 13;
        pub const PHASE_TWO_STRIKE_TICKS: i32 = 40;
        pub const PHASE_TWO_STRIKE_HIT_TICK: i32 = 19;
        pub const POUNCE_AIR_FACTOR: f64 = 3.93f64;
        pub const CHARGE_TICKS: i32 = 53;
        pub const CHARGE_LUNGE_TICK: i32 = 15;
        pub const CHARGE_LUNGE_END_TICK: i32 = 31;
        pub const CHARGE_HIT_TICK: i32 = 33;
        pub const POUNCE_TICKS: i32 = 53;
        pub const POUNCE_LEAP_TICK: i32 = 15;
        // LichEntity.champion(0): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // LichEntity.spawnInvulTicks(0): not translated, it calls LichEntity.champion, which isn't translated.
        // LichEntity.phaseInvulTicks(0): not translated, it calls LichEntity.champion, which isn't translated.
        // LichEntity.LichEntity(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.registerControllers(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.blendTicks(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.isSpawning(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.registerGoals(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.createLichAttributes(0): not translated, it uses Minecraft's `Builder`.
        // LichEntity.tick(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.restartAttackClipOnRearm(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.handleEntityEvent(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.aiStep(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.spawnHealHelix(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.spawnTransitionAndDeathParticles(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.spawnInHitbox(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.spawnOutwardBurst(3): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.baseTick(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.customServerAiStep(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        pub fn is_bone_break_tick(elapsed: i32) -> bool {
            return (((((elapsed == 34) || (elapsed == 44)) || (elapsed == 53)) || (elapsed == 70)) || (elapsed == 132)) || (elapsed == 144);
        }
        // LichEntity.enterStunned(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.tickCharge(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.startCrawlStrike(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.tickCrawlStrike(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.isCharging(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.startPhaseTwoStrike(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.tickPhaseTwoStrike(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.stopRush(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.endCharge(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.tickPounce(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.isPouncing(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getMaxHealthPerPhase(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setCustomName(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.releaseAltar(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.remove(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getHomePos(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.isHomeKnown(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.onSummoned(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.applyInvulAndResetBossBar(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.tickDeath(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getDefaultDimensions(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.causeFallDamage(3): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.addEffect(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.canRide(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.canTeleport(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.startSeenByPlayer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.stopSeenByPlayer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.hurtServer(3): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.respawn(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.randomDisplayTick(3): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getPlayersInRange(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.canMeeleAttack(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.canSpellCast(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.ignoreExplosion(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.isSummonCell(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.summonCell(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.summonMob(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.requiresCustomPersistence(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.canHuntStart(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setHuntStart(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setCanMove(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.canMove(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getInvulnerableTimer(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setInvulTimer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getFightDurationTimer(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setFightDurationTimer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getLevitationDurationTimer(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setLevitationDurationTimer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getConjureFangTimer(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setConjureFangTimer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getSpawnDuration(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getCorpseSpellTimer(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setCorpseSpellTimer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getHealTimer(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setHealTimer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getPhaseInvulnerableTimer(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setPhaseInvulTimer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getHuntTimer(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setHuntTimer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getAnimationState(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setAnimationState(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getPhase(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setPhase(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getStartPhaseTwoAnimTimer(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setStartPhaseTwoAnimTimer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getStartPhaseThreeAnimTimer(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setStartPhaseThreeAnimTimer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getAttackAnimTimer(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setAttackAnimTimer(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getHuntCooldownTicker(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setHuntCooldownTicker(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getMusicDelay(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.setMusicDelay(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.openFight(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.isAtHome(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.awakenBurst(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playSpawnSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playMeleeSwingSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playHealSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playCorpseSpellSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playStartPhaseTwoSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playStartPhaseThreeSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playStartPhaseThreeAttackSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playDeathSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playScareSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playHuntSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playShootSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playLevitationSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playAmbientSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.playHurtSound(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getHurtSound(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.getBossMusic(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.canPlayMusic(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.canPlayerHearMusic(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // LichEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
    }

    /// `LichEntity.ConjureFangsGoal` (com/lion/graveyard/entities/LichEntity.java)
    pub mod lich_entity_conjure_fangs_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // LichEntity.ConjureFangsGoal.ConjureFangsGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ConjureFangsGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ConjureFangsGoal.requiresUpdateEveryTick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ConjureFangsGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ConjureFangsGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ConjureFangsGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ConjureFangsGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ConjureFangsGoal.conjureFangs(6): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `LichEntity.LevitationGoal` (com/lion/graveyard/entities/LichEntity.java)
    pub mod lich_entity_levitation_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // LichEntity.LevitationGoal.LevitationGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.LevitationGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.LevitationGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.LevitationGoal.requiresUpdateEveryTick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.LevitationGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.LevitationGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.LevitationGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `LichEntity.ShootSkullGoal` (com/lion/graveyard/entities/LichEntity.java)
    pub mod lich_entity_shoot_skull_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // LichEntity.ShootSkullGoal.ShootSkullGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ShootSkullGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ShootSkullGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ShootSkullGoal.requiresUpdateEveryTick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ShootSkullGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.ShootSkullGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `LichEntity.SummonFallenCorpsesGoal` (com/lion/graveyard/entities/LichEntity.java)
    pub mod lich_entity_summon_fallen_corpses_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // LichEntity.SummonFallenCorpsesGoal.SummonFallenCorpsesGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.SummonFallenCorpsesGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.SummonFallenCorpsesGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.SummonFallenCorpsesGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.SummonFallenCorpsesGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.SummonFallenCorpsesGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `LichEntity.TeleportAndHealGoal` (com/lion/graveyard/entities/LichEntity.java)
    pub mod lich_entity_teleport_and_heal_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // LichEntity.TeleportAndHealGoal.TeleportAndHealGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.TeleportAndHealGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.TeleportAndHealGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.TeleportAndHealGoal.requiresUpdateEveryTick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.TeleportAndHealGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.TeleportAndHealGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // LichEntity.TeleportAndHealGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `MagpieEntity` (com/lion/graveyard/entities/MagpieEntity.java)
    pub mod magpie_entity {
        use super::*;
        // The class itself isn't translated: it extends `CorvidEntity`. Its static parts are below.
        // MagpieEntity.MagpieEntity(2): not translated, its class isn't translated (it extends `CorvidEntity`).
        // MagpieEntity.getSpecies(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // MagpieEntity.registerGoals(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // MagpieEntity.isHopping(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // MagpieEntity.setHopping(1): not translated, its class isn't translated (it extends `CorvidEntity`).
        // MagpieEntity.canWalk(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // MagpieEntity.isWalking(0): not translated, its class isn't translated (it extends `CorvidEntity`).
        // MagpieEntity.setWalking(1): not translated, its class isn't translated (it extends `CorvidEntity`).
        // MagpieEntity.wantsToPickUp(2): not translated, its class isn't translated (it extends `CorvidEntity`).
        // MagpieEntity.pickUpItem(2): not translated, its class isn't translated (it extends `CorvidEntity`).
        // MagpieEntity.hurtServer(3): not translated, its class isn't translated (it extends `CorvidEntity`).
    }

    /// `MobSpawnGate` (com/lion/graveyard/entities/MobSpawnGate.java)
    pub mod mob_spawn_gate {
        use super::*;
        pub static MANAGED_MOBS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["ghoul".to_string(), "revenant".to_string(), "nightmare".to_string(), "skeleton_creeper".to_string(), "acolyte".to_string(), "reaper".to_string(), "corrupted_vindicator".to_string(), "corrupted_pillager".to_string(), "corrupted_evoker".to_string(), "wraith".to_string(), "raven".to_string(), "magpie".to_string(), "crow".to_string(), "tormented_villager".to_string(), "nameless_hanged".to_string()]);
        // AUTOMATIC: not translated, it uses Minecraft's `EntitySpawnReason`.
        // NO_MOB_SPAWNS: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct MobSpawnGate {
        }
        impl MobSpawnGate {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MobSpawnGate.inForbiddenBiome(4): not translated, it uses Minecraft's `LevelAccessor`.
        // MobSpawnGate.blocks(2): not translated, it uses Minecraft's `EntityType`.
        // MobSpawnGate.isSwitchedOff(1): not translated, it uses Minecraft's `EntityType`.
    }

    /// `BaseGhoulModel` (com/lion/graveyard/entities/models/BaseGhoulModel.java)
    pub mod base_ghoul_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // BaseGhoulModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // BaseGhoulModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // BaseGhoulModel.addAdditionalStateData(3): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // BaseGhoulModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `CorruptedIllagerGeoModel` (com/lion/graveyard/entities/models/CorruptedIllagerGeoModel.java)
    pub mod corrupted_illager_geo_model {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CorruptedIllagerGeoModel.getModelResource(1): not translated, its class isn't translated (it's generic).
        // CorruptedIllagerGeoModel.getTextureResource(1): not translated, its class isn't translated (it's generic).
        // CorruptedIllagerGeoModel.addAdditionalStateData(3): not translated, its class isn't translated (it's generic).
        // CorruptedIllagerGeoModel.getAnimationResource(1): not translated, its class isn't translated (it's generic).
    }

    /// `CorruptedIllagerModel` (com/lion/graveyard/entities/models/CorruptedIllagerModel.java)
    pub mod corrupted_illager_model {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityModel`. Its static parts are below.
        // CorruptedIllagerModel.CorruptedIllagerModel(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // CorruptedIllagerModel.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // CorruptedIllagerModel.setupAnim(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // CorruptedIllagerModel.getAttackingArm(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // CorruptedIllagerModel.getHat(0): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // CorruptedIllagerModel.getHead(0): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // CorruptedIllagerModel.translateToHand(3): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
    }

    /// `CorvidModel` (com/lion/graveyard/entities/models/CorvidModel.java)
    pub mod corvid_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        pub static ANIMATION: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("corvid/corvid"));
        // CorvidModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // CorvidModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // CorvidModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // CorvidModel.addAdditionalStateData(3): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // CorvidModel.isPlayingPeck(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `FallingCorpseModel` (com/lion/graveyard/entities/models/FallingCorpseModel.java)
    pub mod falling_corpse_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // FallingCorpseModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // FallingCorpseModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // FallingCorpseModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `GhoulingModel` (com/lion/graveyard/entities/models/GhoulingModel.java)
    pub mod ghouling_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // GhoulingModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // GhoulingModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // GhoulingModel.addAdditionalStateData(3): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        pub fn texture_variant(variant: i8) -> i32 {
            return if ((variant as i32) >= 1) && ((variant as i32) <= 5) { (variant as i32) } else { 1 };
        }
        // GhoulingModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `LichModel` (com/lion/graveyard/entities/models/LichModel.java)
    pub mod lich_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // LichModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // LichModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // LichModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // LichModel.addAdditionalStateData(3): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `NamelessHangedModel` (com/lion/graveyard/entities/models/NamelessHangedModel.java)
    pub mod nameless_hanged_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // NamelessHangedModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // NamelessHangedModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // NamelessHangedModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `NightmareModel` (com/lion/graveyard/entities/models/NightmareModel.java)
    pub mod nightmare_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // NightmareModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // NightmareModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // NightmareModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `ReaperModel` (com/lion/graveyard/entities/models/ReaperModel.java)
    pub mod reaper_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // ReaperModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // ReaperModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // ReaperModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `RevenantModel` (com/lion/graveyard/entities/models/RevenantModel.java)
    pub mod revenant_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // RevenantModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // RevenantModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // RevenantModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `TormentedVillagerModel` (com/lion/graveyard/entities/models/TormentedVillagerModel.java)
    pub mod tormented_villager_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // TormentedVillagerModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // TormentedVillagerModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // TormentedVillagerModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `WraithModel` (com/lion/graveyard/entities/models/WraithModel.java)
    pub mod wraith_model {
        use super::*;
        // The class itself isn't translated: it extends `GeoModel` (com.geckolib.model). Its static parts are below.
        // WraithModel.getModelResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // WraithModel.getTextureResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
        // WraithModel.getAnimationResource(1): not translated, its class isn't translated (it extends `GeoModel` (com.geckolib.model)).
    }

    /// `NamelessHangedEntity` (com/lion/graveyard/entities/NamelessHangedEntity.java)
    pub mod nameless_hanged_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractVillager`. Its static parts are below.
        pub const OFFERS_DAY_KEY: &str = "OffersDay";
        // NamelessHangedEntity.NamelessHangedEntity(2): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.registerGoals(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.createNamelessHangedAttributes(0): not translated, it uses Minecraft's `Builder`.
        // NamelessHangedEntity.registerControllers(1): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.canBeSeenAsEnemy(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.isPushable(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.doPush(1): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.hurtServer(3): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.ignoreExplosion(1): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.thunderHit(2): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.isAffectedByPotions(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.getBreedOffspring(2): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.showProgressBar(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.aiStep(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.tick(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.currentDay(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.mobInteract(2): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.updateTrades(1): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.minOffers(0): not translated, it calls Graveyard.getConfig, which isn't translated.
        // NamelessHangedEntity.maxOffers(0): not translated, it calls NamelessHangedEntity.minOffers, which isn't translated.
        // NamelessHangedEntity.villagertradingplus$rerollAll(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.villagertradingplus$catalogPool(1): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.villagertradingplus$catalogMaxLevel(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.villagertradingplus$catalogPicks(1): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.rewardTradeXp(1): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.removeWhenFarAway(1): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.playAmbientSound(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.getNotifyTradeSound(0): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
        // NamelessHangedEntity.getTradeUpdatedSound(1): not translated, its class isn't translated (it extends Minecraft's `AbstractVillager`).
    }

    /// `NightmareEntity` (com/lion/graveyard/entities/NightmareEntity.java)
    pub mod nightmare_entity {
        use super::*;
        // The class itself isn't translated: it extends `HostileGraveyardEntity`. Its static parts are below.
        pub const ANIMATION_IDLE: i8 = 0i8;
        pub const ANIMATION_WALK: i8 = 1i8;
        pub const ANIMATION_DEATH: i8 = 3i8;
        pub const ANIMATION_ATTACK: i8 = 5i8;
        // ANIMATION: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ATTACK_ANIM_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ANGRY: not translated, it uses Minecraft's `EntityDataAccessor`.
        // PROVOKED: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ANGER_TIME: not translated, it uses Minecraft's `UniformInt`.
        // NightmareEntity.NightmareEntity(2): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.defineSynchedData(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.getAnimationState(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.setState(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.registerGoals(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.tick(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.restartAttackClipOnRearm(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.customServerAiStep(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.aiStep(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.readPersistentAngerSaveData(2): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.createNightmareAttributes(0): not translated, it uses Minecraft's `Builder`.
        // NightmareEntity.tickDeath(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.hurtServer(3): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.getAnimation(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.setAnimation(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.getAttackAnimTimer(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.setAttackAnimTimer(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.registerControllers(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.playAmbientSound(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.playHurtSound(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.die(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.isPlayerStaring(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.teleportTo(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.teleportToPosition(3): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.teleportRandomly(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.isAngry(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.setPersistentAngerEndTime(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.getPersistentAngerEndTime(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.setProvoked(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.getPersistentAngerTarget(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.setPersistentAngerTarget(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.startPersistentAngerTimer(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.setTarget(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // NightmareEntity.createWitherRose(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
    }

    /// `NightmareEntity.ChasePlayerGoal` (com/lion/graveyard/entities/NightmareEntity.java)
    pub mod nightmare_entity_chase_player_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // NightmareEntity.ChasePlayerGoal.ChasePlayerGoal(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // NightmareEntity.ChasePlayerGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // NightmareEntity.ChasePlayerGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // NightmareEntity.ChasePlayerGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `NightmareEntity.TeleportTowardsPlayerGoal` (com/lion/graveyard/entities/NightmareEntity.java)
    pub mod nightmare_entity_teleport_towards_player_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `NearestAttackableTargetGoal`. Its static parts are below.
        // NightmareEntity.TeleportTowardsPlayerGoal.TeleportTowardsPlayerGoal(2): not translated, its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`).
        // NightmareEntity.TeleportTowardsPlayerGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`).
        // NightmareEntity.TeleportTowardsPlayerGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`).
        // NightmareEntity.TeleportTowardsPlayerGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`).
        // NightmareEntity.TeleportTowardsPlayerGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`).
        // NightmareEntity.TeleportTowardsPlayerGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`).
    }

    /// `SkullEntity` (com/lion/graveyard/entities/projectiles/SkullEntity.java)
    pub mod skull_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractHurtingProjectile`. Its static parts are below.
        // CHARGED: not translated, it uses Minecraft's `EntityDataAccessor`.
        pub const INITIAL_SPEED: f64 = 0.001f64;
        // SkullEntity.SkullEntity(2): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.SkullEntity(5): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.getInertia(0): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.getLightLevelDependentMagicValue(0): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.isOnFire(0): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.getBlockExplosionResistance(6): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.onHitEntity(1): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.tick(0): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.onHit(1): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.isPickable(0): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.hurtServer(3): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.isCharged(0): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.setCharged(1): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
        // SkullEntity.shouldBurn(0): not translated, its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`).
    }

    /// `ReaperEntity` (com/lion/graveyard/entities/ReaperEntity.java)
    pub mod reaper_entity {
        use super::*;
        // The class itself isn't translated: it extends `HostileGraveyardEntity`. Its static parts are below.
        // ANIMATION: not translated, it uses Minecraft's `EntityDataAccessor`.
        // VEX_FLAGS: not translated, it uses Minecraft's `EntityDataAccessor`.
        pub const CHARGING_FLAG: i32 = 1;
        pub const SPAWN_TICKS: i32 = 14;
        pub const ATTACK_TICKS: i32 = 31;
        pub const DEATH_TICKS: i32 = 25;
        // SPAWN_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ATTACK_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ReaperEntity.ReaperEntity(2): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.defineSynchedData(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.registerGoals(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.tick(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.finalizeSpawn(4): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.tickDown(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.isSpawning(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.doHurtTarget(2): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.tickDeath(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.getBounds(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.areFlagsSet(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.setVexFlag(2): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.isCharging(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.setCharging(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.createReaperAttributes(0): not translated, it uses Minecraft's `Builder`.
        // ReaperEntity.getAnimation(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.setAnimation(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.registerControllers(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.playAmbientSound(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.playHurtSound(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // ReaperEntity.die(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
    }

    /// `ReaperEntity.ChargeTargetGoal` (com/lion/graveyard/entities/ReaperEntity.java)
    pub mod reaper_entity_charge_target_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // ReaperEntity.ChargeTargetGoal.ChargeTargetGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // ReaperEntity.ChargeTargetGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // ReaperEntity.ChargeTargetGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // ReaperEntity.ChargeTargetGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // ReaperEntity.ChargeTargetGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // ReaperEntity.ChargeTargetGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `ReaperEntity.LookAtTargetGoal` (com/lion/graveyard/entities/ReaperEntity.java)
    pub mod reaper_entity_look_at_target_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // ReaperEntity.LookAtTargetGoal.LookAtTargetGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // ReaperEntity.LookAtTargetGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // ReaperEntity.LookAtTargetGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // ReaperEntity.LookAtTargetGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `ReaperEntity.ReaperMoveControl` (com/lion/graveyard/entities/ReaperEntity.java)
    pub mod reaper_entity_reaper_move_control {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MoveControl`. Its static parts are below.
        // ReaperEntity.ReaperMoveControl.ReaperMoveControl(1): not translated, its class isn't translated (it extends Minecraft's `MoveControl`).
        // ReaperEntity.ReaperMoveControl.tick(0): not translated, its class isn't translated (it extends Minecraft's `MoveControl`).
    }

    /// `ReaperEntity.TrackOwnerTargetGoal` (com/lion/graveyard/entities/ReaperEntity.java)
    pub mod reaper_entity_track_owner_target_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `TargetGoal`. Its static parts are below.
        // ReaperEntity.TrackOwnerTargetGoal.TrackOwnerTargetGoal(1): not translated, its class isn't translated (it extends Minecraft's `TargetGoal`).
        // ReaperEntity.TrackOwnerTargetGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `TargetGoal`).
        // ReaperEntity.TrackOwnerTargetGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `TargetGoal`).
    }

    /// `AcolyteRender` (com/lion/graveyard/entities/renders/AcolyteRender.java)
    pub mod acolyte_render {
        use super::*;
        // The class itself isn't translated: it extends `CorruptedIllagerRenderer`. Its static parts are below.
        pub static TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/acolyte.png"));
        // AcolyteRender.AcolyteRender(1): not translated, its body couldn't be read: an anonymous class.
        // AcolyteRender.getTextureLocation(1): not translated, its class isn't translated (it extends `CorruptedIllagerRenderer`).
    }

    /// `CorruptedIllagerGeoRenderer` (com/lion/graveyard/entities/renders/CorruptedIllagerGeoRenderer.java)
    pub mod corrupted_illager_geo_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CorruptedIllagerGeoRenderer.CorruptedIllagerGeoRenderer(1): not translated, its class isn't translated (it's generic).
        // CorruptedIllagerGeoRenderer.getDeathMaxRotation(1): not translated, its class isn't translated (it's generic).
        // CorruptedIllagerGeoRenderer.applyAnimationControllers(2): not translated, its class isn't translated (it's generic).
        // CorruptedIllagerGeoRenderer.addRenderData(4): not translated, its class isn't translated (it's generic).
        // CorruptedIllagerGeoRenderer.adjustModelBonesForRender(2): not translated, its class isn't translated (it's generic).
    }

    /// `CorruptedIllagerRenderer` (com/lion/graveyard/entities/renders/CorruptedIllagerRenderer.java)
    pub mod corrupted_illager_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CorruptedIllagerRenderer.CorruptedIllagerRenderer(3): not translated, its class isn't translated (it's generic).
        // CorruptedIllagerRenderer.createRenderState(0): not translated, its class isn't translated (it's generic).
        // CorruptedIllagerRenderer.extractRenderState(3): not translated, its class isn't translated (it's generic).
        // CorruptedIllagerRenderer.scale(2): not translated, its class isn't translated (it's generic).
    }

    /// `CorruptedIllagerRenderState` (com/lion/graveyard/entities/renders/CorruptedIllagerRenderState.java)
    pub mod corrupted_illager_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `IllagerRenderState`. Its static parts are below.
    }

    /// `CorvidRenderer` (com/lion/graveyard/entities/renders/CorvidRenderer.java)
    pub mod corvid_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const MAX_HEAD_PITCH: f32 = 40.0f32;
        pub const MAX_HEAD_YAW: f32 = 75.0f32;
        // CorvidRenderer.CorvidRenderer(1): not translated, its class isn't translated (it's generic).
        // CorvidRenderer.getDeathMaxRotation(1): not translated, its class isn't translated (it's generic).
        // CorvidRenderer.adjustModelBonesForRender(2): not translated, its class isn't translated (it's generic).
    }

    /// `FallingCorpseRenderer` (com/lion/graveyard/entities/renders/FallingCorpseRenderer.java)
    pub mod falling_corpse_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FallingCorpseRenderer.FallingCorpseRenderer(1): not translated, its class isn't translated (it's generic).
        // FallingCorpseRenderer.addRenderData(4): not translated, its class isn't translated (it's generic).
        // FallingCorpseRenderer.getRenderType(2): not translated, its class isn't translated (it's generic).
    }

    /// `AcolyteEyes` (com/lion/graveyard/entities/renders/features/AcolyteEyes.java)
    pub mod acolyte_eyes {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EyesLayer`. Its static parts are below.
        // EYES: not translated, it uses Minecraft's `RenderType`.
        // AcolyteEyes.AcolyteEyes(1): not translated, its class isn't translated (it extends Minecraft's `EyesLayer`).
        // AcolyteEyes.renderType(0): not translated, its class isn't translated (it extends Minecraft's `EyesLayer`).
    }

    /// `CorvidHeldItemLayer` (com/lion/graveyard/entities/renders/features/CorvidHeldItemLayer.java)
    pub mod corvid_held_item_layer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CorvidHeldItemLayer.CorvidHeldItemLayer(2): not translated, its class isn't translated (it's generic).
        // CorvidHeldItemLayer.getRelevantBones(4): not translated, its class isn't translated (it's generic).
        // CorvidHeldItemLayer.addRenderData(4): not translated, its class isn't translated (it's generic).
        // CorvidHeldItemLayer.submitItemStackRender(7): not translated, its class isn't translated (it's generic).
    }

    /// `GhoulingCoffinLayer` (com/lion/graveyard/entities/renders/features/GhoulingCoffinLayer.java)
    pub mod ghouling_coffin_layer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const TORSO: &str = "torso";
        // GhoulingCoffinLayer.GhoulingCoffinLayer(1): not translated, its class isn't translated (it's generic).
        // GhoulingCoffinLayer.getTextureResource(1): not translated, its class isn't translated (it's generic).
        // GhoulingCoffinLayer.getRenderType(2): not translated, its class isn't translated (it's generic).
        // GhoulingCoffinLayer.preRender(2): not translated, its class isn't translated (it's generic).
        // GhoulingCoffinLayer.addPerBoneRender(2): not translated, its class isn't translated (it's generic).
        // GhoulingCoffinLayer.hasCoffin(1): not translated, its class isn't translated (it's generic).
        // GhoulingCoffinLayer.submitOffhandItem(3): not translated, its class isn't translated (it's generic).
    }

    /// `GraveyardEyesLayer` (com/lion/graveyard/entities/renders/features/GraveyardEyesLayer.java)
    pub mod graveyard_eyes_layer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const FULL_LIGHT: i32 = 15728640;
        pub const OPAQUE_WHITE: i32 = -1;
        // GraveyardEyesLayer.GraveyardEyesLayer(2): not translated, its class isn't translated (it's generic).
        // GraveyardEyesLayer.GraveyardEyesLayer(2): not translated, its class isn't translated (it's generic).
        // GraveyardEyesLayer.getTextureResource(1): not translated, its class isn't translated (it's generic).
        // GraveyardEyesLayer.submitRenderTask(2): not translated, its class isn't translated (it's generic).
    }

    /// `SkeletonCreeperChargeFeatureRenderer` (com/lion/graveyard/entities/renders/features/SkeletonCreeperChargeFeatureRenderer.java)
    pub mod skeleton_creeper_charge_feature_renderer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EnergySwirlLayer`. Its static parts are below.
        pub static SKIN: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "textures/entity/creeper/creeper_armor.png"));
        // SkeletonCreeperChargeFeatureRenderer.SkeletonCreeperChargeFeatureRenderer(2): not translated, its class isn't translated (it extends Minecraft's `EnergySwirlLayer`).
        // SkeletonCreeperChargeFeatureRenderer.isPowered(1): not translated, its class isn't translated (it extends Minecraft's `EnergySwirlLayer`).
        // SkeletonCreeperChargeFeatureRenderer.xOffset(1): not translated, its class isn't translated (it extends Minecraft's `EnergySwirlLayer`).
        // SkeletonCreeperChargeFeatureRenderer.getTextureLocation(0): not translated, its class isn't translated (it extends Minecraft's `EnergySwirlLayer`).
        // SkeletonCreeperChargeFeatureRenderer.model(0): not translated, its class isn't translated (it extends Minecraft's `EnergySwirlLayer`).
    }

    /// `SkeletonCreeperEyes` (com/lion/graveyard/entities/renders/features/SkeletonCreeperEyes.java)
    pub mod skeleton_creeper_eyes {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EyesLayer`. Its static parts are below.
        // EYES: not translated, it uses Minecraft's `RenderType`.
        // SkeletonCreeperEyes.SkeletonCreeperEyes(1): not translated, its class isn't translated (it extends Minecraft's `EyesLayer`).
        // SkeletonCreeperEyes.renderType(0): not translated, its class isn't translated (it extends Minecraft's `EyesLayer`).
    }

    /// `GhoulingRenderer` (com/lion/graveyard/entities/renders/GhoulingRenderer.java)
    pub mod ghouling_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GhoulingRenderer.GhoulingRenderer(1): not translated, its class isn't translated (it's generic).
        // GhoulingRenderer.applyAnimationControllers(2): not translated, its class isn't translated (it's generic).
        // GhoulingRenderer.getDeathMaxRotation(1): not translated, its class isn't translated (it's generic).
        // GhoulingRenderer.addRenderData(4): not translated, its class isn't translated (it's generic).
        // GhoulingRenderer.adjustModelBonesForRender(2): not translated, its class isn't translated (it's generic).
    }

    /// `GhoulRenderer` (com/lion/graveyard/entities/renders/GhoulRenderer.java)
    pub mod ghoul_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GhoulRenderer.GhoulRenderer(1): not translated, its class isn't translated (it's generic).
        // GhoulRenderer.applyAnimationControllers(2): not translated, its class isn't translated (it's generic).
        // GhoulRenderer.getDeathMaxRotation(1): not translated, its class isn't translated (it's generic).
        // GhoulRenderer.eyesTexture(1): not translated, it uses `GeoRenderState` (com.geckolib.renderer.base).
    }

    /// `LichRenderer` (com/lion/graveyard/entities/renders/LichRenderer.java)
    pub mod lich_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub static EYES: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/lich_eye_texture.png"));
        pub const MAX_HEAD_YAW: f32 = 50.0f32;
        // LichRenderer.LichRenderer(1): not translated, its class isn't translated (it's generic).
        // LichRenderer.applyAnimationControllers(2): not translated, its class isn't translated (it's generic).
        // LichRenderer.getDeathMaxRotation(1): not translated, its class isn't translated (it's generic).
        // LichRenderer.adjustModelBonesForRender(2): not translated, its class isn't translated (it's generic).
        // LichRenderer.getRenderType(2): not translated, its class isn't translated (it's generic).
    }

    /// `NamelessHangedRenderer` (com/lion/graveyard/entities/renders/NamelessHangedRenderer.java)
    pub mod nameless_hanged_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NamelessHangedRenderer.NamelessHangedRenderer(1): not translated, its class isn't translated (it's generic).
        // NamelessHangedRenderer.getRenderType(2): not translated, its class isn't translated (it's generic).
    }

    /// `NightmareRenderer` (com/lion/graveyard/entities/renders/NightmareRenderer.java)
    pub mod nightmare_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NightmareRenderer.NightmareRenderer(1): not translated, its class isn't translated (it's generic).
        // NightmareRenderer.getDeathMaxRotation(1): not translated, its class isn't translated (it's generic).
    }

    /// `ReaperRenderer` (com/lion/graveyard/entities/renders/ReaperRenderer.java)
    pub mod reaper_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub static EYES: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/reaper_eyes.png"));
        // ReaperRenderer.ReaperRenderer(1): not translated, its class isn't translated (it's generic).
        // ReaperRenderer.applyAnimationControllers(2): not translated, its class isn't translated (it's generic).
        // ReaperRenderer.getDeathMaxRotation(1): not translated, its class isn't translated (it's generic).
    }

    /// `RevenantRenderer` (com/lion/graveyard/entities/renders/RevenantRenderer.java)
    pub mod revenant_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub static EYES: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/revenant_eyes.png"));
        // RevenantRenderer.RevenantRenderer(1): not translated, its class isn't translated (it's generic).
        // RevenantRenderer.getDeathMaxRotation(1): not translated, its class isn't translated (it's generic).
    }

    /// `SkeletonCreeperRender` (com/lion/graveyard/entities/renders/SkeletonCreeperRender.java)
    pub mod skeleton_creeper_render {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobRenderer`. Its static parts are below.
        pub static TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/skeleton_creeper.png"));
        // SkeletonCreeperRender.SkeletonCreeperRender(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // SkeletonCreeperRender.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // SkeletonCreeperRender.extractRenderState(3): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // SkeletonCreeperRender.scale(2): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // SkeletonCreeperRender.getWhiteOverlayProgress(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // SkeletonCreeperRender.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
    }

    /// `SkullEntityRenderer` (com/lion/graveyard/entities/renders/SkullEntityRenderer.java)
    pub mod skull_entity_renderer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityRenderer`. Its static parts are below.
        pub static TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "textures/entity/skeleton/skeleton.png"));
        // SkullEntityRenderer.SkullEntityRenderer(1): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // SkullEntityRenderer.getBlockLightLevel(2): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // SkullEntityRenderer.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // SkullEntityRenderer.extractRenderState(3): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // SkullEntityRenderer.submit(4): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
    }

    /// `TormentedVillagerRenderer` (com/lion/graveyard/entities/renders/TormentedVillagerRenderer.java)
    pub mod tormented_villager_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub static EYES: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/tormented_villager_eyes.png"));
        // TormentedVillagerRenderer.TormentedVillagerRenderer(1): not translated, its class isn't translated (it's generic).
        // TormentedVillagerRenderer.getDeathMaxRotation(1): not translated, its class isn't translated (it's generic).
        // TormentedVillagerRenderer.getRenderType(2): not translated, its class isn't translated (it's generic).
    }

    /// `WraithRenderer` (com/lion/graveyard/entities/renders/WraithRenderer.java)
    pub mod wraith_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub static EYES: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/wraith_eyes.png"));
        // WraithRenderer.WraithRenderer(1): not translated, its class isn't translated (it's generic).
        // WraithRenderer.applyAnimationControllers(2): not translated, its class isn't translated (it's generic).
        // WraithRenderer.getDeathMaxRotation(1): not translated, its class isn't translated (it's generic).
        // WraithRenderer.getRenderType(2): not translated, its class isn't translated (it's generic).
    }

    /// `RevenantEntity` (com/lion/graveyard/entities/RevenantEntity.java)
    pub mod revenant_entity {
        use super::*;
        // The class itself isn't translated: it extends `AngerableGraveyardEntity`. Its static parts are below.
        pub static SLOWNESS_ID: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("slowness_id"));
        // SLOWNESS_EFFECT: not translated, it uses Minecraft's `AttributeModifier`.
        // ATTACK_ANIM_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // REANIMATE_ANIM_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // ANIMATION: not translated, it uses Minecraft's `EntityDataAccessor`.
        // CAN_REANIMATE: not translated, it uses Minecraft's `EntityDataAccessor`.
        // CHASING: not translated, it uses Minecraft's `EntityDataAccessor`.
        pub const CHASE_END_TICKS: i32 = 20;
        pub const ANIMATION_IDLE: i32 = 0;
        pub const ANIMATION_WALK: i32 = 1;
        pub const ANIMATION_DEATH: i32 = 2;
        pub const ANIMATION_RUNNING: i32 = 3;
        pub const ANIMATION_MELEE: i32 = 4;
        pub const ANIMATION_REANIMATE: i32 = 5;
        pub const ANIMATION_FAKE_DEATH: i32 = 6;
        // RevenantEntity.RevenantEntity(2): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.defineSynchedData(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.registerGoals(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.createRevenantAttributes(0): not translated, it uses Minecraft's `Builder`.
        // RevenantEntity.tick(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.restartAttackClipOnRearm(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.customServerAiStep(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.trackChase(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.aiStep(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.tickDeath(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.hurtServer(3): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.registerControllers(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.isRunning(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.playAmbientSound(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.playHurtSound(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.die(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.playStepSound(2): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.getAnimationState(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.setAnimationState(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.getAttackAnimTimer(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.setAttackAnimTimer(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.getReanimateAnimTimer(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.setReanimateAnimTimer(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.canReanimate(0): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.setCanReanimate(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
        // RevenantEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends `AngerableGraveyardEntity`).
    }

    /// `SkeletonCreeper` (com/lion/graveyard/entities/SkeletonCreeper.java)
    pub mod skeleton_creeper {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Creeper`. Its static parts are below.
        // SkeletonCreeper.SkeletonCreeper(2): not translated, its class isn't translated (it extends Minecraft's `Creeper`).
        // SkeletonCreeper.createSkeletonCreeperAttributes(0): not translated, it uses Minecraft's `Builder`.
        // SkeletonCreeper.canStart(0): not translated, its class isn't translated (it extends Minecraft's `Creeper`).
        // SkeletonCreeper.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Creeper`).
        // SkeletonCreeper.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Creeper`).
        // SkeletonCreeper.dropCustomDeathLoot(3): not translated, its class isn't translated (it extends Minecraft's `Creeper`).
        // SkeletonCreeper.explode(0): not translated, its class isn't translated (it extends Minecraft's `Creeper`).
        // SkeletonCreeper.spawnEffectsCloud(0): not translated, its class isn't translated (it extends Minecraft's `Creeper`).
        // SkeletonCreeper.aiStep(0): not translated, its class isn't translated (it extends Minecraft's `Creeper`).
        pub fn explosion_radius(config: &graveyard_config::GraveyardConfig) -> f32 {
            let radius: f64 = config.number("skeletonCreeperExplosionRadius", 3.5f64);
            return if radius.is_finite() { ((0.0f64.max(16.0f64.min(radius))) as f32) } else { 3.5f32 };
        }
    }

    /// `TormentedVillagerEntity` (com/lion/graveyard/entities/TormentedVillagerEntity.java)
    pub mod tormented_villager_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `PathfinderMob`. Its static parts are below.
        // MOOD: not translated, it uses Minecraft's `EntityDataAccessor`.
        // SPAWN_TICKS_LEFT: not translated, it uses Minecraft's `EntityDataAccessor`.
        pub const BELL_RANGE: i32 = 48;
        pub const REST_CHANCE: i32 = 350;
        pub const STEP_DISTANCE: f32 = 0.6f32;
        pub const STEP_VOLUME: f32 = 0.35f32;
        pub const AMBIENT_SOUND_DELAY: i32 = 160;
        pub const RAGE_SOUND_TICKS: i32 = 60;
        pub const MIN_AGE_TO_FADE: i32 = 100;
        pub const REDEMPTION_WITNESS_RANGE: f64 = 16.0f64;
        pub const DAWN_WITNESS_RANGE: f64 = 12.0f64;
        pub const LIGHT_LEVEL: i32 = 5;
        pub const REDEMPTION_EXPERIENCE: i32 = 5;
        pub const LIGHT_KEY: &str = "LightPos";
        pub const MOOD_TRANSITION_TICKS: i32 = 6;
        // TormentedVillagerEntity.TormentedVillagerEntity(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.createTormentedVillagerAttributes(0): not translated, it uses Minecraft's `Builder`.
        // TormentedVillagerEntity.canSpawn(5): not translated, it uses Minecraft's `EntityType`.
        // TormentedVillagerEntity.onBellRung(2): not translated, it uses Minecraft's `ServerLevel`.
        // TormentedVillagerEntity.defineSynchedData(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.registerGoals(0): not translated, its body couldn't be read: an anonymous class.
        // TormentedVillagerEntity.finalizeSpawn(4): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.getMood(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.setMood(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.customServerAiStep(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.startRandomMood(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.hurtServer(3): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.redeem(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.vanish(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.rewardRedemption(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.releaseLegacyLight(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.remove(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.restoreFrom(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.canTeleport(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.fireImmune(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.getAmbientSound(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.getAmbientSoundInterval(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.nextStep(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.playStepSound(2): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.registerControllers(1): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.spawnClipOffset(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
        // TormentedVillagerEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends Minecraft's `PathfinderMob`).
    }

    /// `TormentedVillagerEntity.Mood` (com/lion/graveyard/entities/TormentedVillagerEntity.java)
    pub mod tormented_villager_entity_mood {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // WALK: not translated, it uses `RawAnimation` (com.geckolib.animation).
        // TormentedVillagerEntity.Mood.animationNames(0): not translated, it calls Mood.values, which isn't in the mod's source.
        // TormentedVillagerEntity.Mood.Mood(3): not translated, its class isn't translated (its constants carry values).
    }

    /// `WraithEntity` (com/lion/graveyard/entities/WraithEntity.java)
    pub mod wraith_entity {
        use super::*;
        // The class itself isn't translated: it extends `HostileGraveyardEntity`. Its static parts are below.
        pub static ATTACKING_SPEED_BOOST_ID: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("attacking_speed_boost_id"));
        // ATTACKING_SPEED_BOOST: not translated, it uses Minecraft's `AttributeModifier`.
        // ANIMATION: not translated, it uses Minecraft's `EntityDataAccessor`.
        // SPAWN_TIMER: not translated, it uses Minecraft's `EntityDataAccessor`.
        // CHASING: not translated, it uses Minecraft's `EntityDataAccessor`.
        pub const SPAWN_TICKS: i32 = 20;
        pub const CHASE_END_TICKS: i32 = 20;
        pub const DEATH_TICKS: i32 = 46;
        // WraithEntity.WraithEntity(2): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.move(2): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.makeStuckInBlock(2): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.registerGoals(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.createNavigation(1): not translated, its body couldn't be read: an anonymous class.
        // WraithEntity.hurtServer(3): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.defineSynchedData(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.startSpawnAnimation(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.tick(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.readAdditionalSaveData(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.aiStep(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.finalizeSpawn(4): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.tickDeath(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.addParticles(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.addDeathParticles(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.getTimeSinceExtinguish(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.addExtinguishCounter(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.hasHomePosition(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.isInvulnerableTo(2): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.createWraithAttributes(0): not translated, it uses Minecraft's `Builder`.
        // WraithEntity.trackChase(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.getAnimation(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.setAnimation(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.registerControllers(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.isSpawning(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.getAnimatableInstanceCache(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.isWithinDistance(2): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.playAmbientSound(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.playHurtSound(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.die(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.playDeathSound(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.canCollideWith(1): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.playBlockFallSound(0): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.playStepSound(2): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.causeFallDamage(3): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.checkFallDamage(4): not translated, its class isn't translated (it extends `HostileGraveyardEntity`).
        // WraithEntity.triggerAdvancement(2): not translated, it uses WraithEntity, which isn't translated (it extends `HostileGraveyardEntity`).
    }

    /// `WraithEntity.ExtinguishGoal` (com/lion/graveyard/entities/WraithEntity.java)
    pub mod wraith_entity_extinguish_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // WraithEntity.ExtinguishGoal.ExtinguishGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // WraithEntity.ExtinguishGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // WraithEntity.ExtinguishGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // WraithEntity.ExtinguishGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `WraithEntity.WraithMeleeGoal` (com/lion/graveyard/entities/WraithEntity.java)
    pub mod wraith_entity_wraith_melee_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MeleeAttackGoal`. Its static parts are below.
        // WraithEntity.WraithMeleeGoal.WraithMeleeGoal(3): not translated, its class isn't translated (it extends Minecraft's `MeleeAttackGoal`).
        // WraithEntity.WraithMeleeGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `MeleeAttackGoal`).
        // WraithEntity.WraithMeleeGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `MeleeAttackGoal`).
    }

    /// `WraithEntity.WraithWanderAroundGoal` (com/lion/graveyard/entities/WraithEntity.java)
    pub mod wraith_entity_wraith_wander_around_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        pub const MAX_DISTANCE: i32 = 15;
        // WraithEntity.WraithWanderAroundGoal.WraithWanderAroundGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // WraithEntity.WraithWanderAroundGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // WraithEntity.WraithWanderAroundGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // WraithEntity.WraithWanderAroundGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // WraithEntity.WraithWanderAroundGoal.getRandomLocation(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `CommonEvents` (com/lion/graveyard/event/CommonEvents.java)
    pub mod common_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommonEvents {
        }
        impl CommonEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommonEvents.onEntityLoad(1): not translated, it uses Minecraft's `Entity`.
        // CommonEvents.addFleeGoal(3): not translated, it uses Minecraft's `AbstractVillager`.
        // CommonEvents.onDeath(1): not translated, it uses Minecraft's `LivingEntity`.
        // CommonEvents.onUseItem(2): not translated, it uses Minecraft's `Player`.
        // CommonEvents.onUseBlock(3): not translated, it uses Minecraft's `Player`.
        // CommonEvents.usesBlock(1): not translated, it uses Minecraft's `Player`.
        // CommonEvents.findPage(2): not translated, it uses Minecraft's `ServerPlayer`.
        // CommonEvents.smolderingStrippedState(1): not translated, it uses Minecraft's `BlockState`.
        // CommonEvents.stripSmolderingPillar(4): not translated, it uses Minecraft's `InteractionResult`.
        // CommonEvents.shieldTakesPriority(2): not translated, it uses Minecraft's `Player`.
        // CommonEvents.mayModify(4): not translated, it uses Minecraft's `Player`.
    }

    /// `GraveyardClientFabric` (com/lion/graveyard/fabric/GraveyardClientFabric.java)
    pub mod graveyard_client_fabric {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // GraveyardClientFabric.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
    }

    /// `GraveyardFabric` (com/lion/graveyard/fabric/GraveyardFabric.java)
    pub mod graveyard_fabric {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        // GraveyardFabric.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // GraveyardFabric.registerCommands(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // GraveyardFabric.registerServerEvents(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
    }

    /// `Graveyard` (com/lion/graveyard/Graveyard.java)
    pub mod graveyard {
        use super::*;
        pub const MOD_ID: &str = "graveyard";
        // LOGGER: the logger. Log calls become comments.
        // CONFIG: not translated, it creates a GraveyardConfig with a constructor that isn't translated.
        #[derive(Clone, Debug)]
        pub struct Graveyard {
        }
        impl Graveyard {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn id(path: &str) -> JIdentifier {
            return JIdentifier::new("graveyard", &path);
        }
        // Graveyard.getConfig(0): not translated, it uses Graveyard.CONFIG, which isn't translated (it creates a GraveyardConfig with a constructor that isn't translated).
        // Graveyard.getLogger(0): not translated, it uses `Logger` (org.slf4j).
        // Graveyard.init(0): not translated, it calls TGItemGroups.init, which isn't translated.
        // Graveyard.postInit(0): not translated, it calls TGBlocks.postInit, which isn't translated.
        pub fn create_string_id(name: &str) -> String {
            return format!("graveyard:{}", name);
        }
    }

    /// `GraveyardClient` (com/lion/graveyard/GraveyardClient.java)
    pub mod graveyard_client {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct GraveyardClient {
        }
        impl GraveyardClient {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GraveyardClient.init(0): not translated, it uses Minecraft's `RangeSelectItemModelProperties`.ID_MAPPER.
        // GraveyardClient.postInit(0): not translated, it calls TGEntityRenderers.postInit, which isn't translated.
    }

    /// `GravestoneScreen` (com/lion/graveyard/gui/GravestoneScreen.java)
    pub mod gravestone_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub const FACE_SCALE: f32 = 3.9f32;
        pub const TEXT_SCALE: f32 = 0.9765628f32;
        pub const SIGN_Y_OFFSET: f32 = 90.0f32;
        // GravestoneScreen.GravestoneScreen(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.GravestoneScreen(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.setCurrentRowMessage(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.removed(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.isInputCaptured(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.tick(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.finishEditing(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.charTyped(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.keyPressed(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.extractBackground(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.extractSignBackground(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GravestoneScreen.extractSignText(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `OssuaryScreen` (com/lion/graveyard/gui/OssuaryScreen.java)
    pub mod ossuary_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerScreen`. Its static parts are below.
        pub static TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "textures/gui/container/stonecutter.png"));
        pub static SCROLLER_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/stonecutter/scroller"));
        pub static SCROLLER_DISABLED_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/stonecutter/scroller_disabled"));
        pub static RECIPE_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/stonecutter/recipe"));
        pub static RECIPE_SELECTED_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/stonecutter/recipe_selected"));
        pub static RECIPE_HIGHLIGHTED_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/stonecutter/recipe_highlighted"));
        // INPUT_HINT: not translated, it uses Minecraft's `ItemStack`.
        // INPUT_HINT_TEXT: a game value, so it's in game_side.rs.
        // OssuaryScreen.OssuaryScreen(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // OssuaryScreen.receive(1): not translated, it uses OssuaryRecipesSync, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // OssuaryScreen.extractBackground(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // OssuaryScreen.extractTooltip(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // OssuaryScreen.extractRecipeBackground(6): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // OssuaryScreen.extractRecipeIcons(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // OssuaryScreen.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // OssuaryScreen.mouseDragged(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // OssuaryScreen.mouseScrolled(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // OssuaryScreen.shouldScroll(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // OssuaryScreen.getMaxScroll(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // OssuaryScreen.onInventoryChange(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
    }

    /// `OssuaryScreenHandler` (com/lion/graveyard/gui/OssuaryScreenHandler.java)
    pub mod ossuary_screen_handler {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerMenu`. Its static parts are below.
        // OssuaryScreenHandler.OssuaryScreenHandler(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.OssuaryScreenHandler(3): not translated, its body couldn't be read: an anonymous class.
        // OssuaryScreenHandler.getSelectedRecipe(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.getAvailableResults(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.getAvailableRecipeCount(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.setAvailableResults(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.canCraft(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.stillValid(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.clickMenuButton(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.isInBounds(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.slotsChanged(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.updateInput(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.findRecipes(2): not translated, it uses Minecraft's `RecipeHolder`.
        // OssuaryScreenHandler.populateResult(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.getType(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.setContentsChangedListener(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.canTakeItemForPickAll(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.quickMoveStack(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // OssuaryScreenHandler.removed(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
    }

    /// `TGAdvancements` (com/lion/graveyard/init/TGAdvancements.java)
    pub mod tg_advancements {
        use super::*;
        // KILLED_BY_BONE_DAGGER: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // KILL_WHILE_BLINDED: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // DIM_LIGHT: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // KILL_HORDE: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // SPAWN_WRAITH: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // EQUIP_COFFIN: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // REDEEM_TORMENTED_VILLAGER: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // TORMENTED_VILLAGER_DAWN: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // EXTINGUISH_SMOLDERING: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // IGNITE_SMOLDERING: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // SCARE_CROWS: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // MAGPIE_LOOT: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // RAVEN_ON_GRAVESTONE: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // SCARECROW: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // FIRST_CHRONICLE_PAGE: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // CHRONICLE_ACT_ONE: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // CHRONICLE_ACT_TWO: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // PLACE_LORE_BOOK: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // BONE_COLLECTOR: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // GHOUL_EMERGES: not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        pub static REDEEM_TEN_ADVANCEMENT: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("graveyard/redeem_ten_tormented_villagers"));
        pub const REDEEM_TEN_STEPS: i32 = 10;
        #[derive(Clone, Debug)]
        pub struct TGAdvancements {
        }
        impl TGAdvancements {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
        // TGAdvancements.trigger(1): not translated, it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // TGAdvancements.triggerNearby(4): not translated, it uses Minecraft's `ServerLevel`.
        // TGAdvancements.grantNextStep(3): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `TGBiomes` (com/lion/graveyard/init/TGBiomes.java)
    pub mod tg_biomes {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TGBiomes {
        }
        impl TGBiomes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `TGBlockEntities` (com/lion/graveyard/init/TGBlockEntities.java)
    pub mod tg_block_entities {
        use super::*;
        // GRAVESTONE_BLOCK_ENTITY: not translated, it uses Minecraft's `BlockEntityType`.
        // URN_BLOCK_ENTITY: not translated, it uses Minecraft's `BlockEntityType`.
        // SARCOPHAGUS_BLOCK_ENTITY: not translated, it uses Minecraft's `BlockEntityType`.
        // BRAZIER_BLOCK_ENTITY: not translated, it uses Minecraft's `BlockEntityType`.
        // LORE_BOOK_BLOCK_ENTITY: not translated, it uses Minecraft's `BlockEntityType`.
        // OSSUARY_BLOCK_ENTITY: not translated, it uses Minecraft's `BlockEntityType`.
        #[derive(Clone, Debug)]
        pub struct TGBlockEntities {
        }
        impl TGBlockEntities {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
    }

    /// `TGBlockEntityRenderers` (com/lion/graveyard/init/TGBlockEntityRenderers.java)
    pub mod tg_block_entity_renderers {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TGBlockEntityRenderers {
        }
        impl TGBlockEntityRenderers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TGBlockEntityRenderers.postInit(0): not translated, it uses Minecraft's `BlockEntityType`.
    }

    /// `TGBlocks` (com/lion/graveyard/init/TGBlocks.java)
    pub mod tg_blocks {
        use super::*;
        // coffins: not translated, it uses Minecraft's `Block`.
        // TG_ROOTED_DIRT: not translated, it uses Minecraft's `Block`.
        // TG_TUFF: not translated, it uses Minecraft's `Block`.
        // TG_MOSS_BLOCK: not translated, it uses Minecraft's `Block`.
        // TG_DEEPSLATE: not translated, it uses Minecraft's `Block`.
        // TG_GRASS_BLOCK: not translated, it uses Minecraft's `Block`.
        // TG_DIRT: not translated, it uses Minecraft's `Block`.
        // TG_COARSE_DIRT: not translated, it uses Minecraft's `Block`.
        // TG_ANDESITE: not translated, it uses Minecraft's `Block`.
        // TG_GRANITE: not translated, it uses Minecraft's `Block`.
        // TG_DIORITE: not translated, it uses Minecraft's `Block`.
        // TG_STONE: not translated, it uses Minecraft's `Block`.
        // TG_PODZOL: not translated, it uses Minecraft's `Block`.
        // DARK_IRON_BARS: not translated, it uses Minecraft's `Block`.
        // SOUL_FIRE_BRAZIER: not translated, it uses Minecraft's `Block`.
        // FIRE_BRAZIER: not translated, it uses Minecraft's `Block`.
        // PEDESTAL: not translated, it uses Minecraft's `Block`.
        // CANDLE_HOLDER: not translated, it uses Minecraft's `Block`.
        // DARK_IRON_TRAPDOOR: not translated, it uses Minecraft's `Block`.
        // DARK_IRON_DOOR: not translated, it uses Minecraft's `Block`.
        // DARK_IRON_BLOCK: not translated, it uses Minecraft's `Block`.
        // SKULL_WITH_RIB_CAGE: not translated, it uses Minecraft's `Block`.
        // LEANING_SKELETON: not translated, it uses Minecraft's `Block`.
        // SKULL_PILE: not translated, it uses Minecraft's `Block`.
        // LYING_SKELETON: not translated, it uses Minecraft's `Block`.
        // WITHER_SKULL_WITH_RIB_CAGE: not translated, it uses Minecraft's `Block`.
        // LEANING_WITHER_SKELETON: not translated, it uses Minecraft's `Block`.
        // WITHER_SKULL_PILE: not translated, it uses Minecraft's `Block`.
        // LYING_WITHER_SKELETON: not translated, it uses Minecraft's `Block`.
        // CREEPER_SKELETON: not translated, it uses Minecraft's `Block`.
        // SKELETON_HAND: not translated, it uses Minecraft's `Block`.
        // WITHER_SKELETON_HAND: not translated, it uses Minecraft's `Block`.
        // BONE_REMAINS: not translated, it uses Minecraft's `Block`.
        // WITHER_BONE_REMAINS: not translated, it uses Minecraft's `Block`.
        // LATERALLY_LYING_SKELETON: not translated, it uses Minecraft's `Block`.
        // LATERALLY_LYING_WITHER_SKELETON: not translated, it uses Minecraft's `Block`.
        // SKULL_ON_PIKE: not translated, it uses Minecraft's `Block`.
        // WITHER_SKULL_ON_PIKE: not translated, it uses Minecraft's `Block`.
        // TORSO_PILE: not translated, it uses Minecraft's `Block`.
        // WITHER_TORSO_PILE: not translated, it uses Minecraft's `Block`.
        // HANGED_SKELETON: not translated, it uses Minecraft's `Block`.
        // HANGED_WITHER_SKELETON: not translated, it uses Minecraft's `Block`.
        // SEATED_FULL_SKELETON: not translated, it uses Minecraft's `Block`.
        // SEATED_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Block`.
        // SKELETON_SKULL_WITH_AXE: not translated, it uses Minecraft's `Block`.
        // WITHER_SKELETON_SKULL_WITH_AXE: not translated, it uses Minecraft's `Block`.
        // SWORD_IN_SKELETON: not translated, it uses Minecraft's `Block`.
        // SWORD_IN_WITHER_SKELETON: not translated, it uses Minecraft's `Block`.
        // SMALL_SKELETON_REMAINS: not translated, it uses Minecraft's `Block`.
        // SMALL_WITHER_SKELETON_REMAINS: not translated, it uses Minecraft's `Block`.
        // HANGED_FULL_SKELETON: not translated, it uses Minecraft's `Block`.
        // HANGED_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Block`.
        // LYING_FULL_SKELETON: not translated, it uses Minecraft's `Block`.
        // LYING_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Block`.
        // BACKSTABBED_FULL_SKELETON: not translated, it uses Minecraft's `Block`.
        // BACKSTABBED_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Block`.
        // SHOT_FULL_SKELETON: not translated, it uses Minecraft's `Block`.
        // SHOT_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Block`.
        // SIDEWAYS_FULL_SKELETON: not translated, it uses Minecraft's `Block`.
        // SIDEWAYS_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Block`.
        // BLACK_URN: not translated, it uses Minecraft's `Block`.
        // BROWN_URN: not translated, it uses Minecraft's `Block`.
        // WHITE_URN: not translated, it uses Minecraft's `Block`.
        // GRAY_URN: not translated, it uses Minecraft's `Block`.
        // LIGHT_GRAY_URN: not translated, it uses Minecraft's `Block`.
        // BLUE_URN: not translated, it uses Minecraft's `Block`.
        // CYAN_URN: not translated, it uses Minecraft's `Block`.
        // LIGHT_BLUE_URN: not translated, it uses Minecraft's `Block`.
        // RED_URN: not translated, it uses Minecraft's `Block`.
        // ORANGE_URN: not translated, it uses Minecraft's `Block`.
        // YELLOW_URN: not translated, it uses Minecraft's `Block`.
        // PINK_URN: not translated, it uses Minecraft's `Block`.
        // MAGENTA_URN: not translated, it uses Minecraft's `Block`.
        // PURPLE_URN: not translated, it uses Minecraft's `Block`.
        // GREEN_URN: not translated, it uses Minecraft's `Block`.
        // LIME_URN: not translated, it uses Minecraft's `Block`.
        // TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // BLACK_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // BLUE_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // BROWN_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // CYAN_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // GRAY_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // GREEN_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // LIGHT_BLUE_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // LIGHT_GRAY_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // LIME_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // MAGENTA_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // ORANGE_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // PINK_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // PURPLE_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // RED_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // WHITE_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // YELLOW_TERRACOTTA_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_BLACK_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_BROWN_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_WHITE_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_GRAY_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_LIGHT_GRAY_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_BLUE_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_CYAN_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_LIGHT_BLUE_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_RED_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_ORANGE_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_YELLOW_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_PINK_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_MAGENTA_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_PURPLE_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_GREEN_URN: not translated, it uses Minecraft's `Block`.
        // SMALL_LIME_URN: not translated, it uses Minecraft's `Block`.
        // SARCOPHAGUS: not translated, it uses Minecraft's `Block`.
        // OAK_COFFIN: not translated, it uses Minecraft's `Block`.
        // SPRUCE_COFFIN: not translated, it uses Minecraft's `Block`.
        // BIRCH_COFFIN: not translated, it uses Minecraft's `Block`.
        // DARK_OAK_COFFIN: not translated, it uses Minecraft's `Block`.
        // JUNGLE_COFFIN: not translated, it uses Minecraft's `Block`.
        // ACACIA_COFFIN: not translated, it uses Minecraft's `Block`.
        // WARPED_COFFIN: not translated, it uses Minecraft's `Block`.
        // CRIMSON_COFFIN: not translated, it uses Minecraft's `Block`.
        // MANGROVE_COFFIN: not translated, it uses Minecraft's `Block`.
        // BAMBOO_COFFIN: not translated, it uses Minecraft's `Block`.
        // CHERRY_COFFIN: not translated, it uses Minecraft's `Block`.
        // PALE_OAK_COFFIN: not translated, it uses Minecraft's `Block`.
        // POPLAR_COFFIN: not translated, it uses Minecraft's `Block`.
        // VASE_BLOCK: not translated, it uses Minecraft's `Block`.
        // GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // COBBLESTONE_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // MOSSY_COBBLESTONE_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // DEEPSLATE_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // BLACKSTONE_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // CRACKED_BLACKSTONE_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // STONE_BRICKS_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // MOSSY_STONE_BRICKS_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // BRICKS_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // SANDSTONE_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // RED_SANDSTONE_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // GILDED_BLACKSTONE_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // QUARTZ_BRICKS_GRAVESTONE: not translated, it uses Minecraft's `Block`.
        // ALTAR: not translated, it uses Minecraft's `Block`.
        // ALTAR_SIDE: not translated, it uses Minecraft's `Block`.
        // ALTAR_CORNER: not translated, it uses Minecraft's `Block`.
        // ALTAR_CENTER: not translated, it uses Minecraft's `Block`.
        // LOWER_BONE_STAFF: not translated, it uses Minecraft's `Block`.
        // MIDDLE_BONE_STAFF: not translated, it uses Minecraft's `Block`.
        // UPPER_BONE_STAFF: not translated, it uses Minecraft's `Block`.
        // OSSUARY: not translated, it uses Minecraft's `Block`.
        // LORE_BOOK: not translated, it uses Minecraft's `Block`.
        #[derive(Clone, Debug)]
        pub struct TGBlocks {
        }
        impl TGBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TGBlocks.boneDisplayProperties(0): not translated, it calls Minecraft's `Properties`.of.
        // TGBlocks.urnProperties(0): not translated, it calls Minecraft's `Properties`.of.
        // TGBlocks.vaseProperties(0): not translated, it calls Minecraft's `Properties`.of.
        // TGBlocks.boneStaffFragmentProperties(0): not translated, it calls Minecraft's `Properties`.of.
        // TGBlocks.loreBookProperties(0): not translated, it calls Minecraft's `Properties`.of.
        // TGBlocks.gravestoneProperties(0): not translated, it calls Minecraft's `Properties`.of.
        pub fn init() {
        }
        // TGBlocks.postInit(0): not translated, it uses TGBlocks.coffins, which isn't translated (it uses Minecraft's `Block`).
    }

    /// `TGEntities` (com/lion/graveyard/init/TGEntities.java)
    pub mod tg_entities {
        use super::*;
        // REAPER: not translated, it uses Minecraft's `EntityType`.
        // ACOLYTE: not translated, it uses Minecraft's `EntityType`.
        // CORRUPTED_VINDICATOR: not translated, it uses Minecraft's `EntityType`.
        // CORRUPTED_PILLAGER: not translated, it uses Minecraft's `EntityType`.
        // CORRUPTED_EVOKER: not translated, it uses Minecraft's `EntityType`.
        // SKELETON_CREEPER: not translated, it uses Minecraft's `EntityType`.
        // FALLING_CORPSE: not translated, it uses Minecraft's `EntityType`.
        // GHOUL: not translated, it uses Minecraft's `EntityType`.
        // TORMENTED_VILLAGER: not translated, it uses Minecraft's `EntityType`.
        // REVENANT: not translated, it uses Minecraft's `EntityType`.
        // WRAITH: not translated, it uses Minecraft's `EntityType`.
        // NIGHTMARE: not translated, it uses Minecraft's `EntityType`.
        // GHOULING: not translated, it uses Minecraft's `EntityType`.
        // LICH: not translated, it uses Minecraft's `EntityType`.
        // SKULL: not translated, it uses Minecraft's `EntityType`.
        // NAMELESS_HANGED: not translated, it uses Minecraft's `EntityType`.
        // RAVEN: not translated, it uses Minecraft's `EntityType`.
        // MAGPIE: not translated, it uses Minecraft's `EntityType`.
        // CROW: not translated, it uses Minecraft's `EntityType`.
        // SCALED_ATTRIBUTES: not translated, it uses Minecraft's `Holder`.
        #[derive(Clone, Debug)]
        pub struct TGEntities {
        }
        impl TGEntities {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TGEntities.init(0): not translated, it calls TGEntities.createMobAttributes, which isn't translated.
        // TGEntities.postInit(0): not translated, it calls TGEntities.addSpawns, which isn't translated.
        // TGEntities.naturalSpawns(0): not translated, it uses Minecraft's `TagKey`.
        // TGEntities.biomeTag(1): not translated, it uses Minecraft's `TagKey`.
        // TGEntities.addSpawns(0): not translated, it calls TGEntities.naturalSpawns, which isn't translated.
        // TGEntities.gated(1): not translated, it's generic.
        // TGEntities.addSpawnRestrictions(0): not translated, it uses Minecraft's `EntityType`.
        // TGEntities.scaled(2): not translated, it uses Minecraft's `Builder`.
        // TGEntities.factor(2): not translated, it uses Minecraft's `Holder`.
        // TGEntities.createMobAttributes(0): not translated, it uses Minecraft's `EntityType`.
    }

    /// `TGEntities.NaturalSpawn` (com/lion/graveyard/init/TGEntities.java)
    pub mod tg_entities_natural_spawn {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `TagKey`. Its static parts are below.
        // TGEntities.NaturalSpawn.NaturalSpawn(4): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // TGEntities.NaturalSpawn.weight(0): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // TGEntities.NaturalSpawn.minGroup(0): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // TGEntities.NaturalSpawn.maxGroup(0): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // TGEntities.NaturalSpawn.entry(0): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // TGEntities.NaturalSpawn.mob(0): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // TGEntities.NaturalSpawn.tag(0): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // TGEntities.NaturalSpawn.group(0): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // TGEntities.NaturalSpawn.type(0): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
    }

    /// `TGEntityModelLayers` (com/lion/graveyard/init/TGEntityModelLayers.java)
    pub mod tg_entity_model_layers {
        use super::*;
        // CORRUPTED_ILLAGER_MODEL_LAYER: not translated, it uses Minecraft's `ModelLayerLocation`.
        #[derive(Clone, Debug)]
        pub struct TGEntityModelLayers {
        }
        impl TGEntityModelLayers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TGEntityModelLayers.init(0): not translated, it uses Minecraft's `ModelLayerLocation`.
    }

    /// `TGEntityRenderers` (com/lion/graveyard/init/TGEntityRenderers.java)
    pub mod tg_entity_renderers {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TGEntityRenderers {
        }
        impl TGEntityRenderers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TGEntityRenderers.postInit(0): not translated, it uses Minecraft's `EntityType`.
    }

    /// `TGItemGroups` (com/lion/graveyard/init/TGItemGroups.java)
    pub mod tg_item_groups {
        use super::*;
        // ITEM_GROUP: not translated, it uses Minecraft's `ResourceKey`.
        // CHRONICLE_GROUP: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct TGItemGroups {
        }
        impl TGItemGroups {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TGItemGroups.init(0): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `TGItems` (com/lion/graveyard/init/TGItems.java)
    pub mod tg_items {
        use super::*;
        // INCARNATED_EVIL_SONG: not translated, it uses Minecraft's `ResourceKey`.
        // SARCOPHAGUS_LID: not translated, it uses Minecraft's `Item`.
        // SARCOPHAGUS_BASE: not translated, it uses Minecraft's `Item`.
        // DARK_OAK_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // DARK_OAK_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // OAK_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // OAK_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // BIRCH_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // BIRCH_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // JUNGLE_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // JUNGLE_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // ACACIA_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // ACACIA_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // SPRUCE_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // SPRUCE_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // WARPED_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // WARPED_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // CRIMSON_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // CRIMSON_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // MANGROVE_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // MANGROVE_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // BAMBOO_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // BAMBOO_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // CHERRY_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // CHERRY_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // PALE_OAK_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // PALE_OAK_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // POPLAR_COFFIN_LID: not translated, it uses Minecraft's `Item`.
        // POPLAR_COFFIN_BASE: not translated, it uses Minecraft's `Item`.
        // OSSUARY: not translated, it uses Minecraft's `Item`.
        // INCARNATED_EVIL_MUSIC_DISC: not translated, it uses Minecraft's `Item`.
        // SKELETON_CREEPER_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // RAVEN_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // MAGPIE_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // CROW_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // ACOLYTE_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // GHOUL_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // REAPER_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // TORMENTED_VILLAGER_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // REVENANT_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // NIGHTMARE_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // LICH_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // NAMELESS_HANGED_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // WRAITH_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // CORRUPTED_PILLAGER_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // CORRUPTED_VINDICATOR_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // CORRUPTED_EVOKER_SPAWN_EGG: not translated, it uses Minecraft's `Item`.
        // BONE_DAGGER: not translated, it uses Minecraft's `Item`.
        // DARK_IRON_INGOT: not translated, it uses Minecraft's `Item`.
        // CORRUPTION: not translated, it uses Minecraft's `Item`.
        // DIARY_PAGE: not translated, it uses Minecraft's `Item`.
        // RUINS_MAP: not translated, it uses Minecraft's `Item`.
        // TG_ROOTED_DIRT: not translated, it uses Minecraft's `Item`.
        // TG_TUFF: not translated, it uses Minecraft's `Item`.
        // TG_MOSS_BLOCK: not translated, it uses Minecraft's `Item`.
        // TG_DEEPSLATE: not translated, it uses Minecraft's `Item`.
        // TG_GRASS_BLOCK: not translated, it uses Minecraft's `Item`.
        // TG_DIRT: not translated, it uses Minecraft's `Item`.
        // TG_COARSE_DIRT: not translated, it uses Minecraft's `Item`.
        // TG_ANDESITE: not translated, it uses Minecraft's `Item`.
        // TG_GRANITE: not translated, it uses Minecraft's `Item`.
        // TG_DIORITE: not translated, it uses Minecraft's `Item`.
        // TG_STONE: not translated, it uses Minecraft's `Item`.
        // TG_PODZOL: not translated, it uses Minecraft's `Item`.
        // DARK_IRON_BARS: not translated, it uses Minecraft's `Item`.
        // SOUL_FIRE_BRAZIER: not translated, it uses Minecraft's `Item`.
        // FIRE_BRAZIER: not translated, it uses Minecraft's `Item`.
        // PEDESTAL: not translated, it uses Minecraft's `Item`.
        // CANDLE_HOLDER: not translated, it uses Minecraft's `Item`.
        // DARK_IRON_TRAPDOOR: not translated, it uses Minecraft's `Item`.
        // DARK_IRON_DOOR: not translated, it uses Minecraft's `Item`.
        // DARK_IRON_BLOCK: not translated, it uses Minecraft's `Item`.
        // SKULL_WITH_RIB_CAGE: not translated, it uses Minecraft's `Item`.
        // LEANING_SKELETON: not translated, it uses Minecraft's `Item`.
        // SKULL_PILE: not translated, it uses Minecraft's `Item`.
        // LYING_SKELETON: not translated, it uses Minecraft's `Item`.
        // WITHER_SKULL_WITH_RIB_CAGE: not translated, it uses Minecraft's `Item`.
        // LEANING_WITHER_SKELETON: not translated, it uses Minecraft's `Item`.
        // WITHER_SKULL_PILE: not translated, it uses Minecraft's `Item`.
        // LYING_WITHER_SKELETON: not translated, it uses Minecraft's `Item`.
        // CREEPER_SKELETON: not translated, it uses Minecraft's `Item`.
        // SKELETON_HAND: not translated, it uses Minecraft's `Item`.
        // WITHER_SKELETON_HAND: not translated, it uses Minecraft's `Item`.
        // HANGED_SKELETON: not translated, it uses Minecraft's `Item`.
        // HANGED_WITHER_SKELETON: not translated, it uses Minecraft's `Item`.
        // SEATED_FULL_SKELETON: not translated, it uses Minecraft's `Item`.
        // SEATED_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Item`.
        // SKELETON_SKULL_WITH_AXE: not translated, it uses Minecraft's `Item`.
        // WITHER_SKELETON_SKULL_WITH_AXE: not translated, it uses Minecraft's `Item`.
        // SWORD_IN_SKELETON: not translated, it uses Minecraft's `Item`.
        // SWORD_IN_WITHER_SKELETON: not translated, it uses Minecraft's `Item`.
        // SMALL_SKELETON_REMAINS: not translated, it uses Minecraft's `Item`.
        // SMALL_WITHER_SKELETON_REMAINS: not translated, it uses Minecraft's `Item`.
        // HANGED_FULL_SKELETON: not translated, it uses Minecraft's `Item`.
        // HANGED_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Item`.
        // LYING_FULL_SKELETON: not translated, it uses Minecraft's `Item`.
        // LYING_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Item`.
        // BACKSTABBED_FULL_SKELETON: not translated, it uses Minecraft's `Item`.
        // BACKSTABBED_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Item`.
        // SHOT_FULL_SKELETON: not translated, it uses Minecraft's `Item`.
        // SHOT_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Item`.
        // SIDEWAYS_FULL_SKELETON: not translated, it uses Minecraft's `Item`.
        // SIDEWAYS_FULL_WITHER_SKELETON: not translated, it uses Minecraft's `Item`.
        // VASE_BLOCK: not translated, it uses Minecraft's `Item`.
        // GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // COBBLESTONE_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // MOSSY_COBBLESTONE_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // DEEPSLATE_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // BLACKSTONE_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // CRACKED_BLACKSTONE_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // STONE_BRICKS_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // MOSSY_STONE_BRICKS_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // BRICKS_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // GILDED_BLACKSTONE_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // SANDSTONE_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // RED_SANDSTONE_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // QUARTZ_BRICKS_GRAVESTONE: not translated, it uses Minecraft's `Item`.
        // BLACK_URN: not translated, it uses Minecraft's `Item`.
        // BROWN_URN: not translated, it uses Minecraft's `Item`.
        // WHITE_URN: not translated, it uses Minecraft's `Item`.
        // GRAY_URN: not translated, it uses Minecraft's `Item`.
        // LIGHT_GRAY_URN: not translated, it uses Minecraft's `Item`.
        // BLUE_URN: not translated, it uses Minecraft's `Item`.
        // CYAN_URN: not translated, it uses Minecraft's `Item`.
        // LIGHT_BLUE_URN: not translated, it uses Minecraft's `Item`.
        // RED_URN: not translated, it uses Minecraft's `Item`.
        // ORANGE_URN: not translated, it uses Minecraft's `Item`.
        // YELLOW_URN: not translated, it uses Minecraft's `Item`.
        // PINK_URN: not translated, it uses Minecraft's `Item`.
        // MAGENTA_URN: not translated, it uses Minecraft's `Item`.
        // PURPLE_URN: not translated, it uses Minecraft's `Item`.
        // GREEN_URN: not translated, it uses Minecraft's `Item`.
        // LIME_URN: not translated, it uses Minecraft's `Item`.
        // TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // BLACK_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // BLUE_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // BROWN_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // CYAN_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // GRAY_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // GREEN_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // LIGHT_BLUE_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // LIGHT_GRAY_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // LIME_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // MAGENTA_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // ORANGE_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // PINK_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // PURPLE_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // RED_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // WHITE_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // YELLOW_TERRACOTTA_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_BLACK_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_BROWN_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_WHITE_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_GRAY_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_LIGHT_GRAY_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_BLUE_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_CYAN_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_LIGHT_BLUE_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_RED_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_ORANGE_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_YELLOW_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_PINK_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_MAGENTA_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_PURPLE_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_GREEN_URN: not translated, it uses Minecraft's `Item`.
        // SMALL_LIME_URN: not translated, it uses Minecraft's `Item`.
        // SARCOPHAGUS: not translated, it uses Minecraft's `Item`.
        // OAK_COFFIN: not translated, it uses Minecraft's `Item`.
        // SPRUCE_COFFIN: not translated, it uses Minecraft's `Item`.
        // BIRCH_COFFIN: not translated, it uses Minecraft's `Item`.
        // DARK_OAK_COFFIN: not translated, it uses Minecraft's `Item`.
        // JUNGLE_COFFIN: not translated, it uses Minecraft's `Item`.
        // ACACIA_COFFIN: not translated, it uses Minecraft's `Item`.
        // WARPED_COFFIN: not translated, it uses Minecraft's `Item`.
        // CRIMSON_COFFIN: not translated, it uses Minecraft's `Item`.
        // MANGROVE_COFFIN: not translated, it uses Minecraft's `Item`.
        // BAMBOO_COFFIN: not translated, it uses Minecraft's `Item`.
        // CHERRY_COFFIN: not translated, it uses Minecraft's `Item`.
        // PALE_OAK_COFFIN: not translated, it uses Minecraft's `Item`.
        // POPLAR_COFFIN: not translated, it uses Minecraft's `Item`.
        // VIAL_OF_BLOOD: not translated, it uses Minecraft's `Item`.
        // TORSO_PILE: not translated, it uses Minecraft's `Item`.
        // WITHER_TORSO_PILE: not translated, it uses Minecraft's `Item`.
        // SKULL_ON_PIKE: not translated, it uses Minecraft's `Item`.
        // WITHER_SKULL_ON_PIKE: not translated, it uses Minecraft's `Item`.
        // BONE_REMAINS: not translated, it uses Minecraft's `Item`.
        // WITHER_BONE_REMAINS: not translated, it uses Minecraft's `Item`.
        // LATERALLY_LYING_SKELETON: not translated, it uses Minecraft's `Item`.
        // LATERALLY_LYING_WITHER_SKELETON: not translated, it uses Minecraft's `Item`.
        // WHITE_BONE_STAFF: not translated, it uses Minecraft's `Item`.
        // BLACK_BONE_STAFF: not translated, it uses Minecraft's `Item`.
        // RED_BONE_STAFF: not translated, it uses Minecraft's `Item`.
        // CYAN_BONE_STAFF: not translated, it uses Minecraft's `Item`.
        // PURPLE_BONE_STAFF: not translated, it uses Minecraft's `Item`.
        // ALTAR: not translated, it uses Minecraft's `Item`.
        // ALTAR_SIDE: not translated, it uses Minecraft's `Item`.
        // ALTAR_CORNER: not translated, it uses Minecraft's `Item`.
        // ALTAR_CENTER: not translated, it uses Minecraft's `Item`.
        // UPPER_BONE_STAFF: not translated, it uses Minecraft's `Item`.
        // MIDDLE_BONE_STAFF: not translated, it uses Minecraft's `Item`.
        // LOWER_BONE_STAFF: not translated, it uses Minecraft's `Item`.
        #[derive(Clone, Debug)]
        pub struct TGItems {
        }
        impl TGItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
        // TGItems.addItemsToGroup(0): not translated, it uses Minecraft's `ResourceKey`.
        // TGItems.vialStages(0): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `TGLootConditions` (com/lion/graveyard/init/TGLootConditions.java)
    pub mod tg_loot_conditions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TGLootConditions {
        }
        impl TGLootConditions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TGLootConditions.init(0): not translated, it uses Minecraft's `MapCodec`.
    }

    /// `TGOverrunVillage` (com/lion/graveyard/init/TGOverrunVillage.java)
    pub mod tg_overrun_village {
        use super::*;
        // CHARRED_PLANKS: not translated, it uses Minecraft's `Block`.
        // CHARRED_LOG: not translated, it uses Minecraft's `Block`.
        // SCORCHED_PLANKS: not translated, it uses Minecraft's `Block`.
        // SCORCHED_LOG: not translated, it uses Minecraft's `Block`.
        // SMOLDERING_DEBRIS: not translated, it uses Minecraft's `Block`.
        // CHARRED_STAIRS: not translated, it uses Minecraft's `Block`.
        // CHARRED_SLAB: not translated, it uses Minecraft's `Block`.
        // CHARRED_FENCE: not translated, it uses Minecraft's `Block`.
        // CHARRED_FENCE_GATE: not translated, it uses Minecraft's `Block`.
        // CHARRED_DOOR: not translated, it uses Minecraft's `Block`.
        // CHARRED_TRAPDOOR: not translated, it uses Minecraft's `Block`.
        // SCORCHED_STAIRS: not translated, it uses Minecraft's `Block`.
        // SCORCHED_SLAB: not translated, it uses Minecraft's `Block`.
        // SCORCHED_FENCE: not translated, it uses Minecraft's `Block`.
        // SCORCHED_FENCE_GATE: not translated, it uses Minecraft's `Block`.
        // SCORCHED_DOOR: not translated, it uses Minecraft's `Block`.
        // SCORCHED_TRAPDOOR: not translated, it uses Minecraft's `Block`.
        // CHARRED_WOOD_BLOCK: not translated, it uses Minecraft's `Block`.
        // STRIPPED_CHARRED_LOG: not translated, it uses Minecraft's `Block`.
        // STRIPPED_CHARRED_WOOD: not translated, it uses Minecraft's `Block`.
        // SCORCHED_WOOD_BLOCK: not translated, it uses Minecraft's `Block`.
        // STRIPPED_SCORCHED_LOG: not translated, it uses Minecraft's `Block`.
        // STRIPPED_SCORCHED_WOOD: not translated, it uses Minecraft's `Block`.
        // CHARRED_WOOD: not translated, it uses Minecraft's `Block`.
        // SCORCHED_WOOD: not translated, it uses Minecraft's `Block`.
        // CHARRED_PLANKS_ITEM: not translated, it uses Minecraft's `Item`.
        // CHARRED_LOG_ITEM: not translated, it uses Minecraft's `Item`.
        // SCORCHED_PLANKS_ITEM: not translated, it uses Minecraft's `Item`.
        // SCORCHED_LOG_ITEM: not translated, it uses Minecraft's `Item`.
        // CHARRED_WOOD_BLOCK_ITEM: not translated, it uses Minecraft's `Item`.
        // STRIPPED_CHARRED_LOG_ITEM: not translated, it uses Minecraft's `Item`.
        // STRIPPED_CHARRED_WOOD_ITEM: not translated, it uses Minecraft's `Item`.
        // SCORCHED_WOOD_BLOCK_ITEM: not translated, it uses Minecraft's `Item`.
        // STRIPPED_SCORCHED_LOG_ITEM: not translated, it uses Minecraft's `Item`.
        // STRIPPED_SCORCHED_WOOD_ITEM: not translated, it uses Minecraft's `Item`.
        // SMOLDERING_DEBRIS_ITEM: not translated, it uses Minecraft's `Item`.
        // CHARRED_STAIRS_ITEM: not translated, it uses Minecraft's `Item`.
        // CHARRED_SLAB_ITEM: not translated, it uses Minecraft's `Item`.
        // CHARRED_FENCE_ITEM: not translated, it uses Minecraft's `Item`.
        // CHARRED_FENCE_GATE_ITEM: not translated, it uses Minecraft's `Item`.
        // CHARRED_DOOR_ITEM: not translated, it uses Minecraft's `Item`.
        // CHARRED_TRAPDOOR_ITEM: not translated, it uses Minecraft's `Item`.
        // SCORCHED_STAIRS_ITEM: not translated, it uses Minecraft's `Item`.
        // SCORCHED_SLAB_ITEM: not translated, it uses Minecraft's `Item`.
        // SCORCHED_FENCE_ITEM: not translated, it uses Minecraft's `Item`.
        // SCORCHED_FENCE_GATE_ITEM: not translated, it uses Minecraft's `Item`.
        // SCORCHED_DOOR_ITEM: not translated, it uses Minecraft's `Item`.
        // SCORCHED_TRAPDOOR_ITEM: not translated, it uses Minecraft's `Item`.
        // SMOLDERING_DEBRIS_BLOCK_ENTITY: not translated, it uses Minecraft's `BlockEntityType`.
        #[derive(Clone, Debug)]
        pub struct TGOverrunVillage {
        }
        impl TGOverrunVillage {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TGOverrunVillage.wood(1): not translated, it uses Minecraft's `MapColor`.
        pub fn init() {
        }
        // TGOverrunVillage.addItemsToGroup(0): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `TGParticles` (com/lion/graveyard/init/TGParticles.java)
    pub mod tg_particles {
        use super::*;
        // GRAVEYARD_FOG_PARTICLE: not translated, it uses Minecraft's `SimpleParticleType`.
        // GRAVEYARD_SOUL_PARTICLE: not translated, it uses Minecraft's `SimpleParticleType`.
        // GRAVEYARD_HAND_PARTICLE: not translated, it uses Minecraft's `SimpleParticleType`.
        // GRAVEYARD_LEFT_HAND_PARTICLE: not translated, it uses Minecraft's `SimpleParticleType`.
        // GRAVEYARD_SOUL_BEAM_PARTICLE: not translated, it uses Minecraft's `SimpleParticleType`.
        #[derive(Clone, Debug)]
        pub struct TGParticles {
        }
        impl TGParticles {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
    }

    /// `TGProcessors` (com/lion/graveyard/init/TGProcessors.java)
    pub mod tg_processors {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TGProcessors {
        }
        impl TGProcessors {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
    }

    /// `TGRecipeTypes` (com/lion/graveyard/init/TGRecipeTypes.java)
    pub mod tg_recipe_types {
        use super::*;
        // OSSUARY_CARVING: not translated, it uses Minecraft's `RecipeType`.
        // OSSUARY_CARVING_SERIALIZER: not translated, it uses Minecraft's `RecipeSerializer`.
        #[derive(Clone, Debug)]
        pub struct TGRecipeTypes {
        }
        impl TGRecipeTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
    }

    /// `TGRecipeTypes.Type` (com/lion/graveyard/init/TGRecipeTypes.java)
    pub mod tg_recipe_types_type {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RecipeType`. Its static parts are below.
        // INSTANCE: not translated, it uses Type, which isn't translated (it implements Minecraft's `RecipeType`).
        // TGRecipeTypes.Type.Type(0): not translated, its class isn't translated (it implements Minecraft's `RecipeType`).
    }

    /// `TGScreens` (com/lion/graveyard/init/TGScreens.java)
    pub mod tg_screens {
        use super::*;
        // OSSUARY_SCREEN_HANDLER: not translated, it uses Minecraft's `MenuType`.
        #[derive(Clone, Debug)]
        pub struct TGScreens {
        }
        impl TGScreens {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
    }

    /// `TGSounds` (com/lion/graveyard/init/TGSounds.java)
    pub mod tg_sounds {
        use super::*;
        // LICH_SPAWN: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_MELEE: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_PHASE_02: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_PHASE_03: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_CORPSE_SPELL: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_CAST_SKULL: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_CAST_LEVITATION: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_CAST_TELEPORT: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_DEATH: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_HUNT: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_SCARE: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_PHASE_03_ATTACK: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_IDLE: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_HURT: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_BONE_CRACK: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_STUNNED: not translated, it uses Minecraft's `SoundEvent`.
        // NAMELESS_HANGED_BREATH: not translated, it uses Minecraft's `SoundEvent`.
        // NAMELESS_HANGED_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // NAMELESS_HANGED_INTERACT: not translated, it uses Minecraft's `SoundEvent`.
        // CORRUPTED_ILLAGER_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // CORRUPTED_ILLAGER_HURT: not translated, it uses Minecraft's `SoundEvent`.
        // CORRUPTED_ILLAGER_STRIKE: not translated, it uses Minecraft's `SoundEvent`.
        // CORRUPTED_ILLAGER_BITE: not translated, it uses Minecraft's `SoundEvent`.
        // CORRUPTED_ILLAGER_DEATH: not translated, it uses Minecraft's `SoundEvent`.
        // CORRUPTED_ILLAGER_STEP: not translated, it uses Minecraft's `SoundEvent`.
        // DIG: not translated, it uses Minecraft's `SoundEvent`.
        // CORRUPTED_ILLAGER_SPAWN: not translated, it uses Minecraft's `SoundEvent`.
        // CORRUPTED_ILLAGER_IDLE: not translated, it uses Minecraft's `SoundEvent`.
        // CORRUPTED_ILLAGER_WANDER: not translated, it uses Minecraft's `SoundEvent`.
        // CORRUPTED_ILLAGER_ATTACK_RUN: not translated, it uses Minecraft's `SoundEvent`.
        // ACOLYTE_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // ACOLYTE_HURT: not translated, it uses Minecraft's `SoundEvent`.
        // ACOLYTE_DEATH: not translated, it uses Minecraft's `SoundEvent`.
        // REAPER_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // REAPER_HURT: not translated, it uses Minecraft's `SoundEvent`.
        // REAPER_DEATH: not translated, it uses Minecraft's `SoundEvent`.
        // REAPER_CHARGE: not translated, it uses Minecraft's `SoundEvent`.
        // GHOUL_ROAR: not translated, it uses Minecraft's `SoundEvent`.
        // GHOUL_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // GHOUL_HURT: not translated, it uses Minecraft's `SoundEvent`.
        // GHOUL_DEATH: not translated, it uses Minecraft's `SoundEvent`.
        // GHOUL_STEP: not translated, it uses Minecraft's `SoundEvent`.
        // GHOULING_GROAN: not translated, it uses Minecraft's `SoundEvent`.
        // GHOULING_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // GHOULING_HURT: not translated, it uses Minecraft's `SoundEvent`.
        // GHOULING_DEATH: not translated, it uses Minecraft's `SoundEvent`.
        // GHOULING_ATTACK: not translated, it uses Minecraft's `SoundEvent`.
        // GHOULING_STEP: not translated, it uses Minecraft's `SoundEvent`.
        // GHOULING_SPAWN: not translated, it uses Minecraft's `SoundEvent`.
        // GHOULING_SOUL_DRAW: not translated, it uses Minecraft's `SoundEvent`.
        // REVENANT_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // REVENANT_HURT: not translated, it uses Minecraft's `SoundEvent`.
        // REVENANT_DEATH: not translated, it uses Minecraft's `SoundEvent`.
        // REVENANT_STEP: not translated, it uses Minecraft's `SoundEvent`.
        // NIGHTMARE_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // NIGHTMARE_HURT: not translated, it uses Minecraft's `SoundEvent`.
        // NIGHTMARE_DEATH: not translated, it uses Minecraft's `SoundEvent`.
        // WRAITH_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // WRAITH_HURT: not translated, it uses Minecraft's `SoundEvent`.
        // ALTAR_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // COFFIN_OPEN: not translated, it uses Minecraft's `SoundEvent`.
        // COFFIN_CLOSE: not translated, it uses Minecraft's `SoundEvent`.
        // URN_OPEN: not translated, it uses Minecraft's `SoundEvent`.
        // URN_CLOSE: not translated, it uses Minecraft's `SoundEvent`.
        // OSSUARY_OPEN: not translated, it uses Minecraft's `SoundEvent`.
        // SARCOPHAGUS_USE: not translated, it uses Minecraft's `SoundEvent`.
        // BONE_PLACED: not translated, it uses Minecraft's `SoundEvent`.
        // BONE_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // CROW_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // CROW_ALARM: not translated, it uses Minecraft's `SoundEvent`.
        // RAVEN_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // MAGPIE_AMBIENT: not translated, it uses Minecraft's `SoundEvent`.
        // CORVID_HURT: not translated, it uses Minecraft's `SoundEvent`.
        // CORVID_DEATH: not translated, it uses Minecraft's `SoundEvent`.
        // CORVID_TAKEOFF: not translated, it uses Minecraft's `SoundEvent`.
        // CORVID_FLY: not translated, it uses Minecraft's `SoundEvent`.
        // CORVID_LAND: not translated, it uses Minecraft's `SoundEvent`.
        // TORMENTED_VILLAGER_IDLE: not translated, it uses Minecraft's `SoundEvent`.
        // TORMENTED_VILLAGER_LOST: not translated, it uses Minecraft's `SoundEvent`.
        // TORMENTED_VILLAGER_CRYING: not translated, it uses Minecraft's `SoundEvent`.
        // TORMENTED_VILLAGER_GLITCH: not translated, it uses Minecraft's `SoundEvent`.
        // TORMENTED_VILLAGER_RAGE: not translated, it uses Minecraft's `SoundEvent`.
        // TORMENTED_VILLAGER_BREAKDOWN: not translated, it uses Minecraft's `SoundEvent`.
        // TORMENTED_VILLAGER_SPAWN: not translated, it uses Minecraft's `SoundEvent`.
        // TORMENTED_VILLAGER_REDEMPTION: not translated, it uses Minecraft's `SoundEvent`.
        // VIAL_SPLASH: not translated, it uses Minecraft's `SoundEvent`.
        // LICH_THEME_01: not translated, it uses Minecraft's `SoundEvent`.
        #[derive(Clone, Debug)]
        pub struct TGSounds {
        }
        impl TGSounds {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
        // TGSounds.register(1): not translated, it uses Minecraft's `SoundEvent`.
    }

    /// `TGStructures` (com/lion/graveyard/init/TGStructures.java)
    pub mod tg_structures {
        use super::*;
        // TG_JIGSAW: not translated, it uses Minecraft's `StructureType`.
        // ALTAR: not translated, it uses Minecraft's `StructureType`.
        // DEAD_TREE: not translated, it uses Minecraft's `StructureType`.
        // CRYPT: not translated, it uses Minecraft's `StructureType`.
        // GIANT_MUSHROOM: not translated, it uses Minecraft's `StructureType`.
        // HAUNTED_HOUSE: not translated, it uses Minecraft's `StructureType`.
        // LARGE_GRAVEYARD: not translated, it uses Minecraft's `StructureType`.
        // LICH_PRISON: not translated, it uses Minecraft's `StructureType`.
        // MEDIUM_GRAVEYARD: not translated, it uses Minecraft's `StructureType`.
        // MEMORIAL_TREE: not translated, it uses Minecraft's `StructureType`.
        // MUSHROOM_GRAVE: not translated, it uses Minecraft's `StructureType`.
        // RUINS: not translated, it uses Minecraft's `StructureType`.
        // SMALL_DESERT_GRAVE: not translated, it uses Minecraft's `StructureType`.
        // SMALL_DESERT_GRAVEYARD: not translated, it uses Minecraft's `StructureType`.
        // SMALL_GRAVE: not translated, it uses Minecraft's `StructureType`.
        // SMALL_GRAVEYARD: not translated, it uses Minecraft's `StructureType`.
        // SMALL_MOUNTAIN_GRAVE: not translated, it uses Minecraft's `StructureType`.
        // SMALL_SAVANNA_GRAVE: not translated, it uses Minecraft's `StructureType`.
        #[derive(Clone, Debug)]
        pub struct TGStructures {
        }
        impl TGStructures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
    }

    /// `TGTags` (com/lion/graveyard/init/TGTags.java)
    pub mod tg_tags {
        use super::*;
        // GHOUL_SPAWNS: not translated, it uses Minecraft's `TagKey`.
        // REVENANT_SPAWNS: not translated, it uses Minecraft's `TagKey`.
        // NIGHTMARE_SPAWNS: not translated, it uses Minecraft's `TagKey`.
        // SKELETON_CREEPER_SPAWNS: not translated, it uses Minecraft's `TagKey`.
        // REAPER_SPAWNS: not translated, it uses Minecraft's `TagKey`.
        // RAVEN_SPAWNS: not translated, it uses Minecraft's `TagKey`.
        // MAGPIE_SPAWNS: not translated, it uses Minecraft's `TagKey`.
        // CROW_SPAWNS: not translated, it uses Minecraft's `TagKey`.
        // NATURAL_HELPER_BLOCKS: not translated, it uses Minecraft's `TagKey`.
        // CORVID_PERCHES: not translated, it uses Minecraft's `TagKey`.
        // CORVID_BONES: not translated, it uses Minecraft's `TagKey`.
        // CROW_SETTLE: not translated, it uses Minecraft's `TagKey`.
        // CROW_LANDMARKS: not translated, it uses Minecraft's `TagKey`.
        // CROW_REPELLENTS: not translated, it uses Minecraft's `TagKey`.
        // SCARECROW_HEADS: not translated, it uses Minecraft's `TagKey`.
        // SCARECROW_BODIES: not translated, it uses Minecraft's `TagKey`.
        // MAGPIE_TREASURES: not translated, it uses Minecraft's `TagKey`.
        // SKELETON_DECORATION: not translated, it uses Minecraft's `TagKey`.
        // ON_RUINS_EXPLORER_MAPS: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct TGTags {
        }
        impl TGTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `BoneStaffItem` (com/lion/graveyard/item/BoneStaffItem.java)
    pub mod bone_staff_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        pub const FETCH_DISTANCE_SQ: f64 = 576.0f64;
        pub const USE_COOLDOWN_TICKS: i32 = 10;
        // FETCH_TICKET: not translated, it uses Minecraft's `TicketType`.
        pub const MAX_FETCH_MISSES: i32 = 5;
        pub const MAX_LANDING_DROP: i32 = 8;
        pub const CALL_TIMEOUT: i32 = 200;
        pub const ITEM_AIM_DISTANCE: f64 = 24.0f64;
        pub const ITEM_AIM_MARGIN: f64 = 0.3f64;
        pub const STORE_TICKS: i32 = 30;
        pub const STORE_RANGE: f64 = 12.0f64;
        pub const STORE_RANGE_SQ: f64 = 144.0f64;
        pub const DRAW_SOUND_DELAY: i32 = 4;
        pub const SIT_CLICK_TICKS: i32 = 8;
        pub const MAX_STORED_NBT: i64 = 1500000i64;
        // DRAWING: not translated, it uses `UUID` (java.util).
        // STORE_DONE_SOUND: not translated, it uses Minecraft's `SoundEvent`.
        // BoneStaffItem.BoneStaffItem(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // BoneStaffItem.useOn(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // BoneStaffItem.spawnPosition(3): not translated, it uses Minecraft's `BlockPos`.
        // BoneStaffItem.clipAtBlocks(3): not translated, it uses Minecraft's `Vec3`.
        // BoneStaffItem.isBoundTo(2): not translated, it uses Minecraft's `ItemStack`.
        // BoneStaffItem.startDrawing(2): not translated, it uses Minecraft's `Player`.
        // BoneStaffItem.stopDrawSound(2): not translated, it uses Minecraft's `ServerLevel`.
        // BoneStaffItem.streamSouls(4): not translated, it uses Minecraft's `ServerLevel`.
        // BoneStaffItem.release(4): not translated, it uses Minecraft's `ServerLevel`.
        // BoneStaffItem.store(4): not translated, it uses Minecraft's `ServerLevel`.
        // BoneStaffItem.fitsInStack(1): not translated, it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`).
        // BoneStaffItem.getUseDuration(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // BoneStaffItem.getUseAnimation(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // BoneStaffItem.onUseTick(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // BoneStaffItem.releaseUsing(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // BoneStaffItem.isFoil(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // BoneStaffItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // BoneStaffItem.findLoaded(2): not translated, it uses Minecraft's `Entity`.
        // BoneStaffItem.isUnbound(1): not translated, it uses Minecraft's `ItemStack`.
        // BoneStaffItem.rebind(3): not translated, it uses Minecraft's `ItemStack`.
        // BoneStaffItem.rejectsStaff(3): not translated, it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`).
        // BoneStaffItem.ownerAirborne(1): not translated, it uses Minecraft's `Player`.
        // BoneStaffItem.isOwnedBy(2): not translated, it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`).
        // BoneStaffItem.isAlive(2): not translated, it uses Minecraft's `ServerLevel`.
        // BoneStaffItem.inventoryTick(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // BoneStaffItem.call(4): not translated, it uses Minecraft's `ServerLevel`.
        // BoneStaffItem.findLanding(3): not translated, it uses Minecraft's `Vec3`.
        // BoneStaffItem.fetch(3): not translated, it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`).
        // BoneStaffItem.findItemInSight(2): not translated, it uses Minecraft's `ItemEntity`.
        // BoneStaffItem.startFarDraw(4): not translated, it uses Minecraft's `Level`.
        // BoneStaffItem.use(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `DaggerItem` (com/lion/graveyard/item/DaggerItem.java)
    pub mod dagger_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // DaggerItem.DaggerItem(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // DaggerItem.getDestroySpeed(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // DaggerItem.hurtEnemy(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // DaggerItem.fillHotbarVial(1): not translated, it uses Minecraft's `Player`.
        // DaggerItem.convertHotbarBottle(1): not translated, it uses Minecraft's `Player`.
        // DaggerItem.giveVial(2): not translated, it uses Minecraft's `Player`.
    }

    /// `DiaryPageItem` (com/lion/graveyard/item/DiaryPageItem.java)
    pub mod diary_page_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // DiaryPageItem.DiaryPageItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // DiaryPageItem.of(2): not translated, it uses Minecraft's `ItemStack`.
        // DiaryPageItem.pageOf(1): not translated, it uses Minecraft's `ItemStack`.
        // DiaryPageItem.use(3): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // DiaryPageItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `LegacyItemData` (com/lion/graveyard/item/LegacyItemData.java)
    pub mod legacy_item_data {
        use super::*;
        pub const BLOOD: &str = "Blood";
        pub const PAGE: &str = "page";
        pub const GHOULING_UUID: &str = "GhoulingUUID";
        pub const OWNER_UUID: &str = "OwnerUUID";
        pub const BOND_ID: &str = "BondId";
        pub const FETCH_CALL: &str = "FetchCall";
        pub const FETCH_MISSES: &str = "FetchMisses";
        pub const STORED_GHOULING: &str = "StoredGhouling";
        // NONE: not translated, it uses Minecraft's `CompoundTag`.
        #[derive(Clone, Debug)]
        pub struct LegacyItemData {
        }
        impl LegacyItemData {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LegacyItemData.hasData(1): needs the game, so it's in game_side.rs.
        // LegacyItemData.view(1): not translated, it uses Minecraft's `CompoundTag`.
        // LegacyItemData.edit(2): not translated, it uses Minecraft's `ItemStack`.
        // LegacyItemData.uuid(2): not translated, it uses `UUID` (java.util).
        // LegacyItemData.ownerUuid(1): not translated, it uses `UUID` (java.util).
        // LegacyItemData.ghoulingUuid(1): not translated, it uses `UUID` (java.util).
        // LegacyItemData.bondId(1): not translated, it uses `UUID` (java.util).
        // LegacyItemData.fetchMisses(1): not translated, it uses Minecraft's `ItemStack`.
        // LegacyItemData.putUuid(3): not translated, it uses Minecraft's `CompoundTag`.
        // LegacyItemData.isStored(1): not translated, it uses Minecraft's `ItemStack`.
        // LegacyItemData.store(2): not translated, it uses Minecraft's `ItemStack`.
        // LegacyItemData.clearStored(1): not translated, it uses Minecraft's `ItemStack`.
        // LegacyItemData.clearBond(1): not translated, it uses Minecraft's `ItemStack`.
        // LegacyItemData.clearFetchCall(1): not translated, it uses Minecraft's `ItemStack`.
        // LegacyItemData.clearFetchMisses(1): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `SarcophagusItem` (com/lion/graveyard/item/SarcophagusItem.java)
    pub mod sarcophagus_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // SarcophagusItem.SarcophagusItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SarcophagusItem.placeBlock(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `TooltipBlockItem` (com/lion/graveyard/item/TooltipBlockItem.java)
    pub mod tooltip_block_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // TooltipBlockItem.TooltipBlockItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // TooltipBlockItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `VialOfBlood` (com/lion/graveyard/item/VialOfBlood.java)
    pub mod vial_of_blood {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // VialOfBlood.VialOfBlood(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // VialOfBlood.getBlood(1): not translated, it uses Minecraft's `ItemStack`.
        // VialOfBlood.setBlood(2): not translated, it uses Minecraft's `ItemStack`.
        // VialOfBlood.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `PagesInLootCondition` (com/lion/graveyard/loot/PagesInLootCondition.java)
    pub mod pages_in_loot_condition {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `LootItemCondition`. Its static parts are below.
        pub const CONFIG_KEY: &str = "chroniclePagesInLoot";
        // INSTANCE: not translated, it uses PagesInLootCondition, which isn't translated (it implements Minecraft's `LootItemCondition`).
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // PagesInLootCondition.PagesInLootCondition(0): not translated, its class isn't translated (it implements Minecraft's `LootItemCondition`).
        // PagesInLootCondition.codec(0): not translated, its class isn't translated (it implements Minecraft's `LootItemCondition`).
        // PagesInLootCondition.test(1): not translated, its class isn't translated (it implements Minecraft's `LootItemCondition`).
    }

    /// `GraveyardFogParticle` (com/lion/graveyard/particles/GraveyardFogParticle.java)
    pub mod graveyard_fog_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SingleQuadParticle`. Its static parts are below.
        pub const PEAK_ALPHA_MIN: f32 = 0.35f32;
        pub const PEAK_ALPHA_RANGE: f32 = 0.15f32;
        pub const FADE_IN_END: f32 = 0.25f32;
        pub const FADE_OUT_START: f32 = 0.65f32;
        pub const GROWTH: f32 = 0.4f32;
        pub static MAX_TILT: LazyLock<f32> = LazyLock::new(|| (3.0f64.to_radians() as f32));
        pub static FLAT_ROTATION: LazyLock<f32> = LazyLock::new(|| (((-std::f64::consts::PI) / 2.0f64) as f32));
        // GraveyardFogParticle.GraveyardFogParticle(5): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // GraveyardFogParticle.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // GraveyardFogParticle.tick(0): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
        // GraveyardFogParticle.extract(3): not translated, its class isn't translated (it extends Minecraft's `SingleQuadParticle`).
    }

    /// `GraveyardFogParticle.FogFactory` (com/lion/graveyard/particles/GraveyardFogParticle.java)
    pub mod graveyard_fog_particle_fog_factory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // GraveyardFogParticle.FogFactory.FogFactory(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // GraveyardFogParticle.FogFactory.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `GraveyardHandParticle` (com/lion/graveyard/particles/GraveyardHandParticle.java)
    pub mod graveyard_hand_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RisingParticle`. Its static parts are below.
        // GraveyardHandParticle.GraveyardHandParticle(8): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
        // GraveyardHandParticle.getLightCoords(1): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
        // GraveyardHandParticle.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
        // GraveyardHandParticle.tick(0): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
    }

    /// `GraveyardHandParticle.Factory` (com/lion/graveyard/particles/GraveyardHandParticle.java)
    pub mod graveyard_hand_particle_factory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // GraveyardHandParticle.Factory.Factory(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // GraveyardHandParticle.Factory.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `GraveyardLineFogParticle` (com/lion/graveyard/particles/GraveyardLineFogParticle.java)
    pub mod graveyard_line_fog_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RisingParticle`. Its static parts are below.
        // GraveyardLineFogParticle.GraveyardLineFogParticle(8): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
        // GraveyardLineFogParticle.getLightCoords(1): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
        // GraveyardLineFogParticle.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
        // GraveyardLineFogParticle.tick(0): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
    }

    /// `GraveyardLineFogParticle.Factory` (com/lion/graveyard/particles/GraveyardLineFogParticle.java)
    pub mod graveyard_line_fog_particle_factory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // GraveyardLineFogParticle.Factory.Factory(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // GraveyardLineFogParticle.Factory.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `GraveyardSoulParticle` (com/lion/graveyard/particles/GraveyardSoulParticle.java)
    pub mod graveyard_soul_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RisingParticle`. Its static parts are below.
        // GraveyardSoulParticle.GraveyardSoulParticle(8): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
        // GraveyardSoulParticle.getLightCoords(1): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
        // GraveyardSoulParticle.getLayer(0): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
        // GraveyardSoulParticle.tick(0): not translated, its class isn't translated (it extends Minecraft's `RisingParticle`).
    }

    /// `GraveyardSoulParticle.Factory` (com/lion/graveyard/particles/GraveyardSoulParticle.java)
    pub mod graveyard_soul_particle_factory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // GraveyardSoulParticle.Factory.Factory(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // GraveyardSoulParticle.Factory.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `GraveyardSoulParticle.SculkSoulFactory` (com/lion/graveyard/particles/GraveyardSoulParticle.java)
    pub mod graveyard_soul_particle_sculk_soul_factory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // GraveyardSoulParticle.SculkSoulFactory.SculkSoulFactory(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // GraveyardSoulParticle.SculkSoulFactory.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `BiomeModifications` (com/lion/graveyard/platform/BiomeModifications.java)
    pub mod biome_modifications {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BiomeModifications {
        }
        impl BiomeModifications {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BiomeModifications.addMobSpawn(6): not translated, it uses Minecraft's `TagKey`.
        // BiomeModifications.addPlacedFeature(3): not translated, it uses Minecraft's `TagKey`.
    }

    /// `ChronicleSync` (com/lion/graveyard/platform/ChronicleSync.java)
    pub mod chronicle_sync {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Type, which isn't translated (it implements Minecraft's `RecipeType`).
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ChronicleSync.ChronicleSync(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ChronicleSync.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ChronicleSync.send(3): not translated, it uses Minecraft's `ServerPlayer`.
        // ChronicleSync.write(2): not translated, it uses Minecraft's `FriendlyByteBuf`.
        // ChronicleSync.read(1): not translated, it uses ChronicleSync, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ChronicleSync.found(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ChronicleSync.newlyFound(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ConfigDirectory` (com/lion/graveyard/platform/ConfigDirectory.java)
    pub mod config_directory {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConfigDirectory {
        }
        impl ConfigDirectory {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn get_config_directory() -> std::path::PathBuf {
            return config_directory_impl::get_config_directory();
        }
    }

    /// `DevEnvironment` (com/lion/graveyard/platform/DevEnvironment.java)
    pub mod dev_environment {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DevEnvironment {
        }
        impl DevEnvironment {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn is_development() -> bool {
            return dev_environment_impl::is_development();
        }
    }

    /// `BiomeModificationsImpl` (com/lion/graveyard/platform/fabric/BiomeModificationsImpl.java)
    pub mod biome_modifications_impl {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BiomeModificationsImpl {
        }
        impl BiomeModificationsImpl {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BiomeModificationsImpl.addMobSpawn(6): not translated, it uses Minecraft's `TagKey`.
        // BiomeModificationsImpl.addPlacedFeature(3): not translated, it uses Minecraft's `TagKey`.
    }

    /// `ChronicleSyncImpl` (com/lion/graveyard/platform/fabric/ChronicleSyncImpl.java)
    pub mod chronicle_sync_impl {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ChronicleSyncImpl {
        }
        impl ChronicleSyncImpl {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ChronicleSyncImpl.send(3): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `ConfigDirectoryImpl` (com/lion/graveyard/platform/fabric/ConfigDirectoryImpl.java)
    pub mod config_directory_impl {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConfigDirectoryImpl {
        }
        impl ConfigDirectoryImpl {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn get_config_directory() -> std::path::PathBuf {
            return j_config_dir();
        }
    }

    /// `DevEnvironmentImpl` (com/lion/graveyard/platform/fabric/DevEnvironmentImpl.java)
    pub mod dev_environment_impl {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DevEnvironmentImpl {
        }
        impl DevEnvironmentImpl {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn is_development() -> bool {
            return false;
        }
    }

    /// `HordeSpawner` (com/lion/graveyard/platform/fabric/HordeSpawner.java)
    pub mod horde_spawner {
        use super::*;
        // spawners: not translated, it uses GraveyardHordeSpawner, which isn't translated (it implements Minecraft's `CustomSpawner`).
        pub static LOADED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct HordeSpawner {
        }
        impl HordeSpawner {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // HordeSpawner.getSpawners(0): not translated, it uses GraveyardHordeSpawner, which isn't translated (it implements Minecraft's `CustomSpawner`).
        // HordeSpawner.addToSpawners(2): not translated, it uses GraveyardHordeSpawner, which isn't translated (it implements Minecraft's `CustomSpawner`).
    }

    /// `HordeSpawner.OnWorldTick` (com/lion/graveyard/platform/fabric/HordeSpawner.java)
    pub mod horde_spawner_on_world_tick {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `EndLevelTick`. Its static parts are below.
        // HordeSpawner.OnWorldTick.onEndTick(1): not translated, its class isn't translated (it implements Fabric's `EndLevelTick`).
    }

    /// `HordeSpawner.ServerStopped` (com/lion/graveyard/platform/fabric/HordeSpawner.java)
    pub mod horde_spawner_server_stopped {
        use super::*;
        // The class itself isn't translated: it implements `ServerStopped`. Its static parts are below.
        // HordeSpawner.ServerStopped.onServerStopped(1): not translated, its class isn't translated (it implements `ServerStopped`).
    }

    /// `HordeSpawner.WorldLoad` (com/lion/graveyard/platform/fabric/HordeSpawner.java)
    pub mod horde_spawner_world_load {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `Load`. Its static parts are below.
        // HordeSpawner.WorldLoad.onLevelLoad(2): not translated, its class isn't translated (it implements Fabric's `Load`).
    }

    /// `NamelessHangedTradeOfferResourceListener` (com/lion/graveyard/platform/fabric/NamelessHangedTradeOfferResourceListener.java)
    pub mod nameless_hanged_trade_offer_resource_listener {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleJsonResourceReloadListener`. Its static parts are below.
        // NamelessHangedTradeOfferResourceListener.NamelessHangedTradeOfferResourceListener(0): not translated, its class isn't translated (it extends Minecraft's `SimpleJsonResourceReloadListener`).
        // NamelessHangedTradeOfferResourceListener.getFabricId(0): not translated, its class isn't translated (it extends Minecraft's `SimpleJsonResourceReloadListener`).
        // NamelessHangedTradeOfferResourceListener.prepare(2): not translated, its class isn't translated (it extends Minecraft's `SimpleJsonResourceReloadListener`).
        // NamelessHangedTradeOfferResourceListener.apply(3): not translated, its class isn't translated (it extends Minecraft's `SimpleJsonResourceReloadListener`).
    }

    /// `RegistryHelperImpl` (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java)
    pub mod registry_helper_impl {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RegistryHelperImpl {
        }
        impl RegistryHelperImpl {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RegistryHelperImpl.registerBlock(3): not translated, it's generic.
        // RegistryHelperImpl.registerItem(3): not translated, it's generic.
        // RegistryHelperImpl.registerItemGroup(4): not translated, it uses Minecraft's `ResourceKey`.
        // RegistryHelperImpl.registerItemGroupWithIcon(4): not translated, it uses Minecraft's `ResourceKey`.
        // RegistryHelperImpl.addToItemGroup(2): not translated, it uses Minecraft's `ResourceKey`.
        // RegistryHelperImpl.addStacksToItemGroup(2): not translated, it uses Minecraft's `ResourceKey`.
        // RegistryHelperImpl.registerSpawnEggItem(3): not translated, it's generic.
        // RegistryHelperImpl.registerMusicDiscItem(2): not translated, it's generic.
        // RegistryHelperImpl.registerBlockEntity(2): not translated, it's generic.
        // RegistryHelperImpl.registerStructureType(2): not translated, it's generic.
        // RegistryHelperImpl.registerParticleType(2): not translated, it uses Minecraft's `SimpleParticleType`.
        // RegistryHelperImpl.registerStructureProcessor(2): not translated, it uses Minecraft's `MapCodec`.
        // RegistryHelperImpl.registerCriterion(2): not translated, it uses Minecraft's `CriterionTrigger`.
        // RegistryHelperImpl.registerDataComponent(2): not translated, it's generic.
        // RegistryHelperImpl.registerSpawnPlacement(4): not translated, it's generic.
        // RegistryHelperImpl.registerScreenHandlerType(2): not translated, it uses Minecraft's `MenuType`.
        // RegistryHelperImpl.registerSoundEvent(2): not translated, it's generic.
        // RegistryHelperImpl.registerEntityRenderer(2): not translated, it's generic.
        // RegistryHelperImpl.registerBlockEntityRenderer(2): not translated, it's generic.
        // RegistryHelperImpl.registerEntityType(2): not translated, it's generic.
        // RegistryHelperImpl.registerEntityModelLayer(2): not translated, it uses Minecraft's `ModelLayerLocation`.
        // RegistryHelperImpl.registerEntityAttribute(2): not translated, it uses Minecraft's `EntityType`.
        // RegistryHelperImpl.registerRecipeType(2): not translated, it's generic.
        // RegistryHelperImpl.registerRecipeSerializer(2): not translated, it's generic.
        // RegistryHelperImpl.registerLootCondition(2): not translated, it's generic.
    }

    /// `ServerEvents` (com/lion/graveyard/platform/fabric/ServerEvents.java)
    pub mod server_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ServerEvents {
        }
        impl ServerEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ServerEvents.DeleteOldConfigs` (com/lion/graveyard/platform/fabric/ServerEvents.java)
    pub mod server_events_delete_old_configs {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ServerStarted`. Its static parts are below.
        // ServerEvents.DeleteOldConfigs.onServerStarted(1): not translated, its class isn't translated (it implements Fabric's `ServerStarted`).
    }

    /// `OssuaryRecipesSync` (com/lion/graveyard/platform/OssuaryRecipesSync.java)
    pub mod ossuary_recipes_sync {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Type, which isn't translated (it implements Minecraft's `RecipeType`).
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // OssuaryRecipesSync.OssuaryRecipesSync(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // OssuaryRecipesSync.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // OssuaryRecipesSync.send(3): not translated, it uses Minecraft's `ServerPlayer`.
        // OssuaryRecipesSync.containerId(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // OssuaryRecipesSync.results(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `RegistryHelper` (com/lion/graveyard/platform/RegistryHelper.java)
    pub mod registry_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RegistryHelper {
        }
        impl RegistryHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RegistryHelper.registerBlock(3): not translated, it's generic.
        // RegistryHelper.registerBlockEntity(2): not translated, it's generic.
        // RegistryHelper.registerItemGroup(4): not translated, it uses Minecraft's `ResourceKey`.
        // RegistryHelper.registerItem(3): not translated, it's generic.
        // RegistryHelper.registerItemGroupWithIcon(4): not translated, it uses Minecraft's `ResourceKey`.
        // RegistryHelper.addToItemGroup(2): not translated, it uses Minecraft's `ResourceKey`.
        // RegistryHelper.addStacksToItemGroup(2): not translated, it uses Minecraft's `ResourceKey`.
        // RegistryHelper.registerSpawnEggItem(3): not translated, it's generic.
        // RegistryHelper.registerMusicDiscItem(2): not translated, it's generic.
        // RegistryHelper.registerStructureType(2): not translated, it's generic.
        // RegistryHelper.registerParticleType(2): not translated, it uses Minecraft's `SimpleParticleType`.
        // RegistryHelper.registerStructureProcessor(2): not translated, it uses Minecraft's `MapCodec`.
        // RegistryHelper.registerCriterion(2): not translated, it uses Minecraft's `CriterionTrigger`.
        // RegistryHelper.registerScreenHandlerType(2): not translated, it uses Minecraft's `MenuType`.
        // RegistryHelper.registerSoundEvent(2): not translated, it's generic.
        // RegistryHelper.registerEntityType(2): not translated, it's generic.
        // RegistryHelper.registerEntityAttribute(2): not translated, it uses Minecraft's `EntityType`.
        // RegistryHelper.registerEntityModelLayer(2): not translated, it uses Minecraft's `ModelLayerLocation`.
        // RegistryHelper.registerEntityRenderer(2): not translated, it's generic.
        // RegistryHelper.registerBlockEntityRenderer(2): not translated, it's generic.
        // RegistryHelper.registerRecipeType(2): not translated, it's generic.
        // RegistryHelper.registerRecipeSerializer(2): not translated, it's generic.
        // RegistryHelper.registerDataComponent(2): not translated, it's generic.
        // RegistryHelper.registerSpawnPlacement(4): not translated, it's generic.
        // RegistryHelper.registerLootCondition(2): not translated, it's generic.
    }

    /// `CarvingRecipe` (com/lion/graveyard/recipe/CarvingRecipe.java)
    pub mod carving_recipe {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Recipe`. Its static parts are below.
        // INGREDIENT_CODEC: not translated, it uses Minecraft's `Codec`.
        // CarvingRecipe.CarvingRecipe(5): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CarvingRecipe.getType(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CarvingRecipe.getSerializer(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CarvingRecipe.group(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CarvingRecipe.placementInfo(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CarvingRecipe.recipeBookCategory(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CarvingRecipe.getOutputItem(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CarvingRecipe.getOutput(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CarvingRecipe.assemble(1): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CarvingRecipe.showNotification(0): not translated, its class isn't translated (it implements Minecraft's `Recipe`).
        // CarvingRecipe.notAir(1): not translated, it uses Minecraft's `DataResult`.
        // CarvingRecipe.serializer(1): not translated, it's generic.
    }

    /// `CarvingRecipe.LegacyIngredientCodec` (com/lion/graveyard/recipe/CarvingRecipe.java)
    pub mod carving_recipe_legacy_ingredient_codec {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Codec`. Its static parts are below.
        // CarvingRecipe.LegacyIngredientCodec.decode(2): not translated, it's generic.
        // CarvingRecipe.LegacyIngredientCodec.encode(3): not translated, it's generic.
        // CarvingRecipe.LegacyIngredientCodec.toStringForm(2): not translated, it's generic.
        // CarvingRecipe.LegacyIngredientCodec.fromObject(2): not translated, it's generic.
    }

    /// `CarvingRecipe.RecipeFactory` (com/lion/graveyard/recipe/CarvingRecipe.java)
    pub mod carving_recipe_recipe_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CarvingRecipe.RecipeFactory.create(3): not translated, it has no body.
    }

    /// `OssuaryRecipe` (com/lion/graveyard/recipe/OssuaryRecipe.java)
    pub mod ossuary_recipe {
        use super::*;
        // The class itself isn't translated: it extends `CarvingRecipe`. Its static parts are below.
        // OssuaryRecipe.OssuaryRecipe(3): not translated, its class isn't translated (it extends `CarvingRecipe`).
        // OssuaryRecipe.matches(2): not translated, its class isn't translated (it extends `CarvingRecipe`).
        // OssuaryRecipe.isSpecial(0): not translated, its class isn't translated (it extends `CarvingRecipe`).
    }

    /// `BossMusicPlayer` (com/lion/graveyard/sounds/BossMusicPlayer.java)
    pub mod boss_music_player {
        use super::*;
        // music: not translated, it uses LichBossMusic, which isn't translated (it extends Minecraft's `AbstractSoundInstance`).
        #[derive(Clone, Debug)]
        pub struct BossMusicPlayer {
        }
        impl BossMusicPlayer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BossMusicPlayer.playBossMusic(1): not translated, it uses LichEntity, which isn't translated (it extends Minecraft's `Monster`).
        // BossMusicPlayer.stopBossMusic(1): not translated, it uses LichEntity, which isn't translated (it extends Minecraft's `Monster`).
    }

    /// `LichBossMusic` (com/lion/graveyard/sounds/LichBossMusic.java)
    pub mod lich_boss_music {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractSoundInstance`. Its static parts are below.
        // LichBossMusic.LichBossMusic(3): not translated, its class isn't translated (it extends Minecraft's `AbstractSoundInstance`).
        // LichBossMusic.canPlaySound(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSoundInstance`).
        // LichBossMusic.isStopped(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSoundInstance`).
        // LichBossMusic.setDone(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSoundInstance`).
        // LichBossMusic.isAudibleBoss(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSoundInstance`).
        // LichBossMusic.tick(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSoundInstance`).
    }

    /// `NamelessHangedTrades` (com/lion/graveyard/trades/NamelessHangedTrades.java)
    pub mod nameless_hanged_trades {
        use super::*;
        pub const PROFESSION: &str = "graveyard:nameless_hanged";
        pub static PENDING_TRADES: LazyLock<JCell<Vec<JJson>>> = LazyLock::new(|| JCell::new(Vec::new()));
        // factories: not translated, it uses `TradeFactory` (com.lion.villagertradingplus.tradeoffers).
        #[derive(Clone, Debug)]
        pub struct NamelessHangedTrades {
        }
        impl NamelessHangedTrades {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NamelessHangedTrades.clear(0): not translated, it uses NamelessHangedTrades.factories, which isn't translated (it uses `TradeFactory` (com.lion.villagertradingplus.tradeoffers)).
        // NamelessHangedTrades.factories(0): not translated, it uses `TradeFactory` (com.lion.villagertradingplus.tradeoffers).
        // NamelessHangedTrades.deserializeJson(1): not translated, it calls Graveyard.getLogger, which isn't translated.
        // NamelessHangedTrades.parse(1): not translated, it uses `TradeFactory` (com.lion.villagertradingplus.tradeoffers).
    }

    /// `ChronicleCommand` (com/lion/graveyard/util/ChronicleCommand.java)
    pub mod chronicle_command {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ChronicleCommand {
        }
        impl ChronicleCommand {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ChronicleCommand.register(1): not translated, it uses Minecraft's `CommandDispatcher`.
    }

    /// `CreeperExplosionAccessor` (com/lion/graveyard/util/CreeperExplosionAccessor.java)
    pub mod creeper_explosion_accessor {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CreeperExplosionAccessor {
        }
        impl CreeperExplosionAccessor {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CreeperExplosionAccessor.explode(2): not translated, it uses Minecraft's `Creeper`.
    }

    /// `CrowAnchors` (com/lion/graveyard/util/CrowAnchors.java)
    pub mod crow_anchors {
        use super::*;
        pub const RADIUS: i32 = 32;
        pub const STEP: i32 = 2;
        pub const MIN_FIELD_HITS: i32 = 8;
        pub const MIN_LANDMARK_HITS: i32 = 2;
        pub const DEATH_MEMORY_TICKS: i32 = 2400;
        pub const MAX_DEATH_SITES: i32 = 64;
        pub const DEATH_SEARCH_RADIUS_SQ: i32 = 2304;
        // DEATH_SITES: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct CrowAnchors {
        }
        impl CrowAnchors {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CrowAnchors.recordDeath(2): not translated, it uses Minecraft's `Level`.
        // CrowAnchors.find(2): not translated, it uses Minecraft's `ServerLevel`.
        // CrowAnchors.findDeathSite(2): not translated, it uses Minecraft's `BlockPos`.
        // CrowAnchors.surface(3): not translated, it uses Minecraft's `BlockPos`.
    }

    /// `CrowAnchors.Anchor` (com/lion/graveyard/util/CrowAnchors.java)
    pub mod crow_anchors_anchor {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // CrowAnchors.Anchor.Anchor(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // CrowAnchors.Anchor.pos(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // CrowAnchors.Anchor.kind(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `CrowAnchors.DeathSite` (com/lion/graveyard/util/CrowAnchors.java)
    pub mod crow_anchors_death_site {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // CrowAnchors.DeathSite.DeathSite(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // CrowAnchors.DeathSite.pos(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // CrowAnchors.DeathSite.expiresAt(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `CrowAnchors.Kind` (com/lion/graveyard/util/CrowAnchors.java)
    pub mod crow_anchors_kind {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Kind {
            Death,
            Field,
            Landmark,
        }
        impl Kind {
            pub const VALUES: [Kind; 3] = [Kind::Death, Kind::Field, Kind::Landmark, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Kind::Death => "DEATH",
                    Kind::Field => "FIELD",
                    Kind::Landmark => "LANDMARK",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "DEATH" => Kind::Death,
                    "FIELD" => Kind::Field,
                    "LANDMARK" => Kind::Landmark,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `EntityChunkUpgrade` (com/lion/graveyard/util/EntityChunkUpgrade.java)
    pub mod entity_chunk_upgrade {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EntityChunkUpgrade {
        }
        impl EntityChunkUpgrade {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EntityChunkUpgrade.upgradeWithoutGraveyardEntities(3): not translated, it uses Minecraft's `CompoundTag`.
        // EntityChunkUpgrade.upgradeWithoutGraveyardEntities(4): not translated, it uses Minecraft's `CompoundTag`.
        // EntityChunkUpgrade.hasModEntity(1): not translated, it uses Minecraft's `ListTag`.
        // EntityChunkUpgrade.isModEntity(1): not translated, it uses Minecraft's `CompoundTag`.
    }

    /// `GravestoneIdentifier` (com/lion/graveyard/util/GravestoneIdentifier.java)
    pub mod gravestone_identifier {
        use super::*;
        pub static POLISHED_BASALT_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/polished_basalt.png"));
        pub static COBBLESTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/cobblestone.png"));
        pub static MOSSY_COBBLESTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/mossy_cobblestone.png"));
        pub static DEEPSLATE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/deepslate.png"));
        pub static ANCHOR_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/anchor.png"));
        pub static ANDESITE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/andesite.png"));
        pub static BLACKSTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/blackstone.png"));
        pub static BLACKSTONE_BRICKS_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/blackstone_bricks.png"));
        pub static BRICKS_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/bricks.png"));
        pub static CHISELED_BLACKSTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/chiseled_blackstone.png"));
        pub static CHISELED_QUARTZ_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/chiseled_quartz.png"));
        pub static CHISELED_RED_SANDSTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/chiseled_red_sandstone.png"));
        pub static CHISELED_SANDSTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/chiseled_sandstone.png"));
        pub static CRACKED_BLACKSTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/cracked_blackstone.png"));
        pub static CRACKED_STONE_BRICKS_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/cracked_stone_bricks.png"));
        pub static DARK_PRISMARINE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/dark_prismarine.png"));
        pub static DIORITE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/diorite.png"));
        pub static GILDED_BLACKSTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/gilded_blackstone.png"));
        pub static GRANITE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/granite.png"));
        pub static IRON_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/iron.png"));
        pub static LODESTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/lodestone.png"));
        pub static MOSSY_STONE_BRICKS_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/mossy_stone_bricks.png"));
        pub static NETHERBRICKS_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/netherbricks.png"));
        pub static NETHERITE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/netherite.png"));
        pub static POLISHED_ANDESITE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/polished_andesite.png"));
        pub static POLISHED_DIORITE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/polished_diorite.png"));
        pub static POLISHED_BLACKSTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/polished_blackstone.png"));
        pub static POLISHED_GRANITE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/polished_granite.png"));
        pub static PRISMARINE_BRICKS_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/prismarine_bricks.png"));
        pub static QUARTZ_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/quartz.png"));
        pub static QUARTZ_BRICKS_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/quartz_bricks.png"));
        pub static QUARTZ_PILLAR_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/quartz_pillar.png"));
        pub static RED_NETHERBRICKS_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/red_netherbricks.png"));
        pub static RED_SANDSTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/red_sandstone.png"));
        pub static SANDSTONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/sandstone.png"));
        pub static STONE_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/stone.png"));
        pub static STONE_BRICKS_GRAVESTONE_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| graveyard::id("textures/entity/gravestone/stone_bricks.png"));
        #[derive(Clone, Debug)]
        pub struct GravestoneIdentifier {
        }
        impl GravestoneIdentifier {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `LegacyContainerName` (com/lion/graveyard/util/LegacyContainerName.java)
    pub mod legacy_container_name {
        use super::*;
        pub const CUSTOM_NAME_KEY: &str = "CustomName";
        #[derive(Clone, Debug)]
        pub struct LegacyContainerName {
        }
        impl LegacyContainerName {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LegacyContainerName.read(1): not translated, it uses Minecraft's `Component`.
        pub fn looks_like_json(text: &str) -> bool {
            return ((text.starts_with("{")) || (text.starts_with("["))) || (text.starts_with("\""));
        }
        // LegacyContainerName.runFixer(3): not translated, it uses Minecraft's `Component`.
        // LegacyContainerName.parseJson(2): not translated, it uses Minecraft's `Component`.
    }

    /// `LegacyItemStacks` (com/lion/graveyard/util/LegacyItemStacks.java)
    pub mod legacy_item_stacks {
        use super::*;
        pub const LEGACY_DATA_VERSION: i32 = 3465;
        pub const PORT_1_21_1_DATA_VERSION: i32 = 3955;
        pub const UNKNOWN_VERSION: i32 = -1;
        pub const DATA_VERSION_KEY: &str = "GraveyardDataVersion";
        pub const EMPTY_ID: &str = "minecraft:air";
        #[derive(Clone, Debug)]
        pub struct LegacyItemStacks {
        }
        impl LegacyItemStacks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LegacyItemStacks.currentDataVersion(0): not translated, it calls Minecraft's `SharedConstants`.getCurrentVersion.
        // LegacyItemStacks.writeDataVersion(1): not translated, it uses Minecraft's `ValueOutput`.
        // LegacyItemStacks.readDataVersion(1): not translated, it uses Minecraft's `ValueInput`.
        // LegacyItemStacks.parse(3): not translated, it uses Minecraft's `ItemStack`.
        // LegacyItemStacks.read(2): not translated, it uses Minecraft's `ItemStack`.
        // LegacyItemStacks.write(3): not translated, it uses Minecraft's `ValueOutput`.
        // LegacyItemStacks.loadSlots(2): not translated, it uses Minecraft's `ValueInput`.
        // LegacyItemStacks.saveSlots(2): not translated, it uses Minecraft's `ValueOutput`.
        // LegacyItemStacks.sourceVersion(2): not translated, it uses Minecraft's `CompoundTag`.
        // LegacyItemStacks.upgrade(2): not translated, it uses Minecraft's `CompoundTag`.
    }

    /// `LegacySavedData` (com/lion/graveyard/util/LegacySavedData.java)
    pub mod legacy_saved_data {
        use super::*;
        // LOGGER: the logger. Log calls become comments.
        #[derive(Clone, Debug)]
        pub struct LegacySavedData {
        }
        impl LegacySavedData {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LegacySavedData.read(2): not translated, it uses Minecraft's `CompoundTag`.
    }

    /// `LenientJsonFiles` (com/lion/graveyard/util/LenientJsonFiles.java)
    pub mod lenient_json_files {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LenientJsonFiles {
        }
        impl LenientJsonFiles {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LenientJsonFiles.read(2): not translated, it uses Minecraft's `ResourceManager`.
    }

    /// `MathUtil` (com/lion/graveyard/util/MathUtil.java)
    pub mod math_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MathUtil {
        }
        impl MathUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MathUtil.createParticleDisk(10): not translated, it's generic.
        // MathUtil.createParticleCircle(11): not translated, it's generic.
        // MathUtil.createParticleSpiral(10): not translated, it's generic.
        // MathUtil.createParticleFlare(9): not translated, it's generic.
    }

    /// `NotifySounds` (com/lion/graveyard/util/NotifySounds.java)
    pub mod notify_sounds {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct NotifySounds {
        }
        impl NotifySounds {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NotifySounds.play(5): not translated, it uses Minecraft's `Player`.
    }

    /// `OptionalTargetMixinPlugin` (com/lion/graveyard/util/OptionalTargetMixinPlugin.java)
    pub mod optional_target_mixin_plugin {
        use super::*;
        // The class itself isn't translated: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility). Its static parts are below.
        // OptionalTargetMixinPlugin.onLoad(1): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // OptionalTargetMixinPlugin.getRefMapperConfig(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // OptionalTargetMixinPlugin.shouldApplyMixin(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // OptionalTargetMixinPlugin.acceptTargets(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // OptionalTargetMixinPlugin.getMixins(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // OptionalTargetMixinPlugin.preApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // OptionalTargetMixinPlugin.postApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
    }

    /// `Scarecrows` (com/lion/graveyard/util/Scarecrows.java)
    pub mod scarecrows {
        use super::*;
        pub const RADIUS: i32 = 8;
        #[derive(Clone, Debug)]
        pub struct Scarecrows {
        }
        impl Scarecrows {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Scarecrows.isNear(3): not translated, it uses Minecraft's `Level`.
        // Scarecrows.hasBlockScarecrow(3): not translated, it uses Minecraft's `Level`.
        // Scarecrows.isRelevant(1): not translated, it uses Minecraft's `BlockState`.
        // Scarecrows.isWearingPumpkin(1): not translated, it uses Minecraft's `ArmorStand`.
        // Scarecrows.isBuiltScarecrow(2): not translated, it uses Minecraft's `Level`.
    }

    /// `SpawnHordeCommand` (com/lion/graveyard/util/SpawnHordeCommand.java)
    pub mod spawn_horde_command {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SpawnHordeCommand {
        }
        impl SpawnHordeCommand {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SpawnHordeCommand.register(1): not translated, it uses Minecraft's `CommandDispatcher`.
        // SpawnHordeCommand.executeSpawn(1): not translated, it uses Minecraft's `CommandSourceStack`.
        // SpawnHordeCommand.spawnHordeEntity(5): not translated, it uses Minecraft's `ServerLevel`.
    }

    /// `StaffRecipeLock` (com/lion/graveyard/util/StaffRecipeLock.java)
    pub mod staff_recipe_lock {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StaffRecipeLock {
        }
        impl StaffRecipeLock {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // StaffRecipeLock.isStaffRecipe(1): not translated, it uses Minecraft's `RecipeHolder`.
        // StaffRecipeLock.isLocked(2): not translated, it uses Minecraft's `ServerPlayer`.
        // StaffRecipeLock.blocksTake(2): not translated, it uses Minecraft's `Player`.
    }

    /// `PoolAliasProcessor` (com/lion/graveyard/world/processors/PoolAliasProcessor.java)
    pub mod pool_alias_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // PoolAliasProcessor.PoolAliasProcessor(1): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // PoolAliasProcessor.apply(1): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // PoolAliasProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `RemoveWaterloggedCryptProcessor` (com/lion/graveyard/world/processors/RemoveWaterloggedCryptProcessor.java)
    pub mod remove_waterlogged_crypt_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // RemoveWaterloggedCryptProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // RemoveWaterloggedCryptProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `RemoveWaterloggedProcessor` (com/lion/graveyard/world/processors/RemoveWaterloggedProcessor.java)
    pub mod remove_waterlogged_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // INSTANCE: not translated, it uses RemoveWaterloggedProcessor, which isn't translated (it implements Minecraft's `StructureProcessor`).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // RemoveWaterloggedProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // RemoveWaterloggedProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `RuinProcessor` (com/lion/graveyard/world/processors/RuinProcessor.java)
    pub mod ruin_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // DEFAULT_FLOOR_REMAINS: not translated, it uses Minecraft's `TagKey`.
        // DEFAULT_WALL_REMAINS: not translated, it uses Minecraft's `TagKey`.
        // DEFAULT_LONG_REMAINS: not translated, it uses Minecraft's `TagKey`.
        // DEFAULT_LITTER: not translated, it uses Minecraft's `TagKey`.
        // RUBBLE_STONE: not translated, it uses Minecraft's `TagKey`.
        // SCORCHABLE_STONE: not translated, it uses Minecraft's `TagKey`.
        // SCORCHABLE_TERRACOTTA: not translated, it uses Minecraft's `TagKey`.
        pub const LOG_PART: i32 = 1;
        pub const WOOD_PART: i32 = 7;
        pub const STRIPPED_LOG_PART: i32 = 8;
        pub const STRIPPED_WOOD_PART: i32 = 9;
        // WOOD_PARTS: not translated, it uses Minecraft's `TagKey`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const LOG_RESISTANCE: f32 = 1.5f32;
        pub const NOISE_CELL: i32 = 4;
        pub const ROOF_HOLE_CHANCE: f32 = 0.25f32;
        pub const MIN_BITE_RADIUS: f32 = 2.5f32;
        pub const BITE_STEEPNESS: f32 = 1.6f32;
        pub const SMALL_HEAP_BLOCKS: i32 = 10;
        pub const LARGE_HEAP_BLOCKS: i32 = 40;
        pub const MAX_HEAP_LAYERS: i32 = 3;
        pub const HEAP_ROUGHNESS: f32 = 0.7f32;
        pub const GRAVEL_CHANCE: f32 = 0.2f32;
        pub const STRAY_CHAR_CHANCE: f32 = 0.25f32;
        pub const BURNT_EDGE_CHAR_CHANCE: f32 = 0.9f32;
        pub const EDGE_CHAR_CHANCE: f32 = 0.5f32;
        pub const VEIN_CHANCE: f32 = 0.3f32;
        pub const VEIN_STRAIGHT_CHANCE: f32 = 0.6f32;
        pub const MIN_VEIN_LENGTH: i32 = 3;
        pub const VEIN_LENGTH_SPREAD: i32 = 6;
        pub const LONE_SCORCH_MARKS: i32 = 3;
        pub const STRAY_EMBER_CHANCE: f32 = 0.1f32;
        pub const MAGMA_CHANCE: f32 = 0.35f32;
        pub const WALL_REMAINS_CHANCE: f32 = 0.5f32;
        pub const SINGE_CHANCE: f32 = 0.75f32;
        pub const SINGED_RUBBLE_CHANCE: f32 = 0.2f32;
        pub const MIN_EMBER_NEIGHBORS: i32 = 2;
        pub const SMALL_PIECE_COLUMNS: i32 = 25;
        pub const LARGE_PIECE_COLUMNS: i32 = 80;
        pub const FEATHER_REACH: i32 = 3;
        pub const PATH_FRAY_CHANCE: f32 = 0.2f32;
        pub const MAX_SINK: i32 = 3;
        pub const MAX_LONE_STACK: i32 = 2;
        pub const FEATHER_PATH_CHANCE: f32 = 0.25f32;
        pub const FEATHER_COARSE_DIRT_CHANCE: f32 = 0.75f32;
        pub const GROUND_SEARCH_UP: i32 = 2;
        pub const GROUND_SEARCH_DOWN: i32 = 4;
        pub static NO_GROUND: LazyLock<i32> = LazyLock::new(|| i32::MIN);
        pub const SMOLDER_SEED: i64 = 1592613387i64;
        // RuinProcessor.RuinProcessor(16): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // RuinProcessor.villageTag(1): not translated, it uses Minecraft's `TagKey`.
        // RuinProcessor.finalizeProcessing(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // RuinProcessor.dried(4): not translated, it uses Minecraft's `StructureBlockInfo`.
        // RuinProcessor.leavesMud(2): not translated, it uses Minecraft's `BlockPos`.
        // RuinProcessor.dropToGround(4): not translated, it uses Minecraft's `StructureBlockInfo`.
        // RuinProcessor.reconnected(5): not translated, it uses Minecraft's `StructureBlockInfo`.
        // RuinProcessor.connectsTo(3): not translated, it uses Minecraft's `BlockState`.
        // RuinProcessor.dropAsUnit(3): not translated, it uses Minecraft's `StructureBlockInfo`.
        // RuinProcessor.groundShift(4): not translated, it uses Minecraft's `ServerLevelAccessor`.
        // RuinProcessor.isGround(2): not translated, it uses Minecraft's `ServerLevelAccessor`.
        // RuinProcessor.isColumnIn(3): not translated, it uses Minecraft's `BoundingBox`.
        // RuinProcessor.members(1): not translated, it uses Minecraft's `Block`.
        // RuinProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // RuinProcessor.evaluatesEntirePieceState(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `RuinProcessor.Bite` (com/lion/graveyard/world/processors/RuinProcessor.java)
    pub mod ruin_processor_bite {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Bite {
            pub x: f32,
            pub z: f32,
            pub radius_x: f32,
            pub radius_z: f32,
            pub depth: f32,
        }
        impl Bite {
            pub fn new(x: f32, z: f32, radius_x: f32, radius_z: f32, depth: f32) -> Self {
                let mut s = Self { x: x, z: z, radius_x: radius_x, radius_z: radius_z, depth: depth };
                s.x = x;
                s.z = z;
                s.radius_x = radius_x;
                s.radius_z = radius_z;
                s.depth = depth;
                s
            }
            pub fn depth_at(&self, block_x: i32, block_z: i32) -> f32 {
                let offset_x: f32 = ((block_x as f32) - self.x) / self.radius_x;
                let offset_z: f32 = ((block_z as f32) - self.z) / self.radius_z;
                let inside: f32 = (1.0f32 - (offset_x * offset_x)) - (offset_z * offset_z);
                return if inside <= 0.0f32 { 0.0f32 } else { self.depth * (1.0f32.min(1.6f32 * inside)) };
            }
            pub fn x(&self) -> f32 {
                return self.x;
            }
            pub fn z(&self) -> f32 {
                return self.z;
            }
            pub fn radius_x(&self) -> f32 {
                return self.radius_x;
            }
            pub fn radius_z(&self) -> f32 {
                return self.radius_z;
            }
            pub fn depth(&self) -> f32 {
                return self.depth;
            }
        }
    }

    /// `RuinProcessor.DryMode` (com/lion/graveyard/world/processors/RuinProcessor.java)
    pub mod ruin_processor_dry_mode {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // RuinProcessor.DryMode.DryMode(1): not translated, its class isn't translated (its constants carry values).
        // RuinProcessor.DryMode.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `RuinProcessor.Ember` (com/lion/graveyard/world/processors/RuinProcessor.java)
    pub mod ruin_processor_ember {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // RuinProcessor.Ember.Ember(3): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ember.spot(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ember.replaced(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ember.cap(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `RuinProcessor.Extras` (com/lion/graveyard/world/processors/RuinProcessor.java)
    pub mod ruin_processor_extras {
        use super::*;
        // The class itself isn't translated: it uses DryMode, which isn't translated (its constants carry values). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // RuinProcessor.Extras.Extras(3): not translated, its class isn't translated (it uses DryMode, which isn't translated (its constants carry values)).
        // RuinProcessor.Extras.dry(0): not translated, its class isn't translated (it uses DryMode, which isn't translated (its constants carry values)).
        // RuinProcessor.Extras.variant(0): not translated, its class isn't translated (it uses DryMode, which isn't translated (its constants carry values)).
        // RuinProcessor.Extras.smolderChance(0): not translated, its class isn't translated (it uses DryMode, which isn't translated (its constants carry values)).
    }

    /// `RuinProcessor.Heap` (com/lion/graveyard/world/processors/RuinProcessor.java)
    pub mod ruin_processor_heap {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Heap {
            pub x: f32,
            pub z: f32,
            pub height: i32,
            pub radius: f32,
        }
        impl Heap {
            pub fn new(x: f32, z: f32, height: i32, radius: f32) -> Self {
                let mut s = Self { x: x, z: z, height: height, radius: radius };
                s.x = x;
                s.z = z;
                s.height = height;
                s.radius = radius;
                s
            }
            pub fn x(&self) -> f32 {
                return self.x;
            }
            pub fn z(&self) -> f32 {
                return self.z;
            }
            pub fn height(&self) -> i32 {
                return self.height;
            }
            pub fn radius(&self) -> f32 {
                return self.radius;
            }
        }
    }

    /// `RuinProcessor.Ruin` (com/lion/graveyard/world/processors/RuinProcessor.java)
    pub mod ruin_processor_ruin {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // RuinProcessor.Ruin.Ruin(7): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.dominantBlock(3): not translated, it uses Minecraft's `Block`.
        // RuinProcessor.Ruin.planksOf(1): not translated, it uses Minecraft's `Block`.
        // RuinProcessor.Ruin.slabOf(2): not translated, it uses Minecraft's `Block`.
        // RuinProcessor.Ruin.isSmall(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.isLarge(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.result(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.quenchEmbers(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.smolderingAllowed(0): not translated, it calls Graveyard.getConfig, which isn't translated.
        // RuinProcessor.Ruin.isCharredWood(1): not translated, it uses Minecraft's `BlockState`.
        // RuinProcessor.Ruin.smoldering(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.collapse(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.piles(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.randomBite(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.spread(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.scorch(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.touchesFracture(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.growVein(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.singe(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.touchesChar(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.isCharred(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.singed(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.woodPart(1): not translated, it uses Minecraft's `BlockState`.
        // RuinProcessor.Ruin.logKind(2): not translated, it uses Minecraft's `BlockState`.
        // RuinProcessor.Ruin.indexIn(2): not translated, it uses Minecraft's `Block`.
        // RuinProcessor.Ruin.scorchBlock(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.canChar(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.charred(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.charStone(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.settle(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.lostSupport(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.lostOtherHalf(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.isProtected(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.dropOrphanSnow(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.wasRemoved(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.pileRubble(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.standsOnTemplate(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.pile(3): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.isFree(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.templateBase(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.worldBase(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.nearestDebris(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.rubbleBlock(3): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.turned(2): not translated, it uses Minecraft's `BlockState`.
        // RuinProcessor.Ruin.rubbleCap(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.placeEmbers(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.heapNeighbors(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.disconnect(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.disconnectedWall(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.needsPost(1): not translated, it uses Minecraft's `BlockState`.
        // RuinProcessor.Ruin.wallSide(1): not translated, it uses Minecraft's `EnumProperty`.
        // RuinProcessor.Ruin.isDisconnected(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.drain(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.sinkLoneColumns(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.columnHeight(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.isLone(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.scatterLitter(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.scatterByCell(4): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.featherPathEdges(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.frayPath(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.isPathEdge(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.featherState(2): not translated, it uses Minecraft's `BlockState`.
        // RuinProcessor.Ruin.oriented(3): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.placeLongRemains(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.placeRemains(3): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.freeNeighbor(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.adjacentWall(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.floorCells(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.groundLevel(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.columnTops(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.put(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.toWorld(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.toTemplate(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // RuinProcessor.Ruin.resistance(1): not translated, it uses Minecraft's `BlockState`.
        // RuinProcessor.Ruin.isRubbleMaterial(1): not translated, it uses Minecraft's `BlockState`.
        // RuinProcessor.Ruin.isFloor(1): not translated, it uses Minecraft's `BlockState`.
        // RuinProcessor.Ruin.isFullCube(1): not translated, it uses Minecraft's `BlockState`.
        pub fn value_noise(seed: i64, x: i32, z: i32) -> f32 {
            let cell_x: i32 = x.div_euclid(4);
            let cell_z: i32 = z.div_euclid(4);
            let delta_x: f32 = ((x.rem_euclid(4)) as f32) / 4.0f32;
            let delta_z: f32 = ((z.rem_euclid(4)) as f32) / 4.0f32;
            let near: f32 = j_lerp(delta_x, ruin_processor_ruin::hash(seed, cell_x, cell_z), ruin_processor_ruin::hash(seed, cell_x + 1, cell_z));
            let far: f32 = j_lerp(delta_x, ruin_processor_ruin::hash(seed, cell_x, cell_z + 1), ruin_processor_ruin::hash(seed, cell_x + 1, cell_z + 1));
            return (j_lerp(delta_z, near, far) * 2.0f32) - 1.0f32;
        }
        pub fn hash(seed: i64, x: i32, z: i32) -> f32 {
            let mut mixed: i64 = (seed + ((x as i64) * (-7046029254386353131i64))) + ((z as i64) * (-4417276706812531889i64));
            mixed = (mixed ^ (((mixed as u64) >> 30) as i64)) * (-4658895280553007687i64);
            mixed = (mixed ^ (((mixed as u64) >> 27) as i64)) * (-7723592293110705685i64);
            mixed ^= ((mixed as u64) >> 31) as i64;
            return ((((mixed as u64) >> 40) as i64) as f32) / 1.6777216e7f32;
        }
    }

    /// `RuinProcessor.Variant` (com/lion/graveyard/world/processors/RuinProcessor.java)
    pub mod ruin_processor_variant {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // DEFAULT: not translated, it uses Variant, which isn't translated (it uses Minecraft's `Block`).
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // RuinProcessor.Variant.Variant(7): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // RuinProcessor.Variant.charStone(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // RuinProcessor.Variant.stoneFallback(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // RuinProcessor.Variant.stoneCap(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // RuinProcessor.Variant.looseRubble(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // RuinProcessor.Variant.scorchTerracotta(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // RuinProcessor.Variant.woodRubble(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // RuinProcessor.Variant.rubble(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `SwitchSpawnerProcessor` (com/lion/graveyard/world/processors/SwitchSpawnerProcessor.java)
    pub mod switch_spawner_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // INSTANCE: not translated, it uses SwitchSpawnerProcessor, which isn't translated (it implements Minecraft's `StructureProcessor`).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SwitchSpawnerProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // SwitchSpawnerProcessor.swapEntityId(1): not translated, it uses Minecraft's `CompoundTag`.
        // SwitchSpawnerProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `TGJigsawStructure` (com/lion/graveyard/world/structures/TGJigsawStructure.java)
    pub mod tg_jigsaw_structure {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Structure`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TGJigsawStructure.TGJigsawStructure(11): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // TGJigsawStructure.findGenerationPoint(1): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // TGJigsawStructure.startPieces(2): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // TGJigsawStructure.canGenerate(5): not translated, it uses Minecraft's `GenerationContext`.
        // TGJigsawStructure.isTerrainFlat(5): not translated, it uses Minecraft's `GenerationContext`.
        // TGJigsawStructure.type(0): not translated, its class isn't translated (it extends Minecraft's `Structure`).
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

    
    // ---- Gson (com.google.gson): JSON values, a reader and Gson's writer ----
    // JsonElement, JsonObject, JsonArray and JsonPrimitive are all one Rust type. Numbers keep their text, as
    // Gson's LazilyParsedNumber does. Objects keep their insertion order, as Gson's LinkedTreeMap does.
    
    #[derive(Clone, Debug, PartialEq)]
    pub enum JJson {
        Null,
        Bool(bool),
        Num(String),
        Str(String),
        Arr(Vec<JJson>),
        Obj(Vec<(String, JJson)>),
    }
    
    /// A float or double as Java prints it (Float.toString / Double.toString).
    pub fn j_java_num(x: f64, float: bool) -> String {
        if x.is_nan() {
            return "NaN".into();
        }
        if x.is_infinite() {
            return if x > 0.0 { "Infinity".into() } else { "-Infinity".into() };
        }
        let a = x.abs();
        if x == 0.0 {
            return if x.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
        }
        let digits = if float { format!("{:e}", x as f32) } else { format!("{:e}", x) };
        let (mant, exp) = digits.split_once('e').unwrap();
        let exp: i32 = exp.parse().unwrap();
        if (1e-3..1e7).contains(&a) {
            let plain = if float { format!("{}", x as f32) } else { format!("{}", x) };
            if plain.contains('.') { plain } else { plain + ".0" }
        } else {
            let mant = if mant.contains('.') { mant.to_string() } else { format!("{mant}.0") };
            format!("{mant}E{exp}")
        }
    }
    
    struct JJsonReader<'a> {
        s: &'a [char],
        i: usize,
    }
    
    impl JJsonReader<'_> {
        fn ws(&mut self) {
            while self.i < self.s.len() && self.s[self.i].is_whitespace() {
                self.i += 1;
            }
        }
        fn fail(&self, what: &str) -> ! {
            panic!("JsonSyntaxException: {what} at {}", self.i)
        }
        fn value(&mut self) -> JJson {
            self.ws();
            match self.s.get(self.i) {
                Some('{') => {
                    self.i += 1;
                    let mut members = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&'}') {
                        self.i += 1;
                        return JJson::Obj(members);
                    }
                    loop {
                        self.ws();
                        let k = match self.s.get(self.i) {
                            Some('"') | Some('\'') => self.string(),
                            _ => self.bare(),
                        };
                        self.ws();
                        if !matches!(self.s.get(self.i), Some(':') | Some('=')) {
                            self.fail("expected ':'");
                        }
                        self.i += 1;
                        let v = self.value();
                        // A repeated name: Gson keeps the last one.
                        members.retain(|(n, _): &(String, JJson)| *n != k);
                        members.push((k, v));
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some('}') => {
                                self.i += 1;
                                return JJson::Obj(members);
                            }
                            _ => self.fail("expected ',' or '}'"),
                        }
                    }
                }
                Some('[') => {
                    self.i += 1;
                    let mut items = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&']') {
                        self.i += 1;
                        return JJson::Arr(items);
                    }
                    loop {
                        items.push(self.value());
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some(']') => {
                                self.i += 1;
                                return JJson::Arr(items);
                            }
                            _ => self.fail("expected ',' or ']'"),
                        }
                    }
                }
                Some('"') | Some('\'') => JJson::Str(self.string()),
                Some(_) => {
                    let word = self.bare();
                    match word.as_str() {
                        "true" => JJson::Bool(true),
                        "false" => JJson::Bool(false),
                        "null" => JJson::Null,
                        w if w.parse::<f64>().is_ok() => JJson::Num(w.to_string()),
                        // Gson is lenient: an unquoted word is a string.
                        w => JJson::Str(w.to_string()),
                    }
                }
                None => self.fail("end of input"),
            }
        }
        fn bare(&mut self) -> String {
            let start = self.i;
            while self.i < self.s.len() && !matches!(self.s[self.i], ',' | ':' | '=' | ';' | '}' | ']' | '{' | '[' | '"' | '\'') && !self.s[self.i].is_whitespace() {
                self.i += 1;
            }
            if start == self.i {
                self.fail("expected a value");
            }
            self.s[start..self.i].iter().collect()
        }
        fn string(&mut self) -> String {
            let q = self.s[self.i];
            self.i += 1;
            let mut out = String::new();
            loop {
                let Some(&c) = self.s.get(self.i) else { self.fail("unterminated string") };
                self.i += 1;
                match c {
                    c if c == q => return out,
                    '\\' => {
                        let Some(&e) = self.s.get(self.i) else { self.fail("unterminated string") };
                        self.i += 1;
                        match e {
                            'n' => out.push('\n'),
                            't' => out.push('\t'),
                            'r' => out.push('\r'),
                            'b' => out.push('\u{8}'),
                            'f' => out.push('\u{c}'),
                            'u' => {
                                let hex: String = self.s[self.i..(self.i + 4).min(self.s.len())].iter().collect();
                                self.i += 4;
                                let unit = u16::from_str_radix(&hex, 16).unwrap_or_else(|_| self.fail("bad \\u escape"));
                                // A surrogate pair: two escapes make one char.
                                if (0xD800..0xDC00).contains(&unit) && self.s.get(self.i) == Some(&'\\') && self.s.get(self.i + 1) == Some(&'u') {
                                    let hex2: String = self.s[self.i + 2..(self.i + 6).min(self.s.len())].iter().collect();
                                    if let Ok(low) = u16::from_str_radix(&hex2, 16) {
                                        self.i += 6;
                                        out.extend(char::decode_utf16([unit, low]).map(|r| r.unwrap_or('\u{FFFD}')));
                                        continue;
                                    }
                                }
                                out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
                            }
                            other => out.push(other),
                        }
                    }
                    c => out.push(c),
                }
            }
        }
    }
    
    impl JJson {
        /// JsonParser.parseString: panics (Java throws JsonSyntaxException) on bad JSON.
        pub fn parse(s: &str) -> JJson {
            let chars: Vec<char> = s.chars().collect();
            let mut r = JJsonReader { s: &chars, i: 0 };
            let v = r.value();
            r.ws();
            if r.i != chars.len() {
                r.fail("more after the value");
            }
            v
        }
        pub fn object() -> JJson {
            JJson::Obj(Vec::new())
        }
        pub fn array() -> JJson {
            JJson::Arr(Vec::new())
        }
        pub fn int(v: i64) -> JJson {
            JJson::Num(v.to_string())
        }
        pub fn float(v: f32) -> JJson {
            JJson::Num(j_java_num(v as f64, true))
        }
        pub fn double(v: f64) -> JJson {
            JJson::Num(j_java_num(v, false))
        }
        pub fn str(v: &str) -> JJson {
            JJson::Str(v.to_string())
        }
        pub fn is_json_object(&self) -> bool {
            matches!(self, JJson::Obj(_))
        }
        pub fn is_json_array(&self) -> bool {
            matches!(self, JJson::Arr(_))
        }
        pub fn is_json_null(&self) -> bool {
            matches!(self, JJson::Null)
        }
        pub fn is_json_primitive(&self) -> bool {
            matches!(self, JJson::Bool(_) | JJson::Num(_) | JJson::Str(_))
        }
        pub fn is_string(&self) -> bool {
            matches!(self, JJson::Str(_))
        }
        pub fn is_number(&self) -> bool {
            matches!(self, JJson::Num(_))
        }
        pub fn is_boolean(&self) -> bool {
            matches!(self, JJson::Bool(_))
        }
        /// getAsJsonObject() / getAsJsonArray(): Java throws IllegalStateException on the wrong kind.
        pub fn as_object(&self) -> JJson {
            assert!(self.is_json_object(), "Not a JSON Object: {self}");
            self.clone()
        }
        pub fn as_array(&self) -> JJson {
            assert!(self.is_json_array(), "Not a JSON Array: {self}");
            self.clone()
        }
        fn members(&self) -> &Vec<(String, JJson)> {
            match self {
                JJson::Obj(m) => m,
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        /// The (name, value) pairs of an object, as entrySet() walks them.
        pub fn entries(&self) -> impl Iterator<Item = (&String, &JJson)> {
            self.members().iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &String> {
            self.members().iter().map(|(k, _)| k)
        }
        pub fn elements(&self) -> impl Iterator<Item = &JJson> {
            match self {
                JJson::Arr(a) => a.iter(),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn has(&self, k: &str) -> bool {
            self.members().iter().any(|(n, _)| n == k)
        }
        /// JsonObject.get: None (null) when absent.
        pub fn get(&self, k: &str) -> Option<JJson> {
            self.members().iter().find(|(n, _)| n == k).map(|(_, v)| v.clone())
        }
        /// JsonObject.getAsJsonObject(name) / getAsJsonArray(name): null when absent, a cast that throws on the wrong kind.
        pub fn member_object(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_object())
        }
        pub fn member_array(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_array())
        }
        /// JsonArray.get(i).
        pub fn at(&self, i: i32) -> JJson {
            match self {
                JJson::Arr(a) => a.get(i as usize).cloned().unwrap_or_else(|| panic!("Index {i} out of bounds for length {}", a.len())),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn size(&self) -> i32 {
            match self {
                JJson::Arr(a) => a.len() as i32,
                JJson::Obj(m) => m.len() as i32,
                _ => panic!("size() of a JSON primitive"),
            }
        }
        pub fn is_empty(&self) -> bool {
            self.size() == 0
        }
        /// The single value of a one-element array, as Gson's JsonArray.getAsX does.
        fn single(&self) -> &JJson {
            match self {
                JJson::Arr(a) if a.len() == 1 => &a[0],
                JJson::Arr(_) => panic!("Array must have size 1"),
                other => other,
            }
        }
        pub fn as_string(&self) -> String {
            match self.single() {
                JJson::Str(s) => s.clone(),
                JJson::Num(n) => n.clone(),
                JJson::Bool(b) => b.to_string(),
                other => panic!("UnsupportedOperationException: {other} as a string"),
            }
        }
        fn number_text(&self) -> String {
            match self.single() {
                JJson::Num(n) | JJson::Str(n) => n.trim().to_string(),
                other => panic!("UnsupportedOperationException: {other} as a number"),
            }
        }
        pub fn as_f64(&self) -> f64 {
            let t = self.number_text();
            t.parse().unwrap_or_else(|_| panic!("NumberFormatException: {t}"))
        }
        pub fn as_float(&self) -> f32 {
            self.as_f64() as f32
        }
        pub fn as_double(&self) -> f64 {
            self.as_f64()
        }
        /// getAsInt / getAsLong: a whole number, else the number cut to one.
        pub fn as_long(&self) -> i64 {
            let t = self.number_text();
            t.parse::<i64>().unwrap_or_else(|_| t.parse::<f64>().map(|f| f as i64).unwrap_or_else(|_| panic!("NumberFormatException: {t}")))
        }
        pub fn as_int(&self) -> i32 {
            self.as_long() as i32
        }
        pub fn as_bool(&self) -> bool {
            match self.single() {
                JJson::Bool(b) => *b,
                // Gson: Boolean.parseBoolean of the text
                other => other.as_string().eq_ignore_ascii_case("true"),
            }
        }
        pub fn add(&mut self, k: &str, v: JJson) {
            match self {
                JJson::Obj(m) => {
                    m.retain(|(n, _)| n != k);
                    m.push((k.to_string(), v));
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn remove(&mut self, k: &str) -> Option<JJson> {
            match self {
                JJson::Obj(m) => {
                    let i = m.iter().position(|(n, _)| n == k)?;
                    Some(m.remove(i).1)
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn push(&mut self, v: JJson) {
            match self {
                JJson::Arr(a) => a.push(v),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        fn write(&self, out: &mut String, pretty: bool, html: bool, depth: usize) {
            let nl = |out: &mut String, d: usize| {
                if pretty {
                    out.push('\n');
                    out.push_str(&"  ".repeat(d));
                }
            };
            match self {
                JJson::Null => out.push_str("null"),
                JJson::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                JJson::Num(n) => out.push_str(n),
                JJson::Str(s) => j_json_quote(s, html, out),
                JJson::Arr(a) => {
                    out.push('[');
                    for (i, v) in a.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !a.is_empty() {
                        nl(out, depth);
                    }
                    out.push(']');
                }
                JJson::Obj(m) => {
                    out.push('{');
                    for (i, (k, v)) in m.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        j_json_quote(k, html, out);
                        out.push(':');
                        if pretty {
                            out.push(' ');
                        }
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !m.is_empty() {
                        nl(out, depth);
                    }
                    out.push('}');
                }
            }
        }
    }
    
    fn j_json_quote(s: &str, html: bool, out: &mut String) {
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{8}' => out.push_str("\\b"),
                '\u{c}' => out.push_str("\\f"),
                '<' | '>' | '&' | '=' | '\'' if html => out.push_str(&format!("\\u{:04x}", c as u32)),
                '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
    }
    
    /// JsonElement.toString: compact, without Gson's HTML escaping.
    impl std::fmt::Display for JJson {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut out = String::new();
            self.write(&mut out, false, false, 0);
            f.write_str(&out)
        }
    }
    
    /// A Gson (from `new Gson()` or a GsonBuilder): how it writes.
    #[derive(Clone, Copy, Debug)]
    pub struct JGson {
        pub pretty: bool,
        pub html: bool,
        pub nulls: bool,
    }
    
    impl Default for JGson {
        fn default() -> Self {
            JGson { pretty: false, html: true, nulls: false }
        }
    }
    
    impl JGson {
        /// Gson.toJson.
        pub fn to_json(&self, v: &JJson) -> String {
            let mut out = String::new();
            self.strip(v).write(&mut out, self.pretty, self.html, 0);
            out
        }
        /// Without serializeNulls, Gson leaves out null fields of objects.
        fn strip(&self, v: &JJson) -> JJson {
            match v {
                JJson::Obj(m) => JJson::Obj(m.iter().filter(|(_, x)| self.nulls || !x.is_json_null()).map(|(k, x)| (k.clone(), self.strip(x))).collect()),
                JJson::Arr(a) => JJson::Arr(a.iter().map(|x| self.strip(x)).collect()),
                other => other.clone(),
            }
        }
    }
    
    #[cfg(test)]
    mod jjson_tests {
        use super::*;
    
        #[test]
        fn read_and_write() {
            let j = JJson::parse("{\"a\": 1, \"b\": [1.5, \"x\", true, null], \"c\": {\"d\": -2e3}}");
            assert!(j.has("a") && !j.has("z"));
            assert_eq!(j.get("a").unwrap().as_int(), 1);
            assert_eq!(j.member_array("b").unwrap().at(0).as_float(), 1.5);
            assert_eq!(j.member_array("b").unwrap().size(), 4);
            assert_eq!(j.member_object("c").unwrap().get("d").unwrap().as_double(), -2000.0);
            assert_eq!(j.get("z"), None);
            assert_eq!(j.to_string(), "{\"a\":1,\"b\":[1.5,\"x\",true,null],\"c\":{\"d\":-2e3}}");
            let g = JGson { pretty: true, ..JGson::default() };
            assert_eq!(g.to_json(&JJson::parse("{\"a\":[1,2],\"b\":{},\"n\":null,\"s\":\"<x>\"}")),
                       "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {},\n  \"s\": \"\\u003cx\\u003e\"\n}");
        }
    
        #[test]
        fn java_numbers() {
            assert_eq!(j_java_num(1.0, true), "1.0");
            assert_eq!(j_java_num(0.1, true), "0.1");
            assert_eq!(j_java_num(1.0e-4, false), "1.0E-4");
            assert_eq!(j_java_num(12345678.0, false), "1.2345678E7");
            assert_eq!(j_java_num(-0.75, false), "-0.75");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "graveyard");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
