use std::collections::VecDeque;

use crate::anim::{Animation, AnimationDuration, Frame};

#[derive(Debug)]
pub struct Animator<'a> {
    state: AnimatorState,
    animation_player: AnimationPlayer<'a>,
    current_animation: Animation<'a>,
}

#[derive(Debug)]
pub enum AnimatorState {
    OneShot,
    Loop,
}

impl<'a> Animator<'a> {
    pub fn new(state: AnimatorState, animation: Animation<'a>) -> Self {
        let anim = animation.frames().iter().cloned().next().unwrap();
        let mut animation_player = AnimationPlayer::new(anim);
        for frame in animation.frames().iter().cloned() {
            animation_player.frame_queue.push_back(frame);
        }
        Animator {
            state,
            animation_player,
            current_animation: animation,
        }
    }
    pub fn current_frame(&self) -> &Frame<'a> {
        &self.animation_player.current_frame
    }
    pub fn elapsed_duration(&mut self, duration: AnimationDuration) -> AnimatorUpdateResponse<'a> {
        match self.animation_player.elapsed_duration(duration) {
            AnimationPlayerUpdateResponse::UnchangedAnimation => {
                AnimatorUpdateResponse::UnchangedAnimation
            }
            AnimationPlayerUpdateResponse::NextFrame(next_frame) => {
                AnimatorUpdateResponse::NextFrame(next_frame)
            }
            AnimationPlayerUpdateResponse::End => match self.state {
                AnimatorState::OneShot => AnimatorUpdateResponse::UnchangedAnimation,
                AnimatorState::Loop => {
                    self.animation_player
                        .set_current_animation(&self.current_animation);
                    AnimatorUpdateResponse::NextFrame(
                        self.current_animation.frames().get(0).unwrap().clone(),
                    )
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum AnimatorUpdateResponse<'a> {
    UnchangedAnimation,
    NextFrame(Frame<'a>),
}

#[derive(Debug)]
struct AnimationPlayer<'a> {
    current_frame: Frame<'a>,
    frame_queue: VecDeque<Frame<'a>>,
    current_duration: AnimationDuration,
}

impl<'a> AnimationPlayer<'a> {
    fn new(current_frame: Frame<'a>) -> Self {
        let dur = current_frame.duration();
        AnimationPlayer {
            current_frame,
            frame_queue: VecDeque::new(),
            current_duration: dur,
        }
    }
    fn current_animation(&self) -> &Frame<'a> {
        &self.current_frame
    }
    fn set_current_animation(&mut self, animation: &Animation<'a>) {
        for frame in animation.frames().iter().cloned() {
            self.frame_queue.push_back(frame);
        }
        self.current_frame = self.frame_queue.pop_front().unwrap();
    }
    fn elapsed_duration(
        &mut self,
        duration: AnimationDuration,
    ) -> AnimationPlayerUpdateResponse<'a> {
        self.current_duration += duration;
        let dur = self.current_frame.duration();
        if self.current_duration > dur {
            self.current_duration -= dur;
            if let Some(next_animation) = self.frame_queue.pop_front() {
                self.current_frame = next_animation;
                AnimationPlayerUpdateResponse::NextFrame(self.current_frame.clone())
            } else {
                AnimationPlayerUpdateResponse::End
            }
        } else {
            AnimationPlayerUpdateResponse::UnchangedAnimation
        }
    }
}

#[derive(Debug)]
enum AnimationPlayerUpdateResponse<'a> {
    UnchangedAnimation,
    NextFrame(Frame<'a>),
    End,
}
