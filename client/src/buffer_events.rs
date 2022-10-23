// fn buffer_events(all_events: EventReader<everything>) {
//     for event in all_events.iter() {
//         add_to_buffer(event, current_tick)
//     }
// }

// fn reemit_since_tick(tick: u32) {
//     for each key after tick {
//         for each event in buffer[key] {
//             emit event
//         }
//     }

//     clear_buffer_up_to_tick(tick)
// }

// [0]: [e1, e2, e3]
// [1]: [...]
// [2 SERVER]: [...]
// [3]: [...]
// [4]: [...]
// [5]