
use std::
{
    path::{Path, PathBuf},
    io,
    fs::*,
    time::*,
    sync::mpsc
};
use anyhow::bail;
use notify::Watcher;

// ------------------------------------------------------------

enum FileEvent
{
    Write,
    Remove,
    Rename(PathBuf)
}

// ------------------------------------------------------------

struct FileWatcher
{
    filepath: PathBuf,
    #[allow(dead_code)]
    watcher: notify::RecommendedWatcher,
    receiver: mpsc::Receiver<notify::DebouncedEvent>,
}

impl FileWatcher
{
    fn watch
    (
        path: impl AsRef<Path>,
        delay: Duration
    ) -> anyhow::Result<Self>
    {
        let path = path.as_ref().to_path_buf();
        if !path.metadata()?.is_file()
        {
            bail!(format!("{path:?} is not a filepath"))
        }
        let (sender, receiver) = mpsc::channel();
        let mut watcher = notify::watcher(sender, delay)?;
        watcher.watch(&path, notify::RecursiveMode::NonRecursive)?;
        Ok(Self{filepath: path, watcher, receiver})
    }

    fn filepath(&self) -> &Path
    {
        &self.filepath
    }

    fn event(&self) -> anyhow::Result<Option<FileEvent>>
    {
        match self.receiver.try_recv()
        {
            Ok(event) => match event
            {
                notify::DebouncedEvent::NoticeWrite(_) => Ok(None),
                notify::DebouncedEvent::NoticeRemove(_) => Ok(None),
                notify::DebouncedEvent::Create(_) => unreachable!(),
                notify::DebouncedEvent::Write(_) => Ok(Some(FileEvent::Write)),
                notify::DebouncedEvent::Chmod(_) => Ok(None),
                notify::DebouncedEvent::Remove(_) => Ok(Some(FileEvent::Remove)),
                notify::DebouncedEvent::Rename(_, destination)
                    => Ok(Some(FileEvent::Rename(destination))),
                notify::DebouncedEvent::Rescan => unreachable!(),
                notify::DebouncedEvent::Error(error, _)
                    => match error
                {
                    notify::Error::Generic(description)
                        => bail!(description.to_string()),
                    notify::Error::Io(io)
                        => Err(io.into()),
                    _ => unreachable!()
                }
            }
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(disconnected) => Err(disconnected.into())
        }
    }
}

// ------------------------------------------------------------

pub struct CodeWatcher(FileWatcher);

impl CodeWatcher
{
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self>
    {
        FileWatcher::watch
        (
            &path,
            Duration::from_millis(10)
        ).map(Self)
        .map_err(Into::into)
    }

    pub fn filepath(&self) -> &Path
    {
        self.0.filepath()
    }

    pub fn refresh(&self) -> anyhow::Result<bool>
    {
        match self.0.event()?
        {
            Some(FileEvent::Write) =>
                Ok(true),
            Some(FileEvent::Remove) =>
                unimplemented!(),
            Some(FileEvent::Rename(_path)) =>
                unimplemented!(),
            None => Ok(false)
        }
    }

    pub fn code(&self) -> io::Result<String>
    {
        read_to_string(self.0.filepath())
    }
}

