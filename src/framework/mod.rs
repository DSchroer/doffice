use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;
use std::sync::mpsc::{channel};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use notify::{Op, raw_watcher, RecursiveMode, Watcher};
use tiny_http::{Request, Response, Server, StatusCode};

pub trait Loader {
    type Result;
    fn load(&self) -> Result<Self::Result, Box<dyn Error>>;
}

pub trait Printer<T> {
    fn print(&self, value: T) -> Result<Vec<u8>, Box<dyn Error>>;
    fn extension() -> &'static str;
}

pub fn print_to_vec<T>(loader: impl Loader<Result=T>, printer: impl Printer<T>) -> Result<Vec<u8>, Box<dyn Error>> {
    printer.print(loader.load()?)
}

pub fn print_to_file<T>(loader: impl Loader<Result=T>, printer: impl Printer<T>, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut out_file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    let value = loader.load()?;
    let printed = printer.print(value)?;
    out_file.write_all(&printed)?;
    Ok(())
}

pub fn print_to_web<T>(loader: impl Loader<Result=T>, printer: impl Printer<T>, port: u32, paths: Vec<String>) -> Result<(), Box<dyn Error>>  {
    let addr = format!("localhost:{}", port);
    let server = Server::http(&addr).unwrap();
    println!("server listening at http://{}/", addr);

    for request in server.incoming_requests() {
        match request.url().trim_end_matches("/") {
            "/watch" => watcher(paths.clone(), request),
            "" => request.respond(render(loader.load()?, &printer)?)?,
            _ => request.respond(Response::empty(404))?,
        }
    }
    Ok(())
}

fn watcher(paths: Vec<String>, request: Request) {
    thread::spawn(move || {
        match event_stream(paths, request) {
            _ => {}
        }
    });
}

fn render<T>(value: T, printer: &impl Printer<T>) -> Result<Response<Cursor<Vec<u8>>>, Box<dyn Error>> {
    let printed = printer.print(value)?;

    let mut response = Response::from_data(printed);
    let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
    response.add_header(header);

    Ok(response)
}

fn event_stream(paths: Vec<String>, request: Request) -> Result<(), Box<dyn Error>> {
    let mut res = Response::new(StatusCode::from(200), Vec::new(), std::io::Empty::default(), None, None);
    let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/event-stream"[..]).unwrap();
    res.add_header(header);

    let http_version = request.http_version().clone();
    let mut writer = request.into_writer();

    let mut buffer: Vec<u8> = Vec::new();
    res.raw_print(&mut buffer, http_version, &[], true, None)?;

    writer.write_all(&buffer)?;
    writer.flush()?;

    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx)?;

    for path in paths {
        watcher.watch(path, RecursiveMode::NonRecursive)?;
    }

    loop {
        match rx.recv() {
            Ok(e) => {
                match e.op {
                    Ok(Op::WRITE) => {
                        let message = "event: reload\ndata:\n\n";
                        write!(writer, "{:x}\r\n", message.len())?;
                        write!(writer, "{}\r\n", message)?;
                        writer.flush()?;
                        sleep(Duration::from_secs(1));
                    }
                    _ => {}
                }
            },
            Err(e) => return Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read};
    use crate::doc::Document;
    use crate::framework::render;
    use crate::HtmlPrinter;

    #[test]
    fn render_supports_utf8() {
        let doc = Document::new("😊");
        let printer = HtmlPrinter::new(false, None);
        let response = render(doc, &printer).unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        response.into_reader().read_to_end(&mut buffer).unwrap();

        let data = String::from_utf8(buffer).unwrap();
        assert!(data.contains("😊"))
    }
}

