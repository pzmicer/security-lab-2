use druid::widget::{Button, Flex, Label, LensWrap, Switch, WidgetExt};
use druid::{
    commands, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, FileDialogOptions,
    Handled, Lens, Widget, WindowDesc, Target, LocalizedString,
};

use crate::{payloads, windows};

#[derive(Clone, Data, Lens)]
pub struct State {
    pub file_path: String,
    pub should_archive: bool,
    pub should_hash: bool,
    pub should_hide: bool,
}

pub fn build_my_widget() -> impl Widget<State> {
    let choose_button = Button::new("Choose file").on_click(move |ctx, _, _| {
        ctx.submit_command(commands::SHOW_OPEN_PANEL.with(FileDialogOptions::new()))
    });

    let file_label = Label::new(|data: &State, _env: &_| data.file_path.to_owned());

    let switch1 = LensWrap::new(Switch::new(), State::should_archive).padding(5.0);
    let switch2 = LensWrap::new(Switch::new(), State::should_hash).padding(5.0);
    let switch3 = LensWrap::new(Switch::new(), State::should_hide).padding(5.0);

    let switch1_label = Label::new("Archive").padding(5.0);
    let switch2_label = Label::new("Hash").padding(5.0);
    let switch3_label = Label::new("Make Hidden").padding(5.0);

    let submit_button = Button::<State>::new("Submit").on_click(|_ctx, data, _| {
        #[cfg(debug_assertions)]
        {
            println!("Submit clicked!");
            println!("Choosen file = {:?}", &data.file_path);
        }

        payloads::do_things(&data).unwrap_or_else(|e| {
            windows::show_message(&e.to_string());
        });
    });

    Flex::column()
        .with_child(file_label)
        .with_default_spacer()
        .with_default_spacer()
        .with_child(choose_button)
        .with_spacer(30.0)
        .with_child(switch1_label)
        .with_child(switch1)
        .with_default_spacer()
        .with_child(switch2_label)
        .with_child(switch2)
        .with_default_spacer()
        .with_child(switch3_label)
        .with_child(switch3)
        .with_spacer(40.0)
        .with_child(submit_button)
        .center()
}

pub struct Delegate;

impl AppDelegate<State> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut State,
        _env: &Env,
    ) -> Handled {
        if let Some(file) = cmd.get(commands::OPEN_FILE) {
            data.file_path = file.path().to_str().unwrap().to_string();
            return Handled::Yes;
        }
        Handled::No
    }
}

pub fn run() {
    let window = WindowDesc::new(|| build_my_widget())
        .window_size((150.0, 500.0))
        .title(LocalizedString::new("title").with_placeholder("Security"));
    AppLauncher::with_window(window)
        .delegate(Delegate)
        // .use_simple_logger()
        .launch(State {
            file_path: String::from("file_path"),
            should_archive: false,
            should_hash: false,
            should_hide: false,
        })
        .expect("launch failed");
}
