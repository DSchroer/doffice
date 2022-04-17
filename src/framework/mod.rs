use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Write};
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use notify::{RecursiveMode, watcher, Watcher};
use tiny_http::{Request, Response, Server, StatusCode};

pub trait Command {
    fn execute(&self, writer: &mut impl Write) -> Result<(), Box<dyn Error>>;
}

pub struct Runner<T>{
    command: T,
}

pub enum RunnerConfig<'a> {
    ToFile(String),
    ToData(&'a mut Vec<u8>),
    Watch(u32, String)
}

impl<T: Command> Runner<T> {
    pub fn new(command: T) -> Self {
        Runner{ command }
    }

    pub fn exec<'a>(&self, config: RunnerConfig<'a>) -> Result<(), Box<dyn Error>> {
        match config {
            RunnerConfig::ToFile(path) => self.exec_file(&path),
            RunnerConfig::Watch(port, path) => self.exec_watch(&port, path),
            RunnerConfig::ToData(buffer) => self.exec_str(buffer),
        }
    }

    fn exec_str(&self, buffer: &mut Vec<u8>) -> Result<(), Box<dyn Error>>{
        self.command.execute(buffer)
    }

    fn exec_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut out_file = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        self.command.execute(&mut out_file)
    }

    fn exec_watch(&self, port: &u32, path: String) -> Result<(), Box<dyn Error>>  {
        let addr = format!("localhost:{}", port);
        let server = Server::http(&addr).unwrap();
        println!("server listening at http://{}/", addr);

        for request in server.incoming_requests() {
            match request.url().trim_end_matches("/") {
                "/watch" => self.watcher(path.clone(), request),
                "" => request.respond(self.render()?)?,
                _ => request.respond(Response::empty(404))?,
            }
        }
        Ok(())
    }

    fn watcher(&self, path: String, request: Request) {
        thread::spawn(move || {
            match event_stream(path, request) {
                _ => {}
            }
        });
    }

    fn render(&self) -> Result<Response<Cursor<Vec<u8>>>, Box<dyn Error>> {
        let mut buffer = Vec::new();
        self.command.execute(&mut buffer)?;

        let mut response = Response::from_data(buffer);
        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();
        response.add_header(header);

        Ok(response)
    }
}

fn event_stream(path: String, request: Request) -> Result<(), Box<dyn Error>> {
    let mut res = Response::new(StatusCode::from(200), Vec::new(), std::io::Empty::default(), None, None);
    let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/event-stream"[..]).unwrap();
    res.add_header(header);

    let http_version = request.http_version().clone();
    let mut writer = request.into_writer();

    let mut buffer: Vec<u8> = Vec::new();
    res.raw_print(&mut buffer, http_version, &[], true, None)?;
    let http_str = String::from_utf8(buffer.clone()).unwrap();

    writer.write_all(&http_str.as_bytes())?;
    writer.flush()?;

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(0)).unwrap();
    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(_) => {
                let message = "event: reload\ndata:\n\n";
                write!(writer, "{:x}\r\n", message.len())?;
                write!(writer, "{}\r\n", message)?;
                writer.flush()?;
                sleep(Duration::from_secs(1));
            },
            Err(e) => return Err(e.into()),
        }
    }
}



