use crate::control::actions::general::Sequence;
use crate::control::states::odometry::OdometryState;
use crate::devices::gyro::state::GyroState;
use crate::devices::maixcam::state::MaixcamState;
use crate::devices::qr::state::QrState;
use crate::devices::stm32::controller::Stm32Controller;
use crate::devices::stm32::state::Stm32State;
use arc_swap::{ArcSwap, Guard};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

#[derive(Default)]
pub struct Robot {
    // Device states
    gyro_state: ArcSwap<GyroState>,
    stm32_state: ArcSwap<Stm32State>,
    maixcam_state: ArcSwap<MaixcamState>,
    qr_state: Arc<Mutex<QrState>>,

    // Control states
    odometry_state: Arc<Mutex<OdometryState>>,
    action_queue: Arc<Mutex<Sequence>>,

    stm32_controller: OnceLock<Stm32Controller>,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            gyro_state: ArcSwap::from_pointee(GyroState::new()),
            stm32_state: ArcSwap::from_pointee(Stm32State::new()),
            maixcam_state: ArcSwap::from_pointee(MaixcamState::new()),
            qr_state: Arc::new(Mutex::new(QrState::new())),
            odometry_state: Arc::new(Mutex::new(OdometryState::new())),
            action_queue: Arc::new(Mutex::new(Sequence::new("Action Queue"))),

            stm32_controller: OnceLock::new(),
        }
    }

    // READ METHODS

    pub fn gyro_state(&self) -> Guard<Arc<GyroState>> {
        self.gyro_state.load()
    }

    pub fn stm32_state(&self) -> Guard<Arc<Stm32State>> {
        self.stm32_state.load()
    }

    pub fn maixcam_state(&self) -> Guard<Arc<MaixcamState>> {
        self.maixcam_state.load()
    }

    pub fn qr_state(&self) -> QrState {
        self.qr_state.lock().unwrap().clone()
    }

    pub fn odometry_state(&self) -> OdometryState {
        self.odometry_state.lock().unwrap().clone()
    }

    pub fn stm32_controller(&self) -> Stm32Controller {
        self.stm32_controller
            .get()
            .expect("STM32 controller not initialized")
            .clone()
    }


    // WRITE METHODS

    pub fn set_gyro_state(&self, state: GyroState) {
        self.gyro_state.store(Arc::new(state));
    }


    pub fn set_stm32_state(&self, state: Stm32State) {
        self.stm32_state.store(Arc::new(state));
    }


    pub fn set_maixcam_state(&self, state: MaixcamState) {
        self.maixcam_state.store(Arc::new(state));
    }

    pub fn set_stm32_controller(&self, controller: Stm32Controller) {
        self.stm32_controller.set(controller).unwrap()
    }

    pub fn qr_state_mut(&self) -> MutexGuard<'_, QrState> {
        self.qr_state.lock().unwrap()
    }

    pub fn odometry_state_mut(&self) -> MutexGuard<'_, OdometryState> {
        self.odometry_state.lock().unwrap()
    }

    pub fn action_queue_mut(&self) -> MutexGuard<'_, Sequence> {
        self.action_queue.lock().unwrap()
    }

}
