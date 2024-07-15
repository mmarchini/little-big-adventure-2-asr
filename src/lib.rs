#![no_std]

mod lba2;
mod splits;
mod debug;

use asr::{future::next_tick, Process};
use asr::watcher::Watcher;
use asr::timer;

use splits::Splits;

asr::async_main!(stable);
asr::panic_handler!();

macro_rules! update_game_var {
    ( $var: ident, $new_val: expr ) => {
        match $var.update(Some($new_val)) {
            Some(pair) => {
                if pair.changed() {
                    debug_log!("{}: {} ->  {}", stringify!($var), pair.old, pair.current);
                };
                pair
            },
            None => {
                asr::print_message("Unexpected Watcher failed update");
                next_tick().await;
                continue;
            },
        }
    }
}

macro_rules! read_game_memory {
    ( $process: ident, $addr: expr, $T: ident ) => {
        match $process.read::<$T>($addr) {
            Ok(state) => state,
            Err(_) => {
                asr::print_message("Could not read memory during loop");
                next_tick().await;
                continue;
            }
        }
    }
}

async fn main() {
    // TODO: Set up some general state and settings.

    asr::print_message("Hello, World!");

    loop {
        let process = Process::wait_attach("dosbox").await;
        process
            .until_closes(async {
                let mut maybe_base: Option<asr::Address> = None;

                while maybe_base.is_none() {
                    asr::print_message("Trying to find signature across all memory maps");
                    for range in process.memory_ranges() {
                        if let (Ok(address), Ok(size)) = (range.address(), range.size()) {
                            let maybe_signature = lba2::SIGNATURE.scan_process_range(&process, (address, size));

                            if maybe_signature.is_some() {
                                maybe_base = maybe_signature;
                                break;
                            }
                        }
                        next_tick().await;
                    }
                    if maybe_base.is_none() {
                        next_tick().await;
                    }
                }
                asr::print_message("Signature found");
                // // The offsets are actually negative because Address doesn't allow subtraction
                let inventory_addr = maybe_base.unwrap() + lba2::STRING_TO_INVENTORY_OFFSET;
                debug_log!("ListVarGame addr: {}", inventory_addr);

                let the_end_addr = inventory_addr + ((2 * lba2::FLAG_THE_END));
                debug_log!("The End addr: {}", the_end_addr);

                let num_cube_addr = maybe_base.unwrap() + lba2::STRING_TO_NUM_CUBE_OFFSET;
                debug_log!("NumCube addr: {}", num_cube_addr);

                let mut the_end_watch = Watcher::<i16>::new();
                let mut cube_watch = Watcher::<i32>::new();

                let mut current_split = splits::Splits::NotStarted;
                timer::set_variable("current_split", "Not Started");
                loop {
                    let cube = update_game_var!(cube_watch, read_game_memory!(process, num_cube_addr, i32));
                    let the_end = update_game_var!(the_end_watch, read_game_memory!(process, the_end_addr, i16));

                    match current_split {
                        Splits::NotStarted => {
                            if cube.old == -1 && cube.current == 0 {
                                timer::reset();
                                timer::start();
                                timer::set_variable("current_split", "Twinsun");
                                current_split = Splits::Twinsun;
                            }
                        },
                        Splits::Twinsun => {
                            if cube.check(|val| -> bool {
                                *val == lba2::EMERALD_MOON_NEAR_CIRCLE_ENTRANCE
                            }) {
                                timer::split();
                                timer::set_variable("current_split", "Moon");
                                current_split = Splits::Moon;
                            }
                        },
                        Splits::Moon => {
                            if cube.check(|val| -> bool {
                                *val == lba2::OTRINGAL_CRASH_SITE
                            }) {
                                timer::split();
                                timer::set_variable("current_split", "Otringal");
                                current_split = Splits::Otringal;
                            }
                        },
                        Splits::Otringal => {
                            if cube.check(|val| -> bool {
                                *val == lba2::FRANCOS_VILLAGE
                            }) {
                                timer::split();
                                timer::set_variable("current_split", "Francos");
                                current_split = Splits::Francos;
                            }
                        },
                        Splits::Francos => {
                            if cube.check(|val| -> bool {
                                *val == lba2::UNDERGAS_ELEVATOR_OUTSIDE
                            }) {
                                timer::split();
                                timer::set_variable("current_split", "Wannies");
                                current_split = Splits::Wannies;
                            }
                        },
                        Splits::Wannies => {
                            if cube.check(|val| -> bool {
                                *val == lba2::MOSQUIBEES_FERRY
                            }) {
                                timer::split();
                                timer::set_variable("current_split", "Mosquibees");
                                current_split = Splits::Mosquibees;
                            }
                        },
                        Splits::Mosquibees => {
                            if cube.check(|val| -> bool {
                                *val == lba2::CX_OUTSIDE
                            }) {
                                timer::split();
                                timer::set_variable("current_split", "Island CX");
                                current_split = Splits::CX;
                            }
                        },
                        Splits::CX => {
                            if cube.check(|val| -> bool {
                                *val == lba2::OTRINGAL_OUTSIDE_PALACE_SHUTTLE
                            }) {
                                timer::split();
                                timer::set_variable("current_split", "Palace");
                                current_split = Splits::Palace;
                            }
                        },
                        Splits::Palace => {
                            if cube.check(|val| -> bool {
                                *val == lba2::DARK_MONK_STATUE_FIRST
                            }) {
                                timer::split();
                                timer::set_variable("current_split", "Dark Monk");
                                current_split = Splits::DarkMonk;
                            }
                        },
                        Splits::DarkMonk => {
                            if the_end.check(|val| -> bool {
                                *val == 1
                            }) {
                                timer::split();
                                timer::set_variable("current_split", "Finished");
                                current_split = Splits::NotStarted;
                            }
                        },
                    }
                    next_tick().await;
                }
            })
            .await;
    }
}
