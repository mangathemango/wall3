use crate::control::actions::Action;
use core::fmt;
use std::collections::VecDeque;
use std::{fmt::Display, time::Duration};

#[derive(Default)]
pub struct Sequence {
    pub name: String,
    pub action_queue: VecDeque<Box<dyn Action>>,
    pub current_action: Option<Box<dyn Action>>,
}

impl Sequence {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn enqueue<A>(&mut self, action: A) -> &mut Self
    where
        A: Action + 'static,
    {
        self.action_queue.push_back(Box::new(action));
        self
    }

    pub fn then<A>(mut self, action: A) -> Self
    where
        A: Action + 'static,
    {
        self.action_queue.push_back(Box::new(action));
        self
    }

    fn current_action_string(&self) -> String {
        if let Some(action) = self.current_action.as_ref() {
            format!("{}", action)
        } else {
            "Empty".into()
        }
    }

    pub fn abort(&mut self) {
        if let Some(mut action) = self.current_action.take() {
            action.stop();
            action.abort();
        }
        self.action_queue.clear();
        self.current_action = None;
    }
}

impl Action for Sequence {
    fn start(&mut self) {}

    fn update(&mut self, dt: Duration) {
        match &mut self.current_action {
            None => {
                self.current_action = self.action_queue.pop_front();

                if let Some(action) = &mut self.current_action {
                    action.start();
                }
            }
            Some(action) => {
                action.update(dt);
                if action.is_finished() {
                    action.stop();
                    self.current_action = None;
                }
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.action_queue.is_empty() && self.current_action.is_none()
    }

    fn stop(&mut self) {
        if let Some(mut action) = self.current_action.take() {
            action.stop();
        }
        self.action_queue.clear();
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: \n{}", self.name, self.current_action_string())
    }
}

pub struct RuntimeSequence {
    generator: Box<dyn Fn() -> Sequence + Send + Sync>,
    generated: Sequence,
}

impl RuntimeSequence {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> Sequence + Send + Sync + 'static,
    {
        Self {
            generator: Box::new(f),
            generated: Sequence::new("Unknown generated function"),
        }
    }
}

impl Action for RuntimeSequence {
    fn start(&mut self) {
        self.generated = (self.generator)()
    }

    fn update(&mut self, dt: Duration) {
        self.generated.update(dt);
    }

    fn stop(&mut self) {
        self.generated.stop();
    }

    fn is_finished(&self) -> bool {
        self.generated.is_finished()
    }
}

impl Display for RuntimeSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Runtime Generated) {}", self.generated)
    }
}

pub struct OneShot {
    f: Box<dyn FnMut() + Send + Sync>,
    elapsed: Duration,
}

impl OneShot {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut() + 'static + Send + Sync,
    {
        Self {
            f: Box::new(f),
            elapsed: Duration::ZERO,
        }
    }
}

impl Action for OneShot {
    fn start(&mut self) {
        (self.f)()
    }

    fn update(&mut self, dt: Duration) {
        self.elapsed += dt
    }

    fn is_finished(&self) -> bool {
        self.elapsed > Duration::from_millis(100)
    }
}

impl Display for OneShot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Instant")
    }
}

#[derive(Debug, Default)]
pub struct WaitFor {
    wait_duration: Duration,
    elapsed: Duration,
}

impl WaitFor {
    pub fn new(wait_duration: Duration) -> Self {
        Self {
            wait_duration,
            elapsed: Duration::ZERO,
        }
    }
}

impl Action for WaitFor {
    fn update(&mut self, dt: Duration) {
        self.elapsed += dt
    }

    fn is_finished(&self) -> bool {
        self.elapsed > self.wait_duration
    }
}

impl Display for WaitFor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Waiting for {:?} / {:?}",
            self.elapsed, self.wait_duration
        )
    }
}

pub struct WaitUntil {
    condition: Box<dyn Fn() -> bool + Send + Sync>,
    name: String,
    true_time: Duration,
    timer: Duration,
}

impl WaitUntil {
    pub fn new<F>(condition_name: &str, true_time: Duration, condition: F) -> Self
    where
        F: Fn() -> bool + 'static + Send + Sync,
    {
        Self {
            name: condition_name.into(),
            true_time,
            condition: Box::new(condition),
            timer: Duration::ZERO,
        }
    }
}

impl Action for WaitUntil {
    fn update(&mut self, dt: Duration) {
        if (self.condition)() {
            self.timer += dt
        } else {
            self.timer = Duration::ZERO
        }
    }
    fn is_finished(&self) -> bool {
        self.timer > self.true_time
    }
}

impl Display for WaitUntil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Waiting until {}...\n{:.3?} / {:.3?}",
            self.name, self.timer, self.true_time
        )
    }
}

#[derive(Default)]
pub struct ParallelSequence {
    actions: Vec<Box<dyn Action>>,
    name: String
}

impl ParallelSequence {
    pub fn new<A>(name: &str) -> Self
    where
        A: Action + 'static,
    {
        Self {
            actions: Vec::new(),
            name: name.to_string()
        }
    }
    
    pub fn push<A>(&mut self, action: A)
    where
        A: Action + 'static,
    {
        self.actions.push(Box::new(action));
    }
}

impl Action for ParallelSequence {
    fn start(&mut self) {
        self.actions.iter_mut().for_each(|action|
            action.start()
        );  
    }

    fn update(&mut self, dt: Duration) {
        self.actions.iter_mut().for_each(|action|
            action.update(dt)
        );  
    }
    
    fn is_finished(&self) -> bool {
        self.actions.iter().all(|action|
            action.is_finished()
        )
    }

    fn stop(&mut self) {
        self.actions.iter_mut().for_each(|action|
            action.stop()
        );  
    }

    fn abort(&mut self) {
        self.actions.iter_mut().for_each(|action|
            action.abort()
        );  
    }
}

impl Display for ParallelSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parallel Sequence: {}", 
            self.actions.iter().fold("".to_string(), |acc, action| acc + format!("{}\n", action).as_str())
        )
    }
}