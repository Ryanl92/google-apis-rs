<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/README.md.mako'
DO NOT EDIT !
-->
The `google-translate2` library allows access to all features of the *Google translate* service.

This documentation was generated from *translate* crate version *0.1.1+20141123*, where *20141123* is the exact revision of the *translate:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.1*.

Everything else about the *translate* *v2* API can be found at the
[official documentation site](https://developers.google.com/translate/v2/using_rest).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google-translate2/struct.Translate.html) ... 

* detections
 * [*list*](http://byron.github.io/google-apis-rs/google-translate2/struct.DetectionListCall.html)
* languages
 * [*list*](http://byron.github.io/google-apis-rs/google-translate2/struct.LanguageListCall.html)
* translations
 * [*list*](http://byron.github.io/google-apis-rs/google-translate2/struct.TranslationListCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google-translate2/struct.Translate.html)**
    * a central object to maintain state and allow accessing all *Activities*
* **[Resources](http://byron.github.io/google-apis-rs/google-translate2/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google-translate2/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google-translate2/trait.CallBuilder.html)**
    * operations to apply to *Resources*

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.detections().list(...).doit()
```

The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
The `doit()` method performs the actual communication with the server and returns the respective result.

# Usage

## Setting up your Project

To use this library, you would put the following lines into your `Cargo.toml` file:

```toml
[dependencies]
google-translate2 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate "yup-oauth2" as oauth2;
extern crate "google-translate2" as translate2;
use translate2::Result;
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use translate2::Translate;

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                              hyper::Client::new(),
                              <MemoryStorage as Default>::default(), None);
let mut hub = Translate::new(hyper::Client::new(), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.detections().list("q")
             .doit();

match result {
    Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
    Result::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
    Result::MissingToken => println!("OAuth2: Missing Token"),
    Result::Cancelled => println!("Operation cancelled by user"),
    Result::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
    Result::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
    Result::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
    Result::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
    Result::Success(_) => println!("Success (value doesn't print)"),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google-translate2/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google-translate2/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google-translate2/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downlods
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google-translate2/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google-translate2/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google-translate2/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google-translate2/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google-translate2/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google-translate2/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google-translate2/trait.ResponseResult.html) via json. Optionals are used to indicate that partial requests are responses are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google-translate2/trait.Part.html) which are identifyable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google-translate2/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google-translate2/trait.RequestValue.html) are borrowed

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **translate2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md