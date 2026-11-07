// Traveler's Backpack, ported to Horizon by horizon-port from travelersbackpack-fabric-26.3-11.4.3.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_travelersbackpack). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "travelersbackpack";
pub const NAME: &str = "Traveler's Backpack";
pub const VERSION: &str = "11.4.3";
pub const DESCRIPTION: &str = "Unique and upgradeable backpacks with customization, Trinkets/Accessories integration and more!";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[
    ["ability", "Enable/Disable Special Ability", "key.keyboard.unknown"],
    ["cycle_tool", "Swap Tool/Change Hose Mode", "key.keyboard.unknown"],
    ["inventory", "Open Backpack", "key.keyboard.unknown"],
    ["sort", "Sort Backpack", "key.keyboard.unknown"],
    ["toggle_upgrade_0", "Toggle Upgrade in the 1st slot", "key.keyboard.unknown"],
    ["toggle_upgrade_1", "Toggle Upgrade in the 2nd slot", "key.keyboard.unknown"],
    ["toggle_upgrade_2", "Toggle Upgrade in the 3rd slot", "key.keyboard.unknown"],
    ["toggle_upgrade_3", "Toggle Upgrade in the 4th slot", "key.keyboard.unknown"],
];

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

    /// `ActionTypeTrigger` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java)
    pub mod action_type_trigger {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleCriterionTrigger`. Its static parts are below.
        pub const UNDYE_BACKPACK: &str = "undye_backpack";
        pub const CHANGE_SLEEPING_BAG: &str = "change_sleeping_bag";
        pub const USE_SLEEPING_BAG: &str = "use_sleeping_bag";
        pub const REVERT_CUSTOM_BACKPACK: &str = "revert_custom_backpack";
        pub const SWAP_TOOLS: &str = "swap_tools";
        pub const HOSE_SUCK: &str = "hose_suck";
        pub const HOSE_SPILL: &str = "hose_spill";
        pub const HOSE_DRINK: &str = "hose_drink";
        pub const HOSE_SPILL_POTION: &str = "hose_spill_potion";
        pub const HOSE_DRINK_POTION: &str = "hose_drink_potion";
        // ActionTypeTrigger.trigger(2): not translated, its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // ActionTypeTrigger.codec(0): not translated, its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
    }

    /// `ActionTypeTrigger.TriggerInstance` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java)
    pub mod action_type_trigger_trigger_instance {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `SimpleInstance`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // ActionTypeTrigger.TriggerInstance.TriggerInstance(2): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
        // ActionTypeTrigger.TriggerInstance.test(1): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
        // ActionTypeTrigger.TriggerInstance.player(0): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
        // ActionTypeTrigger.TriggerInstance.action(0): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
    }

    /// `HoseEvent` (com/tiviacz/travelersbackpack/api/event/HoseEvent.java)
    pub mod hose_event {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Player`. Its static parts are below.
        // HoseEvent.HoseEvent(3): not translated, its class isn't translated (it uses Minecraft's `Player`).
        // HoseEvent.getPlayer(0): not translated, its class isn't translated (it uses Minecraft's `Player`).
        // HoseEvent.getLevel(0): not translated, its class isn't translated (it uses Minecraft's `Player`).
        // HoseEvent.getFluid(0): not translated, its class isn't translated (it uses Minecraft's `Player`).
        // HoseEvent.isCanceled(0): not translated, its class isn't translated (it uses Minecraft's `Player`).
        // HoseEvent.cancel(0): not translated, its class isn't translated (it uses Minecraft's `Player`).
    }

    /// `HoseEvent.Drink` (com/tiviacz/travelersbackpack/api/event/HoseEvent.java)
    pub mod hose_event_drink {
        use super::*;
        // The class itself isn't translated: it extends `HoseEvent`. Its static parts are below.
        // EVENT: not translated, it uses Fabric's `Event`.
        // HoseEvent.Drink.Drink(4): not translated, its class isn't translated (it extends `HoseEvent`).
        // HoseEvent.Drink.getAmount(0): not translated, its class isn't translated (it extends `HoseEvent`).
        // HoseEvent.Drink.setAmount(1): not translated, its class isn't translated (it extends `HoseEvent`).
    }

    /// `HoseEvent.Drink.Callback` (com/tiviacz/travelersbackpack/api/event/HoseEvent.java)
    pub mod hose_event_drink_callback {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // HoseEvent.Drink.Callback.onDrink(1): not translated, it has no body.
    }

    /// `HoseEvent.PickUp` (com/tiviacz/travelersbackpack/api/event/HoseEvent.java)
    pub mod hose_event_pick_up {
        use super::*;
        // The class itself isn't translated: it extends `HoseEvent`. Its static parts are below.
        // EVENT: not translated, it uses Fabric's `Event`.
        // HoseEvent.PickUp.PickUp(5): not translated, its class isn't translated (it extends `HoseEvent`).
        // HoseEvent.PickUp.getPos(0): not translated, its class isn't translated (it extends `HoseEvent`).
        // HoseEvent.PickUp.getState(0): not translated, its class isn't translated (it extends `HoseEvent`).
        // HoseEvent.PickUp.setFluid(1): not translated, its class isn't translated (it extends `HoseEvent`).
    }

    /// `HoseEvent.PickUp.Callback` (com/tiviacz/travelersbackpack/api/event/HoseEvent.java)
    pub mod hose_event_pick_up_callback {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // HoseEvent.PickUp.Callback.onPickUp(1): not translated, it has no body.
    }

    /// `HoseEvent.Spill` (com/tiviacz/travelersbackpack/api/event/HoseEvent.java)
    pub mod hose_event_spill {
        use super::*;
        // The class itself isn't translated: it extends `HoseEvent`. Its static parts are below.
        // EVENT: not translated, it uses Fabric's `Event`.
        // HoseEvent.Spill.Spill(4): not translated, its class isn't translated (it extends `HoseEvent`).
        // HoseEvent.Spill.getPos(0): not translated, its class isn't translated (it extends `HoseEvent`).
    }

    /// `HoseEvent.Spill.Callback` (com/tiviacz/travelersbackpack/api/event/HoseEvent.java)
    pub mod hose_event_spill_callback {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // HoseEvent.Spill.Callback.onSpill(1): not translated, it has no body.
    }

    /// `EffectFluid` (com/tiviacz/travelersbackpack/api/fluids/EffectFluid.java)
    pub mod effect_fluid {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Fluid`. Its static parts are below.
        // EffectFluid.EffectFluid(3): not translated, its class isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluid.EffectFluid(3): not translated, its class isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluid.EffectFluid(4): not translated, its class isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluid.getUniqueId(0): not translated, its class isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluid.setEffectID(1): not translated, its class isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluid.getEffectID(0): not translated, its class isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluid.affectDrinker(3): not translated, it has no body.
        // EffectFluid.canExecuteEffect(3): not translated, it has no body.
    }

    /// `AttachmentUtils` (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java)
    pub mod attachment_utils {
        use super::*;
        // LOAD_ALL: not translated, a function kept in a variable or field (only parameters can be).
        // NO_ITEMS: not translated, a function kept in a variable or field (only parameters can be).
        // STORAGE_ONLY: not translated, a function kept in a variable or field (only parameters can be).
        // UPGRADES_ONLY: not translated, a function kept in a variable or field (only parameters can be).
        // TOOLS_ONLY: not translated, a function kept in a variable or field (only parameters can be).
        #[derive(Clone, Debug)]
        pub struct AttachmentUtils {
        }
        impl AttachmentUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AttachmentUtils.getAttachment(1): not translated, it uses Minecraft's `Player`.
        // AttachmentUtils.registerJoinEquip(0): not translated, it uses Fabric's `ServerPlayerEvents`.JOIN.
        // AttachmentUtils.synchronise(1): not translated, it uses Minecraft's `Player`.
        // AttachmentUtils.isWearingBackpack(1): not translated, it uses Minecraft's `Player`.
        // AttachmentUtils.getWearingBackpack(1): not translated, it uses Minecraft's `ItemStack`.
        // AttachmentUtils.equipBackpack(2): not translated, it uses Minecraft's `Player`.
        // AttachmentUtils.getBackpackWrapper(2): not translated, it's overloaded with the same number of parameters.
        // AttachmentUtils.getBackpackWrapper(3): not translated, it uses Minecraft's `Player`.
        // AttachmentUtils.getBackpackWrapperArtificial(1): not translated, it uses Minecraft's `Player`.
        // AttachmentUtils.getBackpackWrapper(1): not translated, it uses Minecraft's `Player`.
        // AttachmentUtils.getBackpackWrapper(2): not translated, it's overloaded with the same number of parameters.
    }

    /// `BackpackAttachment` (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java)
    pub mod backpack_attachment {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // BackpackAttachment.BackpackAttachment(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackAttachment.hasBackpack(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackAttachment.getBackpack(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackAttachment.equipBackpack(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackAttachment.updateBackpack(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackAttachment.applyComponents(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackAttachment.removeWearable(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackAttachment.removeWrapper(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackAttachment.remove(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackAttachment.getWrapper(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackAttachment.synchronise(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackAttachment.synchronise(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `BackpackBlockEntity` (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java)
    pub mod backpack_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        pub const BACKPACK: &str = "Backpack";
        pub const SLEEPING_BAG: &str = "SleepingBag";
        pub const SETTINGS_USER: &str = "SettingsUser";
        // BackpackBlockEntity.BackpackBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.getWrapper(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.removeWrapper(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.setBackpack(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.setBackpackFromNbt(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.setLevel(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.writeBackpack(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.preRemoveSideEffects(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.getBlockDirection(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.isSleepingBagDeployed(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.setSleepingBagDeployed(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.canPlaceSleepingBag(2): not translated, it uses Minecraft's `BlockPos`.
        // BackpackBlockEntity.deploySleepingBag(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.removeSleepingBag(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.isThereSleepingBag(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.getProperSleepingBag(1): not translated, it uses Minecraft's `BlockState`.
        // BackpackBlockEntity.collectImplicitComponents(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.toItemStack(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.notifyBlockUpdate(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.tick(4): not translated, it uses Minecraft's `Level`.
        // BackpackBlockEntity.getDisplayName(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.getDefaultName(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.setSettingsUser(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.getSettingsUser(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.removeSettingsUser(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.canOpenSettings(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.getUpdatePacket(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.getUpdateTag(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.createMenu(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.openBackpack(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.openSettings(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.openBackpackFromCommand(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.shouldCloseCurrentScreen(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackBlockEntity.saveSettingsExtraData(1): not translated, it uses Minecraft's `BlockPos`.
        // BackpackBlockEntity.saveExtraData(2): not translated, it uses Minecraft's `BlockPos`.
        // BackpackBlockEntity.getRenderData(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
    }

    /// `BackpackBlockEntity.BackpackRenderData` (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java)
    pub mod backpack_block_entity_backpack_render_data {
        use super::*;
        // The class itself isn't translated: it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`). Its static parts are below.
        // BackpackBlockEntity.BackpackRenderData.BackpackRenderData(4): not translated, its class isn't translated (it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`)).
        // BackpackBlockEntity.BackpackRenderData.info(0): not translated, its class isn't translated (it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`)).
        // BackpackBlockEntity.BackpackRenderData.dyeColor(0): not translated, its class isn't translated (it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`)).
        // BackpackBlockEntity.BackpackRenderData.isSleepingBagDeployed(0): not translated, its class isn't translated (it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`)).
        // BackpackBlockEntity.BackpackRenderData.sleepingBagColor(0): not translated, its class isn't translated (it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`)).
    }

    /// `BackpackBlockEntity.ExtendedScreen` (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java)
    pub mod backpack_block_entity_extended_screen {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // BackpackBlockEntity.ExtendedScreen.ExtendedScreen(2): not translated, its class isn't translated (it's generic).
        // BackpackBlockEntity.ExtendedScreen.shouldCloseCurrentScreen(0): not translated, its class isn't translated (it's generic).
        // BackpackBlockEntity.ExtendedScreen.getScreenOpeningData(1): not translated, its class isn't translated (it's generic).
        // BackpackBlockEntity.ExtendedScreen.getDisplayName(0): not translated, its class isn't translated (it's generic).
        // BackpackBlockEntity.ExtendedScreen.createMenu(3): not translated, its class isn't translated (it's generic).
        // BackpackBlockEntity.ExtendedScreen.containerSupplier(0): not translated, its class isn't translated (it's generic).
        // BackpackBlockEntity.ExtendedScreen.screenOpeningData(0): not translated, its class isn't translated (it's generic).
    }

    /// `SleepingBagBlock` (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java)
    pub mod sleeping_bag_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BedBlock`. Its static parts are below.
        // PART: not translated, it uses Minecraft's `EnumProperty`.
        // OCCUPIED: not translated, it uses Minecraft's `BooleanProperty`.
        // CAN_DROP: not translated, it uses Minecraft's `BooleanProperty`.
        // SLEEPING_BAG: not translated, it uses Minecraft's `VoxelShape`.
        // SLEEPING_BAG_NORTH: not translated, it uses Minecraft's `VoxelShape`.
        // SLEEPING_BAG_EAST: not translated, it uses Minecraft's `VoxelShape`.
        // SLEEPING_BAG_SOUTH: not translated, it uses Minecraft's `VoxelShape`.
        // SLEEPING_BAG_WEST: not translated, it uses Minecraft's `VoxelShape`.
        // SleepingBagBlock.SleepingBagBlock(2): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.destroyOnUse(4): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.kickVillagerOutOfBed(2): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.getNeighbourDirection(2): not translated, it uses Minecraft's `Direction`.
        // SleepingBagBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.playerWillDestroy(4): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.getSeed(2): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
        // SleepingBagBlock.getDrops(2): not translated, its class isn't translated (it extends Minecraft's `BedBlock`).
    }

    /// `TravelersBackpackBlock` (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java)
    pub mod travelers_backpack_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // ALL_DIRECTIONS: not translated, it uses Minecraft's `Direction`.
        // BACKPACK_TANKS_SHAPE_NORTH: not translated, it uses Minecraft's `VoxelShape`.
        // BACKPACK_TANKS_SHAPE_EAST: not translated, it uses Minecraft's `VoxelShape`.
        // BACKPACK_TANKS_SHAPE_SOUTH: not translated, it uses Minecraft's `VoxelShape`.
        // BACKPACK_TANKS_SHAPE_WEST: not translated, it uses Minecraft's `VoxelShape`.
        // BACKPACK_SHAPE_NORTH: not translated, it uses Minecraft's `VoxelShape`.
        // BACKPACK_SHAPE_EAST: not translated, it uses Minecraft's `VoxelShape`.
        // BACKPACK_SHAPE_SOUTH: not translated, it uses Minecraft's `VoxelShape`.
        // BACKPACK_SHAPE_WEST: not translated, it uses Minecraft's `VoxelShape`.
        // TravelersBackpackBlock.TravelersBackpackBlock(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.onExplosionHit(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.playerWillDestroy(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.entityInside(6): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.getAnalogOutputSignal(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.getTicker(3): not translated, it's generic.
        // TravelersBackpackBlock.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.getSignal(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.isSignalSource(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.onPlace(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.neighborChanged(6): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.tryAbsorbWater(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.removeWaterBreadthFirstSearch(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // TravelersBackpackBlock.registerDispenserBehaviour(0): not translated, it uses Minecraft's `BuiltInRegistries`.ITEM.
    }

    /// `BackpackDynamicModel` (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java)
    pub mod backpack_dynamic_model {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `UnbakedModel`. Its static parts are below.
        // BackpackDynamicModel.BackpackDynamicModel(1): not translated, its class isn't translated (it implements Minecraft's `UnbakedModel`).
        // BackpackDynamicModel.bakeBlockStateModel(3): not translated, its class isn't translated (it implements Minecraft's `UnbakedModel`).
        // BackpackDynamicModel.getTextureSlots(3): not translated, its class isn't translated (it implements Minecraft's `UnbakedModel`).
        // BackpackDynamicModel.resolveDependencies(1): not translated, its class isn't translated (it implements Minecraft's `UnbakedModel`).
    }

    /// `BackpackDynamicModel.DynamicBlockStateModel` (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java)
    pub mod backpack_dynamic_model_dynamic_block_state_model {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockStateModel`. Its static parts are below.
        // BackpackDynamicModel.DynamicBlockStateModel.DynamicBlockStateModel(3): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.emitQuads(6): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.rebakeSleepingBag(2): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.addExtras(1): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.getVector(3): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.rotate(2): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.createQuad(10): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.collectParts(2): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.collectParts(1): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.addBaseBackpack(1): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.addTanks(1): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.addFluids(2): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.addFluid(5): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.addSleepingBag(1): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.addAll(2): not translated, it uses `Builder` (com.google.common.collect.ImmutableMap).
        // BackpackDynamicModel.DynamicBlockStateModel.getQuads(0): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.particleMaterial(0): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // BackpackDynamicModel.DynamicBlockStateModel.materialFlags(0): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
    }

    /// `BackpackDynamicModel.Loader` (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java)
    pub mod backpack_dynamic_model_loader {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `UnbakedModelDeserializer`. Its static parts are below.
        // INSTANCE: not translated, it uses Loader, which isn't translated (it implements Fabric's `UnbakedModelDeserializer`).
        // BackpackDynamicModel.Loader.deserialize(2): not translated, its class isn't translated (it implements Fabric's `UnbakedModelDeserializer`).
        // BackpackDynamicModel.Loader.addPartModel(3): not translated, its class isn't translated (it implements Fabric's `UnbakedModelDeserializer`).
    }

    /// `BackpackDynamicModel.Loader.PartModel` (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java)
    pub mod backpack_dynamic_model_loader_part_model {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `UnbakedModel`. Its static parts are below.
        // BackpackDynamicModel.Loader.PartModel.PartModel(2): not translated, its class isn't translated (it implements Minecraft's `UnbakedModel`).
        // BackpackDynamicModel.Loader.PartModel.ambientOcclusion(0): not translated, its class isn't translated (it implements Minecraft's `UnbakedModel`).
        // BackpackDynamicModel.Loader.PartModel.transforms(0): not translated, its class isn't translated (it implements Minecraft's `UnbakedModel`).
        // BackpackDynamicModel.Loader.PartModel.textureSlots(0): not translated, its class isn't translated (it implements Minecraft's `UnbakedModel`).
        // BackpackDynamicModel.Loader.PartModel.parent(0): not translated, its class isn't translated (it implements Minecraft's `UnbakedModel`).
        // BackpackDynamicModel.Loader.PartModel.textures(0): not translated, its class isn't translated (it implements Minecraft's `UnbakedModel`).
    }

    /// `BackpackDynamicModel.ModelParts` (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java)
    pub mod backpack_dynamic_model_model_parts {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ModelParts {
            Base,
            BaseDyed,
            Extras,
            Tanks,
            SleepingBag,
            SleepingBagExtras,
            FoxNose,
            OcelotNose,
            WolfNose,
            VillagerNose,
            PigNose,
            WardenHorns,
        }
        impl ModelParts {
            pub const VALUES: [ModelParts; 12] = [ModelParts::Base, ModelParts::BaseDyed, ModelParts::Extras, ModelParts::Tanks, ModelParts::SleepingBag, ModelParts::SleepingBagExtras, ModelParts::FoxNose, ModelParts::OcelotNose, ModelParts::WolfNose, ModelParts::VillagerNose, ModelParts::PigNose, ModelParts::WardenHorns, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ModelParts::Base => "BASE",
                    ModelParts::BaseDyed => "BASE_DYED",
                    ModelParts::Extras => "EXTRAS",
                    ModelParts::Tanks => "TANKS",
                    ModelParts::SleepingBag => "SLEEPING_BAG",
                    ModelParts::SleepingBagExtras => "SLEEPING_BAG_EXTRAS",
                    ModelParts::FoxNose => "FOX_NOSE",
                    ModelParts::OcelotNose => "OCELOT_NOSE",
                    ModelParts::WolfNose => "WOLF_NOSE",
                    ModelParts::VillagerNose => "VILLAGER_NOSE",
                    ModelParts::PigNose => "PIG_NOSE",
                    ModelParts::WardenHorns => "WARDEN_HORNS",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "BASE" => ModelParts::Base,
                    "BASE_DYED" => ModelParts::BaseDyed,
                    "EXTRAS" => ModelParts::Extras,
                    "TANKS" => ModelParts::Tanks,
                    "SLEEPING_BAG" => ModelParts::SleepingBag,
                    "SLEEPING_BAG_EXTRAS" => ModelParts::SleepingBagExtras,
                    "FOX_NOSE" => ModelParts::FoxNose,
                    "OCELOT_NOSE" => ModelParts::OcelotNose,
                    "WOLF_NOSE" => ModelParts::WolfNose,
                    "VILLAGER_NOSE" => ModelParts::VillagerNose,
                    "PIG_NOSE" => ModelParts::PigNose,
                    "WARDEN_HORNS" => ModelParts::WardenHorns,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `BackpackDynamicModel.UnbakedBlockStateModel` (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java)
    pub mod backpack_dynamic_model_unbaked_block_state_model {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `CustomUnbakedBlockStateModel`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "backpack_loader"));
        // BackpackDynamicModel.UnbakedBlockStateModel.UnbakedBlockStateModel(1): not translated, its class isn't translated (it implements Fabric's `CustomUnbakedBlockStateModel`).
        // BackpackDynamicModel.UnbakedBlockStateModel.bake(1): not translated, its class isn't translated (it implements Fabric's `CustomUnbakedBlockStateModel`).
        // BackpackDynamicModel.UnbakedBlockStateModel.resolveDependencies(1): not translated, its class isn't translated (it implements Fabric's `CustomUnbakedBlockStateModel`).
        // BackpackDynamicModel.UnbakedBlockStateModel.codec(0): not translated, its class isn't translated (it implements Fabric's `CustomUnbakedBlockStateModel`).
        // BackpackDynamicModel.UnbakedBlockStateModel.variant(0): not translated, its class isn't translated (it implements Fabric's `CustomUnbakedBlockStateModel`).
    }

    /// `BackpackItemModel` (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java)
    pub mod backpack_item_model {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ItemModel`. Its static parts are below.
        // ITEM_TRANSFORMS: not translated, it uses Minecraft's `ItemTransforms`.
        // BackpackItemModel.createItemTransforms(0): not translated, it uses Minecraft's `ItemTransforms`.
        // BackpackItemModel.BackpackItemModel(2): not translated, its class isn't translated (it implements Minecraft's `ItemModel`).
        // BackpackItemModel.update(7): not translated, its class isn't translated (it implements Minecraft's `ItemModel`).
        // BackpackItemModel.setProperties(2): not translated, its class isn't translated (it implements Minecraft's `ItemModel`).
    }

    /// `BackpackItemModel.SpecialRenderer` (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java)
    pub mod backpack_item_model_special_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `NoDataSpecialModelRenderer`. Its static parts are below.
        // BackpackItemModel.SpecialRenderer.setModelRenderParameters(2): not translated, its class isn't translated (it implements Minecraft's `NoDataSpecialModelRenderer`).
        // BackpackItemModel.SpecialRenderer.submit(6): not translated, its class isn't translated (it implements Minecraft's `NoDataSpecialModelRenderer`).
        // BackpackItemModel.SpecialRenderer.getExtents(1): not translated, its class isn't translated (it implements Minecraft's `NoDataSpecialModelRenderer`).
    }

    /// `BackpackItemModel.Unbaked` (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java)
    pub mod backpack_item_model_unbaked {
        use super::*;
        // The class itself isn't translated: it implements `Unbaked`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // BackpackItemModel.Unbaked.Unbaked(2): not translated, its class isn't translated (it implements `Unbaked`).
        // BackpackItemModel.Unbaked.type(0): not translated, its class isn't translated (it implements `Unbaked`).
        // BackpackItemModel.Unbaked.bake(2): not translated, its class isn't translated (it implements `Unbaked`).
        // BackpackItemModel.Unbaked.resolveDependencies(1): not translated, its class isn't translated (it implements `Unbaked`).
        // BackpackItemModel.Unbaked.base(0): not translated, its class isn't translated (it implements `Unbaked`).
        // BackpackItemModel.Unbaked.tintSources(0): not translated, its class isn't translated (it implements `Unbaked`).
    }

    /// `BackpackModel` (com/tiviacz/travelersbackpack/client/model/BackpackModel.java)
    pub mod backpack_model {
        use super::*;
        // The class itself isn't translated: it extends `BackpackModelPart`. Its static parts are below.
        // BackpackModel.render(6): not translated, its class isn't translated (it extends `BackpackModelPart`).
    }

    /// `BackpackModelPart` (com/tiviacz/travelersbackpack/client/model/BackpackModelPart.java)
    pub mod backpack_model_part {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BackpackModelPart {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub x_rot: f32,
            pub y_rot: f32,
            pub z_rot: f32,
            pub x_scale: f32,
            pub y_scale: f32,
            pub z_scale: f32,
        }
        impl BackpackModelPart {
            pub fn new() -> Self {
                let mut s = Self { x: 0.0f32, y: 0.0f32, z: 0.0f32, x_rot: 0.0f32, y_rot: 0.0f32, z_rot: 0.0f32, x_scale: 1.0f32, y_scale: 1.0f32, z_scale: 1.0f32 };
                s
            }
            // BackpackModelPart.copyFrom(1): not translated, it uses Minecraft's `ModelPart`.
            // BackpackModelPart.translateAndRotate(1): not translated, PoseStack.rotate.
        }
    }

    /// `StackModelPart` (com/tiviacz/travelersbackpack/client/model/StackModelPart.java)
    pub mod stack_model_part {
        use super::*;
        // The class itself isn't translated: it extends `BackpackModelPart`. Its static parts are below.
        // StackModelPart.prepare(1): not translated, its class isn't translated (it extends `BackpackModelPart`).
        // StackModelPart.render(5): not translated, its class isn't translated (it extends `BackpackModelPart`).
        // StackModelPart.render(5): not translated, its class isn't translated (it extends `BackpackModelPart`).
        // StackModelPart.getDisplayContext(1): not translated, its class isn't translated (it extends `BackpackModelPart`).
    }

    /// `StarModelReloadListener` (com/tiviacz/travelersbackpack/client/model/StarModelReloadListener.java)
    pub mod star_model_reload_listener {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `SimpleSynchronousResourceReloadListener`. Its static parts are below.
        // INSTANCE: not translated, it uses StarModelReloadListener, which isn't translated (it implements Fabric's `SimpleSynchronousResourceReloadListener`).
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "star_model"));
        // StarModelReloadListener.getStarModel(0): not translated, its class isn't translated (it implements Fabric's `SimpleSynchronousResourceReloadListener`).
        // StarModelReloadListener.onResourceManagerReload(1): not translated, its class isn't translated (it implements Fabric's `SimpleSynchronousResourceReloadListener`).
        // StarModelReloadListener.getFabricId(0): not translated, its class isn't translated (it implements Fabric's `SimpleSynchronousResourceReloadListener`).
        // StarModelReloadListener.getFabricDependencies(0): not translated, its class isn't translated (it implements Fabric's `SimpleSynchronousResourceReloadListener`).
    }

    /// `SupporterBadgeModel` (com/tiviacz/travelersbackpack/client/model/SupporterBadgeModel.java)
    pub mod supporter_badge_model {
        use super::*;
        pub static IDENTITY_MATRIX4FC: LazyLock<JMat4> = LazyLock::new(|| JMat4::IDENTITY);
        #[derive(Clone, Debug)]
        pub struct SupporterBadgeModel {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub x_rot: f32,
            pub y_rot: f32,
            pub z_rot: f32,
            pub x_scale: f32,
            pub y_scale: f32,
            pub z_scale: f32,
        }
        impl SupporterBadgeModel {
            pub fn new() -> Self {
                let mut s = Self { x: 0.0f32, y: 0.0f32, z: 0.0f32, x_rot: 0.0f32, y_rot: 0.0f32, z_rot: 0.0f32, x_scale: 1.0f32, y_scale: 1.0f32, z_scale: 1.0f32 };
                s
            }
            // SupporterBadgeModel.copyFrom(1): not translated, it uses Minecraft's `ModelPart`.
            // SupporterBadgeModel.translateAndRotate(1): not translated, PoseStack.rotate.
            // SupporterBadgeModel.render(3): not translated, it uses Minecraft's `SubmitNodeCollector`.
            // SupporterBadgeModel.renderModel(4): not translated, it uses Minecraft's `SubmitNodeCollector`.
        }
    }

    /// `BackpackEntityLayer` (com/tiviacz/travelersbackpack/client/renderer/BackpackEntityLayer.java)
    pub mod backpack_entity_layer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RenderLayer`. Its static parts are below.
        // BackpackEntityLayer.BackpackEntityLayer(1): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // BackpackEntityLayer.submit(6): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
    }

    /// `BackpackLayer` (com/tiviacz/travelersbackpack/client/renderer/BackpackLayer.java)
    pub mod backpack_layer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RenderLayer`. Its static parts are below.
        // BACKPACK_MODEL: not translated, it uses BackpackModel, which isn't translated (it extends `BackpackModelPart`).
        // BackpackLayer.BackpackLayer(1): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // BackpackLayer.submit(6): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // BackpackLayer.renderBackpackLayer(8): not translated, it uses Minecraft's `HumanoidModel`.
        // BackpackLayer.alignModel(4): not translated, it uses Minecraft's `HumanoidModel`.
    }

    /// `HoseSpecialRenderer` (com/tiviacz/travelersbackpack/client/renderer/HoseSpecialRenderer.java)
    pub mod hose_special_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `SelectItemModelProperty`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // HoseSpecialRenderer.HoseSpecialRenderer(0): not translated, its class isn't translated (it implements Minecraft's `SelectItemModelProperty`).
        // HoseSpecialRenderer.register(0): not translated, it uses Minecraft's `SelectItemModelProperties`.ID_MAPPER.
        // HoseSpecialRenderer.get(5): not translated, its class isn't translated (it implements Minecraft's `SelectItemModelProperty`).
        // HoseSpecialRenderer.valueCodec(0): not translated, its class isn't translated (it implements Minecraft's `SelectItemModelProperty`).
        // HoseSpecialRenderer.type(0): not translated, its class isn't translated (it implements Minecraft's `SelectItemModelProperty`).
    }

    /// `AbstractBackpackScreen` (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java)
    pub mod abstract_backpack_screen {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub static BACKGROUND_11: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "textures/gui/background_11.png"));
        pub static BACKGROUND_9: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "textures/gui/background_9.png"));
        pub static SLOTS: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "textures/gui/slots.png"));
        pub static TANKS: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "textures/gui/tanks.png"));
        pub static ICONS: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "textures/gui/icons.png"));
        pub static TABS: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "textures/gui/tabs.png"));
        pub const TOP_BAR_OFFSET: i32 = 17;
        pub const HEIGHT_WITHOUT_STORAGE: i32 = 114;
        // AbstractBackpackScreen.AbstractBackpackScreen(3): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.getWrapper(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.getScreenPlayer(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.getScreenFont(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.getHoveredSlot(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.updateDimensions(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.updateBackpackSlotsPosition(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.updatePlayerSlotsPosition(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.calculateSlotHeight(1): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.calculateSlotHeight(1): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.getRows(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.getSlotsInRow(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.setScrollAmount(1): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.getMaxScrollAmount(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.renderInventoryBackground(6): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.renderSlots(4): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.renderLockedBackpackSlot(1): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.extractBackground(4): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.renderScreen(6): not translated, it has no body.
        // AbstractBackpackScreen.drawUnsortableSlots(1): not translated, it has no body.
        // AbstractBackpackScreen.drawMemorySlots(1): not translated, it has no body.
        // AbstractBackpackScreen.mouseScrolled(4): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.mouseDragged(3): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.mouseClicked(2): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.sendDataToServer(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.playUIClickSound(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.getGuiLeft(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.getGuiTop(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.getImageWidth(0): not translated, its class isn't translated (it's generic).
        // AbstractBackpackScreen.getImageHeight(0): not translated, its class isn't translated (it's generic).
    }

    /// `BackpackScreen` (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java)
    pub mod backpack_screen {
        use super::*;
        // The class itself isn't translated: it extends `AbstractBackpackScreen`. Its static parts are below.
        // BackpackScreen.BackpackScreen(3): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.init(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.resize(2): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.containerTick(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.recalculate(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.getMiddleBar(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.getWidthAdditions(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.updateScreen(1): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.renderScreen(6): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.renderUpgradeSlots(3): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.initializeUpgradeSlots(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.addRenderableWidget(1): not translated, it's generic.
        // BackpackScreen.initWidgets(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.initButtons(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.extractRenderState(4): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.extractTooltip(3): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.drawUnsortableSlots(1): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.drawMemorySlots(1): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.hasClickedOutside(4): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.mouseClicked(2): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.mouseScrolled(4): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.keyPressed(1): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.addCompatWidget(1): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.removeCompatWidget(1): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.displayTanksUpgradeWarning(1): not translated, it uses Minecraft's `Player`.
        // BackpackScreen.renderFluidWarningTooltip(3): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackScreen.createFluidWarning(2): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
    }

    /// `BackpackSettingsScreen` (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java)
    pub mod backpack_settings_screen {
        use super::*;
        // The class itself isn't translated: it extends `AbstractBackpackScreen`. Its static parts are below.
        // BackpackSettingsScreen.BackpackSettingsScreen(3): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.sendDataToServer(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.init(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.resize(2): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.recalculate(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.renderScreen(6): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.initWidgets(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.closeTabs(1): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.updateWidgetsPosition(1): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.extractRenderState(4): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.extractTooltip(3): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.drawUnsortableSlots(1): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.drawMemorySlots(1): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.slotClicked(4): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.mouseDragged(3): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.selectSlots(2): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.selectSlotsIndex(4): not translated, it uses Minecraft's `Slot`.
        // BackpackSettingsScreen.onClose(0): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackSettingsScreen.keyPressed(1): not translated, its class isn't translated (it extends `AbstractBackpackScreen`).
    }

    /// `AbilitySliderButton` (com/tiviacz/travelersbackpack/client/screens/buttons/AbilitySliderButton.java)
    pub mod ability_slider_button {
        use super::*;
        // The class itself isn't translated: it extends `Button`. Its static parts are below.
        // AbilitySliderButton.AbilitySliderButton(3): not translated, its class isn't translated (it extends `Button`).
        // AbilitySliderButton.render(4): not translated, its class isn't translated (it extends `Button`).
        // AbilitySliderButton.drawButton(4): not translated, its class isn't translated (it extends `Button`).
        // AbilitySliderButton.renderTooltip(3): not translated, its class isn't translated (it extends `Button`).
        // AbilitySliderButton.mouseClicked(2): not translated, its class isn't translated (it extends `Button`).
    }

    /// `Button` (com/tiviacz/travelersbackpack/client/screens/buttons/Button.java)
    pub mod button {
        use super::*;
        // The class itself isn't translated: it implements `IButton`. Its static parts are below.
        // Button.Button(5): not translated, its class isn't translated (it implements `IButton`).
        // Button.drawButton(8): not translated, its class isn't translated (it implements `IButton`).
        // Button.inButton(1): not translated, its class isn't translated (it implements `IButton`).
        // Button.inButton(2): not translated, its class isn't translated (it implements `IButton`).
        // Button.isWithinBounds(4): not translated, its class isn't translated (it implements `IButton`).
        // Button.isWithinBounds(3): not translated, its class isn't translated (it implements `IButton`).
    }

    /// `EquipButton` (com/tiviacz/travelersbackpack/client/screens/buttons/EquipButton.java)
    pub mod equip_button {
        use super::*;
        // The class itself isn't translated: it extends `Button`. Its static parts are below.
        // EquipButton.EquipButton(2): not translated, its class isn't translated (it extends `Button`).
        // EquipButton.render(4): not translated, its class isn't translated (it extends `Button`).
        // EquipButton.renderTooltip(3): not translated, its class isn't translated (it extends `Button`).
        // EquipButton.mouseClicked(2): not translated, its class isn't translated (it extends `Button`).
    }

    /// `IButton` (com/tiviacz/travelersbackpack/client/screens/buttons/IButton.java)
    pub mod i_button {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IButton.render(4): not translated, it has no body.
        // IButton.renderTooltip(3): not translated, it has no body.
        // IButton.mouseClicked(2): not translated, it has no body.
    }

    /// `MoreButton` (com/tiviacz/travelersbackpack/client/screens/buttons/MoreButton.java)
    pub mod more_button {
        use super::*;
        // The class itself isn't translated: it extends `Button`. Its static parts are below.
        // MoreButton.MoreButton(1): not translated, its class isn't translated (it extends `Button`).
        // MoreButton.render(4): not translated, its class isn't translated (it extends `Button`).
        // MoreButton.renderTooltip(3): not translated, its class isn't translated (it extends `Button`).
        // MoreButton.mouseClicked(2): not translated, its class isn't translated (it extends `Button`).
    }

    /// `SleepingBagButton` (com/tiviacz/travelersbackpack/client/screens/buttons/SleepingBagButton.java)
    pub mod sleeping_bag_button {
        use super::*;
        // The class itself isn't translated: it extends `Button`. Its static parts are below.
        // SleepingBagButton.SleepingBagButton(3): not translated, its class isn't translated (it extends `Button`).
        // SleepingBagButton.render(4): not translated, its class isn't translated (it extends `Button`).
        // SleepingBagButton.renderTooltip(3): not translated, its class isn't translated (it extends `Button`).
        // SleepingBagButton.mouseClicked(2): not translated, its class isn't translated (it extends `Button`).
    }

    /// `UnequipButton` (com/tiviacz/travelersbackpack/client/screens/buttons/UnequipButton.java)
    pub mod unequip_button {
        use super::*;
        // The class itself isn't translated: it extends `Button`. Its static parts are below.
        // UnequipButton.UnequipButton(1): not translated, its class isn't translated (it extends `Button`).
        // UnequipButton.render(4): not translated, its class isn't translated (it extends `Button`).
        // UnequipButton.renderTooltip(3): not translated, its class isn't translated (it extends `Button`).
        // UnequipButton.mouseClicked(2): not translated, its class isn't translated (it extends `Button`).
    }

    /// `HudOverlay` (com/tiviacz/travelersbackpack/client/screens/HudOverlay.java)
    pub mod hud_overlay {
        use super::*;
        pub static OVERLAY: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "textures/gui/overlay.png"));
        #[derive(Clone, Debug)]
        pub struct HudOverlay {
        }
        impl HudOverlay {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // HudOverlay.renderOverlay(2): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // HudOverlay.drawGuiTank(6): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
    }

    /// `RadialToolsOverlay` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java)
    pub mod radial_tools_overlay {
        use super::*;
        pub static TOOLS_OVERLAY: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "textures/gui/tools_overlay.png"));
        pub const ICON_SIZE: i32 = 16;
        pub const ITEM_RING_RADIUS: i32 = 42;
        pub const DEADZONE_RADIUS: i32 = 30;
        pub const ADD_NEW: i32 = -999;
        pub static DRAW_CROSSHAIR: JCell<bool> = JCell::new(true);
        #[derive(Clone, Debug)]
        pub struct RadialToolsOverlay {
        }
        impl RadialToolsOverlay {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RadialToolsOverlay.renderRadial(11): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // RadialToolsOverlay.renderRadialItems(11): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // RadialToolsOverlay.buildSegToSlot(3): not translated, it uses Minecraft's `NonNullList`.
        // RadialToolsOverlay.getTooltipFromItem(2): not translated, it uses Minecraft's `Component`.
        pub fn get_hovered_index(cx: i32, cy: i32, mx: i32, my: i32, segments: i32) -> i32 {
            if segments == 0 {
                return -1;
            }
            let dx: i32 = mx - cx;
            let dy: i32 = my - cy;
            if ((dx * dx) + (dy * dy)) < 900 {
                return -1;
            }
            let mut ang: f64 = (dy as f64).atan2((dx as f64));
            ang = radial_tools_overlay::normalize0_to2_pi(ang);
            let start: f64 = (std::f64::consts::PI * 3.0f64) / 2.0f64;
            let rel: f64 = radial_tools_overlay::normalize0_to2_pi(ang - start);
            let step: f64 = (std::f64::consts::PI * 2.0f64) / (segments as f64);
            let idx: i32 = ((((rel + (step / 2.0f64)) / step).floor()) as i32);
            return ((idx % segments) + segments) % segments;
        }
        // RadialToolsOverlay.renderCenteredText(4): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // RadialToolsOverlay.renderPlusButton(5): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // RadialToolsOverlay.renderCenteredItem(6): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        pub fn normalize0_to2_pi(mut a: f64) -> f64 {
            let two_pi: f64 = std::f64::consts::PI * 2.0f64;
            a %= two_pi;
            if a < 0.0f64 {
                a += two_pi;
            }
            return a;
        }
    }

    /// `ToolsScreen` (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java)
    pub mod tools_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub const REF_W: f64 = 1920.0f64;
        pub const REF_H: f64 = 1080.0f64;
        // hoseMenu: not translated, it uses Minecraft's `NonNullList`.
        // ToolsScreen.ToolsScreen(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ToolsScreen.getOpenProgress(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ToolsScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ToolsScreen.extractBackground(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ToolsScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ToolsScreen.getScaledWindow(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ToolsScreen.selectHoseAction(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ToolsScreen.getNextModeMessage(2): needs the game, so it's in game_side.rs.
        // ToolsScreen.getNonEmptyTools(1): not translated, it uses Minecraft's `NonNullList`.
        // ToolsScreen.getHoseMenu(0): not translated, it uses Minecraft's `NonNullList`.
        // ToolsScreen.isPauseScreen(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ToolsScreen.mouseScrolled(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ToolsScreen.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `BackpackTooltipComponent` (com/tiviacz/travelersbackpack/client/screens/tooltip/BackpackTooltipComponent.java)
    pub mod backpack_tooltip_component {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `TooltipComponent`. Its static parts are below.
        // BackpackTooltipComponent.BackpackTooltipComponent(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // BackpackTooltipComponent.BackpackTooltipComponent(2): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // BackpackTooltipComponent.isHoveredWithItem(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // BackpackTooltipComponent.loadComponentData(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // BackpackTooltipComponent.loadFluidStacks(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // BackpackTooltipComponent.loadStorage(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // BackpackTooltipComponent.loadUpgrades(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // BackpackTooltipComponent.mergeStacks(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // BackpackTooltipComponent.loadTools(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
    }

    /// `ClientBackpackTooltipComponent` (com/tiviacz/travelersbackpack/client/screens/tooltip/ClientBackpackTooltipComponent.java)
    pub mod client_backpack_tooltip_component {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ClientTooltipComponent`. Its static parts are below.
        // ClientBackpackTooltipComponent.ClientBackpackTooltipComponent(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientBackpackTooltipComponent.show(0): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientBackpackTooltipComponent.getHeight(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientBackpackTooltipComponent.getWidth(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientBackpackTooltipComponent.extractText(4): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientBackpackTooltipComponent.extractImage(6): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientBackpackTooltipComponent.renderItem(5): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientBackpackTooltipComponent.getFluidTankTooltip(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientBackpackTooltipComponent.renderFluidTankTooltip(5): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
    }

    /// `UpgradeSlot` (com/tiviacz/travelersbackpack/client/screens/UpgradeSlot.java)
    pub mod upgrade_slot {
        use super::*;
        // The class itself isn't translated: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`). Its static parts are below.
        pub const SLOT_SIZE: i32 = 24;
        pub const ICON_SIZE: i32 = 18;
        // UpgradeSlot.UpgradeSlot(6): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // UpgradeSlot.render(3): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // UpgradeSlot.getIndex(0): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeSlot.isHidden(0): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeSlot.getUpgradeSlotSizeAndPos(0): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
    }

    /// `FilterUpgradeWidgetBase` (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java)
    pub mod filter_upgrade_widget_base {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterUpgradeWidgetBase.FilterUpgradeWidgetBase(5): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.addFilterButton(1): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.hideButton(2): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.hasButton(1): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.getFilterButton(1): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.renderBg(5): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.extractRenderState(4): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.renderTooltip(3): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.updateAddableTags(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.isAdding(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.getSelectedTagIndex(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.setNextSelectedTagIndex(1): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.getTags(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.addTag(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.removeTag(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.getTooltip(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.calculateTagBoxY(1): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.getTrimmedText(1): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.isMouseOverTags(2): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.mouseScrolled(4): not translated, its class isn't translated (it's generic).
        // FilterUpgradeWidgetBase.mouseClicked(2): not translated, its class isn't translated (it's generic).
    }

    /// `InventoryScroll` (com/tiviacz/travelersbackpack/client/screens/widgets/InventoryScroll.java)
    pub mod inventory_scroll {
        use super::*;
        // The class itself isn't translated: it extends `ScrollPanel`. Its static parts are below.
        // InventoryScroll.InventoryScroll(6): not translated, its class isn't translated (it extends `ScrollPanel`).
        // InventoryScroll.getScrollAmount(0): not translated, its class isn't translated (it extends `ScrollPanel`).
        // InventoryScroll.getContentHeight(0): not translated, its class isn't translated (it extends `ScrollPanel`).
        // InventoryScroll.mouseScrolled(4): not translated, its class isn't translated (it extends `ScrollPanel`).
        // InventoryScroll.mouseDragged(3): not translated, its class isn't translated (it extends `ScrollPanel`).
        // InventoryScroll.mouseClicked(2): not translated, its class isn't translated (it extends `ScrollPanel`).
        // InventoryScroll.narrationPriority(0): not translated, its class isn't translated (it extends `ScrollPanel`).
        // InventoryScroll.updateNarration(1): not translated, its class isn't translated (it extends `ScrollPanel`).
        // InventoryScroll.setScrollDistance(1): not translated, its class isn't translated (it extends `ScrollPanel`).
    }

    /// `ScrollPanel` (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java)
    pub mod scroll_panel {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerEventHandler`. Its static parts are below.
        // ScrollPanel.ScrollPanel(6): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.ScrollPanel(7): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.ScrollPanel(10): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.getContentHeight(0): not translated, it has no body.
        // ScrollPanel.clickPanel(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.getMaxScroll(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.applyScrollLimits(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.mouseScrolled(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.getScrollAmount(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.isMouseOver(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.mouseReleased(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.getBarHeight(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.mouseDragged(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
        // ScrollPanel.children(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`).
    }

    /// `SettingsWidget` (com/tiviacz/travelersbackpack/client/screens/widgets/SettingsWidget.java)
    pub mod settings_widget {
        use super::*;
        // The class itself isn't translated: it extends `WidgetBase`. Its static parts are below.
        // SettingsWidget.SettingsWidget(3): not translated, its class isn't translated (it extends `WidgetBase`).
        // SettingsWidget.renderBg(5): not translated, its class isn't translated (it extends `WidgetBase`).
        // SettingsWidget.getSettingsUser(0): not translated, its class isn't translated (it extends `WidgetBase`).
        // SettingsWidget.renderTooltip(3): not translated, its class isn't translated (it extends `WidgetBase`).
        // SettingsWidget.mouseClicked(2): not translated, its class isn't translated (it extends `WidgetBase`).
    }

    /// `MemoryWidget` (com/tiviacz/travelersbackpack/client/screens/widgets/settings/MemoryWidget.java)
    pub mod memory_widget {
        use super::*;
        // The class itself isn't translated: it extends `SettingsWidgetBase`. Its static parts are below.
        // IGNORE_MODE_TOOLTIPS: a game value, so it's in game_side.rs.
        // MemoryWidget.MemoryWidget(3): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // MemoryWidget.isTabOpened(0): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // MemoryWidget.sendDataToServer(0): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // MemoryWidget.contains(2): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // MemoryWidget.renderBg(5): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // MemoryWidget.renderTooltip(3): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // MemoryWidget.isMouseOverMatchComponentsButton(2): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // MemoryWidget.mouseClicked(2): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
    }

    /// `SettingsWidgetBase` (com/tiviacz/travelersbackpack/client/screens/widgets/settings/SettingsWidgetBase.java)
    pub mod settings_widget_base {
        use super::*;
        // The class itself isn't translated: it extends `WidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets). Its static parts are below.
        // SettingsWidgetBase.SettingsWidgetBase(3): not translated, its class isn't translated (it extends `WidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // SettingsWidgetBase.updatePos(2): not translated, its class isn't translated (it extends `WidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // SettingsWidgetBase.isMouseOver(2): not translated, its class isn't translated (it extends `WidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // SettingsWidgetBase.getWidgetSizeAndPos(0): not translated, its class isn't translated (it extends `WidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
    }

    /// `SupporterBadgeWidget` (com/tiviacz/travelersbackpack/client/screens/widgets/settings/SupporterBadgeWidget.java)
    pub mod supporter_badge_widget {
        use super::*;
        // The class itself isn't translated: it extends `SettingsWidgetBase`. Its static parts are below.
        // SupporterBadgeWidget.SupporterBadgeWidget(2): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // SupporterBadgeWidget.sendDataToServer(0): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // SupporterBadgeWidget.renderBg(5): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // SupporterBadgeWidget.renderTooltip(3): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // SupporterBadgeWidget.mouseClicked(2): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
    }

    /// `UnsortablesWidget` (com/tiviacz/travelersbackpack/client/screens/widgets/settings/UnsortablesWidget.java)
    pub mod unsortables_widget {
        use super::*;
        // The class itself isn't translated: it extends `SettingsWidgetBase`. Its static parts are below.
        // UnsortablesWidget.UnsortablesWidget(2): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // UnsortablesWidget.isTabOpened(0): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // UnsortablesWidget.sendDataToServer(0): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // UnsortablesWidget.renderBg(5): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // UnsortablesWidget.renderTooltip(3): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // UnsortablesWidget.isMouseOverSelectAllButton(2): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // UnsortablesWidget.isMouseOverRemoveAllButton(2): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // UnsortablesWidget.mouseClicked(2): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // UnsortablesWidget.getWidgetSizeAndPos(0): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
    }

    /// `VisibilityWidget` (com/tiviacz/travelersbackpack/client/screens/widgets/settings/VisibilityWidget.java)
    pub mod visibility_widget {
        use super::*;
        // The class itself isn't translated: it extends `SettingsWidgetBase`. Its static parts are below.
        // VisibilityWidget.VisibilityWidget(2): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // VisibilityWidget.sendDataToServer(0): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // VisibilityWidget.renderBg(5): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // VisibilityWidget.renderTooltip(3): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
        // VisibilityWidget.mouseClicked(2): not translated, its class isn't translated (it extends `SettingsWidgetBase`).
    }

    /// `SortingButtons` (com/tiviacz/travelersbackpack/client/screens/widgets/SortingButtons.java)
    pub mod sorting_buttons {
        use super::*;
        // The class itself isn't translated: it extends `WidgetBase`. Its static parts are below.
        // SortingButtons.SortingButtons(4): not translated, its class isn't translated (it extends `WidgetBase`).
        // SortingButtons.extractRenderState(4): not translated, its class isn't translated (it extends `WidgetBase`).
        // SortingButtons.renderTooltip(3): not translated, its class isn't translated (it extends `WidgetBase`).
        // SortingButtons.mouseClicked(2): not translated, its class isn't translated (it extends `WidgetBase`).
        // SortingButtons.isButtonHovered(3): not translated, its class isn't translated (it extends `WidgetBase`).
        // SortingButtons.isButtonHovered(3): not translated, its class isn't translated (it extends `WidgetBase`).
        // SortingButtons.isButtonHovered(4): not translated, its class isn't translated (it extends `WidgetBase`).
    }

    /// `SortingButtons.Buttons` (com/tiviacz/travelersbackpack/client/screens/widgets/SortingButtons.java)
    pub mod sorting_buttons_buttons {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Buttons {
            Sort,
            QuickStack,
            TransferToBackpack,
            TransferToPlayer,
        }
        impl Buttons {
            pub const VALUES: [Buttons; 4] = [Buttons::Sort, Buttons::QuickStack, Buttons::TransferToBackpack, Buttons::TransferToPlayer, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Buttons::Sort => "SORT",
                    Buttons::QuickStack => "QUICK_STACK",
                    Buttons::TransferToBackpack => "TRANSFER_TO_BACKPACK",
                    Buttons::TransferToPlayer => "TRANSFER_TO_PLAYER",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "SORT" => Buttons::Sort,
                    "QUICK_STACK" => Buttons::QuickStack,
                    "TRANSFER_TO_BACKPACK" => Buttons::TransferToBackpack,
                    "TRANSFER_TO_PLAYER" => Buttons::TransferToPlayer,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ToolSlotsWidget` (com/tiviacz/travelersbackpack/client/screens/widgets/ToolSlotsWidget.java)
    pub mod tool_slots_widget {
        use super::*;
        // The class itself isn't translated: it extends `WidgetBase`. Its static parts are below.
        // ToolSlotsWidget.ToolSlotsWidget(3): not translated, its class isn't translated (it extends `WidgetBase`).
        // ToolSlotsWidget.extractRenderState(4): not translated, its class isn't translated (it extends `WidgetBase`).
        // ToolSlotsWidget.mouseClicked(2): not translated, its class isn't translated (it extends `WidgetBase`).
        // ToolSlotsWidget.renderTooltip(3): not translated, its class isn't translated (it extends `WidgetBase`).
        // ToolSlotsWidget.renderToolsAddition(4): not translated, its class isn't translated (it extends `WidgetBase`).
        // ToolSlotsWidget.getAdditionSizeAndPos(0): not translated, its class isn't translated (it extends `WidgetBase`).
    }

    /// `UpgradeWidgetBase` (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java)
    pub mod upgrade_widget_base {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // UpgradeWidgetBase.UpgradeWidgetBase(5): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.getUpgrade(0): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.renderBg(5): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.extractRenderState(4): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.renderTooltip(3): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.getAdditionalTooltips(1): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.mouseClicked(2): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.isMouseOver(2): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.isMouseOverRemoveButton(2): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.isMouseOverEnableButton(2): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.renderMatchContentsSlotOverlay(5): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.renderRemoveButton(3): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.removeButtonMouseClicked(2): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.removeUpgrade(0): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.renderEnableButton(4): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.renderEnableButtonTooltip(3): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.enableButtonMouseClicked(2): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.hasButtons(0): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.isBackpackOwner(0): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.isTabOpened(0): not translated, its class isn't translated (it's generic).
        // UpgradeWidgetBase.getWidgetSizeAndPos(0): not translated, its class isn't translated (it's generic).
    }

    /// `WidgetBase` (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java)
    pub mod widget_base {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // WidgetBase.WidgetBase(4): not translated, its class isn't translated (it's generic).
        // WidgetBase.getPos(0): not translated, its class isn't translated (it's generic).
        // WidgetBase.extractRenderState(4): not translated, its class isn't translated (it's generic).
        // WidgetBase.renderBg(5): not translated, its class isn't translated (it's generic).
        // WidgetBase.renderUnderTooltip(4): not translated, its class isn't translated (it's generic).
        // WidgetBase.renderTooltip(3): not translated, its class isn't translated (it's generic).
        // WidgetBase.renderAboveBg(6): not translated, its class isn't translated (it's generic).
        // WidgetBase.renderOnTop(4): not translated, its class isn't translated (it's generic).
        // WidgetBase.isMouseOver(2): not translated, its class isn't translated (it's generic).
        // WidgetBase.setFocused(1): not translated, its class isn't translated (it's generic).
        // WidgetBase.isFocused(0): not translated, its class isn't translated (it's generic).
        // WidgetBase.isMouseOverIcon(1): not translated, its class isn't translated (it's generic).
        // WidgetBase.isMouseOverIcon(2): not translated, its class isn't translated (it's generic).
        // WidgetBase.in(6): not translated, its class isn't translated (it's generic).
        // WidgetBase.isWithinBounds(4): not translated, its class isn't translated (it's generic).
        // WidgetBase.isWithinBounds(3): not translated, its class isn't translated (it's generic).
        // WidgetBase.narrationPriority(0): not translated, its class isn't translated (it's generic).
        // WidgetBase.updateNarration(1): not translated, its class isn't translated (it's generic).
        // WidgetBase.getWidgetSizeAndPos(0): not translated, its class isn't translated (it's generic).
    }

    /// `WidgetElement` (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetElement.java)
    pub mod widget_element {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct WidgetElement {
            pub pos: point::Point,
            pub size: point::Point,
        }
        impl WidgetElement {
            pub fn new(pos: &point::Point, size: &point::Point) -> Self {
                let mut s = Self { pos: pos.clone(), size: size.clone() };
                s.pos = pos.clone();
                s.size = size.clone();
                s
            }
            pub fn pos(&self) -> point::Point {
                return self.pos.clone();
            }
            pub fn size(&self) -> point::Point {
                return self.size.clone();
            }
        }
    }

    /// `AccessCommand` (com/tiviacz/travelersbackpack/commands/AccessCommand.java)
    pub mod access_command {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AccessCommand {
        }
        impl AccessCommand {
            // AccessCommand.AccessCommand(3): not translated, it uses Minecraft's `CommandDispatcher`.
            // AccessCommand.openTargetBlockEntity(2): not translated, it uses Minecraft's `CommandSourceStack`.
            // AccessCommand.openTargetInventory(2): not translated, it uses Minecraft's `CommandSourceStack`.
        }
    }

    /// `BackpackIconCommands` (com/tiviacz/travelersbackpack/commands/BackpackIconCommands.java)
    pub mod backpack_icon_commands {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BackpackIconCommands {
        }
        impl BackpackIconCommands {
            // BackpackIconCommands.BackpackIconCommands(2): not translated, it uses Minecraft's `CommandDispatcher`.
            // BackpackIconCommands.hideIcon(1): not translated, it uses Fabric's `FabricClientCommandSource`.
            // BackpackIconCommands.showIcon(0): not translated, it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec).
        }
    }

    /// `ClearCommand` (com/tiviacz/travelersbackpack/commands/ClearCommand.java)
    pub mod clear_command {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClearCommand {
        }
        impl ClearCommand {
            // ClearCommand.ClearCommand(3): not translated, it uses Minecraft's `CommandDispatcher`.
        }
        // ClearCommand.removeBackpack(2): not translated, it uses Minecraft's `CommandSourceStack`.
        // ClearCommand.clearBackpack(2): not translated, it uses Minecraft's `CommandSourceStack`.
    }

    /// `RestoreCommand` (com/tiviacz/travelersbackpack/commands/RestoreCommand.java)
    pub mod restore_command {
        use super::*;
        // SUGGESTION_PROVIDER: not translated, it uses Minecraft's `SuggestionProvider`.
        #[derive(Clone, Debug)]
        pub struct RestoreCommand {
        }
        impl RestoreCommand {
            // RestoreCommand.RestoreCommand(3): not translated, it uses Minecraft's `CommandDispatcher`.
            // RestoreCommand.restoreBackpack(3): not translated, it uses Minecraft's `CommandSourceStack`.
        }
    }

    /// `UnpackCommand` (com/tiviacz/travelersbackpack/commands/UnpackCommand.java)
    pub mod unpack_command {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct UnpackCommand {
        }
        impl UnpackCommand {
            // UnpackCommand.UnpackCommand(3): not translated, it uses Minecraft's `CommandDispatcher`.
            // UnpackCommand.unpackTargetBlockEntity(2): not translated, it uses Minecraft's `CommandSourceStack`.
            // UnpackCommand.unpackTargetInventory(2): not translated, it uses Minecraft's `CommandSourceStack`.
            // UnpackCommand.collectItems(1): not translated, it uses Minecraft's `NonNullList`.
            // UnpackCommand.collectItems(1): not translated, it uses Minecraft's `NonNullList`.
        }
    }

    /// `BackpackAbilities` (com/tiviacz/travelersbackpack/common/BackpackAbilities.java)
    pub mod backpack_abilities {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `AttributeModifier`. Its static parts are below.
        // ABILITIES: not translated, it uses BackpackAbilities, which isn't translated (it uses Minecraft's `AttributeModifier`).
        // ALLOWED_ABILITIES: a game value, so it's in game_side.rs.
        // BACKPACK_EFFECTS: not translated, it uses `Multimap` (com.google.common.collect).
        // COOLDOWNS: a game value, so it's in game_side.rs.
        // ITEM_ABILITIES_LIST: not translated, it uses Minecraft's `Item`.
        // ITEM_ABILITIES_REMOVAL_LIST: not translated, it uses Minecraft's `Item`.
        // BLOCK_ABILITIES_LIST: not translated, it uses Minecraft's `Item`.
        // CUSTOM_DESCRIPTIONS: not translated, it uses Minecraft's `Item`.
        // BackpackAbilities.abilityTick(2): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.abilityTickBlock(1): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.abilityRemoval(2): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.animateTick(5): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.getAttributeAbilityMultimap(1): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.attributeAbility(4): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.armorAbilityRemovals(1): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.lapisAbility(1): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.bookshelfAbility(2): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.spongeAbility(1): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.cakeAbility(2): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.chickenAbility(3): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.cactusAbilityEquipped(2): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.cactusAbilityBlockEntity(2): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.melonAbility(1): not translated, it uses BackpackBlockEntity, which isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackAbilities.pumpkinAbility(2): not translated, it uses Minecraft's `Player`.
        // BackpackAbilities.creeperAbility(1): not translated, it uses Minecraft's `Player`.
        // BackpackAbilities.blazeAbility(1): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.blazeAbility(3): not translated, it uses Minecraft's `EntityHitResult`.
        // BackpackAbilities.ghastAbility(3): not translated, it uses Minecraft's `Ghast`.
        // BackpackAbilities.spiderAbility(1): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.witherAbilityTick(1): not translated, it uses Minecraft's `Player`.
        // BackpackAbilities.witherAbility(2): not translated, it uses Minecraft's `Player`.
        // BackpackAbilities.wardenAbility(2): not translated, it uses Minecraft's `Player`.
        // BackpackAbilities.beeAbility(2): not translated, it uses Minecraft's `Player`.
        // BackpackAbilities.ocelotAbility(1): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.cowAbility(2): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.removeAllNegativeEffects(2): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.isUnderRain(2): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.isAbilityEnabledInConfig(1): not translated, it uses Minecraft's `ItemStack`.
        // BackpackAbilities.checkBackpack(2): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.addTimedMobEffect(8): not translated, its class isn't translated (it uses Minecraft's `AttributeModifier`).
        // BackpackAbilities.sendParticlesPacket(3): not translated, it uses Minecraft's `ParticleOptions`.
        // BackpackAbilities.isOnList(2): needs the game, so it's in game_side.rs.
        // BackpackAbilities.hasCooldown(1): not translated, it uses Minecraft's `ItemStack`.
        // BackpackAbilities.setCooldown(2): not translated, it uses Minecraft's `Item`.
        // BackpackAbilities.getAllowedAbilities(0): needs the game, so it's in game_side.rs.
        // BackpackAbilities.getCooldowns(0): needs the game, so it's in game_side.rs.
        // BackpackAbilities.getBackpackEffects(0): not translated, it uses `Multimap` (com.google.common.collect).
    }

    /// `BackpackManager` (com/tiviacz/travelersbackpack/common/BackpackManager.java)
    pub mod backpack_manager {
        use super::*;
        // BACKPACKS: not translated, it uses Minecraft's `LevelResource`.
        #[derive(Clone, Debug)]
        pub struct BackpackManager {
        }
        impl BackpackManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BackpackManager.addBackpack(2): not translated, it uses Minecraft's `ServerPlayer`.
        // BackpackManager.readBackpack(3): not translated, it uses Minecraft's `ItemStack`.
        // BackpackManager.getBackpack(2): not translated, it uses Minecraft's `ItemStack`.
        // BackpackManager.getBackpackFile(3): not translated, it uses `File` (java.io).
        // BackpackManager.getBackpackFile(2): not translated, it uses `File` (java.io).
        // BackpackManager.getPlayerBackpackFolder(1): not translated, it uses `File` (java.io).
        // BackpackManager.getPlayerBackpackFolder(2): not translated, it uses `File` (java.io).
        // BackpackManager.getBackpackFolder(1): not translated, it uses `File` (java.io).
        // BackpackManager.getWorldFolder(2): not translated, it uses `File` (java.io).
    }

    /// `BackpackUpgradeRecipe` (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java)
    pub mod backpack_upgrade_recipe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleSmithingRecipe`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // SERIALIZER: not translated, it uses Minecraft's `RecipeSerializer`.
        // BackpackUpgradeRecipe.BackpackUpgradeRecipe(5): not translated, its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`).
        // BackpackUpgradeRecipe.assemble(1): not translated, its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`).
        // BackpackUpgradeRecipe.upgradeInventory(2): not translated, its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`).
        // BackpackUpgradeRecipe.getUpgradedTanksCapacity(2): not translated, its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`).
        // BackpackUpgradeRecipe.templateIngredient(0): not translated, its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`).
        // BackpackUpgradeRecipe.baseIngredient(0): not translated, its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`).
        // BackpackUpgradeRecipe.additionIngredient(0): not translated, its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`).
        // BackpackUpgradeRecipe.getSerializer(0): not translated, its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`).
        // BackpackUpgradeRecipe.createPlacementInfo(0): not translated, its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`).
        // BackpackUpgradeRecipe.display(0): not translated, its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`).
    }

    /// `BackpackUpgradeRecipeBuilder` (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipeBuilder.java)
    pub mod backpack_upgrade_recipe_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Ingredient`. Its static parts are below.
        // BackpackUpgradeRecipeBuilder.BackpackUpgradeRecipeBuilder(5): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // BackpackUpgradeRecipeBuilder.backpackUpgrade(5): not translated, it uses BackpackUpgradeRecipeBuilder, which isn't translated (it uses Minecraft's `Ingredient`).
        // BackpackUpgradeRecipeBuilder.unlocks(2): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // BackpackUpgradeRecipeBuilder.save(2): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // BackpackUpgradeRecipeBuilder.save(2): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
    }

    /// `ShapedBackpackRecipe` (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java)
    pub mod shaped_backpack_recipe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `NormalCraftingRecipe`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // SERIALIZER: not translated, it uses Minecraft's `RecipeSerializer`.
        // ShapedBackpackRecipe.ShapedBackpackRecipe(4): not translated, its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`).
        // ShapedBackpackRecipe.assemble(1): not translated, its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`).
        // ShapedBackpackRecipe.getProperColor(1): not translated, it uses Minecraft's `Item`.
        // ShapedBackpackRecipe.getIngredients(0): not translated, its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`).
        // ShapedBackpackRecipe.createPlacementInfo(0): not translated, its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`).
        // ShapedBackpackRecipe.matches(2): not translated, its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`).
        // ShapedBackpackRecipe.getWidth(0): not translated, its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`).
        // ShapedBackpackRecipe.getHeight(0): not translated, its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`).
        // ShapedBackpackRecipe.display(0): not translated, its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`).
        // ShapedBackpackRecipe.getSerializer(0): not translated, its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`).
        // ShapedBackpackRecipe.getType(0): not translated, its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`).
    }

    /// `ShapedBackpackRecipeBuilder` (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java)
    pub mod shaped_backpack_recipe_builder {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RecipeBuilder`. Its static parts are below.
        // ShapedBackpackRecipeBuilder.ShapedBackpackRecipeBuilder(3): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.ShapedBackpackRecipeBuilder(4): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.shaped(3): not translated, it uses ShapedBackpackRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.shaped(4): not translated, it uses ShapedBackpackRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.define(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.define(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.define(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.pattern(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.unlockedBy(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.group(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.showNotification(1): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.defaultId(0): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
        // ShapedBackpackRecipeBuilder.save(2): not translated, its class isn't translated (it implements Minecraft's `RecipeBuilder`).
    }

    /// `ServerActions` (com/tiviacz/travelersbackpack/common/ServerActions.java)
    pub mod server_actions {
        use super::*;
        pub const TAB_OPEN: i32 = 0;
        pub const UPGRADE_ENABLED: i32 = 1;
        pub const SHIFT_CLICK_TO_BACKPACK: i32 = 2;
        pub const PLAY_RECORD: i32 = 3;
        pub const SLOT: i32 = 0;
        pub const TANK: i32 = 1;
        #[derive(Clone, Debug)]
        pub struct ServerActions {
        }
        impl ServerActions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ServerActions.swapTool(3): not translated, it uses Minecraft's `Player`.
        // ServerActions.equipBackpack(2): not translated, it uses Minecraft's `Player`.
        // ServerActions.swapBackpack(1): not translated, it uses Minecraft's `Player`.
        // ServerActions.runAbilitiesRemoval(1): not translated, it uses Minecraft's `Player`.
        // ServerActions.equipBackpack(1): not translated, it uses Minecraft's `Player`.
        // ServerActions.handleEquipBackpack(1): not translated, it uses Minecraft's `Player`.
        // ServerActions.unequipBackpack(1): not translated, it uses Minecraft's `Player`.
        // ServerActions.handleUnequipBackpack(1): not translated, it uses Minecraft's `Player`.
        // ServerActions.playEquippingSound(1): not translated, it uses Minecraft's `Player`.
        // ServerActions.openBackpackFromSlot(3): not translated, it uses Minecraft's `ServerPlayer`.
        // ServerActions.openBackpackSettings(3): not translated, it uses Minecraft's `ServerPlayer`.
        // ServerActions.modifyUpgradeTab(5): not translated, it uses Minecraft's `ServerPlayer`.
        // ServerActions.modifyUpgradeTab(4): not translated, it uses Minecraft's `ItemStack`.
        // ServerActions.getPacketType(1): not translated, it uses Minecraft's `DataComponentType`.
        // ServerActions.removeBackpackUpgrade(2): not translated, it uses Minecraft's `ServerPlayer`.
        // ServerActions.switchAbilitySlider(2): not translated, it uses Minecraft's `ServerPlayer`.
        // ServerActions.showToolSlots(2): not translated, it uses Minecraft's `ServerPlayer`.
        // ServerActions.sortBackpack(3): not translated, it uses Minecraft's `ServerPlayer`.
        // ServerActions.toggleVisibility(1): not translated, it uses Minecraft's `Player`.
        // ServerActions.toggleButtonsVisibility(1): not translated, it uses Minecraft's `Player`.
        // ServerActions.toggleSleepingBag(4): not translated, it uses Minecraft's `Player`.
        // ServerActions.placeAndUseSleepingBag(6): not translated, it uses Minecraft's `Player`.
        // ServerActions.setStack(4): not translated, it uses Minecraft's `Player`.
        // ServerActions.throwPotion(4): not translated, it uses Minecraft's `Level`.
        // ServerActions.setFluidEffect(3): not translated, it uses Minecraft's `Level`.
        // ServerActions.switchHoseMode(2): not translated, it uses Minecraft's `Player`.
        // ServerActions.toggleHoseTank(2): not translated, it uses Minecraft's `Player`.
    }

    /// `ComfortsCompat` (com/tiviacz/travelersbackpack/compat/comforts/ComfortsCompat.java)
    pub mod comforts_compat {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ComfortsCompat {
        }
        impl ComfortsCompat {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ComfortsCompat.getComfortsSleepingBagColor(1): not translated, it uses Minecraft's `Item`.
    }

    /// `BackpackCraftingGridAddition` (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridAddition.java)
    pub mod backpack_crafting_grid_addition {
        use super::*;
        // The class itself isn't translated: it implements `ICraftingTweaks`. Its static parts are below.
        // BackpackCraftingGridAddition.registerCraftingTweaksAddition(0): not translated, it uses ICraftingTweaks, which isn't translated (it's an interface).
        // BackpackCraftingGridAddition.addButton(1): not translated, its class isn't translated (it implements `ICraftingTweaks`).
        // BackpackCraftingGridAddition.onCraftingSlotsHidden(0): not translated, its class isn't translated (it implements `ICraftingTweaks`).
        // BackpackCraftingGridAddition.onCraftingSlotsDisplayed(0): not translated, its class isn't translated (it implements `ICraftingTweaks`).
        // BackpackCraftingGridAddition.setScreen(1): not translated, its class isn't translated (it implements `ICraftingTweaks`).
        // BackpackCraftingGridAddition.getButtonX(1): not translated, its class isn't translated (it implements `ICraftingTweaks`).
        // BackpackCraftingGridAddition.getButtonY(2): not translated, its class isn't translated (it implements `ICraftingTweaks`).
    }

    /// `BackpackCraftingGridProvider` (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java)
    pub mod backpack_crafting_grid_provider {
        use super::*;
        // The class itself isn't translated: it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api). Its static parts are below.
        // BackpackCraftingGridProvider.getModId(0): not translated, its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)).
        // BackpackCraftingGridProvider.requiresServerSide(0): not translated, its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)).
        // BackpackCraftingGridProvider.handles(1): not translated, its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)).
        // BackpackCraftingGridProvider.buildCraftingGrids(2): not translated, its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)).
        // BackpackCraftingGridProvider.clearGrid(4): not translated, its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)).
        // BackpackCraftingGridProvider.ignoresSlotId(1): not translated, its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)).
        // BackpackCraftingGridProvider.rotateSlotId(2): not translated, its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)).
        // BackpackCraftingGridProvider.rotateGrid(4): not translated, its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)).
    }

    /// `BackpackCraftingGridProvider.BackpackBalanceGridHandler` (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java)
    pub mod backpack_crafting_grid_provider_backpack_balance_grid_handler {
        use super::*;
        // The class itself isn't translated: it implements `GridBalanceHandler` (net.blay09.mods.craftingtweaks.api). Its static parts are below.
        // BackpackCraftingGridProvider.BackpackBalanceGridHandler.balanceGrid(3): not translated, its class isn't translated (it implements `GridBalanceHandler` (net.blay09.mods.craftingtweaks.api)).
        // BackpackCraftingGridProvider.BackpackBalanceGridHandler.spreadGrid(3): not translated, its class isn't translated (it implements `GridBalanceHandler` (net.blay09.mods.craftingtweaks.api)).
    }

    /// `BackpackCraftingGridProvider.BackpackTransferGridHandler` (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java)
    pub mod backpack_crafting_grid_provider_backpack_transfer_grid_handler {
        use super::*;
        // The class itself isn't translated: it implements `GridTransferHandler` (net.blay09.mods.craftingtweaks.api). Its static parts are below.
        // BackpackCraftingGridProvider.BackpackTransferGridHandler.putIntoGrid(5): not translated, its class isn't translated (it implements `GridTransferHandler` (net.blay09.mods.craftingtweaks.api)).
        // BackpackCraftingGridProvider.BackpackTransferGridHandler.transferIntoGrid(4): not translated, its class isn't translated (it implements `GridTransferHandler` (net.blay09.mods.craftingtweaks.api)).
        // BackpackCraftingGridProvider.BackpackTransferGridHandler.canTransferFrom(4): not translated, its class isn't translated (it implements `GridTransferHandler` (net.blay09.mods.craftingtweaks.api)).
    }

    /// `CraftingTweaksCompat` (com/tiviacz/travelersbackpack/compat/craftingtweaks/CraftingTweaksCompat.java)
    pub mod crafting_tweaks_compat {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CraftingTweaksCompat {
        }
        impl CraftingTweaksCompat {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CraftingTweaksCompat.registerCraftingTweaksAddition(0): not translated, it creates a BackpackCraftingGridProvider, which isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)).
        // CraftingTweaksCompat.registerCraftingTweaksAdditionClient(0): not translated, it calls BackpackCraftingGridAddition.registerCraftingTweaksAddition, which isn't translated.
    }

    /// `ICraftingTweaks` (com/tiviacz/travelersbackpack/compat/craftingtweaks/ICraftingTweaks.java)
    pub mod i_crafting_tweaks {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ICraftingTweaks.onCraftingSlotsHidden(0): not translated, it has no body.
        // ICraftingTweaks.onCraftingSlotsDisplayed(0): not translated, it has no body.
        // ICraftingTweaks.setScreen(1): not translated, it has no body.
    }

    /// `EntityModelFeaturesCompat` (com/tiviacz/travelersbackpack/compat/emf/EntityModelFeaturesCompat.java)
    pub mod entity_model_features_compat {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EntityModelFeaturesCompat {
        }
        impl EntityModelFeaturesCompat {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EntityModelFeaturesCompat.alignModel(4): not translated, it uses Minecraft's `HumanoidModel`.
    }

    /// `BackpackUpgradeCategoryExtension` (com/tiviacz/travelersbackpack/compat/jei/BackpackUpgradeCategoryExtension.java)
    pub mod backpack_upgrade_category_extension {
        use super::*;
        // The class itself isn't translated: it implements `ISmithingCategoryExtension` (mezz.jei.api.recipe.category.extensions.vanilla.smithing). Its static parts are below.
        // BackpackUpgradeCategoryExtension.setTemplate(2): not translated, it's generic.
        // BackpackUpgradeCategoryExtension.setBase(2): not translated, it's generic.
        // BackpackUpgradeCategoryExtension.setAddition(2): not translated, it's generic.
        // BackpackUpgradeCategoryExtension.setOutput(2): not translated, it's generic.
    }

    /// `BlockEntityTransferHandler` (com/tiviacz/travelersbackpack/compat/jei/BlockEntityTransferHandler.java)
    pub mod block_entity_transfer_handler {
        use super::*;
        // The class itself isn't translated: it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer). Its static parts are below.
        // BlockEntityTransferHandler.BlockEntityTransferHandler(4): not translated, its class isn't translated (it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer)).
        // BlockEntityTransferHandler.transferRecipe(6): not translated, its class isn't translated (it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer)).
    }

    /// `BlockEntityTransferInfo` (com/tiviacz/travelersbackpack/compat/jei/BlockEntityTransferInfo.java)
    pub mod block_entity_transfer_info {
        use super::*;
        // The class itself isn't translated: it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer). Its static parts are below.
        // BlockEntityTransferInfo.getContainerClass(0): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
        // BlockEntityTransferInfo.getMenuType(0): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
        // BlockEntityTransferInfo.getRecipeType(0): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
        // BlockEntityTransferInfo.canHandle(2): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
        // BlockEntityTransferInfo.getRecipeSlots(2): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
        // BlockEntityTransferInfo.getInventorySlots(2): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
    }

    /// `ItemTransferHandler` (com/tiviacz/travelersbackpack/compat/jei/ItemTransferHandler.java)
    pub mod item_transfer_handler {
        use super::*;
        // The class itself isn't translated: it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer). Its static parts are below.
        // ItemTransferHandler.ItemTransferHandler(4): not translated, its class isn't translated (it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer)).
        // ItemTransferHandler.transferRecipe(6): not translated, its class isn't translated (it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer)).
    }

    /// `ItemTransferInfo` (com/tiviacz/travelersbackpack/compat/jei/ItemTransferInfo.java)
    pub mod item_transfer_info {
        use super::*;
        // The class itself isn't translated: it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer). Its static parts are below.
        // ItemTransferInfo.getContainerClass(0): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
        // ItemTransferInfo.getMenuType(0): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
        // ItemTransferInfo.getRecipeType(0): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
        // ItemTransferInfo.canHandle(2): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
        // ItemTransferInfo.getRecipeSlots(2): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
        // ItemTransferInfo.getInventorySlots(2): not translated, its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)).
    }

    /// `JeiGhostIngredientHandler` (com/tiviacz/travelersbackpack/compat/jei/JeiGhostIngredientHandler.java)
    pub mod jei_ghost_ingredient_handler {
        use super::*;
        // The class itself isn't translated: it implements `IGhostIngredientHandler` (mezz.jei.api.gui.handlers). Its static parts are below.
        // JeiGhostIngredientHandler.getTargetsTyped(3): not translated, its body couldn't be read: an anonymous class.
        // JeiGhostIngredientHandler.onComplete(0): not translated, its class isn't translated (it implements `IGhostIngredientHandler` (mezz.jei.api.gui.handlers)).
    }

    /// `TravelersBackpackPlugin` (com/tiviacz/travelersbackpack/compat/jei/TravelersBackpackPlugin.java)
    pub mod travelers_backpack_plugin {
        use super::*;
        // The class itself isn't translated: it implements `IModPlugin` (mezz.jei.api). Its static parts are below.
        // TravelersBackpackPlugin.registerRecipeTransferHandlers(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // TravelersBackpackPlugin.registerVanillaCategoryExtensions(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // TravelersBackpackPlugin.registerGuiHandlers(1): not translated, its body couldn't be read: an anonymous class.
        // TravelersBackpackPlugin.getPluginUid(0): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
    }

    /// `BackpackDynamicLight` (com/tiviacz/travelersbackpack/compat/lambdynamiclight/BackpackDynamicLight.java)
    pub mod backpack_dynamic_light {
        use super::*;
        // The class itself isn't translated: it implements `DynamicLightsInitializer` (dev.lambdaurora.lambdynlights.api). Its static parts are below.
        // TYPE: not translated, it uses `Type` (dev.lambdaurora.lambdynlights.api.entity.luminance.EntityLuminance).
        // BackpackDynamicLight.onInitializeDynamicLights(1): not translated, its class isn't translated (it implements `DynamicLightsInitializer` (dev.lambdaurora.lambdynlights.api)).
    }

    /// `BackpackEntityLuminance` (com/tiviacz/travelersbackpack/compat/lambdynamiclight/BackpackEntityLuminance.java)
    pub mod backpack_entity_luminance {
        use super::*;
        // The class itself isn't translated: it implements `EntityLuminance` (dev.lambdaurora.lambdynlights.api.entity.luminance). Its static parts are below.
        // INSTANCE: not translated, it uses BackpackEntityLuminance, which isn't translated (it implements `EntityLuminance` (dev.lambdaurora.lambdynlights.api.entity.luminance)).
        // BackpackEntityLuminance.BackpackEntityLuminance(0): not translated, its class isn't translated (it implements `EntityLuminance` (dev.lambdaurora.lambdynlights.api.entity.luminance)).
        // BackpackEntityLuminance.type(0): not translated, its class isn't translated (it implements `EntityLuminance` (dev.lambdaurora.lambdynlights.api.entity.luminance)).
        // BackpackEntityLuminance.getLuminance(2): not translated, its class isn't translated (it implements `EntityLuminance` (dev.lambdaurora.lambdynlights.api.entity.luminance)).
    }

    /// `BackpackDataType` (com/tiviacz/travelersbackpack/compat/pneumonogravestones/BackpackDataType.java)
    pub mod backpack_data_type {
        use super::*;
        // The class itself isn't translated: it extends `GravestoneDataType` (net.pneumono.gravestones.api). Its static parts are below.
        pub const KEY: &str = "travelersbackpack";
        // BackpackDataType.writeData(3): not translated, its class isn't translated (it extends `GravestoneDataType` (net.pneumono.gravestones.api)).
        // BackpackDataType.onBreak(5): not translated, its class isn't translated (it extends `GravestoneDataType` (net.pneumono.gravestones.api)).
        // BackpackDataType.onCollect(6): not translated, its class isn't translated (it extends `GravestoneDataType` (net.pneumono.gravestones.api)).
    }

    /// `PneumonoGravestonesCompat` (com/tiviacz/travelersbackpack/compat/pneumonogravestones/PneumonoGravestonesCompat.java)
    pub mod pneumono_gravestones_compat {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PneumonoGravestonesCompat {
        }
        impl PneumonoGravestonesCompat {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PneumonoGravestonesCompat.register(0): not translated, it calls `Gravestones` (net.pneumono.gravestones).id.
    }

    /// `BackpackLayout` (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java)
    pub mod backpack_layout {
        use super::*;
        // The class itself isn't translated: it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout). Its static parts are below.
        pub static DEFAULT_SNAP_ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "bottom_right"));
        // DEFAULT_RECT_ID: not translated, it uses `ScreenBoundsProvider` (net.blay09.mods.trashslot.api.layout).SCREEN_ID.
        pub static EXTENDED_TOP_RECT_ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "extended_top"));
        pub static EXTENDED_BOTTOM_RECT_ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "extended_bottom"));
        // EXTENDED_TOP_RECT: not translated, it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`).
        // EXTENDED_BOTTOM_RECT: not translated, it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`).
        // BackpackLayout.getBackpackExtendedLayout(0): not translated, it uses `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout).
        // BackpackLayout.getDefaultLayout(0): not translated, it uses `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout).
        // BackpackLayout.getAvailability(0): not translated, its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)).
        // BackpackLayout.getSnaps(1): not translated, its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)).
        // BackpackLayout.getSnap(2): not translated, its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)).
        // BackpackLayout.getDefaultSnap(1): not translated, its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)).
        // BackpackLayout.getBounds(2): not translated, its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)).
        // BackpackLayout.getAllBounds(1): not translated, its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)).
    }

    /// `TrashSlotCompat` (com/tiviacz/travelersbackpack/compat/trashslot/TrashSlotCompat.java)
    pub mod trash_slot_compat {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TrashSlotCompat {
        }
        impl TrashSlotCompat {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TrashSlotCompat.register(0): not translated, it uses `RegisterTrashSlotContainerLayoutsEvent` (net.blay09.mods.trashslot.api.event).EVENT.
    }

    /// `BackpackTrinket` (com/tiviacz/travelersbackpack/compat/trinkets/BackpackTrinket.java)
    pub mod backpack_trinket {
        use super::*;
        // The class itself isn't translated: it implements `TrinketCallback` (eu.pb4.trinkets.api.callback). Its static parts are below.
        // BackpackTrinket.init(0): not translated, it uses Minecraft's `BuiltInRegistries`.ITEM.
        // BackpackTrinket.canEquip(3): not translated, its class isn't translated (it implements `TrinketCallback` (eu.pb4.trinkets.api.callback)).
        // BackpackTrinket.canEquipFromUse(2): not translated, its class isn't translated (it implements `TrinketCallback` (eu.pb4.trinkets.api.callback)).
        // BackpackTrinket.tick(3): not translated, its class isn't translated (it implements `TrinketCallback` (eu.pb4.trinkets.api.callback)).
    }

    /// `BackpackTrinketRenderer` (com/tiviacz/travelersbackpack/compat/trinkets/BackpackTrinketRenderer.java)
    pub mod backpack_trinket_renderer {
        use super::*;
        // The class itself isn't translated: it implements `TrinketRenderer` (eu.pb4.trinkets.api.client). Its static parts are below.
        // BackpackTrinketRenderer.initClient(0): not translated, it uses Minecraft's `BuiltInRegistries`.ITEM.
        // BackpackTrinketRenderer.submit(9): not translated, its class isn't translated (it implements `TrinketRenderer` (eu.pb4.trinkets.api.client)).
    }

    /// `UniversalGravesCompat` (com/tiviacz/travelersbackpack/compat/universalgraves/UniversalGravesCompat.java)
    pub mod universal_graves_compat {
        use super::*;
        // The class itself isn't translated: it implements `GraveInventoryMask` (eu.pb4.graves.grave). Its static parts are below.
        // INSTANCE: not translated, it uses `GraveInventoryMask` (eu.pb4.graves.grave).
        // UniversalGravesCompat.register(0): not translated, it uses UniversalGravesCompat.INSTANCE, which isn't translated (it uses `GraveInventoryMask` (eu.pb4.graves.grave)).
        // UniversalGravesCompat.addToGrave(2): not translated, its class isn't translated (it implements `GraveInventoryMask` (eu.pb4.graves.grave)).
        // UniversalGravesCompat.moveToPlayerExactly(4): not translated, its class isn't translated (it implements `GraveInventoryMask` (eu.pb4.graves.grave)).
        // UniversalGravesCompat.moveToPlayerClosest(4): not translated, its class isn't translated (it implements `GraveInventoryMask` (eu.pb4.graves.grave)).
    }

    /// `ServerboundVinURLStartPacket` (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStartPacket.java)
    pub mod serverbound_vin_url_start_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ServerboundVinURLStartPacket.ServerboundVinURLStartPacket(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundVinURLStartPacket.handle(2): not translated, it uses ServerboundVinURLStartPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundVinURLStartPacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundVinURLStartPacket.stack(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ServerboundVinURLStopPacket` (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStopPacket.java)
    pub mod serverbound_vin_url_stop_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ServerboundVinURLStopPacket.ServerboundVinURLStopPacket(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundVinURLStopPacket.handle(2): not translated, it uses ServerboundVinURLStopPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundVinURLStopPacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundVinURLStopPacket.stack(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundVinURLStopPacket.cancelable(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `VinURLNetwork` (com/tiviacz/travelersbackpack/compat/vinurl/VinURLNetwork.java)
    pub mod vin_url_network {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct VinURLNetwork {
        }
        impl VinURLNetwork {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VinURLNetwork.register(0): not translated, it calls Fabric's `PayloadTypeRegistry`.serverboundPlay.
    }

    /// `Fluids` (com/tiviacz/travelersbackpack/component/Fluids.java)
    pub mod fluids {
        use super::*;
        // The class itself isn't translated: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // Fluids.Fluids(2): not translated, its class isn't translated (it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`)).
        // Fluids.empty(0): not translated, it uses Fluids, which isn't translated (it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`)).
        // Fluids.leftFluidStack(0): not translated, its class isn't translated (it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`)).
        // Fluids.rightFluidStack(0): not translated, its class isn't translated (it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`)).
    }

    /// `RenderInfo` (com/tiviacz/travelersbackpack/component/RenderInfo.java)
    pub mod render_info {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `CompoundTag`. Its static parts are below.
        // EMPTY: not translated, it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`).
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        pub const LEFT_TANK: &str = "LeftTank";
        pub const RIGHT_TANK: &str = "RightTank";
        pub const CAPACITY: &str = "Capacity";
        pub const LANTERN: &str = "Lantern";
        // RenderInfo.RenderInfo(1): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
        // RenderInfo.isEmpty(0): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
        // RenderInfo.hasTanks(0): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
        // RenderInfo.hasLantern(0): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
        // RenderInfo.getLeftFluidStack(0): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
        // RenderInfo.getRightFluidStack(0): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
        // RenderInfo.updateCapacity(1): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
        // RenderInfo.getCapacity(0): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
        // RenderInfo.createCreativeTabInfo(0): not translated, it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`).
        // RenderInfo.equals(1): not translated, its body couldn't be read: expected `;` but found `(` (line 69).
        // RenderInfo.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
        // RenderInfo.compoundTag(0): not translated, its class isn't translated (it uses Minecraft's `CompoundTag`).
    }

    /// `Slots` (com/tiviacz/travelersbackpack/component/Slots.java)
    pub mod slots {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Pair`. Its static parts are below.
        // EMPTY: not translated, it uses Slots, which isn't translated (it uses Minecraft's `Pair`).
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // Slots.Slots(2): not translated, its class isn't translated (it uses Minecraft's `Pair`).
        // Slots.updateUnsortables(2): not translated, it uses Slots, which isn't translated (it uses Minecraft's `Pair`).
        // Slots.updateMemory(2): not translated, it uses Slots, which isn't translated (it uses Minecraft's `Pair`).
        // Slots.equals(1): not translated, its body couldn't be read: expected `;` but found `(` (line 49).
        // Slots.memoryMatch(1): not translated, its class isn't translated (it uses Minecraft's `Pair`).
        // Slots.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `Pair`).
        // Slots.hashSlots(2): not translated, it uses Minecraft's `Pair`.
        // Slots.unsortables(0): not translated, its class isn't translated (it uses Minecraft's `Pair`).
        // Slots.memory(0): not translated, its class isn't translated (it uses Minecraft's `Pair`).
    }

    /// `BackpackEffect` (com/tiviacz/travelersbackpack/config/BackpackEffect.java)
    pub mod backpack_effect {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Holder`. Its static parts are below.
        // BackpackEffect.BackpackEffect(4): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // BackpackEffect.effect(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // BackpackEffect.minDuration(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // BackpackEffect.maxDuration(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // BackpackEffect.amplifier(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
    }

    /// `Cooldown` (com/tiviacz/travelersbackpack/config/Cooldown.java)
    pub mod cooldown {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Cooldown {
            pub min_cooldown: i32,
            pub max_cooldown: i32,
        }
        impl Cooldown {
            pub fn new(min_cooldown: i32, max_cooldown: i32) -> Self {
                let mut s = Self { min_cooldown: min_cooldown, max_cooldown: max_cooldown };
                s.min_cooldown = min_cooldown;
                s.max_cooldown = max_cooldown;
                s
            }
            pub fn min_cooldown(&self) -> i32 {
                return self.min_cooldown;
            }
            pub fn max_cooldown(&self) -> i32 {
                return self.max_cooldown;
            }
        }
    }

    /// `TravelersBackpackConfig` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config {
        use super::*;
        // serverSpec: not translated, it uses `ModConfigSpec` (net.neoforged.neoforge.common).
        // SERVER: not translated, it uses Server, which isn't translated (it uses BackpackSettings, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec))).
        // commonSpec: not translated, it uses `ModConfigSpec` (net.neoforged.neoforge.common).
        // COMMON: not translated, it uses Common, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // clientSpec: not translated, it uses `ModConfigSpec` (net.neoforged.neoforge.common).
        // CLIENT: not translated, it uses Client, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        #[derive(Clone, Debug)]
        pub struct TravelersBackpackConfig {
        }
        impl TravelersBackpackConfig {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `TravelersBackpackConfig.Client` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_client {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Client.Client(1): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Client.Overlay` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_client_overlay {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Client.Overlay.Overlay(6): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Client.ToolsOverlay` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_client_tools_overlay {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Client.ToolsOverlay.ToolsOverlay(3): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Common` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_common {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Common.Common(1): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Server` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server {
        use super::*;
        // The class itself isn't translated: it uses BackpackSettings, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)). Its static parts are below.
        pub const REGISTRY_NAME_MATCHER: &str = "([a-z0-9_.-]+:[a-z0-9_/.-]+)";
        // TravelersBackpackConfig.Server.Server(1): not translated, it uses `Builder` (net.neoforged.neoforge.common.ModConfigSpec).
        // TravelersBackpackConfig.Server.loadItemsFromConfig(2): not translated, it uses Minecraft's `Item`.
        // TravelersBackpackConfig.Server.loadEntityTypesFromConfig(2): not translated, it uses Minecraft's `EntityType`.
        // TravelersBackpackConfig.Server.loadBackpackEffectsFromConfig(2): not translated, it uses `Multimap` (com.google.common.collect).
        // TravelersBackpackConfig.Server.loadCooldownsFromConfig(2): not translated, it uses Minecraft's `Item`.
        // TravelersBackpackConfig.Server.reload(0): not translated, it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)).
    }

    /// `TravelersBackpackConfig.Server.BackpackAbilities` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server_backpack_abilities {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        pub const REGISTRY_NAME_MATCHER: &str = "([a-z0-9_.-]+:[a-z0-9_/.-]+)";
        pub const EFFECT_ABILITY_MATCHER: &str = "([a-z0-9_.-]+:[a-z0-9_/.-]+),\\s*([a-z0-9_.-]+:[a-z0-9_/.-]+),\\s*(\\d+),\\s*(\\d+),\\s*(\\d+)";
        pub const COOLDOWNS_MATCHER: &str = "([a-z0-9_.-]+:[a-z0-9_/.-]+),\\s*(\\d+),\\s*(\\d+)";
        // TravelersBackpackConfig.Server.BackpackAbilities.BackpackAbilities(2): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // TravelersBackpackConfig.Server.BackpackAbilities.getAllowedAbilities(0): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // TravelersBackpackConfig.Server.BackpackAbilities.getCooldowns(0): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // TravelersBackpackConfig.Server.BackpackAbilities.getBackpackEffects(0): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Server.BackpackSettings` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server_backpack_settings {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Server.BackpackSettings.BackpackSettings(2): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Server.BackpackSettings.Tier` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server_backpack_settings_tier {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Tier {
            pub inventory_slot_count: i32,
            pub tool_slot_count: i32,
            pub tank_capacity: i32,
        }
        impl Tier {
            pub fn new(inventory_slot_count: i32, tool_slot_count: i32, tank_capacity: i32) -> Self {
                let mut s = Self { inventory_slot_count: inventory_slot_count, tool_slot_count: tool_slot_count, tank_capacity: tank_capacity };
                s.inventory_slot_count = inventory_slot_count;
                s.tool_slot_count = tool_slot_count;
                s.tank_capacity = tank_capacity;
                s
            }
            pub fn inventory_slot_count(&self) -> i32 {
                return self.inventory_slot_count;
            }
            pub fn tool_slot_count(&self) -> i32 {
                return self.tool_slot_count;
            }
            pub fn tank_capacity(&self) -> i32 {
                return self.tank_capacity;
            }
        }
    }

    /// `TravelersBackpackConfig.Server.BackpackSettings.TierConfig` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server_backpack_settings_tier_config {
        use super::*;
        // The class itself isn't translated: it uses `IntValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Server.BackpackSettings.TierConfig.TierConfig(6): not translated, its class isn't translated (it uses `IntValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Server.BackpackUpgrades` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server_backpack_upgrades {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Server.BackpackUpgrades.BackpackUpgrades(2): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Server.BackpackUpgrades.FeedingUpgradeSettings` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server_backpack_upgrades_feeding_upgrade_settings {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Server.BackpackUpgrades.FeedingUpgradeSettings.FeedingUpgradeSettings(2): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Server.BackpackUpgrades.FilterUpgradeSettings` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server_backpack_upgrades_filter_upgrade_settings {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Server.BackpackUpgrades.FilterUpgradeSettings.FilterUpgradeSettings(3): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Server.BackpackUpgrades.MagnetUpgradeSettings` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server_backpack_upgrades_magnet_upgrade_settings {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Server.BackpackUpgrades.MagnetUpgradeSettings.MagnetUpgradeSettings(2): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Server.BackpackUpgrades.RefillUpgradeSettings` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server_backpack_upgrades_refill_upgrade_settings {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Server.BackpackUpgrades.RefillUpgradeSettings.RefillUpgradeSettings(2): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Server.SlownessDebuff` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server_slowness_debuff {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Server.SlownessDebuff.SlownessDebuff(2): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `TravelersBackpackConfig.Server.World` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java)
    pub mod travelers_backpack_config_server_world {
        use super::*;
        // The class itself isn't translated: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec). Its static parts are below.
        // TravelersBackpackConfig.Server.World.World(2): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // TravelersBackpackConfig.Server.World.getPossibleOverworldEntityTypes(0): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // TravelersBackpackConfig.Server.World.getPossibleNetherEntityTypes(0): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // TravelersBackpackConfig.Server.World.getOverworldBackpacksList(0): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // TravelersBackpackConfig.Server.World.getNetherBackpacksList(0): not translated, its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
    }

    /// `ModBlockLootTables` (com/tiviacz/travelersbackpack/datagen/ModBlockLootTables.java)
    pub mod mod_block_loot_tables {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricBlockLootSubProvider`. Its static parts are below.
        // ModBlockLootTables.ModBlockLootTables(2): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // ModBlockLootTables.generate(0): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // ModBlockLootTables.createBackpackDrop(1): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
        // ModBlockLootTables.createSleepingBagDrop(1): not translated, its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`).
    }

    /// `ModDataGeneration` (com/tiviacz/travelersbackpack/datagen/ModDataGeneration.java)
    pub mod mod_data_generation {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `DataGeneratorEntrypoint`. Its static parts are below.
        // ModDataGeneration.onInitializeDataGenerator(1): not translated, its class isn't translated (it implements Fabric's `DataGeneratorEntrypoint`).
    }

    /// `ModRecipeProvider` (com/tiviacz/travelersbackpack/datagen/ModRecipeProvider.java)
    pub mod mod_recipe_provider {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `FabricRecipeProvider`. Its static parts are below.
        // BACKPACKS: not translated, it uses Minecraft's `Item`.
        // ModRecipeProvider.ModRecipeProvider(2): not translated, its class isn't translated (it extends Fabric's `FabricRecipeProvider`).
        // ModRecipeProvider.createRecipeProvider(3): not translated, its body couldn't be read: an anonymous class.
        // ModRecipeProvider.getName(0): not translated, its class isn't translated (it extends Fabric's `FabricRecipeProvider`).
    }

    /// `BackpackItemEntity` (com/tiviacz/travelersbackpack/entity/BackpackItemEntity.java)
    pub mod backpack_item_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ItemEntity`. Its static parts are below.
        // BackpackItemEntity.BackpackItemEntity(2): not translated, its class isn't translated (it extends Minecraft's `ItemEntity`).
        // BackpackItemEntity.tick(0): not translated, its class isn't translated (it extends Minecraft's `ItemEntity`).
        // BackpackItemEntity.isInWater(0): not translated, its class isn't translated (it extends Minecraft's `ItemEntity`).
        // BackpackItemEntity.fireImmune(0): not translated, its class isn't translated (it extends Minecraft's `ItemEntity`).
        // BackpackItemEntity.ignoreExplosion(1): not translated, its class isn't translated (it extends Minecraft's `ItemEntity`).
        // BackpackItemEntity.isInvulnerable(0): not translated, its class isn't translated (it extends Minecraft's `ItemEntity`).
        // BackpackItemEntity.onBelowWorld(0): not translated, its class isn't translated (it extends Minecraft's `ItemEntity`).
    }

    /// `EffectFluidRegistry` (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java)
    pub mod effect_fluid_registry {
        use super::*;
        // EFFECT_REGISTRY: not translated, it uses `BiMap` (com.google.common.collect).
        // WATER_EFFECT: not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // LAVA_EFFECT: not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // POTION_EFFECT: not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // MILK_EFFECT: not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // TAN_POTION_EFFECT: not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // TAN_WATER_CANTEEN_EFFECT: not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // TAN_WATER_EFFECT: not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        pub static EFFECT_ID_COUNTER: JCell<i32> = JCell::new(0);
        #[derive(Clone, Debug)]
        pub struct EffectFluidRegistry {
        }
        impl EffectFluidRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EffectFluidRegistry.initEffects(0): not translated, it uses EffectFluidRegistry.EFFECT_REGISTRY, which isn't translated (it uses `BiMap` (com.google.common.collect)).
        // EffectFluidRegistry.registerFluidEffect(1): not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluidRegistry.getFluidName(1): not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluidRegistry.getRegisteredFluidEffects(0): not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluidRegistry.getHighestFluidEffectAmount(1): not translated, it uses Minecraft's `Fluid`.
        // EffectFluidRegistry.getEffectsForFluid(1): not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluidRegistry.hasEffects(1): not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluidRegistry.hasExecutableEffects(3): not translated, it uses Minecraft's `Level`.
        // EffectFluidRegistry.getExecutableEffects(3): not translated, it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`).
        // EffectFluidRegistry.executeEffects(3): not translated, it uses Minecraft's `Entity`.
        pub fn can_initialize(modid: &str) -> bool {
            return false;
        }
    }

    /// `LavaEffect` (com/tiviacz/travelersbackpack/fluids/effects/LavaEffect.java)
    pub mod lava_effect {
        use super::*;
        // The class itself isn't translated: it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids). Its static parts are below.
        // LavaEffect.LavaEffect(0): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
        // LavaEffect.affectDrinker(3): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
        // LavaEffect.canExecuteEffect(3): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
    }

    /// `MilkEffect` (com/tiviacz/travelersbackpack/fluids/effects/MilkEffect.java)
    pub mod milk_effect {
        use super::*;
        // The class itself isn't translated: it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids). Its static parts are below.
        // MilkEffect.MilkEffect(0): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
        // MilkEffect.affectDrinker(3): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
        // MilkEffect.canExecuteEffect(3): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
    }

    /// `PotionEffect` (com/tiviacz/travelersbackpack/fluids/effects/PotionEffect.java)
    pub mod potion_effect {
        use super::*;
        // The class itself isn't translated: it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids). Its static parts are below.
        // PotionEffect.PotionEffect(2): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
        // PotionEffect.PotionEffect(3): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
        // PotionEffect.affectDrinker(3): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
        // PotionEffect.canExecuteEffect(3): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
    }

    /// `WaterEffect` (com/tiviacz/travelersbackpack/fluids/effects/WaterEffect.java)
    pub mod water_effect {
        use super::*;
        // The class itself isn't translated: it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids). Its static parts are below.
        // WaterEffect.WaterEffect(0): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
        // WaterEffect.affectDrinker(3): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
        // WaterEffect.canExecuteEffect(3): not translated, its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)).
    }

    /// `PotionFluid` (com/tiviacz/travelersbackpack/fluids/PotionFluid.java)
    pub mod potion_fluid {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `FlowingFluid`. Its static parts are below.
        // PotionFluid.getFlowing(0): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.getSource(0): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.getBucket(0): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.getDripParticle(0): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.beforeDestroyingBlock(3): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.getSlopeFindDistance(1): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.createLegacyBlock(1): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.isSame(1): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.getDropOff(1): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.getTickDelay(1): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.canBeReplacedWith(5): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.getExplosionResistance(0): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
        // PotionFluid.getPickupSound(0): not translated, its class isn't translated (it extends Minecraft's `FlowingFluid`).
    }

    /// `PotionFluid.Flowing` (com/tiviacz/travelersbackpack/fluids/PotionFluid.java)
    pub mod potion_fluid_flowing {
        use super::*;
        // The class itself isn't translated: it extends `PotionFluid`. Its static parts are below.
        // PotionFluid.Flowing.createFluidStateDefinition(1): not translated, its class isn't translated (it extends `PotionFluid`).
        // PotionFluid.Flowing.canConvertToSource(1): not translated, its class isn't translated (it extends `PotionFluid`).
        // PotionFluid.Flowing.isSource(1): not translated, its class isn't translated (it extends `PotionFluid`).
        // PotionFluid.Flowing.getAmount(1): not translated, its class isn't translated (it extends `PotionFluid`).
    }

    /// `PotionFluid.Still` (com/tiviacz/travelersbackpack/fluids/PotionFluid.java)
    pub mod potion_fluid_still {
        use super::*;
        // The class itself isn't translated: it extends `PotionFluid`. Its static parts are below.
        // PotionFluid.Still.isSource(1): not translated, its class isn't translated (it extends `PotionFluid`).
        // PotionFluid.Still.canConvertToSource(1): not translated, its class isn't translated (it extends `PotionFluid`).
        // PotionFluid.Still.getAmount(1): not translated, its class isn't translated (it extends `PotionFluid`).
    }

    /// `PotionFluidVariantAttributeHandler` (com/tiviacz/travelersbackpack/fluids/potion/PotionFluidVariantAttributeHandler.java)
    pub mod potion_fluid_variant_attribute_handler {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `FluidVariantAttributeHandler`. Its static parts are below.
        // PotionFluidVariantAttributeHandler.getName(1): not translated, its class isn't translated (it implements Fabric's `FluidVariantAttributeHandler`).
        // PotionFluidVariantAttributeHandler.getTranslationKey(1): not translated, its class isn't translated (it implements Fabric's `FluidVariantAttributeHandler`).
    }

    /// `PotionFluidVariantRenderHandler` (com/tiviacz/travelersbackpack/fluids/potion/PotionFluidVariantRenderHandler.java)
    pub mod potion_fluid_variant_render_handler {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `FluidVariantRenderHandler`. Its static parts are below.
        pub const EMPTY_COLOR: i32 = 16253176;
        // PotionFluidVariantRenderHandler.getColor(3): not translated, its class isn't translated (it implements Fabric's `FluidVariantRenderHandler`).
    }

    /// `DeathHandler` (com/tiviacz/travelersbackpack/handlers/DeathHandler.java)
    pub mod death_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DeathHandler {
        }
        impl DeathHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DeathHandler.registerListeners(0): not translated, it uses Fabric's `ServerLivingEntityEvents`.ALLOW_DEATH.
    }

    /// `EntityItemHandler` (com/tiviacz/travelersbackpack/handlers/EntityItemHandler.java)
    pub mod entity_item_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EntityItemHandler {
        }
        impl EntityItemHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register_listeners() {
        }
    }

    /// `KeybindHandler` (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java)
    pub mod keybind_handler {
        use super::*;
        pub static TRAVELERS_BACKPACK_PHASE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "phase"));
        // CATEGORY: not translated, it uses Minecraft's `Category`.
        // OPEN_BACKPACK: not translated, it uses Minecraft's `KeyMapping`.
        // SORT_BACKPACK: not translated, it uses Minecraft's `KeyMapping`.
        // ABILITY: not translated, it uses Minecraft's `KeyMapping`.
        // SWAP_TOOL: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_UPGRADE_0: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_UPGRADE_1: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_UPGRADE_2: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_UPGRADE_3: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_UPGRADE_KEYS: not translated, it uses Minecraft's `KeyMapping`.
        #[derive(Clone, Debug)]
        pub struct KeybindHandler {
        }
        impl KeybindHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // KeybindHandler.initKeybinds(0): not translated, it uses KeybindHandler.OPEN_BACKPACK, which isn't translated (it uses Minecraft's `KeyMapping`).
        // KeybindHandler.registerListener(0): not translated, it uses Fabric's `ClientTickEvents`.START_CLIENT_TICK.
        // KeybindHandler.isKeyDown(1): not translated, it uses Minecraft's `KeyMapping`.
        // KeybindHandler.sdlButtonMask(1): not translated, it uses `SDLMouse` (org.lwjgl.sdl).SDL_BUTTON_LMASK.
    }

    /// `LootHandler` (com/tiviacz/travelersbackpack/handlers/LootHandler.java)
    pub mod loot_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LootHandler {
        }
        impl LootHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LootHandler.registerListeners(0): not translated, it uses Common, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)).
        // LootHandler.addLootPool(3): not translated, it uses Minecraft's `Builder`.
        // LootHandler.addRandomDyedBackpackLoot(2): not translated, it uses Minecraft's `Builder`.
        // LootHandler.addBackpackWithTanksLoot(3): not translated, it uses Minecraft's `Builder`.
    }

    /// `RightClickHandler` (com/tiviacz/travelersbackpack/handlers/RightClickHandler.java)
    pub mod right_click_handler {
        use super::*;
        // UPGRADES: not translated, it uses Minecraft's `Item`.
        #[derive(Clone, Debug)]
        pub struct RightClickHandler {
        }
        impl RightClickHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RightClickHandler.registerListeners(0): not translated, it uses Fabric's `UseBlockCallback`.EVENT.
        // RightClickHandler.getUpgrades(1): not translated, it uses Minecraft's `NonNullList`.
    }

    /// `ScreenRenderHandler` (com/tiviacz/travelersbackpack/handlers/ScreenRenderHandler.java)
    pub mod screen_render_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ScreenRenderHandler {
        }
        impl ScreenRenderHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ScreenRenderHandler.renderAboveContents(4): not translated, it uses Minecraft's `AbstractContainerScreen`.
        // ScreenRenderHandler.registerScreenEvents(0): not translated, it uses Fabric's `ScreenEvents`.AFTER_INIT.
    }

    /// `SleepHandler` (com/tiviacz/travelersbackpack/handlers/SleepHandler.java)
    pub mod sleep_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SleepHandler {
        }
        impl SleepHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SleepHandler.registerListener(0): not translated, it uses Fabric's `EntitySleepEvents`.ALLOW_SETTING_SPAWN.
    }

    /// `TickHandler` (com/tiviacz/travelersbackpack/handlers/TickHandler.java)
    pub mod tick_handler {
        use super::*;
        pub static NEXT_SUPPORTERS_FETCH: JCell<i32> = JCell::new(0);
        #[derive(Clone, Debug)]
        pub struct TickHandler {
        }
        impl TickHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TickHandler.register(0): not translated, it uses Fabric's `ServerTickEvents`.END_SERVER_TICK.
    }

    /// `ModAdvancements` (com/tiviacz/travelersbackpack/init/ModAdvancements.java)
    pub mod mod_advancements {
        use super::*;
        // ACTION_TRIGGER: not translated, it uses ActionTypeTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        #[derive(Clone, Debug)]
        pub struct ModAdvancements {
        }
        impl ModAdvancements {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModAdvancements.init(0): not translated, it uses ModAdvancements.ACTION_TRIGGER, which isn't translated (it uses ActionTypeTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`)).
    }

    /// `ModAttachmentTypes` (com/tiviacz/travelersbackpack/init/ModAttachmentTypes.java)
    pub mod mod_attachment_types {
        use super::*;
        // TRAVELERS_BACKPACK: not translated, it uses Fabric's `AttachmentType`.
        #[derive(Clone, Debug)]
        pub struct ModAttachmentTypes {
        }
        impl ModAttachmentTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
    }

    /// `ModBlockEntityTypes` (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java)
    pub mod mod_block_entity_types {
        use super::*;
        // BACKPACK: not translated, it uses Minecraft's `BlockEntityType`.
        #[derive(Clone, Debug)]
        pub struct ModBlockEntityTypes {
        }
        impl ModBlockEntityTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModBlockEntityTypes.init(0): not translated, it uses ModBlockEntityTypes.BACKPACK, which isn't translated (it uses Minecraft's `BlockEntityType`).
        // ModBlockEntityTypes.initSidedStorage(0): not translated, it uses Fabric's `FluidStorage`.SIDED.
        // ModBlockEntityTypes.getProperTank(2): not translated, it uses Fabric's `SingleVariantStorage`.
        // ModBlockEntityTypes.getProperInventory(2): not translated, it uses Fabric's `Storage`.
    }

    /// `ModBlockEntityTypes.BackpackStorage` (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java)
    pub mod mod_block_entity_types_backpack_storage {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `SnapshotParticipant`. Its static parts are below.
        // ModBlockEntityTypes.BackpackStorage.BackpackStorage(1): not translated, its class isn't translated (it extends Fabric's `SnapshotParticipant`).
        // ModBlockEntityTypes.BackpackStorage.insert(3): not translated, its class isn't translated (it extends Fabric's `SnapshotParticipant`).
        // ModBlockEntityTypes.BackpackStorage.extract(3): not translated, its class isn't translated (it extends Fabric's `SnapshotParticipant`).
        // ModBlockEntityTypes.BackpackStorage.getSlotCount(0): not translated, its class isn't translated (it extends Fabric's `SnapshotParticipant`).
        // ModBlockEntityTypes.BackpackStorage.getSlot(1): not translated, its class isn't translated (it extends Fabric's `SnapshotParticipant`).
        // ModBlockEntityTypes.BackpackStorage.iterator(0): not translated, its class isn't translated (it extends Fabric's `SnapshotParticipant`).
        // ModBlockEntityTypes.BackpackStorage.createSnapshot(0): not translated, its class isn't translated (it extends Fabric's `SnapshotParticipant`).
        // ModBlockEntityTypes.BackpackStorage.readSnapshot(1): not translated, its class isn't translated (it extends Fabric's `SnapshotParticipant`).
    }

    /// `ModBlockEntityTypes.BackpackStorage.BackpackStorageView` (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java)
    pub mod mod_block_entity_types_backpack_storage_backpack_storage_view {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `SingleSlotStorage`. Its static parts are below.
        // ModBlockEntityTypes.BackpackStorage.BackpackStorageView.BackpackStorageView(1): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // ModBlockEntityTypes.BackpackStorage.BackpackStorageView.insert(3): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // ModBlockEntityTypes.BackpackStorage.BackpackStorageView.extract(3): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // ModBlockEntityTypes.BackpackStorage.BackpackStorageView.isResourceBlank(0): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // ModBlockEntityTypes.BackpackStorage.BackpackStorageView.getResource(0): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // ModBlockEntityTypes.BackpackStorage.BackpackStorageView.getAmount(0): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // ModBlockEntityTypes.BackpackStorage.BackpackStorageView.getCapacity(0): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
    }

    /// `ModBlocks` (com/tiviacz/travelersbackpack/init/ModBlocks.java)
    pub mod mod_blocks {
        use super::*;
        // STANDARD_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // NETHERITE_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // DIAMOND_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // GOLD_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // EMERALD_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // IRON_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // LAPIS_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // REDSTONE_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // COAL_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // QUARTZ_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // BOOKSHELF_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // END_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // NETHER_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // SANDSTONE_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // SNOW_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // SPONGE_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // CAKE_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // CACTUS_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // HAY_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // MELON_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // PUMPKIN_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // CREEPER_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // DRAGON_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // ENDERMAN_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // BLAZE_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // GHAST_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // MAGMA_CUBE_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // SKELETON_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // SPIDER_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // WITHER_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // WARDEN_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // BAT_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // BEE_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // WOLF_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // FOX_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // OCELOT_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // HORSE_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // COW_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // PIG_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // SHEEP_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // CHICKEN_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // SQUID_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // VILLAGER_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // IRON_GOLEM_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `Block`.
        // WHITE_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // ORANGE_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // MAGENTA_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // LIGHT_BLUE_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // YELLOW_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // LIME_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // PINK_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // GRAY_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // LIGHT_GRAY_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // CYAN_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // PURPLE_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // BLUE_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // BROWN_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // GREEN_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // RED_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        // BLACK_SLEEPING_BAG: not translated, it uses Minecraft's `Block`.
        #[derive(Clone, Debug)]
        pub struct ModBlocks {
        }
        impl ModBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModBlocks.init(0): not translated, it uses ModBlocks.STANDARD_TRAVELERS_BACKPACK, which isn't translated (it uses Minecraft's `Block`).
        pub fn resource_location(name: &str) -> JIdentifier {
            return JIdentifier::new("travelersbackpack", &name);
        }
        // ModBlocks.resourceKey(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `ModCommands` (com/tiviacz/travelersbackpack/init/ModCommands.java)
    pub mod mod_commands {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModCommands {
        }
        impl ModCommands {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModCommands.registerCommands(0): not translated, it uses Fabric's `CommandRegistrationCallback`.EVENT.
    }

    /// `ModDataComponents` (com/tiviacz/travelersbackpack/init/ModDataComponents.java)
    pub mod mod_data_components {
        use super::*;
        // TIER: not translated, it uses Minecraft's `DataComponentType`.
        // STORAGE_SLOTS: not translated, it uses Minecraft's `DataComponentType`.
        // UPGRADE_SLOTS: not translated, it uses Minecraft's `DataComponentType`.
        // TOOL_SLOTS: not translated, it uses Minecraft's `DataComponentType`.
        // FLUIDS: not translated, it uses Minecraft's `DataComponentType`.
        // TAB_OPEN: not translated, it uses Minecraft's `DataComponentType`.
        // FILTER_SETTINGS: not translated, it uses Minecraft's `DataComponentType`.
        // FILTER_TAGS: not translated, it uses Minecraft's `DataComponentType`.
        // UPGRADE_ENABLED: not translated, it uses Minecraft's `DataComponentType`.
        // SHIFT_CLICK_TO_BACKPACK: not translated, it uses Minecraft's `DataComponentType`.
        // SHOW_TOOL_SLOTS: not translated, it uses Minecraft's `DataComponentType`.
        // SHOW_MORE_BUTTONS: not translated, it uses Minecraft's `DataComponentType`.
        // IS_PLAYING: not translated, it uses Minecraft's `DataComponentType`.
        // COOLDOWN: not translated, it uses Minecraft's `DataComponentType`.
        // STARTER_UPGRADES: not translated, it uses Minecraft's `DataComponentType`.
        // RENDER_INFO: not translated, it uses Minecraft's `DataComponentType`.
        // BACKPACK_CONTAINER: not translated, it uses Minecraft's `DataComponentType`.
        // UPGRADES: not translated, it uses Minecraft's `DataComponentType`.
        // TOOLS_CONTAINER: not translated, it uses Minecraft's `DataComponentType`.
        // SLEEPING_BAG_COLOR: not translated, it uses Minecraft's `DataComponentType`.
        // SORT_TYPE: not translated, it uses Minecraft's `DataComponentType`.
        // ABILITY_ENABLED: not translated, it uses Minecraft's `DataComponentType`.
        // IS_VISIBLE: not translated, it uses Minecraft's `DataComponentType`.
        // SLOTS: not translated, it uses Minecraft's `DataComponentType`.
        // UPGRADE_TICK_INTERVAL: not translated, it uses Minecraft's `DataComponentType`.
        // HOSE_MODES: not translated, it uses Minecraft's `DataComponentType`.
        // BURN_FINISH_TIME: not translated, it uses Minecraft's `DataComponentType`.
        // COOKING_FINISH_TIME: not translated, it uses Minecraft's `DataComponentType`.
        // BURN_TOTAL_TIME: not translated, it uses Minecraft's `DataComponentType`.
        // COOKING_TOTAL_TIME: not translated, it uses Minecraft's `DataComponentType`.
        // SPEED_MULTIPLIER: not translated, it uses Minecraft's `DataComponentType`.
        #[derive(Clone, Debug)]
        pub struct ModDataComponents {
        }
        impl ModDataComponents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModDataComponents.init(0): not translated, it uses Minecraft's `BuiltInRegistries`.DATA_COMPONENT_TYPE.
    }

    /// `ModFluids` (com/tiviacz/travelersbackpack/init/ModFluids.java)
    pub mod mod_fluids {
        use super::*;
        // POTION_STILL: not translated, it uses Minecraft's `FlowingFluid`.
        // POTION_FLOWING: not translated, it uses Minecraft's `FlowingFluid`.
        #[derive(Clone, Debug)]
        pub struct ModFluids {
        }
        impl ModFluids {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModFluids.register(2): not translated, it uses Minecraft's `FlowingFluid`.
    }

    /// `ModItemGroups` (com/tiviacz/travelersbackpack/init/ModItemGroups.java)
    pub mod mod_item_groups {
        use super::*;
        // TRAVELERS_BACKPACK: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct ModItemGroups {
        }
        impl ModItemGroups {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModItemGroups.registerItemGroup(0): not translated, it uses Minecraft's `BuiltInRegistries`.CREATIVE_MODE_TAB.
        // ModItemGroups.createTabStack(0): not translated, it uses Minecraft's `ItemStack`.
        // ModItemGroups.addItemGroup(0): not translated, it uses ModItemGroups.TRAVELERS_BACKPACK, which isn't translated (it uses Minecraft's `ResourceKey`).
        // ModItemGroups.createTieredBackpack(1): not translated, it uses Minecraft's `ItemStack`.
        // ModItemGroups.withTanks(1): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `ModItems` (com/tiviacz/travelersbackpack/init/ModItems.java)
    pub mod mod_items {
        use super::*;
        // STANDARD_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // NETHERITE_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // DIAMOND_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // GOLD_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // EMERALD_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // IRON_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // LAPIS_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // REDSTONE_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // COAL_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // QUARTZ_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // BOOKSHELF_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // END_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // NETHER_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // SANDSTONE_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // SNOW_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // SPONGE_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // CAKE_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // CACTUS_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // HAY_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // MELON_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // PUMPKIN_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // CREEPER_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // DRAGON_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // ENDERMAN_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // BLAZE_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // GHAST_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // MAGMA_CUBE_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // SKELETON_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // SPIDER_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // WITHER_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // WARDEN_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // BAT_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // BEE_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // WOLF_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // FOX_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // OCELOT_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // HORSE_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // COW_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // PIG_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // SHEEP_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // CHICKEN_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // SQUID_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // VILLAGER_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // IRON_GOLEM_TRAVELERS_BACKPACK: not translated, it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`).
        // WHITE_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // ORANGE_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // MAGENTA_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // LIGHT_BLUE_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // YELLOW_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // LIME_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // PINK_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // GRAY_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // LIGHT_GRAY_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // CYAN_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // PURPLE_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // BLUE_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // BROWN_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // GREEN_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // RED_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // BLACK_SLEEPING_BAG: not translated, it uses Minecraft's `Item`.
        // BACKPACK_TANK: not translated, it uses Minecraft's `Item`.
        // HOSE: not translated, it uses Minecraft's `Item`.
        // HOSE_NOZZLE: not translated, it uses Minecraft's `Item`.
        // BLANK_UPGRADE: not translated, it uses Minecraft's `Item`.
        // IRON_TIER_UPGRADE: not translated, it uses Minecraft's `Item`.
        // GOLD_TIER_UPGRADE: not translated, it uses Minecraft's `Item`.
        // DIAMOND_TIER_UPGRADE: not translated, it uses Minecraft's `Item`.
        // NETHERITE_TIER_UPGRADE: not translated, it uses Minecraft's `Item`.
        // TANKS_UPGRADE: not translated, it uses Minecraft's `Item`.
        // CRAFTING_UPGRADE: not translated, it uses Minecraft's `Item`.
        // FURNACE_UPGRADE: not translated, it uses Minecraft's `Item`.
        // SMOKER_UPGRADE: not translated, it uses Minecraft's `Item`.
        // BLAST_FURNACE_UPGRADE: not translated, it uses Minecraft's `Item`.
        // PICKUP_UPGRADE: not translated, it uses Minecraft's `Item`.
        // MAGNET_UPGRADE: not translated, it uses Minecraft's `Item`.
        // JUKEBOX_UPGRADE: not translated, it uses Minecraft's `Item`.
        // VOID_UPGRADE: not translated, it uses Minecraft's `Item`.
        // FEEDING_UPGRADE: not translated, it uses Minecraft's `Item`.
        // REFILL_UPGRADE: not translated, it uses Minecraft's `Item`.
        // LANTERN_UPGRADE: not translated, it uses Minecraft's `Item`.
        // BACKPACK_ITEM_ENTITY: not translated, it uses Minecraft's `EntityType`.
        // COMPATIBLE_OVERWORLD_BACKPACK_ENTRIES: a game value, so it's in game_side.rs.
        // COMPATIBLE_NETHER_BACKPACK_ENTRIES: a game value, so it's in game_side.rs.
        #[derive(Clone, Debug)]
        pub struct ModItems {
        }
        impl ModItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModItems.init(0): not translated, it uses ModItems.STANDARD_TRAVELERS_BACKPACK, which isn't translated (it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`)).
        pub fn resource_location(name: &str) -> JIdentifier {
            return JIdentifier::new("travelersbackpack", &name);
        }
        // ModItems.resourceKey(1): not translated, it uses Minecraft's `ResourceKey`.
        // ModItems.registerItemFluidStorage(0): not translated, it uses Fabric's `FluidStorage`.ITEM.
    }

    /// `ModNetwork` (com/tiviacz/travelersbackpack/init/ModNetwork.java)
    pub mod mod_network {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModNetwork {
        }
        impl ModNetwork {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModNetwork.initClient(0): not translated, it uses Minecraft's `Type`.
        // ModNetwork.initServer(0): not translated, it calls Fabric's `PayloadTypeRegistry`.clientboundPlay.
    }

    /// `ModRecipeSerializers` (com/tiviacz/travelersbackpack/init/ModRecipeSerializers.java)
    pub mod mod_recipe_serializers {
        use super::*;
        // BACKPACK_SHAPED: not translated, it uses Minecraft's `RecipeSerializer`.
        // BACKPACK_UPGRADE: not translated, it uses Minecraft's `RecipeSerializer`.
        #[derive(Clone, Debug)]
        pub struct ModRecipeSerializers {
        }
        impl ModRecipeSerializers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModRecipeSerializers.init(0): not translated, it uses ModRecipeSerializers.BACKPACK_SHAPED, which isn't translated (it uses Minecraft's `RecipeSerializer`).
    }

    /// `ModScreenHandlerTypes` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java)
    pub mod mod_screen_handler_types {
        use super::*;
        // BACKPACK_MENU: not translated, it uses Fabric's `ExtendedMenuType`.
        // BACKPACK_BLOCK_MENU: not translated, it uses Fabric's `ExtendedMenuType`.
        // BACKPACK_SETTINGS_MENU: not translated, it uses Fabric's `ExtendedMenuType`.
        #[derive(Clone, Debug)]
        pub struct ModScreenHandlerTypes {
        }
        impl ModScreenHandlerTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModScreenHandlerTypes.init(0): not translated, it uses Minecraft's `BuiltInRegistries`.MENU.
    }

    /// `ModScreenHandlerTypes.BlockEntityScreenData` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java)
    pub mod mod_screen_handler_types_block_entity_screen_data {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // PACKET_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ModScreenHandlerTypes.BlockEntityScreenData.BlockEntityScreenData(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ModScreenHandlerTypes.BlockEntityScreenData.entityId(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ModScreenHandlerTypes.BlockEntityScreenData.pos(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `ModScreenHandlerTypes.ItemScreenData` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java)
    pub mod mod_screen_handler_types_item_screen_data {
        use super::*;
        // PACKET_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct ItemScreenData {
            pub screen_id: i32,
            pub entity_id: i32,
        }
        impl ItemScreenData {
            pub fn new(screen_id: i32, entity_id: i32) -> Self {
                let mut s = Self { screen_id: screen_id, entity_id: entity_id };
                s.screen_id = screen_id;
                s.entity_id = entity_id;
                s
            }
            pub fn screen_id(&self) -> i32 {
                return self.screen_id;
            }
            pub fn entity_id(&self) -> i32 {
                return self.entity_id;
            }
        }
    }

    /// `ModScreenHandlerTypes.SettingsScreenData` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java)
    pub mod mod_screen_handler_types_settings_screen_data {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // PACKET_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ModScreenHandlerTypes.SettingsScreenData.SettingsScreenData(4): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ModScreenHandlerTypes.SettingsScreenData.isBlockEntity(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ModScreenHandlerTypes.SettingsScreenData.screenId(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ModScreenHandlerTypes.SettingsScreenData.pos(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ModScreenHandlerTypes.SettingsScreenData.index(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `ModTags` (com/tiviacz/travelersbackpack/init/ModTags.java)
    pub mod mod_tags {
        use super::*;
        // BLACKLISTED_ITEMS: not translated, it uses Minecraft's `TagKey`.
        // ACCEPTABLE_TOOLS: not translated, it uses Minecraft's `TagKey`.
        // CUSTOM_TRAVELERS_BACKPACK: not translated, it uses Minecraft's `TagKey`.
        // SLEEPING_BAGS: not translated, it uses Minecraft's `TagKey`.
        // BACKPACK_UPGRADES: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct ModTags {
        }
        impl ModTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `BackpackContainer` (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java)
    pub mod backpack_container {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // BackpackContainer.BackpackContainer(4): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackContainer.saveExtraData(2): not translated, it's overloaded with the same number of parameters.
        // BackpackContainer.saveExtraData(2): not translated, it's overloaded with the same number of parameters.
        // BackpackContainer.openBackpack(3): not translated, its body couldn't be read: an anonymous class.
        // BackpackContainer.openBackpack(4): not translated, its body couldn't be read: an anonymous class.
        // BackpackContainer.openAnotherPlayerBackpack(4): not translated, its body couldn't be read: an anonymous class.
        // BackpackContainer.synchroniseToOpener(2): not translated, it uses Minecraft's `ServerPlayer`.
        // BackpackContainer.stack(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackContainer.player(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackContainer.screenID(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackContainer.index(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `BackpackSettingsContainer` (com/tiviacz/travelersbackpack/inventory/BackpackSettingsContainer.java)
    pub mod backpack_settings_container {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ExtendedMenuProvider`. Its static parts are below.
        // BackpackSettingsContainer.BackpackSettingsContainer(4): not translated, its class isn't translated (it implements Fabric's `ExtendedMenuProvider`).
        // BackpackSettingsContainer.getScreenOpeningData(1): not translated, its class isn't translated (it implements Fabric's `ExtendedMenuProvider`).
        // BackpackSettingsContainer.getDisplayName(0): not translated, its class isn't translated (it implements Fabric's `ExtendedMenuProvider`).
        // BackpackSettingsContainer.shouldCloseCurrentScreen(0): not translated, its class isn't translated (it implements Fabric's `ExtendedMenuProvider`).
        // BackpackSettingsContainer.createMenu(3): not translated, its class isn't translated (it implements Fabric's `ExtendedMenuProvider`).
        // BackpackSettingsContainer.openSettings(4): not translated, it uses Minecraft's `ServerPlayer`.
        // BackpackSettingsContainer.stack(0): not translated, its class isn't translated (it implements Fabric's `ExtendedMenuProvider`).
        // BackpackSettingsContainer.player(0): not translated, its class isn't translated (it implements Fabric's `ExtendedMenuProvider`).
        // BackpackSettingsContainer.screenID(0): not translated, its class isn't translated (it implements Fabric's `ExtendedMenuProvider`).
        // BackpackSettingsContainer.index(0): not translated, its class isn't translated (it implements Fabric's `ExtendedMenuProvider`).
    }

    /// `BackpackWrapper` (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java)
    pub mod backpack_wrapper {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // DUMMY: not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        pub const STORAGE_ID: i32 = 0;
        pub const UPGRADES_ID: i32 = 1;
        pub const TOOLS_ID: i32 = 2;
        // BackpackWrapper.BackpackWrapper(5): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.BackpackWrapper(4): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.BackpackWrapper(5): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.fromStack(1): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setBackpackStack(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getBackpackStack(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getBackpackSlotIndex(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setBackpackOwner(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getBackpackOwner(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getPlayersUsing(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.addUser(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setLevel(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getLevel(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getServerLevel(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.loadHandler(4): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.loadHandlers(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.loadStorage(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.loadUpgrades(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.loadTools(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.loadAdditionally(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setStarterUpgrade(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getStorageSize(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getUpgradesSize(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getToolSize(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getStorageForInputOutput(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getStorageForInputOutput(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getStorage(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getUpgrades(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getTools(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getUpgradeManager(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getRegistryAccess(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getUpgrade(1): not translated, it's generic.
        // BackpackWrapper.getUnsortableSlots(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getMemorySlots(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getScreenID(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getBackpackScreenTitle(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setUnsortableSlots(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setMemorySlots(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setDataAndSync(2): not translated, it's generic.
        // BackpackWrapper.setData(2): not translated, it's generic.
        // BackpackWrapper.showToolSlots(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.showMoreButtons(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.tanksVisible(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getBackpackTankCapacity(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setBackpackPos(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getBackpackPos(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setBackpackTankCapacity(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getRenderInfo(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setRenderInfo(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.updateRenderInfo(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.isAbilityEnabled(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getSortType(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setNextSortType(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getSleepingBagColor(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setSleepingBagColor(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.isOwner(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.isDyed(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getDyeColor(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getCooldown(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setCooldown(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.decreaseCooldown(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.canUpgradeTick(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.hasTickingUpgrade(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getUpgradeTickInterval(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.setUpgradeTickInterval(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.removeUpgradeTickInterval(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.isExtended(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getSlotsInRow(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getRows(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.sendDataToClients(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.getMemorizedSlot(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.createHandler(2): not translated, its body couldn't be read: an anonymous class.
        // BackpackWrapper.setSlotChanged(3): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.applyLowestTickInterval(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.updateMinimalTickInterval(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.createUpgradeHandler(2): not translated, its body couldn't be read: an anonymous class.
        // BackpackWrapper.isSizeInitialized(1): not translated, it uses Minecraft's `ItemStack`.
        // BackpackWrapper.initializeSize(1): not translated, it uses Minecraft's `ItemStack`.
        // BackpackWrapper.initializeSleepingBag(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.requestMenuAndScreenUpdate(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.requestMenuAndScreenUpdate(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.requestMenuUpdate(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.requestMenuUpdate(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.requestScreenUpdate(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.tickForBlockEntity(1): not translated, it uses BackpackBlockEntity, which isn't translated (it extends Minecraft's `BlockEntity`).
        // BackpackWrapper.getBackpackWrapper(3): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackWrapper.tick(3): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `CommonFluid` (com/tiviacz/travelersbackpack/inventory/CommonFluid.java)
    pub mod common_fluid {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommonFluid {
        }
        impl CommonFluid {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommonFluid.empty(0): not translated, it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`).
        // CommonFluid.getFluidName(1): not translated, it uses Minecraft's `Component`.
    }

    /// `FluidTank` (com/tiviacz/travelersbackpack/inventory/FluidTank.java)
    pub mod fluid_tank {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `SingleVariantStorage`. Its static parts are below.
        // FluidTank.FluidTank(1): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.FluidTank(2): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.setCapacity(1): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.setValidator(1): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.isFluidValid(1): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getBlankVariant(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getCapacity(1): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getCapacity(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getViewCapacity(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getFluid(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getFluidAmount(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.readFromNBT(2): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.writeToNBT(2): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getTanks(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getFluidInTank(1): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getTankCapacity(1): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.isFluidValid(2): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.fill(2): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.fill(3): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.drain(2): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.drain(2): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.drain(3): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.insert(3): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.extract(3): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.onContentsChanged(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.saveToStack(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.setFluid(1): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.isEmpty(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getSpace(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.isResourceBlank(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getResource(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.getAmount(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.createSnapshot(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.readSnapshot(1): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidTank.toString(0): not translated, its class isn't translated (it extends Fabric's `SingleVariantStorage`).
    }

    /// `FluidVariantWrapper` (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java)
    pub mod fluid_variant_wrapper {
        use super::*;
        // The class itself isn't translated: it uses Fabric's `FluidVariant`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // FluidVariantWrapper.FluidVariantWrapper(2): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.parse(1): not translated, it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.parseOptional(1): not translated, it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.save(0): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.saveOptional(0): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.isEmpty(0): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.getAmount(0): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.getViewAmount(0): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.blank(0): not translated, it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.copyWithAmount(1): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.copy(0): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.setAmount(1): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.grow(1): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.shrink(1): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.fluidVariant(0): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
        // FluidVariantWrapper.amount(0): not translated, its class isn't translated (it uses Fabric's `FluidVariant`).
    }

    /// `IItemHandlerModifiable` (com/tiviacz/travelersbackpack/inventory/handler/IItemHandlerModifiable.java)
    pub mod i_item_handler_modifiable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IItemHandlerModifiable.getSlots(0): not translated, it has no body.
        // IItemHandlerModifiable.getStackInSlot(1): not translated, it has no body.
        // IItemHandlerModifiable.insertItem(3): not translated, it has no body.
        // IItemHandlerModifiable.extractItem(3): not translated, it has no body.
        // IItemHandlerModifiable.getSlotLimit(1): not translated, it has no body.
        // IItemHandlerModifiable.isItemValid(2): not translated, it has no body.
        // IItemHandlerModifiable.setStackInSlot(2): not translated, it has no body.
    }

    /// `ItemStackHandler` (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java)
    pub mod item_stack_handler {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleContainer`. Its static parts are below.
        // ItemStackHandler.ItemStackHandler(0): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.ItemStackHandler(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.ItemStackHandler(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.setSize(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.setStackInSlot(2): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.getSlots(0): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.getStackInSlot(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.insertItem(3): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.extractItem(3): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.getSlotLimit(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.getStackLimit(2): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.isItemValid(2): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.serialize(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.deserialize(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.validateSlotIndex(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.onLoad(0): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.onContentsChanged(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.getContainerSize(0): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.isEmpty(0): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.getItem(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.removeItem(2): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.removeItemNoUpdate(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.setItem(2): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.setChanged(0): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.stillValid(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.clearContent(0): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // ItemStackHandler.canPlaceItem(2): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
    }

    /// `StorageAccessWrapper` (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java)
    pub mod storage_access_wrapper {
        use super::*;
        // The class itself isn't translated: it extends `ItemStackHandler`. Its static parts are below.
        // StorageAccessWrapper.StorageAccessWrapper(3): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.StorageAccessWrapper(2): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.setStackInSlot(2): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.getSlots(0): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.getStackInSlot(1): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.insertItem(3): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.matchesStack(2): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.tryVoiding(1): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.extractItem(3): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.getSlotLimit(1): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.isItemValid(2): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.canTakeItem(3): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.canPlaceItem(2): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.getContainerSize(0): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.isEmpty(0): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.getItem(1): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.removeItem(2): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.removeItemNoUpdate(1): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.setItem(2): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.setChanged(0): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.stillValid(1): not translated, its class isn't translated (it extends `ItemStackHandler`).
        // StorageAccessWrapper.clearContent(0): not translated, its class isn't translated (it extends `ItemStackHandler`).
    }

    /// `InventoryActions` (com/tiviacz/travelersbackpack/inventory/InventoryActions.java)
    pub mod inventory_actions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct InventoryActions {
        }
        impl InventoryActions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // InventoryActions.transferContainerTank(4): not translated, it uses TanksUpgrade, which isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // InventoryActions.transferFluid(6): not translated, its body couldn't be read: a labelled statement.
        // InventoryActions.transferPotion(8): not translated, it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`).
        // InventoryActions.isSameFluid(2): not translated, it uses Fabric's `Storage`.
        // InventoryActions.playFluidSound(4): not translated, it uses Minecraft's `Player`.
    }

    /// `AbstractBackpackMenu` (com/tiviacz/travelersbackpack/inventory/menu/AbstractBackpackMenu.java)
    pub mod abstract_backpack_menu {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerMenu`. Its static parts are below.
        // AbstractBackpackMenu.AbstractBackpackMenu(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // AbstractBackpackMenu.getWrapper(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // AbstractBackpackMenu.getPlayerInventory(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // AbstractBackpackMenu.addBackpackStorageSlots(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
    }

    /// `BackpackBaseMenu` (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java)
    pub mod backpack_base_menu {
        use super::*;
        // The class itself isn't translated: it extends `AbstractBackpackMenu`. Its static parts are below.
        // BackpackBaseMenu.BackpackBaseMenu(4): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.addSlots(0): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.rebuildSlots(0): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.addModifiableSlots(0): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.rebuildModifiableSlots(0): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.updateModifiableSlotsPosition(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.updateSlotsPosition(0): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.updateBackpackUpgradeSlots(0): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.updateUpgradeSlotsPosition(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.addSlot(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.addBackpackToolSlots(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.addBackpackUpgradeSlots(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.addPlayerInventoryAndHotbar(2): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.addUpgradeListeners(0): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.addUpgradeSlots(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.doClick(4): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.canCraft(2): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.slotsChanged(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.sendAllDataToRemote(0): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.canTakeItemForPickAll(2): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.quickMoveStack(2): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.checkMemorySlots(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.moveItemStackTo(4): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.handleShiftCraft(3): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.resetStackedContents(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.slotChangedCraftingGrid(3): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.removed(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackBaseMenu.clearSlotsAndPlaySound(4): not translated, it uses Minecraft's `Player`.
        // BackpackBaseMenu.clearSlot(4): not translated, it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`).
        // BackpackBaseMenu.playSound(1): not translated, it uses Minecraft's `Player`.
        // BackpackBaseMenu.stillValid(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
    }

    /// `BackpackBlockEntityMenu` (com/tiviacz/travelersbackpack/inventory/menu/BackpackBlockEntityMenu.java)
    pub mod backpack_block_entity_menu {
        use super::*;
        // The class itself isn't translated: it extends `BackpackBaseMenu`. Its static parts are below.
        // BackpackBlockEntityMenu.BackpackBlockEntityMenu(3): not translated, its class isn't translated (it extends `BackpackBaseMenu`).
        // BackpackBlockEntityMenu.BackpackBlockEntityMenu(4): not translated, its class isn't translated (it extends `BackpackBaseMenu`).
        // BackpackBlockEntityMenu.isInfiniteAccess(1): not translated, it uses BlockEntityScreenData, which isn't translated (it uses Minecraft's `BlockPos`).
        // BackpackBlockEntityMenu.getBlockEntity(2): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackBlockEntityMenu.stillValid(1): not translated, its class isn't translated (it extends `BackpackBaseMenu`).
        // BackpackBlockEntityMenu.removed(1): not translated, its class isn't translated (it extends `BackpackBaseMenu`).
    }

    /// `BackpackItemMenu` (com/tiviacz/travelersbackpack/inventory/menu/BackpackItemMenu.java)
    pub mod backpack_item_menu {
        use super::*;
        // The class itself isn't translated: it extends `BackpackBaseMenu`. Its static parts are below.
        // BackpackItemMenu.BackpackItemMenu(3): not translated, its class isn't translated (it extends `BackpackBaseMenu`).
        // BackpackItemMenu.BackpackItemMenu(3): not translated, its class isn't translated (it extends `BackpackBaseMenu`).
        // BackpackItemMenu.createWrapper(2): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackItemMenu.clicked(4): not translated, its class isn't translated (it extends `BackpackBaseMenu`).
        // BackpackItemMenu.addPlayerInventoryAndHotbar(2): not translated, its class isn't translated (it extends `BackpackBaseMenu`).
        // BackpackItemMenu.removed(1): not translated, its class isn't translated (it extends `BackpackBaseMenu`).
        // BackpackItemMenu.stillValid(1): not translated, its class isn't translated (it extends `BackpackBaseMenu`).
    }

    /// `BackpackSettingsMenu` (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java)
    pub mod backpack_settings_menu {
        use super::*;
        // The class itself isn't translated: it extends `AbstractBackpackMenu`. Its static parts are below.
        // BackpackSettingsMenu.BackpackSettingsMenu(3): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackSettingsMenu.BackpackSettingsMenu(3): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackSettingsMenu.BackpackSettingsMenu(4): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackSettingsMenu.updateSlots(0): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackSettingsMenu.addSlots(0): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackSettingsMenu.addPlayerInventoryAndHotbar(2): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackSettingsMenu.clicked(4): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackSettingsMenu.canTakeItemForPickAll(2): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackSettingsMenu.quickMoveStack(2): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackSettingsMenu.removed(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackSettingsMenu.stillValid(1): not translated, its class isn't translated (it extends `AbstractBackpackMenu`).
        // BackpackSettingsMenu.createWrapper(2): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackSettingsMenu.getWrapper(2): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // BackpackSettingsMenu.getBlockEntity(2): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `BackpackSlotItemHandler` (com/tiviacz/travelersbackpack/inventory/menu/slot/BackpackSlotItemHandler.java)
    pub mod backpack_slot_item_handler {
        use super::*;
        // The class itself isn't translated: it extends `SlotItemHandler`. Its static parts are below.
        // BLACKLISTED_ITEMS: a game value, so it's in game_side.rs.
        // BackpackSlotItemHandler.BackpackSlotItemHandler(4): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // BackpackSlotItemHandler.isItemValid(1): not translated, it uses Minecraft's `ItemStack`.
        // BackpackSlotItemHandler.onTake(2): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // BackpackSlotItemHandler.mayPlace(1): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // BackpackSlotItemHandler.mayPickup(1): not translated, its class isn't translated (it extends `SlotItemHandler`).
    }

    /// `CraftingSlot` (com/tiviacz/travelersbackpack/inventory/menu/slot/CraftingSlot.java)
    pub mod crafting_slot {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slot`. Its static parts are below.
        // CraftingSlot.CraftingSlot(4): not translated, its class isn't translated (it extends Minecraft's `Slot`).
    }

    /// `DisabledSlot` (com/tiviacz/travelersbackpack/inventory/menu/slot/DisabledSlot.java)
    pub mod disabled_slot {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slot`. Its static parts are below.
        // DisabledSlot.DisabledSlot(4): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // DisabledSlot.mayPickup(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
    }

    /// `FilterSlotItemHandler` (com/tiviacz/travelersbackpack/inventory/menu/slot/FilterSlotItemHandler.java)
    pub mod filter_slot_item_handler {
        use super::*;
        // The class itself isn't translated: it extends `SlotItemHandler`. Its static parts are below.
        // FilterSlotItemHandler.FilterSlotItemHandler(6): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // FilterSlotItemHandler.isActive(0): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // FilterSlotItemHandler.mayPlace(1): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // FilterSlotItemHandler.isFake(0): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // FilterSlotItemHandler.mayPickup(1): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // FilterSlotItemHandler.tryRemove(3): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // FilterSlotItemHandler.safeInsert(2): not translated, its class isn't translated (it extends `SlotItemHandler`).
    }

    /// `FluidSlotItemHandler` (com/tiviacz/travelersbackpack/inventory/menu/slot/FluidSlotItemHandler.java)
    pub mod fluid_slot_item_handler {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeSlotItemHandler`. Its static parts are below.
        // FluidSlotItemHandler.FluidSlotItemHandler(7): not translated, its class isn't translated (it extends `UpgradeSlotItemHandler`).
        // FluidSlotItemHandler.setByPlayer(2): not translated, its class isn't translated (it extends `UpgradeSlotItemHandler`).
        // FluidSlotItemHandler.mayPickup(1): not translated, its class isn't translated (it extends `UpgradeSlotItemHandler`).
    }

    /// `ResultSlotExt` (com/tiviacz/travelersbackpack/inventory/menu/slot/ResultSlotExt.java)
    pub mod result_slot_ext {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ResultSlot`. Its static parts are below.
        // ResultSlotExt.ResultSlotExt(7): not translated, its class isn't translated (it extends Minecraft's `ResultSlot`).
        // ResultSlotExt.mayPickup(1): not translated, its class isn't translated (it extends Minecraft's `ResultSlot`).
        // ResultSlotExt.isActive(0): not translated, its class isn't translated (it extends Minecraft's `ResultSlot`).
        // ResultSlotExt.remove(1): not translated, its class isn't translated (it extends Minecraft's `ResultSlot`).
        // ResultSlotExt.onSwapCraft(1): not translated, its class isn't translated (it extends Minecraft's `ResultSlot`).
        // ResultSlotExt.set(1): not translated, its class isn't translated (it extends Minecraft's `ResultSlot`).
        // ResultSlotExt.checkTakeAchievements(1): not translated, its class isn't translated (it extends Minecraft's `ResultSlot`).
        // ResultSlotExt.onTake(2): not translated, its class isn't translated (it extends Minecraft's `ResultSlot`).
        // ResultSlotExt.safeTake(3): not translated, its class isn't translated (it extends Minecraft's `ResultSlot`).
        // ResultSlotExt.getItem(0): not translated, its class isn't translated (it extends Minecraft's `ResultSlot`).
    }

    /// `SlotItemHandler` (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java)
    pub mod slot_item_handler {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slot`. Its static parts are below.
        // emptyInventory: not translated, it uses Minecraft's `Container`.
        // SlotItemHandler.SlotItemHandler(4): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotItemHandler.mayPlace(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotItemHandler.getItem(0): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotItemHandler.set(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotItemHandler.initialize(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotItemHandler.onQuickCraft(2): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotItemHandler.getMaxStackSize(0): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotItemHandler.getMaxStackSize(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotItemHandler.mayPickup(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotItemHandler.remove(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotItemHandler.getItemHandler(0): not translated, its class isn't translated (it extends Minecraft's `Slot`).
    }

    /// `ToolSlotItemHandler` (com/tiviacz/travelersbackpack/inventory/menu/slot/ToolSlotItemHandler.java)
    pub mod tool_slot_item_handler {
        use super::*;
        // The class itself isn't translated: it extends `SlotItemHandler`. Its static parts are below.
        // TOOL_SLOTS_ACCEPTABLE_ITEMS: a game value, so it's in game_side.rs.
        // ToolSlotItemHandler.ToolSlotItemHandler(4): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // ToolSlotItemHandler.isActive(0): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // ToolSlotItemHandler.mayPlace(1): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // ToolSlotItemHandler.mayPickup(1): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // ToolSlotItemHandler.isValid(1): not translated, it uses Minecraft's `ItemStack`.
        // ToolSlotItemHandler.setChanged(0): not translated, its class isn't translated (it extends `SlotItemHandler`).
    }

    /// `UpgradeLockableSlotItemHandler` (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeLockableSlotItemHandler.java)
    pub mod upgrade_lockable_slot_item_handler {
        use super::*;
        // The class itself isn't translated: it extends `SlotItemHandler`. Its static parts are below.
        // UpgradeLockableSlotItemHandler.UpgradeLockableSlotItemHandler(5): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // UpgradeLockableSlotItemHandler.setLocked(1): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // UpgradeLockableSlotItemHandler.setHidden(1): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // UpgradeLockableSlotItemHandler.mayPlace(1): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // UpgradeLockableSlotItemHandler.mayPickup(1): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // UpgradeLockableSlotItemHandler.isActive(0): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // UpgradeLockableSlotItemHandler.setByPlayer(2): not translated, its class isn't translated (it extends `SlotItemHandler`).
        // UpgradeLockableSlotItemHandler.setChanged(0): not translated, its class isn't translated (it extends `SlotItemHandler`).
    }

    /// `UpgradeSlotItemHandler` (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeSlotItemHandler.java)
    pub mod upgrade_slot_item_handler {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // UpgradeSlotItemHandler.UpgradeSlotItemHandler(5): not translated, its class isn't translated (it's generic).
        // UpgradeSlotItemHandler.getUpgradeParent(0): not translated, its class isn't translated (it's generic).
        // UpgradeSlotItemHandler.shiftClickToBackpack(0): not translated, its class isn't translated (it's generic).
    }

    /// `ContainerSorter` (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java)
    pub mod container_sorter {
        use super::*;
        pub const SORT_BACKPACK: i32 = 0;
        pub const QUICK_STACK: i32 = 1;
        pub const TRANSFER_TO_BACKPACK: i32 = 2;
        pub const TRANSFER_TO_PLAYER: i32 = 3;
        #[derive(Clone, Debug)]
        pub struct ContainerSorter {
        }
        impl ContainerSorter {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ContainerSorter.selectSort(4): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // ContainerSorter.sortBackpack(4): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // ContainerSorter.quickStackToBackpackNoSort(3): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // ContainerSorter.transferToBackpackNoSort(3): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // ContainerSorter.transferToPlayer(2): not translated, it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`).
        // ContainerSorter.addStackWithMerge(2): not translated, it uses Minecraft's `ItemStack`.
        // ContainerSorter.combineStacks(2): not translated, it uses Minecraft's `ItemStack`.
        // ContainerSorter.canMergeItems(2): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `ContainerSorter.CustomWrapper` (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java)
    pub mod container_sorter_custom_wrapper {
        use super::*;
        // The class itself isn't translated: it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler). Its static parts are below.
        // ContainerSorter.CustomWrapper.CustomWrapper(2): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // ContainerSorter.CustomWrapper.CustomWrapper(3): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // ContainerSorter.CustomWrapper.setStackInSlot(2): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // ContainerSorter.CustomWrapper.getSlots(0): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // ContainerSorter.CustomWrapper.getStackInSlot(1): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // ContainerSorter.CustomWrapper.insertItem(3): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // ContainerSorter.CustomWrapper.extractItem(3): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // ContainerSorter.CustomWrapper.getSlotLimit(1): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // ContainerSorter.CustomWrapper.isItemValid(2): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
    }

    /// `InvWrapper` (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java)
    pub mod inv_wrapper {
        use super::*;
        // The class itself isn't translated: it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler). Its static parts are below.
        // InvWrapper.InvWrapper(1): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // InvWrapper.equals(1): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // InvWrapper.hashCode(0): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // InvWrapper.getSlots(0): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // InvWrapper.getStackInSlot(1): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // InvWrapper.insertItem(3): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // InvWrapper.extractItem(3): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // InvWrapper.setStackInSlot(2): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // InvWrapper.getSlotLimit(1): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // InvWrapper.isItemValid(2): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // InvWrapper.getInv(0): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
    }

    /// `SortSelector` (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java)
    pub mod sort_selector {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SortSelector {
        }
        impl SortSelector {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SortSelector.getSortTypeComparator(2): not translated, it uses `Comparator` (java.util).
        // SortSelector.getCountTypeComparator(1): not translated, it uses `Comparator` (java.util).
        // SortSelector.calculateCount(1): needs the game, so it's in game_side.rs.
        // SortSelector.getStringForCategorySort(1): not translated, it uses Minecraft's `ItemStack`.
        // SortSelector.getStringForNameSort(1): not translated, it uses Minecraft's `ItemStack`.
        // SortSelector.specialCases(2): not translated, it uses Minecraft's `ItemStack`.
        // SortSelector.stackSize(2): not translated, it uses Minecraft's `ItemStack`.
        // SortSelector.enchantedBookNameCase(3): not translated, it uses Minecraft's `ItemStack`.
        // SortSelector.toolDurabilityCase(2): needs the game, so it's in game_side.rs.
    }

    /// `SortSelector.SortType` (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java)
    pub mod sort_selector_sort_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum SortType {
            Category,
            Name,
            Count,
        }
        impl SortType {
            pub const VALUES: [SortType; 3] = [SortType::Category, SortType::Name, SortType::Count, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    SortType::Category => "CATEGORY",
                    SortType::Name => "NAME",
                    SortType::Count => "COUNT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "CATEGORY" => SortType::Category,
                    "NAME" => SortType::Name,
                    "COUNT" => SortType::Count,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl SortType {
            // SortSelector.SortType.next(0): not translated, it calls SortType.values, which isn't in the mod's source.
        }
    }

    /// `Tiers` (com/tiviacz/travelersbackpack/inventory/Tiers.java)
    pub mod tiers {
        use super::*;
        pub static LEATHER: LazyLock<tiers_tier::Tier> = LazyLock::new(|| tiers_tier::Tier::new("leather", 27, 2, 2, 81000));
        pub static IRON: LazyLock<tiers_tier::Tier> = LazyLock::new(|| tiers_tier::Tier::new("iron", 45, 3, 3, 81000));
        pub static GOLD: LazyLock<tiers_tier::Tier> = LazyLock::new(|| tiers_tier::Tier::new("gold", 63, 4, 4, 81000));
        pub static DIAMOND: LazyLock<tiers_tier::Tier> = LazyLock::new(|| tiers_tier::Tier::new("diamond", 81, 5, 5, 81000));
        pub static NETHERITE: LazyLock<tiers_tier::Tier> = LazyLock::new(|| tiers_tier::Tier::new("netherite", 99, 6, 6, 81000));
        #[derive(Clone, Debug)]
        pub struct Tiers {
        }
        impl Tiers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Tiers.of(1): not translated, it's overloaded with the same number of parameters.
        // Tiers.of(1): not translated, it's overloaded with the same number of parameters.
    }

    /// `Tiers.Tier` (com/tiviacz/travelersbackpack/inventory/Tiers.java)
    pub mod tiers_tier {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Tier {
            pub name: String,
            pub tool_slots: i32,
            pub storage_slots: i32,
            pub upgrade_slots: i32,
            pub tank_capacity_per_row: i32,
        }
        impl Tier {
            pub fn new(name: &str, storage_slots: i32, upgrade_slots: i32, tool_slots: i32, tank_capacity_per_row: i32) -> Self {
                let mut s = Self { name: String::new(), tool_slots: tool_slots, storage_slots: storage_slots, upgrade_slots: upgrade_slots, tank_capacity_per_row: tank_capacity_per_row };
                s.name = name.to_string();
                s.storage_slots = storage_slots;
                s.upgrade_slots = upgrade_slots;
                s.tool_slots = tool_slots;
                s.tank_capacity_per_row = tank_capacity_per_row;
                s
            }
            pub fn get_name(&self) -> String {
                return self.name.to_string();
            }
            // Tiers.Tier.getLocalizedName(0): not translated, it uses Minecraft's `Component`.
            // Tiers.Tier.getStorageSlots(0): not translated, it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)).
            // Tiers.Tier.getUpgradeSlots(0): not translated, it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)).
            // Tiers.Tier.getToolSlots(0): not translated, it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)).
            // Tiers.Tier.getTankCapacityPerRow(0): not translated, it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)).
            // Tiers.Tier.getNextTier(0): not translated, `this` used as a value.
            // Tiers.Tier.getOrdinal(0): not translated, `this` used as a value.
            // Tiers.Tier.getTierUpgradeIngredient(0): not translated, it uses Minecraft's `Item`.
        }
    }

    /// `UpgradeManager` (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java)
    pub mod upgrade_manager {
        use super::*;
        // The class itself isn't translated: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`). Its static parts are below.
        // UpgradeManager.UpgradeManager(1): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.getWrapper(0): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.getUpgradesHandler(0): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.hasUpgradeInSlot(1): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.getUpgrade(1): not translated, it's generic.
        // UpgradeManager.canAddUpgrade(1): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.invalidateUpgrade(1): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.initializeUpgrades(0): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.detectedChange(2): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.applyUpgrade(1): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.getTabStatus(1): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.isTagSelector(2): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.isTagSelector(1): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
        // UpgradeManager.hasTickingUpgrade(0): not translated, its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)).
    }

    /// `CraftingContainerImproved` (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java)
    pub mod crafting_container_improved {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CraftingContainer`. Its static parts are below.
        // CraftingContainerImproved.CraftingContainerImproved(2): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.getContainerSize(0): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.getStackList(0): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.isEmpty(0): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.getItem(1): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.removeItemNoUpdate(1): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.removeItemShiftClick(2): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.removeItem(2): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.setItem(2): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.setChanged(0): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.stillValid(1): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.clearContent(0): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.getHeight(0): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.getWidth(0): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.getItems(0): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
        // CraftingContainerImproved.fillStackedContents(1): not translated, its class isn't translated (it implements Minecraft's `CraftingContainer`).
    }

    /// `CraftingUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingUpgrade.java)
    pub mod crafting_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        // CraftingUpgrade.CraftingUpgrade(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // CraftingUpgrade.createWidget(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // CraftingUpgrade.initializeContainers(2): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // CraftingUpgrade.onUpgradeRemoved(2): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // CraftingUpgrade.getUpgradeSlotsPosition(2): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // CraftingUpgrade.getUpgradeSlots(4): not translated, its body couldn't be read: an anonymous class.
        // CraftingUpgrade.setSlotChanged(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // CraftingUpgrade.createHandler(1): not translated, its body couldn't be read: an anonymous class.
    }

    /// `CraftingWidget` (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingWidget.java)
    pub mod crafting_widget {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets). Its static parts are below.
        // craftingTweaksAddition: not translated, it uses ICraftingTweaks, which isn't translated (it's an interface).
        // CraftingWidget.CraftingWidget(3): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // CraftingWidget.renderBg(5): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // CraftingWidget.renderTooltip(3): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // CraftingWidget.mouseClicked(2): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // CraftingWidget.setCraftingTweaksAddition(1): not translated, it uses ICraftingTweaks, which isn't translated (it's an interface).
        // CraftingWidget.getCraftingTweaksAddition(0): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // CraftingWidget.isCraftingTweaksAdditionEnabled(0): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
    }

    /// `FeedingFilterSettings` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java)
    pub mod feeding_filter_settings {
        use super::*;
        // The class itself isn't translated: it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        pub const ALLOW_MODE: i32 = 0;
        pub const HUNGER_MODE: i32 = 1;
        pub const IGNORE_EFFECT_MODE: i32 = 2;
        pub const ALLOW: i32 = 0;
        pub const BLOCK: i32 = 1;
        pub const ALWAYS_EAT: i32 = 0;
        pub const HALF_NUTRITION: i32 = 1;
        pub const FULL_NUTRITION: i32 = 2;
        pub const BLOCK_BAD_EFFECTS: i32 = 0;
        pub const ALLOW_BAD_EFFECTS: i32 = 1;
        // FeedingFilterSettings.FeedingFilterSettings(3): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingFilterSettings.matchesFilter(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingFilterSettings.compareHungerLevel(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingFilterSettings.checkHarmfulEffects(1): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingFilterSettings.checkHarmfulEffect(1): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingFilterSettings.getNutritionDifference(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingFilterSettings.getHalfOfStackHunger(1): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
    }

    /// `FeedingUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java)
    pub mod feeding_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        pub const STILL_HUNGRY_COOLDOWN: i32 = 10;
        pub const FEEDING_RANGE: f64 = 3.0f64;
        // FeedingUpgrade.FeedingUpgrade(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingUpgrade.hasTagSelector(0): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingUpgrade.getFilter(0): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingUpgrade.createFilterSettings(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingUpgrade.canEat(2): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingUpgrade.createWidget(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingUpgrade.createFilter(2): not translated, its body couldn't be read: an anonymous class.
        // FeedingUpgrade.getTickRate(0): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingUpgrade.tick(4): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingUpgrade.feedPlayerAndGetHungry(2): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingUpgrade.tryFeedingFoodFromStorage(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingUpgrade.tryFeedingStack(6): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // FeedingUpgrade.isEdible(2): not translated, it uses Minecraft's `ItemStack`.
        // FeedingUpgrade.spawnHeartParticles(2): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
    }

    /// `FeedingWidget` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingWidget.java)
    pub mod feeding_widget {
        use super::*;
        // The class itself isn't translated: it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets). Its static parts are below.
        // WHITELIST_TOOLTIPS: a game value, so it's in game_side.rs.
        // HUNGER_MODE_TOOLTIPS: a game value, so it's in game_side.rs.
        // IGNORE_MODE_TOOLTIPS: a game value, so it's in game_side.rs.
        // FeedingWidget.FeedingWidget(3): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // FeedingWidget.renderTooltip(3): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
    }

    /// `FilterSettingsBase` (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java)
    pub mod filter_settings_base {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // FilterSettingsBase.FilterSettingsBase(4): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.FilterSettingsBase(5): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.matchesFilter(2): not translated, it has no body.
        // FilterSettingsBase.getSettings(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.isTagFilter(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.compareModId(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.streamStorageContents(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.reloadItemTags(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.updateFilter(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.updateFilterTags(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.updateSettings(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.getFilterItems(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.getFilterTags(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FilterSettingsBase.getAddableTags(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `FilterUpgradeBase` (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java)
    pub mod filter_upgrade_base {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterUpgradeBase.FilterUpgradeBase(7): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.getFilterSettings(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.getFilter(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.getFilterSlotCount(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.getSlotsInRow(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.updateSettings(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.hasTagSelector(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.isTagSelector(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.getFirstFilterStack(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.addChangeListener(1): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.getUpgradeSlotsPosition(2): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.getUpgradeSlots(4): not translated, its body couldn't be read: an anonymous class.
        // FilterUpgradeBase.getTabSize(0): not translated, its class isn't translated (it's generic).
        // FilterUpgradeBase.createFilterSettings(3): not translated, it has no body.
        // FilterUpgradeBase.createFilter(2): not translated, it has no body.
    }

    /// `ButtonStates` (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java)
    pub mod button_states {
        use super::*;
        // ALLOW: not translated, its value couldn't be read: an anonymous class.
        // ALLOW_FEEDING: not translated, its value couldn't be read: an anonymous class.
        // OBJECT_TYPE: not translated, its value couldn't be read: an anonymous class.
        // IGNORE_MODE: not translated, its value couldn't be read: an anonymous class.
        // HUNGER_MODE: not translated, its value couldn't be read: an anonymous class.
        // IGNORE_EFFECT_MODE: not translated, its value couldn't be read: an anonymous class.
        #[derive(Clone, Debug)]
        pub struct ButtonStates {
        }
        impl ButtonStates {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ButtonStates.ButtonState` (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java)
    pub mod button_states_button_state {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ButtonState {
            pub modes: JMap<String, point::Point>,
        }
        impl ButtonState {
            pub fn new(modes: &JMap<String, point::Point>) -> Self {
                let mut s = Self { modes: modes.clone() };
                s.modes = modes.clone();
                s
            }
            // ButtonStates.ButtonState.getByIndex(1): not translated, .toArray on a map's keys.
            pub fn get_states_count(&self) -> i32 {
                return (self.modes.len() as i32);
            }
            // ButtonStates.ButtonState.getButtonIcon(1): not translated, it calls ButtonState.getByIndex, which isn't translated.
        }
    }

    /// `FilterButton` (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterButton.java)
    pub mod filter_button {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterButton.FilterButton(4): not translated, its class isn't translated (it's generic).
        // FilterButton.getButtonState(0): not translated, its class isn't translated (it's generic).
        // FilterButton.getCurrentState(0): not translated, its class isn't translated (it's generic).
        // FilterButton.nextState(0): not translated, its class isn't translated (it's generic).
        // FilterButton.setHidden(1): not translated, its class isn't translated (it's generic).
        // FilterButton.renderButton(3): not translated, its class isn't translated (it's generic).
        // FilterButton.mouseClicked(2): not translated, its class isn't translated (it's generic).
        // FilterButton.isMouseOver(2): not translated, its class isn't translated (it's generic).
    }

    /// `FilterHandler` (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterHandler.java)
    pub mod filter_handler {
        use super::*;
        // The class itself isn't translated: it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler). Its static parts are below.
        // FilterHandler.FilterHandler(2): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // FilterHandler.isItemValid(2): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
        // FilterHandler.getSlotLimit(1): not translated, its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)).
    }

    /// `IFilter` (com/tiviacz/travelersbackpack/inventory/upgrades/filter/IFilter.java)
    pub mod i_filter {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IFilter.getFilter(0): not translated, it has no body.
        // IFilter.updateSettings(0): not translated, it has no body.
    }

    /// `IFilterSlots` (com/tiviacz/travelersbackpack/inventory/upgrades/filter/IFilterSlots.java)
    pub mod i_filter_slots {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IFilterSlots.getFilterSlotCount(0): not translated, it has no body.
        // IFilterSlots.getSlotsInRow(0): not translated, it has no body.
        // IFilterSlots.getRows(0): not translated, its class isn't translated (it's an interface).
        // IFilterSlots.getSlotsInRow(1): not translated, its class isn't translated (it's an interface).
    }

    /// `IEnable` (com/tiviacz/travelersbackpack/inventory/upgrades/IEnable.java)
    pub mod i_enable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IEnable.isEnabled(1): not translated, its class isn't translated (it's an interface).
        // IEnable.setEnabled(1): not translated, its class isn't translated (it's an interface).
    }

    /// `IMoveSelector` (com/tiviacz/travelersbackpack/inventory/upgrades/IMoveSelector.java)
    pub mod i_move_selector {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IMoveSelector.shiftClickToBackpack(1): not translated, its class isn't translated (it's an interface).
    }

    /// `ITickableUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/ITickableUpgrade.java)
    pub mod i_tickable_upgrade {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ITickableUpgrade.tick(4): not translated, it has no body.
        // ITickableUpgrade.getTickRate(0): not translated, it has no body.
    }

    /// `IUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/IUpgrade.java)
    pub mod i_upgrade {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IUpgrade.remove(0): not translated, its class isn't translated (it's an interface).
        // IUpgrade.onUpgradeRemoved(1): not translated, its class isn't translated (it's an interface).
        // IUpgrade.onUpgradeRemoved(2): not translated, its class isn't translated (it's an interface).
        // IUpgrade.createWidget(3): not translated, it has no body.
        // IUpgrade.getUpgradeSlotsPosition(2): not translated, it has no body.
        // IUpgrade.getUpgradeSlots(4): not translated, it has no body.
        // IUpgrade.initializeContainers(2): not translated, its class isn't translated (it's an interface).
        // IUpgrade.hasTab(0): not translated, its class isn't translated (it's an interface).
        // IUpgrade.getTabSize(0): not translated, it has no body.
        // IUpgrade.isTabOpened(0): not translated, it has no body.
    }

    /// `JukeboxUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxUpgrade.java)
    pub mod jukebox_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        // JukeboxUpgrade.JukeboxUpgrade(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // JukeboxUpgrade.onUpgradeRemoved(1): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // JukeboxUpgrade.createWidget(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // JukeboxUpgrade.getUpgradeSlotsPosition(2): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // JukeboxUpgrade.getUpgradeSlots(4): not translated, its body couldn't be read: an anonymous class.
        // JukeboxUpgrade.isPlayingRecord(0): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // JukeboxUpgrade.canPlayRecord(0): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // JukeboxUpgrade.setSlotChanged(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // JukeboxUpgrade.createHandler(1): not translated, its body couldn't be read: an anonymous class.
    }

    /// `JukeboxWidget` (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java)
    pub mod jukebox_widget {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets). Its static parts are below.
        // JukeboxWidget.JukeboxWidget(3): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // JukeboxWidget.extractRenderState(4): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // JukeboxWidget.renderBg(5): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // JukeboxWidget.mouseClicked(2): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // JukeboxWidget.isMouseOverPlayButton(2): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // JukeboxWidget.isMouseOverStopButton(2): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // JukeboxWidget.getFromDisk(1): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // JukeboxWidget.playDiscToPlayer(2): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // JukeboxWidget.stopDisc(1): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
    }

    /// `JukeboxWidget.MovingSound` (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java)
    pub mod jukebox_widget_moving_sound {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractTickableSoundInstance`. Its static parts are below.
        // JukeboxWidget.MovingSound.MovingSound(2): not translated, its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
        // JukeboxWidget.MovingSound.tick(0): not translated, its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
        // JukeboxWidget.MovingSound.shouldStopPlaying(1): not translated, its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`).
    }

    /// `LanternUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternUpgrade.java)
    pub mod lantern_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        pub const LANTERN: &str = "Lantern";
        // LanternUpgrade.LanternUpgrade(2): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // LanternUpgrade.createWidget(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // LanternUpgrade.setEnabled(1): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // LanternUpgrade.writeToRenderData(2): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // LanternUpgrade.onUpgradeRemoved(1): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // LanternUpgrade.hasTab(0): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // LanternUpgrade.getUpgradeSlotsPosition(2): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // LanternUpgrade.getUpgradeSlots(4): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
    }

    /// `LanternWidget` (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternWidget.java)
    pub mod lantern_widget {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets). Its static parts are below.
        // LanternWidget.LanternWidget(5): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // LanternWidget.getAdditionalTooltips(1): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
    }

    /// `MagnetFilterSettings` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java)
    pub mod magnet_filter_settings {
        use super::*;
        // The class itself isn't translated: it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        pub const ALLOW_MODE: i32 = 0;
        pub const OBJECT_CATEGORY: i32 = 1;
        pub const IGNORE_MODE: i32 = 2;
        pub const ALLOW: i32 = 0;
        pub const BLOCK: i32 = 1;
        pub const ITEM: i32 = 0;
        pub const MOD_ID: i32 = 1;
        pub const TAG_ID: i32 = 2;
        pub const MATCH_COMPONENTS: i32 = 0;
        pub const IGNORE_COMPONENTS: i32 = 1;
        // MagnetFilterSettings.MagnetFilterSettings(4): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // MagnetFilterSettings.matchesFilter(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // MagnetFilterSettings.isTagFilter(0): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // MagnetFilterSettings.compare(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // MagnetFilterSettings.compareItemStack(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
    }

    /// `MagnetUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetUpgrade.java)
    pub mod magnet_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        // MagnetUpgrade.MagnetUpgrade(4): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // MagnetUpgrade.createFilterSettings(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // MagnetUpgrade.createWidget(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // MagnetUpgrade.getTickRate(0): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // MagnetUpgrade.getPullRange(0): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // MagnetUpgrade.tick(4): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // MagnetUpgrade.teleportNearbyItems(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // MagnetUpgrade.createFilter(2): not translated, its body couldn't be read: an anonymous class.
    }

    /// `MagnetWidget` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetWidget.java)
    pub mod magnet_widget {
        use super::*;
        // The class itself isn't translated: it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets). Its static parts are below.
        // WHITELIST_TOOLTIPS: a game value, so it's in game_side.rs.
        // OBJECT_TOOLTIPS: a game value, so it's in game_side.rs.
        // IGNORE_MODE_TOOLTIPS: a game value, so it's in game_side.rs.
        // MagnetWidget.MagnetWidget(3): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // MagnetWidget.renderTooltip(3): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
    }

    /// `AutoPickupFilterSettings` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java)
    pub mod auto_pickup_filter_settings {
        use super::*;
        // The class itself isn't translated: it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        pub const ALLOW_MODE: i32 = 0;
        pub const OBJECT_CATEGORY: i32 = 1;
        pub const IGNORE_MODE: i32 = 2;
        pub const ALLOW: i32 = 0;
        pub const BLOCK: i32 = 1;
        pub const MATCH_CONTENTS: i32 = 2;
        pub const ITEM: i32 = 0;
        pub const MOD_ID: i32 = 1;
        pub const TAG_ID: i32 = 2;
        pub const MATCH_COMPONENTS: i32 = 0;
        pub const IGNORE_COMPONENTS: i32 = 1;
        // AutoPickupFilterSettings.AutoPickupFilterSettings(4): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // AutoPickupFilterSettings.matchesFilter(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // AutoPickupFilterSettings.isTagFilter(0): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // AutoPickupFilterSettings.compare(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // AutoPickupFilterSettings.compareItemStack(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
    }

    /// `AutoPickupUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupUpgrade.java)
    pub mod auto_pickup_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        pub const PICKUP_RANGE: f64 = 1.0f64;
        // AutoPickupUpgrade.AutoPickupUpgrade(4): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // AutoPickupUpgrade.canPickup(1): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // AutoPickupUpgrade.createFilterSettings(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // AutoPickupUpgrade.createWidget(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // AutoPickupUpgrade.getTickRate(0): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // AutoPickupUpgrade.tick(4): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // AutoPickupUpgrade.tryPickup(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // AutoPickupUpgrade.createFilter(2): not translated, its body couldn't be read: an anonymous class.
    }

    /// `AutoPickupWidget` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupWidget.java)
    pub mod auto_pickup_widget {
        use super::*;
        // The class itself isn't translated: it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets). Its static parts are below.
        // WHITELIST_TOOLTIPS: a game value, so it's in game_side.rs.
        // OBJECT_TOOLTIPS: a game value, so it's in game_side.rs.
        // IGNORE_MODE_TOOLTIPS: a game value, so it's in game_side.rs.
        // AutoPickupWidget.AutoPickupWidget(3): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // AutoPickupWidget.renderBg(5): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // AutoPickupWidget.renderTooltip(3): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
    }

    /// `Point` (com/tiviacz/travelersbackpack/inventory/upgrades/Point.java)
    pub mod point {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Point {
            pub x: i32,
            pub y: i32,
        }
        impl Point {
            pub fn new(x: i32, y: i32) -> Self {
                let mut s = Self { x: x, y: y };
                s.x = x;
                s.y = y;
                s
            }
            pub fn x(&self) -> i32 {
                return self.x;
            }
            pub fn y(&self) -> i32 {
                return self.y;
            }
        }
    }

    /// `RefillUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java)
    pub mod refill_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        pub const REFILL_RANGE: f64 = 3.0f64;
        // RefillUpgrade.RefillUpgrade(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // RefillUpgrade.getFilterSlotCount(0): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // RefillUpgrade.getSlotsInRow(0): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // RefillUpgrade.createWidget(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // RefillUpgrade.getUpgradeSlotsPosition(2): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // RefillUpgrade.getUpgradeSlots(4): not translated, its body couldn't be read: an anonymous class.
        // RefillUpgrade.tick(4): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // RefillUpgrade.tryRefillItems(1): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // RefillUpgrade.refill(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // RefillUpgrade.countAndSupply(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // RefillUpgrade.getMissingCount(2): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // RefillUpgrade.getTickRate(0): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // RefillUpgrade.createFilter(2): not translated, its body couldn't be read: an anonymous class.
    }

    /// `ResultArrowElement` (com/tiviacz/travelersbackpack/inventory/upgrades/ResultArrowElement.java)
    pub mod result_arrow_element {
        use super::*;
        // The class itself isn't translated: it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`). Its static parts are below.
        // ResultArrowElement.ResultArrowElement(3): not translated, its class isn't translated (it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`)).
        // ResultArrowElement.renderBg(5): not translated, its class isn't translated (it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`)).
        // ResultArrowElement.renderTooltip(3): not translated, its class isn't translated (it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`)).
        // ResultArrowElement.mouseClicked(1): not translated, its class isn't translated (it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`)).
        // ResultArrowElement.isMouseOverShiftClickButton(2): not translated, its class isn't translated (it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`)).
    }

    /// `AbstractSmeltingUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java)
    pub mod abstract_smelting_upgrade {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const SLOT_INPUT: i32 = 0;
        pub const SLOT_FUEL: i32 = 1;
        pub const SLOT_RESULT: i32 = 2;
        // AbstractSmeltingUpgrade.AbstractSmeltingUpgrade(5): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.createWidget(3): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.onUpgradeRemoved(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getUpgradeSlotsPosition(2): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getUpgradeSlots(4): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.setEnabled(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.syncClient(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.tick(4): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getTickRate(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.isFuel(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.isBurning(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.isCooking(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.hasFuel(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.tickSmelting(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.checkCooking(2): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.startCooking(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.startBurning(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.canBurn(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.finishCooking(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.shrinkFuelSlot(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.finishBurning(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.setStack(2): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getStack(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getSpeedMultiplier(2): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getTotalCookTime(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getBurnDuration(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getLootContextBlockState(0): not translated, it has no body.
        // AbstractSmeltingUpgrade.getLootContext(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getBurnTotalTime(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getBurnFinishTime(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getCookingTotalTime(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getCookingFinishTime(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.getSpeedMultiplier(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.setBurnTotalTime(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.setBurnFinishTime(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.setCookingTotalTime(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.setCookingFinishTime(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.setSpeedMultiplier(1): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.stopCooking(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.stopBurning(0): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.setSlotChanged(3): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingUpgrade.setStackData(2): not translated, it's generic.
        // AbstractSmeltingUpgrade.createHandler(1): not translated, its body couldn't be read: an anonymous class.
    }

    /// `AbstractSmeltingWidget` (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingWidget.java)
    pub mod abstract_smelting_widget {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractSmeltingWidget.AbstractSmeltingWidget(4): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingWidget.renderBg(5): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingWidget.extractRenderState(4): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingWidget.renderTooltip(3): not translated, its class isn't translated (it's generic).
        // AbstractSmeltingWidget.mouseClicked(2): not translated, its class isn't translated (it's generic).
    }

    /// `BlastFurnaceUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/BlastFurnaceUpgrade.java)
    pub mod blast_furnace_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `AbstractSmeltingUpgrade`. Its static parts are below.
        // BlastFurnaceUpgrade.BlastFurnaceUpgrade(3): not translated, its class isn't translated (it extends `AbstractSmeltingUpgrade`).
        // BlastFurnaceUpgrade.getLootContextBlockState(0): not translated, its class isn't translated (it extends `AbstractSmeltingUpgrade`).
    }

    /// `FurnaceUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/FurnaceUpgrade.java)
    pub mod furnace_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `AbstractSmeltingUpgrade`. Its static parts are below.
        // FurnaceUpgrade.FurnaceUpgrade(3): not translated, its class isn't translated (it extends `AbstractSmeltingUpgrade`).
        // FurnaceUpgrade.getLootContextBlockState(0): not translated, its class isn't translated (it extends `AbstractSmeltingUpgrade`).
    }

    /// `SmokerUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/SmokerUpgrade.java)
    pub mod smoker_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `AbstractSmeltingUpgrade`. Its static parts are below.
        // SmokerUpgrade.SmokerUpgrade(3): not translated, its class isn't translated (it extends `AbstractSmeltingUpgrade`).
        // SmokerUpgrade.getLootContextBlockState(0): not translated, its class isn't translated (it extends `AbstractSmeltingUpgrade`).
    }

    /// `FluidStorageItemWrapper` (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/FluidStorageItemWrapper.java)
    pub mod fluid_storage_item_wrapper {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `SingleSlotStorage`. Its static parts are below.
        // FluidStorageItemWrapper.FluidStorageItemWrapper(2): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // FluidStorageItemWrapper.insert(3): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // FluidStorageItemWrapper.extract(3): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // FluidStorageItemWrapper.getTanksUpgrade(1): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // FluidStorageItemWrapper.isResourceBlank(0): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // FluidStorageItemWrapper.getResource(0): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // FluidStorageItemWrapper.getAmount(0): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // FluidStorageItemWrapper.getCapacity(0): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
        // FluidStorageItemWrapper.getTank(1): not translated, its class isn't translated (it implements Fabric's `SingleSlotStorage`).
    }

    /// `TankActions` (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankActions.java)
    pub mod tank_actions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TankActions {
        }
        impl TankActions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TankActions.fillTank(2): not translated, it uses Minecraft's `ServerPlayer`.
        // TankActions.tryEmptyPotion(3): not translated, it uses Minecraft's `ItemStack`.
        // TankActions.tryFillPotion(4): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `TanksUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java)
    pub mod tanks_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        // TanksUpgrade.TanksUpgrade(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.getLeftTank(0): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.getRightTank(0): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.getFluidSlotsHandler(0): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.setTanksCapacity(0): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.setFluids(1): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.syncClients(1): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.createFluidHandler(1): not translated, its body couldn't be read: an anonymous class.
        // TanksUpgrade.writeToRenderData(1): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.remove(0): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.createWidget(3): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.getUpgradeSlotsPosition(2): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.getUpgradeSlots(4): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.onUpgradeRemoved(2): not translated, its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // TanksUpgrade.createTemporaryHandler(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `TankWidget` (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankWidget.java)
    pub mod tank_widget {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets). Its static parts are below.
        // TankWidget.TankWidget(3): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // TankWidget.renderTooltip(3): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // TankWidget.renderAboveBg(6): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // TankWidget.getTankHeight(1): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // TankWidget.mouseClicked(2): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // TankWidget.isMouseOver(2): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // TankWidget.isValid(1): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // TankWidget.renderTank(8): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // TankWidget.getTankTooltip(1): not translated, it uses Minecraft's `Component`.
        // TankWidget.inTank(3): not translated, its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
    }

    /// `UpgradeBase` (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java)
    pub mod upgrade_base {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // UpgradeBase.UpgradeBase(3): not translated, its class isn't translated (it's generic).
        // UpgradeBase.getUpgradeManager(0): not translated, its class isn't translated (it's generic).
        // UpgradeBase.getDataHolderSlot(0): not translated, its class isn't translated (it's generic).
        // UpgradeBase.isTabOpened(0): not translated, its class isn't translated (it's generic).
        // UpgradeBase.getDataHolderStack(0): not translated, its class isn't translated (it's generic).
        // UpgradeBase.getTabSize(0): not translated, its class isn't translated (it's generic).
        // UpgradeBase.updateDataHolderUnchecked(1): not translated, its class isn't translated (it's generic).
        // UpgradeBase.updateDataHolderUnchecked(2): not translated, it's generic.
        // UpgradeBase.setCooldown(1): not translated, its class isn't translated (it's generic).
        // UpgradeBase.getCooldown(0): not translated, its class isn't translated (it's generic).
        // UpgradeBase.hasCooldown(0): not translated, its class isn't translated (it's generic).
    }

    /// `VoidFilterSettings` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java)
    pub mod void_filter_settings {
        use super::*;
        // The class itself isn't translated: it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        pub const ALLOW_MODE: i32 = 0;
        pub const OBJECT_CATEGORY: i32 = 1;
        pub const IGNORE_MODE: i32 = 2;
        pub const ALLOW: i32 = 0;
        pub const BLOCK: i32 = 1;
        pub const MATCH_CONTENTS: i32 = 2;
        pub const ITEM: i32 = 0;
        pub const MOD_ID: i32 = 1;
        pub const TAG_ID: i32 = 2;
        pub const MATCH_COMPONENTS: i32 = 0;
        pub const IGNORE_COMPONENTS: i32 = 1;
        // VoidFilterSettings.VoidFilterSettings(4): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // VoidFilterSettings.matchesFilter(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // VoidFilterSettings.isTagFilter(0): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // VoidFilterSettings.compare(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // VoidFilterSettings.compareItemStack(2): not translated, its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
    }

    /// `VoidUpgrade` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidUpgrade.java)
    pub mod void_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades). Its static parts are below.
        // VoidUpgrade.VoidUpgrade(4): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // VoidUpgrade.getFilter(0): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // VoidUpgrade.createFilterSettings(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // VoidUpgrade.canVoid(1): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // VoidUpgrade.createWidget(3): not translated, its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)).
        // VoidUpgrade.createFilter(2): not translated, its body couldn't be read: an anonymous class.
    }

    /// `VoidWidget` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java)
    pub mod void_widget {
        use super::*;
        // The class itself isn't translated: it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets). Its static parts are below.
        pub static TRASH_UPGRADE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "textures/item/void_upgrade.png"));
        pub const RED_HIGHLIGHT_COLOR: i32 = 2143884822;
        pub const RED_ARROW_COLOR: i32 = -1079377920;
        pub const HIGHLIGHT_TANK_COLOR: i32 = -2130706433;
        // WHITELIST_TOOLTIPS: a game value, so it's in game_side.rs.
        // OBJECT_TOOLTIPS: a game value, so it's in game_side.rs.
        // IGNORE_MODE_TOOLTIPS: a game value, so it's in game_side.rs.
        // VoidWidget.VoidWidget(3): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.renderBg(5): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.renderUnderTooltip(4): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.renderTooltip(3): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.renderOnTop(4): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.isMouseOver(2): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.isWithinTrashBinSlot(2): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.mouseDragged(3): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.mouseClicked(2): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.isHoveringWithTrashBin(0): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.select(1): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.selectTank(5): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.canSelectSlot(1): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.selectSlots(2): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.drawRedTankHighlight(6): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.drawRedHighlight(2): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.renderRedDownArrow(3): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.renderTrashBinAnimation(4): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.renderTrashBin(5): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
        // VoidWidget.tickAnimation(0): not translated, its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)).
    }

    /// `BackpackTankItem` (com/tiviacz/travelersbackpack/item/BackpackTankItem.java)
    pub mod backpack_tank_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // BackpackTankItem.BackpackTankItem(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // BackpackTankItem.isEnabled(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `HoseItem` (com/tiviacz/travelersbackpack/item/HoseItem.java)
    pub mod hose_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        pub const NO_ASSIGN: i32 = 0;
        pub const SUCK_MODE: i32 = 1;
        pub const SPILL_MODE: i32 = 2;
        pub const DRINK_MODE: i32 = 3;
        // HoseItem.HoseItem(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.getUseAnimation(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.getUseDuration(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.onUseTick(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.shouldEmitDrinkingSounds(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.consumeTicks(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.consumeSeconds(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.emitDrinkingSound(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.use(3): not translated, its body couldn't be read: a labelled statement.
        // HoseItem.drink(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.spillPotion(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.emptyContents(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.playEmptySound(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.triggerAdvancement(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.finishUsingItem(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.getHoseMode(1): not translated, it uses Minecraft's `ItemStack`.
        // HoseItem.getHoseTank(1): not translated, it uses Minecraft's `ItemStack`.
        // HoseItem.getSelectedFluidTank(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // HoseItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `SleepingBagItem` (com/tiviacz/travelersbackpack/item/SleepingBagItem.java)
    pub mod sleeping_bag_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // SleepingBagItem.SleepingBagItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SleepingBagItem.isEnabled(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // SleepingBagItem.getDefaultColor(0): not translated, it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)).
        // SleepingBagItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `TravelersBackpackItem` (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java)
    pub mod travelers_backpack_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // TravelersBackpackItem.TravelersBackpackItem(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // TravelersBackpackItem.use(3): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // TravelersBackpackItem.useOn(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // TravelersBackpackItem.place(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // TravelersBackpackItem.isCreative(1): not translated, it uses Minecraft's `Player`.
        // TravelersBackpackItem.overrideStackedOnOther(4): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // TravelersBackpackItem.overrideOtherStackedOnMe(6): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // TravelersBackpackItem.add(4): not translated, it uses Minecraft's `Player`.
        // TravelersBackpackItem.playInsertSound(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // TravelersBackpackItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // TravelersBackpackItem.addAttributeModifierTooltip(3): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // TravelersBackpackItem.getTooltipImage(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // TravelersBackpackItem.canFitInsideContainerItems(0): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `BlastFurnaceUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/BlastFurnaceUpgradeItem.java)
    pub mod blast_furnace_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // BlastFurnaceUpgradeItem.BlastFurnaceUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // BlastFurnaceUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // BlastFurnaceUpgradeItem.isTickingUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // BlastFurnaceUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // BlastFurnaceUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `CraftingUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/CraftingUpgradeItem.java)
    pub mod crafting_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // CraftingUpgradeItem.CraftingUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // CraftingUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // CraftingUpgradeItem.requiresEquippedBackpack(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // CraftingUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // CraftingUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `FeedingUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/FeedingUpgradeItem.java)
    pub mod feeding_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // FeedingUpgradeItem.FeedingUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // FeedingUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // FeedingUpgradeItem.isTickingUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // FeedingUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // FeedingUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `FurnaceUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/FurnaceUpgradeItem.java)
    pub mod furnace_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // FurnaceUpgradeItem.FurnaceUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // FurnaceUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // FurnaceUpgradeItem.isTickingUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // FurnaceUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // FurnaceUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `JukeboxUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/JukeboxUpgradeItem.java)
    pub mod jukebox_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // JukeboxUpgradeItem.JukeboxUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // JukeboxUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // JukeboxUpgradeItem.hasBlockFunctionality(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // JukeboxUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // JukeboxUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `LanternUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/LanternUpgradeItem.java)
    pub mod lantern_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // LanternUpgradeItem.LanternUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // LanternUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // LanternUpgradeItem.appendHoverText(5): not translated, its class isn't translated (it extends `UpgradeItem`).
        // LanternUpgradeItem.addStatusTooltip(1): needs the game, so it's in game_side.rs.
        // LanternUpgradeItem.hasBlockFunctionality(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // LanternUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // LanternUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `MagnetUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/MagnetUpgradeItem.java)
    pub mod magnet_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // MagnetUpgradeItem.MagnetUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // MagnetUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // MagnetUpgradeItem.appendHoverText(5): not translated, its class isn't translated (it extends `UpgradeItem`).
        // MagnetUpgradeItem.isTickingUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // MagnetUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // MagnetUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `PickupUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/PickupUpgradeItem.java)
    pub mod pickup_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // PickupUpgradeItem.PickupUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // PickupUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // PickupUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // PickupUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `RefillUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/RefillUpgradeItem.java)
    pub mod refill_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // RefillUpgradeItem.RefillUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // RefillUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // RefillUpgradeItem.isTickingUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // RefillUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // RefillUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `SmokerUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/SmokerUpgradeItem.java)
    pub mod smoker_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // SmokerUpgradeItem.SmokerUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // SmokerUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // SmokerUpgradeItem.isTickingUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // SmokerUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // SmokerUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `TanksUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java)
    pub mod tanks_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // TanksUpgradeItem.TanksUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // TanksUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // TanksUpgradeItem.canBePutInBackpack(2): not translated, it uses Minecraft's `ItemStack`.
        // TanksUpgradeItem.getLeftFluidStack(1): not translated, it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`).
        // TanksUpgradeItem.getRightFluidStack(1): not translated, it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`).
        // TanksUpgradeItem.writeToRenderData(0): not translated, it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`).
        // TanksUpgradeItem.writeToRenderData(1): not translated, it uses Minecraft's `CompoundTag`.
        // TanksUpgradeItem.appendHoverText(5): not translated, its class isn't translated (it extends `UpgradeItem`).
        // TanksUpgradeItem.requiresEquippedBackpack(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // TanksUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // TanksUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `TierUpgrade` (com/tiviacz/travelersbackpack/item/upgrade/TierUpgrade.java)
    pub mod tier_upgrade {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // TierUpgrade.TierUpgrade(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // TierUpgrade.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `TierUpgrade.Upgrade` (com/tiviacz/travelersbackpack/item/upgrade/TierUpgrade.java)
    pub mod tier_upgrade_upgrade {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Upgrade {
            BlankUpgrade,
            IronTierUpgrade,
            GoldTierUpgrade,
            DiamondTierUpgrade,
            NetheriteTierUpgrade,
        }
        impl Upgrade {
            pub const VALUES: [Upgrade; 5] = [Upgrade::BlankUpgrade, Upgrade::IronTierUpgrade, Upgrade::GoldTierUpgrade, Upgrade::DiamondTierUpgrade, Upgrade::NetheriteTierUpgrade, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Upgrade::BlankUpgrade => "BLANK_UPGRADE",
                    Upgrade::IronTierUpgrade => "IRON_TIER_UPGRADE",
                    Upgrade::GoldTierUpgrade => "GOLD_TIER_UPGRADE",
                    Upgrade::DiamondTierUpgrade => "DIAMOND_TIER_UPGRADE",
                    Upgrade::NetheriteTierUpgrade => "NETHERITE_TIER_UPGRADE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "BLANK_UPGRADE" => Upgrade::BlankUpgrade,
                    "IRON_TIER_UPGRADE" => Upgrade::IronTierUpgrade,
                    "GOLD_TIER_UPGRADE" => Upgrade::GoldTierUpgrade,
                    "DIAMOND_TIER_UPGRADE" => Upgrade::DiamondTierUpgrade,
                    "NETHERITE_TIER_UPGRADE" => Upgrade::NetheriteTierUpgrade,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `UpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/UpgradeItem.java)
    pub mod upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // UpgradeItem.UpgradeItem(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // UpgradeItem.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // UpgradeItem.isTickingUpgrade(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // UpgradeItem.requiresEquippedBackpack(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // UpgradeItem.hasBlockFunctionality(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // UpgradeItem.getUpgradeClass(0): not translated, it has no body.
        // UpgradeItem.getUpgrade(0): not translated, it has no body.
    }

    /// `VoidUpgradeItem` (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java)
    pub mod void_upgrade_item {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeItem`. Its static parts are below.
        // VoidUpgradeItem.VoidUpgradeItem(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // VoidUpgradeItem.isEnabled(1): not translated, its class isn't translated (it extends `UpgradeItem`).
        // VoidUpgradeItem.requiresEquippedBackpack(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // VoidUpgradeItem.getUpgradeClass(0): not translated, its class isn't translated (it extends `UpgradeItem`).
        // VoidUpgradeItem.getUpgrade(0): not translated, its class isn't translated (it extends `UpgradeItem`).
    }

    /// `ClientboundSendMessagePacket` (com/tiviacz/travelersbackpack/network/ClientboundSendMessagePacket.java)
    pub mod clientbound_send_message_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "send_message"));
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ClientboundSendMessagePacket.ClientboundSendMessagePacket(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSendMessagePacket.handle(2): not translated, it uses ClientboundSendMessagePacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSendMessagePacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSendMessagePacket.drop(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSendMessagePacket.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ClientboundSyncAttachmentPacket` (com/tiviacz/travelersbackpack/network/ClientboundSyncAttachmentPacket.java)
    pub mod clientbound_sync_attachment_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "sync_attachment"));
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ClientboundSyncAttachmentPacket.ClientboundSyncAttachmentPacket(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncAttachmentPacket.ClientboundSyncAttachmentPacket(3): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncAttachmentPacket.handle(2): not translated, it uses ClientboundSyncAttachmentPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncAttachmentPacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncAttachmentPacket.entityID(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncAttachmentPacket.backpack(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncAttachmentPacket.removeData(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ClientboundSyncComponentsPacket` (com/tiviacz/travelersbackpack/network/ClientboundSyncComponentsPacket.java)
    pub mod clientbound_sync_components_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "sync_components"));
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ClientboundSyncComponentsPacket.ClientboundSyncComponentsPacket(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncComponentsPacket.handle(2): not translated, it uses ClientboundSyncComponentsPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncComponentsPacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncComponentsPacket.entityID(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncComponentsPacket.map(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ClientboundSyncItemStackPacket` (com/tiviacz/travelersbackpack/network/ClientboundSyncItemStackPacket.java)
    pub mod clientbound_sync_item_stack_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "sync_itemstack"));
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ClientboundSyncItemStackPacket.ClientboundSyncItemStackPacket(4): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncItemStackPacket.handle(2): not translated, it uses ClientboundSyncItemStackPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncItemStackPacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncItemStackPacket.entityId(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncItemStackPacket.slot(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncItemStackPacket.itemInstance(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundSyncItemStackPacket.map(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ClientboundUpdateRecipePacket` (com/tiviacz/travelersbackpack/network/ClientboundUpdateRecipePacket.java)
    pub mod clientbound_update_recipe_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "update_recipe"));
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ClientboundUpdateRecipePacket.ClientboundUpdateRecipePacket(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundUpdateRecipePacket.handle(2): not translated, it uses ClientboundUpdateRecipePacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundUpdateRecipePacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientboundUpdateRecipePacket.output(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ServerboundActionTagPacket` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java)
    pub mod serverbound_action_tag_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        pub const UPGRADE_TAB: i32 = 0;
        pub const OPEN_SCREEN: i32 = 1;
        pub const OPEN_BACKPACK: i32 = 2;
        pub const SORTER: i32 = 3;
        pub const SLEEPING_BAG: i32 = 4;
        pub const FILL_TANK: i32 = 5;
        pub const SWAP_TOOL: i32 = 6;
        pub const TOGGLE_BUTTONS_VISIBILITY: i32 = 7;
        pub const SHOW_TOOL_SLOTS: i32 = 8;
        pub const REMOVE_UPGRADE: i32 = 9;
        pub const OPEN_SETTINGS: i32 = 10;
        pub const SWITCH_HOSE_MODE: i32 = 11;
        pub const SWITCH_HOSE_TANK: i32 = 12;
        pub const TOGGLE_VISIBILITY: i32 = 13;
        pub const ABILITY_SLIDER: i32 = 14;
        pub const EQUIP_BACKPACK: i32 = 15;
        pub const SET_STACK: i32 = 16;
        // ServerboundActionTagPacket.ServerboundActionTagPacket(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundActionTagPacket.handle(2): not translated, it uses ServerboundActionTagPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundActionTagPacket.create(2): not translated, it uses `Object`.
        // ServerboundActionTagPacket.createPacketTag(2): not translated, it uses Minecraft's `CompoundTag`.
        // ServerboundActionTagPacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundActionTagPacket.actionTag(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ServerboundFilterSettingsPacket` (com/tiviacz/travelersbackpack/network/ServerboundFilterSettingsPacket.java)
    pub mod serverbound_filter_settings_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ServerboundFilterSettingsPacket.ServerboundFilterSettingsPacket(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundFilterSettingsPacket.handle(2): not translated, it uses ServerboundFilterSettingsPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundFilterSettingsPacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundFilterSettingsPacket.slot(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundFilterSettingsPacket.settings(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ServerboundFilterTagsPacket` (com/tiviacz/travelersbackpack/network/ServerboundFilterTagsPacket.java)
    pub mod serverbound_filter_tags_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ServerboundFilterTagsPacket.ServerboundFilterTagsPacket(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundFilterTagsPacket.handle(2): not translated, it uses ServerboundFilterTagsPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundFilterTagsPacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundFilterTagsPacket.slot(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundFilterTagsPacket.tags(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ServerboundRetrieveBackpackPacket` (com/tiviacz/travelersbackpack/network/ServerboundRetrieveBackpackPacket.java)
    pub mod serverbound_retrieve_backpack_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ServerboundRetrieveBackpackPacket.ServerboundRetrieveBackpackPacket(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundRetrieveBackpackPacket.handle(2): not translated, it uses ServerboundRetrieveBackpackPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundRetrieveBackpackPacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundRetrieveBackpackPacket.backpackHolder(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ServerboundSlotPacket` (com/tiviacz/travelersbackpack/network/ServerboundSlotPacket.java)
    pub mod serverbound_slot_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        pub const UNSORTABLES: i32 = 0;
        pub const MEMORY: i32 = 1;
        // ServerboundSlotPacket.ServerboundSlotPacket(3): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundSlotPacket.handle(2): not translated, it uses ServerboundSlotPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundSlotPacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundSlotPacket.selectType(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundSlotPacket.unsortables(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ServerboundSlotPacket.memorizedSlots(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `SupporterBadgePacket` (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java)
    pub mod supporter_badge_packet {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupporterBadgePacket {
        }
        impl SupporterBadgePacket {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `SupporterBadgePacket.Clientbound` (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java)
    pub mod supporter_badge_packet_clientbound {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // SupporterBadgePacket.Clientbound.Clientbound(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SupporterBadgePacket.Clientbound.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SupporterBadgePacket.Clientbound.handle(2): not translated, it uses Clientbound, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SupporterBadgePacket.Clientbound.isEnabledForPlayer(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SupporterBadgePacket.Clientbound.playerName(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `SupporterBadgePacket.Serverbound` (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java)
    pub mod supporter_badge_packet_serverbound {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // SupporterBadgePacket.Serverbound.Serverbound(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SupporterBadgePacket.Serverbound.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SupporterBadgePacket.Serverbound.handle(2): not translated, it uses Serverbound, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SupporterBadgePacket.Serverbound.isEnabledForPlayer(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `TravelersBackpack` (com/tiviacz/travelersbackpack/TravelersBackpack.java)
    pub mod travelers_backpack {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        pub const MODID: &str = "travelersbackpack";
        // LOGGER: the logger. Log calls become comments.
        // currentServer: not translated, it uses Minecraft's `MinecraftServer`.
        pub static ACCESSORIES_LOADED: JCell<bool> = JCell::new(false);
        pub static TRINKETS_LOADED: JCell<bool> = JCell::new(false);
        pub static CRAFTING_TWEAKS_LOADED: JCell<bool> = JCell::new(false);
        pub static TOUGHASNAILS_LOADED: JCell<bool> = JCell::new(false);
        pub static COMFORTS_LOADED: JCell<bool> = JCell::new(false);
        pub static UNIVERSAL_GRAVES_LOADED: JCell<bool> = JCell::new(false);
        pub static PNEUMONO_GRAVESTONES_LOADED: JCell<bool> = JCell::new(false);
        pub static VINURL_LOADED: JCell<bool> = JCell::new(false);
        pub static POLYMORPH_LOADED: JCell<bool> = JCell::new(false);
        pub static TRASH_SLOT_LOADED: JCell<bool> = JCell::new(false);
        pub static LAMB_DYNAMIC_LIGHTS_LOADED: JCell<bool> = JCell::new(false);
        pub static MOUSE_TWEAKS_LOADED: JCell<bool> = JCell::new(false);
        // TravelersBackpack.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // TravelersBackpack.registerDeathHandler(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // TravelersBackpack.getCurrentServer(0): not translated, it uses Minecraft's `MinecraftServer`.
        // TravelersBackpack.enableIntegration(0): not translated, it calls TravelersBackpack.enableTrinkets, which isn't translated.
        // TravelersBackpack.enableAccessories(0): not translated, it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)).
        // TravelersBackpack.enableTrinkets(0): not translated, it calls TravelersBackpack.enableAccessories, which isn't translated.
        pub fn is_any_grave_mod_installed() -> bool {
            return travelers_backpack::UNIVERSAL_GRAVES_LOADED.get() || travelers_backpack::PNEUMONO_GRAVESTONES_LOADED.get();
        }
    }

    /// `TravelersBackpackClient` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java)
    pub mod travelers_backpack_client {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        pub static ENTITY_MODEL_FEATURES_LOADED: JCell<bool> = JCell::new(false);
        pub static STAR_MODEL: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/supporter_star"));
        // STAR_MODEL_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_BASE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_base"));
        // BACKPACK_BASE_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_BASE_DYED: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_base_dyed"));
        // BACKPACK_BASE_DYED_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_EXTRAS: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_extras"));
        // BACKPACK_EXTRAS_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_FOX_NOSE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_fox_nose"));
        // BACKPACK_FOX_NOSE_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_OCELOT_NOSE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_ocelot_nose"));
        // BACKPACK_OCELOT_NOSE_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_PIG_NOSE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_pig_nose"));
        // BACKPACK_PIG_NOSE_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_SLEEPING_BAG: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_sleeping_bag"));
        // BACKPACK_SLEEPING_BAG_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_SLEEPING_BAG_EXTRAS: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_sleeping_bag_extras"));
        // BACKPACK_SLEEPING_BAG_EXTRAS_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_TANKS: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_tanks"));
        // BACKPACK_TANKS_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_VILLAGER_NOSE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_villager_nose"));
        // BACKPACK_VILLAGER_NOSE_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_WARDEN_HORNS: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_warden_horns"));
        // BACKPACK_WARDEN_HORNS_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        pub static BACKPACK_WOLF_NOSE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("travelersbackpack", "block/backpack_wolf_nose"));
        // BACKPACK_WOLF_NOSE_KEY: not translated, it uses Fabric's `ExtraModelKey`.
        // POTION_MODEL: not translated, it uses Unbaked, which isn't translated (it implements `Unbaked`).
        // TravelersBackpackClient.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // TravelersBackpackClient.registerCustomModels(0): not translated, a lambda.
        // TravelersBackpackClient.registerBlockColorProvider(0): not translated, it creates a BackpackBlockTintSource, which isn't translated (it implements Minecraft's `BlockTintSource`).
        // TravelersBackpackClient.registerItemColorProvider(0): not translated, it uses Minecraft's `ItemTintSources`.ID_MAPPER.
        // TravelersBackpackClient.registerBackpackItemEntityRenderer(0): not translated, it uses ModItems.BACKPACK_ITEM_ENTITY, which isn't translated (it uses Minecraft's `EntityType`).
        // TravelersBackpackClient.registerSpecialRenderers(0): not translated, it calls HoseSpecialRenderer.register, which isn't translated.
        // TravelersBackpackClient.registerFeatureRenderers(0): not translated, it uses Fabric's `LivingEntityRenderLayerRegistrationCallback`.EVENT.
        // TravelersBackpackClient.registerHudOverlay(0): not translated, it uses Fabric's `VanillaHudElements`.HOTBAR.
        // TravelersBackpackClient.setupFluidRendering(0): not translated, it uses ModFluids.POTION_STILL, which isn't translated (it uses Minecraft's `FlowingFluid`).
        // TravelersBackpackClient.registerTooltipComponent(0): not translated, it uses Fabric's `ClientTooltipComponentCallback`.EVENT.
        // TravelersBackpackClient.registerClientCommands(0): not translated, it uses Fabric's `ClientCommandRegistrationCallback`.EVENT.
    }

    /// `TravelersBackpackClient.BackpackTintSources` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java)
    pub mod travelers_backpack_client_backpack_tint_sources {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BackpackTintSources {
        }
        impl BackpackTintSources {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `TravelersBackpackClient.BackpackTintSources.BackpackBlockTintSource` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java)
    pub mod travelers_backpack_client_backpack_tint_sources_backpack_block_tint_source {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockTintSource`. Its static parts are below.
        // TravelersBackpackClient.BackpackTintSources.BackpackBlockTintSource.color(1): not translated, its class isn't translated (it implements Minecraft's `BlockTintSource`).
        // TravelersBackpackClient.BackpackTintSources.BackpackBlockTintSource.colorInWorld(3): not translated, its class isn't translated (it implements Minecraft's `BlockTintSource`).
    }

    /// `TravelersBackpackClient.BackpackTintSources.BackpackItemTintSource` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java)
    pub mod travelers_backpack_client_backpack_tint_sources_backpack_item_tint_source {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ItemTintSource`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TravelersBackpackClient.BackpackTintSources.BackpackItemTintSource.BackpackItemTintSource(1): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
        // TravelersBackpackClient.BackpackTintSources.BackpackItemTintSource.calculate(3): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
        // TravelersBackpackClient.BackpackTintSources.BackpackItemTintSource.type(0): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
        // TravelersBackpackClient.BackpackTintSources.BackpackItemTintSource.defaultColor(0): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
    }

    /// `TravelersBackpackClient.BackpackTintSources.LeftFluidBlockTintSource` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java)
    pub mod travelers_backpack_client_backpack_tint_sources_left_fluid_block_tint_source {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockTintSource`. Its static parts are below.
        // TravelersBackpackClient.BackpackTintSources.LeftFluidBlockTintSource.color(1): not translated, its class isn't translated (it implements Minecraft's `BlockTintSource`).
        // TravelersBackpackClient.BackpackTintSources.LeftFluidBlockTintSource.colorInWorld(3): not translated, its class isn't translated (it implements Minecraft's `BlockTintSource`).
    }

    /// `TravelersBackpackClient.BackpackTintSources.LeftFluidItemTintSource` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java)
    pub mod travelers_backpack_client_backpack_tint_sources_left_fluid_item_tint_source {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ItemTintSource`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TravelersBackpackClient.BackpackTintSources.LeftFluidItemTintSource.LeftFluidItemTintSource(1): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
        // TravelersBackpackClient.BackpackTintSources.LeftFluidItemTintSource.calculate(3): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
        // TravelersBackpackClient.BackpackTintSources.LeftFluidItemTintSource.type(0): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
        // TravelersBackpackClient.BackpackTintSources.LeftFluidItemTintSource.defaultColor(0): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
    }

    /// `TravelersBackpackClient.BackpackTintSources.RightFluidBlockTintSource` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java)
    pub mod travelers_backpack_client_backpack_tint_sources_right_fluid_block_tint_source {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockTintSource`. Its static parts are below.
        // TravelersBackpackClient.BackpackTintSources.RightFluidBlockTintSource.color(1): not translated, its class isn't translated (it implements Minecraft's `BlockTintSource`).
        // TravelersBackpackClient.BackpackTintSources.RightFluidBlockTintSource.colorInWorld(3): not translated, its class isn't translated (it implements Minecraft's `BlockTintSource`).
    }

    /// `TravelersBackpackClient.BackpackTintSources.RightFluidItemTintSource` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java)
    pub mod travelers_backpack_client_backpack_tint_sources_right_fluid_item_tint_source {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ItemTintSource`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TravelersBackpackClient.BackpackTintSources.RightFluidItemTintSource.RightFluidItemTintSource(1): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
        // TravelersBackpackClient.BackpackTintSources.RightFluidItemTintSource.calculate(3): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
        // TravelersBackpackClient.BackpackTintSources.RightFluidItemTintSource.type(0): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
        // TravelersBackpackClient.BackpackTintSources.RightFluidItemTintSource.defaultColor(0): not translated, its class isn't translated (it implements Minecraft's `ItemTintSource`).
    }

    /// `TravelersBackpackMixinPlugin` (com/tiviacz/travelersbackpack/TravelersBackpackMixinPlugin.java)
    pub mod travelers_backpack_mixin_plugin {
        use super::*;
        // The class itself isn't translated: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility). Its static parts are below.
        // TravelersBackpackMixinPlugin.onLoad(1): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // TravelersBackpackMixinPlugin.getRefMapperConfig(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // TravelersBackpackMixinPlugin.shouldApplyMixin(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // TravelersBackpackMixinPlugin.acceptTargets(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // TravelersBackpackMixinPlugin.getMixins(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // TravelersBackpackMixinPlugin.preApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // TravelersBackpackMixinPlugin.postApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
    }

    /// `BackpackDeathHelper` (com/tiviacz/travelersbackpack/util/BackpackDeathHelper.java)
    pub mod backpack_death_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BackpackDeathHelper {
        }
        impl BackpackDeathHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BackpackDeathHelper.onPlayerDrops(3): not translated, it uses Minecraft's `Level`.
        // BackpackDeathHelper.placeBackpack(4): not translated, it uses Minecraft's `Level`.
        // BackpackDeathHelper.placeBackpackInTheWorld(5): not translated, it uses Minecraft's `Level`.
        // BackpackDeathHelper.tryPlace(3): not translated, it uses Minecraft's `Level`.
        // BackpackDeathHelper.getTicker(3): not translated, it's generic.
        // BackpackDeathHelper.getNearestEmptyChunkCoordinatesSpiral(9): not translated, it uses Minecraft's `BlockPos`.
        // BackpackDeathHelper.isTopSolid(3): not translated, it uses Minecraft's `Level`.
        // BackpackDeathHelper.checkCoordsForBackpack(6): not translated, it uses Minecraft's `BlockPos`.
        // BackpackDeathHelper.areCoordinatesTheSame(2): not translated, it uses Minecraft's `BlockPos`.
        // BackpackDeathHelper.findBlock3D(7): not translated, it uses Minecraft's `BlockPos`.
    }

    /// `ClientRegistryHelper` (com/tiviacz/travelersbackpack/util/ClientRegistryHelper.java)
    pub mod client_registry_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientRegistryHelper {
        }
        impl ClientRegistryHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientRegistryHelper.getRegistryAccess(0): not translated, it uses Minecraft's `RegistryAccess`.
    }

    /// `ContainerContentsHelper` (com/tiviacz/travelersbackpack/util/ContainerContentsHelper.java)
    pub mod container_contents_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ContainerContentsHelper {
        }
        impl ContainerContentsHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ContainerContentsHelper.getItems(2): not translated, it uses Minecraft's `NonNullList`.
        // ContainerContentsHelper.updateStack(4): not translated, it uses Minecraft's `ItemContainerContents`.
    }

    /// `CooldownHelper` (com/tiviacz/travelersbackpack/util/CooldownHelper.java)
    pub mod cooldown_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CooldownHelper {
        }
        impl CooldownHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CooldownHelper.createCooldown(2): not translated, it calls Minecraft's `RandomSource`.create.
        pub fn seconds(seconds: i32) -> i32 {
            return seconds * 20;
        }
    }

    /// `FluidStackHelper` (com/tiviacz/travelersbackpack/util/FluidStackHelper.java)
    pub mod fluid_stack_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct FluidStackHelper {
        }
        impl FluidStackHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FluidStackHelper.getFluidEmptySound(1): not translated, it uses Minecraft's `SoundEvent`.
        // FluidStackHelper.getFluidFillSound(1): not translated, it uses Minecraft's `SoundEvent`.
        // FluidStackHelper.setPotionFluidVariant(2): not translated, it uses Fabric's `FluidVariant`.
        // FluidStackHelper.getPotionTypeFromFluidVariant(1): not translated, it uses Minecraft's `PotionContents`.
        // FluidStackHelper.getItemStackFromFluidStack(1): not translated, it uses Minecraft's `ItemStack`.
        // FluidStackHelper.getSplashItemStackFromFluidStack(1): not translated, it uses Minecraft's `ItemStack`.
        // FluidStackHelper.getLingeringItemStackFromFluidStack(1): not translated, it uses Minecraft's `ItemStack`.
        // FluidStackHelper.createPotionStack(2): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `FluidTypeHelper` (com/tiviacz/travelersbackpack/util/FluidTypeHelper.java)
    pub mod fluid_type_helper {
        use super::*;
        pub const BUCKET_EMPTY: bool = false;
        pub const BUCKET_FILL: bool = true;
        #[derive(Clone, Debug)]
        pub struct FluidTypeHelper {
        }
        impl FluidTypeHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FluidTypeHelper.getFluidVariantName(1): not translated, it uses Minecraft's `Component`.
        // FluidTypeHelper.getSound(2): not translated, it uses Minecraft's `SoundEvent`.
    }

    /// `FluidUtil` (com/tiviacz/travelersbackpack/util/FluidUtil.java)
    pub mod fluid_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct FluidUtil {
        }
        impl FluidUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FluidUtil.getFluidStorageAtCursor(2): not translated, it uses Fabric's `Storage`.
        // FluidUtil.getFluidStorageAtSlot(1): not translated, it uses Fabric's `Storage`.
        // FluidUtil.hasFluidStorageConstant(1): not translated, it uses Minecraft's `ItemStack`.
        // FluidUtil.getFluidStorageConstant(1): not translated, it uses Fabric's `Storage`.
        // FluidUtil.hasFluid(2): not translated, it uses Minecraft's `Player`.
        // FluidUtil.hasFluid(1): not translated, it uses Fabric's `Storage`.
        // FluidUtil.tryEmptyContainerAtCursor(4): not translated, its body couldn't be read: a labelled statement.
        // FluidUtil.tryFillContainerAtCursor(4): not translated, its body couldn't be read: a labelled statement.
        // FluidUtil.tryFillBucketAtCursor(4): not translated, its body couldn't be read: a labelled statement.
        // FluidUtil.tryFillContainerAtSlot(5): not translated, it uses FluidTank, which isn't translated (it extends Fabric's `SingleVariantStorage`).
        // FluidUtil.isSameVariant(2): not translated, it uses Fabric's `FluidVariant`.
    }

    /// `HumanoidRenderStateBackpackInject` (com/tiviacz/travelersbackpack/util/HumanoidRenderStateBackpackInject.java)
    pub mod humanoid_render_state_backpack_inject {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // HumanoidRenderStateBackpackInject.setBackpackStack(1): not translated, it has no body.
        // HumanoidRenderStateBackpackInject.getBackpackStack(0): not translated, it has no body.
        // HumanoidRenderStateBackpackInject.setChestItem(1): not translated, it has no body.
        // HumanoidRenderStateBackpackInject.setName(1): not translated, it has no body.
        // HumanoidRenderStateBackpackInject.getName(0): not translated, it has no body.
    }

    /// `InventoryHelper` (com/tiviacz/travelersbackpack/util/InventoryHelper.java)
    pub mod inventory_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct InventoryHelper {
        }
        impl InventoryHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // InventoryHelper.removeItem(3): not translated, it uses Minecraft's `ItemStack`.
        // InventoryHelper.removeItemShiftClick(3): not translated, it uses Minecraft's `ItemStack`.
        // InventoryHelper.takeItem(2): not translated, it uses Minecraft's `ItemStack`.
        // InventoryHelper.isEmpty(1): not translated, it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`).
        // InventoryHelper.iteratePlayerInv(2): not translated, it uses Minecraft's `Inventory`.
        // InventoryHelper.iterateHandler(2): not translated, it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`).
        // InventoryHelper.iterate(2): not translated, it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`).
        // InventoryHelper.addItemStackToHandler(3): not translated, it uses Minecraft's `ItemStack`.
        // InventoryHelper.extractFromBackpack(4): not translated, it uses Minecraft's `ItemStack`.
        // InventoryHelper.insertItemStacked(3): not translated, it uses Minecraft's `ItemStack`.
        // InventoryHelper.insertItem(3): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `ItemStackUtils` (com/tiviacz/travelersbackpack/util/ItemStackUtils.java)
    pub mod item_stack_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ItemStackUtils {
        }
        impl ItemStackUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ItemStackUtils.isSameItemSameTags(2): not translated, it uses Minecraft's `ItemStack`.
        // ItemStackUtils.isSameItemSameComponents(2): not translated, it uses Minecraft's `ItemStack`.
        // ItemStackUtils.checkComponentsIgnoreDamage(2): not translated, it uses Minecraft's `DataComponentMap`.
        // ItemStackUtils.createDataComponentMap(2): not translated, it uses Minecraft's `DataComponentMap`.
        // ItemStackUtils.reduceSize(1): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `KeyHelper` (com/tiviacz/travelersbackpack/util/KeyHelper.java)
    pub mod key_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct KeyHelper {
        }
        impl KeyHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // KeyHelper.isShiftPressed(0): not translated, it calls Minecraft's `InputConstants`.isKeyDown.
        // KeyHelper.isCtrlPressed(0): not translated, it calls Minecraft's `InputConstants`.isKeyDown.
    }

    /// `LogHelper` (com/tiviacz/travelersbackpack/util/LogHelper.java)
    pub mod log_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LogHelper {
        }
        impl LogHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LogHelper.log(3): not translated, it uses `Level` (org.apache.logging.log4j).
        // LogHelper.info(2): not translated, it uses `Object`.
        // LogHelper.error(2): not translated, it uses `Object`.
    }

    /// `PacketDistributor` (com/tiviacz/travelersbackpack/util/PacketDistributor.java)
    pub mod packet_distributor {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PacketDistributor {
        }
        impl PacketDistributor {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PacketDistributor.sendToServer(1): not translated, it uses Minecraft's `CustomPacketPayload`.
        // PacketDistributor.sendToPlayer(2): not translated, it uses Minecraft's `ServerPlayer`.
        // PacketDistributor.sendToPlayersTrackingEntityAndSelf(2): not translated, it uses Minecraft's `Player`.
        // PacketDistributor.sendToAllPlayers(2): not translated, it uses Minecraft's `CustomPacketPayload`.
    }

    /// `Reference` (com/tiviacz/travelersbackpack/util/Reference.java)
    pub mod reference {
        use super::*;
        pub const ITEM_SCREEN_ID: i8 = 1i8;
        pub const WEARABLE_SCREEN_ID: i8 = 2i8;
        pub const BLOCK_ENTITY_SCREEN_ID: i8 = 3i8;
        pub const NO_SPACE: &str = "action.travelersbackpack.unequip_nospace";
        pub const DEPLOY: &str = "action.travelersbackpack.deploy_sleeping_bag";
        // ALLOWED_TYPE_ENTRIES: not translated, it uses Minecraft's `EntityType`.
        #[derive(Clone, Debug)]
        pub struct Reference {
        }
        impl Reference {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `RegistryHelper` (com/tiviacz/travelersbackpack/util/RegistryHelper.java)
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
        // RegistryHelper.getRegistryAccess(0): not translated, it uses Minecraft's `RegistryAccess`.
    }

    /// `RenderHelper` (com/tiviacz/travelersbackpack/util/RenderHelper.java)
    pub mod render_helper {
        use super::*;
        pub static SLOT_HIGHLIGHT_BACK_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/slot_highlight_back"));
        pub static SLOT_HIGHLIGHT_FRONT_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/slot_highlight_front"));
        #[derive(Clone, Debug)]
        pub struct RenderHelper {
        }
        impl RenderHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RenderHelper.renderScreenTank(7): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // RenderHelper.renderScreenTank(9): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // RenderHelper.renderSlotHighlightBack(3): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // RenderHelper.renderSlotHighlightFront(3): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
    }

    /// `Supporters` (com/tiviacz/travelersbackpack/util/Supporters.java)
    pub mod supporters {
        use super::*;
        pub static SUPPORTERS_REFERENCE: LazyLock<Vec<String>> = LazyLock::new(|| Vec::new());
        pub static SUPPORTERS: LazyLock<Vec<String>> = LazyLock::new(|| Vec::new());
        #[derive(Clone, Debug)]
        pub struct Supporters {
        }
        impl Supporters {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Supporters.fetchSupporters(0): not translated, a future's callback (it runs on the game thread).
        // Supporters.updateSupporters(0): not translated, a future's callback (it runs on the game thread).
        // Supporters.getGistFileAsync(0): not translated, it calls `HttpRequest` (java.net.http).newBuilder.
    }

    /// `TextUtils` (com/tiviacz/travelersbackpack/util/TextUtils.java)
    pub mod text_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TextUtils {
        }
        impl TextUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TextUtils.getTranslatedSplittedText(2): not translated, it uses Minecraft's `Component`.
        pub fn get_converted_time(ticks: i32) -> String {
            let i: i32 = ticks / 20;
            let minutes: i32 = i / 60;
            let seconds: i32 = i % 60;
            return if seconds < 10 { format!("{}:0{}", minutes, seconds) } else { format!("{}:{}", minutes, seconds) };
        }
    }

    /// JOML's Vector3f as a value (Java's methods change the vector; the translation writes the result back).
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct JVec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl JVec3 {
        pub const fn new(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z }
        }
        pub fn add(self, o: JVec3) -> Self {
            Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
        }
        pub fn add3(self, x: f32, y: f32, z: f32) -> Self {
            Self::new(self.x + x, self.y + y, self.z + z)
        }
        pub fn sub(self, o: JVec3) -> Self {
            Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
        }
        pub fn mul_s(self, s: f32) -> Self {
            Self::new(self.x * s, self.y * s, self.z * s)
        }
        pub fn mul_v(self, o: JVec3) -> Self {
            Self::new(self.x * o.x, self.y * o.y, self.z * o.z)
        }
        pub fn negate(self) -> Self {
            Self::new(-self.x, -self.y, -self.z)
        }
        pub fn dot(self, o: JVec3) -> f32 {
            self.x * o.x + self.y * o.y + self.z * o.z
        }
        pub fn length(self) -> f32 {
            self.dot(self).sqrt()
        }
        pub fn length_squared(self) -> f32 {
            self.dot(self)
        }
        /// JOML's normalize: divides by the length (a zero vector becomes NaNs, as in Java).
        pub fn normalize(self) -> Self {
            let inv = 1.0 / self.length();
            self.mul_s(inv)
        }
        pub fn cross(self, o: JVec3) -> Self {
            Self::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
        }
        /// JOML's lerp: this + (other - this) * t.
        pub fn lerp(self, o: JVec3, t: f32) -> Self {
            Self::new(self.x + (o.x - self.x) * t, self.y + (o.y - self.y) * t, self.z + (o.z - self.z) * t)
        }
        pub fn distance(self, o: JVec3) -> f32 {
            self.sub(o).length()
        }
    }
    
    /// JOML's Quaternionf (x, y, z, w).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JQuat {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    impl Default for JQuat {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JQuat {
        pub const IDENTITY: JQuat = JQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            Self { x, y, z, w }
        }
        /// `rotationX(angle)`: a turn about +X by `angle` radians (Axis.XP.rotation).
        pub fn rotation_x(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(s, 0.0, 0.0, c)
        }
        pub fn rotation_y(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, s, 0.0, c)
        }
        pub fn rotation_z(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, 0.0, s, c)
        }
        pub fn normalize(self) -> Self {
            let inv = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
            Self::new(self.x * inv, self.y * inv, self.z * inv, self.w * inv)
        }
        /// this * q.
        pub fn mul(self, q: JQuat) -> Self {
            Self::new(
                self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
                self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x,
                self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w,
                self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z,
            )
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.mul(Self::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.mul(Self::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.mul(Self::rotation_z(a))
        }
    }
    
    /// JOML's Matrix3f, column-major, from three column vectors.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat3 {
        pub m: [f32; 9],
    }
    impl JMat3 {
        pub fn from_cols(c0: JVec3, c1: JVec3, c2: JVec3) -> Self {
            Self { m: [c0.x, c0.y, c0.z, c1.x, c1.y, c1.z, c2.x, c2.y, c2.z] }
        }
    }
    
    /// JOML's Matrix4f: column-major, `m[col * 4 + row]`; `a.mul(b)` is a * b, and translate/rotate/scale
    /// multiply on the right, as JOML does.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat4 {
        pub m: [f32; 16],
    }
    impl Default for JMat4 {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JMat4 {
        pub const IDENTITY: JMat4 = JMat4 { m: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0] };
        pub fn identity() -> Self {
            Self::IDENTITY
        }
        fn at(&self, row: usize, col: usize) -> f32 {
            self.m[col * 4 + row]
        }
        pub fn mul(self, r: JMat4) -> Self {
            let mut out = [0.0f32; 16];
            for col in 0..4 {
                for row in 0..4 {
                    let mut v = 0.0;
                    for k in 0..4 {
                        v += self.at(row, k) * r.at(k, col);
                    }
                    out[col * 4 + row] = v;
                }
            }
            Self { m: out }
        }
        pub fn translation(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[12] = x;
            t.m[13] = y;
            t.m[14] = z;
            t
        }
        pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::translation(x, y, z))
        }
        pub fn translate_v(self, v: JVec3) -> Self {
            self.translate(v.x, v.y, v.z)
        }
        pub fn scaling(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[0] = x;
            t.m[5] = y;
            t.m[10] = z;
            t
        }
        pub fn scale(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::scaling(x, y, z))
        }
        pub fn scale_s(self, s: f32) -> Self {
            self.scale(s, s, s)
        }
        /// The rotation a unit quaternion stands for.
        pub fn rotation(q: JQuat) -> Self {
            let (x, y, z, w) = (q.x, q.y, q.z, q.w);
            let (xx, yy, zz) = (x * x, y * y, z * z);
            let (xy, xz, yz, wx, wy, wz) = (x * y, x * z, y * z, w * x, w * y, w * z);
            Self {
                m: [
                    1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0,
                    2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0,
                    2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ],
            }
        }
        pub fn rotate(self, q: JQuat) -> Self {
            self.mul(Self::rotation(q))
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_z(a))
        }
        /// JOML's rotateXYZ: about X, then Y, then Z.
        pub fn rotate_xyz(self, x: f32, y: f32, z: f32) -> Self {
            self.rotate_x(x).rotate_y(y).rotate_z(z)
        }
        /// JOML's rotateAround(q, ox, oy, oz): about the point (ox, oy, oz).
        pub fn rotate_around(self, q: JQuat, ox: f32, oy: f32, oz: f32) -> Self {
            self.translate(ox, oy, oz).rotate(q).translate(-ox, -oy, -oz)
        }
        /// The general inverse (JOML's invert); a singular matrix gives infinities, as in Java.
        pub fn invert(self) -> Self {
            let a = &self.m;
            let mut inv = [0.0f32; 16];
            inv[0] = a[5] * a[10] * a[15] - a[5] * a[11] * a[14] - a[9] * a[6] * a[15] + a[9] * a[7] * a[14] + a[13] * a[6] * a[11] - a[13] * a[7] * a[10];
            inv[4] = -a[4] * a[10] * a[15] + a[4] * a[11] * a[14] + a[8] * a[6] * a[15] - a[8] * a[7] * a[14] - a[12] * a[6] * a[11] + a[12] * a[7] * a[10];
            inv[8] = a[4] * a[9] * a[15] - a[4] * a[11] * a[13] - a[8] * a[5] * a[15] + a[8] * a[7] * a[13] + a[12] * a[5] * a[11] - a[12] * a[7] * a[9];
            inv[12] = -a[4] * a[9] * a[14] + a[4] * a[10] * a[13] + a[8] * a[5] * a[14] - a[8] * a[6] * a[13] - a[12] * a[5] * a[10] + a[12] * a[6] * a[9];
            inv[1] = -a[1] * a[10] * a[15] + a[1] * a[11] * a[14] + a[9] * a[2] * a[15] - a[9] * a[3] * a[14] - a[13] * a[2] * a[11] + a[13] * a[3] * a[10];
            inv[5] = a[0] * a[10] * a[15] - a[0] * a[11] * a[14] - a[8] * a[2] * a[15] + a[8] * a[3] * a[14] + a[12] * a[2] * a[11] - a[12] * a[3] * a[10];
            inv[9] = -a[0] * a[9] * a[15] + a[0] * a[11] * a[13] + a[8] * a[1] * a[15] - a[8] * a[3] * a[13] - a[12] * a[1] * a[11] + a[12] * a[3] * a[9];
            inv[13] = a[0] * a[9] * a[14] - a[0] * a[10] * a[13] - a[8] * a[1] * a[14] + a[8] * a[2] * a[13] + a[12] * a[1] * a[10] - a[12] * a[2] * a[9];
            inv[2] = a[1] * a[6] * a[15] - a[1] * a[7] * a[14] - a[5] * a[2] * a[15] + a[5] * a[3] * a[14] + a[13] * a[2] * a[7] - a[13] * a[3] * a[6];
            inv[6] = -a[0] * a[6] * a[15] + a[0] * a[7] * a[14] + a[4] * a[2] * a[15] - a[4] * a[3] * a[14] - a[12] * a[2] * a[7] + a[12] * a[3] * a[6];
            inv[10] = a[0] * a[5] * a[15] - a[0] * a[7] * a[13] - a[4] * a[1] * a[15] + a[4] * a[3] * a[13] + a[12] * a[1] * a[7] - a[12] * a[3] * a[5];
            inv[14] = -a[0] * a[5] * a[14] + a[0] * a[6] * a[13] + a[4] * a[1] * a[14] - a[4] * a[2] * a[13] - a[12] * a[1] * a[6] + a[12] * a[2] * a[5];
            inv[3] = -a[1] * a[6] * a[11] + a[1] * a[7] * a[10] + a[5] * a[2] * a[11] - a[5] * a[3] * a[10] - a[9] * a[2] * a[7] + a[9] * a[3] * a[6];
            inv[7] = a[0] * a[6] * a[11] - a[0] * a[7] * a[10] - a[4] * a[2] * a[11] + a[4] * a[3] * a[10] + a[8] * a[2] * a[7] - a[8] * a[3] * a[6];
            inv[11] = -a[0] * a[5] * a[11] + a[0] * a[7] * a[9] + a[4] * a[1] * a[11] - a[4] * a[3] * a[9] - a[8] * a[1] * a[7] + a[8] * a[3] * a[5];
            inv[15] = a[0] * a[5] * a[10] - a[0] * a[6] * a[9] - a[4] * a[1] * a[10] + a[4] * a[2] * a[9] + a[8] * a[1] * a[6] - a[8] * a[2] * a[5];
            let det = a[0] * inv[0] + a[1] * inv[4] + a[2] * inv[8] + a[3] * inv[12];
            let d = 1.0 / det;
            for v in inv.iter_mut() {
                *v *= d;
            }
            Self { m: inv }
        }
        /// The point v moved by this matrix (w = 1).
        pub fn transform_position(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z + self.at(0, 3),
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z + self.at(1, 3),
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z + self.at(2, 3),
            )
        }
        /// The direction v turned by this matrix (w = 0).
        pub fn transform_direction(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z,
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z,
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z,
            )
        }
        /// JOML's set3x3: the upper-left 3x3 from `m`, the rest kept.
        pub fn set3x3(self, m: JMat3) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = m.m[col * 3 + row];
                }
            }
            out
        }
        /// JOML's transpose3x3: the upper-left 3x3 transposed, the rest kept.
        pub fn transpose3x3(self) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = self.m[row * 4 + col];
                }
            }
            out
        }
        pub fn get_translation(&self) -> JVec3 {
            JVec3::new(self.m[12], self.m[13], self.m[14])
        }
    }
    
    /// Minecraft's PoseStack: a stack of matrices (normals aren't kept: Horizon's hand renderer takes one matrix).
    #[derive(Clone, Debug)]
    pub struct JPoseStack {
        pub stack: Vec<JMat4>,
    }
    impl Default for JPoseStack {
        fn default() -> Self {
            Self { stack: vec![JMat4::IDENTITY] }
        }
    }
    impl JPoseStack {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn of(m: JMat4) -> Self {
            Self { stack: vec![m] }
        }
        fn top(&mut self) -> &mut JMat4 {
            self.stack.last_mut().expect("PoseStack is empty")
        }
        pub fn push_pose(&mut self) {
            let m = *self.top();
            self.stack.push(m);
        }
        pub fn pop_pose(&mut self) {
            self.stack.pop();
        }
        /// Writes the top matrix (Java changed `pose.last().pose()` in place).
        pub fn set_last(&mut self, m: JMat4) {
            *self.top() = m;
        }
        pub fn last_pose(&self) -> JMat4 {
            *self.stack.last().expect("PoseStack is empty")
        }
        pub fn set_identity(&mut self) {
            *self.top() = JMat4::IDENTITY;
        }
        pub fn translate(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.translate(x, y, z);
        }
        pub fn scale(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.scale(x, y, z);
        }
        pub fn mul_pose(&mut self, q: JQuat) {
            let m = self.top();
            *m = m.rotate(q);
        }
        pub fn mul_pose_matrix(&mut self, r: JMat4) {
            let m = self.top();
            *m = m.mul(r);
        }
        pub fn rotate_around(&mut self, q: JQuat, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.rotate_around(q, x, y, z);
        }
    }
    
    #[cfg(test)]
    mod joml_tests {
        use super::*;
    
        fn close(a: JVec3, b: JVec3) -> bool {
            (a.x - b.x).abs() < 1e-5 && (a.y - b.y).abs() < 1e-5 && (a.z - b.z).abs() < 1e-5
        }
    
        #[test]
        fn rotations_turn_axes_as_joml_does() {
            let q = JQuat::rotation_x(std::f32::consts::FRAC_PI_2);
            // A positive turn about X takes +Y to +Z (right-handed), as JOML's rotationX.
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(0.0, 0.0, 1.0)));
            let q = JQuat::rotation_y(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 0.0, 1.0)), JVec3::new(1.0, 0.0, 0.0)));
            let q = JQuat::rotation_z(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
        }
    
        #[test]
        fn translate_then_rotate_multiplies_on_the_right() {
            // JOML: new Matrix4f().translate(1, 0, 0).rotateZ(90) moves (1, 0, 0) to (1, 1, 0).
            let m = JMat4::identity().translate(1.0, 0.0, 0.0).rotate_z(std::f32::consts::FRAC_PI_2);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(1.0, 1.0, 0.0)));
        }
    
        #[test]
        fn invert_undoes() {
            let m = JMat4::identity().translate(1.0, 2.0, 3.0).rotate_x(0.7).rotate_y(-0.3).scale(2.0, 0.5, 1.5);
            let p = JVec3::new(0.3, -1.2, 4.0);
            assert!(close(m.invert().transform_position(m.transform_position(p)), p));
            assert!(close(m.mul(m.invert()).transform_position(p), p));
        }
    
        #[test]
        fn rotate_around_keeps_its_point() {
            let m = JMat4::identity().rotate_around(JQuat::rotation_y(1.1), 0.5, 0.25, 0.0);
            assert!(close(m.transform_position(JVec3::new(0.5, 0.25, 0.0)), JVec3::new(0.5, 0.25, 0.0)));
        }
    
        #[test]
        fn pose_stack_push_pop() {
            let mut p = JPoseStack::new();
            p.translate(1.0, 0.0, 0.0);
            p.push_pose();
            p.scale(2.0, 2.0, 2.0);
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(3.0, 0.0, 0.0)));
            p.pop_pose();
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(2.0, 0.0, 0.0)));
        }
    
        #[test]
        fn quaternion_matrix_agrees_with_axis_turns() {
            let q = JQuat::rotation_x(0.4).mul(JQuat::rotation_y(-0.9));
            let a = JMat4::rotation(q);
            let b = JMat4::identity().rotate_x(0.4).rotate_y(-0.9);
            let p = JVec3::new(1.0, 2.0, 3.0);
            assert!(close(a.transform_position(p), b.transform_position(p)));
        }
    
        #[test]
        fn set3x3_and_transpose() {
            let m3 = JMat3::from_cols(JVec3::new(0.0, 1.0, 0.0), JVec3::new(-1.0, 0.0, 0.0), JVec3::new(0.0, 0.0, 1.0));
            let m = JMat4::identity().set3x3(m3);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
            assert!(close(m.transpose3x3().transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(1.0, 0.0, 0.0)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "travelersbackpack");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
