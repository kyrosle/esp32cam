use anyhow::Result;
use embedded_svc::{
  http::{client::Client as HttpClient, Method, Status},
  io::Write,
  utils::io,
};
use esp_idf_svc::http::client::EspHttpConnection;

/// Send a HTTP GET request.
#[allow(unused)]
pub fn get_request(client: &mut HttpClient<EspHttpConnection>) -> Result<()> {
  // Prepare headers and URL
  let headers = [("accept", "text/plain"), ("connection", "close")];
  let url = "http://ifconfig.net/";

  // Send request
  //
  // Note: If you don't want to pass in any headers, you can also use `client.get(url, headers)`.
  let request = client.request(Method::Get, url, &headers)?;
  println!("-> GET {}", url);
  let mut response = request.submit()?;

  // Process response
  let status = response.status();
  println!("<- {}", status);
  println!();
  let (_headers, mut body) = response.split();
  let mut buf = [0u8; 1024];
  let bytes_read = io::try_read_full(&mut body, &mut buf).map_err(|e| e.0)?;
  println!("Read {} bytes", bytes_read);
  match std::str::from_utf8(&buf[0..bytes_read]) {
    Ok(body_string) => println!(
      "Response body (truncated to {} bytes): {:?}",
      buf.len(),
      body_string
    ),
    Err(e) => eprintln!("Error decoding response body: {}", e),
  };

  // Drain the remaining response bytes
  while body.read(&mut buf)? > 0 {}

  Ok(())
}

/// Send a HTTP POST request.
#[allow(unused)]
pub fn post_request(client: &mut HttpClient<EspHttpConnection>) -> Result<()> {
  // Prepare payload
  let payload = b"Hello world!";

  // Prepare headers and URL
  let content_length_header = format!("{}", payload.len());
  let headers = [
    ("accept", "text/plain"),
    ("content-type", "text/plain"),
    ("connection", "close"),
    ("content-length", &*content_length_header),
  ];
  let url = "http://example.org/";

  // Send request
  let mut request = client.post(url, &headers)?;
  request.write_all(payload)?;
  request.flush()?;
  println!("-> POST {}", url);
  let mut response = request.submit()?;

  // Process response
  let status = response.status();
  println!("<- {}", status);
  println!();
  let (_headers, mut body) = response.split();
  let mut buf = [0u8; 1024];
  let bytes_read = io::try_read_full(&mut body, &mut buf).map_err(|e| e.0)?;
  println!("Read {} bytes", bytes_read);
  match std::str::from_utf8(&buf[0..bytes_read]) {
    Ok(body_string) => println!(
      "Response body (truncated to {} bytes): {:?}",
      buf.len(),
      body_string
    ),
    Err(e) => eprintln!("Error decoding response body: {}", e),
  };

  // Drain the remaining response bytes
  while body.read(&mut buf)? > 0 {}

  Ok(())
}
