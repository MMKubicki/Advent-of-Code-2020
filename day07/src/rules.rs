use std::collections::{HashMap, HashSet};
use std::fmt;
use std::iter::FromIterator;

// Rules
//##################

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Rules {
    rules: HashMap<BagInfo, Vec<BagCount>>,
}

impl Rules {
    pub fn can_contain(&self, bag: &BagInfo) -> Vec<&BagInfo> {
        // Collection for all possible "parents"
        let mut can = HashSet::new();

        // collection for upper generation search
        // -> at start we search for parents of given bag
        let mut search_targets = HashSet::new();
        search_targets.insert(bag);

        // loop until no new parents are found
        'outer: loop {
            // collection for parents found this loop
            let mut found_this_loop = HashSet::new();

            // go through all rules
            for (key, rule_content) in self.rules.iter() {
                // see if rule wraps one of search_targets in other bag
                // if yes -> new parent found -> put in collection -> continue with next rule
                if rule_content.iter().any(|c| search_targets.contains(&c.bag)) {
                    found_this_loop.insert(key);
                }
            }

            // if found new parents save them and begin search again with new parents as search-children
            if !found_this_loop.is_empty() {
                can.extend(&found_this_loop);
                search_targets = found_this_loop;
            } else {
                break 'outer;
            }
        }

        can.into_iter().collect()
    }

    pub fn get_contents_bag_count(&self, bag: &BagInfo) -> usize {
        // for given bag:
        //      search for rule what has to be inside (sub bags)
        //          if not found return 0 (last match statement)
        //      map those to (count of sub bag) * (sub bag itself + content_of sub bag)
        //          recurse till end (last sub bag will have count 0 and no rules begin with it)
        //      sum those up

        let result = self.rules.get(bag).map(|rule| {
            rule.iter()
                .map(|sub_bag| sub_bag.count * (1 + self.get_contents_bag_count(&sub_bag.bag)))
                .sum()
        });

        match result {
            Some(value) => value,
            None => 0,
        }
    }
}

impl<T: AsRef<str>> From<T> for Rules {
    fn from(s: T) -> Self {
        let s = s.as_ref();

        // one rule per line
        // parse line individually and collect to HashMap
        //
        // basic overview
        // [ADJECTIVE] [COLOR] bags contain {[COUNT] [ADJECTIVE] [COLOR] bag{s}{, }}.

        let rules = s
            .lines()
            .map(Rule::from)
            .map(Rule::deconstruct)
            .collect::<HashMap<_, _>>();

        Rules { rules }
    }
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for (key, value) in self.rules.iter() {
            s.push_str(&format!("{} CONTAINS ", key));

            s.push_str(
                &value
                    .iter()
                    .map(BagCount::to_string)
                    .collect::<Vec<_>>()
                    .join(", "),
            );

            s.push('\n');
        }

        f.write_str(&s)
    }
}

//##################

// Rule
//##################

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Rule {
    key: BagInfo,
    value: Vec<BagCount>,
}

impl Rule {
    pub fn deconstruct(self) -> (BagInfo, Vec<BagCount>) {
        (self.key, self.value)
    }
}

impl<T: AsRef<str>> From<T> for Rule {
    fn from(s: T) -> Self {
        let s = s.as_ref();

        // Rule format:
        // [BAG PROPERTIES] contain {[BAG COUNT], }.
        //
        // remove "." suffix
        // split by contain
        // parse first as BagInfo
        // split second by ", " to separate and then parse bag counts

        let stripped = s.strip_suffix('.').unwrap();

        let mut iter = stripped.split("contain ");
        let key = iter.next().unwrap().into();

        let value = iter
            .next()
            .unwrap()
            .split(", ")
            .map(BagCount::from)
            .collect::<Vec<_>>();

        Rule { key, value }
    }
}

//##################

// BagCount
//##################

#[derive(Debug, Default, Hash, Clone, Eq, PartialEq)]
pub struct BagCount {
    count: usize,
    bag: BagInfo,
}

impl<T: AsRef<str>> From<T> for BagCount {
    fn from(s: T) -> Self {
        let s = s.as_ref();

        // format:
        // [COUNT] [BAG PROPERTY]
        //
        // split by " "
        // parse first as integer
        // parse second as BagInfo
        // discard rest

        if s.contains("no other bags") {
            return Default::default();
        }

        let mut iter = s.split(' ');

        let count = iter.next().unwrap().parse::<usize>().unwrap();
        let bag = iter.collect();

        BagCount { count, bag }
    }
}

impl fmt::Display for BagCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.count {
            0 => write!(f, "nothing"),
            _ => write!(f, "{}x{}", self.count, self.bag),
        }
    }
}

//##################

// BagInfo
//##################

#[derive(Debug, Default, Hash, Clone, Eq, PartialEq)]
pub struct BagInfo {
    adjective: String,
    color: String,
}

impl<A: AsRef<str> + Clone> FromIterator<A> for BagInfo {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut iter = iter.into_iter();

        // first item = adjective
        // second item = color
        // keep both as string
        // discard rest (bag/bags/...)

        let adjective = iter.next().unwrap().as_ref().to_owned();
        let color = iter.next().unwrap().as_ref().to_owned();

        BagInfo { adjective, color }
    }
}

impl<T: AsRef<str>> From<T> for BagInfo {
    fn from(s: T) -> Self {
        s.as_ref().split(' ').collect()
    }
}

impl fmt::Display for BagInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {} bag]", self.adjective, self.color)
    }
}

//##################
