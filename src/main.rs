use process_memory::{DataMember, Memory, TryIntoProcessHandle};
use utils::get_pid;

mod utils;

fn main() {
    let process_name = "notes";
    let handle = get_pid(&process_name).try_into_process_handle().unwrap();
    let member: DataMember<usize> = DataMember::new_offset(handle, vec![0x01]);

    println!("Current value: {}", unsafe { member.read().unwrap() });
}
