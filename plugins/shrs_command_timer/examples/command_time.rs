use shrs::prelude::{styled_buf::StyledBuf, *};
use shrs_command_timer::{CommandTimerPlugin, CommandTimerState};

struct MyPrompt;

impl Prompt for MyPrompt {
    fn prompt_left(&self, _line_ctx: &LineCtx) -> StyledBuf {
        styled!("> ")
    }
    fn prompt_right(&self, line_ctx: &LineCtx) -> StyledBuf {
        let time_str = line_ctx
            .ctx
            .state
            .get::<CommandTimerState>()
            .and_then(|x| x.command_time())
            .map(|x| format!("{x:?}"))
            .unwrap_or(String::new());
        styled!(time_str.reset())
    }
}

fn main() {
    let myline = LineBuilder::default()
        .with_prompt(MyPrompt)
        .build()
        .unwrap();

    let myshell = ShellBuilder::default()
        .with_plugin(CommandTimerPlugin)
        .with_readline(myline)
        .build()
        .unwrap();

    myshell.run();
}
