#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GalleryChangeType {
    ///The contents of the gallery have changed.
    ContentsChanged = "contents_changed",
    ///The watch has been dropped because the device has been detached, the gallery permission has been removed, or any other reason.
    WatchDropped = "watch_dropped",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GetMediaFileSystemsInteractivity {
    ///Do not act interactively.
    No = "no",
    ///Ask the user to manage permitted media galleries.
    Yes = "yes",
    ///Ask the user to manage permitted galleries only if the return set would otherwise be empty.
    IfNeeded = "if_needed",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GetMetadataType {
    ///Retrieve the mime type, metadata tags, and attached images.
    All = "all",
    ///Retrieve only the mime type and the metadata tags.
    MimeTypeAndTags = "mimeTypeAndTags",
    ///Retrieve only the mime type.
    MimeTypeOnly = "mimeTypeOnly",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GalleryChangeDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GalleryChangeDetails;
    ///Get the `galleryId` field of this object.
    #[wasm_bindgen(method, getter = "galleryId")]
    pub fn get_gallery_id(this: &GalleryChangeDetails) -> String;
    ///Change the `galleryId` field of this object.
    #[wasm_bindgen(method, setter = "galleryId")]
    pub fn set_gallery_id(this: &GalleryChangeDetails, val: String);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &GalleryChangeDetails) -> GalleryChangeType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &GalleryChangeDetails, val: GalleryChangeType);
}
impl GalleryChangeDetails {
    ///Construct a new `GalleryChangeDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_gallery_id()` instead."]
    pub fn gallery_id(&mut self, val: String) -> &mut Self {
        self.set_gallery_id(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: GalleryChangeType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for GalleryChangeDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GalleryChangeDetails`.
pub struct GalleryChangeDetailsData {
    ///Identifies the modified gallery.
    pub gallery_id: String,
    ///Type of change event.
    pub r#type: GalleryChangeType,
}
#[cfg(feature = "serde")]
impl From<&GalleryChangeDetails> for GalleryChangeDetailsData {
    fn from(val: &GalleryChangeDetails) -> Self {
        Self {
            gallery_id: val.get_gallery_id(),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MediaFileSystemsDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MediaFileSystemsDetails;
    ///Get the `interactive` field of this object.
    #[wasm_bindgen(method, getter = "interactive")]
    pub fn get_interactive(
        this: &MediaFileSystemsDetails,
    ) -> Option<GetMediaFileSystemsInteractivity>;
    ///Change the `interactive` field of this object.
    #[wasm_bindgen(method, setter = "interactive")]
    pub fn set_interactive(this: &MediaFileSystemsDetails, val: GetMediaFileSystemsInteractivity);
}
impl MediaFileSystemsDetails {
    ///Construct a new `MediaFileSystemsDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_interactive()` instead."]
    pub fn interactive(&mut self, val: GetMediaFileSystemsInteractivity) -> &mut Self {
        self.set_interactive(val);
        self
    }
}
impl Default for MediaFileSystemsDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MediaFileSystemsDetails`.
pub struct MediaFileSystemsDetailsData {
    ///Whether to prompt the user for permission to additional media galleries before returning the permitted set. Default is silent. If the value 'yes' is passed, or if the application has not been granted access to any media galleries and the value 'if_needed' is passed, then the media gallery configuration dialog will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<GetMediaFileSystemsInteractivity>,
}
#[cfg(feature = "serde")]
impl From<&MediaFileSystemsDetails> for MediaFileSystemsDetailsData {
    fn from(val: &MediaFileSystemsDetails) -> Self {
        Self {
            interactive: val.get_interactive(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MediaMetadataOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MediaMetadataOptions;
    ///Get the `metadataType` field of this object.
    #[wasm_bindgen(method, getter = "metadataType")]
    pub fn get_metadata_type(this: &MediaMetadataOptions) -> Option<GetMetadataType>;
    ///Change the `metadataType` field of this object.
    #[wasm_bindgen(method, setter = "metadataType")]
    pub fn set_metadata_type(this: &MediaMetadataOptions, val: GetMetadataType);
}
impl MediaMetadataOptions {
    ///Construct a new `MediaMetadataOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_metadata_type()` instead."]
    pub fn metadata_type(&mut self, val: GetMetadataType) -> &mut Self {
        self.set_metadata_type(val);
        self
    }
}
impl Default for MediaMetadataOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MediaMetadataOptions`.
pub struct MediaMetadataOptionsData {
    ///Specifies which subset of the metadata to retrieve. Defaults to 'all' if the option is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_type: Option<GetMetadataType>,
}
#[cfg(feature = "serde")]
impl From<&MediaMetadataOptions> for MediaMetadataOptionsData {
    fn from(val: &MediaMetadataOptions) -> Self {
        Self {
            metadata_type: val.get_metadata_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MediaFileSystemMetadata")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MediaFileSystemMetadata;
    ///Get the `deviceId` field of this object.
    #[wasm_bindgen(method, getter = "deviceId")]
    pub fn get_device_id(this: &MediaFileSystemMetadata) -> Option<String>;
    ///Change the `deviceId` field of this object.
    #[wasm_bindgen(method, setter = "deviceId")]
    pub fn set_device_id(this: &MediaFileSystemMetadata, val: String);
    ///Get the `galleryId` field of this object.
    #[wasm_bindgen(method, getter = "galleryId")]
    pub fn get_gallery_id(this: &MediaFileSystemMetadata) -> String;
    ///Change the `galleryId` field of this object.
    #[wasm_bindgen(method, setter = "galleryId")]
    pub fn set_gallery_id(this: &MediaFileSystemMetadata, val: String);
    ///Get the `isAvailable` field of this object.
    #[wasm_bindgen(method, getter = "isAvailable")]
    pub fn get_is_available(this: &MediaFileSystemMetadata) -> bool;
    ///Change the `isAvailable` field of this object.
    #[wasm_bindgen(method, setter = "isAvailable")]
    pub fn set_is_available(this: &MediaFileSystemMetadata, val: bool);
    ///Get the `isMediaDevice` field of this object.
    #[wasm_bindgen(method, getter = "isMediaDevice")]
    pub fn get_is_media_device(this: &MediaFileSystemMetadata) -> bool;
    ///Change the `isMediaDevice` field of this object.
    #[wasm_bindgen(method, setter = "isMediaDevice")]
    pub fn set_is_media_device(this: &MediaFileSystemMetadata, val: bool);
    ///Get the `isRemovable` field of this object.
    #[wasm_bindgen(method, getter = "isRemovable")]
    pub fn get_is_removable(this: &MediaFileSystemMetadata) -> bool;
    ///Change the `isRemovable` field of this object.
    #[wasm_bindgen(method, setter = "isRemovable")]
    pub fn set_is_removable(this: &MediaFileSystemMetadata, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &MediaFileSystemMetadata) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &MediaFileSystemMetadata, val: String);
}
impl MediaFileSystemMetadata {
    ///Construct a new `MediaFileSystemMetadata`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_device_id()` instead."]
    pub fn device_id(&mut self, val: String) -> &mut Self {
        self.set_device_id(val);
        self
    }
    #[deprecated = "Use `set_gallery_id()` instead."]
    pub fn gallery_id(&mut self, val: String) -> &mut Self {
        self.set_gallery_id(val);
        self
    }
    #[deprecated = "Use `set_is_available()` instead."]
    pub fn is_available(&mut self, val: bool) -> &mut Self {
        self.set_is_available(val);
        self
    }
    #[deprecated = "Use `set_is_media_device()` instead."]
    pub fn is_media_device(&mut self, val: bool) -> &mut Self {
        self.set_is_media_device(val);
        self
    }
    #[deprecated = "Use `set_is_removable()` instead."]
    pub fn is_removable(&mut self, val: bool) -> &mut Self {
        self.set_is_removable(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for MediaFileSystemMetadata {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MediaFileSystemMetadata`.
pub struct MediaFileSystemMetadataData {
    ///If the media gallery is on a removable device, a unique id for the device while the device is online.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    ///A unique and persistent id for the media gallery.
    pub gallery_id: String,
    ///True if the device is currently available.
    pub is_available: bool,
    ///True if the device the media gallery is on was detected as a media device. i.e. a PTP or MTP device, or a DCIM directory is present.
    pub is_media_device: bool,
    ///True if the media gallery is on a removable device.
    pub is_removable: bool,
    ///The name of the file system.
    pub name: String,
}
#[cfg(feature = "serde")]
impl From<&MediaFileSystemMetadata> for MediaFileSystemMetadataData {
    fn from(val: &MediaFileSystemMetadata) -> Self {
        Self {
            device_id: val.get_device_id(),
            gallery_id: val.get_gallery_id(),
            is_available: val.get_is_available(),
            is_media_device: val.get_is_media_device(),
            is_removable: val.get_is_removable(),
            name: val.get_name(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "StreamInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type StreamInfo;
    ///Get the `tags` field of this object.
    #[wasm_bindgen(method, getter = "tags")]
    pub fn get_tags(this: &StreamInfo) -> Object;
    ///Change the `tags` field of this object.
    #[wasm_bindgen(method, setter = "tags")]
    pub fn set_tags(this: &StreamInfo, val: &Object);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &StreamInfo) -> String;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &StreamInfo, val: String);
}
impl StreamInfo {
    ///Construct a new `StreamInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_tags()` instead."]
    pub fn tags(&mut self, val: &Object) -> &mut Self {
        self.set_tags(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: String) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for StreamInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `StreamInfo`.
pub struct StreamInfoData {
    ///An unfiltered string-string dictionary of tags for the stream.
    pub tags: serde_json::Value,
    ///Describes format of container or codec of stream, i.e. "mp3", "h264".
    pub r#type: String,
}
#[cfg(feature = "serde")]
impl From<&StreamInfo> for StreamInfoData {
    fn from(val: &StreamInfo) -> Self {
        Self {
            tags: serde_wasm_bindgen::from_value(val.get_tags().into()).unwrap_or_default(),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MediaMetadata")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MediaMetadata;
    ///Get the `album` field of this object.
    #[wasm_bindgen(method, getter = "album")]
    pub fn get_album(this: &MediaMetadata) -> Option<String>;
    ///Change the `album` field of this object.
    #[wasm_bindgen(method, setter = "album")]
    pub fn set_album(this: &MediaMetadata, val: String);
    ///Get the `artist` field of this object.
    #[wasm_bindgen(method, getter = "artist")]
    pub fn get_artist(this: &MediaMetadata) -> Option<String>;
    ///Change the `artist` field of this object.
    #[wasm_bindgen(method, setter = "artist")]
    pub fn set_artist(this: &MediaMetadata, val: String);
    ///Get the `attachedImages` field of this object.
    #[wasm_bindgen(method, getter = "attachedImages")]
    pub fn get_attached_images(this: &MediaMetadata) -> Array;
    ///Change the `attachedImages` field of this object.
    #[wasm_bindgen(method, setter = "attachedImages")]
    pub fn set_attached_images(this: &MediaMetadata, val: &Array);
    ///Get the `comment` field of this object.
    #[wasm_bindgen(method, getter = "comment")]
    pub fn get_comment(this: &MediaMetadata) -> Option<String>;
    ///Change the `comment` field of this object.
    #[wasm_bindgen(method, setter = "comment")]
    pub fn set_comment(this: &MediaMetadata, val: String);
    ///Get the `copyright` field of this object.
    #[wasm_bindgen(method, getter = "copyright")]
    pub fn get_copyright(this: &MediaMetadata) -> Option<String>;
    ///Change the `copyright` field of this object.
    #[wasm_bindgen(method, setter = "copyright")]
    pub fn set_copyright(this: &MediaMetadata, val: String);
    ///Get the `disc` field of this object.
    #[wasm_bindgen(method, getter = "disc")]
    pub fn get_disc(this: &MediaMetadata) -> Option<i32>;
    ///Change the `disc` field of this object.
    #[wasm_bindgen(method, setter = "disc")]
    pub fn set_disc(this: &MediaMetadata, val: i32);
    ///Get the `duration` field of this object.
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &MediaMetadata) -> Option<f64>;
    ///Change the `duration` field of this object.
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration(this: &MediaMetadata, val: f64);
    ///Get the `genre` field of this object.
    #[wasm_bindgen(method, getter = "genre")]
    pub fn get_genre(this: &MediaMetadata) -> Option<String>;
    ///Change the `genre` field of this object.
    #[wasm_bindgen(method, setter = "genre")]
    pub fn set_genre(this: &MediaMetadata, val: String);
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &MediaMetadata) -> Option<i32>;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &MediaMetadata, val: i32);
    ///Get the `language` field of this object.
    #[wasm_bindgen(method, getter = "language")]
    pub fn get_language(this: &MediaMetadata) -> Option<String>;
    ///Change the `language` field of this object.
    #[wasm_bindgen(method, setter = "language")]
    pub fn set_language(this: &MediaMetadata, val: String);
    ///Get the `mimeType` field of this object.
    #[wasm_bindgen(method, getter = "mimeType")]
    pub fn get_mime_type(this: &MediaMetadata) -> String;
    ///Change the `mimeType` field of this object.
    #[wasm_bindgen(method, setter = "mimeType")]
    pub fn set_mime_type(this: &MediaMetadata, val: String);
    ///Get the `rawTags` field of this object.
    #[wasm_bindgen(method, getter = "rawTags")]
    pub fn get_raw_tags(this: &MediaMetadata) -> Array;
    ///Change the `rawTags` field of this object.
    #[wasm_bindgen(method, setter = "rawTags")]
    pub fn set_raw_tags(this: &MediaMetadata, val: &Array);
    ///Get the `rotation` field of this object.
    #[wasm_bindgen(method, getter = "rotation")]
    pub fn get_rotation(this: &MediaMetadata) -> Option<i32>;
    ///Change the `rotation` field of this object.
    #[wasm_bindgen(method, setter = "rotation")]
    pub fn set_rotation(this: &MediaMetadata, val: i32);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &MediaMetadata) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &MediaMetadata, val: String);
    ///Get the `track` field of this object.
    #[wasm_bindgen(method, getter = "track")]
    pub fn get_track(this: &MediaMetadata) -> Option<i32>;
    ///Change the `track` field of this object.
    #[wasm_bindgen(method, setter = "track")]
    pub fn set_track(this: &MediaMetadata, val: i32);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &MediaMetadata) -> Option<i32>;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &MediaMetadata, val: i32);
}
impl MediaMetadata {
    ///Construct a new `MediaMetadata`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_album()` instead."]
    pub fn album(&mut self, val: String) -> &mut Self {
        self.set_album(val);
        self
    }
    #[deprecated = "Use `set_artist()` instead."]
    pub fn artist(&mut self, val: String) -> &mut Self {
        self.set_artist(val);
        self
    }
    #[deprecated = "Use `set_attached_images()` instead."]
    pub fn attached_images(&mut self, val: &Array) -> &mut Self {
        self.set_attached_images(val);
        self
    }
    #[deprecated = "Use `set_comment()` instead."]
    pub fn comment(&mut self, val: String) -> &mut Self {
        self.set_comment(val);
        self
    }
    #[deprecated = "Use `set_copyright()` instead."]
    pub fn copyright(&mut self, val: String) -> &mut Self {
        self.set_copyright(val);
        self
    }
    #[deprecated = "Use `set_disc()` instead."]
    pub fn disc(&mut self, val: i32) -> &mut Self {
        self.set_disc(val);
        self
    }
    #[deprecated = "Use `set_duration()` instead."]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.set_duration(val);
        self
    }
    #[deprecated = "Use `set_genre()` instead."]
    pub fn genre(&mut self, val: String) -> &mut Self {
        self.set_genre(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_language()` instead."]
    pub fn language(&mut self, val: String) -> &mut Self {
        self.set_language(val);
        self
    }
    #[deprecated = "Use `set_mime_type()` instead."]
    pub fn mime_type(&mut self, val: String) -> &mut Self {
        self.set_mime_type(val);
        self
    }
    #[deprecated = "Use `set_raw_tags()` instead."]
    pub fn raw_tags(&mut self, val: &Array) -> &mut Self {
        self.set_raw_tags(val);
        self
    }
    #[deprecated = "Use `set_rotation()` instead."]
    pub fn rotation(&mut self, val: i32) -> &mut Self {
        self.set_rotation(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_track()` instead."]
    pub fn track(&mut self, val: i32) -> &mut Self {
        self.set_track(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for MediaMetadata {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MediaMetadata`.
pub struct MediaMetadataData {
    ///Defined for audio and video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    ///The images embedded in the media file's metadata. This is most often used for album art or video thumbnails.
    pub attached_images: Vec<serde_json::Value>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc: Option<i32>,
    ///Defined for audio and video. In seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    ///Defined for video. In pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    ///The browser sniffed mime type.
    pub mime_type: String,
    ///All the metadata in the media file. For formats with multiple streams, stream order will be preserved. Container metadata is the first element.
    pub raw_tags: Vec<StreamInfoData>,
    ///Defined for video. In degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<i32>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<i32>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&MediaMetadata> for MediaMetadataData {
    fn from(val: &MediaMetadata) -> Self {
        Self {
            album: val.get_album(),
            artist: val.get_artist(),
            attached_images: serde_wasm_bindgen::from_value(val.get_attached_images().into())
                .unwrap_or_default(),
            comment: val.get_comment(),
            copyright: val.get_copyright(),
            disc: val.get_disc(),
            duration: val.get_duration(),
            genre: val.get_genre(),
            height: val.get_height(),
            language: val.get_language(),
            mime_type: val.get_mime_type(),
            raw_tags: serde_wasm_bindgen::from_value(val.get_raw_tags().into()).unwrap_or_default(),
            rotation: val.get_rotation(),
            title: val.get_title(),
            track: val.get_track(),
            width: val.get_width(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AddGalleryWatchResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AddGalleryWatchResult;
    ///Get the `galleryId` field of this object.
    #[wasm_bindgen(method, getter = "galleryId")]
    pub fn get_gallery_id(this: &AddGalleryWatchResult) -> String;
    ///Change the `galleryId` field of this object.
    #[wasm_bindgen(method, setter = "galleryId")]
    pub fn set_gallery_id(this: &AddGalleryWatchResult, val: String);
    ///Get the `success` field of this object.
    #[wasm_bindgen(method, getter = "success")]
    pub fn get_success(this: &AddGalleryWatchResult) -> bool;
    ///Change the `success` field of this object.
    #[wasm_bindgen(method, setter = "success")]
    pub fn set_success(this: &AddGalleryWatchResult, val: bool);
}
impl AddGalleryWatchResult {
    ///Construct a new `AddGalleryWatchResult`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_gallery_id()` instead."]
    pub fn gallery_id(&mut self, val: String) -> &mut Self {
        self.set_gallery_id(val);
        self
    }
    #[deprecated = "Use `set_success()` instead."]
    pub fn success(&mut self, val: bool) -> &mut Self {
        self.set_success(val);
        self
    }
}
impl Default for AddGalleryWatchResult {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AddGalleryWatchResult`.
pub struct AddGalleryWatchResultData {
    ///
    pub gallery_id: String,
    ///
    pub success: bool,
}
#[cfg(feature = "serde")]
impl From<&AddGalleryWatchResult> for AddGalleryWatchResultData {
    fn from(val: &AddGalleryWatchResult) -> Self {
        Self {
            gallery_id: val.get_gallery_id(),
            success: val.get_success(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Get the media galleries configured in this user agent. If none are configured or available, the callback will receive an empty array.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "mediaGalleries"],
        js_name = "getMediaFileSystems"
    )]
    pub fn get_media_file_systems(details: Option<MediaFileSystemsDetails>) -> Promise;
    ///Present a directory picker to the user and add the selected directory as a gallery. If the user cancels the picker, selectedFileSystemName will be empty. A user gesture is required for the dialog to display. Without a user gesture, the callback will run as though the user canceled.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "mediaGalleries"],
        js_name = "addUserSelectedFolder"
    )]
    pub fn add_user_selected_folder() -> Promise;
    ///Get metadata about a specific media file system.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "mediaGalleries"],
        js_name = "getMediaFileSystemMetadata"
    )]
    pub fn get_media_file_system_metadata(media_file_system: Object) -> MediaFileSystemMetadata;
    ///Gets the media-specific metadata for a media file. This should work for files in media galleries as well as other DOM filesystems.
    #[wasm_bindgen(js_namespace = ["chrome", "mediaGalleries"], js_name = "getMetadata")]
    pub fn get_metadata(media_file: Object, options: Option<MediaMetadataOptions>) -> Promise;
    ///Adds a gallery watch for the gallery with the specified gallery ID. The given callback is then fired with a success or failure result.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "mediaGalleries"],
        js_name = "addGalleryWatch"
    )]
    pub fn add_gallery_watch(gallery_id: String) -> Promise;
    ///Removes a gallery watch for the gallery with the specified gallery ID.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "mediaGalleries"],
        js_name = "removeGalleryWatch"
    )]
    pub fn remove_gallery_watch(gallery_id: String);
    ///Fired when a media gallery is changed or a gallery watch is dropped.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "mediaGalleries",
        "onGalleryChanged"],
        js_name = "addListener"
    )]
    pub fn on_gallery_changed_add_listener(callback: &Function);
}
