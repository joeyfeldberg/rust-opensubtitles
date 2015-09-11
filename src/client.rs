use xmlrpc::{Client, Request};

const OPENSUBTITLES_SERVER: &'static str = "http://api.opensubtitles.org/xml-rpc";

#[derive(PartialEq, Debug)]
pub enum ClientError {
  InvalidResponse,
  InvalidCredentials,
  NoSubtitlesFound,
}

pub struct OpenSubtitlesClient {
  token: String,
  client: Client,
}

#[allow(non_snake_case)]
#[derive(RustcEncodable)]
struct SubtitlesQuery {
  sublanguageid: String,
  moviehash: String,
  moviebytesize: String,
}

#[derive(RustcDecodable)]
struct TokenResponse {
  token: String,
  status: String,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, Debug)]
pub struct SubtitleSearchResponse {
  pub IDSubMovieFile: String,
  pub ZipDownloadLink: String,
}

#[derive(RustcDecodable)]
struct SubtitleSearchResponseWrapper {
  status: String,
  data: Vec<SubtitleSearchResponse>,
}

macro_rules! prase_response {
    ($response:expr) => {
      match $response {
        Ok(mut list) => list.pop().unwrap(),
        Err(_) => return Err(ClientError::InvalidResponse)
      }
    };
}

impl OpenSubtitlesClient {
  pub fn create_client(username: &str, password: &str, lang: &str, useragent: &str)
                        -> Result<OpenSubtitlesClient, ClientError> {
    let client = Client::new(OPENSUBTITLES_SERVER);
    let mut request = Request::new("LogIn");
    request = request.argument(&username).argument(&password)
                     .argument(&lang).argument(&useragent).finalize();

    let response = client.remote_call(&request).unwrap().result::<TokenResponse>();

    let res : TokenResponse = prase_response!(response);
    if res.status.starts_with("200") {
        Ok(OpenSubtitlesClient { token: res.token, client: client })
    } else {
        Err(ClientError::InvalidCredentials)
    }
  }

  pub fn search_subtitles(&self, hash: &str, size: u64, lang: &str)
                          -> Result<Vec<SubtitleSearchResponse>, ClientError> {
    let mut request = Request::new("SearchSubtitles");
    let size_str = size.to_string();
    let query = SubtitlesQuery{ sublanguageid: lang.into(), moviehash: hash.into(), moviebytesize: size_str };
    request = request.argument(&self.token).argument(&[query]).finalize();
    let response = self.client.remote_call(&request).unwrap().result::<SubtitleSearchResponseWrapper>();

    let res : SubtitleSearchResponseWrapper = prase_response!(response);
    if res.status.starts_with("200") {
        Ok(res.data)
    } else {
        Err(ClientError::NoSubtitlesFound)
    }
  }
}

#[test]
fn test_bad_login() {
  let res = OpenSubtitlesClient::create_client("fakeuser", "fakepassword", "fakelang", "qwe");
  assert_eq!(res.err().unwrap(), ClientError::InvalidCredentials);
}

#[test]
fn test_good_login() {
  let res = OpenSubtitlesClient::create_client("", "", "eng", "OSTestUserAgent");
  assert!(res.is_ok());
}
