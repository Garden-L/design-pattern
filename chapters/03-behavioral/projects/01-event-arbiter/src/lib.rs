use std::{cell::RefCell, rc::Rc};

/// A candidate that may become the single active event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub id: &'static str,
    pub priority: i32,
    pub distance: u32,
    pub cooldown: bool,
}

/// Runtime information shared by filters and scoring strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Context {
    pub now: u64,
}

/// A pipeline step that decides whether a candidate remains eligible.
pub trait EventFilter {
    fn accepts(&self, event: &Event, context: &Context) -> bool;
}

/// Rejects events that are currently cooling down.
#[derive(Debug, Default, Clone, Copy)]
pub struct RejectCooldown;

impl EventFilter for RejectCooldown {
    fn accepts(&self, event: &Event, _context: &Context) -> bool {
        !event.cooldown
    }
}

/// Rejects events farther away than the configured limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxDistance {
    max: u32,
}

impl MaxDistance {
    pub fn new(max: u32) -> Self {
        Self { max }
    }

    pub fn max(&self) -> u32 {
        self.max
    }
}

impl EventFilter for MaxDistance {
    fn accepts(&self, event: &Event, _context: &Context) -> bool {
        event.distance <= self.max
    }
}

/// A replaceable strategy that assigns a score to an eligible candidate.
pub trait EventScorer {
    fn score(&self, event: &Event, context: &Context) -> i32;
}

/// Scores candidates by their declared priority.
#[derive(Debug, Default, Clone, Copy)]
pub struct PriorityScorer;

impl EventScorer for PriorityScorer {
    fn score(&self, event: &Event, _context: &Context) -> i32 {
        event.priority
    }
}

/// Rewards proximity by subtracting distance from the total score.
#[derive(Debug, Default, Clone, Copy)]
pub struct DistanceScorer;

impl EventScorer for DistanceScorer {
    fn score(&self, event: &Event, _context: &Context) -> i32 {
        -(event.distance as i32)
    }
}

/// Composes multiple scoring strategies by adding their scores.
pub struct CombinedScorer {
    scorers: Vec<Box<dyn EventScorer>>,
}

impl CombinedScorer {
    pub fn new(scorers: Vec<Box<dyn EventScorer>>) -> Self {
        Self { scorers }
    }
}

impl EventScorer for CombinedScorer {
    fn score(&self, event: &Event, context: &Context) -> i32 {
        self.scorers
            .iter()
            .map(|scorer| scorer.score(event, context))
            .sum()
    }
}

/// The value returned by reconciliation and delivered to listeners on change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionResult {
    pub previous: Option<&'static str>,
    pub active: Option<&'static str>,
    pub deactivated: Option<&'static str>,
    pub reason: String,
    pub changed: bool,
}

/// Receives active-event transitions without owning selection policy.
pub trait EventListener {
    fn on_selection_changed(&mut self, result: &SelectionResult);
}

/// A small Observer implementation useful in examples and tests.
#[derive(Clone, Default)]
pub struct RecordingListener {
    records: Rc<RefCell<Vec<SelectionResult>>>,
}

impl RecordingListener {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn records(&self) -> Vec<SelectionResult> {
        self.records.borrow().clone()
    }
}

impl EventListener for RecordingListener {
    fn on_selection_changed(&mut self, result: &SelectionResult) {
        self.records.borrow_mut().push(result.clone());
    }
}

/// Coordinates filtering, scoring, single-active reconciliation, and observers.
pub struct EventArbiter {
    filters: Vec<Box<dyn EventFilter>>,
    scorer: Box<dyn EventScorer>,
    active: Option<&'static str>,
    listeners: Vec<Box<dyn EventListener>>,
}

impl EventArbiter {
    pub fn new(filters: Vec<Box<dyn EventFilter>>, scorer: Box<dyn EventScorer>) -> Self {
        Self {
            filters,
            scorer,
            active: None,
            listeners: Vec::new(),
        }
    }

    pub fn add_listener<L>(&mut self, listener: L)
    where
        L: EventListener + 'static,
    {
        self.listeners.push(Box::new(listener));
    }

    pub fn active(&self) -> Option<&'static str> {
        self.active
    }

    pub fn reconcile(&mut self, candidates: &[Event], context: &Context) -> SelectionResult {
        let previous = self.active;
        let selected = self.select(candidates, context);
        let changed = previous != selected;
        let result = SelectionResult {
            previous,
            active: selected,
            deactivated: previous.filter(|old| Some(*old) != selected),
            reason: match selected {
                Some(id) if changed => format!("selected highest-scoring candidate: {id}"),
                Some(id) => format!("active event retained: {id}"),
                None => "no candidate passed the filters".to_owned(),
            },
            changed,
        };

        self.active = selected;
        if changed {
            for listener in &mut self.listeners {
                listener.on_selection_changed(&result);
            }
        }
        result
    }

    fn select(&self, candidates: &[Event], context: &Context) -> Option<&'static str> {
        let mut best: Option<(&Event, i32)> = None;
        for candidate in candidates {
            if !self
                .filters
                .iter()
                .all(|filter| filter.accepts(candidate, context))
            {
                continue;
            }

            let score = self.scorer.score(candidate, context);
            let should_replace = best
                .map(|(current, current_score)| is_better(candidate, score, current, current_score))
                .unwrap_or(true);
            if should_replace {
                best = Some((candidate, score));
            }
        }
        best.map(|(event, _)| event.id)
    }
}

fn is_better(candidate: &Event, score: i32, current: &Event, current_score: i32) -> bool {
    score > current_score
        || (score == current_score
            && (candidate.priority > current.priority
                || (candidate.priority == current.priority
                    && (candidate.distance < current.distance
                        || (candidate.distance == current.distance && candidate.id < current.id)))))
}
