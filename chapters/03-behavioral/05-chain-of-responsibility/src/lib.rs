//! 책임 연쇄 패턴을 단일 활성 이벤트 관리자에 적용한 예제.

/// 이벤트 후보. 한 번의 평가에서 최종적으로 하나만 활성화될 수 있다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: &'static str,
    pub priority: u32,
    pub urgent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventRequest {
    pub candidates: Vec<Event>,
}

impl EventRequest {
    pub fn new(candidates: Vec<Event>) -> Self {
        Self { candidates }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }
}

/// `Continue`만 다음 처리기로 전달되고, `Activate`는 즉시 연쇄를 종료한다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventDecision {
    Continue,
    Activate(Event),
}

pub trait EventHandler {
    fn handle(&self, request: &EventRequest) -> EventDecision;
}

#[derive(Default)]
pub struct EventManager {
    handlers: Vec<Box<dyn EventHandler>>,
    active: Option<Event>,
}

impl EventManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_handler(handler: impl EventHandler + 'static) -> Self {
        let mut manager = Self::new();
        manager.push(handler);
        manager
    }

    pub fn push(&mut self, handler: impl EventHandler + 'static) {
        self.handlers.push(Box::new(handler));
    }

    pub fn active_event(&self) -> Option<&Event> {
        self.active.as_ref()
    }

    pub fn process(&mut self, request: EventRequest) -> EventTransition {
        let selected = self.select(&request);
        let previous = self.active.clone();

        if previous == selected {
            return EventTransition {
                deactivated: None,
                activated: None,
                active: selected,
                changed: false,
            };
        }

        let deactivated = previous;
        let activated = selected.clone();
        self.active = selected.clone();
        EventTransition {
            deactivated,
            activated,
            active: selected,
            changed: true,
        }
    }

    fn select(&self, request: &EventRequest) -> Option<Event> {
        for handler in &self.handlers {
            match handler.handle(request) {
                EventDecision::Continue => continue,
                EventDecision::Activate(event) => return Some(event),
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventTransition {
    pub deactivated: Option<Event>,
    pub activated: Option<Event>,
    pub active: Option<Event>,
    pub changed: bool,
}

impl EventTransition {
    pub fn deactivated(event: Event) -> Self {
        Self {
            deactivated: Some(event),
            activated: None,
            active: None,
            changed: true,
        }
    }
}

// 기존 문서 조회 예제와 다른 장의 호환성을 위한 요청 연쇄 API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainError {
    Continue,
    Rejected,
    Unhandled,
}

pub trait Handler {
    fn handle(&self, request: &Request) -> Result<Response, ChainError>;
}

#[derive(Default)]
pub struct RequestChain {
    handlers: Vec<Box<dyn Handler>>,
}

impl RequestChain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, handler: impl Handler + 'static) {
        self.handlers.push(Box::new(handler));
    }

    pub fn handle(&self, request: &Request) -> Result<Response, ChainError> {
        for handler in &self.handlers {
            match handler.handle(request) {
                Err(ChainError::Continue) => continue,
                answer => return answer,
            }
        }
        Err(ChainError::Unhandled)
    }
}
