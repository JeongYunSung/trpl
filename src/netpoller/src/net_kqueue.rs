use std::io::Result;

pub fn watch_pid(pid: libc::pid_t) -> Result<()> {
    let mut watcher = kqueue::Watcher::new()?;

    watcher.add_pid(
        pid,
        kqueue::EventFilter::EVFILT_PROC,
        kqueue::FilterFlag::NOTE_EXIT,
    )?;

    watcher.watch();

    println!("Watching for events, press Ctrl+C to stop...");
    for ev in watcher.iter() {
        println!("{:?}", ev);
    }

    Ok(())
}
