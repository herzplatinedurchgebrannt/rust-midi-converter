
use midly::{Smf, TrackEvent, TrackEventKind, MidiMessage, num::u7};

struct ChangeNote {
    note_in: u7,
    note_out: u7,
    name: String
}

fn main() {

    println!("Welcome to midi-converter...");
    println!("Choose function or --help");

    // dictionary in / out
    let mut vec: Vec<ChangeNote> = Vec::new();

    vec.push(ChangeNote{ note_in: 36.into(), note_out: 35.into(), name: String::from("Kick")});
    vec.push(ChangeNote{ note_in: 49.into(), note_out: 38.into(), name: String::from("Snare_1")});
    vec.push(ChangeNote{ note_in: 42.into(), note_out: 43.into(), name: String::from("Snare_2")});
    // vec.push(ChangeNote{ note_in: 38.into(), note_out: 38.into(), name: String::from("Crash_1")});
    // vec.push(ChangeNote{ note_in: 38.into(), note_out: 38.into(), name: String::from("Crash_2")});
    // vec.push(ChangeNote{ note_in: 38.into(), note_out: 38.into(), name: String::from("Tom_1")});
    // vec.push(ChangeNote{ note_in: 38.into(), note_out: 38.into(), name: String::from("Tom_2")});
    // vec.push(ChangeNote{ note_in: 38.into(), note_out: 38.into(), name: String::from("Tom_3")});
    // vec.push(ChangeNote{ note_in: 38.into(), note_out: 38.into(), name: String::from("Tom_4")});
    // vec.push(ChangeNote{ note_in: 38.into(), note_out: 38.into(), name: String::from("Hihat_1")});
    // vec.push(ChangeNote{ note_in: 38.into(), note_out: 38.into(), name: String::from("Hihat_2")});
    // vec.push(ChangeNote{ note_in: 38.into(), note_out: 38.into(), name: String::from("Hihat_3")});
    // vec.push(ChangeNote{ note_in: 38.into(), note_out: 38.into(), name: String::from("Hihat_4")});
    // vec.push(ChangeNote{ note_in: 38.into(), note_out: 38.into(), name: String::from("Tom_4")});

    // read midi file
    let mut smf = Smf::parse(include_bytes!("../files/Matt-Song1-Pattern01.mid")).unwrap();
    
    for element in &mut smf.tracks[0].iter_mut(){

        match element.kind {
            TrackEventKind::Midi { channel: _, message: MidiMessage::NoteOn { key, vel } } => { 
                
                for change_midi in vec.iter(){

                    if key == change_midi.note_in{
                        *element = TrackEvent{delta: element.delta, kind: TrackEventKind::Midi { channel: 1.into(), message: MidiMessage:: NoteOn { key: change_midi.note_out, vel: vel }}};
                        println!("Changed {} from {} to {}", change_midi.name, change_midi.note_in, change_midi.note_out);
                    }
                }            
            },
            _ => println!("no midi"),
        }
    }
    // iterate all tracks. drum midi file should only have 1 track
    for (i, track) in smf.tracks.iter().enumerate() {
        println!("track {} has {} events", i, track.len());

        // iterate all events of track
        for (j, event) in smf.tracks[i].iter().enumerate(){
            println!("delta: {}, kind: {:?}", event.delta, event.kind)
    }

    match smf.save("Matt-Song1-Pattern01.mid") {
        Ok(_) => { println!("nice!")},
        Err(_) => { println!("not so nice :-(!")},
    };

    println!("Done.");
}
}

