// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *plus* crate version *0.1.5+20150303*, where *20150303* is the exact revision of the *plus:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.5*.
//! 
//! Everything else about the *plus* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/+/api/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/plus1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Plus.html) ... 
//! 
//! * [activities](struct.Activity.html)
//!  * [*get*](struct.ActivityGetCall.html), [*list*](struct.ActivityListCall.html) and [*search*](struct.ActivitySearchCall.html)
//! * [comments](struct.Comment.html)
//!  * [*get*](struct.CommentGetCall.html) and [*list*](struct.CommentListCall.html)
//! * [moments](struct.Moment.html)
//!  * [*insert*](struct.MomentInsertCall.html), [*list*](struct.MomentListCall.html) and [*remove*](struct.MomentRemoveCall.html)
//! * people
//!  * [*get*](struct.PeopleGetCall.html), [*list*](struct.PeopleListCall.html), [*list by activity*](struct.PeopleListByActivityCall.html) and [*search*](struct.PeopleSearchCall.html)
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
//! * **[Hub](struct.Plus.html)**
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
//! let r = hub.people().search(...).doit()
//! let r = hub.people().list(...).doit()
//! let r = hub.people().list_by_activity(...).doit()
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
//! google-plus1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_plus1 as plus1;
//! use plus1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use plus1::Plus;
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
//! let mut hub = Plus::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.people().list("userId", "collection")
//!              .page_token("accusam")
//!              .order_by("takimata")
//!              .max_results(31)
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!         Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_) => println!("{}", e),
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
//! * [request values](trait.RequestValue.html) are borrowed
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(std_misc)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin, slice_patterns)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep_ms;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// Know your basic profile info and list of people in your circles.
    Login,

    /// View your email address
    UserinfoEmail,

    /// View your basic profile info
    UserinfoProfile,

    /// Know who you are on Google
    Me,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Login => "https://www.googleapis.com/auth/plus.login",
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            Scope::UserinfoProfile => "https://www.googleapis.com/auth/userinfo.profile",
            Scope::Me => "https://www.googleapis.com/auth/plus.me",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Me
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Plus related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_plus1 as plus1;
/// use plus1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use plus1::Plus;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().list("userId", "collection")
///              .page_token("labore")
///              .order_by("sea")
///              .max_results(11)
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///         Error::HttpError(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct Plus<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
}

impl<'a, C, A> Hub for Plus<C, A> {}

impl<'a, C, A> Plus<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Plus<C, A> {
        Plus {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.5".to_string(),
        }
    }

    pub fn activities(&'a self) -> ActivityMethods<'a, C, A> {
        ActivityMethods { hub: &self }
    }
    pub fn comments(&'a self) -> CommentMethods<'a, C, A> {
        CommentMethods { hub: &self }
    }
    pub fn moments(&'a self) -> MomentMethods<'a, C, A> {
        MomentMethods { hub: &self }
    }
    pub fn people(&'a self) -> PeopleMethods<'a, C, A> {
        PeopleMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// If the attachment is an album, this property is a list of potential additional thumbnails from the album.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachmentsThumbnails {
    /// URL of the webpage containing the image.
    pub url: Option<String>,
    /// Image resource.
    pub image: Option<ActivityObjectAttachmentsThumbnailsImage>,
    /// Potential name of the thumbnail.
    pub description: Option<String>,
}

impl NestedType for ActivityObjectAttachmentsThumbnails {}
impl Part for ActivityObjectAttachmentsThumbnails {}


/// An object representation of the individual components of name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityActorName {
    /// The given name ("first name") of the actor.
    #[serde(rename="givenName")]
    pub given_name: Option<String>,
    /// The family name ("last name") of the actor.
    #[serde(rename="familyName")]
    pub family_name: Option<String>,
}

impl NestedType for ActivityActorName {}
impl Part for ActivityActorName {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list activities](struct.ActivityListCall.html) (response)
/// * [search activities](struct.ActivitySearchCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityFeed {
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies this resource as a collection of activities. Value: "plus#activityFeed".
    pub kind: Option<String>,
    /// The title of this collection of activities, which is a truncated portion of the content.
    pub title: Option<String>,
    /// The activities in this page of results.
    pub items: Option<Vec<Activity>>,
    /// The time at which this collection of activities was last updated. Formatted as an RFC 3339 timestamp.
    pub updated: Option<String>,
    /// Link to the next page of activities.
    #[serde(rename="nextLink")]
    pub next_link: Option<String>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The ID of this collection of activities. Deprecated.
    pub id: Option<String>,
    /// Link to this activity resource.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl ResponseResult for ActivityFeed {}


/// A list of URLs for this person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonUrls {
    /// The type of URL. Possible values include, but are not limited to, the following values:  
    /// - "otherProfile" - URL for another profile. 
    /// - "contributor" - URL to a site for which this person is a contributor. 
    /// - "website" - URL for this Google+ Page's primary website. 
    /// - "other" - Other URL.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The URL value.
    pub value: Option<String>,
    /// The label of the URL.
    pub label: Option<String>,
}

impl NestedType for PersonUrls {}
impl Part for PersonUrls {}


/// A list of email addresses that this person has, including their Google account email address, and the public verified email addresses on their Google+ profile. The plus.profile.emails.read scope is needed to retrieve these email addresses, or the email scope can be used to retrieve just the Google account email address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonEmails {
    /// The type of address. Possible values include, but are not limited to, the following values:  
    /// - "account" - Google account email address. 
    /// - "home" - Home email address. 
    /// - "work" - Work email address. 
    /// - "other" - Other.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The email address.
    pub value: Option<String>,
}

impl NestedType for PersonEmails {}
impl Part for PersonEmails {}


/// A list of current or past organizations with which this person is associated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonOrganizations {
    /// The date that the person joined this organization.
    #[serde(rename="startDate")]
    pub start_date: Option<String>,
    /// The date that the person left this organization.
    #[serde(rename="endDate")]
    pub end_date: Option<String>,
    /// A short description of the person's role in this organization. Deprecated.
    pub description: Option<String>,
    /// The person's job title or role within the organization.
    pub title: Option<String>,
    /// If "true", indicates this organization is the person's primary one, which is typically interpreted as the current one.
    pub primary: Option<bool>,
    /// The location of this organization. Deprecated.
    pub location: Option<String>,
    /// The department within the organization. Deprecated.
    pub department: Option<String>,
    /// The type of organization. Possible values include, but are not limited to, the following values:  
    /// - "work" - Work. 
    /// - "school" - School.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The name of the organization.
    pub name: Option<String>,
}

impl NestedType for PersonOrganizations {}
impl Part for PersonOrganizations {}


/// Comments in reply to this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectReplies {
    /// Total number of comments on this activity.
    #[serde(rename="totalItems")]
    pub total_items: Option<u32>,
    /// The URL for the collection of comments in reply to this activity.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl NestedType for ActivityObjectReplies {}
impl Part for ActivityObjectReplies {}


/// If the attachment is a video, the embeddable link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachmentsEmbed {
    /// URL of the link.
    pub url: Option<String>,
    /// Media type of the link.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl NestedType for ActivityObjectAttachmentsEmbed {}
impl Part for ActivityObjectAttachmentsEmbed {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlusAclentryResource {
    /// A descriptive name for this entry. Suitable for display.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The type of entry describing to whom access is granted. Possible values are:  
    /// - "person" - Access to an individual. 
    /// - "circle" - Access to members of a circle. 
    /// - "myCircles" - Access to members of all the person's circles. 
    /// - "extendedCircles" - Access to members of all the person's circles, plus all of the people in their circles. 
    /// - "domain" - Access to members of the person's Google Apps domain. 
    /// - "public" - Access to anyone on the web.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The ID of the entry. For entries of type "person" or "circle", this is the ID of the resource. For other types, this property is not set.
    pub id: Option<String>,
}

impl Part for PlusAclentryResource {}


/// A list of places where this person has lived.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonPlacesLived {
    /// If "true", this place of residence is this person's primary residence.
    pub primary: Option<bool>,
    /// A place where this person has lived. For example: "Seattle, WA", "Near Toronto".
    pub value: Option<String>,
}

impl NestedType for PersonPlacesLived {}
impl Part for PersonPlacesLived {}


/// The object of this comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentObject {
    /// The HTML-formatted content, suitable for display.
    pub content: Option<String>,
    /// The object type of this comment. Possible values are:  
    /// - "comment" - A comment in reply to an activity.
    #[serde(rename="objectType")]
    pub object_type: Option<String>,
    /// The content (text) as provided by the author, stored without any HTML formatting. When creating or updating a comment, this value must be supplied as plain text in the request.
    #[serde(rename="originalContent")]
    pub original_content: Option<String>,
}

impl NestedType for CommentObject {}
impl Part for CommentObject {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ItemScope {
    /// The start date and time of the event (in ISO 8601 date format).
    #[serde(rename="startDate")]
    pub start_date: Option<String>,
    /// The end date and time of the event (in ISO 8601 date format).
    #[serde(rename="endDate")]
    pub end_date: Option<String>,
    /// The text that is the result of the app activity. For example, if a user leaves a review of a restaurant, this might be the text of the review.
    pub text: Option<String>,
    /// A URL to the image that represents this result. For example, if a user writes a review of a restaurant and attaches a photo of their meal, you might use that photo as the result.image.
    pub image: Option<String>,
    /// Review rating.
    #[serde(rename="reviewRating")]
    pub review_rating: Option<Option<Box<ItemScope>>>,
    /// Date of birth.
    #[serde(rename="birthDate")]
    pub birth_date: Option<String>,
    /// The initial date that the result was published. For example, a user writes a comment on a blog, which has a result.dateCreated of when they submit it. If the blog users comment moderation, the result.datePublished value would match the date when the owner approved the message.
    #[serde(rename="datePublished")]
    pub date_published: Option<String>,
    /// Address locality.
    #[serde(rename="addressLocality")]
    pub address_locality: Option<String>,
    /// The duration of the item (movie, audio recording, event, etc.) in ISO 8601 date format.
    pub duration: Option<String>,
    /// An additional name for a Person, can be used for a middle name.
    #[serde(rename="additionalName")]
    pub additional_name: Option<Vec<String>>,
    /// Worst possible rating value that a result might obtain. This property defines the lower bound for the ratingValue.
    #[serde(rename="worstRating")]
    pub worst_rating: Option<String>,
    /// A list of contributors to this result.
    pub contributor: Option<Vec<ItemScope>>,
    /// A URL to a thumbnail image that represents this result.
    #[serde(rename="thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    /// An identifier for the object. Your app can choose how to identify objects. The object.id is required if you are writing an action that does not have a corresponding web page or object.url property.
    pub id: Option<String>,
    /// Post office box number.
    #[serde(rename="postOfficeBoxNumber")]
    pub post_office_box_number: Option<String>,
    /// A person attending the event.
    pub attendees: Option<Vec<ItemScope>>,
    /// The person or persons who created this result. In the example of restaurant reviews, this might be the reviewer's name.
    pub author: Option<Vec<ItemScope>>,
    /// The encoding.
    pub associated_media: Option<Vec<ItemScope>>,
    /// Best possible rating value that a result might obtain. This property defines the upper bound for the ratingValue. For example, you might have a 5 star rating scale, you would provide 5 as the value for this property.
    #[serde(rename="bestRating")]
    pub best_rating: Option<String>,
    /// Address country.
    #[serde(rename="addressCountry")]
    pub address_country: Option<i64>,
    /// The width of the media object.
    pub width: Option<String>,
    /// Street address.
    #[serde(rename="streetAddress")]
    pub street_address: Option<String>,
    /// The location of the event or organization.
    pub location: Option<Option<Box<ItemScope>>>,
    /// Latitude.
    pub latitude: Option<f64>,
    /// From http://schema.org/MusicRecording, the artist that performed this recording.
    #[serde(rename="byArtist")]
    pub by_artist: Option<Option<Box<ItemScope>>>,
    /// The schema.org URL that best describes the referenced object and matches the type of moment.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The date the result was last modified such as the date that a review was last edited.
    #[serde(rename="dateModified")]
    pub date_modified: Option<String>,
    /// File size in (mega/kilo) bytes.
    #[serde(rename="contentSize")]
    pub content_size: Option<String>,
    /// Actual bytes of the media object, for example the image file or video file.
    #[serde(rename="contentUrl")]
    pub content_url: Option<String>,
    /// Family name. This property can be used with givenName instead of the name property.
    #[serde(rename="familyName")]
    pub family_name: Option<String>,
    /// The string that describes the content of the result.
    pub description: Option<String>,
    /// Property of http://schema.org/TVEpisode indicating which series the episode belongs to.
    #[serde(rename="partOfTVSeries")]
    pub part_of_tv_series: Option<Option<Box<ItemScope>>>,
    /// The subject matter of the content.
    pub about: Option<Option<Box<ItemScope>>>,
    /// The date the result was created such as the date that a review was first created.
    #[serde(rename="dateCreated")]
    pub date_created: Option<String>,
    /// Postal code.
    #[serde(rename="postalCode")]
    pub postal_code: Option<String>,
    /// Number of attendees.
    #[serde(rename="attendeeCount")]
    pub attendee_count: Option<i32>,
    /// From http://schema.org/MusicRecording, which album a song is in.
    #[serde(rename="inAlbum")]
    pub in_album: Option<Option<Box<ItemScope>>>,
    /// Postal address.
    pub address: Option<Option<Box<ItemScope>>>,
    /// The height of the media object.
    pub height: Option<String>,
    /// Geo coordinates.
    pub geo: Option<Option<Box<ItemScope>>>,
    /// A URL pointing to a player for a specific video. In general, this is the information in the src element of an embed tag and should not be the same as the content of the loc tag.
    #[serde(rename="embedUrl")]
    pub embed_url: Option<String>,
    /// The exchange traded instrument associated with a Corporation object. The tickerSymbol is expressed as an exchange and an instrument name separated by a space character. For the exchange component of the tickerSymbol attribute, we recommend using the controlled vocabulary of Market Identifier Codes (MIC) specified in ISO15022.
    #[serde(rename="tickerSymbol")]
    pub ticker_symbol: Option<String>,
    /// The name of the result. In the example of a restaurant review, this might be the summary the user gave their review such as "Great ambiance, but overpriced."
    pub name: Option<String>,
    /// Identifies this resource as an itemScope.
    pub kind: Option<String>,
    /// From http://schema.org/MusicRecording, the audio file.
    pub audio: Option<Option<Box<ItemScope>>>,
    /// Player type that is required. For example: Flash or Silverlight.
    #[serde(rename="playerType")]
    pub player_type: Option<String>,
    /// The main performer or performers of the event-for example, a presenter, musician, or actor.
    pub performers: Option<Vec<ItemScope>>,
    /// The URL that points to the result object. For example, a permalink directly to a restaurant reviewer's comment.
    pub url: Option<String>,
    /// Gender of the person.
    pub gender: Option<String>,
    /// Longitude.
    pub longitude: Option<f64>,
    /// Thumbnail image for an image or video.
    pub thumbnail: Option<Option<Box<ItemScope>>>,
    /// Address region.
    #[serde(rename="addressRegion")]
    pub address_region: Option<String>,
    /// The caption for this object.
    pub caption: Option<String>,
    /// Rating value.
    #[serde(rename="ratingValue")]
    pub rating_value: Option<String>,
    /// Given name. This property can be used with familyName instead of the name property.
    #[serde(rename="givenName")]
    pub given_name: Option<String>,
}

impl Part for ItemScope {}


/// The preview image for photos or videos.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachmentsImage {
    /// Image URL.
    pub url: Option<String>,
    /// The width, in pixels, of the linked resource.
    pub width: Option<u32>,
    /// Media type of the link.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The height, in pixels, of the linked resource.
    pub height: Option<u32>,
}

impl NestedType for ActivityObjectAttachmentsImage {}
impl Part for ActivityObjectAttachmentsImage {}


/// The activity this comment replied to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentInReplyTo {
    /// The URL of the activity.
    pub url: Option<String>,
    /// The ID of the activity.
    pub id: Option<String>,
}

impl NestedType for CommentInReplyTo {}
impl Part for CommentInReplyTo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search people](struct.PeopleSearchCall.html) (response)
/// * [list people](struct.PeopleListCall.html) (response)
/// * [list by activity people](struct.PeopleListByActivityCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PeopleFeed {
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies this resource as a collection of people. Value: "plus#peopleFeed".
    pub kind: Option<String>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The title of this collection of people.
    pub title: Option<String>,
    /// The total number of people available in this list. The number of people in a response might be smaller due to paging. This might not be set for all collections.
    #[serde(rename="totalItems")]
    pub total_items: Option<i32>,
    /// The people in this page of results. Each item includes the id, displayName, image, and url for the person. To retrieve additional profile data, see the people.get method.
    pub items: Option<Vec<Person>>,
    /// Link to this resource.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl ResponseResult for PeopleFeed {}


/// The person who posted this comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentActor {
    /// A link to the Person resource for this actor.
    pub url: Option<String>,
    /// The image representation of this actor.
    pub image: Option<CommentActorImage>,
    /// The name of this actor, suitable for display.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The ID of the actor.
    pub id: Option<String>,
}

impl NestedType for CommentActor {}
impl Part for CommentActor {}


/// The person who performed this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityActor {
    /// The link to the actor's Google profile.
    pub url: Option<String>,
    /// The image representation of the actor.
    pub image: Option<ActivityActorImage>,
    /// The name of the actor, suitable for display.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The ID of the actor's Person resource.
    pub id: Option<String>,
    /// An object representation of the individual components of name.
    pub name: Option<ActivityActorName>,
}

impl NestedType for ActivityActor {}
impl Part for ActivityActor {}


/// The service provider that initially published this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityProvider {
    /// Name of the service provider.
    pub title: Option<String>,
}

impl NestedType for ActivityProvider {}
impl Part for ActivityProvider {}


/// The position of the place.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacePosition {
    /// The latitude of this position.
    pub latitude: Option<f64>,
    /// The longitude of this position.
    pub longitude: Option<f64>,
}

impl NestedType for PlacePosition {}
impl Part for PlacePosition {}


/// People who +1'd this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectPlusoners {
    /// Total number of people who +1'd this activity.
    #[serde(rename="totalItems")]
    pub total_items: Option<u32>,
    /// The URL for the collection of people who +1'd this activity.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl NestedType for ActivityObjectPlusoners {}
impl Part for ActivityObjectPlusoners {}


/// Extra information about the cover photo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonCoverCoverInfo {
    /// The difference between the left position of the cover image and the actual displayed cover image. Only valid for banner layout.
    #[serde(rename="leftImageOffset")]
    pub left_image_offset: Option<i32>,
    /// The difference between the top position of the cover image and the actual displayed cover image. Only valid for banner layout.
    #[serde(rename="topImageOffset")]
    pub top_image_offset: Option<i32>,
}

impl NestedType for PersonCoverCoverInfo {}
impl Part for PersonCoverCoverInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list moments](struct.MomentListCall.html) (none)
/// * [insert moments](struct.MomentInsertCall.html) (request|response)
/// * [remove moments](struct.MomentRemoveCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Moment {
    /// Time stamp of when the action occurred in RFC3339 format.
    #[serde(rename="startDate")]
    pub start_date: Option<String>,
    /// Identifies this resource as a moment.
    pub kind: Option<String>,
    /// The object on which the action was performed.
    pub target: Option<ItemScope>,
    /// The object on which the action was performed. Specifying this is equivalent with specifying "target". Note that responses from the server will use the "target" field instead for backward-compatibility with older clients.
    pub object: Option<ItemScope>,
    /// The schema.org type for the type of moment to write. For example, http://schema.org/AddAction. Note that responses from the server will use the Google schema type instead for backward-compatibility with older clients. For example, http://schemas.google.com/AddActivity.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The moment ID.
    pub id: Option<String>,
    /// The object generated by performing the action on the object. For example, a user writes a review of a restaurant, the object is the restaurant and the result is the review.
    pub result: Option<ItemScope>,
}

impl RequestValue for Moment {}
impl Resource for Moment {}
impl ResponseResult for Moment {}


/// The image representation of this actor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentActorImage {
    /// The URL of the actor's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.
    pub url: Option<String>,
}

impl NestedType for CommentActorImage {}
impl Part for CommentActorImage {}


/// The representation of the person's profile photo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonImage {
    /// The URL of the person's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.
    pub url: Option<String>,
    /// Whether the person's profile photo is the default one
    #[serde(rename="isDefault")]
    pub is_default: Option<bool>,
}

impl NestedType for PersonImage {}
impl Part for PersonImage {}


/// People who +1'd this comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentPlusoners {
    /// Total number of people who +1'd this comment.
    #[serde(rename="totalItems")]
    pub total_items: Option<u32>,
}

impl NestedType for CommentPlusoners {}
impl Part for CommentPlusoners {}


/// The media objects attached to this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachments {
    /// The title of the attachment, such as a photo caption or an article title.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// If the attachment is an album, this property is a list of potential additional thumbnails from the album.
    pub thumbnails: Option<Vec<ActivityObjectAttachmentsThumbnails>>,
    /// The full image URL for photo attachments.
    #[serde(rename="fullImage")]
    pub full_image: Option<ActivityObjectAttachmentsFullImage>,
    /// The link to the attachment, which should be of type text/html.
    pub url: Option<String>,
    /// The preview image for photos or videos.
    pub image: Option<ActivityObjectAttachmentsImage>,
    /// If the attachment is an article, this property contains a snippet of text from the article. It can also include descriptions for other types.
    pub content: Option<String>,
    /// If the attachment is a video, the embeddable link.
    pub embed: Option<ActivityObjectAttachmentsEmbed>,
    /// The ID of the attachment.
    pub id: Option<String>,
    /// The type of media object. Possible values include, but are not limited to, the following values:  
    /// - "photo" - A photo. 
    /// - "album" - A photo album. 
    /// - "video" - A video. 
    /// - "article" - An article, specified by a link.
    #[serde(rename="objectType")]
    pub object_type: Option<String>,
}

impl NestedType for ActivityObjectAttachments {}
impl Part for ActivityObjectAttachments {}


/// The person's primary cover image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonCoverCoverPhoto {
    /// The URL of the image.
    pub url: Option<String>,
    /// The width of the image.
    pub width: Option<i32>,
    /// The height of the image.
    pub height: Option<i32>,
}

impl NestedType for PersonCoverCoverPhoto {}
impl Part for PersonCoverCoverPhoto {}


/// The full image URL for photo attachments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachmentsFullImage {
    /// URL of the image.
    pub url: Option<String>,
    /// The width, in pixels, of the linked resource.
    pub width: Option<u32>,
    /// Media type of the link.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The height, in pixels, of the linked resource.
    pub height: Option<u32>,
}

impl NestedType for ActivityObjectAttachmentsFullImage {}
impl Part for ActivityObjectAttachmentsFullImage {}


/// The cover photo content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonCover {
    /// Extra information about the cover photo.
    #[serde(rename="coverInfo")]
    pub cover_info: Option<PersonCoverCoverInfo>,
    /// The layout of the cover art. Possible values include, but are not limited to, the following values:  
    /// - "banner" - One large image banner.
    pub layout: Option<String>,
    /// The person's primary cover image.
    #[serde(rename="coverPhoto")]
    pub cover_photo: Option<PersonCoverCoverPhoto>,
}

impl NestedType for PersonCover {}
impl Part for PersonCover {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Acl {
    /// The list of access entries.
    pub items: Option<Vec<PlusAclentryResource>>,
    /// Identifies this resource as a collection of access controls. Value: "plus#acl".
    pub kind: Option<String>,
    /// Description of the access granted, suitable for display.
    pub description: Option<String>,
}

impl Part for Acl {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get people](struct.PeopleGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    /// The "bragging rights" line of this person.
    #[serde(rename="braggingRights")]
    pub bragging_rights: Option<String>,
    /// The person's relationship status. Possible values include, but are not limited to, the following values:  
    /// - "single" - Person is single. 
    /// - "in_a_relationship" - Person is in a relationship. 
    /// - "engaged" - Person is engaged. 
    /// - "married" - Person is married. 
    /// - "its_complicated" - The relationship is complicated. 
    /// - "open_relationship" - Person is in an open relationship. 
    /// - "widowed" - Person is widowed. 
    /// - "in_domestic_partnership" - Person is in a domestic partnership. 
    /// - "in_civil_union" - Person is in a civil union.
    #[serde(rename="relationshipStatus")]
    pub relationship_status: Option<String>,
    /// The representation of the person's profile photo.
    pub image: Option<PersonImage>,
    /// The hosted domain name for the user's Google Apps account. For instance, example.com. The plus.profile.emails.read or email scope is needed to get this domain name.
    pub domain: Option<String>,
    /// A list of places where this person has lived.
    #[serde(rename="placesLived")]
    pub places_lived: Option<Vec<PersonPlacesLived>>,
    /// The person's date of birth, represented as YYYY-MM-DD.
    pub birthday: Option<String>,
    /// The name of this person, which is suitable for display.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The nickname of this person.
    pub nickname: Option<String>,
    /// A list of email addresses that this person has, including their Google account email address, and the public verified email addresses on their Google+ profile. The plus.profile.emails.read scope is needed to retrieve these email addresses, or the email scope can be used to retrieve just the Google account email address.
    pub emails: Option<Vec<PersonEmails>>,
    /// The occupation of this person.
    pub occupation: Option<String>,
    /// A list of current or past organizations with which this person is associated.
    pub organizations: Option<Vec<PersonOrganizations>>,
    /// Identifies this resource as a person. Value: "plus#person".
    pub kind: Option<String>,
    /// Whether the person or Google+ Page has been verified.
    pub verified: Option<bool>,
    /// An object representation of the individual components of a person's name.
    pub name: Option<PersonName>,
    /// The user's preferred language for rendering.
    pub language: Option<String>,
    /// Whether this user has signed up for Google+.
    #[serde(rename="isPlusUser")]
    pub is_plus_user: Option<bool>,
    /// The person's skills.
    pub skills: Option<String>,
    /// The brief description (tagline) of this person.
    pub tagline: Option<String>,
    /// (this field is not currently used)
    #[serde(rename="currentLocation")]
    pub current_location: Option<String>,
    /// The cover photo content.
    pub cover: Option<PersonCover>,
    /// The ID of this person.
    pub id: Option<String>,
    /// The URL of this person's profile.
    pub url: Option<String>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// If a Google+ Page, the number of people who have +1'd this page.
    #[serde(rename="plusOneCount")]
    pub plus_one_count: Option<i32>,
    /// A list of URLs for this person.
    pub urls: Option<Vec<PersonUrls>>,
    /// The person's gender. Possible values include, but are not limited to, the following values:  
    /// - "male" - Male gender. 
    /// - "female" - Female gender. 
    /// - "other" - Other.
    pub gender: Option<String>,
    /// The age range of the person. Valid ranges are 17 or younger, 18 to 20, and 21 or older. Age is determined from the user's birthday using Western age reckoning.
    #[serde(rename="ageRange")]
    pub age_range: Option<PersonAgeRange>,
    /// For followers who are visible, the number of people who have added this person or page to a circle.
    #[serde(rename="circledByCount")]
    pub circled_by_count: Option<i32>,
    /// Type of person within Google+. Possible values include, but are not limited to, the following values:  
    /// - "person" - represents an actual person. 
    /// - "page" - represents a page.
    #[serde(rename="objectType")]
    pub object_type: Option<String>,
    /// A short biography for this person.
    #[serde(rename="aboutMe")]
    pub about_me: Option<String>,
}

impl ResponseResult for Person {}


/// If this activity's object is itself another activity, such as when a person reshares an activity, this property specifies the original activity's actor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectActor {
    /// A link to the original actor's Google profile.
    pub url: Option<String>,
    /// The image representation of the original actor.
    pub image: Option<ActivityObjectActorImage>,
    /// The original actor's name, which is suitable for display.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// ID of the original actor.
    pub id: Option<String>,
}

impl NestedType for ActivityObjectActor {}
impl Part for ActivityObjectActor {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list moments](struct.MomentListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MomentsFeed {
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies this resource as a collection of moments. Value: "plus#momentsFeed".
    pub kind: Option<String>,
    /// The title of this collection of moments.
    pub title: Option<String>,
    /// The moments in this page of results.
    pub items: Option<Vec<Moment>>,
    /// The RFC 339 timestamp for when this collection of moments was last updated.
    pub updated: Option<String>,
    /// Link to the next page of moments.
    #[serde(rename="nextLink")]
    pub next_link: Option<String>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// Link to this page of moments.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl ResponseResult for MomentsFeed {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get activities](struct.ActivityGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    /// Name of the place where this activity occurred.
    #[serde(rename="placeName")]
    pub place_name: Option<String>,
    /// The time at which this activity was last updated. Formatted as an RFC 3339 timestamp.
    pub updated: Option<String>,
    /// The object of this activity.
    pub object: Option<ActivityObject>,
    /// This activity's verb, which indicates the action that was performed. Possible values include, but are not limited to, the following values:  
    /// - "post" - Publish content to the stream. 
    /// - "share" - Reshare an activity.
    pub verb: Option<String>,
    /// Radius, in meters, of the region where this activity occurred, centered at the latitude and longitude identified in geocode.
    pub radius: Option<String>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// Street address where this activity occurred.
    pub address: Option<String>,
    /// Additional content added by the person who shared this activity, applicable only when resharing an activity.
    pub annotation: Option<String>,
    /// Identifies this resource as an activity. Value: "plus#activity".
    pub kind: Option<String>,
    /// Title of this activity.
    pub title: Option<String>,
    /// The link to this activity.
    pub url: Option<String>,
    /// ID of the place where this activity occurred.
    #[serde(rename="placeId")]
    pub place_id: Option<String>,
    /// The person who performed this activity.
    pub actor: Option<ActivityActor>,
    /// The ID of this activity.
    pub id: Option<String>,
    /// Identifies who has access to see this activity.
    pub access: Option<Acl>,
    /// Latitude and longitude where this activity occurred. Format is latitude followed by longitude, space separated.
    pub geocode: Option<String>,
    /// The location where this activity occurred.
    pub location: Option<Place>,
    /// The service provider that initially published this activity.
    pub provider: Option<ActivityProvider>,
    /// If this activity is a crosspost from another system, this property specifies the ID of the original activity.
    #[serde(rename="crosspostSource")]
    pub crosspost_source: Option<String>,
    /// The time at which this activity was initially published. Formatted as an RFC 3339 timestamp.
    pub published: Option<String>,
}

impl ResponseResult for Activity {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list comments](struct.CommentListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentFeed {
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Identifies this resource as a collection of comments. Value: "plus#commentFeed".
    pub kind: Option<String>,
    /// The title of this collection of comments.
    pub title: Option<String>,
    /// The comments in this page of results.
    pub items: Option<Vec<Comment>>,
    /// The time at which this collection of comments was last updated. Formatted as an RFC 3339 timestamp.
    pub updated: Option<String>,
    /// Link to the next page of activities.
    #[serde(rename="nextLink")]
    pub next_link: Option<String>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The ID of this collection of comments.
    pub id: Option<String>,
}

impl ResponseResult for CommentFeed {}


/// People who reshared this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectResharers {
    /// Total number of people who reshared this activity.
    #[serde(rename="totalItems")]
    pub total_items: Option<u32>,
    /// The URL for the collection of resharers.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl NestedType for ActivityObjectResharers {}
impl Part for ActivityObjectResharers {}


/// The image representation of the actor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityActorImage {
    /// The URL of the actor's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.
    pub url: Option<String>,
}

impl NestedType for ActivityActorImage {}
impl Part for ActivityActorImage {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list comments](struct.CommentListCall.html) (none)
/// * [get comments](struct.CommentGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    /// The activity this comment replied to.
    #[serde(rename="inReplyTo")]
    pub in_reply_to: Option<Vec<CommentInReplyTo>>,
    /// Identifies this resource as a comment. Value: "plus#comment".
    pub kind: Option<String>,
    /// People who +1'd this comment.
    pub plusoners: Option<CommentPlusoners>,
    /// The object of this comment.
    pub object: Option<CommentObject>,
    /// The time at which this comment was last updated. Formatted as an RFC 3339 timestamp.
    pub updated: Option<String>,
    /// The person who posted this comment.
    pub actor: Option<CommentActor>,
    /// This comment's verb, indicating what action was performed. Possible values are:  
    /// - "post" - Publish content to the stream.
    pub verb: Option<String>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The time at which this comment was initially published. Formatted as an RFC 3339 timestamp.
    pub published: Option<String>,
    /// The ID of this comment.
    pub id: Option<String>,
    /// Link to this comment resource.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl Resource for Comment {}
impl ResponseResult for Comment {}


/// The image representation of the original actor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectActorImage {
    /// A URL that points to a thumbnail photo of the original actor.
    pub url: Option<String>,
}

impl NestedType for ActivityObjectActorImage {}
impl Part for ActivityObjectActorImage {}


/// The age range of the person. Valid ranges are 17 or younger, 18 to 20, and 21 or older. Age is determined from the user's birthday using Western age reckoning.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonAgeRange {
    /// The age range's upper bound, if any. Possible values include, but are not limited to, the following:  
    /// - "17" - for age 17 
    /// - "20" - for age 20
    pub max: Option<i32>,
    /// The age range's lower bound, if any. Possible values include, but are not limited to, the following:  
    /// - "21" - for age 21 
    /// - "18" - for age 18
    pub min: Option<i32>,
}

impl NestedType for PersonAgeRange {}
impl Part for PersonAgeRange {}


/// An object representation of the individual components of a person's name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonName {
    /// The honorific prefixes (such as "Dr." or "Mrs.") for this person.
    #[serde(rename="honorificPrefix")]
    pub honorific_prefix: Option<String>,
    /// The given name (first name) of this person.
    #[serde(rename="givenName")]
    pub given_name: Option<String>,
    /// The middle name of this person.
    #[serde(rename="middleName")]
    pub middle_name: Option<String>,
    /// The family name (last name) of this person.
    #[serde(rename="familyName")]
    pub family_name: Option<String>,
    /// The full name of this person, including middle names, suffixes, etc.
    pub formatted: Option<String>,
    /// The honorific suffixes (such as "Jr.") for this person.
    #[serde(rename="honorificSuffix")]
    pub honorific_suffix: Option<String>,
}

impl NestedType for PersonName {}
impl Part for PersonName {}


/// Image resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachmentsThumbnailsImage {
    /// Image url.
    pub url: Option<String>,
    /// The width, in pixels, of the linked resource.
    pub width: Option<u32>,
    /// Media type of the link.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The height, in pixels, of the linked resource.
    pub height: Option<u32>,
}

impl NestedType for ActivityObjectAttachmentsThumbnailsImage {}
impl Part for ActivityObjectAttachmentsThumbnailsImage {}


/// The physical address of the place.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaceAddress {
    /// The formatted address for display.
    pub formatted: Option<String>,
}

impl NestedType for PlaceAddress {}
impl Part for PlaceAddress {}


/// The object of this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObject {
    /// The HTML-formatted content, which is suitable for display.
    pub content: Option<String>,
    /// The media objects attached to this activity.
    pub attachments: Option<Vec<ActivityObjectAttachments>>,
    /// The content (text) as provided by the author, which is stored without any HTML formatting. When creating or updating an activity, this value must be supplied as plain text in the request.
    #[serde(rename="originalContent")]
    pub original_content: Option<String>,
    /// People who +1'd this activity.
    pub plusoners: Option<ActivityObjectPlusoners>,
    /// If this activity's object is itself another activity, such as when a person reshares an activity, this property specifies the original activity's actor.
    pub actor: Option<ActivityObjectActor>,
    /// People who reshared this activity.
    pub resharers: Option<ActivityObjectResharers>,
    /// The URL that points to the linked resource.
    pub url: Option<String>,
    /// Comments in reply to this activity.
    pub replies: Option<ActivityObjectReplies>,
    /// The ID of the object. When resharing an activity, this is the ID of the activity that is being reshared.
    pub id: Option<String>,
    /// The type of the object. Possible values include, but are not limited to, the following values:  
    /// - "note" - Textual content. 
    /// - "activity" - A Google+ activity.
    #[serde(rename="objectType")]
    pub object_type: Option<String>,
}

impl NestedType for ActivityObject {}
impl Part for ActivityObject {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Place {
    /// The position of the place.
    pub position: Option<PlacePosition>,
    /// Identifies this resource as a place. Value: "plus#place".
    pub kind: Option<String>,
    /// The display name of the place.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The id of the place.
    pub id: Option<String>,
    /// The physical address of the place.
    pub address: Option<PlaceAddress>,
}

impl Part for Place {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *moment* resources.
/// It is not used directly, but through the `Plus` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_plus1 as plus1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use plus1::Plus;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Plus::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`, `list(...)` and `remove(...)`
/// // to build up your call.
/// let rb = hub.moments();
/// # }
/// ```
pub struct MomentMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
}

impl<'a, C, A> MethodsBuilder for MomentMethods<'a, C, A> {}

impl<'a, C, A> MomentMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Record a moment representing a user's action such as making a purchase or commenting on a blog.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The ID of the user to record actions for. The only valid values are "me" and the ID of the authenticated user.
    /// * `collection` - The collection to which to write moments.
    pub fn insert(&self, request: &Moment, user_id: &str, collection: &str) -> MomentInsertCall<'a, C, A> {
        MomentInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _user_id: user_id.to_string(),
            _collection: collection.to_string(),
            _debug: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all of the moments for a particular user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The ID of the user to get moments for. The special value "me" can be used to indicate the authenticated user.
    /// * `collection` - The collection of moments to list.
    pub fn list(&self, user_id: &str, collection: &str) -> MomentListCall<'a, C, A> {
        MomentListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _collection: collection.to_string(),
            _type_: Default::default(),
            _target_url: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a moment.
    /// 
    /// # Arguments
    ///
    /// * `id` - The ID of the moment to delete.
    pub fn remove(&self, id: &str) -> MomentRemoveCall<'a, C, A> {
        MomentRemoveCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the `Plus` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_plus1 as plus1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use plus1::Plus;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Plus::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)` and `search(...)`
/// // to build up your call.
/// let rb = hub.activities();
/// # }
/// ```
pub struct ActivityMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
}

impl<'a, C, A> MethodsBuilder for ActivityMethods<'a, C, A> {}

impl<'a, C, A> ActivityMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Search public activities.
    /// 
    /// # Arguments
    ///
    /// * `query` - Full-text search query string.
    pub fn search(&self, query: &str) -> ActivitySearchCall<'a, C, A> {
        ActivitySearchCall {
            hub: self.hub,
            _query: query.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an activity.
    /// 
    /// # Arguments
    ///
    /// * `activityId` - The ID of the activity to get.
    pub fn get(&self, activity_id: &str) -> ActivityGetCall<'a, C, A> {
        ActivityGetCall {
            hub: self.hub,
            _activity_id: activity_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all of the activities in the specified collection for a particular user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The ID of the user to get activities for. The special value "me" can be used to indicate the authenticated user.
    /// * `collection` - The collection of activities to list.
    pub fn list(&self, user_id: &str, collection: &str) -> ActivityListCall<'a, C, A> {
        ActivityListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _collection: collection.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *comment* resources.
/// It is not used directly, but through the `Plus` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_plus1 as plus1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use plus1::Plus;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Plus::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.comments();
/// # }
/// ```
pub struct CommentMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
}

impl<'a, C, A> MethodsBuilder for CommentMethods<'a, C, A> {}

impl<'a, C, A> CommentMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all of the comments for an activity.
    /// 
    /// # Arguments
    ///
    /// * `activityId` - The ID of the activity to get comments for.
    pub fn list(&self, activity_id: &str) -> CommentListCall<'a, C, A> {
        CommentListCall {
            hub: self.hub,
            _activity_id: activity_id.to_string(),
            _sort_order: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a comment.
    /// 
    /// # Arguments
    ///
    /// * `commentId` - The ID of the comment to get.
    pub fn get(&self, comment_id: &str) -> CommentGetCall<'a, C, A> {
        CommentGetCall {
            hub: self.hub,
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *people* resources.
/// It is not used directly, but through the `Plus` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_plus1 as plus1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use plus1::Plus;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Plus::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)`, `list_by_activity(...)` and `search(...)`
/// // to build up your call.
/// let rb = hub.people();
/// # }
/// ```
pub struct PeopleMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
}

impl<'a, C, A> MethodsBuilder for PeopleMethods<'a, C, A> {}

impl<'a, C, A> PeopleMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Search all public profiles.
    /// 
    /// # Arguments
    ///
    /// * `query` - Specify a query string for full text search of public text in all profiles.
    pub fn search(&self, query: &str) -> PeopleSearchCall<'a, C, A> {
        PeopleSearchCall {
            hub: self.hub,
            _query: query.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all of the people in the specified collection for a particular activity.
    /// 
    /// # Arguments
    ///
    /// * `activityId` - The ID of the activity to get the list of people for.
    /// * `collection` - The collection of people to list.
    pub fn list_by_activity(&self, activity_id: &str, collection: &str) -> PeopleListByActivityCall<'a, C, A> {
        PeopleListByActivityCall {
            hub: self.hub,
            _activity_id: activity_id.to_string(),
            _collection: collection.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all of the people in the specified collection.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Get the collection of people for the person identified. Use "me" to indicate the authenticated user.
    /// * `collection` - The collection of people to list.
    pub fn list(&self, user_id: &str, collection: &str) -> PeopleListCall<'a, C, A> {
        PeopleListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _collection: collection.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a person's profile. If your app uses scope https://www.googleapis.com/auth/plus.login, this method is guaranteed to return ageRange and language.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The ID of the person to get the profile for. The special value "me" can be used to indicate the authenticated user.
    pub fn get(&self, user_id: &str) -> PeopleGetCall<'a, C, A> {
        PeopleGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Record a moment representing a user's action such as making a purchase or commenting on a blog.
///
/// A builder for the *insert* method supported by a *moment* resource.
/// It is not used directly, but through a `MomentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// use plus1::Moment;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Moment::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.moments().insert(&req, "userId", "collection")
///              .debug(false)
///              .doit();
/// # }
/// ```
pub struct MomentInsertCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _request: Moment,
    _user_id: String,
    _collection: String,
    _debug: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MomentInsertCall<'a, C, A> {}

impl<'a, C, A> MomentInsertCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Moment)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.moments.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("userId", self._user_id.to_string()));
        params.push(("collection", self._collection.to_string()));
        if let Some(value) = self._debug {
            params.push(("debug", value.to_string()));
        }
        for &field in ["alt", "userId", "collection", "debug"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/plus/v1/people/{userId}/moments/{collection}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{collection}", "collection")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["userId", "collection"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: &Moment) -> MomentInsertCall<'a, C, A> {
        self._request = new_value.clone();
        self
    }
    /// The ID of the user to record actions for. The only valid values are "me" and the ID of the authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> MomentInsertCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The collection to which to write moments.
    ///
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn collection(mut self, new_value: &str) -> MomentInsertCall<'a, C, A> {
        self._collection = new_value.to_string();
        self
    }
    /// Return the moment as written. Should be used only for debugging.
    ///
    /// Sets the *debug* query property to the given value.
    pub fn debug(mut self, new_value: bool) -> MomentInsertCall<'a, C, A> {
        self._debug = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MomentInsertCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MomentInsertCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MomentInsertCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List all of the moments for a particular user.
///
/// A builder for the *list* method supported by a *moment* resource.
/// It is not used directly, but through a `MomentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.moments().list("userId", "collection")
///              .type_("no")
///              .target_url("justo")
///              .page_token("justo")
///              .max_results(67)
///              .doit();
/// # }
/// ```
pub struct MomentListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _user_id: String,
    _collection: String,
    _type_: Option<String>,
    _target_url: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MomentListCall<'a, C, A> {}

impl<'a, C, A> MomentListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, MomentsFeed)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.moments.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("userId", self._user_id.to_string()));
        params.push(("collection", self._collection.to_string()));
        if let Some(value) = self._type_ {
            params.push(("type", value.to_string()));
        }
        if let Some(value) = self._target_url {
            params.push(("targetUrl", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "userId", "collection", "type", "targetUrl", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/plus/v1/people/{userId}/moments/{collection}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{collection}", "collection")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["userId", "collection"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the user to get moments for. The special value "me" can be used to indicate the authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> MomentListCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The collection of moments to list.
    ///
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn collection(mut self, new_value: &str) -> MomentListCall<'a, C, A> {
        self._collection = new_value.to_string();
        self
    }
    /// Only moments of this type will be returned.
    ///
    /// Sets the *type* query property to the given value.
    pub fn type_(mut self, new_value: &str) -> MomentListCall<'a, C, A> {
        self._type_ = Some(new_value.to_string());
        self
    }
    /// Only moments containing this targetUrl will be returned.
    ///
    /// Sets the *target url* query property to the given value.
    pub fn target_url(mut self, new_value: &str) -> MomentListCall<'a, C, A> {
        self._target_url = Some(new_value.to_string());
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> MomentListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of moments to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> MomentListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MomentListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MomentListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MomentListCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Delete a moment.
///
/// A builder for the *remove* method supported by a *moment* resource.
/// It is not used directly, but through a `MomentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.moments().remove("id")
///              .doit();
/// # }
/// ```
pub struct MomentRemoveCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for MomentRemoveCall<'a, C, A> {}

impl<'a, C, A> MomentRemoveCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.moments.remove", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        for &field in ["id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/plus/v1/moments/{id}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{id}", "id")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["id"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the moment to delete.
    ///
    /// Sets the *id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn id(mut self, new_value: &str) -> MomentRemoveCall<'a, C, A> {
        self._id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MomentRemoveCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MomentRemoveCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MomentRemoveCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Search public activities.
///
/// A builder for the *search* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().search("query")
///              .page_token("ipsum")
///              .order_by("Lorem")
///              .max_results(80)
///              .language("duo")
///              .doit();
/// # }
/// ```
pub struct ActivitySearchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _query: String,
    _page_token: Option<String>,
    _order_by: Option<String>,
    _max_results: Option<u32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ActivitySearchCall<'a, C, A> {}

impl<'a, C, A> ActivitySearchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ActivityFeed)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.activities.search", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("query", self._query.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "query", "pageToken", "orderBy", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/plus/v1/activities".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Full-text search query string.
    ///
    /// Sets the *query* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn query(mut self, new_value: &str) -> ActivitySearchCall<'a, C, A> {
        self._query = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response. This token can be of any length.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ActivitySearchCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Specifies how to order search results.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> ActivitySearchCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The maximum number of activities to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ActivitySearchCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Specify the preferred language to search with. See search language codes for available values.
    ///
    /// Sets the *language* query property to the given value.
    pub fn language(mut self, new_value: &str) -> ActivitySearchCall<'a, C, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActivitySearchCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ActivitySearchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ActivitySearchCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Get an activity.
///
/// A builder for the *get* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().get("activityId")
///              .doit();
/// # }
/// ```
pub struct ActivityGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _activity_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ActivityGetCall<'a, C, A> {}

impl<'a, C, A> ActivityGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Activity)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.activities.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("activityId", self._activity_id.to_string()));
        for &field in ["alt", "activityId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/plus/v1/activities/{activityId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{activityId}", "activityId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["activityId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the activity to get.
    ///
    /// Sets the *activity id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn activity_id(mut self, new_value: &str) -> ActivityGetCall<'a, C, A> {
        self._activity_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActivityGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ActivityGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ActivityGetCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List all of the activities in the specified collection for a particular user.
///
/// A builder for the *list* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().list("userId", "collection")
///              .page_token("eos")
///              .max_results(20)
///              .doit();
/// # }
/// ```
pub struct ActivityListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _user_id: String,
    _collection: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ActivityListCall<'a, C, A> {}

impl<'a, C, A> ActivityListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ActivityFeed)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.activities.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("userId", self._user_id.to_string()));
        params.push(("collection", self._collection.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "userId", "collection", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/plus/v1/people/{userId}/activities/{collection}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{collection}", "collection")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["userId", "collection"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the user to get activities for. The special value "me" can be used to indicate the authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The collection of activities to list.
    ///
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn collection(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._collection = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ActivityListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of activities to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ActivityListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActivityListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ActivityListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ActivityListCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List all of the comments for an activity.
///
/// A builder for the *list* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().list("activityId")
///              .sort_order("dolor")
///              .page_token("eirmod")
///              .max_results(58)
///              .doit();
/// # }
/// ```
pub struct CommentListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _activity_id: String,
    _sort_order: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CommentListCall<'a, C, A> {}

impl<'a, C, A> CommentListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CommentFeed)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.comments.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("activityId", self._activity_id.to_string()));
        if let Some(value) = self._sort_order {
            params.push(("sortOrder", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "activityId", "sortOrder", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/plus/v1/activities/{activityId}/comments".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{activityId}", "activityId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["activityId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the activity to get comments for.
    ///
    /// Sets the *activity id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn activity_id(mut self, new_value: &str) -> CommentListCall<'a, C, A> {
        self._activity_id = new_value.to_string();
        self
    }
    /// The order in which to sort the list of comments.
    ///
    /// Sets the *sort order* query property to the given value.
    pub fn sort_order(mut self, new_value: &str) -> CommentListCall<'a, C, A> {
        self._sort_order = Some(new_value.to_string());
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CommentListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of comments to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> CommentListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CommentListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CommentListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> CommentListCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Get a comment.
///
/// A builder for the *get* method supported by a *comment* resource.
/// It is not used directly, but through a `CommentMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().get("commentId")
///              .doit();
/// # }
/// ```
pub struct CommentGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _comment_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for CommentGetCall<'a, C, A> {}

impl<'a, C, A> CommentGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Comment)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.comments.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("commentId", self._comment_id.to_string()));
        for &field in ["alt", "commentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/plus/v1/comments/{commentId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{commentId}", "commentId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["commentId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the comment to get.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> CommentGetCall<'a, C, A> {
        self._comment_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> CommentGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> CommentGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> CommentGetCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Search all public profiles.
///
/// A builder for the *search* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().search("query")
///              .page_token("labore")
///              .max_results(62)
///              .language("dolore")
///              .doit();
/// # }
/// ```
pub struct PeopleSearchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _query: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleSearchCall<'a, C, A> {}

impl<'a, C, A> PeopleSearchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PeopleFeed)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.people.search", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("query", self._query.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "query", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/plus/v1/people".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Specify a query string for full text search of public text in all profiles.
    ///
    /// Sets the *query* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn query(mut self, new_value: &str) -> PeopleSearchCall<'a, C, A> {
        self._query = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response. This token can be of any length.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PeopleSearchCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of people to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> PeopleSearchCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Specify the preferred language to search with. See search language codes for available values.
    ///
    /// Sets the *language* query property to the given value.
    pub fn language(mut self, new_value: &str) -> PeopleSearchCall<'a, C, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PeopleSearchCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleSearchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PeopleSearchCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List all of the people in the specified collection for a particular activity.
///
/// A builder for the *listByActivity* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().list_by_activity("activityId", "collection")
///              .page_token("accusam")
///              .max_results(45)
///              .doit();
/// # }
/// ```
pub struct PeopleListByActivityCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _activity_id: String,
    _collection: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleListByActivityCall<'a, C, A> {}

impl<'a, C, A> PeopleListByActivityCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PeopleFeed)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.people.listByActivity", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("activityId", self._activity_id.to_string()));
        params.push(("collection", self._collection.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "activityId", "collection", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/plus/v1/activities/{activityId}/people/{collection}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{activityId}", "activityId"), ("{collection}", "collection")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["activityId", "collection"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the activity to get the list of people for.
    ///
    /// Sets the *activity id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn activity_id(mut self, new_value: &str) -> PeopleListByActivityCall<'a, C, A> {
        self._activity_id = new_value.to_string();
        self
    }
    /// The collection of people to list.
    ///
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn collection(mut self, new_value: &str) -> PeopleListByActivityCall<'a, C, A> {
        self._collection = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PeopleListByActivityCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of people to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> PeopleListByActivityCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PeopleListByActivityCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleListByActivityCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PeopleListByActivityCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// List all of the people in the specified collection.
///
/// A builder for the *list* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().list("userId", "collection")
///              .page_token("duo")
///              .order_by("et")
///              .max_results(61)
///              .doit();
/// # }
/// ```
pub struct PeopleListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _user_id: String,
    _collection: String,
    _page_token: Option<String>,
    _order_by: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleListCall<'a, C, A> {}

impl<'a, C, A> PeopleListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PeopleFeed)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.people.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("userId", self._user_id.to_string()));
        params.push(("collection", self._collection.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "userId", "collection", "pageToken", "orderBy", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/plus/v1/people/{userId}/people/{collection}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{collection}", "collection")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["userId", "collection"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Get the collection of people for the person identified. Use "me" to indicate the authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> PeopleListCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The collection of people to list.
    ///
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn collection(mut self, new_value: &str) -> PeopleListCall<'a, C, A> {
        self._collection = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PeopleListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The order to return people in.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> PeopleListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The maximum number of people to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> PeopleListCall<'a, C, A> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PeopleListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PeopleListCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


/// Get a person's profile. If your app uses scope https://www.googleapis.com/auth/plus.login, this method is guaranteed to return ageRange and language.
///
/// A builder for the *get* method supported by a *people* resource.
/// It is not used directly, but through a `PeopleMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_plus1 as plus1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use plus1::Plus;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Plus::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().get("userId")
///              .doit();
/// # }
/// ```
pub struct PeopleGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Plus<C, A>,
    _user_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for PeopleGetCall<'a, C, A> {}

impl<'a, C, A> PeopleGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Person)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "plus.people.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("userId", self._user_id.to_string()));
        for &field in ["alt", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/plus/v1/people/{userId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Login.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["userId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_ref()))));
        }



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_ref())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep_ms(d.num_milliseconds() as u32);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep_ms(d.num_milliseconds() as u32);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the person to get the profile for. The special value "me" can be used to indicate the authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> PeopleGetCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PeopleGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PeopleGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Login`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PeopleGetCall<'a, C, A> 
                                                        where T: AsRef<str> {
        self._scopes.insert(scope.as_ref().to_string(), ());
        self
    }
}


