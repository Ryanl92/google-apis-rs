<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-books1` library allows access to all features of the *Google books* service.

This documentation was generated from *books* crate version *0.1.8+20150318*, where *20150318* is the exact revision of the *books:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.8*.

Everything else about the *books* *v1* API can be found at the
[official documentation site](https://developers.google.com/books/docs/v1/getting_started).
# Features

Handle the following *Resources* with ease from the central [hub](http://byron.github.io/google-apis-rs/google_books1/struct.Books.html) ... 

* bookshelves
 * [*get*](http://byron.github.io/google-apis-rs/google_books1/struct.BookshelveGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_books1/struct.BookshelveListCall.html) and [*volumes list*](http://byron.github.io/google-apis-rs/google_books1/struct.BookshelveVolumeListCall.html)
* cloudloading
 * [*add book*](http://byron.github.io/google-apis-rs/google_books1/struct.CloudloadingAddBookCall.html), [*delete book*](http://byron.github.io/google-apis-rs/google_books1/struct.CloudloadingDeleteBookCall.html) and [*update book*](http://byron.github.io/google-apis-rs/google_books1/struct.CloudloadingUpdateBookCall.html)
* dictionary
 * [*list offline metadata*](http://byron.github.io/google-apis-rs/google_books1/struct.DictionaryListOfflineMetadataCall.html)
* layers
 * [*annotation data get*](http://byron.github.io/google-apis-rs/google_books1/struct.LayerAnnotationDataGetCall.html), [*annotation data list*](http://byron.github.io/google-apis-rs/google_books1/struct.LayerAnnotationDataListCall.html), [*get*](http://byron.github.io/google-apis-rs/google_books1/struct.LayerGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_books1/struct.LayerListCall.html), [*volume annotations get*](http://byron.github.io/google-apis-rs/google_books1/struct.LayerVolumeAnnotationGetCall.html) and [*volume annotations list*](http://byron.github.io/google-apis-rs/google_books1/struct.LayerVolumeAnnotationListCall.html)
* myconfig
 * [*get user settings*](http://byron.github.io/google-apis-rs/google_books1/struct.MyconfigGetUserSettingCall.html), [*release download access*](http://byron.github.io/google-apis-rs/google_books1/struct.MyconfigReleaseDownloadAccesCall.html), [*request access*](http://byron.github.io/google-apis-rs/google_books1/struct.MyconfigRequestAccesCall.html), [*sync volume licenses*](http://byron.github.io/google-apis-rs/google_books1/struct.MyconfigSyncVolumeLicenseCall.html) and [*update user settings*](http://byron.github.io/google-apis-rs/google_books1/struct.MyconfigUpdateUserSettingCall.html)
* mylibrary
 * [*annotations delete*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryAnnotationDeleteCall.html), [*annotations insert*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryAnnotationInsertCall.html), [*annotations list*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryAnnotationListCall.html), [*annotations summary*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryAnnotationSummaryCall.html), [*annotations update*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryAnnotationUpdateCall.html), [*bookshelves add volume*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryBookshelveAddVolumeCall.html), [*bookshelves clear volumes*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryBookshelveClearVolumeCall.html), [*bookshelves get*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryBookshelveGetCall.html), [*bookshelves list*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryBookshelveListCall.html), [*bookshelves move volume*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryBookshelveMoveVolumeCall.html), [*bookshelves remove volume*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryBookshelveRemoveVolumeCall.html), [*bookshelves volumes list*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryBookshelveVolumeListCall.html), [*readingpositions get*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryReadingpositionGetCall.html) and [*readingpositions set position*](http://byron.github.io/google-apis-rs/google_books1/struct.MylibraryReadingpositionSetPositionCall.html)
* onboarding
 * [*list categories*](http://byron.github.io/google-apis-rs/google_books1/struct.OnboardingListCategoryCall.html) and [*list category volumes*](http://byron.github.io/google-apis-rs/google_books1/struct.OnboardingListCategoryVolumeCall.html)
* promooffer
 * [*accept*](http://byron.github.io/google-apis-rs/google_books1/struct.PromoofferAcceptCall.html), [*dismiss*](http://byron.github.io/google-apis-rs/google_books1/struct.PromoofferDismisCall.html) and [*get*](http://byron.github.io/google-apis-rs/google_books1/struct.PromoofferGetCall.html)
* [volumes](http://byron.github.io/google-apis-rs/google_books1/struct.Volume.html)
 * [*associated list*](http://byron.github.io/google-apis-rs/google_books1/struct.VolumeAssociatedListCall.html), [*get*](http://byron.github.io/google-apis-rs/google_books1/struct.VolumeGetCall.html), [*list*](http://byron.github.io/google-apis-rs/google_books1/struct.VolumeListCall.html), [*mybooks list*](http://byron.github.io/google-apis-rs/google_books1/struct.VolumeMybookListCall.html), [*recommended list*](http://byron.github.io/google-apis-rs/google_books1/struct.VolumeRecommendedListCall.html), [*recommended rate*](http://byron.github.io/google-apis-rs/google_books1/struct.VolumeRecommendedRateCall.html) and [*useruploaded list*](http://byron.github.io/google-apis-rs/google_books1/struct.VolumeUseruploadedListCall.html)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](http://byron.github.io/google-apis-rs/google_books1/struct.Books.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](http://byron.github.io/google-apis-rs/google_books1/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](http://byron.github.io/google-apis-rs/google_books1/trait.CallBuilder.html)
* **[Resources](http://byron.github.io/google-apis-rs/google_books1/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](http://byron.github.io/google-apis-rs/google_books1/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](http://byron.github.io/google-apis-rs/google_books1/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.volumes().useruploaded_list(...).doit()
let r = hub.myconfig().sync_volume_licenses(...).doit()
let r = hub.volumes().list(...).doit()
let r = hub.volumes().associated_list(...).doit()
let r = hub.bookshelves().volumes_list(...).doit()
let r = hub.volumes().recommended_list(...).doit()
let r = hub.mylibrary().bookshelves_volumes_list(...).doit()
let r = hub.volumes().mybooks_list(...).doit()
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
google-books1 = "*"
```

## A complete example

```Rust
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_books1 as books1;
use books1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use books1::Books;

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
let mut hub = Books::new(hyper::Client::new(), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.volumes().list("q")
             .start_index(53)
             .source("Stet")
             .show_preorders(true)
             .projection("et")
             .print_type("dolores")
             .partner("kasd")
             .order_by("accusam")
             .max_results(93)
             .library_restrict("justo")
             .lang_restrict("amet.")
             .filter("erat")
             .download("labore")
             .doit();

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
        |Error::MissingAPIKey
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](http://byron.github.io/google-apis-rs/google_books1/enum.Result.html) enumeration as return value of 
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](http://byron.github.io/google-apis-rs/google_books1/trait.Delegate.html), or the [Authenticator Delegate](http://byron.github.io/google-apis-rs/google_books1/../yup-oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](http://byron.github.io/google-apis-rs/google_books1/enum.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](http://byron.github.io/google-apis-rs/google_books1/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](http://byron.github.io/google-apis-rs/google_books1/trait.Delegate.html) to the 
[Method Builder](http://byron.github.io/google-apis-rs/google_books1/trait.CallBuilder.html) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](http://byron.github.io/google-apis-rs/google_books1/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [enocodable](http://byron.github.io/google-apis-rs/google_books1/trait.RequestValue.html) and 
[decodable](http://byron.github.io/google-apis-rs/google_books1/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](http://byron.github.io/google-apis-rs/google_books1/trait.Part.html) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](http://byron.github.io/google-apis-rs/google_books1/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](http://byron.github.io/google-apis-rs/google_books1/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **books1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rs/LICENSE.md
