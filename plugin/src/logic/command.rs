use crate::utils::Fixed64;
use na::Vector3;

#[derive(Clone, Debug)]
pub enum Command {
    NewStage(CmdNewStage),
    NewCharacter(CmdNewCharacter),
}

#[derive(Clone, Debug)]
pub struct CmdNewStage {}

#[derive(Clone, Debug)]
pub struct CmdNewCharacter {
    pub(crate) position: Vector3<Fixed64>,
}
