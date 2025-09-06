use crate::{audio, timer::{Timer, parse_duration}};
use crossterm::event::{KeyCode, KeyEvent};
use std::{collections::VecDeque, time::{Duration, Instant}};

pub enum Event<T> {
    Input(T),
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Adding,
    Editing(usize),
    Description(usize),
}

pub struct App {
    pub queue: VecDeque<Timer>,
    pub input: String,
    pub running: bool,
    pub started_at: Option<Instant>,
    pub should_quit: bool,
    pub alarm_flash: Option<Instant>,
    pub selected: usize, // idx of selected timer in queue
    pub mode: Mode,
}

impl App {
    pub fn new() -> Self {
        Self {
            selected: 0,
            mode: Mode::Normal,
            queue: VecDeque::new(),
            input: String::new(),
            running: false,
            started_at: None,
            should_quit: false,
            alarm_flash: None,
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        // Trap Description mode separately
        if let Mode::Description(idx) = self.mode {
            match key.code {
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    self.input.clear();
                }
                KeyCode::Enter => {
                    if idx < self.queue.len() {
                        self.queue[idx].description = self.input.clone();
                    }
                    self.input.clear();
                    self.mode = Mode::Normal;
                }
                KeyCode::Char(c) => self.input.push(c),
                KeyCode::Backspace => { self.input.pop(); }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Up => {
                if !self.queue.is_empty() && self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down => {
                if !self.queue.is_empty() && self.selected + 1 < self.queue.len() {
                    self.selected += 1;
                }
            }
            KeyCode::Char('e') => {
                if !self.queue.is_empty() && self.selected < self.queue.len() {
                    if !(self.selected == 0 && self.running) {
                        self.mode = Mode::Editing(self.selected);
                        self.input.clear();
                    }
                }
            }
            KeyCode::Char('r') => {
                if !self.queue.is_empty() && self.selected < self.queue.len() {
                    if self.selected == 0 && self.running {
                        // can't delete currently running timer
                    } else {
                        self.queue.remove(self.selected);
                        if self.selected >= self.queue.len() && self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                }
            }
            KeyCode::Char('d') => {
                if !self.queue.is_empty() && self.selected < self.queue.len() {
                    self.mode = Mode::Description(self.selected);
                    self.input.clear();
                }
            }
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('a') => {
                self.mode = Mode::Adding;
                self.input.clear();
            }
            KeyCode::Enter => {
                match self.mode {
                    Mode::Adding => {
                        self.add_from_input();
                        self.input.clear();
                        self.mode = Mode::Normal;
                    }
                    Mode::Editing(idx) => {
                        if !self.input.is_empty() {
                            if let Some(dur) = parse_duration(&self.input) {
                                if idx < self.queue.len() {
                                    let label = format!("{}m", dur.as_secs() / 60);
                                    let mut timer = Timer::new(label, dur);
                                    timer.description = self.queue[idx].description.clone();
                                    self.queue[idx] = timer;
                                }
                            }
                        }
                        self.input.clear();
                        self.mode = Mode::Normal;
                    }
                    Mode::Normal => {}
                    _ => {}
                }
            }
            KeyCode::Esc => {
                self.mode = Mode::Normal;
            }
            KeyCode::Char(c) if matches!(self.mode, Mode::Adding | Mode::Editing(_)) => {
                self.input.push(c);
            }
            KeyCode::Backspace if matches!(self.mode, Mode::Adding | Mode::Editing(_)) => {
                self.input.pop();
            }
            KeyCode::Char(' ') => self.toggle_run(),
            _ => {}
        }
    }

    fn add_from_input(&mut self) {
        for token in self.input.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            if let Some(dur) = parse_duration(token) {
                let label = format!("{}m", dur.as_secs() / 60);
                self.queue.push_back(Timer::new(label, dur));
            }
        }
    }

    fn toggle_run(&mut self) {
        if self.running {
            self.running = false;
            self.started_at = None;
        } else if !self.queue.is_empty() {
            if self.selected < self.queue.len() {
                // rotate so that selected becomes front
                let selected = self.selected;
                let mut rest: VecDeque<_> = self.queue.drain(selected..).collect();
                let mut front: VecDeque<_> = self.queue.drain(..selected).collect();
                rest.append(&mut front);
                self.queue = rest;
                self.selected = 0;
            }
            self.running = true;
            self.started_at = Some(Instant::now());
        }
    }

    pub fn tick(&mut self) {
        if !self.running {
            return;
        }
        if let Some(timer) = self.queue.front_mut() {
            if let Some(start) = self.started_at {
                let elapsed = start.elapsed();
                if elapsed >= timer.original {
                    self.queue.pop_front();
                    self.alarm_flash = Some(Instant::now());
                    audio::play_alarm();
                    if !self.queue.is_empty() {
                        self.started_at = Some(Instant::now());
                    } else {
                        self.running = false;
                        self.started_at = None;
                    }
                } else {
                    timer.left = timer.original.saturating_sub(elapsed);
                }
            }
        }
    }
}
