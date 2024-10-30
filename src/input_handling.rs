use device_query::{DeviceState, Keycode};

pub fn obtain_keypress() -> Option<Keycode> {
    let device_state = DeviceState::new();

    // loop {
    //     let keys: Vec<Keycode> = device_state.get_keys();
    //     if !keys.is_empty() {
    //         return keys.first();
    //     }
    // }

    None
}
