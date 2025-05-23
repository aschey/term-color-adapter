use rstest::rstest;

use crate::ColorSupport;

use super::{IsTerminal, TermVar, TermVars};

#[test]
fn default_terminal() {
    let vars = TermVars::default();
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn default_no_terminal() {
    let vars = TermVars::default();
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn truecolor() {
    let mut vars = TermVars::default();
    vars.meta.colorterm = TermVar::new("24bit");
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::TrueColor, support);
}

#[test]
fn truecolor_no_term() {
    let mut vars = TermVars::default();
    vars.meta.colorterm = TermVar::new("24bit");
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn truecolor_truthy() {
    let mut vars = TermVars::default();
    vars.meta.colorterm = truthy_var();
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::TrueColor, support);
}

#[test]
fn ansi256_no_term() {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new("xterm-256color");
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn no_color() {
    let mut vars = TermVars::default();
    vars.overrides.no_color = truthy_var();
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn no_color_precedence() {
    let mut vars = TermVars::default();
    vars.overrides.no_color = truthy_var();
    vars.overrides.force_color = truthy_var();
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn force_color() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = truthy_var();
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::Ansi16, support);
}

#[test]
fn clicolor_force() {
    let mut vars = TermVars::default();
    vars.overrides.clicolor_force = truthy_var();
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::Ansi16, support);
}

#[test]
fn force_color_level_2() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = TermVar::new("2");
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::Ansi256, support);
}

#[test]
fn force_color_level_3() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = TermVar::new("3");
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::TrueColor, support);
}

#[rstest]
#[case("alacritty")]
#[case("wezterm")]
#[case("xterm-kitty")]
fn truecolor_term(#[case] term: &str) {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new(term);
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::TrueColor, support);
}

#[rstest]
#[case("xterm-256color")]
#[case("screen.xterm-256color")]
fn ansi256_term(#[case] term: &str) {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new(term);
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::Ansi256, support);
}

#[rstest]
#[case("linux")]
#[case("xterm")]
fn ansi16_term(#[case] term: &str) {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new(term);
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::Ansi16, support);
}

#[test]
fn screen() {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new("screen.xterm-256color");
    vars.meta.colorterm = TermVar::new("truecolor");
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::Ansi256, support);
}

#[test]
fn tmux_term() {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new("tmux-256color");
    vars.meta.colorterm = TermVar::new("truecolor");
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::Ansi256, support);
}

#[test]
fn tmux_term_program() {
    let mut vars = TermVars::default();
    vars.meta.term_program = TermVar::new("tmux");
    vars.meta.term = TermVar::new("xterm-256color");
    vars.meta.colorterm = TermVar::new("truecolor");
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::Ansi256, support);
}

#[test]
fn tmux_truecolor() {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new("tmux-256color");
    vars.tmux.tmux_info = "Tc: (flag) true".to_string();
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::TrueColor, support);
}

#[test]
fn apple_terminal() {
    let mut vars = TermVars::default();
    vars.meta.term_program = TermVar::new("apple_terminal");
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::Ansi256, support);
}

fn truthy_var() -> TermVar {
    TermVar::new("1")
}

struct ForceTerminal;

impl IsTerminal for ForceTerminal {
    fn is_terminal(&self) -> bool {
        true
    }
}

struct ForceNoTerminal;

impl IsTerminal for ForceNoTerminal {
    fn is_terminal(&self) -> bool {
        false
    }
}
