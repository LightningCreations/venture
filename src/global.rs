use bevy::{
    app::{Plugin, StateTransition, Update},
    ecs::{
        component::Component,
        schedule::{NextState, ScheduleLabel, State, States},
        system::{Res, ResMut, Resource},
    },
    input::{keyboard::KeyCode, ButtonInput},
};

#[derive(States, Default, Clone, Debug, Hash, PartialEq, Eq)]
pub enum GlobalState {
    #[default]
    EngineSplash,
    GameSplash,
    TopMenu,
    GameMenu(GameMenu),
    Running,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum GameMenu {
    SaveStates(SaveMenuMode),
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum SaveMenuMode {
    SaveFilesList,
    LoadFilesList,
    SaveQuick,
    SaveLoad,
    SaveConfirm,
    LoadConfirm,
}

fn global_reset(
    mut next_state: ResMut<NextState<GlobalState>>,
    state: Res<State<GlobalState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.all_pressed([KeyCode::AltLeft, KeyCode::ShiftRight]) && keys.just_pressed(KeyCode::KeyR)
    {
        bevy::log::info!(name: "global_reset", "Global Reset Input, entering EngineSplash from {:?}", state.get());

        next_state.set(GlobalState::EngineSplash);
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn name(&self) -> &str {
        "venture-state"
    }

    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<State<GlobalState>>()
            .init_resource::<NextState<GlobalState>>();
        app.add_systems(Update, global_reset);
    }
}
