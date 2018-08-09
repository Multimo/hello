// TODO: handle following cases
// handle duplicate 400
// api throttleing
// success http 201

// help links:
// https://gist.github.com/lebedov/8c3f33ebb55a67b732c1
// https://developer.linkedin.com/docs/oauth2

// step 1
// LINKEDIN_CLIENT_ID=XXXX
// LINKEDIN_CLIENT_SECRET=XXXX

// step 2 get auth code (clientId) ->  authCode
// https://www.linkedin.com/oauth/v2/authorization?
// response_type=code always code
// &client_id= 
// &redirect_uri=https%3A%2F%2Fwww.example.com%2Fauth%2Flinkedin // redirect url
// &state=987654321 // randomly generated string
// &scope=r_basicprofile



// step 3 exchange code for token (clientId, secret, authCode,) -> token
// generate url and get uset to enable app. returns with token
// grant_type=authorization_code
// &code=987654321
// &redirect_uri=https%3A%2F%2Fwww.myapp.com%2Fauth%2Flinkedin
// &client_id=
// &client_secret=
// LINKEDIN_ACCESS_CODE=XXXX

// step 3 
// LINKEDIN_ACCESS_TOKEN=XXXX


pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub client_code: Option<String>,
    pub client_token: Option<String>,
}

pub struct Client {
    credentials: Credentials,
}

impl Client {
    pub fn new(credentials: Credentials) -> Client {
        Client { 
          client_id: &credentials.client_id,  
          client_secret: &credentials.client_secret,
        }
    }

    pub fn generateAuthCode(&self) -> String {
      let state = "987654321";

      let linkedinUrl = format!(
        "https://www.linkedin.com/oauth/v2/authorization
         ?response_type=code
         &client_id={}
         &redirect_uri=http://localhost:4567/linkedin/callback
         &state={}
         &scope=r_basicprofile",
         &self.credentials.client_id,
         &state, 
      );

        println!("Go to the following URL, sign in and allow the app permission to your linkedin account:");
        println!("{}", egg_mode::authorize_url(&request_token));

      // start server on http://localhost:4567/linkedin/callback to listen for callback

    // GET https://www.linkedin.com/oauth/v2/authorization?
    // open headless browser and generate auth code
    // get code from localhost callback url
    // or
    // make sure go

    // then step 3
    }

    pub fn exchangeAuthCodeForToken(clientId, secret, code) -> String {
      // POST https://www.linkedin.com/oauth/v2/accessToken
    }

    pub fn submit(&self, title: String, url: String) -> Result<(), Error> {
      // POST https://api.linkedin.com/v1/people/~/shares?format=json
    }
}


fn accept 