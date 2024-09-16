use bevy::prelude::*;

#[derive(Resource)]
pub struct FpsCounter {
    frames: u32,
    timer: Timer,
}

pub fn fps_counter_system(
    time: Res<Time>,
    mut counter: ResMut<FpsCounter>,
) {
    counter.frames += 1;
    if counter.timer.tick(time.delta()).just_finished() {
        println!("FPS: {}", counter.frames);
        counter.frames = 0;
    }
}

pub fn setup_fps_counter(mut commands: Commands) {
    commands.insert_resource(FpsCounter {
        frames: 0,
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
}