// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use deno_core::op2;
use deno_core::OpState;

use crate::BootstrapOptions;

deno_core::extension!(
  deno_bootstrap,
  ops = [
    op_bootstrap_args,
    op_bootstrap_pid,
    op_bootstrap_numcpus,
    op_bootstrap_user_agent,
    op_bootstrap_language,
    op_bootstrap_log_level,
    op_bootstrap_no_color,
  ],
);

#[op2]
#[serde]
pub fn op_bootstrap_args(state: &mut OpState) -> Vec<String> {
  state.borrow::<BootstrapOptions>().args.clone()
}

#[op2(fast)]
#[smi]
pub fn op_bootstrap_pid() -> u32 {
  std::process::id()
}

#[op2(fast)]
#[smi]
pub fn op_bootstrap_numcpus(state: &mut OpState) -> u32 {
  state.borrow::<BootstrapOptions>().cpu_count as u32
}

#[op2]
#[string]
pub fn op_bootstrap_user_agent(state: &mut OpState) -> String {
  state.borrow::<BootstrapOptions>().user_agent.clone()
}

#[op2]
#[string]
pub fn op_bootstrap_language(state: &mut OpState) -> String {
  state.borrow::<BootstrapOptions>().locale.clone()
}

#[op2(fast)]
#[smi]
pub fn op_bootstrap_log_level(state: &mut OpState) -> i32 {
  state.borrow::<BootstrapOptions>().log_level as i32
}

#[op2(fast)]
pub fn op_bootstrap_no_color(state: &mut OpState) -> bool {
  let options = state.borrow::<BootstrapOptions>();
  options.no_color || !options.is_tty
}
