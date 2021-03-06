// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *genomics* crate version *0.1.8+20150326*, where *20150326* is the exact revision of the *genomics:v1beta2* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.8*.
//! 
//! Everything else about the *genomics* *v1_beta2* API can be found at the
//! [official documentation site](https://developers.google.com/genomics/v1beta2/reference).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/genomics1_beta2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Genomics.html) ... 
//! 
//! * [annotation sets](struct.AnnotationSet.html)
//!  * [*create*](struct.AnnotationSetCreateCall.html), [*delete*](struct.AnnotationSetDeleteCall.html), [*get*](struct.AnnotationSetGetCall.html), [*patch*](struct.AnnotationSetPatchCall.html), [*search*](struct.AnnotationSetSearchCall.html) and [*update*](struct.AnnotationSetUpdateCall.html)
//! * [annotations](struct.Annotation.html)
//!  * [*batch create*](struct.AnnotationBatchCreateCall.html), [*create*](struct.AnnotationCreateCall.html), [*delete*](struct.AnnotationDeleteCall.html), [*get*](struct.AnnotationGetCall.html), [*patch*](struct.AnnotationPatchCall.html), [*search*](struct.AnnotationSearchCall.html) and [*update*](struct.AnnotationUpdateCall.html)
//! * callsets
//!  * [*create*](struct.CallsetCreateCall.html), [*delete*](struct.CallsetDeleteCall.html), [*get*](struct.CallsetGetCall.html), [*patch*](struct.CallsetPatchCall.html), [*search*](struct.CallsetSearchCall.html) and [*update*](struct.CallsetUpdateCall.html)
//! * [datasets](struct.Dataset.html)
//!  * [*create*](struct.DatasetCreateCall.html), [*delete*](struct.DatasetDeleteCall.html), [*get*](struct.DatasetGetCall.html), [*list*](struct.DatasetListCall.html), [*patch*](struct.DatasetPatchCall.html), [*undelete*](struct.DatasetUndeleteCall.html) and [*update*](struct.DatasetUpdateCall.html)
//! * experimental
//!  * [*jobs create*](struct.ExperimentalJobCreateCall.html)
//! * [jobs](struct.Job.html)
//!  * [*cancel*](struct.JobCancelCall.html), [*get*](struct.JobGetCall.html) and [*search*](struct.JobSearchCall.html)
//! * readgroupsets
//!  * [*align*](struct.ReadgroupsetAlignCall.html), [*call*](struct.ReadgroupsetCallCall.html), [*coveragebuckets list*](struct.ReadgroupsetCoveragebucketListCall.html), [*delete*](struct.ReadgroupsetDeleteCall.html), [*export*](struct.ReadgroupsetExportCall.html), [*get*](struct.ReadgroupsetGetCall.html), [*import*](struct.ReadgroupsetImportCall.html), [*patch*](struct.ReadgroupsetPatchCall.html), [*search*](struct.ReadgroupsetSearchCall.html) and [*update*](struct.ReadgroupsetUpdateCall.html)
//! * [reads](struct.Read.html)
//!  * [*search*](struct.ReadSearchCall.html)
//! * [references](struct.Reference.html)
//!  * [*bases list*](struct.ReferenceBaseListCall.html), [*get*](struct.ReferenceGetCall.html) and [*search*](struct.ReferenceSearchCall.html)
//! * referencesets
//!  * [*get*](struct.ReferencesetGetCall.html) and [*search*](struct.ReferencesetSearchCall.html)
//! * streaming readstore
//!  * [*streamreads*](struct.StreamingReadstoreStreamreadCall.html)
//! * [variants](struct.Variant.html)
//!  * [*create*](struct.VariantCreateCall.html), [*delete*](struct.VariantDeleteCall.html), [*get*](struct.VariantGetCall.html), [*search*](struct.VariantSearchCall.html) and [*update*](struct.VariantUpdateCall.html)
//! * variantsets
//!  * [*delete*](struct.VariantsetDeleteCall.html), [*export*](struct.VariantsetExportCall.html), [*get*](struct.VariantsetGetCall.html), [*import variants*](struct.VariantsetImportVariantCall.html), [*merge variants*](struct.VariantsetMergeVariantCall.html), [*patch*](struct.VariantsetPatchCall.html), [*search*](struct.VariantsetSearchCall.html) and [*update*](struct.VariantsetUpdateCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Genomics.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.annotations().search(...).doit()
//! let r = hub.annotations().patch(...).doit()
//! let r = hub.annotations().create(...).doit()
//! let r = hub.annotations().delete(...).doit()
//! let r = hub.annotations().update(...).doit()
//! let r = hub.annotations().get(...).doit()
//! let r = hub.annotations().batch_create(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-genomics1_beta2 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_genomics1_beta2 as genomics1_beta2;
//! use genomics1_beta2::Annotation;
//! use genomics1_beta2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use genomics1_beta2::Genomics;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Genomics::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Annotation::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.annotations().patch(req, "annotationId")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

include!(concat!(env!("OUT_DIR"), "/lib.rs"));