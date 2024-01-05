use std::collections::HashMap;

#[derive(Debug)]
pub(super) struct Day19Inputs {
    pub parts: Vec<Part>,
    pub keyed_workflows: HashMap<String, Workflow>,
}
impl Day19Inputs {
    pub fn new(workflows: Vec<Workflow>, parts: Vec<Part>) -> Self {
        let keyed_workflows = HashMap::from_iter(workflows.iter().map(|w| (w.name.clone(), w.clone())));
        Day19Inputs { parts, keyed_workflows }
    }
}

#[derive(Debug, Clone)]
pub(super) enum Prop {
    X,
    M,
    A,
    S,
}
impl From<&str> for Prop {
    fn from(value: &str) -> Self {
        match value {
            "x" | "X" => Self::X,
            "m" | "M" => Self::M,
            "a" | "A" => Self::A,
            "s" | "S" => Self::S,
            s => panic!("Unepected prop name: {s}"),
        }
    }
}

#[derive(Debug)]
pub(super) struct Part {
    pub x: i64, // Extremely cool looking
    pub m: i64, // Musical (it makes a noise when you hit it)
    pub a: i64, // Aerodynamic
    pub s: i64, // Shiny
}
impl Part {
    pub fn get(&self, prop: &Prop) -> i64 {
        match prop {
            Prop::X => self.x,
            Prop::M => self.m,
            Prop::A => self.a,
            Prop::S => self.s,
        }
    }

    pub fn sum_value(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
pub(super) struct Workflow {
    pub(super) name: String,
    pub(super) branches: Vec<Branch>,
}
impl Workflow {
    pub fn run_to_action(&self, part: &Part) -> &Action {
        self.branches
            .iter()
            .find_map(|b| {
                if b.predicate.is_true(part) {
                    Some(&b.action)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("No action found for part {:?}", part))
    }
}

#[derive(Debug, Clone)]
pub(super) struct Branch {
    pub(super) predicate: Predicate,
    pub(super) action: Action,
}

#[derive(Debug, Clone)]
pub(super) enum Predicate {
    Always,
    LessThan { prop: Prop, value: i64 },
    GreaterThan { prop: Prop, value: i64 },
}
impl Predicate {
    fn is_true(&self, part: &Part) -> bool {
        match self {
            Self::Always => true,
            Self::LessThan { prop, value } => part.get(prop) < *value,
            Self::GreaterThan { prop, value } => part.get(prop) > *value,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum Action {
    RunWorkflow(String),
    Reject,
    Accept,
}
impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "R" => Self::Reject,
            "A" => Self::Accept,
            name => Self::RunWorkflow(name.to_owned()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct PartRanges {
    pub x: (i64, i64),
    pub m: (i64, i64),
    pub a: (i64, i64),
    pub s: (i64, i64),
}
impl PartRanges {
    pub fn get(&self, prop: &Prop) -> &(i64, i64) {
        match prop {
            Prop::X => &self.x,
            Prop::M => &self.m,
            Prop::A => &self.a,
            Prop::S => &self.s,
        }
    }
    pub fn get_mut(&mut self, prop: &Prop) -> &mut (i64, i64) {
        match prop {
            Prop::X => &mut self.x,
            Prop::M => &mut self.m,
            Prop::A => &mut self.a,
            Prop::S => &mut self.s,
        }
    }
    pub fn combinations(&self) -> i64 {
        Self::size(self.x) * Self::size(self.m) * Self::size(self.a) * Self::size(self.s)
    }
    fn size(range: (i64, i64)) -> i64 {
        (range.1 - range.0).abs() + 1
    }

    pub(crate) fn reduce_range_to_below(&mut self, prop: &Prop, value: i64) {
        let range = self.get_mut(prop);
        range.1 = value - 1;
    }

    pub(crate) fn reduce_range_to_above(&mut self, prop: &Prop, value: i64) {
        let range = self.get_mut(prop);
        range.0 = value + 1;
    }
}
impl Default for PartRanges {
    fn default() -> Self {
        Self {
            x: (1_i64, 4000),
            m: (1_i64, 4000),
            a: (1_i64, 4000),
            s: (1_i64, 4000),
        }
    }
}
