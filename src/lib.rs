pub mod template;

// Use this file to add helper functions and additional modules.

use glam::IVec2;
use grid::Grid;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fmt::Debug, str::FromStr};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;

pub trait IsNumeric {}

impl IsNumeric for u8 {}
impl IsNumeric for u32 {}
impl IsNumeric for u64 {}
impl IsNumeric for usize {}
impl IsNumeric for i8 {}
impl IsNumeric for i32 {}
impl IsNumeric for i64 {}
impl IsNumeric for isize {}
impl IsNumeric for f32 {}
impl IsNumeric for f64 {}

pub trait AocGrid<T> {
    fn from_input(input: &str) -> Self;

    fn get_ivec(&self, pos: IVec2) -> Option<&T>;
}

impl<T> AocGrid<T> for Grid<T>
where
    T: Default + IsNumeric + FromStr + 'static,
    <T as FromStr>::Err: Debug,
{
    fn get_ivec(&self, pos: IVec2) -> Option<&T> {
        self.get(usize::try_from(pos.y).ok()?, usize::try_from(pos.x).ok()?)
    }

    fn from_input(input: &str) -> Self {
        let v = input
            .chars()
            .filter(|c| c != &'\n')
            .map(|c| c.to_string().parse::<T>().unwrap())
            .collect();

        let width = input.lines().next().map(|l| l.chars().count()).unwrap();
        Grid::from_vec(v, width)
    }
}

impl AocGrid<char> for Grid<char> {
    fn get_ivec(&self, pos: IVec2) -> Option<&char> {
        self.get(usize::try_from(pos.y).ok()?, usize::try_from(pos.x).ok()?)
    }

    fn from_input(input: &str) -> Self {
        let v = input.chars().filter(|c| c != &'\n').collect();
        let width = input.lines().next().map(|l| l.chars().count()).unwrap();
        Grid::from_vec(v, width)
    }
}

pub fn tracing_init(level: impl Into<LevelFilter>) {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(level)
        .without_time()
        .with_target(false)
        .with_span_events(FmtSpan::ENTER)
        .finish();
    tracing::subscriber::set_global_default(subscriber).ok();
}

pub fn progressbar_init(total_iterations: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(total_iterations);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    progress_bar
}
