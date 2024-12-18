// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::api;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: api = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Api {
    openapi: String,

    info: Info,

    external_docs: ExternalDocs,

    servers: Vec<Server>,

    paths: Paths,

    components: Components,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    schemas: Schemas,

    security_schemes: SecuritySchemes,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Schemas {
    error: Error,

    user_lite: UserLite,

    user_detailed_not_me_only: UserDetailedNotMeOnly,

    me_detailed_only: MeDetailedOnly,

    user_detailed_not_me: MeDetailed,

    me_detailed: MeDetailed,

    user_detailed: UserClass,

    user: UserClass,

    user_list: UserList,

    ad: Ad,

    announcement: Announcement,

    app: App,

    note: Note,

    note_reaction: NoteReaction,

    note_favorite: NoteFavorite,

    notification: Notification,

    drive_file: DriveFile,

    drive_folder: DriveFolder,

    following: Following,

    muting: Muting,

    renote_muting: Muting,

    blocking: Blocking,

    hashtag: Hashtag,

    invite_code: InviteCode,

    page: Page,

    page_block: PageBlock,

    channel: Channel,

    queue_count: QueueCount,

    antenna: Antenna,

    clip: Clip,

    federation_instance: FederationInstance,

    gallery_post: GalleryPost,

    emoji_simple: EmojiSimple,

    emoji_detailed: EmojiDetailed,

    flash: Flash,

    signin: Signin,

    role_cond_formula_logics: RoleCondFormulaLogics,

    role_cond_formula_value_not: RoleCondFormulaValueNot,

    role_cond_formula_value_is_local_or_remote: RoleCondFormulaValueIsLocalOrRemoteClass,

    role_cond_formula_value_user_setting_boolean_schema: RoleCondFormulaValueIsLocalOrRemoteClass,

    role_cond_formula_value_assigned_role: RoleCondFormulaValueAssignedRole,

    role_cond_formula_value_created: RoleCondFormulaValueCreated,

    role_cond_formula_followers_or_following_or_notes: RoleCondFormulaFollowersOrFollowingOrNotes,

    role_cond_formula_value: RoleCondFormulaValue,

    role_lite: RoleLite,

    role: Role,

    role_policies: RolePolicies,

    reversi_game_lite: ReversiGame,

    reversi_game_detailed: ReversiGame,

    meta_lite: MetaLite,

    meta_detailed_only: MetaDetailedOnly,

    meta_detailed: MeDetailed,

    system_webhook: SystemWebhookClass,

    abuse_report_notification_recipient: AbuseReportNotificationRecipient,
}

#[derive(Serialize, Deserialize)]
pub struct AbuseReportNotificationRecipient {
    #[serde(rename = "type")]
    abuse_report_notification_recipient_type: TentacledType,

    properties: AbuseReportNotificationRecipientProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TentacledType {
    Boolean,

    Integer,

    Null,

    Number,

    Object,

    String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbuseReportNotificationRecipientProperties {
    id: IdElement,

    is_active: IdElement,

    updated_at: StartsAtClass,

    name: IdElement,

    method: Method,

    user_id: IdElement,

    user: SystemWebhook,

    system_webhook_id: IdElement,

    system_webhook: SystemWebhook,
}

#[derive(Serialize, Deserialize)]
pub struct IdElement {
    #[serde(rename = "type")]
    id_type: TentacledType,
}

#[derive(Serialize, Deserialize)]
pub struct Method {
    #[serde(rename = "type")]
    method_type: TentacledType,

    #[serde(rename = "enum")]
    method_enum: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemWebhook {
    #[serde(rename = "type")]
    system_webhook_type: TentacledType,

    all_of: Vec<OneOf>,
}

#[derive(Serialize, Deserialize)]
pub struct OneOf {
    #[serde(rename = "$ref")]
    one_of_ref: String,
}

#[derive(Serialize, Deserialize)]
pub struct StartsAtClass {
    #[serde(rename = "type")]
    updated_at_type: TentacledType,

    format: Option<Format>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Format {
    Bytes,

    #[serde(rename = "date-time")]
    DateTime,

    Id,

    Md5,

    #[serde(rename = "misskey:id")]
    MisskeyId,

    Uri,

    Url,
}

#[derive(Serialize, Deserialize)]
pub struct Ad {
    #[serde(rename = "type")]
    ad_type: TentacledType,

    properties: AdProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdProperties {
    id: PropertiesMd5,

    expires_at: StartsAtClass,

    starts_at: StartsAtClass,

    place: IdElement,

    priority: IdElement,

    ratio: IdElement,

    url: IdElement,

    image_url: IdElement,

    memo: IdElement,

    day_of_week: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct PropertiesMd5 {
    #[serde(rename = "type")]
    id_type: TentacledType,

    format: Format,

    example: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Announcement {
    #[serde(rename = "type")]
    announcement_type: TentacledType,

    properties: AnnouncementProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    updated_at: BannerUrlClass,

    text: IdElement,

    title: IdElement,

    image_url: ImageUrl,

    icon: Method,

    display: Method,

    need_confirmation_to_read: IdElement,

    silence: IdElement,

    for_you: IdElement,

    is_read: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct ImageUrl {
    #[serde(rename = "type")]
    image_url_type: Vec<TentacledType>,
}

#[derive(Serialize, Deserialize)]
pub struct BannerUrlClass {
    #[serde(rename = "type")]
    updated_at_type: Vec<TentacledType>,

    format: Format,
}

#[derive(Serialize, Deserialize)]
pub struct Antenna {
    #[serde(rename = "type")]
    antenna_type: TentacledType,

    properties: AntennaProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AntennaProperties {
    id: StartsAtClass,

    created_at: StartsAtClass,

    name: IdElement,

    keywords: ExcludeKeywords,

    exclude_keywords: ExcludeKeywords,

    src: Method,

    user_list_id: BannerUrlClass,

    users: Users,

    case_sensitive: CaseSensitive,

    local_only: CaseSensitive,

    exclude_bots: CaseSensitive,

    with_replies: CaseSensitive,

    with_file: IdElement,

    is_active: IdElement,

    has_unread_note: CaseSensitive,

    notify: CaseSensitive,
}

#[derive(Serialize, Deserialize)]
pub struct CaseSensitive {
    #[serde(rename = "type")]
    case_sensitive_type: TentacledType,

    #[serde(rename = "default")]
    case_sensitive_default: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ExcludeKeywords {
    #[serde(rename = "type")]
    exclude_keywords_type: UsersType,

    items: Users,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsersType {
    Array,

    Object,

    String,
}

#[derive(Serialize, Deserialize)]
pub struct Users {
    #[serde(rename = "type")]
    users_type: UsersType,

    items: Option<IdElement>,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    #[serde(rename = "type")]
    app_type: TentacledType,

    properties: AppProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppProperties {
    id: IdElement,

    name: IdElement,

    callback_url: ImageUrl,

    permission: Users,

    secret: IdElement,

    is_authorized: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Blocking {
    #[serde(rename = "type")]
    blocking_type: TentacledType,

    properties: BlockingProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockingProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    blockee_id: StartsAtClass,

    blockee: Blockee,
}

#[derive(Serialize, Deserialize)]
pub struct Blockee {
    #[serde(rename = "type")]
    blockee_type: Option<TentacledType>,

    #[serde(rename = "$ref")]
    blockee_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "type")]
    channel_type: TentacledType,

    properties: ChannelProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    last_noted_at: BannerUrlClass,

    name: IdElement,

    description: ImageUrl,

    user_id: BannerUrlClass,

    banner_url: BannerUrlClass,

    pinned_note_ids: PinnedNoteIds,

    color: IdElement,

    is_archived: IdElement,

    users_count: IdElement,

    notes_count: IdElement,

    is_sensitive: IdElement,

    allow_renote_to_external: IdElement,

    is_following: IdElement,

    is_favorited: IdElement,

    pinned_notes: PinnedNotes,
}

#[derive(Serialize, Deserialize)]
pub struct PinnedNoteIds {
    #[serde(rename = "type")]
    pinned_note_ids_type: UsersType,

    items: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct PinnedNotes {
    #[serde(rename = "type")]
    pinned_notes_type: UsersType,

    items: Blockee,
}

#[derive(Serialize, Deserialize)]
pub struct Clip {
    #[serde(rename = "type")]
    clip_type: TentacledType,

    properties: ClipProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    last_clipped_at: BannerUrlClass,

    user_id: StartsAtClass,

    user: Blockee,

    name: IdElement,

    description: ImageUrl,

    is_public: IdElement,

    favorited_count: IdElement,

    is_favorited: IdElement,

    notes_count: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFile {
    #[serde(rename = "type")]
    drive_file_type: TentacledType,

    properties: DriveFileProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFileProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    name: TypeClass,

    #[serde(rename = "type")]
    properties_type: TypeClass,

    md5: PropertiesMd5,

    size: Size,

    is_sensitive: IdElement,

    blurhash: ImageUrl,

    properties: PurpleProperties,

    url: StartsAtClass,

    thumbnail_url: BannerUrlClass,

    comment: ImageUrl,

    folder_id: ReplyIdClass,

    folder: Folder,

    user_id: ReplyIdClass,

    user: Folder,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    #[serde(rename = "type")]
    folder_type: Vec<TentacledType>,

    all_of: Vec<OneOf>,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyIdClass {
    #[serde(rename = "type")]
    id_type: Vec<TentacledType>,

    format: Format,

    example: String,
}

#[derive(Serialize, Deserialize)]
pub struct TypeClass {
    #[serde(rename = "type")]
    name_type: TentacledType,

    example: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleProperties {
    #[serde(rename = "type")]
    properties_type: TentacledType,

    properties: FluffyProperties,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyProperties {
    width: Size,

    height: Size,

    orientation: Size,

    avg_color: TypeClass,
}

#[derive(Serialize, Deserialize)]
pub struct Size {
    #[serde(rename = "type")]
    size_type: TentacledType,

    example: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFolder {
    #[serde(rename = "type")]
    drive_folder_type: TentacledType,

    properties: DriveFolderProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFolderProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    name: IdElement,

    parent_id: ReplyIdClass,

    folders_count: IdElement,

    files_count: IdElement,

    parent: Folder,
}

#[derive(Serialize, Deserialize)]
pub struct EmojiDetailed {
    #[serde(rename = "type")]
    emoji_detailed_type: TentacledType,

    properties: EmojiDetailedProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiDetailedProperties {
    id: StartsAtClass,

    aliases: PinnedNoteIds,

    name: IdElement,

    category: ImageUrl,

    host: CategoryClass,

    url: IdElement,

    license: ImageUrl,

    is_sensitive: IdElement,

    local_only: IdElement,

    role_ids_that_can_be_used_this_emoji_as_reaction: PinnedNoteIds,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryClass {
    #[serde(rename = "type")]
    host_type: Vec<TentacledType>,

    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmojiSimple {
    #[serde(rename = "type")]
    emoji_simple_type: TentacledType,

    properties: EmojiSimpleProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiSimpleProperties {
    aliases: PinnedNoteIds,

    name: IdElement,

    category: ImageUrl,

    url: IdElement,

    local_only: IdElement,

    is_sensitive: IdElement,

    role_ids_that_can_be_used_this_emoji_as_reaction: PinnedNoteIds,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "type")]
    error_type: TentacledType,

    properties: ErrorProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorProperties {
    error: PropertiesError,
}

#[derive(Serialize, Deserialize)]
pub struct PropertiesError {
    #[serde(rename = "type")]
    error_type: TentacledType,

    description: String,

    properties: ErrorPropertiesClass,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorPropertiesClass {
    code: Code,

    message: Code,

    id: FileClass,
}

#[derive(Serialize, Deserialize)]
pub struct Code {
    #[serde(rename = "type")]
    code_type: TentacledType,

    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileClass {
    #[serde(rename = "type")]
    id_type: TentacledType,

    format: String,

    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct FederationInstance {
    #[serde(rename = "type")]
    federation_instance_type: TentacledType,

    properties: FederationInstanceProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FederationInstanceProperties {
    id: StartsAtClass,

    first_retrieved_at: StartsAtClass,

    host: TypeClass,

    users_count: IdElement,

    notes_count: IdElement,

    following_count: IdElement,

    followers_count: IdElement,

    is_not_responding: IdElement,

    is_suspended: IdElement,

    suspension_state: Method,

    is_blocked: IdElement,

    software_name: SoftwareName,

    software_version: ImageUrl,

    open_registrations: OpenRegistrations,

    name: ImageUrl,

    description: ImageUrl,

    maintainer_name: ImageUrl,

    maintainer_email: ImageUrl,

    is_silenced: IdElement,

    is_media_silenced: IdElement,

    icon_url: BannerUrlClass,

    favicon_url: BannerUrlClass,

    theme_color: ImageUrl,

    info_updated_at: BannerUrlClass,

    latest_request_received_at: BannerUrlClass,

    moderation_note: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct OpenRegistrations {
    #[serde(rename = "type")]
    open_registrations_type: Vec<TentacledType>,

    example: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SoftwareName {
    #[serde(rename = "type")]
    software_name_type: Vec<TentacledType>,

    example: String,
}

#[derive(Serialize, Deserialize)]
pub struct Flash {
    #[serde(rename = "type")]
    flash_type: TentacledType,

    properties: FlashProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    updated_at: StartsAtClass,

    user_id: StartsAtClass,

    user: Blockee,

    title: IdElement,

    summary: IdElement,

    script: IdElement,

    visibility: Method,

    liked_count: ImageUrl,

    is_liked: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Following {
    #[serde(rename = "type")]
    following_type: TentacledType,

    properties: FollowingProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowingProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    followee_id: StartsAtClass,

    follower_id: StartsAtClass,

    followee: SystemWebhook,

    follower: SystemWebhook,
}

#[derive(Serialize, Deserialize)]
pub struct GalleryPost {
    #[serde(rename = "type")]
    gallery_post_type: TentacledType,

    properties: GalleryPostProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GalleryPostProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    updated_at: StartsAtClass,

    user_id: StartsAtClass,

    user: Blockee,

    title: IdElement,

    description: ImageUrl,

    file_ids: PinnedNoteIds,

    files: PinnedNotes,

    tags: Users,

    is_sensitive: IdElement,

    liked_count: IdElement,

    is_liked: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Hashtag {
    #[serde(rename = "type")]
    hashtag_type: TentacledType,

    properties: HashtagProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashtagProperties {
    tag: TypeClass,

    mentioned_users_count: IdElement,

    mentioned_local_users_count: IdElement,

    mentioned_remote_users_count: IdElement,

    attached_users_count: IdElement,

    attached_local_users_count: IdElement,

    attached_remote_users_count: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct InviteCode {
    #[serde(rename = "type")]
    invite_code_type: TentacledType,

    properties: InviteCodeProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteCodeProperties {
    id: PropertiesMd5,

    code: TypeClass,

    expires_at: BannerUrlClass,

    created_at: StartsAtClass,

    created_by: Folder,

    used_by: Folder,

    used_at: BannerUrlClass,

    used: IdElement,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeDetailed {
    #[serde(rename = "type")]
    me_detailed_type: TentacledType,

    all_of: Vec<Blockee>,
}

#[derive(Serialize, Deserialize)]
pub struct MeDetailedOnly {
    #[serde(rename = "type")]
    me_detailed_only_type: TentacledType,

    properties: MeDetailedOnlyProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeDetailedOnlyProperties {
    avatar_id: BannerUrlClass,

    banner_id: BannerUrlClass,

    is_moderator: ImageUrl,

    is_admin: ImageUrl,

    inject_featured_note: IdElement,

    receive_announcement_email: IdElement,

    always_mark_nsfw: IdElement,

    auto_sensitive: IdElement,

    careful_bot: IdElement,

    auto_accept_followed: IdElement,

    no_crawle: IdElement,

    prevent_ai_learning: IdElement,

    is_explorable: IdElement,

    is_deleted: IdElement,

    two_factor_backup_codes_stock: Method,

    hide_online_status: IdElement,

    has_unread_specified_notes: IdElement,

    has_unread_mentions: IdElement,

    has_unread_announcement: IdElement,

    unread_announcements: PinnedNotes,

    has_unread_antenna: IdElement,

    has_unread_channel: IdElement,

    has_unread_notification: IdElement,

    has_pending_received_follow_request: IdElement,

    unread_notifications_count: IdElement,

    muted_words: ExcludeKeywords,

    hard_muted_words: ExcludeKeywords,

    muted_instances: MutedInstances,

    notification_recieve_config: NotificationRecieveConfig,

    email_notification_types: Users,

    achievements: Achievements,

    logged_in_days: IdElement,

    policies: Blockee,

    email: ImageUrl,

    email_verified: ImageUrl,

    security_keys_list: SecurityKeysList,
}

#[derive(Serialize, Deserialize)]
pub struct Achievements {
    #[serde(rename = "type")]
    achievements_type: UsersType,

    items: AchievementsItems,
}

#[derive(Serialize, Deserialize)]
pub struct AchievementsItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: TentacledProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledProperties {
    name: IdElement,

    unlocked_at: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct MutedInstances {
    #[serde(rename = "type")]
    muted_instances_type: Vec<ExcludeCredentialsType>,

    items: IdElement,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExcludeCredentialsType {
    Array,

    Null,
}

#[derive(Serialize, Deserialize)]
pub struct NotificationRecieveConfig {
    #[serde(rename = "type")]
    notification_recieve_config_type: TentacledType,

    properties: HashMap<String, Property>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    #[serde(rename = "type")]
    property_type: TentacledType,

    one_of: Vec<PropertyOneOf>,
}

#[derive(Serialize, Deserialize)]
pub struct PropertyOneOf {
    #[serde(rename = "type")]
    one_of_type: TentacledType,

    properties: StickyProperties,

    required: Vec<OneOfRequired>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyProperties {
    #[serde(rename = "type")]
    properties_type: Method,

    user_list_id: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OneOfRequired {
    Type,

    #[serde(rename = "userListId")]
    UserListId,
}

#[derive(Serialize, Deserialize)]
pub struct SecurityKeysList {
    #[serde(rename = "type")]
    security_keys_list_type: UsersType,

    items: SecurityKeysListItems,
}

#[derive(Serialize, Deserialize)]
pub struct SecurityKeysListItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: IndigoProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoProperties {
    id: PropertiesMd5,

    name: IdElement,

    last_used: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct MetaDetailedOnly {
    #[serde(rename = "type")]
    meta_detailed_only_type: TentacledType,

    properties: MetaDetailedOnlyProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaDetailedOnlyProperties {
    features: Features,

    proxy_account_name: ImageUrl,

    require_setup: RequireSetup,

    cache_remote_files: IdElement,

    cache_remote_sensitive_files: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Features {
    #[serde(rename = "type")]
    features_type: TentacledType,

    properties: FeaturesProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturesProperties {
    registration: IdElement,

    email_required_for_signup: IdElement,

    local_timeline: IdElement,

    global_timeline: IdElement,

    hcaptcha: IdElement,

    turnstile: IdElement,

    recaptcha: IdElement,

    object_storage: IdElement,

    service_worker: IdElement,

    miauth: CaseSensitive,
}

#[derive(Serialize, Deserialize)]
pub struct RequireSetup {
    #[serde(rename = "type")]
    require_setup_type: TentacledType,

    example: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MetaLite {
    #[serde(rename = "type")]
    meta_lite_type: TentacledType,

    properties: MetaLiteProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaLiteProperties {
    maintainer_name: ImageUrl,

    maintainer_email: ImageUrl,

    version: IdElement,

    provides_tarball: IdElement,

    name: ImageUrl,

    short_name: ImageUrl,

    uri: PropertiesMd5,

    description: ImageUrl,

    langs: Users,

    tos_url: ImageUrl,

    repository_url: FeedbackUrl,

    feedback_url: FeedbackUrl,

    default_dark_theme: ImageUrl,

    default_light_theme: ImageUrl,

    disable_registration: IdElement,

    email_required_for_signup: IdElement,

    enable_hcaptcha: IdElement,

    hcaptcha_site_key: ImageUrl,

    enable_mcaptcha: IdElement,

    mcaptcha_site_key: ImageUrl,

    mcaptcha_instance_url: ImageUrl,

    enable_recaptcha: IdElement,

    recaptcha_site_key: ImageUrl,

    enable_turnstile: IdElement,

    turnstile_site_key: ImageUrl,

    sw_publickey: ImageUrl,

    mascot_image_url: MascotImageUrl,

    banner_url: ImageUrl,

    server_error_image_url: ImageUrl,

    info_image_url: ImageUrl,

    not_found_image_url: ImageUrl,

    icon_url: ImageUrl,

    max_note_text_length: IdElement,

    ads: Ads,

    notes_per_one_ad: NotesPerOneAd,

    enable_email: IdElement,

    enable_service_worker: IdElement,

    translator_available: IdElement,

    media_proxy: IdElement,

    enable_url_preview: IdElement,

    background_image_url: ImageUrl,

    impressum_url: ImageUrl,

    logo_image_url: ImageUrl,

    privacy_policy_url: ImageUrl,

    inquiry_url: ImageUrl,

    server_rules: Users,

    theme_color: ImageUrl,

    policies: Blockee,

    note_searchable_scope: NoteSearchableScope,

    max_file_size: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Ads {
    #[serde(rename = "type")]
    ads_type: UsersType,

    items: AdsItems,
}

#[derive(Serialize, Deserialize)]
pub struct AdsItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: IndecentProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentProperties {
    id: PropertiesMd5,

    url: StartsAtClass,

    place: IdElement,

    ratio: IdElement,

    image_url: StartsAtClass,

    day_of_week: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct FeedbackUrl {
    #[serde(rename = "type")]
    feedback_url_type: Vec<TentacledType>,

    #[serde(rename = "default")]
    feedback_url_default: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MascotImageUrl {
    #[serde(rename = "type")]
    mascot_image_url_type: TentacledType,

    #[serde(rename = "default")]
    mascot_image_url_default: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoteSearchableScope {
    #[serde(rename = "type")]
    note_searchable_scope_type: TentacledType,

    #[serde(rename = "enum")]
    note_searchable_scope_enum: Vec<String>,

    #[serde(rename = "default")]
    note_searchable_scope_default: String,
}

#[derive(Serialize, Deserialize)]
pub struct NotesPerOneAd {
    #[serde(rename = "type")]
    notes_per_one_ad_type: TentacledType,

    #[serde(rename = "default")]
    notes_per_one_ad_default: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Muting {
    #[serde(rename = "type")]
    muting_type: TentacledType,

    properties: MutingProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutingProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    expires_at: Option<BannerUrlClass>,

    mutee_id: StartsAtClass,

    mutee: Blockee,
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "type")]
    note_type: TentacledType,

    properties: NoteProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    deleted_at: BannerUrlClass,

    text: ImageUrl,

    cw: ImageUrl,

    user_id: StartsAtClass,

    user: Blockee,

    reply_id: ReplyIdClass,

    renote_id: ReplyIdClass,

    reply: Folder,

    renote: Folder,

    is_hidden: IdElement,

    visibility: Method,

    mentions: PinnedNoteIds,

    visible_user_ids: PinnedNoteIds,

    file_ids: PinnedNoteIds,

    files: PinnedNotes,

    tags: Users,

    poll: PurplePoll,

    emojis: ReactionEmojisClass,

    channel_id: ReplyIdClass,

    channel: ChannelClass,

    local_only: IdElement,

    reaction_acceptance: ReactionAcceptance,

    reaction_emojis: ReactionEmojisClass,

    reactions: ReactionEmojisClass,

    reaction_count: IdElement,

    renote_count: IdElement,

    replies_count: IdElement,

    uri: IdElement,

    url: IdElement,

    reaction_and_user_pair_cache: Users,

    clipped_count: IdElement,

    my_reaction: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelClass {
    #[serde(rename = "type")]
    channel_type: Vec<TentacledType>,

    properties: ChannelPropertiesClass,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelPropertiesClass {
    id: IdElement,

    name: IdElement,

    color: IdElement,

    is_sensitive: IdElement,

    allow_renote_to_external: IdElement,

    user_id: ImageUrl,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactionEmojisClass {
    #[serde(rename = "type")]
    emojis_type: TentacledType,

    additional_properties: EmojisAdditionalProperties,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojisAdditionalProperties {
    any_of: Vec<IdElement>,
}

#[derive(Serialize, Deserialize)]
pub struct PurplePoll {
    #[serde(rename = "type")]
    poll_type: Vec<TentacledType>,

    properties: HilariousProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousProperties {
    expires_at: BannerUrlClass,

    multiple: IdElement,

    choices: PurpleChoices,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleChoices {
    #[serde(rename = "type")]
    choices_type: UsersType,

    items: ChoicesItems,
}

#[derive(Serialize, Deserialize)]
pub struct ChoicesItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: AmbitiousProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbitiousProperties {
    is_voted: IdElement,

    text: IdElement,

    votes: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct ReactionAcceptance {
    #[serde(rename = "type")]
    reaction_acceptance_type: Vec<TentacledType>,

    #[serde(rename = "enum")]
    reaction_acceptance_enum: Vec<Option<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct NoteFavorite {
    #[serde(rename = "type")]
    note_favorite_type: TentacledType,

    properties: NoteFavoriteProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteFavoriteProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    note: Blockee,

    note_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct NoteReaction {
    #[serde(rename = "type")]
    note_reaction_type: TentacledType,

    properties: NoteReactionProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteReactionProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    user: Blockee,

    #[serde(rename = "type")]
    properties_type: IdElement,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    #[serde(rename = "type")]
    notification_type: TentacledType,

    one_of: Vec<NotificationOneOf>,
}

#[derive(Serialize, Deserialize)]
pub struct NotificationOneOf {
    #[serde(rename = "type")]
    one_of_type: TentacledType,

    properties: CunningProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CunningProperties {
    id: StartsAtClass,

    created_at: StartsAtClass,

    #[serde(rename = "type")]
    properties_type: Method,

    user: Option<Blockee>,

    user_id: Option<StartsAtClass>,

    note: Option<Blockee>,

    reaction: Option<IdElement>,

    role: Option<Blockee>,

    achievement: Option<Method>,

    body: Option<IdElement>,

    header: Option<ImageUrl>,

    icon: Option<ImageUrl>,

    reactions: Option<Reactions>,

    users: Option<PinnedNotes>,
}

#[derive(Serialize, Deserialize)]
pub struct Reactions {
    #[serde(rename = "type")]
    reactions_type: UsersType,

    items: ReactionsItems,
}

#[derive(Serialize, Deserialize)]
pub struct ReactionsItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: MagentaProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MagentaProperties {
    user: Blockee,

    reaction: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Page {
    #[serde(rename = "type")]
    page_type: TentacledType,

    properties: PageProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    updated_at: StartsAtClass,

    user_id: StartsAtClass,

    user: Blockee,

    content: PinnedNotes,

    variables: Users,

    title: IdElement,

    name: IdElement,

    summary: ImageUrl,

    hide_title_when_pinned: IdElement,

    align_center: IdElement,

    font: IdElement,

    script: IdElement,

    eye_catching_image_id: ImageUrl,

    eye_catching_image: Folder,

    attached_files: PinnedNotes,

    liked_count: IdElement,

    is_liked: IdElement,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageBlock {
    #[serde(rename = "type")]
    page_block_type: TentacledType,

    one_of: Vec<PageBlockOneOf>,
}

#[derive(Serialize, Deserialize)]
pub struct PageBlockOneOf {
    #[serde(rename = "type")]
    one_of_type: TentacledType,

    properties: FriskyProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriskyProperties {
    id: IdElement,

    #[serde(rename = "type")]
    properties_type: Method,

    text: Option<IdElement>,

    title: Option<IdElement>,

    children: Option<PinnedNotes>,

    file_id: Option<ImageUrl>,

    detailed: Option<IdElement>,

    note: Option<ImageUrl>,
}

#[derive(Serialize, Deserialize)]
pub struct QueueCount {
    #[serde(rename = "type")]
    queue_count_type: TentacledType,

    properties: QueueCountProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct QueueCountProperties {
    waiting: IdElement,

    active: IdElement,

    completed: IdElement,

    failed: IdElement,

    delayed: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct ReversiGame {
    #[serde(rename = "type")]
    reversi_game_type: TentacledType,

    properties: ReversiGameDetailedProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReversiGameDetailedProperties {
    id: StartsAtClass,

    created_at: StartsAtClass,

    started_at: BannerUrlClass,

    ended_at: BannerUrlClass,

    is_started: IdElement,

    is_ended: IdElement,

    form1: Option<ImageUrl>,

    form2: Option<ImageUrl>,

    user1_ready: Option<IdElement>,

    user2_ready: Option<IdElement>,

    user1_id: StartsAtClass,

    user2_id: StartsAtClass,

    user1: Blockee,

    user2: Blockee,

    winner_id: BannerUrlClass,

    winner: Folder,

    surrendered_user_id: BannerUrlClass,

    timeout_user_id: BannerUrlClass,

    black: ImageUrl,

    bw: IdElement,

    no_irregular_rules: IdElement,

    is_llotheo: IdElement,

    can_put_everywhere: IdElement,

    looped_board: IdElement,

    time_limit_for_each_turn: IdElement,

    logs: Option<ExcludeKeywords>,

    map: Option<Users>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    #[serde(rename = "type")]
    role_type: TentacledType,

    all_of: Vec<AllOf>,
}

#[derive(Serialize, Deserialize)]
pub struct AllOf {
    #[serde(rename = "type")]
    all_of_type: TentacledType,

    #[serde(rename = "$ref")]
    all_of_ref: Option<String>,

    properties: Option<AllOfProperties>,

    required: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllOfProperties {
    created_at: StartsAtClass,

    updated_at: StartsAtClass,

    target: Method,

    cond_formula: Blockee,

    is_public: RequireSetup,

    is_explorable: RequireSetup,

    as_badge: RequireSetup,

    can_edit_members_by_moderator: RequireSetup,

    policies: Policies,

    users_count: IdElement,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policies {
    #[serde(rename = "type")]
    policies_type: TentacledType,

    additional_properties: PoliciesAdditionalProperties,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoliciesAdditionalProperties {
    any_of: Vec<AdditionalPropertiesAnyOf>,
}

#[derive(Serialize, Deserialize)]
pub struct AdditionalPropertiesAnyOf {
    #[serde(rename = "type")]
    any_of_type: TentacledType,

    properties: AnyOfProperties,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyOfProperties {
    value: PropertiesValue,

    priority: IdElement,

    use_default: IdElement,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertiesValue {
    one_of: Vec<IdElement>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleCondFormulaFollowersOrFollowingOrNotes {
    #[serde(rename = "type")]
    role_cond_formula_followers_or_following_or_notes_type: TentacledType,

    properties: RoleCondFormulaFollowersOrFollowingOrNotesProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleCondFormulaFollowersOrFollowingOrNotesProperties {
    id: IdElement,

    #[serde(rename = "type")]
    properties_type: Method,

    value: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct RoleCondFormulaLogics {
    #[serde(rename = "type")]
    role_cond_formula_logics_type: TentacledType,

    properties: RoleCondFormulaLogicsProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleCondFormulaLogicsProperties {
    id: IdElement,

    #[serde(rename = "type")]
    properties_type: Method,

    values: Values,
}

#[derive(Serialize, Deserialize)]
pub struct Values {
    #[serde(rename = "type")]
    values_type: UsersType,

    items: OneOf,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleCondFormulaValue {
    #[serde(rename = "type")]
    role_cond_formula_value_type: TentacledType,

    one_of: Vec<OneOf>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleCondFormulaValueAssignedRole {
    #[serde(rename = "type")]
    role_cond_formula_value_assigned_role_type: TentacledType,

    properties: RoleCondFormulaValueAssignedRoleProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleCondFormulaValueAssignedRoleProperties {
    id: IdElement,

    #[serde(rename = "type")]
    properties_type: Method,

    role_id: PropertiesMd5,
}

#[derive(Serialize, Deserialize)]
pub struct RoleCondFormulaValueCreated {
    #[serde(rename = "type")]
    role_cond_formula_value_created_type: TentacledType,

    properties: RoleCondFormulaValueCreatedProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleCondFormulaValueCreatedProperties {
    id: IdElement,

    #[serde(rename = "type")]
    properties_type: Method,

    sec: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct RoleCondFormulaValueIsLocalOrRemoteClass {
    #[serde(rename = "type")]
    role_cond_formula_value_type: TentacledType,

    properties: RoleCondFormulaValueIsLocalOrRemoteProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleCondFormulaValueIsLocalOrRemoteProperties {
    id: IdElement,

    #[serde(rename = "type")]
    properties_type: Method,
}

#[derive(Serialize, Deserialize)]
pub struct RoleCondFormulaValueNot {
    #[serde(rename = "type")]
    role_cond_formula_value_not_type: TentacledType,

    properties: RoleCondFormulaValueNotProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleCondFormulaValueNotProperties {
    id: IdElement,

    #[serde(rename = "type")]
    properties_type: Method,

    value: Blockee,
}

#[derive(Serialize, Deserialize)]
pub struct RoleLite {
    #[serde(rename = "type")]
    role_lite_type: TentacledType,

    properties: RoleLiteProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleLiteProperties {
    id: PropertiesMd5,

    name: TypeClass,

    color: SoftwareName,

    icon_url: ImageUrl,

    description: IdElement,

    is_moderator: RequireSetup,

    is_administrator: RequireSetup,

    display_order: Size,
}

#[derive(Serialize, Deserialize)]
pub struct RolePolicies {
    #[serde(rename = "type")]
    role_policies_type: TentacledType,

    properties: HashMap<String, IdElement>,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Signin {
    #[serde(rename = "type")]
    signin_type: TentacledType,

    properties: SigninProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigninProperties {
    id: IdElement,

    created_at: StartsAtClass,

    ip: IdElement,

    headers: IdElement,

    success: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct SystemWebhookClass {
    #[serde(rename = "type")]
    system_webhook_type: TentacledType,

    properties: SystemWebhookProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemWebhookProperties {
    id: IdElement,

    is_active: IdElement,

    updated_at: StartsAtClass,

    latest_sent_at: BannerUrlClass,

    latest_status: ImageUrl,

    name: IdElement,

    on: On,

    url: IdElement,

    secret: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct On {
    #[serde(rename = "type")]
    on_type: UsersType,

    items: Method,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserClass {
    one_of: Vec<Blockee>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDetailedNotMeOnly {
    #[serde(rename = "type")]
    user_detailed_not_me_only_type: TentacledType,

    properties: UserDetailedNotMeOnlyProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetailedNotMeOnlyProperties {
    url: BannerUrlClass,

    uri: BannerUrlClass,

    moved_to: BannerUrlClass,

    also_known_as: AlsoKnownAs,

    created_at: StartsAtClass,

    updated_at: BannerUrlClass,

    last_fetched_at: BannerUrlClass,

    banner_url: BannerUrlClass,

    banner_blurhash: ImageUrl,

    is_locked: IdElement,

    is_silenced: IdElement,

    is_suspended: RequireSetup,

    description: SoftwareName,

    location: ImageUrl,

    birthday: SoftwareName,

    lang: SoftwareName,

    fields: Fields,

    verified_links: PinnedNoteIds,

    followers_count: IdElement,

    following_count: IdElement,

    notes_count: IdElement,

    pinned_note_ids: PinnedNoteIds,

    pinned_notes: PinnedNotes,

    pinned_page_id: ImageUrl,

    pinned_page: Folder,

    public_reactions: IdElement,

    following_visibility: Method,

    followers_visibility: Method,

    two_factor_enabled: CaseSensitive,

    use_password_less_login: CaseSensitive,

    security_keys: CaseSensitive,

    roles: PinnedNotes,

    memo: ImageUrl,

    moderation_note: IdElement,

    is_following: IdElement,

    is_followed: IdElement,

    has_pending_follow_request_from_you: IdElement,

    has_pending_follow_request_to_you: IdElement,

    is_blocking: IdElement,

    is_blocked: IdElement,

    is_muted: IdElement,

    is_renote_muted: IdElement,

    notify: Method,

    with_replies: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct AlsoKnownAs {
    #[serde(rename = "type")]
    also_known_as_type: Vec<ExcludeCredentialsType>,

    items: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    #[serde(rename = "type")]
    fields_type: UsersType,

    max_items: i64,

    items: FieldsItems,

    min_items: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldsItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: MischievousProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MischievousProperties {
    name: IdElement,

    value: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct UserList {
    #[serde(rename = "type")]
    user_list_type: TentacledType,

    properties: UserListProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListProperties {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    name: IdElement,

    user_ids: PinnedNoteIds,

    is_public: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct UserLite {
    #[serde(rename = "type")]
    user_lite_type: TentacledType,

    properties: UserLiteProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLiteProperties {
    id: PropertiesMd5,

    name: SoftwareName,

    username: TypeClass,

    host: PurpleHost,

    avatar_url: BannerUrlClass,

    avatar_blurhash: ImageUrl,

    avatar_decorations: PurpleAvatarDecorations,

    is_bot: IdElement,

    is_cat: IdElement,

    instance: Instance,

    emojis: PurpleEmojis,

    online_status: Method,

    badge_roles: BadgeRoles,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleAvatarDecorations {
    #[serde(rename = "type")]
    avatar_decorations_type: UsersType,

    items: PurpleItems,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: BraggadociousProperties,

    required: Vec<Format>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BraggadociousProperties {
    id: StartsAtClass,

    angle: IdElement,

    flip_h: IdElement,

    url: StartsAtClass,

    offset_x: IdElement,

    offset_y: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct BadgeRoles {
    #[serde(rename = "type")]
    badge_roles_type: UsersType,

    items: BadgeRolesItems,
}

#[derive(Serialize, Deserialize)]
pub struct BadgeRolesItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: Properties1,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties1 {
    name: IdElement,

    icon_url: ImageUrl,

    display_order: IdElement,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleEmojis {
    #[serde(rename = "type")]
    emojis_type: TentacledType,

    additional_properties: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleHost {
    #[serde(rename = "type")]
    host_type: Vec<TentacledType>,

    example: String,

    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Instance {
    #[serde(rename = "type")]
    instance_type: TentacledType,

    properties: InstanceProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceProperties {
    name: ImageUrl,

    software_name: ImageUrl,

    software_version: ImageUrl,

    icon_url: ImageUrl,

    favicon_url: ImageUrl,

    theme_color: ImageUrl,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritySchemes {
    bearer_auth: BearerAuth,
}

#[derive(Serialize, Deserialize)]
pub struct BearerAuth {
    #[serde(rename = "type")]
    bearer_auth_type: String,

    scheme: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExternalDocs {
    description: ExternalDocsDescription,

    url: String,
}

#[derive(Serialize, Deserialize)]
pub enum ExternalDocsDescription {
    Repository,

    #[serde(rename = "Source code")]
    SourceCode,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    version: String,

    title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Paths {
    #[serde(rename = "/admin/meta")]
    admin_meta: AdminGetIndexStats,

    #[serde(rename = "/admin/abuse-user-reports")]
    admin_abuse_user_reports: AdminAbuseUserReports,

    #[serde(rename = "/admin/abuse-report/notification-recipient/list")]
    admin_abuse_report_notification_recipient_list: AdminAbuseReportNotificationRecipientList,

    #[serde(rename = "/admin/abuse-report/notification-recipient/show")]
    admin_abuse_report_notification_recipient_show: AdminAbuseReportNotificationRecipientShow,

    #[serde(rename = "/admin/abuse-report/notification-recipient/create")]
    admin_abuse_report_notification_recipient_create: AdminAbuseReportNotificationRecipientAte,

    #[serde(rename = "/admin/abuse-report/notification-recipient/update")]
    admin_abuse_report_notification_recipient_update: AdminAbuseReportNotificationRecipientAte,

    #[serde(rename = "/admin/abuse-report/notification-recipient/delete")]
    admin_abuse_report_notification_recipient_delete: AdminDelete,

    #[serde(rename = "/admin/accounts/create")]
    admin_accounts_create: AdminAccountsCreate,

    #[serde(rename = "/admin/accounts/delete")]
    admin_accounts_delete: Admin,

    #[serde(rename = "/admin/accounts/find-by-email")]
    admin_accounts_find_by_email: AdminAccountsFindByEmail,

    #[serde(rename = "/admin/ad/create")]
    admin_ad_create: AdminAdCreate,

    #[serde(rename = "/admin/ad/delete")]
    admin_ad_delete: AdminAdDelete,

    #[serde(rename = "/admin/ad/list")]
    admin_ad_list: AdminAdList,

    #[serde(rename = "/admin/ad/update")]
    admin_ad_update: AdminAdUpdate,

    #[serde(rename = "/admin/announcements/create")]
    admin_announcements_create: AdminAnnouncementsCreate,

    #[serde(rename = "/admin/announcements/delete")]
    admin_announcements_delete: AdminAnnouncementsDelete,

    #[serde(rename = "/admin/announcements/list")]
    admin_announcements_list: AdminAnnouncementsList,

    #[serde(rename = "/admin/announcements/update")]
    admin_announcements_update: AdminAnnouncementsUpdate,

    #[serde(rename = "/admin/avatar-decorations/create")]
    admin_avatar_decorations_create: AdminAvatarDecorationsCreate,

    #[serde(rename = "/admin/avatar-decorations/delete")]
    admin_avatar_decorations_delete: AdminDelete,

    #[serde(rename = "/admin/avatar-decorations/list")]
    admin_avatar_decorations_list: AdminAvatarDecorationsList,

    #[serde(rename = "/admin/avatar-decorations/update")]
    admin_avatar_decorations_update: AdminAvatarDecorationsUpdate,

    #[serde(rename = "/admin/delete-all-files-of-a-user")]
    admin_delete_all_files_of_a_user: Admin,

    #[serde(rename = "/admin/unset-user-avatar")]
    admin_unset_user_avatar: Admin,

    #[serde(rename = "/admin/unset-user-banner")]
    admin_unset_user_banner: Admin,

    #[serde(rename = "/admin/drive/clean-remote-files")]
    admin_drive_clean_remote_files: AdminDriveCleanRemoteFiles,

    #[serde(rename = "/admin/drive/cleanup")]
    admin_drive_cleanup: AdminDriveCleanRemoteFiles,

    #[serde(rename = "/admin/drive/files")]
    admin_drive_files: AdminDriveFiles,

    #[serde(rename = "/admin/drive/show-file")]
    admin_drive_show_file: AdminDriveShowFile,

    #[serde(rename = "/admin/emoji/add-aliases-bulk")]
    admin_emoji_add_aliases_bulk: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/emoji/add")]
    admin_emoji_add: AdminEmojiAdd,

    #[serde(rename = "/admin/emoji/copy")]
    admin_emoji_copy: AdminEmojiCopy,

    #[serde(rename = "/admin/emoji/delete-bulk")]
    admin_emoji_delete_bulk: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/emoji/delete")]
    admin_emoji_delete: AdminEmojiDelete,

    #[serde(rename = "/admin/emoji/import-zip")]
    admin_emoji_import_zip: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/emoji/list-remote")]
    admin_emoji_list_remote: AdminAvatarDecorationsList,

    #[serde(rename = "/admin/emoji/list")]
    admin_emoji_list: AdminAvatarDecorationsList,

    #[serde(rename = "/admin/emoji/remove-aliases-bulk")]
    admin_emoji_remove_aliases_bulk: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/emoji/set-aliases-bulk")]
    admin_emoji_set_aliases_bulk: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/emoji/set-category-bulk")]
    admin_emoji_set_category_bulk: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/emoji/set-license-bulk")]
    admin_emoji_set_license_bulk: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/emoji/update")]
    admin_emoji_update: AdminEmojiUpdate,

    #[serde(rename = "/admin/federation/delete-all-files")]
    admin_federation_delete_all_files: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/federation/refresh-remote-instance-metadata")]
    admin_federation_refresh_remote_instance_metadata: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/federation/remove-all-following")]
    admin_federation_remove_all_following: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/federation/update-instance")]
    admin_federation_update_instance: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/get-index-stats")]
    admin_get_index_stats: AdminGetIndexStats,

    #[serde(rename = "/admin/get-table-stats")]
    admin_get_table_stats: AdminGetIndexStats,

    #[serde(rename = "/admin/get-user-ips")]
    admin_get_user_ips: Admin,

    #[serde(rename = "/admin/invite/create")]
    admin_invite_create: AdminInviteCreate,

    #[serde(rename = "/admin/invite/list")]
    admin_invite_list: AdminInviteList,

    #[serde(rename = "/admin/promo/create")]
    admin_promo_create: AdminPromoCreate,

    #[serde(rename = "/admin/queue/clear")]
    admin_queue_clear: AdminDriveCleanRemoteFiles,

    #[serde(rename = "/admin/queue/deliver-delayed")]
    admin_queue_deliver_delayed: AdminGetIndexStats,

    #[serde(rename = "/admin/queue/inbox-delayed")]
    admin_queue_inbox_delayed: AdminGetIndexStats,

    #[serde(rename = "/admin/queue/promote")]
    admin_queue_promote: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/queue/stats")]
    admin_queue_stats: AdminGetIndexStats,

    #[serde(rename = "/admin/relays/add")]
    admin_relays_add: AdminRelaysAdd,

    #[serde(rename = "/admin/relays/list")]
    admin_relays_list: AdminGetIndexStats,

    #[serde(rename = "/admin/relays/remove")]
    admin_relays_remove: AdminAvatarDecorationsCreate,

    #[serde(rename = "/admin/reset-password")]
    admin_reset_password: Admin,

    #[serde(rename = "/admin/resolve-abuse-user-report")]
    admin_resolve_abuse_user_report: AdminResolveAbuseUserReport,

    #[serde(rename = "/admin/send-email")]
    admin_send_email: AdminAvatarDecorationsCreate,

    #[serde(rename = "/admin/server-info")]
    admin_server_info: AdminGetIndexStats,

    #[serde(rename = "/admin/show-moderation-logs")]
    admin_show_moderation_logs: AdminAvatarDecorationsList,

    #[serde(rename = "/admin/show-user")]
    admin_show_user: Admin,

    #[serde(rename = "/admin/show-users")]
    admin_show_users: AdminDriveFiles,

    #[serde(rename = "/admin/suspend-user")]
    admin_suspend_user: Admin,

    #[serde(rename = "/admin/unsuspend-user")]
    admin_unsuspend_user: Admin,

    #[serde(rename = "/admin/update-meta")]
    admin_update_meta: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/delete-account")]
    admin_delete_account: Admin,

    #[serde(rename = "/admin/update-user-note")]
    admin_update_user_note: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/roles/create")]
    admin_roles_create: AdminRolesCreate,

    #[serde(rename = "/admin/roles/delete")]
    admin_roles_delete: AdminRolesDelete,

    #[serde(rename = "/admin/roles/list")]
    admin_roles_list: AdminGetIndexStats,

    #[serde(rename = "/admin/roles/show")]
    admin_roles_show: AdminRolesDelete,

    #[serde(rename = "/admin/roles/update")]
    admin_roles_update: AdminRolesUpdate,

    #[serde(rename = "/admin/roles/assign")]
    admin_roles_assign: AdminRolesAssign,

    #[serde(rename = "/admin/roles/unassign")]
    admin_roles_unassign: AdminRolesUnassign,

    #[serde(rename = "/admin/roles/update-default-policies")]
    admin_roles_update_default_policies: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/admin/roles/users")]
    admin_roles_users: RolesUsers,

    #[serde(rename = "/admin/system-webhook/create")]
    admin_system_webhook_create: AdminSystemWebhookAte,

    #[serde(rename = "/admin/system-webhook/delete")]
    admin_system_webhook_delete: AdminDelete,

    #[serde(rename = "/admin/system-webhook/list")]
    admin_system_webhook_list: AdminDriveFiles,

    #[serde(rename = "/admin/system-webhook/show")]
    admin_system_webhook_show: AdminSystemWebhookShow,

    #[serde(rename = "/admin/system-webhook/update")]
    admin_system_webhook_update: AdminSystemWebhookAte,

    #[serde(rename = "/admin/system-webhook/test")]
    admin_system_webhook_test: Test,

    #[serde(rename = "/announcements")]
    announcements: AdminDriveFiles,

    #[serde(rename = "/announcements/show")]
    announcements_show: AnnouncementsShow,

    #[serde(rename = "/antennas/create")]
    antennas_create: AntennasAte,

    #[serde(rename = "/antennas/delete")]
    antennas_delete: Antennas,

    #[serde(rename = "/antennas/list")]
    antennas_list: AdminGetIndexStats,

    #[serde(rename = "/antennas/notes")]
    antennas_notes: AntennasNotes,

    #[serde(rename = "/antennas/show")]
    antennas_show: Antennas,

    #[serde(rename = "/antennas/update")]
    antennas_update: AntennasAte,

    #[serde(rename = "/ap/get")]
    ap_get: ApGet,

    #[serde(rename = "/ap/show")]
    ap_show: ApShow,

    #[serde(rename = "/app/create")]
    app_create: AppCreate,

    #[serde(rename = "/app/show")]
    app_show: AppShow,

    #[serde(rename = "/auth/accept")]
    auth_accept: AuthAccept,

    #[serde(rename = "/auth/session/generate")]
    auth_session_generate: AuthSessionGenerate,

    #[serde(rename = "/auth/session/show")]
    auth_session_show: AuthSessionShow,

    #[serde(rename = "/auth/session/userkey")]
    auth_session_userkey: AuthSessionUserkey,

    #[serde(rename = "/blocking/create")]
    blocking_create: BlockingCreateClass,

    #[serde(rename = "/blocking/delete")]
    blocking_delete: BlockingCreateClass,

    #[serde(rename = "/blocking/list")]
    blocking_list: BlockingList,

    #[serde(rename = "/channels/create")]
    channels_create: ChannelsCreate,

    #[serde(rename = "/channels/featured")]
    channels_featured: ChannelsFeatured,

    #[serde(rename = "/channels/follow")]
    channels_follow: Channels,

    #[serde(rename = "/channels/followed")]
    channels_followed: BlockingList,

    #[serde(rename = "/channels/owned")]
    channels_owned: BlockingList,

    #[serde(rename = "/channels/show")]
    channels_show: ChannelsShow,

    #[serde(rename = "/channels/timeline")]
    channels_timeline: ChannelsTimeline,

    #[serde(rename = "/channels/unfollow")]
    channels_unfollow: Channels,

    #[serde(rename = "/channels/update")]
    channels_update: ChannelsUpdate,

    #[serde(rename = "/channels/favorite")]
    channels_favorite: Channels,

    #[serde(rename = "/channels/unfavorite")]
    channels_unfavorite: Channels,

    #[serde(rename = "/channels/my-favorites")]
    channels_my_favorites: AdminGetIndexStats,

    #[serde(rename = "/channels/search")]
    channels_search: AdminAbuseReportNotificationRecipientList,

    #[serde(rename = "/charts/active-users")]
    charts_active_users: Charts,

    #[serde(rename = "/charts/ap-request")]
    charts_ap_request: Charts,

    #[serde(rename = "/charts/drive")]
    charts_drive: Charts,

    #[serde(rename = "/charts/federation")]
    charts_federation: Charts,

    #[serde(rename = "/charts/instance")]
    charts_instance: Charts,

    #[serde(rename = "/charts/notes")]
    charts_notes: Charts,

    #[serde(rename = "/charts/user/drive")]
    charts_user_drive: Charts,

    #[serde(rename = "/charts/user/following")]
    charts_user_following: Charts,

    #[serde(rename = "/charts/user/notes")]
    charts_user_notes: Charts,

    #[serde(rename = "/charts/user/pv")]
    charts_user_pv: Charts,

    #[serde(rename = "/charts/user/reactions")]
    charts_user_reactions: Charts,

    #[serde(rename = "/charts/users")]
    charts_users: Charts,

    #[serde(rename = "/clips/add-note")]
    clips_add_note: ClipsAddNote,

    #[serde(rename = "/clips/remove-note")]
    clips_remove_note: ClipsRemoveNote,

    #[serde(rename = "/clips/create")]
    clips_create: ClipsCreate,

    #[serde(rename = "/clips/delete")]
    clips_delete: ClipsDelete,

    #[serde(rename = "/clips/list")]
    clips_list: AdminGetIndexStats,

    #[serde(rename = "/clips/notes")]
    clips_notes: ClipsNotes,

    #[serde(rename = "/clips/show")]
    clips_show: ClipsShow,

    #[serde(rename = "/clips/update")]
    clips_update: ClipsUpdate,

    #[serde(rename = "/clips/favorite")]
    clips_favorite: ClipsFavorite,

    #[serde(rename = "/clips/unfavorite")]
    clips_unfavorite: ClipsUnfavorite,

    #[serde(rename = "/clips/my-favorites")]
    clips_my_favorites: AdminGetIndexStats,

    #[serde(rename = "/drive")]
    drive: AdminGetIndexStats,

    #[serde(rename = "/drive/files")]
    drive_files: AdminAdList,

    #[serde(rename = "/drive/files/attached-notes")]
    drive_files_attached_notes: DriveFilesAttachedNotes,

    #[serde(rename = "/drive/files/check-existence")]
    drive_files_check_existence: DriveFilesCheckExistence,

    #[serde(rename = "/drive/files/create")]
    drive_files_create: DriveFilesCreate,

    #[serde(rename = "/drive/files/delete")]
    drive_files_delete: DriveFilesDelete,

    #[serde(rename = "/drive/files/find-by-hash")]
    drive_files_find_by_hash: AdminAdList,

    #[serde(rename = "/drive/files/find")]
    drive_files_find: AdminAdList,

    #[serde(rename = "/drive/files/show")]
    drive_files_show: DriveFilesShow,

    #[serde(rename = "/drive/files/update")]
    drive_files_update: DriveFilesUpdate,

    #[serde(rename = "/drive/files/upload-from-url")]
    drive_files_upload_from_url: DriveFilesUploadFromUrl,

    #[serde(rename = "/drive/folders")]
    drive_folders: AdminDriveFiles,

    #[serde(rename = "/drive/folders/create")]
    drive_folders_create: DriveFoldersCreate,

    #[serde(rename = "/drive/folders/delete")]
    drive_folders_delete: DriveFoldersDelete,

    #[serde(rename = "/drive/folders/find")]
    drive_folders_find: AdminAbuseReportNotificationRecipientList,

    #[serde(rename = "/drive/folders/show")]
    drive_folders_show: DriveFoldersShow,

    #[serde(rename = "/drive/folders/update")]
    drive_folders_update: DriveFoldersUpdate,

    #[serde(rename = "/drive/stream")]
    drive_stream: AdminInviteList,

    #[serde(rename = "/email-address/available")]
    email_address_available: EmailAddressAvailable,

    #[serde(rename = "/endpoint")]
    endpoint: Endpoint,

    #[serde(rename = "/endpoints")]
    endpoints: Endpoints,

    #[serde(rename = "/export-custom-emojis")]
    export_custom_emojis: DriveFilesUploadFromUrl,

    #[serde(rename = "/federation/followers")]
    federation_followers: Federation,

    #[serde(rename = "/federation/following")]
    federation_following: Federation,

    #[serde(rename = "/federation/instances")]
    federation_instances: FederationInstances,

    #[serde(rename = "/federation/show-instance")]
    federation_show_instance: FederationShowInstance,

    #[serde(rename = "/federation/update-remote-user")]
    federation_update_remote_user: AdminAvatarDecorationsCreate,

    #[serde(rename = "/federation/users")]
    federation_users: Federation,

    #[serde(rename = "/federation/stats")]
    federation_stats: FederationStats,

    #[serde(rename = "/following/create")]
    following_create: FollowingCreate,

    #[serde(rename = "/following/delete")]
    following_delete: FollowingDeleteClass,

    #[serde(rename = "/following/update")]
    following_update: FollowingUpdate,

    #[serde(rename = "/following/update-all")]
    following_update_all: DriveFilesUploadFromUrl,

    #[serde(rename = "/following/invalidate")]
    following_invalidate: FollowingDeleteClass,

    #[serde(rename = "/following/requests/accept")]
    following_requests_accept: FollowingRequestsAccept,

    #[serde(rename = "/following/requests/cancel")]
    following_requests_cancel: FollowingRequestsCancel,

    #[serde(rename = "/following/requests/list")]
    following_requests_list: AdminAvatarDecorationsList,

    #[serde(rename = "/following/requests/reject")]
    following_requests_reject: FollowingRequestsReject,

    #[serde(rename = "/gallery/featured")]
    gallery_featured: AdminAbuseReportNotificationRecipientList,

    #[serde(rename = "/gallery/popular")]
    gallery_popular: ChannelsFeatured,

    #[serde(rename = "/gallery/posts")]
    gallery_posts: AdminAdList,

    #[serde(rename = "/gallery/posts/create")]
    gallery_posts_create: GalleryPostsAte,

    #[serde(rename = "/gallery/posts/delete")]
    gallery_posts_delete: GalleryPostsDelete,

    #[serde(rename = "/gallery/posts/like")]
    gallery_posts_like: GalleryPostsLike,

    #[serde(rename = "/gallery/posts/show")]
    gallery_posts_show: GalleryPostsShow,

    #[serde(rename = "/gallery/posts/unlike")]
    gallery_posts_unlike: GalleryPostsUnlike,

    #[serde(rename = "/gallery/posts/update")]
    gallery_posts_update: GalleryPostsAte,

    #[serde(rename = "/get-online-users-count")]
    get_online_users_count: GetOnlineUsersCount,

    #[serde(rename = "/get-avatar-decorations")]
    get_avatar_decorations: Endpoints,

    #[serde(rename = "/hashtags/list")]
    hashtags_list: AdminAdList,

    #[serde(rename = "/hashtags/search")]
    hashtags_search: HashtagsSearch,

    #[serde(rename = "/hashtags/show")]
    hashtags_show: HashtagsShow,

    #[serde(rename = "/hashtags/trend")]
    hashtags_trend: HashtagsTrend,

    #[serde(rename = "/hashtags/users")]
    hashtags_users: AdminDriveFiles,

    #[serde(rename = "/i")]
    i: I,

    #[serde(rename = "/i/2fa/done")]
    i_2_fa_done: I2FaDone,

    #[serde(rename = "/i/2fa/key-done")]
    i_2_fa_key_done: I2FaKeyDone,

    #[serde(rename = "/i/2fa/password-less")]
    i_2_fa_password_less: I2FaPasswordLess,

    #[serde(rename = "/i/2fa/register-key")]
    i_2_fa_register_key: I2FaRegisterKey,

    #[serde(rename = "/i/2fa/register")]
    i_2_fa_register: I2_FaRegister,

    #[serde(rename = "/i/2fa/update-key")]
    i_2_fa_update_key: I2FaUpdateKey,

    #[serde(rename = "/i/2fa/remove-key")]
    i_2_fa_remove_key: I2FaRemoveKey,

    #[serde(rename = "/i/2fa/unregister")]
    i_2_fa_unregister: I2_FaRegister,

    #[serde(rename = "/i/apps")]
    i_apps: IApps,

    #[serde(rename = "/i/authorized-apps")]
    i_authorized_apps: IAuthorizedApps,

    #[serde(rename = "/i/claim-achievement")]
    i_claim_achievement: AdminAvatarDecorationsCreate,

    #[serde(rename = "/i/change-password")]
    i_change_password: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/i/delete-account")]
    i_delete_account: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/i/export-blocking")]
    i_export_blocking: DriveFilesUploadFromUrl,

    #[serde(rename = "/i/export-following")]
    i_export_following: DriveFilesUploadFromUrl,

    #[serde(rename = "/i/export-mute")]
    i_export_mute: DriveFilesUploadFromUrl,

    #[serde(rename = "/i/export-notes")]
    i_export_notes: DriveFilesUploadFromUrl,

    #[serde(rename = "/i/export-clips")]
    i_export_clips: DriveFilesUploadFromUrl,

    #[serde(rename = "/i/export-favorites")]
    i_export_favorites: DriveFilesUploadFromUrl,

    #[serde(rename = "/i/export-user-lists")]
    i_export_user_lists: DriveFilesUploadFromUrl,

    #[serde(rename = "/i/export-antennas")]
    i_export_antennas: DriveFilesUploadFromUrl,

    #[serde(rename = "/i/favorites")]
    i_favorites: BlockingList,

    #[serde(rename = "/i/gallery/likes")]
    i_gallery_likes: AdminAvatarDecorationsList,

    #[serde(rename = "/i/gallery/posts")]
    i_gallery_posts: BlockingList,

    #[serde(rename = "/i/import-blocking")]
    i_import_blocking: IImport,

    #[serde(rename = "/i/import-following")]
    i_import_following: IImportFollowing,

    #[serde(rename = "/i/import-muting")]
    i_import_muting: IImport,

    #[serde(rename = "/i/import-user-lists")]
    i_import_user_lists: IImport,

    #[serde(rename = "/i/import-antennas")]
    i_import_antennas: IImportAntennas,

    #[serde(rename = "/i/notifications")]
    i_notifications: INotifications,

    #[serde(rename = "/i/notifications-grouped")]
    i_notifications_grouped: INotifications,

    #[serde(rename = "/i/page-likes")]
    i_page_likes: Likes,

    #[serde(rename = "/i/pages")]
    i_pages: BlockingList,

    #[serde(rename = "/i/pin")]
    i_pin: IPin,

    #[serde(rename = "/i/read-all-unread-notes")]
    i_read_all_unread_notes: AdminDriveCleanRemoteFiles,

    #[serde(rename = "/i/read-announcement")]
    i_read_announcement: AdminAvatarDecorationsCreate,

    #[serde(rename = "/i/regenerate-token")]
    i_regenerate_token: AdminAvatarDecorationsCreate,

    #[serde(rename = "/i/registry/get-all")]
    i_registry_get_all: IRegistryGetAllClass,

    #[serde(rename = "/i/registry/get-detail")]
    i_registry_get_detail: IRegistryGetDetail,

    #[serde(rename = "/i/registry/get")]
    i_registry_get: IRegistryGetClass,

    #[serde(rename = "/i/registry/keys-with-type")]
    i_registry_keys_with_type: IRegistryGetAllClass,

    #[serde(rename = "/i/registry/keys")]
    i_registry_keys: IRegistryGetAllClass,

    #[serde(rename = "/i/registry/remove")]
    i_registry_remove: IRegistryGetClass,

    #[serde(rename = "/i/registry/scopes-with-domain")]
    i_registry_scopes_with_domain: IRegistryScopesWithDomain,

    #[serde(rename = "/i/registry/set")]
    i_registry_set: AdminAvatarDecorationsCreate,

    #[serde(rename = "/i/revoke-token")]
    i_revoke_token: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/i/signin-history")]
    i_signin_history: AdminInviteList,

    #[serde(rename = "/i/unpin")]
    i_unpin: IUnpin,

    #[serde(rename = "/i/update-email")]
    i_update_email: IUpdateEmail,

    #[serde(rename = "/i/update")]
    i_update: IUpdate,

    #[serde(rename = "/i/move")]
    i_move: IMove,

    #[serde(rename = "/i/webhooks/create")]
    i_webhooks_create: IWebhooksCreate,

    #[serde(rename = "/i/webhooks/list")]
    i_webhooks_list: AdminGetIndexStats,

    #[serde(rename = "/i/webhooks/show")]
    i_webhooks_show: IWebhooks,

    #[serde(rename = "/i/webhooks/update")]
    i_webhooks_update: IWebhooksUpdate,

    #[serde(rename = "/i/webhooks/delete")]
    i_webhooks_delete: IWebhooks,

    #[serde(rename = "/i/webhooks/test")]
    i_webhooks_test: Test,

    #[serde(rename = "/invite/create")]
    invite_create: InviteCreate,

    #[serde(rename = "/invite/delete")]
    invite_delete: InviteDelete,

    #[serde(rename = "/invite/list")]
    invite_list: BlockingList,

    #[serde(rename = "/invite/limit")]
    invite_limit: InviteLimit,

    #[serde(rename = "/meta")]
    meta: Meta,

    #[serde(rename = "/emojis")]
    emojis: Emojis,

    #[serde(rename = "/emoji")]
    emoji: Emoji,

    #[serde(rename = "/miauth/gen-token")]
    miauth_gen_token: MiauthGenToken,

    #[serde(rename = "/mute/create")]
    mute_create: MuteCreate,

    #[serde(rename = "/mute/delete")]
    mute_delete: MuteDelete,

    #[serde(rename = "/mute/list")]
    mute_list: BlockingList,

    #[serde(rename = "/renote-mute/create")]
    renote_mute_create: RenoteMuteCreate,

    #[serde(rename = "/renote-mute/delete")]
    renote_mute_delete: MuteDelete,

    #[serde(rename = "/renote-mute/list")]
    renote_mute_list: BlockingList,

    #[serde(rename = "/my/apps")]
    my_apps: MyApps,

    #[serde(rename = "/notes")]
    notes: AdminAbuseReportNotificationRecipientList,

    #[serde(rename = "/notes/children")]
    notes_children: AdminDriveFiles,

    #[serde(rename = "/notes/clips")]
    notes_clips: NotesClips,

    #[serde(rename = "/notes/conversation")]
    notes_conversation: NotesConversation,

    #[serde(rename = "/notes/create")]
    notes_create: NotesCreate,

    #[serde(rename = "/notes/delete")]
    notes_delete: NotesDelete,

    #[serde(rename = "/notes/favorites/create")]
    notes_favorites_create: NotesFavoritesCreate,

    #[serde(rename = "/notes/favorites/delete")]
    notes_favorites_delete: NotesFavoritesDelete,

    #[serde(rename = "/notes/featured")]
    notes_featured: NotesFeatured,

    #[serde(rename = "/notes/global-timeline")]
    notes_global_timeline: NotesGlobalTimeline,

    #[serde(rename = "/notes/hybrid-timeline")]
    notes_hybrid_timeline: NotesHybridTimeline,

    #[serde(rename = "/notes/local-timeline")]
    notes_local_timeline: NotesLocalTimeline,

    #[serde(rename = "/notes/mentions")]
    notes_mentions: AdminInviteList,

    #[serde(rename = "/notes/polls/recommendation")]
    notes_polls_recommendation: AdminAbuseReportNotificationRecipientList,

    #[serde(rename = "/notes/polls/vote")]
    notes_polls_vote: NotesPollsVote,

    #[serde(rename = "/notes/reactions")]
    notes_reactions: NotesReactions,

    #[serde(rename = "/notes/reactions/create")]
    notes_reactions_create: NotesReactionsCreate,

    #[serde(rename = "/notes/reactions/delete")]
    notes_reactions_delete: NotesReactionsDelete,

    #[serde(rename = "/notes/renotes")]
    notes_renotes: NotesRenotes,

    #[serde(rename = "/notes/replies")]
    notes_replies: AdminDriveFiles,

    #[serde(rename = "/notes/search-by-tag")]
    notes_search_by_tag: AdminDriveFiles,

    #[serde(rename = "/notes/search")]
    notes_search: NotesSearch,

    #[serde(rename = "/notes/show")]
    notes_show: IUnpin,

    #[serde(rename = "/notes/state")]
    notes_state: NotesState,

    #[serde(rename = "/notes/thread-muting/create")]
    notes_thread_muting_create: Notes,

    #[serde(rename = "/notes/thread-muting/delete")]
    notes_thread_muting_delete: NotesThreadMutingDelete,

    #[serde(rename = "/notes/timeline")]
    notes_timeline: AdminAdList,

    #[serde(rename = "/notes/translate")]
    notes_translate: NotesTranslate,

    #[serde(rename = "/notes/unrenote")]
    notes_unrenote: Notes,

    #[serde(rename = "/notes/user-list-timeline")]
    notes_user_list_timeline: NotesUserListTimeline,

    #[serde(rename = "/notifications/create")]
    notifications_create: DriveFilesUploadFromUrl,

    #[serde(rename = "/notifications/flush")]
    notifications_flush: AdminDriveCleanRemoteFiles,

    #[serde(rename = "/notifications/mark-all-as-read")]
    notifications_mark_all_as_read: AdminDriveCleanRemoteFiles,

    #[serde(rename = "/notifications/test-notification")]
    notifications_test_notification: DriveFilesUploadFromUrl,

    #[serde(rename = "/page-push")]
    page_push: PagePush,

    #[serde(rename = "/pages/create")]
    pages_create: PagesCreate,

    #[serde(rename = "/pages/delete")]
    pages_delete: PagesDelete,

    #[serde(rename = "/pages/featured")]
    pages_featured: ChannelsFeatured,

    #[serde(rename = "/pages/like")]
    pages_like: PagesLike,

    #[serde(rename = "/pages/show")]
    pages_show: PagesShow,

    #[serde(rename = "/pages/unlike")]
    pages_unlike: PagesUnlike,

    #[serde(rename = "/pages/update")]
    pages_update: PagesUpdate,

    #[serde(rename = "/flash/create")]
    flash_create: FlashCreate,

    #[serde(rename = "/flash/delete")]
    flash_delete: FlashDelete,

    #[serde(rename = "/flash/featured")]
    flash_featured: ChannelsFeatured,

    #[serde(rename = "/flash/like")]
    flash_like: FlashLike,

    #[serde(rename = "/flash/show")]
    flash_show: FlashShow,

    #[serde(rename = "/flash/unlike")]
    flash_unlike: FlashUnlike,

    #[serde(rename = "/flash/update")]
    flash_update: FlashUpdate,

    #[serde(rename = "/flash/my")]
    flash_my: BlockingList,

    #[serde(rename = "/flash/my-likes")]
    flash_my_likes: Likes,

    #[serde(rename = "/ping")]
    ping: Ping,

    #[serde(rename = "/pinned-users")]
    pinned_users: ChannelsFeatured,

    #[serde(rename = "/promo/read")]
    promo_read: NotesThreadMutingDelete,

    #[serde(rename = "/roles/list")]
    roles_list: AdminGetIndexStats,

    #[serde(rename = "/roles/show")]
    roles_show: AdminRolesDelete,

    #[serde(rename = "/roles/users")]
    roles_users: RolesUsers,

    #[serde(rename = "/roles/notes")]
    roles_notes: RolesNotes,

    #[serde(rename = "/request-reset-password")]
    request_reset_password: DriveFilesUploadFromUrl,

    #[serde(rename = "/reset-db")]
    reset_db: AdminAvatarDecorationsUpdate,

    #[serde(rename = "/reset-password")]
    reset_password: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/server-info")]
    server_info: ServerInfo,

    #[serde(rename = "/stats")]
    stats: Endpoints,

    #[serde(rename = "/sw/show-registration")]
    sw_show_registration: SwShowRegistration,

    #[serde(rename = "/sw/update-registration")]
    sw_update_registration: SwUpdateRegistration,

    #[serde(rename = "/sw/register")]
    sw_register: SwRegister,

    #[serde(rename = "/sw/unregister")]
    sw_unregister: AdminAvatarDecorationsCreate,

    #[serde(rename = "/test")]
    test: TestClass,

    #[serde(rename = "/username/available")]
    username_available: UsernameAvailable,

    #[serde(rename = "/users")]
    users: AdminAbuseReportNotificationRecipientList,

    #[serde(rename = "/users/clips")]
    users_clips: UsersClipsClass,

    #[serde(rename = "/users/followers")]
    users_followers: UsersFollow,

    #[serde(rename = "/users/following")]
    users_following: UsersFollow,

    #[serde(rename = "/users/gallery/posts")]
    users_gallery_posts: UsersClipsClass,

    #[serde(rename = "/users/get-frequently-replied-users")]
    users_get_frequently_replied_users: UsersGetFrequentlyRepliedUsers,

    #[serde(rename = "/users/featured-notes")]
    users_featured_notes: UsersFeaturedNotes,

    #[serde(rename = "/users/lists/create")]
    users_lists_create: UsersListsCreate,

    #[serde(rename = "/users/lists/delete")]
    users_lists_delete: UsersListsDelete,

    #[serde(rename = "/users/lists/list")]
    users_lists_list: UsersListsList,

    #[serde(rename = "/users/lists/pull")]
    users_lists_pull: UsersListsPull,

    #[serde(rename = "/users/lists/push")]
    users_lists_push: UsersListsPush,

    #[serde(rename = "/users/lists/show")]
    users_lists_show: UsersListsShow,

    #[serde(rename = "/users/lists/favorite")]
    users_lists_favorite: UsersListsFavorite,

    #[serde(rename = "/users/lists/unfavorite")]
    users_lists_unfavorite: UsersListsFavorite,

    #[serde(rename = "/users/lists/update")]
    users_lists_update: UsersListsUpdate,

    #[serde(rename = "/users/lists/create-from-public")]
    users_lists_create_from_public: UsersListsCreateFromPublic,

    #[serde(rename = "/users/lists/update-membership")]
    users_lists_update_membership: UsersListsUpdateMembership,

    #[serde(rename = "/users/lists/get-memberships")]
    users_lists_get_memberships: UsersListsGetMemberships,

    #[serde(rename = "/users/notes")]
    users_notes: UsersNotes,

    #[serde(rename = "/users/pages")]
    users_pages: UsersClipsClass,

    #[serde(rename = "/users/flashs")]
    users_flashs: UsersClipsClass,

    #[serde(rename = "/users/reactions")]
    users_reactions: UsersReactions,

    #[serde(rename = "/users/recommendation")]
    users_recommendation: MyApps,

    #[serde(rename = "/users/relation")]
    users_relation: UsersRelation,

    #[serde(rename = "/users/report-abuse")]
    users_report_abuse: UsersReportAbuse,

    #[serde(rename = "/users/search-by-username-and-host")]
    users_search_by_username_and_host: AdminAbuseReportNotificationRecipientList,

    #[serde(rename = "/users/search")]
    users_search: AdminInviteList,

    #[serde(rename = "/users/show")]
    users_show: UsersShow,

    #[serde(rename = "/users/achievements")]
    users_achievements: UsersAchievements,

    #[serde(rename = "/users/update-memo")]
    users_update_memo: UsersUpdateMemo,

    #[serde(rename = "/fetch-rss")]
    fetch_rss: FetchRss,

    #[serde(rename = "/fetch-external-resources")]
    fetch_external_resources: FetchExternalResources,

    #[serde(rename = "/retention")]
    retention: Retention,

    #[serde(rename = "/bubble-game/register")]
    bubble_game_register: BubbleGameRegister,

    #[serde(rename = "/bubble-game/ranking")]
    bubble_game_ranking: BubbleGameRanking,

    #[serde(rename = "/reversi/cancel-match")]
    reversi_cancel_match: AdminEmojiAddAliasesBulk,

    #[serde(rename = "/reversi/games")]
    reversi_games: Reversi,

    #[serde(rename = "/reversi/match")]
    reversi_match: ReversiMatch,

    #[serde(rename = "/reversi/invitations")]
    reversi_invitations: Reversi,

    #[serde(rename = "/reversi/show-game")]
    reversi_show_game: ReversiShowGame,

    #[serde(rename = "/reversi/surrender")]
    reversi_surrender: ReversiSurrender,

    #[serde(rename = "/reversi/verify")]
    reversi_verify: ReversiVerify,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAbuseReportNotificationRecipientAte {
    post: AdminAbuseReportNotificationRecipientCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAbuseReportNotificationRecipientCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: PurpleRequestBody,

    responses: HashMap<String, PurpleResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleRequestBody {
    required: bool,

    content: PurpleContent,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleContent {
    #[serde(rename = "application/json")]
    application_json: PurpleApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleApplicationJson {
    schema: PurpleSchema,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleSchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties2,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties2 {
    is_active: IdElement,

    name: UrlClass,

    method: Method,

    user_id: StartsAtClass,

    system_webhook_id: StartsAtClass,

    id: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlClass {
    #[serde(rename = "type")]
    name_type: TentacledType,

    min_length: i64,

    max_length: i64,

    pattern: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleResponse {
    description: ResponseDescription,

    content: FluffyContent,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyContent {
    #[serde(rename = "application/json")]
    application_json: FluffyApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyApplicationJson {
    schema: Blockee,

    examples: Option<PurpleExamples>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PurpleExamples {
    correlation_check_email: Option<AuthenticationFailedValue>,

    correlation_check_webhook: Option<AuthenticationFailedValue>,

    email_address_not_set: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthenticationFailedValue {
    value: ExampleValue,
}

#[derive(Serialize, Deserialize)]
pub struct ExampleValue {
    error: ValueError,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueError {
    message: String,

    code: String,

    id: String,

    http_status_code: Option<i64>,

    kind: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ResponseDescription {
    #[serde(rename = "Authentication error")]
    AuthenticationError,

    #[serde(rename = "Client error")]
    ClientError,

    #[serde(rename = "Forbidden error")]
    ForbiddenError,

    #[serde(rename = "I'm Ai")]
    IMAi,

    #[serde(rename = "Internal server error")]
    InternalServerError,

    #[serde(rename = "OK (with results)")]
    OkWithResults,

    #[serde(rename = "OK (without any results)")]
    OkWithoutAnyResults,

    #[serde(rename = "To many requests")]
    ToManyRequests,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Security {
    bearer_auth: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PurpleTag {
    Admin,

    #[serde(rename = "reset password")]
    ResetPassword,
}

#[derive(Serialize, Deserialize)]
pub struct AdminDelete {
    post: AdminAbuseReportNotificationRecipientDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAbuseReportNotificationRecipientDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: Option<FluffyRequestBody>,

    responses: HashMap<String, FluffyResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyRequestBody {
    required: bool,

    content: TentacledContent,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledContent {
    #[serde(rename = "application/json")]
    application_json: TentacledApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledApplicationJson {
    schema: FluffySchema,
}

#[derive(Serialize, Deserialize)]
pub struct FluffySchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties3,

    required: Vec<Format>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties3 {
    id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyResponse {
    description: ResponseDescription,

    content: Option<StickyContent>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyContent {
    #[serde(rename = "application/json")]
    application_json: StickyApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct StickyApplicationJson {
    schema: OneOf,

    examples: FluffyExamples,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FluffyExamples {
    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAbuseReportNotificationRecipientList {
    post: AdminAbuseReportNotificationRecipientListPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAbuseReportNotificationRecipientListPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Option<Vec<Security>>,

    request_body: Option<TentacledRequestBody>,

    responses: HashMap<String, TentacledResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledRequestBody {
    required: bool,

    content: IndigoContent,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoContent {
    #[serde(rename = "application/json")]
    application_json: IndigoApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoApplicationJson {
    schema: TentacledSchema,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledSchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties4,

    required: Option<Vec<String>>,

    any_of: Option<Vec<SchemaAnyOf>>,
}

#[derive(Serialize, Deserialize)]
pub struct SchemaAnyOf {
    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties4 {
    method: Option<On>,

    since_id: Option<StartsAtClass>,

    until_id: Option<StartsAtClass>,

    limit: Option<Limit>,

    query: Option<IdElement>,

    #[serde(rename = "type")]
    properties_type: Option<NoteSearchableScope>,

    name: Option<IdElement>,

    parent_id: Option<PurpleId>,

    host: Option<CategoryClass>,

    blocked: Option<ImageUrl>,

    not_responding: Option<ImageUrl>,

    suspended: Option<ImageUrl>,

    silenced: Option<ImageUrl>,

    federating: Option<ImageUrl>,

    subscribing: Option<ImageUrl>,

    publishing: Option<ImageUrl>,

    offset: Option<NotesPerOneAd>,

    sort: Option<Sort>,

    local: Option<CaseSensitive>,

    reply: Option<IdElement>,

    renote: Option<IdElement>,

    with_files: Option<IdElement>,

    poll: Option<IdElement>,

    exclude_channels: Option<CaseSensitive>,

    state: Option<NoteSearchableScope>,

    origin: Option<NoteSearchableScope>,

    hostname: Option<Hostname>,

    user_id: Option<StartsAtClass>,

    detail: Option<CaseSensitive>,

    username: Option<ImageUrl>,
}

#[derive(Serialize, Deserialize)]
pub struct Hostname {
    #[serde(rename = "type")]
    hostname_type: Vec<TentacledType>,

    #[serde(rename = "default")]
    hostname_default: Option<serde_json::Value>,

    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Limit {
    #[serde(rename = "type")]
    limit_type: TentacledType,

    minimum: i64,

    maximum: i64,

    #[serde(rename = "default")]
    limit_default: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleId {
    #[serde(rename = "type")]
    id_type: Vec<TentacledType>,

    format: Format,

    #[serde(rename = "default")]
    id_default: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Sort {
    #[serde(rename = "type")]
    sort_type: SortType,

    #[serde(rename = "enum")]
    sort_enum: Vec<Option<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SortType {
    Enum(TentacledType),

    EnumArray(Vec<TentacledType>),
}

#[derive(Serialize, Deserialize)]
pub struct TentacledResponse {
    description: ResponseDescription,

    content: IndecentContent,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentContent {
    #[serde(rename = "application/json")]
    application_json: IndecentApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentApplicationJson {
    schema: StickySchema,

    examples: Option<FluffyExamples>,
}

#[derive(Serialize, Deserialize)]
pub struct StickySchema {
    #[serde(rename = "type")]
    schema_type: Option<UsersType>,

    items: Option<Blockee>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAbuseReportNotificationRecipientShow {
    post: AdminAbuseReportNotificationRecipientShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAbuseReportNotificationRecipientShowPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: FluffyRequestBody,

    responses: HashMap<String, StickyResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyResponse {
    description: ResponseDescription,

    content: HilariousContent,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousContent {
    #[serde(rename = "application/json")]
    application_json: HilariousApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousApplicationJson {
    schema: Blockee,

    examples: Option<TentacledExamples>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TentacledExamples {
    no_such_recipient: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAbuseUserReports {
    post: AdminAbuseUserReportsPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAbuseUserReportsPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: StickyRequestBody,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyRequestBody {
    required: bool,

    content: AmbitiousContent,
}

#[derive(Serialize, Deserialize)]
pub struct AmbitiousContent {
    #[serde(rename = "application/json")]
    application_json: AmbitiousApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct AmbitiousApplicationJson {
    schema: IndigoSchema,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoSchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties5,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties5 {
    limit: Limit,

    since_id: StartsAtClass,

    until_id: StartsAtClass,

    state: FeedbackUrl,

    reporter_origin: NoteSearchableScope,

    target_user_origin: NoteSearchableScope,

    forwarded: CaseSensitive,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoResponse {
    description: ResponseDescription,

    content: Option<CunningContent>,
}

#[derive(Serialize, Deserialize)]
pub struct CunningContent {
    #[serde(rename = "application/json")]
    application_json: CunningApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct CunningApplicationJson {
    schema: IndecentSchema,

    examples: Option<StickyExamples>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StickyExamples {
    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndecentSchema {
    #[serde(rename = "$ref")]
    schema_ref: Option<String>,

    #[serde(rename = "type")]
    schema_type: Option<IndigoType>,

    items: Option<FluffyItems>,

    properties: Option<Properties8>,

    required: Option<Vec<String>>,

    additional_properties: Option<SchemaAdditionalProperties>,

    example: Option<SchemaExample>,

    one_of: Option<Vec<PurpleOneOf>>,
}

#[derive(Serialize, Deserialize)]
pub struct SchemaAdditionalProperties {
    #[serde(rename = "type")]
    additional_properties_type: TentacledType,

    properties: Option<AdditionalPropertiesProperties>,

    required: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct AdditionalPropertiesProperties {
    count: IdElement,

    size: IdElement,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaExample {
    ExampleExampleClass(ExampleExampleClass),

    UnionArray(Vec<PurpleExample>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleExample {
    String(String),

    UnionArray(Vec<FluffyExample>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyExample {
    Integer(i64),

    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct ExampleExampleClass {
    migrations: Migrations,
}

#[derive(Serialize, Deserialize)]
pub struct Migrations {
    count: i64,

    size: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyItems {
    #[serde(rename = "type")]
    items_type: Option<UsersType>,

    #[serde(rename = "$ref")]
    items_ref: Option<String>,

    properties: Option<Properties6>,

    required: Option<Vec<String>>,

    prefix_items: Option<Vec<IdElement>>,

    unevaluated_items: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties6 {
    id: Option<PropertiesMd5>,

    created_at: Option<StartsAtClass>,

    comment: Option<IdElement>,

    resolved: Option<RequireSetup>,

    reporter_id: Option<StartsAtClass>,

    target_user_id: Option<StartsAtClass>,

    assignee_id: Option<BannerUrlClass>,

    reporter: Option<Blockee>,

    target_user: Option<Blockee>,

    assignee: Option<Folder>,

    updated_at: Option<BannerUrlClass>,

    text: Option<IdElement>,

    title: Option<IdElement>,

    image_url: Option<ImageUrl>,

    reads: Option<IdElement>,

    name: Option<IdElement>,

    description: Option<IdElement>,

    url: Option<IdElement>,

    role_ids_that_can_be_used_this_decoration: Option<PinnedNoteIds>,

    aliases: Option<Users>,

    category: Option<ImageUrl>,

    host: Option<CategoryClass>,

    tablename: Option<IdElement>,

    indexname: Option<IdElement>,

    ip: Option<IdElement>,

    inbox: Option<StartsAtClass>,

    status: Option<NoteSearchableScope>,

    #[serde(rename = "type")]
    properties_type: Option<IdElement>,

    info: Option<IdElement>,

    user_id: Option<StartsAtClass>,

    user: Option<PurpleUser>,

    score: Option<IdElement>,

    follower: Option<Blockee>,

    followee: Option<Blockee>,

    tag: Option<IdElement>,

    chart: Option<Users>,

    users_count: Option<IdElement>,

    callback_url: Option<ImageUrl>,

    permission: Option<Permission>,

    is_authorized: Option<IdElement>,

    post: Option<Blockee>,

    on: Option<On>,

    secret: Option<IdElement>,

    active: Option<IdElement>,

    latest_sent_at: Option<BannerUrlClass>,

    latest_status: Option<ImageUrl>,

    users: Option<IdElement>,

    data: Option<ReactionEmojisClass>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
    #[serde(rename = "type")]
    permission_type: UsersType,

    unique_items: bool,

    items: IdElement,

    max_items: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleUser {
    #[serde(rename = "type")]
    user_type: TentacledType,

    #[serde(rename = "$ref")]
    user_ref: Option<String>,

    all_of: Option<Vec<OneOf>>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleOneOf {
    #[serde(rename = "type")]
    one_of_type: UsersType,

    #[serde(rename = "$ref")]
    one_of_ref: Option<String>,

    properties: Option<Properties7>,

    required: Option<Vec<String>>,

    items: Box<Option<OneOfItems>>,
}

#[derive(Serialize, Deserialize)]
pub struct OneOfItems {
    #[serde(rename = "type")]
    items_type: UsersType,

    properties: Option<Properties7>,

    required: Option<Vec<String>>,

    items: Box<Option<OneOfItems>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties7 {
    id: StartsAtClass,

    is_following: IdElement,

    has_pending_follow_request_from_you: IdElement,

    has_pending_follow_request_to_you: IdElement,

    is_followed: IdElement,

    is_blocking: IdElement,

    is_blocked: IdElement,

    is_muted: IdElement,

    is_renote_muted: IdElement,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties8 {
    token: Option<IdElement>,

    id: Option<PropertiesMd5>,

    created_at: Option<StartsAtClass>,

    updated_at: Option<BannerUrlClass>,

    title: Option<IdElement>,

    text: Option<IdElement>,

    image_url: Option<ImageUrl>,

    cache_remote_files: Option<IdElement>,

    cache_remote_sensitive_files: Option<IdElement>,

    email_required_for_signup: Option<IdElement>,

    enable_hcaptcha: Option<IdElement>,

    hcaptcha_site_key: Option<ImageUrl>,

    enable_mcaptcha: Option<IdElement>,

    mcaptcha_site_key: Option<ImageUrl>,

    mcaptcha_instance_url: Option<ImageUrl>,

    enable_recaptcha: Option<IdElement>,

    recaptcha_site_key: Option<ImageUrl>,

    enable_turnstile: Option<IdElement>,

    turnstile_site_key: Option<ImageUrl>,

    sw_publickey: Option<ImageUrl>,

    mascot_image_url: Option<FeedbackUrl>,

    banner_url: Option<ImageUrl>,

    server_error_image_url: Option<ImageUrl>,

    info_image_url: Option<ImageUrl>,

    not_found_image_url: Option<ImageUrl>,

    icon_url: Option<ImageUrl>,

    app192_icon_url: Option<ImageUrl>,

    app512_icon_url: Option<ImageUrl>,

    enable_email: Option<IdElement>,

    enable_service_worker: Option<IdElement>,

    translator_available: Option<IdElement>,

    silenced_hosts: Option<Users>,

    media_silenced_hosts: Option<Users>,

    pinned_users: Option<Users>,

    hidden_tags: Option<Users>,

    blocked_hosts: Option<Users>,

    sensitive_words: Option<Users>,

    prohibited_words: Option<Users>,

    banned_email_domains: Option<Users>,

    preserved_usernames: Option<Users>,

    hcaptcha_secret_key: Option<ImageUrl>,

    mcaptcha_secret_key: Option<ImageUrl>,

    recaptcha_secret_key: Option<ImageUrl>,

    turnstile_secret_key: Option<ImageUrl>,

    sensitive_media_detection: Option<IdElement>,

    sensitive_media_detection_sensitivity: Option<IdElement>,

    set_sensitive_flag_automatically: Option<IdElement>,

    enable_sensitive_media_detection_for_videos: Option<IdElement>,

    proxy_account_id: Option<BannerUrlClass>,

    email: Option<ImageUrl>,

    smtp_secure: Option<IdElement>,

    smtp_host: Option<ImageUrl>,

    smtp_port: Option<ImageUrl>,

    smtp_user: Option<ImageUrl>,

    smtp_pass: Option<ImageUrl>,

    sw_private_key: Option<ImageUrl>,

    use_object_storage: Option<IdElement>,

    object_storage_base_url: Option<ImageUrl>,

    object_storage_bucket: Option<ImageUrl>,

    object_storage_prefix: Option<ImageUrl>,

    object_storage_endpoint: Option<ImageUrl>,

    object_storage_region: Option<ImageUrl>,

    object_storage_port: Option<ImageUrl>,

    object_storage_access_key: Option<ImageUrl>,

    object_storage_secret_key: Option<ImageUrl>,

    #[serde(rename = "objectStorageUseSSL")]
    object_storage_use_ssl: Option<IdElement>,

    object_storage_use_proxy: Option<IdElement>,

    object_storage_set_public_read: Option<IdElement>,

    enable_ip_logging: Option<IdElement>,

    enable_active_email_validation: Option<IdElement>,

    enable_verifymail_api: Option<IdElement>,

    verifymail_auth_key: Option<ImageUrl>,

    enable_truemail_api: Option<IdElement>,

    truemail_instance: Option<ImageUrl>,

    truemail_auth_key: Option<ImageUrl>,

    enable_charts_for_remote_user: Option<IdElement>,

    enable_charts_for_federated_instances: Option<IdElement>,

    enable_server_machine_stats: Option<IdElement>,

    enable_identicon_generation: Option<IdElement>,

    manifest_json_override: Option<IdElement>,

    policies: Option<Blockee>,

    enable_fanout_timeline: Option<IdElement>,

    enable_fanout_timeline_db_fallback: Option<IdElement>,

    per_local_user_user_timeline_cache_max: Option<IdElement>,

    per_remote_user_user_timeline_cache_max: Option<IdElement>,

    per_user_home_timeline_cache_max: Option<IdElement>,

    per_user_list_timeline_cache_max: Option<IdElement>,

    enable_reactions_buffering: Option<IdElement>,

    notes_per_one_ad: Option<IdElement>,

    background_image_url: Option<ImageUrl>,

    deepl_auth_key: Option<ImageUrl>,

    deepl_is_pro: Option<IdElement>,

    default_dark_theme: Option<ImageUrl>,

    default_light_theme: Option<ImageUrl>,

    description: Option<Token>,

    disable_registration: Option<IdElement>,

    impressum_url: Option<ImageUrl>,

    maintainer_email: Option<ImageUrl>,

    maintainer_name: Option<ImageUrl>,

    name: Option<ImageUrl>,

    short_name: Option<ImageUrl>,

    object_storage_s3_force_path_style: Option<IdElement>,

    privacy_policy_url: Option<ImageUrl>,

    inquiry_url: Option<ImageUrl>,

    repository_url: Option<ImageUrl>,

    summaly_proxy: Option<SummalyProxy>,

    theme_color: Option<ImageUrl>,

    tos_url: Option<ImageUrl>,

    uri: Option<IdElement>,

    version: Option<IdElement>,

    url_preview_enabled: Option<IdElement>,

    url_preview_timeout: Option<IdElement>,

    url_preview_maximum_content_length: Option<IdElement>,

    url_preview_require_content_length: Option<IdElement>,

    url_preview_user_agent: Option<ImageUrl>,

    url_preview_summary_proxy_url: Option<ImageUrl>,

    deliver: Option<OneOf>,

    inbox: Option<OneOf>,

    db: Option<OneOf>,

    object_storage: Option<OneOf>,

    password: Option<UrlClass>,

    machine: Option<IdElement>,

    os: Option<TypeClass>,

    node: Option<IdElement>,

    psql: Option<IdElement>,

    cpu: Option<Cpu>,

    mem: Option<PurpleMem>,

    fs: Option<PurpleFs>,

    net: Option<Net>,

    email_verified: Option<IdElement>,

    auto_accept_followed: Option<IdElement>,

    no_crawle: Option<IdElement>,

    prevent_ai_learning: Option<IdElement>,

    always_mark_nsfw: Option<IdElement>,

    auto_sensitive: Option<IdElement>,

    careful_bot: Option<IdElement>,

    inject_featured_note: Option<IdElement>,

    receive_announcement_email: Option<IdElement>,

    muted_words: Option<MutedWords>,

    muted_instances: Option<Users>,

    notification_recieve_config: Option<NotificationRecieveConfig>,

    is_moderator: Option<IdElement>,

    is_silenced: Option<IdElement>,

    is_suspended: Option<IdElement>,

    is_hibernated: Option<IdElement>,

    last_active_date: Option<ImageUrl>,

    moderation_note: Option<IdElement>,

    signins: Option<Values>,

    roles: Option<PinnedNotes>,

    role_assigns: Option<RoleAssigns>,

    read_write: Option<Users>,

    read: Option<Users>,

    write: Option<Users>,

    registered_within_week: Option<Users>,

    registered_within_month: Option<Users>,

    registered_within_year: Option<Users>,

    registered_outside_week: Option<Users>,

    registered_outside_month: Option<Users>,

    registered_outside_year: Option<Users>,

    deliver_failed: Option<Users>,

    deliver_succeeded: Option<Users>,

    inbox_received: Option<Users>,

    local: Option<Local>,

    remote: Option<Local>,

    delivered_instances: Option<Users>,

    inbox_instances: Option<Users>,

    stalled: Option<Users>,

    sub: Option<Users>,

    #[serde(rename = "pub")]
    properties_pub: Option<Users>,

    pubsub: Option<Users>,

    sub_active: Option<Users>,

    pub_active: Option<Users>,

    requests: Option<Requests>,

    notes: Option<Followers>,

    users: Option<Followers>,

    following: Option<Followers>,

    followers: Option<Followers>,

    drive: Option<Drive>,

    total_count: Option<Users>,

    total_size: Option<Users>,

    inc_count: Option<Users>,

    inc_size: Option<Users>,

    dec_count: Option<Users>,

    dec_size: Option<Users>,

    total: Option<Users>,

    inc: Option<Users>,

    dec: Option<Users>,

    diffs: Option<Diffs>,

    upv: Option<Pv>,

    pv: Option<Pv>,

    capacity: Option<IdElement>,

    usage: Option<IdElement>,

    available: Option<IdElement>,

    reason: Option<ImageUrl>,

    emojis: Option<PinnedNotes>,

    top_sub_instances: Option<PinnedNotes>,

    other_followers_count: Option<IdElement>,

    top_pub_instances: Option<PinnedNotes>,

    other_following_count: Option<IdElement>,

    image: Option<Image>,

    pagination_links: Option<PaginationLinks>,

    link: Option<IdElement>,

    items: Option<PropertiesItems>,

    feed_url: Option<IdElement>,

    itunes: Option<Itunes>,

    count: Option<IdElement>,

    backup_codes: Option<Users>,

    is_favorited: Option<IdElement>,

    is_muted_thread: Option<IdElement>,

    notes_count: Option<IdElement>,

    original_notes_count: Option<IdElement>,

    users_count: Option<IdElement>,

    original_users_count: Option<IdElement>,

    instances: Option<IdElement>,

    drive_usage_local: Option<IdElement>,

    drive_usage_remote: Option<IdElement>,

    user_id: Option<IdElement>,

    endpoint: Option<IdElement>,

    send_read_message: Option<IdElement>,
}

#[derive(Serialize, Deserialize)]
pub struct Cpu {
    #[serde(rename = "type")]
    cpu_type: TentacledType,

    properties: CpuProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CpuProperties {
    model: IdElement,

    cores: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "type")]
    token_type: SortType,
}

#[derive(Serialize, Deserialize)]
pub struct Diffs {
    #[serde(rename = "type")]
    diffs_type: TentacledType,

    properties: DiffsProperties,

    required: Vec<DiffsRequired>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffsProperties {
    normal: Users,

    reply: Users,

    renote: Users,

    with_file: Users,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DiffsRequired {
    Normal,

    Renote,

    Reply,

    #[serde(rename = "withFile")]
    WithFile,
}

#[derive(Serialize, Deserialize)]
pub struct Drive {
    #[serde(rename = "type")]
    drive_type: TentacledType,

    properties: DriveProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveProperties {
    total_files: Users,

    inc_files: Users,

    dec_files: Users,

    inc_usage: Users,

    dec_usage: Users,
}

#[derive(Serialize, Deserialize)]
pub struct Followers {
    #[serde(rename = "type")]
    followers_type: TentacledType,

    properties: FollowersProperties,

    required: Vec<FollowersRequired>,
}

#[derive(Serialize, Deserialize)]
pub struct FollowersProperties {
    total: Users,

    inc: Users,

    dec: Users,

    diffs: Option<Diffs>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowersRequired {
    Dec,

    Diffs,

    Inc,

    Total,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleFs {
    #[serde(rename = "type")]
    fs_type: TentacledType,

    properties: Properties9,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties9 {
    total: StartsAtClass,

    used: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "type")]
    image_type: TentacledType,

    properties: ImageProperties,

    required: Vec<Format>,
}

#[derive(Serialize, Deserialize)]
pub struct ImageProperties {
    link: IdElement,

    url: IdElement,

    title: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct PropertiesItems {
    #[serde(rename = "type")]
    items_type: UsersType,

    items: TentacledItems,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: Properties10,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties10 {
    link: IdElement,

    guid: IdElement,

    title: IdElement,

    pub_date: IdElement,

    creator: IdElement,

    summary: IdElement,

    content: IdElement,

    iso_date: IdElement,

    categories: Users,

    content_snippet: IdElement,

    enclosure: Enclosure,
}

#[derive(Serialize, Deserialize)]
pub struct Enclosure {
    #[serde(rename = "type")]
    enclosure_type: TentacledType,

    properties: EnclosureProperties,

    required: Vec<Format>,
}

#[derive(Serialize, Deserialize)]
pub struct EnclosureProperties {
    url: IdElement,

    length: IdElement,

    #[serde(rename = "type")]
    properties_type: IdElement,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Itunes {
    #[serde(rename = "type")]
    itunes_type: TentacledType,

    additional_properties: bool,

    properties: ItunesProperties,
}

#[derive(Serialize, Deserialize)]
pub struct ItunesProperties {
    image: IdElement,

    owner: Owner,

    author: IdElement,

    summary: IdElement,

    explicit: IdElement,

    categories: Users,

    keywords: Users,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "type")]
    owner_type: TentacledType,

    properties: OwnerProperties,
}

#[derive(Serialize, Deserialize)]
pub struct OwnerProperties {
    name: IdElement,

    email: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Local {
    #[serde(rename = "type")]
    local_type: TentacledType,

    properties: LocalProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalProperties {
    inc_count: Option<Users>,

    inc_size: Option<Users>,

    dec_count: Option<Users>,

    dec_size: Option<Users>,

    total: Option<Users>,

    inc: Option<Users>,

    dec: Option<Users>,

    diffs: Option<Diffs>,

    followings: Option<Followers>,

    followers: Option<Followers>,

    count: Option<Users>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleMem {
    #[serde(rename = "type")]
    mem_type: TentacledType,

    properties: Properties11,

    required: Vec<FollowersRequired>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties11 {
    total: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct MutedWords {
    #[serde(rename = "type")]
    muted_words_type: UsersType,

    items: StickyItems,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyItems {
    any_of: Vec<Users>,
}

#[derive(Serialize, Deserialize)]
pub struct Net {
    #[serde(rename = "type")]
    net_type: TentacledType,

    properties: NetProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NetProperties {
    interface: TypeClass,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationLinks {
    #[serde(rename = "type")]
    pagination_links_type: TentacledType,

    properties: PaginationLinksProperties,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationLinksProperties {
    #[serde(rename = "self")]
    properties_self: IdElement,

    first: IdElement,

    next: IdElement,

    last: IdElement,

    prev: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Pv {
    #[serde(rename = "type")]
    pv_type: TentacledType,

    properties: PvProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PvProperties {
    user: Users,

    visitor: Users,
}

#[derive(Serialize, Deserialize)]
pub struct Requests {
    #[serde(rename = "type")]
    requests_type: TentacledType,

    properties: RequestsProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestsProperties {
    failed: Users,

    succeeded: Users,

    received: Users,
}

#[derive(Serialize, Deserialize)]
pub struct RoleAssigns {
    #[serde(rename = "type")]
    role_assigns_type: UsersType,

    items: RoleAssignsItems,
}

#[derive(Serialize, Deserialize)]
pub struct RoleAssignsItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: Properties12,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties12 {
    created_at: IdElement,

    expires_at: ImageUrl,

    role_id: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct SummalyProxy {
    #[serde(rename = "type")]
    summaly_proxy_type: Vec<TentacledType>,

    deprecated: bool,

    description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndigoType {
    Enum(UsersType),

    EnumArray(Vec<TentacledType>),
}

#[derive(Serialize, Deserialize)]
pub struct AdminAccountsCreate {
    post: AdminAccountsCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountsCreatePost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    request_body: IndigoRequestBody,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub enum PostDescription {
    #[serde(rename = "No description provided.\n\n**Credential required**: *No*")]
    NoDescriptionProvidedCredentialRequiredNo,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoRequestBody {
    required: bool,

    content: MagentaContent,
}

#[derive(Serialize, Deserialize)]
pub struct MagentaContent {
    #[serde(rename = "application/json")]
    application_json: MagentaApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct MagentaApplicationJson {
    schema: HilariousSchema,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousSchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties13,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties13 {
    username: Username,

    password: Password,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Password {
    #[serde(rename = "type")]
    password_type: TentacledType,

    min_length: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Username {
    #[serde(rename = "type")]
    username_type: TentacledType,

    pattern: Pattern,
}

#[derive(Serialize, Deserialize)]
pub enum Pattern {
    #[serde(rename = "^[a-zA-Z\\/\\-*]+$")]
    AZAZ,

    #[serde(rename = "^[a-zA-Z0-9_]+$")]
    AZAZ09_,

    #[serde(rename = "^\\w{1,20}$")]
    W120,
}

#[derive(Serialize, Deserialize)]
pub struct Admin {
    post: AdminAccountsDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountsDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: IndecentRequestBody,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentRequestBody {
    required: bool,

    content: FriskyContent,
}

#[derive(Serialize, Deserialize)]
pub struct FriskyContent {
    #[serde(rename = "application/json")]
    application_json: FriskyApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct FriskyApplicationJson {
    schema: AmbitiousSchema,
}

#[derive(Serialize, Deserialize)]
pub struct AmbitiousSchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties14,

    required: Vec<PurpleRequired>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties14 {
    user_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PurpleRequired {
    Host,

    Span,

    #[serde(rename = "userId")]
    UserId,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAccountsFindByEmail {
    post: AdminAccountsFindByEmailPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountsFindByEmailPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: HilariousRequestBody,

    responses: HashMap<String, IndecentResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousRequestBody {
    required: bool,

    content: MischievousContent,
}

#[derive(Serialize, Deserialize)]
pub struct MischievousContent {
    #[serde(rename = "application/json")]
    application_json: MischievousApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct MischievousApplicationJson {
    schema: CunningSchema,
}

#[derive(Serialize, Deserialize)]
pub struct CunningSchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties15,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties15 {
    email: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentResponse {
    description: ResponseDescription,

    content: BraggadociousContent,
}

#[derive(Serialize, Deserialize)]
pub struct BraggadociousContent {
    #[serde(rename = "application/json")]
    application_json: BraggadociousApplicationJson,
}

#[derive(Serialize, Deserialize)]
pub struct BraggadociousApplicationJson {
    schema: Blockee,

    examples: Option<IndigoExamples>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct IndigoExamples {
    user_not_found: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAdCreate {
    post: AdminAdCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAdCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: AmbitiousRequestBody,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct AmbitiousRequestBody {
    required: bool,

    content: Content1,
}

#[derive(Serialize, Deserialize)]
pub struct Content1 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson1,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson1 {
    schema: MagentaSchema,
}

#[derive(Serialize, Deserialize)]
pub struct MagentaSchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties16,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties16 {
    url: Password,

    memo: IdElement,

    place: IdElement,

    priority: IdElement,

    ratio: IdElement,

    expires_at: IdElement,

    starts_at: IdElement,

    image_url: Password,

    day_of_week: IdElement,

    id: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAdDelete {
    post: AdminAdDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAdDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: FluffyRequestBody,

    responses: HashMap<String, HilariousResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousResponse {
    description: ResponseDescription,

    content: Option<Content2>,
}

#[derive(Serialize, Deserialize)]
pub struct Content2 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson2,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson2 {
    schema: OneOf,

    examples: IndecentExamples,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct IndecentExamples {
    no_such_ad: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAdList {
    post: AdminAdListPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAdListPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Option<Vec<Security>>,

    request_body: Option<CunningRequestBody>,

    responses: HashMap<String, TentacledResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct CunningRequestBody {
    required: bool,

    content: Content3,
}

#[derive(Serialize, Deserialize)]
pub struct Content3 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson3,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson3 {
    schema: FriskySchema,
}

#[derive(Serialize, Deserialize)]
pub struct FriskySchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties17,

    required: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties17 {
    limit: Option<Limit>,

    since_id: Option<StartsAtClass>,

    until_id: Option<StartsAtClass>,

    publishing: Option<FeedbackUrl>,

    folder_id: Option<PurpleId>,

    #[serde(rename = "type")]
    properties_type: Option<Birthday>,

    sort: Option<Sort>,

    name: Option<IdElement>,

    md5: Option<IdElement>,

    host: Option<IdElement>,

    attached_to_user_only: Option<CaseSensitive>,

    attached_to_local_user_only: Option<CaseSensitive>,

    attached_to_remote_user_only: Option<CaseSensitive>,

    channel_id: Option<BannerUrlClass>,

    since_date: Option<IdElement>,

    until_date: Option<IdElement>,

    allow_partial: Option<CaseSensitive>,

    include_my_renotes: Option<CaseSensitive>,

    include_renoted_my_notes: Option<CaseSensitive>,

    include_local_renotes: Option<CaseSensitive>,

    with_files: Option<CaseSensitive>,

    with_renotes: Option<CaseSensitive>,

    user_id: Option<StartsAtClass>,

    offset: Option<NotesPerOneAd>,
}

#[derive(Serialize, Deserialize)]
pub struct Birthday {
    #[serde(rename = "type")]
    birthday_type: Vec<TentacledType>,

    pattern: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAdUpdate {
    post: AdminAdUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAdUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: AmbitiousRequestBody,

    responses: HashMap<String, HilariousResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAnnouncementsCreate {
    post: AdminAnnouncementsCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAnnouncementsCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: MagentaRequestBody,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct MagentaRequestBody {
    required: bool,

    content: Content4,
}

#[derive(Serialize, Deserialize)]
pub struct Content4 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson4,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson4 {
    schema: MischievousSchema,
}

#[derive(Serialize, Deserialize)]
pub struct MischievousSchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties18,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties18 {
    title: Password,

    text: Password,

    image_url: PurpleImageUrl,

    icon: NoteSearchableScope,

    display: NoteSearchableScope,

    for_existing_users: CaseSensitive,

    silence: CaseSensitive,

    need_confirmation_to_read: CaseSensitive,

    user_id: PurpleId,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleImageUrl {
    #[serde(rename = "type")]
    image_url_type: Vec<TentacledType>,

    min_length: i64,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAnnouncementsDelete {
    post: AdminAnnouncementsDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAnnouncementsDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: FluffyRequestBody,

    responses: HashMap<String, AmbitiousResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct AmbitiousResponse {
    description: ResponseDescription,

    content: Option<Content5>,
}

#[derive(Serialize, Deserialize)]
pub struct Content5 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson5,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson5 {
    schema: Blockee,

    examples: Option<HilariousExamples>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct HilariousExamples {
    no_such_announcement: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAnnouncementsList {
    post: AdminAnnouncementsListPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAnnouncementsListPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: FriskyRequestBody,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct FriskyRequestBody {
    required: bool,

    content: Content6,
}

#[derive(Serialize, Deserialize)]
pub struct Content6 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson6,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson6 {
    schema: BraggadociousSchema,
}

#[derive(Serialize, Deserialize)]
pub struct BraggadociousSchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties19,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties19 {
    limit: Limit,

    since_id: StartsAtClass,

    until_id: StartsAtClass,

    user_id: BannerUrlClass,

    status: NoteSearchableScope,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAnnouncementsUpdate {
    post: AdminAnnouncementsUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAnnouncementsUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: MischievousRequestBody,

    responses: HashMap<String, AmbitiousResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct MischievousRequestBody {
    required: bool,

    content: Content7,
}

#[derive(Serialize, Deserialize)]
pub struct Content7 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson7,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson7 {
    schema: Schema1,
}

#[derive(Serialize, Deserialize)]
pub struct Schema1 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties20,

    required: Vec<Format>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties20 {
    id: StartsAtClass,

    title: Password,

    text: Password,

    image_url: PurpleImageUrl,

    icon: Method,

    display: Method,

    for_existing_users: IdElement,

    silence: IdElement,

    need_confirmation_to_read: IdElement,

    is_active: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAvatarDecorationsCreate {
    post: AdminAvatarDecorationsCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAvatarDecorationsCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Option<Vec<String>>,

    security: Option<Vec<Security>>,

    request_body: Option<BraggadociousRequestBody>,

    responses: HashMap<String, FluffyResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct BraggadociousRequestBody {
    required: bool,

    content: Content8,
}

#[derive(Serialize, Deserialize)]
pub struct Content8 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson8,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson8 {
    schema: Schema2,
}

#[derive(Serialize, Deserialize)]
pub struct Schema2 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties21,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties21 {
    name: Option<PurpleName>,

    description: Option<IdElement>,

    url: Option<Password>,

    role_ids_that_can_be_used_this_decoration: Option<Users>,

    inbox: Option<IdElement>,

    to: Option<IdElement>,

    subject: Option<IdElement>,

    text: Option<IdElement>,

    user_id: Option<StartsAtClass>,

    announcement_id: Option<StartsAtClass>,

    password: Option<IdElement>,

    key: Option<Password>,

    value: Option<Var>,

    scope: Option<Scope>,

    domain: Option<ImageUrl>,

    endpoint: Option<IdElement>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleName {
    #[serde(rename = "type")]
    name_type: TentacledType,

    min_length: Option<i64>,

    #[serde(rename = "enum")]
    name_enum: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Scope {
    #[serde(rename = "type")]
    scope_type: UsersType,

    #[serde(rename = "default")]
    scope_default: Vec<Option<serde_json::Value>>,

    items: Username,
}

#[derive(Serialize, Deserialize)]
pub struct Var {
}

#[derive(Serialize, Deserialize)]
pub struct AdminAvatarDecorationsList {
    post: AdminAvatarDecorationsListPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAvatarDecorationsListPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody1,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody1 {
    required: bool,

    content: Content9,
}

#[derive(Serialize, Deserialize)]
pub struct Content9 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson9,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson9 {
    schema: Schema3,
}

#[derive(Serialize, Deserialize)]
pub struct Schema3 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties22,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties22 {
    limit: Limit,

    since_id: Option<StartsAtClass>,

    until_id: StartsAtClass,

    publishing: Option<FeedbackUrl>,

    user_id: Option<PurpleUserId>,

    query: Option<FeedbackUrl>,

    host: Option<FluffyHost>,

    role_id: Option<StartsAtClass>,

    #[serde(rename = "type")]
    properties_type: Option<PurpleType>,

    is_active: Option<CaseSensitive>,

    clip_id: Option<StartsAtClass>,

    file_id: Option<StartsAtClass>,

    folder_id: Option<PurpleId>,

    note_id: Option<StartsAtClass>,

    my: Option<CaseSensitive>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyHost {
    #[serde(rename = "type")]
    host_type: SortType,

    #[serde(rename = "default")]
    host_default: Option<serde_json::Value>,

    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleType {
    #[serde(rename = "type")]
    type_type: SortType,

    pattern: Option<Pattern>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleUserId {
    #[serde(rename = "type")]
    user_id_type: SortType,

    format: Format,
}

#[derive(Serialize, Deserialize)]
pub struct AdminAvatarDecorationsUpdate {
    post: AdminAvatarDecorationsUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAvatarDecorationsUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Option<Vec<Security>>,

    request_body: Option<RequestBody2>,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody2 {
    required: bool,

    content: Content10,
}

#[derive(Serialize, Deserialize)]
pub struct Content10 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson10,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson10 {
    schema: Schema4,
}

#[derive(Serialize, Deserialize)]
pub struct Schema4 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties23,

    required: Vec<Format>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties23 {
    name: Password,

    description: IdElement,

    url: Password,

    role_ids_that_can_be_used_this_decoration: Users,

    id: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminDriveCleanRemoteFiles {
    post: AdminDriveCleanRemoteFilesPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminDriveCleanRemoteFilesPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    responses: HashMap<String, FluffyResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminDriveFiles {
    post: AdminDriveFilesPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminDriveFilesPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Option<Vec<Security>>,

    request_body: Option<RequestBody3>,

    responses: HashMap<String, TentacledResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody3 {
    required: bool,

    content: Content11,
}

#[derive(Serialize, Deserialize)]
pub struct Content11 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson11,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson11 {
    schema: Schema5,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema5 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties24,

    required: Option<Vec<String>>,

    any_of: Option<Vec<SchemaAnyOf>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties24 {
    limit: Option<Limit>,

    since_id: Option<StartsAtClass>,

    until_id: Option<StartsAtClass>,

    user_id: Option<PurpleUserId>,

    #[serde(rename = "type")]
    properties_type: Option<Birthday>,

    origin: Option<NoteSearchableScope>,

    hostname: Option<Hostname>,

    offset: Option<NotesPerOneAd>,

    sort: Option<Method>,

    state: Option<NoteSearchableScope>,

    username: Option<FeedbackUrl>,

    is_active: Option<CaseSensitive>,

    on: Option<On>,

    folder_id: Option<PurpleId>,

    host: Option<IdElement>,

    tag: Option<Password>,

    note_id: Option<StartsAtClass>,

    reply: Option<FeedbackUrl>,

    renote: Option<FeedbackUrl>,

    with_files: Option<WithFiles>,

    poll: Option<FeedbackUrl>,

    query: Option<Query>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    #[serde(rename = "type")]
    query_type: UsersType,

    description: String,

    items: QueryItems,

    min_items: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryItems {
    #[serde(rename = "type")]
    items_type: UsersType,

    items: Password,

    min_items: i64,
}

#[derive(Serialize, Deserialize)]
pub struct WithFiles {
    #[serde(rename = "type")]
    with_files_type: TentacledType,

    #[serde(rename = "default")]
    with_files_default: bool,

    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdminDriveShowFile {
    post: AdminDriveShowFilePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminDriveShowFilePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody4,

    responses: HashMap<String, CunningResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody4 {
    required: bool,

    content: Content12,
}

#[derive(Serialize, Deserialize)]
pub struct Content12 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson12,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson12 {
    schema: Schema6,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema6 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties25,

    any_of: Vec<SchemaAnyOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties25 {
    file_id: StartsAtClass,

    url: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct CunningResponse {
    description: ResponseDescription,

    content: Content13,
}

#[derive(Serialize, Deserialize)]
pub struct Content13 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson13,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson13 {
    schema: Schema7,

    examples: Option<AmbitiousExamples>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AmbitiousExamples {
    no_such_file: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema7 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties26>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties26 {
    id: PropertiesMd5,

    created_at: StartsAtClass,

    user_id: ReplyIdClass,

    user_host: CategoryClass,

    md5: PropertiesMd5,

    name: TypeClass,

    #[serde(rename = "type")]
    properties_type: TypeClass,

    size: Size,

    comment: ImageUrl,

    blurhash: ImageUrl,

    properties: Properties27,

    stored_internal: OpenRegistrations,

    url: BannerUrlClass,

    thumbnail_url: BannerUrlClass,

    webpublic_url: BannerUrlClass,

    access_key: ImageUrl,

    thumbnail_access_key: ImageUrl,

    webpublic_access_key: ImageUrl,

    uri: ImageUrl,

    src: ImageUrl,

    folder_id: ReplyIdClass,

    is_sensitive: IdElement,

    is_link: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Properties27 {
    #[serde(rename = "type")]
    properties_type: TentacledType,

    properties: Properties28,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties28 {
    width: IdElement,

    height: IdElement,

    orientation: IdElement,

    avg_color: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct AdminEmojiAdd {
    post: AdminEmojiAddPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminEmojiAddPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody5,

    responses: HashMap<String, MagentaResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody5 {
    required: bool,

    content: Content14,
}

#[derive(Serialize, Deserialize)]
pub struct Content14 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson14,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson14 {
    schema: Schema8,
}

#[derive(Serialize, Deserialize)]
pub struct Schema8 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties29,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties29 {
    name: Username,

    file_id: StartsAtClass,

    category: CategoryClass,

    aliases: Users,

    license: ImageUrl,

    is_sensitive: IdElement,

    local_only: IdElement,

    role_ids_that_can_be_used_this_emoji_as_reaction: Users,

    id: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
pub struct MagentaResponse {
    description: ResponseDescription,

    content: Content15,
}

#[derive(Serialize, Deserialize)]
pub struct Content15 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson15,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson15 {
    schema: Blockee,

    examples: Option<CunningExamples>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CunningExamples {
    no_such_file: Option<AuthenticationFailedValue>,

    duplicate_name: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminEmojiAddAliasesBulk {
    post: AdminEmojiAddAliasesBulkPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminEmojiAddAliasesBulkPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Option<Vec<PurpleTag>>,

    security: Option<Vec<Security>>,

    request_body: Option<RequestBody6>,

    responses: HashMap<String, FluffyResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody6 {
    required: bool,

    content: Content16,
}

#[derive(Serialize, Deserialize)]
pub struct Content16 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson16,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson16 {
    schema: Schema9,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema9 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties30,

    required: Option<Vec<String>>,

    any_of: Option<Vec<SchemaAnyOf>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties30 {
    user_id: Option<PurpleUserId>,

    ids: Option<PinnedNoteIds>,

    aliases: Option<Users>,

    file_id: Option<StartsAtClass>,

    category: Option<CategoryClass>,

    license: Option<CategoryClass>,

    host: Option<IdElement>,

    is_suspended: Option<IdElement>,

    moderation_note: Option<IdElement>,

    #[serde(rename = "type")]
    properties_type: Option<Method>,

    policies: Option<IdElement>,

    disable_registration: Option<ImageUrl>,

    pinned_users: Option<MutedInstances>,

    hidden_tags: Option<MutedInstances>,

    blocked_hosts: Option<MutedInstances>,

    sensitive_words: Option<MutedInstances>,

    prohibited_words: Option<MutedInstances>,

    theme_color: Option<Birthday>,

    mascot_image_url: Option<ImageUrl>,

    banner_url: Option<ImageUrl>,

    server_error_image_url: Option<ImageUrl>,

    info_image_url: Option<ImageUrl>,

    not_found_image_url: Option<ImageUrl>,

    icon_url: Option<ImageUrl>,

    app192_icon_url: Option<ImageUrl>,

    app512_icon_url: Option<ImageUrl>,

    background_image_url: Option<ImageUrl>,

    logo_image_url: Option<ImageUrl>,

    name: Option<ImageUrl>,

    short_name: Option<ImageUrl>,

    description: Option<ImageUrl>,

    default_light_theme: Option<ImageUrl>,

    default_dark_theme: Option<ImageUrl>,

    cache_remote_files: Option<IdElement>,

    cache_remote_sensitive_files: Option<IdElement>,

    email_required_for_signup: Option<IdElement>,

    enable_hcaptcha: Option<IdElement>,

    hcaptcha_site_key: Option<ImageUrl>,

    hcaptcha_secret_key: Option<ImageUrl>,

    enable_mcaptcha: Option<IdElement>,

    mcaptcha_site_key: Option<ImageUrl>,

    mcaptcha_instance_url: Option<ImageUrl>,

    mcaptcha_secret_key: Option<ImageUrl>,

    enable_recaptcha: Option<IdElement>,

    recaptcha_site_key: Option<ImageUrl>,

    recaptcha_secret_key: Option<ImageUrl>,

    enable_turnstile: Option<IdElement>,

    turnstile_site_key: Option<ImageUrl>,

    turnstile_secret_key: Option<ImageUrl>,

    sensitive_media_detection: Option<Method>,

    sensitive_media_detection_sensitivity: Option<Method>,

    set_sensitive_flag_automatically: Option<IdElement>,

    enable_sensitive_media_detection_for_videos: Option<IdElement>,

    proxy_account_id: Option<BannerUrlClass>,

    maintainer_name: Option<ImageUrl>,

    maintainer_email: Option<ImageUrl>,

    langs: Option<Users>,

    deepl_auth_key: Option<ImageUrl>,

    deepl_is_pro: Option<IdElement>,

    enable_email: Option<IdElement>,

    email: Option<ImageUrl>,

    smtp_secure: Option<IdElement>,

    smtp_host: Option<ImageUrl>,

    smtp_port: Option<ImageUrl>,

    smtp_user: Option<ImageUrl>,

    smtp_pass: Option<ImageUrl>,

    enable_service_worker: Option<IdElement>,

    sw_public_key: Option<ImageUrl>,

    sw_private_key: Option<ImageUrl>,

    tos_url: Option<ImageUrl>,

    repository_url: Option<ImageUrl>,

    feedback_url: Option<ImageUrl>,

    impressum_url: Option<ImageUrl>,

    privacy_policy_url: Option<ImageUrl>,

    inquiry_url: Option<ImageUrl>,

    use_object_storage: Option<IdElement>,

    object_storage_base_url: Option<ImageUrl>,

    object_storage_bucket: Option<ImageUrl>,

    object_storage_prefix: Option<ImageUrl>,

    object_storage_endpoint: Option<ImageUrl>,

    object_storage_region: Option<ImageUrl>,

    object_storage_port: Option<ImageUrl>,

    object_storage_access_key: Option<ImageUrl>,

    object_storage_secret_key: Option<ImageUrl>,

    #[serde(rename = "objectStorageUseSSL")]
    object_storage_use_ssl: Option<IdElement>,

    object_storage_use_proxy: Option<IdElement>,

    object_storage_set_public_read: Option<IdElement>,

    object_storage_s3_force_path_style: Option<IdElement>,

    enable_ip_logging: Option<IdElement>,

    enable_active_email_validation: Option<IdElement>,

    enable_verifymail_api: Option<IdElement>,

    verifymail_auth_key: Option<ImageUrl>,

    enable_truemail_api: Option<IdElement>,

    truemail_instance: Option<ImageUrl>,

    truemail_auth_key: Option<ImageUrl>,

    enable_charts_for_remote_user: Option<IdElement>,

    enable_charts_for_federated_instances: Option<IdElement>,

    enable_server_machine_stats: Option<IdElement>,

    enable_identicon_generation: Option<IdElement>,

    server_rules: Option<Users>,

    banned_email_domains: Option<Users>,

    preserved_usernames: Option<Users>,

    manifest_json_override: Option<IdElement>,

    enable_fanout_timeline: Option<IdElement>,

    enable_fanout_timeline_db_fallback: Option<IdElement>,

    per_local_user_user_timeline_cache_max: Option<IdElement>,

    per_remote_user_user_timeline_cache_max: Option<IdElement>,

    per_user_home_timeline_cache_max: Option<IdElement>,

    per_user_list_timeline_cache_max: Option<IdElement>,

    enable_reactions_buffering: Option<IdElement>,

    notes_per_one_ad: Option<IdElement>,

    silenced_hosts: Option<MutedInstances>,

    media_silenced_hosts: Option<MutedInstances>,

    summaly_proxy: Option<CategoryClass>,

    url_preview_enabled: Option<IdElement>,

    url_preview_timeout: Option<IdElement>,

    url_preview_maximum_content_length: Option<IdElement>,

    url_preview_require_content_length: Option<IdElement>,

    url_preview_user_agent: Option<ImageUrl>,

    url_preview_summary_proxy_url: Option<ImageUrl>,

    text: Option<IdElement>,

    current_password: Option<IdElement>,

    new_password: Option<Password>,

    token: Option<Token>,

    password: Option<IdElement>,

    token_id: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminEmojiCopy {
    post: AdminEmojiCopyPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminEmojiCopyPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody7,

    responses: HashMap<String, FriskyResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody7 {
    required: bool,

    content: Content17,
}

#[derive(Serialize, Deserialize)]
pub struct Content17 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson17,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson17 {
    schema: Schema10,
}

#[derive(Serialize, Deserialize)]
pub struct Schema10 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties31,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties31 {
    emoji_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct FriskyResponse {
    description: ResponseDescription,

    content: Content18,
}

#[derive(Serialize, Deserialize)]
pub struct Content18 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson18,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson18 {
    schema: Schema11,

    examples: Option<MagentaExamples>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MagentaExamples {
    no_such_emoji: Option<AuthenticationFailedValue>,

    duplicate_name: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema11 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties3>,

    required: Option<Vec<Format>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminEmojiDelete {
    post: AdminEmojiDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminEmojiDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: FluffyRequestBody,

    responses: HashMap<String, MischievousResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct MischievousResponse {
    description: ResponseDescription,

    content: Option<Content19>,
}

#[derive(Serialize, Deserialize)]
pub struct Content19 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson19,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson19 {
    schema: OneOf,

    examples: FriskyExamples,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FriskyExamples {
    no_such_emoji: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminEmojiUpdate {
    post: AdminEmojiUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminEmojiUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody8,

    responses: HashMap<String, BraggadociousResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody8 {
    required: bool,

    content: Content20,
}

#[derive(Serialize, Deserialize)]
pub struct Content20 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson20,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson20 {
    schema: Schema12,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema12 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties29,

    any_of: Vec<SchemaAnyOf>,
}

#[derive(Serialize, Deserialize)]
pub struct BraggadociousResponse {
    description: ResponseDescription,

    content: Option<Content21>,
}

#[derive(Serialize, Deserialize)]
pub struct Content21 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson21,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson21 {
    schema: OneOf,

    examples: MischievousExamples,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MischievousExamples {
    no_such_emoji: Option<AuthenticationFailedValue>,

    no_such_file: Option<AuthenticationFailedValue>,

    same_name_emoji_exists: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminGetIndexStats {
    post: AdminGetIndexStatsPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminGetIndexStatsPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminInviteCreate {
    post: AdminInviteCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminInviteCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody9,

    responses: HashMap<String, Response1>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody9 {
    required: bool,

    content: Content22,
}

#[derive(Serialize, Deserialize)]
pub struct Content22 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson22,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson22 {
    schema: Schema13,
}

#[derive(Serialize, Deserialize)]
pub struct Schema13 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties32 {
    count: Limit,

    expires_at: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct Response1 {
    description: ResponseDescription,

    content: Content23,
}

#[derive(Serialize, Deserialize)]
pub struct Content23 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson23,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson23 {
    schema: StickySchema,

    examples: Option<BraggadociousExamples>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BraggadociousExamples {
    invalid_date_time: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminInviteList {
    post: AdminInviteListPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminInviteListPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Option<Vec<String>>,

    security: Option<Vec<Security>>,

    request_body: Option<RequestBody10>,

    responses: HashMap<String, TentacledResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody10 {
    required: bool,

    content: Content24,
}

#[derive(Serialize, Deserialize)]
pub struct Content24 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson24,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson24 {
    schema: Schema14,
}

#[derive(Serialize, Deserialize)]
pub struct Schema14 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties33,

    required: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties33 {
    limit: Limit,

    offset: Option<NotesPerOneAd>,

    #[serde(rename = "type")]
    properties_type: Option<FluffyType>,

    sort: Option<Sort>,

    since_id: Option<StartsAtClass>,

    until_id: Option<StartsAtClass>,

    host: Option<CategoryClass>,

    blocked: Option<ImageUrl>,

    not_responding: Option<ImageUrl>,

    suspended: Option<ImageUrl>,

    silenced: Option<ImageUrl>,

    federating: Option<ImageUrl>,

    subscribing: Option<ImageUrl>,

    publishing: Option<ImageUrl>,

    following: Option<CaseSensitive>,

    visibility: Option<IdElement>,

    user_id: Option<StartsAtClass>,

    query: Option<IdElement>,

    origin: Option<NoteSearchableScope>,

    detail: Option<CaseSensitive>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyType {
    #[serde(rename = "type")]
    type_type: TentacledType,

    #[serde(rename = "enum")]
    type_enum: Option<Vec<String>>,

    #[serde(rename = "default")]
    type_default: Option<String>,

    pattern: Option<Pattern>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminPromoCreate {
    post: AdminPromoCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminPromoCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody11,

    responses: HashMap<String, Response2>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody11 {
    required: bool,

    content: Content25,
}

#[derive(Serialize, Deserialize)]
pub struct Content25 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson25,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson25 {
    schema: Schema15,
}

#[derive(Serialize, Deserialize)]
pub struct Schema15 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties34,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties34 {
    note_id: StartsAtClass,

    expires_at: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response2 {
    description: ResponseDescription,

    content: Option<Content26>,
}

#[derive(Serialize, Deserialize)]
pub struct Content26 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson26,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson26 {
    schema: OneOf,

    examples: Examples1,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples1 {
    no_such_note: Option<AuthenticationFailedValue>,

    already_promoted: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminRelaysAdd {
    post: AdminRelaysAddPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminRelaysAddPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody12,

    responses: HashMap<String, Response3>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody12 {
    required: bool,

    content: Content27,
}

#[derive(Serialize, Deserialize)]
pub struct Content27 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson27,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson27 {
    schema: Schema16,
}

#[derive(Serialize, Deserialize)]
pub struct Schema16 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties35,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties35 {
    inbox: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response3 {
    description: ResponseDescription,

    content: Content28,
}

#[derive(Serialize, Deserialize)]
pub struct Content28 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson28,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson28 {
    schema: Schema17,

    examples: Option<Examples2>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples2 {
    invalid_url: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema17 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties36>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties36 {
    id: StartsAtClass,

    inbox: StartsAtClass,

    status: NoteSearchableScope,
}

#[derive(Serialize, Deserialize)]
pub struct AdminResolveAbuseUserReport {
    post: AdminResolveAbuseUserReportPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminResolveAbuseUserReportPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody13,

    responses: HashMap<String, Response4>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody13 {
    required: bool,

    content: Content29,
}

#[derive(Serialize, Deserialize)]
pub struct Content29 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson29,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson29 {
    schema: Schema18,
}

#[derive(Serialize, Deserialize)]
pub struct Schema18 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties37,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties37 {
    report_id: StartsAtClass,

    forward: CaseSensitive,
}

#[derive(Serialize, Deserialize)]
pub struct Response4 {
    description: ResponseDescription,

    content: Option<Content30>,
}

#[derive(Serialize, Deserialize)]
pub struct Content30 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson30,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson30 {
    schema: OneOf,

    examples: Examples3,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples3 {
    no_such_abuse_report: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminRolesAssign {
    post: AdminRolesAssignPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminRolesAssignPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody14,

    responses: HashMap<String, Response5>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody14 {
    required: bool,

    content: Content31,
}

#[derive(Serialize, Deserialize)]
pub struct Content31 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson31,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson31 {
    schema: Schema19,
}

#[derive(Serialize, Deserialize)]
pub struct Schema19 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties38,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties38 {
    role_id: StartsAtClass,

    user_id: StartsAtClass,

    expires_at: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct Response5 {
    description: ResponseDescription,

    content: Option<Content32>,
}

#[derive(Serialize, Deserialize)]
pub struct Content32 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson32,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson32 {
    schema: OneOf,

    examples: Examples4,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples4 {
    no_such_role: Option<AuthenticationFailedValue>,

    no_such_user: Option<AuthenticationFailedValue>,

    access_denied: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    not_assigned: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminRolesCreate {
    post: AdminRolesCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminRolesCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody15,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody15 {
    required: bool,

    content: Content33,
}

#[derive(Serialize, Deserialize)]
pub struct Content33 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson33,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson33 {
    schema: Schema20,
}

#[derive(Serialize, Deserialize)]
pub struct Schema20 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties39,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties39 {
    name: IdElement,

    description: IdElement,

    color: ImageUrl,

    icon_url: ImageUrl,

    target: Method,

    cond_formula: IdElement,

    is_public: IdElement,

    is_moderator: IdElement,

    is_administrator: IdElement,

    is_explorable: CaseSensitive,

    as_badge: IdElement,

    can_edit_members_by_moderator: IdElement,

    display_order: IdElement,

    policies: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct AdminRolesDelete {
    post: AdminRolesDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminRolesDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Option<Vec<Security>>,

    request_body: RequestBody16,

    responses: HashMap<String, Response6>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody16 {
    required: bool,

    content: Content34,
}

#[derive(Serialize, Deserialize)]
pub struct Content34 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson34,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson34 {
    schema: Schema21,
}

#[derive(Serialize, Deserialize)]
pub struct Schema21 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties40,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties40 {
    role_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response6 {
    description: ResponseDescription,

    content: Option<Content35>,
}

#[derive(Serialize, Deserialize)]
pub struct Content35 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson35,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson35 {
    schema: Schema22,

    examples: Option<Examples5>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples5 {
    no_such_role: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema22 {
    #[serde(rename = "$ref")]
    schema_ref: Option<String>,

    #[serde(rename = "type")]
    schema_type: Option<UsersType>,

    items: Option<IndigoItems>,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: Option<Properties41>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    items_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties41 {
    id: StartsAtClass,

    created_at: Option<StartsAtClass>,

    user: Blockee,

    expires_at: Option<BannerUrlClass>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminRolesUnassign {
    post: AdminRolesUnassignPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminRolesUnassignPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody17,

    responses: HashMap<String, Response5>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody17 {
    required: bool,

    content: Content36,
}

#[derive(Serialize, Deserialize)]
pub struct Content36 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson36,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson36 {
    schema: Schema23,
}

#[derive(Serialize, Deserialize)]
pub struct Schema23 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties42,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties42 {
    role_id: StartsAtClass,

    user_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct AdminRolesUpdate {
    post: AdminRolesUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminRolesUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody18,

    responses: HashMap<String, Response6>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody18 {
    required: bool,

    content: Content37,
}

#[derive(Serialize, Deserialize)]
pub struct Content37 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson37,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson37 {
    schema: Schema24,
}

#[derive(Serialize, Deserialize)]
pub struct Schema24 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties43,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties43 {
    role_id: StartsAtClass,

    name: IdElement,

    description: IdElement,

    color: ImageUrl,

    icon_url: ImageUrl,

    target: Method,

    cond_formula: IdElement,

    is_public: IdElement,

    is_moderator: IdElement,

    is_administrator: IdElement,

    is_explorable: IdElement,

    as_badge: IdElement,

    can_edit_members_by_moderator: IdElement,

    display_order: IdElement,

    policies: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct RolesUsers {
    post: AdminRolesUsersPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminRolesUsersPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody19,

    responses: HashMap<String, Response6>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody19 {
    required: bool,

    content: Content38,
}

#[derive(Serialize, Deserialize)]
pub struct Content38 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson38,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson38 {
    schema: Schema25,
}

#[derive(Serialize, Deserialize)]
pub struct Schema25 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties22,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminSystemWebhookAte {
    post: AdminSystemWebhookCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminSystemWebhookCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: RequestBody20,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody20 {
    required: bool,

    content: Content39,
}

#[derive(Serialize, Deserialize)]
pub struct Content39 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson39,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson39 {
    schema: Schema26,
}

#[derive(Serialize, Deserialize)]
pub struct Schema26 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties44,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties44 {
    is_active: IdElement,

    name: UrlClass,

    on: On,

    url: UrlClass,

    secret: UrlClass,

    id: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminSystemWebhookShow {
    post: AdminSystemWebhookShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminSystemWebhookShowPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<PurpleTag>,

    security: Vec<Security>,

    request_body: FluffyRequestBody,

    responses: HashMap<String, Response7>,
}

#[derive(Serialize, Deserialize)]
pub struct Response7 {
    description: ResponseDescription,

    content: Content40,
}

#[derive(Serialize, Deserialize)]
pub struct Content40 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson40,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson40 {
    schema: Blockee,

    examples: Option<Examples6>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples6 {
    no_such_system_webhook: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    post: AdminSystemWebhookTestPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminSystemWebhookTestPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody21,

    responses: HashMap<String, Response8>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody21 {
    required: bool,

    content: Content41,
}

#[derive(Serialize, Deserialize)]
pub struct Content41 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson41,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson41 {
    schema: Schema27,
}

#[derive(Serialize, Deserialize)]
pub struct Schema27 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties45,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties45 {
    webhook_id: StartsAtClass,

    #[serde(rename = "type")]
    properties_type: Method,

    #[serde(rename = "override")]
    properties_override: Override,
}

#[derive(Serialize, Deserialize)]
pub struct Override {
    #[serde(rename = "type")]
    override_type: TentacledType,

    properties: OverrideProperties,
}

#[derive(Serialize, Deserialize)]
pub struct OverrideProperties {
    url: IdElement,

    secret: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response8 {
    description: ResponseDescription,

    content: Option<Content42>,
}

#[derive(Serialize, Deserialize)]
pub struct Content42 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson42,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson42 {
    schema: Schema28,

    examples: Option<Examples7>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples7 {
    no_such_webhook: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema28 {
    #[serde(rename = "$ref")]
    schema_ref: Option<String>,

    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties46>,

    required: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties46 {
    id: StartsAtClass,

    user_id: StartsAtClass,

    name: IdElement,

    on: On,

    url: IdElement,

    secret: IdElement,

    active: IdElement,

    latest_sent_at: BannerUrlClass,

    latest_status: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct AnnouncementsShow {
    post: AnnouncementsShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementsShowPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    request_body: RequestBody22,

    responses: HashMap<String, AmbitiousResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody22 {
    required: bool,

    content: Content43,
}

#[derive(Serialize, Deserialize)]
pub struct Content43 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson43,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson43 {
    schema: Schema29,
}

#[derive(Serialize, Deserialize)]
pub struct Schema29 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties47,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties47 {
    announcement_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationId {
    Account,

    Channels,

    Meta,
}

#[derive(Serialize, Deserialize)]
pub struct AntennasAte {
    post: AntennasCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AntennasCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody23,

    responses: HashMap<String, Response9>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody23 {
    required: bool,

    content: Content44,
}

#[derive(Serialize, Deserialize)]
pub struct Content44 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson44,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson44 {
    schema: Schema30,
}

#[derive(Serialize, Deserialize)]
pub struct Schema30 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties48,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties48 {
    name: UrlClass,

    src: Method,

    user_list_id: BannerUrlClass,

    keywords: ExcludeKeywords,

    exclude_keywords: ExcludeKeywords,

    users: Users,

    case_sensitive: IdElement,

    local_only: IdElement,

    exclude_bots: IdElement,

    with_replies: IdElement,

    with_file: IdElement,

    antenna_id: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
pub struct Response9 {
    description: ResponseDescription,

    content: Content45,
}

#[derive(Serialize, Deserialize)]
pub struct Content45 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson45,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson45 {
    schema: Blockee,

    examples: Option<Examples8>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples8 {
    no_such_user_list: Option<AuthenticationFailedValue>,

    too_many_antennas: Option<AuthenticationFailedValue>,

    empty_keyword: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    no_such_antenna: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Antennas {
    post: AntennasDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AntennasDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody24,

    responses: HashMap<String, Response10>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody24 {
    required: bool,

    content: Content46,
}

#[derive(Serialize, Deserialize)]
pub struct Content46 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson46,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson46 {
    schema: Schema31,
}

#[derive(Serialize, Deserialize)]
pub struct Schema31 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties49,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties49 {
    antenna_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response10 {
    description: ResponseDescription,

    content: Option<Content47>,
}

#[derive(Serialize, Deserialize)]
pub struct Content47 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson47,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson47 {
    schema: StickySchema,

    examples: Option<Examples9>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples9 {
    no_such_antenna: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AntennasNotes {
    post: AntennasNotesPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AntennasNotesPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody25,

    responses: HashMap<String, Response10>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody25 {
    required: bool,

    content: Content48,
}

#[derive(Serialize, Deserialize)]
pub struct Content48 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson48,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson48 {
    schema: Schema32,
}

#[derive(Serialize, Deserialize)]
pub struct Schema32 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties50,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties50 {
    antenna_id: Option<StartsAtClass>,

    limit: Limit,

    since_id: StartsAtClass,

    until_id: StartsAtClass,

    since_date: IdElement,

    until_date: IdElement,

    role_id: Option<StartsAtClass>,

    user_id: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
pub struct ApGet {
    post: ApGetPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApGetPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody26,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody26 {
    required: bool,

    content: Content49,
}

#[derive(Serialize, Deserialize)]
pub struct Content49 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson49,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson49 {
    schema: Schema33,
}

#[derive(Serialize, Deserialize)]
pub struct Schema33 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties51,

    required: Vec<Format>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties51 {
    uri: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response11 {
    description: ResponseDescription,

    content: Option<Content50>,
}

#[derive(Serialize, Deserialize)]
pub struct Content50 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson50,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson50 {
    schema: Schema34,

    examples: Option<StickyExamples>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema34 {
    #[serde(rename = "type")]
    schema_type: Option<IndecentType>,

    properties: Option<Properties53>,

    required: Option<Vec<String>>,

    all_of: Option<Vec<OneOf>>,

    items: Option<IndecentItems>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentItems {
    #[serde(rename = "type")]
    items_type: Option<TentacledType>,

    #[serde(rename = "$ref")]
    items_ref: Option<String>,

    properties: Option<Properties52>,

    required: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties52 {
    id: Option<StartsAtClass>,

    flash: Option<Blockee>,

    tag: Option<IdElement>,

    chart: Option<Users>,

    users_count: Option<IdElement>,

    name: Option<IdElement>,

    created_at: Option<StartsAtClass>,

    last_used_at: Option<StartsAtClass>,

    permission: Option<Permission>,

    page: Option<Blockee>,

    scopes: Option<ExcludeKeywords>,

    domain: Option<ImageUrl>,

    users: Option<IdElement>,

    data: Option<ReactionEmojisClass>,

    unlocked_at: Option<IdElement>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties53 {
    params: Option<Params>,

    image: Option<Image>,

    pagination_links: Option<PaginationLinks>,

    link: Option<IdElement>,

    title: Option<IdElement>,

    items: Option<PropertiesItems>,

    feed_url: Option<IdElement>,

    description: Option<IdElement>,

    itunes: Option<Itunes>,

    count: Option<IdElement>,

    remaining: Option<ImageUrl>,

    token: Option<IdElement>,

    pong: Option<IdElement>,

    machine: Option<IdElement>,

    cpu: Option<Cpu>,

    mem: Option<FluffyMem>,

    fs: Option<FluffyFs>,

    state: Option<Method>,

    key: Option<ImageUrl>,

    user_id: Option<IdElement>,

    endpoint: Option<IdElement>,

    send_read_message: Option<IdElement>,

    id: Option<StartsAtClass>,

    required: Option<IdElement>,

    string: Option<IdElement>,

    #[serde(rename = "default")]
    properties_default: Option<IdElement>,

    nullable_default: Option<FeedbackUrl>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyFs {
    #[serde(rename = "type")]
    fs_type: TentacledType,

    properties: Properties54,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties54 {
    total: IdElement,

    used: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyMem {
    #[serde(rename = "type")]
    mem_type: TentacledType,

    properties: Properties55,

    required: Vec<FollowersRequired>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties55 {
    total: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    #[serde(rename = "type")]
    params_type: UsersType,

    items: ParamsItems,
}

#[derive(Serialize, Deserialize)]
pub struct ParamsItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: Properties56,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties56 {
    name: IdElement,

    #[serde(rename = "type")]
    properties_type: IdElement,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndecentType {
    Enum(StickyType),

    EnumArray(Vec<TentacledType>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StickyType {
    Array,

    Boolean,

    Object,
}

#[derive(Serialize, Deserialize)]
pub struct ApShow {
    post: ApShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApShowPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody26,

    responses: HashMap<String, Response12>,
}

#[derive(Serialize, Deserialize)]
pub struct Response12 {
    description: ResponseDescription,

    content: Content51,
}

#[derive(Serialize, Deserialize)]
pub struct Content51 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson51,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson51 {
    schema: Schema35,

    examples: Option<Examples10>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples10 {
    no_such_object: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema35 {
    one_of: Option<Vec<FluffyOneOf>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyOneOf {
    #[serde(rename = "type")]
    one_of_type: TentacledType,

    properties: Properties57,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties57 {
    #[serde(rename = "type")]
    properties_type: Method,

    object: Blockee,
}

#[derive(Serialize, Deserialize)]
pub struct AppCreate {
    post: AppCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppCreatePost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody27,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody27 {
    required: bool,

    content: Content52,
}

#[derive(Serialize, Deserialize)]
pub struct Content52 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson52,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson52 {
    schema: Schema36,
}

#[derive(Serialize, Deserialize)]
pub struct Schema36 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties58,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties58 {
    name: IdElement,

    description: IdElement,

    permission: Permission,

    callback_url: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct AppShow {
    post: AppShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppShowPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody28,

    responses: HashMap<String, Response13>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody28 {
    required: bool,

    content: Content53,
}

#[derive(Serialize, Deserialize)]
pub struct Content53 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson53,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson53 {
    schema: Schema37,
}

#[derive(Serialize, Deserialize)]
pub struct Schema37 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties59,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties59 {
    app_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response13 {
    description: ResponseDescription,

    content: Content54,
}

#[derive(Serialize, Deserialize)]
pub struct Content54 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson54,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson54 {
    schema: Blockee,

    examples: Option<Examples11>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples11 {
    no_such_app: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthAccept {
    post: AuthAcceptPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthAcceptPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody29,

    responses: HashMap<String, Response14>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody29 {
    required: bool,

    content: Content55,
}

#[derive(Serialize, Deserialize)]
pub struct Content55 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson55,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson55 {
    schema: Schema38,
}

#[derive(Serialize, Deserialize)]
pub struct Schema38 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties60,

    required: Vec<String>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties60 {
    token: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response14 {
    description: ResponseDescription,

    content: Option<Content56>,
}

#[derive(Serialize, Deserialize)]
pub struct Content56 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson56,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson56 {
    schema: Schema39,

    examples: Option<Examples12>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples12 {
    no_such_session: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema39 {
    #[serde(rename = "$ref")]
    schema_ref: Option<String>,

    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties61>,

    required: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties61 {
    id: StartsAtClass,

    app: Blockee,

    token: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct AuthSessionGenerate {
    post: AuthSessionGeneratePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthSessionGeneratePost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody30,

    responses: HashMap<String, Response15>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody30 {
    required: bool,

    content: Content57,
}

#[derive(Serialize, Deserialize)]
pub struct Content57 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson57,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson57 {
    schema: Schema40,
}

#[derive(Serialize, Deserialize)]
pub struct Schema40 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties62,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties62 {
    app_secret: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response15 {
    description: ResponseDescription,

    content: Content58,
}

#[derive(Serialize, Deserialize)]
pub struct Content58 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson58,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson58 {
    schema: Schema41,

    examples: Option<Examples11>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema41 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties63>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties63 {
    token: IdElement,

    url: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct AuthSessionShow {
    post: AuthSessionShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthSessionShowPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody29,

    responses: HashMap<String, Response14>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthSessionUserkey {
    post: AuthSessionUserkeyPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthSessionUserkeyPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody31,

    responses: HashMap<String, Response16>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody31 {
    required: bool,

    content: Content59,
}

#[derive(Serialize, Deserialize)]
pub struct Content59 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson59,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson59 {
    schema: Schema42,
}

#[derive(Serialize, Deserialize)]
pub struct Schema42 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties64,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties64 {
    app_secret: IdElement,

    token: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response16 {
    description: ResponseDescription,

    content: Content60,
}

#[derive(Serialize, Deserialize)]
pub struct Content60 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson60,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson60 {
    schema: Schema43,

    examples: Option<Examples13>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples13 {
    no_such_app: Option<AuthenticationFailedValue>,

    no_such_session: Option<AuthenticationFailedValue>,

    pending_session: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema43 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties65>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties65 {
    access_token: IdElement,

    user: Blockee,
}

#[derive(Serialize, Deserialize)]
pub struct BlockingCreateClass {
    post: BlockingCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockingCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: IndecentRequestBody,

    responses: HashMap<String, Response17>,
}

#[derive(Serialize, Deserialize)]
pub struct Response17 {
    description: ResponseDescription,

    content: Content61,
}

#[derive(Serialize, Deserialize)]
pub struct Content61 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson61,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson61 {
    schema: Blockee,

    examples: Option<Examples14>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples14 {
    no_such_user: Option<AuthenticationFailedValue>,

    blockee_is_yourself: Option<AuthenticationFailedValue>,

    already_blocking: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    not_blocking: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct BlockingList {
    post: BlockingListPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockingListPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody1,

    responses: HashMap<String, TentacledResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct BubbleGameRanking {
    get: BubbleGameRankingGet,

    post: BubbleGameRankingGet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BubbleGameRankingGet {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    request_body: RequestBody32,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody32 {
    required: bool,

    content: Content62,
}

#[derive(Serialize, Deserialize)]
pub struct Content62 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson62,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson62 {
    schema: Schema44,
}

#[derive(Serialize, Deserialize)]
pub struct Schema44 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties66,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties66 {
    game_mode: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct BubbleGameRegister {
    post: BubbleGameRegisterPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BubbleGameRegisterPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody33,

    responses: HashMap<String, Response18>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody33 {
    required: bool,

    content: Content63,
}

#[derive(Serialize, Deserialize)]
pub struct Content63 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson63,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson63 {
    schema: Schema45,
}

#[derive(Serialize, Deserialize)]
pub struct Schema45 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties67,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties67 {
    score: Score,

    seed: UrlClass,

    logs: ExcludeKeywords,

    game_mode: IdElement,

    game_version: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Score {
    #[serde(rename = "type")]
    score_type: TentacledType,

    minimum: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Response18 {
    description: ResponseDescription,

    content: Option<Content64>,
}

#[derive(Serialize, Deserialize)]
pub struct Content64 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson64,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson64 {
    schema: OneOf,

    examples: Examples15,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples15 {
    invalid_seed: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelsCreate {
    post: ChannelsCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody34,

    responses: HashMap<String, Response19>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody34 {
    required: bool,

    content: Content65,
}

#[derive(Serialize, Deserialize)]
pub struct Content65 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson65,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson65 {
    schema: Schema46,
}

#[derive(Serialize, Deserialize)]
pub struct Schema46 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties68,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties68 {
    name: UrlClass,

    description: DescriptionClass,

    banner_id: BannerUrlClass,

    color: UrlClass,

    is_sensitive: ImageUrl,

    allow_renote_to_external: ImageUrl,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionClass {
    #[serde(rename = "type")]
    description_type: Vec<TentacledType>,

    min_length: i64,

    max_length: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Response19 {
    description: ResponseDescription,

    content: Content66,
}

#[derive(Serialize, Deserialize)]
pub struct Content66 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson66,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson66 {
    schema: Blockee,

    examples: Option<Examples16>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples16 {
    no_such_file: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Channels {
    post: ChannelsFavoritePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsFavoritePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody35,

    responses: HashMap<String, Response20>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody35 {
    required: bool,

    content: Content67,
}

#[derive(Serialize, Deserialize)]
pub struct Content67 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson67,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson67 {
    schema: Schema47,
}

#[derive(Serialize, Deserialize)]
pub struct Schema47 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties69,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties69 {
    channel_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response20 {
    description: ResponseDescription,

    content: Option<Content68>,
}

#[derive(Serialize, Deserialize)]
pub struct Content68 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson68,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson68 {
    schema: StickySchema,

    examples: Option<Examples17>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples17 {
    no_such_channel: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelsFeatured {
    post: ChannelsFeaturedPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsFeaturedPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    responses: HashMap<String, TentacledResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelsShow {
    post: ChannelsShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsShowPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    request_body: RequestBody35,

    responses: HashMap<String, Response20>,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelsTimeline {
    post: ChannelsTimelinePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsTimelinePost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody36,

    responses: HashMap<String, Response20>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody36 {
    required: bool,

    content: Content69,
}

#[derive(Serialize, Deserialize)]
pub struct Content69 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson69,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson69 {
    schema: Schema48,
}

#[derive(Serialize, Deserialize)]
pub struct Schema48 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties70,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties70 {
    channel_id: StartsAtClass,

    limit: Limit,

    since_id: StartsAtClass,

    until_id: StartsAtClass,

    since_date: IdElement,

    until_date: IdElement,

    allow_partial: CaseSensitive,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelsUpdate {
    post: ChannelsUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody37,

    responses: HashMap<String, Response21>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody37 {
    required: bool,

    content: Content70,
}

#[derive(Serialize, Deserialize)]
pub struct Content70 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson70,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson70 {
    schema: Schema49,
}

#[derive(Serialize, Deserialize)]
pub struct Schema49 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties71,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties71 {
    channel_id: StartsAtClass,

    name: UrlClass,

    description: DescriptionClass,

    banner_id: BannerUrlClass,

    is_archived: ImageUrl,

    pinned_note_ids: PinnedNoteIds,

    color: UrlClass,

    is_sensitive: ImageUrl,

    allow_renote_to_external: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct Response21 {
    description: ResponseDescription,

    content: Content71,
}

#[derive(Serialize, Deserialize)]
pub struct Content71 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson71,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson71 {
    schema: Blockee,

    examples: Option<Examples18>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples18 {
    no_such_channel: Option<AuthenticationFailedValue>,

    access_denied: Option<AuthenticationFailedValue>,

    no_such_file: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Charts {
    get: ChartsActiveUsersGet,

    post: ChartsActiveUsersGet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartsActiveUsersGet {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<GetTag>,

    request_body: RequestBody38,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody38 {
    required: bool,

    content: Content72,
}

#[derive(Serialize, Deserialize)]
pub struct Content72 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson72,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson72 {
    schema: Schema50,
}

#[derive(Serialize, Deserialize)]
pub struct Schema50 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties72,

    required: Vec<PurpleRequired>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties72 {
    span: Method,

    limit: Limit,

    offset: FeedbackUrl,

    host: Option<IdElement>,

    user_id: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTag {
    Charts,
}

#[derive(Serialize, Deserialize)]
pub struct ClipsAddNote {
    post: ClipsAddNotePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipsAddNotePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody39,

    responses: HashMap<String, Response22>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody39 {
    required: bool,

    content: Content73,
}

#[derive(Serialize, Deserialize)]
pub struct Content73 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson73,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson73 {
    schema: Schema51,
}

#[derive(Serialize, Deserialize)]
pub struct Schema51 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties73,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties73 {
    clip_id: StartsAtClass,

    note_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response22 {
    description: ResponseDescription,

    content: Option<Content74>,
}

#[derive(Serialize, Deserialize)]
pub struct Content74 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson74,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson74 {
    schema: OneOf,

    examples: Examples19,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples19 {
    no_such_clip: Option<AuthenticationFailedValue>,

    no_such_note: Option<AuthenticationFailedValue>,

    already_clipped: Option<AuthenticationFailedValue>,

    too_many_clip_notes: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct ClipsCreate {
    post: ClipsCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipsCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody40,

    responses: HashMap<String, Response23>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody40 {
    required: bool,

    content: Content75,
}

#[derive(Serialize, Deserialize)]
pub struct Content75 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson75,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson75 {
    schema: Schema52,
}

#[derive(Serialize, Deserialize)]
pub struct Schema52 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties74,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties74 {
    name: UrlClass,

    is_public: CaseSensitive,

    description: DescriptionClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response23 {
    description: ResponseDescription,

    content: Content76,
}

#[derive(Serialize, Deserialize)]
pub struct Content76 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson76,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson76 {
    schema: Blockee,

    examples: Option<Examples20>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples20 {
    too_many_clips: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct ClipsDelete {
    post: ClipsDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipsDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody41,

    responses: HashMap<String, Response24>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody41 {
    required: bool,

    content: Content77,
}

#[derive(Serialize, Deserialize)]
pub struct Content77 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson77,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson77 {
    schema: Schema53,
}

#[derive(Serialize, Deserialize)]
pub struct Schema53 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties75,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties75 {
    clip_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response24 {
    description: ResponseDescription,

    content: Option<Content78>,
}

#[derive(Serialize, Deserialize)]
pub struct Content78 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson78,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson78 {
    schema: StickySchema,

    examples: Option<Examples21>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples21 {
    no_such_clip: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct ClipsFavorite {
    post: ClipsFavoritePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipsFavoritePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody41,

    responses: HashMap<String, Response25>,
}

#[derive(Serialize, Deserialize)]
pub struct Response25 {
    description: ResponseDescription,

    content: Option<Content79>,
}

#[derive(Serialize, Deserialize)]
pub struct Content79 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson79,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson79 {
    schema: OneOf,

    examples: Examples22,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples22 {
    no_such_clip: Option<AuthenticationFailedValue>,

    already_favorited: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct ClipsNotes {
    post: ClipsNotesPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipsNotesPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    request_body: RequestBody19,

    responses: HashMap<String, Response24>,
}

#[derive(Serialize, Deserialize)]
pub struct ClipsRemoveNote {
    post: ClipsRemoveNotePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipsRemoveNotePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody39,

    responses: HashMap<String, Response26>,
}

#[derive(Serialize, Deserialize)]
pub struct Response26 {
    description: ResponseDescription,

    content: Option<Content80>,
}

#[derive(Serialize, Deserialize)]
pub struct Content80 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson80,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson80 {
    schema: OneOf,

    examples: Examples23,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples23 {
    no_such_clip: Option<AuthenticationFailedValue>,

    no_such_note: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct ClipsShow {
    post: ClipsShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipsShowPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody41,

    responses: HashMap<String, Response24>,
}

#[derive(Serialize, Deserialize)]
pub struct ClipsUnfavorite {
    post: ClipsUnfavoritePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipsUnfavoritePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody41,

    responses: HashMap<String, Response27>,
}

#[derive(Serialize, Deserialize)]
pub struct Response27 {
    description: ResponseDescription,

    content: Option<Content81>,
}

#[derive(Serialize, Deserialize)]
pub struct Content81 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson81,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson81 {
    schema: OneOf,

    examples: Examples24,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples24 {
    no_such_clip: Option<AuthenticationFailedValue>,

    not_favorited: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct ClipsUpdate {
    post: ClipsUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipsUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody42,

    responses: HashMap<String, Response24>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody42 {
    required: bool,

    content: Content82,
}

#[derive(Serialize, Deserialize)]
pub struct Content82 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson82,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson82 {
    schema: Schema54,
}

#[derive(Serialize, Deserialize)]
pub struct Schema54 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties76,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties76 {
    clip_id: StartsAtClass,

    name: UrlClass,

    is_public: IdElement,

    description: DescriptionClass,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFilesAttachedNotes {
    post: DriveFilesAttachedNotesPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFilesAttachedNotesPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody19,

    responses: HashMap<String, Response28>,
}

#[derive(Serialize, Deserialize)]
pub struct Response28 {
    description: ResponseDescription,

    content: Content83,
}

#[derive(Serialize, Deserialize)]
pub struct Content83 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson83,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson83 {
    schema: StickySchema,

    examples: Option<AmbitiousExamples>,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFilesCheckExistence {
    post: DriveFilesCheckExistencePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFilesCheckExistencePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody43,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody43 {
    required: bool,

    content: Content84,
}

#[derive(Serialize, Deserialize)]
pub struct Content84 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson84,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson84 {
    schema: Schema55,
}

#[derive(Serialize, Deserialize)]
pub struct Schema55 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties77,

    required: Vec<Format>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties77 {
    md5: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFilesCreate {
    post: DriveFilesCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFilesCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody44,

    responses: HashMap<String, Response29>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody44 {
    required: bool,

    content: Content85,
}

#[derive(Serialize, Deserialize)]
pub struct Content85 {
    #[serde(rename = "multipart/form-data")]
    multipart_form_data: MultipartFormData,
}

#[derive(Serialize, Deserialize)]
pub struct MultipartFormData {
    schema: MultipartFormDataSchema,
}

#[derive(Serialize, Deserialize)]
pub struct MultipartFormDataSchema {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties78,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties78 {
    folder_id: PurpleId,

    name: FeedbackUrl,

    comment: PurpleComment,

    is_sensitive: CaseSensitive,

    force: CaseSensitive,

    file: FileClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleComment {
    #[serde(rename = "type")]
    comment_type: Vec<TentacledType>,

    max_length: i64,

    #[serde(rename = "default")]
    comment_default: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Response29 {
    description: ResponseDescription,

    content: Content86,
}

#[derive(Serialize, Deserialize)]
pub struct Content86 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson85,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson85 {
    schema: Blockee,

    examples: Option<Examples25>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples25 {
    invalid_file_name: Option<AuthenticationFailedValue>,

    inappropriate: Option<AuthenticationFailedValue>,

    no_free_space: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFilesDelete {
    post: DriveFilesDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFilesDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody45,

    responses: HashMap<String, Response30>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody45 {
    required: bool,

    content: Content87,
}

#[derive(Serialize, Deserialize)]
pub struct Content87 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson86,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson86 {
    schema: Schema56,
}

#[derive(Serialize, Deserialize)]
pub struct Schema56 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties79,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties79 {
    file_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response30 {
    description: ResponseDescription,

    content: Option<Content71>,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFilesShow {
    post: DriveFilesShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFilesShowPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody4,

    responses: HashMap<String, Response30>,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFilesUpdate {
    post: DriveFilesUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFilesUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody46,

    responses: HashMap<String, Response31>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody46 {
    required: bool,

    content: Content88,
}

#[derive(Serialize, Deserialize)]
pub struct Content88 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson87,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson87 {
    schema: Schema57,
}

#[derive(Serialize, Deserialize)]
pub struct Schema57 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties80,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties80 {
    file_id: StartsAtClass,

    folder_id: BannerUrlClass,

    name: IdElement,

    is_sensitive: IdElement,

    comment: SecretClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretClass {
    #[serde(rename = "type")]
    comment_type: Vec<TentacledType>,

    max_length: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Response31 {
    description: ResponseDescription,

    content: Content89,
}

#[derive(Serialize, Deserialize)]
pub struct Content89 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson88,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson88 {
    schema: Blockee,

    examples: Option<Examples26>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples26 {
    invalid_file_name: Option<AuthenticationFailedValue>,

    no_such_file: Option<AuthenticationFailedValue>,

    access_denied: Option<AuthenticationFailedValue>,

    no_such_folder: Option<AuthenticationFailedValue>,

    restricted_by_role: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFilesUploadFromUrl {
    post: DriveFilesUploadFromUrlPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFilesUploadFromUrlPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Option<Vec<String>>,

    security: Option<Vec<Security>>,

    request_body: Option<RequestBody47>,

    responses: HashMap<String, Response32>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody47 {
    required: bool,

    content: Content90,
}

#[derive(Serialize, Deserialize)]
pub struct Content90 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson89,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson89 {
    schema: Schema58,
}

#[derive(Serialize, Deserialize)]
pub struct Schema58 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties81,

    required: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties81 {
    url: Option<IdElement>,

    folder_id: Option<PurpleId>,

    is_sensitive: Option<CaseSensitive>,

    comment: Option<PurpleComment>,

    marker: Option<FeedbackUrl>,

    force: Option<CaseSensitive>,

    notify: Option<Method>,

    with_replies: Option<IdElement>,

    exclude_muting: Option<CaseSensitive>,

    exclude_inactive: Option<CaseSensitive>,

    body: Option<IdElement>,

    header: Option<ImageUrl>,

    icon: Option<ImageUrl>,

    username: Option<IdElement>,

    email: Option<IdElement>,
}

#[derive(Serialize, Deserialize)]
pub struct Response32 {
    description: ResponseDescription,

    content: Option<Content91>,
}

#[derive(Serialize, Deserialize)]
pub struct Content91 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson90,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson90 {
    schema: OneOf,

    examples: StickyExamples,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFoldersCreate {
    post: DriveFoldersCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFoldersCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody48,

    responses: HashMap<String, Response33>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody48 {
    required: bool,

    content: Content92,
}

#[derive(Serialize, Deserialize)]
pub struct Content92 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson91,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson91 {
    schema: Schema59,
}

#[derive(Serialize, Deserialize)]
pub struct Schema59 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties82,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties82 {
    name: FluffyName,

    parent_id: BannerUrlClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyName {
    #[serde(rename = "type")]
    name_type: TentacledType,

    #[serde(rename = "default")]
    name_default: String,

    max_length: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Response33 {
    description: ResponseDescription,

    content: Content93,
}

#[derive(Serialize, Deserialize)]
pub struct Content93 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson92,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson92 {
    schema: Blockee,

    examples: Option<Examples27>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples27 {
    no_such_folder: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFoldersDelete {
    post: DriveFoldersDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFoldersDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody49,

    responses: HashMap<String, Response34>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody49 {
    required: bool,

    content: Content94,
}

#[derive(Serialize, Deserialize)]
pub struct Content94 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson93,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson93 {
    schema: Schema60,
}

#[derive(Serialize, Deserialize)]
pub struct Schema60 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties83,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties83 {
    folder_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response34 {
    description: ResponseDescription,

    content: Option<Content95>,
}

#[derive(Serialize, Deserialize)]
pub struct Content95 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson94,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson94 {
    schema: OneOf,

    examples: Examples28,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples28 {
    no_such_folder: Option<AuthenticationFailedValue>,

    has_child_files_or_folders: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFoldersShow {
    post: DriveFoldersShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFoldersShowPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody49,

    responses: HashMap<String, Response33>,
}

#[derive(Serialize, Deserialize)]
pub struct DriveFoldersUpdate {
    post: DriveFoldersUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFoldersUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody50,

    responses: HashMap<String, Response35>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody50 {
    required: bool,

    content: Content96,
}

#[derive(Serialize, Deserialize)]
pub struct Content96 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson95,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson95 {
    schema: Schema61,
}

#[derive(Serialize, Deserialize)]
pub struct Schema61 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties84,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties84 {
    folder_id: StartsAtClass,

    name: TentacledName,

    parent_id: BannerUrlClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledName {
    #[serde(rename = "type")]
    name_type: TentacledType,

    max_length: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Response35 {
    description: ResponseDescription,

    content: Content97,
}

#[derive(Serialize, Deserialize)]
pub struct Content97 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson96,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson96 {
    schema: Blockee,

    examples: Option<Examples29>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples29 {
    no_such_folder: Option<AuthenticationFailedValue>,

    no_such_parent_folder: Option<AuthenticationFailedValue>,

    recursive_nesting: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct EmailAddressAvailable {
    post: EmailAddressAvailablePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailAddressAvailablePost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody51,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody51 {
    required: bool,

    content: Content98,
}

#[derive(Serialize, Deserialize)]
pub struct Content98 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson97,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson97 {
    schema: Schema62,
}

#[derive(Serialize, Deserialize)]
pub struct Schema62 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties85,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties85 {
    email_address: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Emoji {
    get: EmojiGet,

    post: EmojiGet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiGet {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    request_body: RequestBody52,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody52 {
    required: bool,

    content: Content99,
}

#[derive(Serialize, Deserialize)]
pub struct Content99 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson98,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson98 {
    schema: Schema63,
}

#[derive(Serialize, Deserialize)]
pub struct Schema63 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties86,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties86 {
    name: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Emojis {
    get: EmojisGet,

    post: EmojisGet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojisGet {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct Endpoint {
    post: EndpointPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    request_body: RequestBody53,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody53 {
    required: bool,

    content: Content100,
}

#[derive(Serialize, Deserialize)]
pub struct Content100 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson99,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson99 {
    schema: Schema64,
}

#[derive(Serialize, Deserialize)]
pub struct Schema64 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties87,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties87 {
    endpoint: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Endpoints {
    post: EmojisGet,
}

#[derive(Serialize, Deserialize)]
pub struct Federation {
    post: FederationFollowersPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FederationFollowersPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody19,

    responses: HashMap<String, TentacledResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct FederationInstances {
    get: AdminAbuseReportNotificationRecipientListPost,

    post: AdminInviteListPost,
}

#[derive(Serialize, Deserialize)]
pub struct FederationShowInstance {
    post: FederationShowInstancePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FederationShowInstancePost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody54,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody54 {
    required: bool,

    content: Content101,
}

#[derive(Serialize, Deserialize)]
pub struct Content101 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson100,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson100 {
    schema: Schema65,
}

#[derive(Serialize, Deserialize)]
pub struct Schema65 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties88,

    required: Vec<PurpleRequired>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties88 {
    host: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct FederationStats {
    get: FederationStatsGet,

    post: FederationStatsGet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FederationStatsGet {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody55,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody55 {
    required: bool,

    content: Content102,
}

#[derive(Serialize, Deserialize)]
pub struct Content102 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson101,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson101 {
    schema: Schema66,
}

#[derive(Serialize, Deserialize)]
pub struct Schema66 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties89,
}

#[derive(Serialize, Deserialize)]
pub struct Properties89 {
    limit: Limit,
}

#[derive(Serialize, Deserialize)]
pub struct FetchExternalResources {
    post: FetchExternalResourcesPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchExternalResourcesPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody56,

    responses: HashMap<String, Response36>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody56 {
    required: bool,

    content: Content103,
}

#[derive(Serialize, Deserialize)]
pub struct Content103 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson102,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson102 {
    schema: Schema67,
}

#[derive(Serialize, Deserialize)]
pub struct Schema67 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties90,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties90 {
    url: IdElement,

    hash: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response36 {
    description: ResponseDescription,

    content: Content104,
}

#[derive(Serialize, Deserialize)]
pub struct Content104 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson103,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson103 {
    schema: Schema68,

    examples: Option<Examples30>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples30 {
    ext_resource_returned_invalid_schema: Option<AuthenticationFailedValue>,

    ext_resource_hash_didnt_match: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema68 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties91>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties91 {
    #[serde(rename = "type")]
    properties_type: IdElement,

    data: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct FetchRss {
    get: FetchRssGet,

    post: FetchRssGet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchRssGet {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    request_body: RequestBody57,

    responses: HashMap<String, Response37>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody57 {
    required: bool,

    content: Content105,
}

#[derive(Serialize, Deserialize)]
pub struct Content105 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson104,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson104 {
    schema: Schema69,
}

#[derive(Serialize, Deserialize)]
pub struct Schema69 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties92,

    required: Vec<Format>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties92 {
    url: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response37 {
    description: ResponseDescription,

    content: Content106,
}

#[derive(Serialize, Deserialize)]
pub struct Content106 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson105,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson105 {
    schema: Schema70,

    examples: Option<FluffyExamples>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema70 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties93>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties93 {
    image: Image,

    pagination_links: PaginationLinks,

    link: IdElement,

    title: IdElement,

    items: PropertiesItems,

    feed_url: IdElement,

    description: IdElement,

    itunes: Itunes,
}

#[derive(Serialize, Deserialize)]
pub struct FlashCreate {
    post: FlashCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody58,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody58 {
    required: bool,

    content: Content107,
}

#[derive(Serialize, Deserialize)]
pub struct Content107 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson106,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson106 {
    schema: Schema71,
}

#[derive(Serialize, Deserialize)]
pub struct Schema71 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties94,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties94 {
    title: IdElement,

    summary: IdElement,

    script: IdElement,

    permissions: Users,

    visibility: NoteSearchableScope,
}

#[derive(Serialize, Deserialize)]
pub struct FlashDelete {
    post: FlashDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody59,

    responses: HashMap<String, Response38>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody59 {
    required: bool,

    content: Content108,
}

#[derive(Serialize, Deserialize)]
pub struct Content108 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson107,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson107 {
    schema: Schema72,
}

#[derive(Serialize, Deserialize)]
pub struct Schema72 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties95,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties95 {
    flash_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response38 {
    description: ResponseDescription,

    content: Option<Content109>,
}

#[derive(Serialize, Deserialize)]
pub struct Content109 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson108,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson108 {
    schema: OneOf,

    examples: Examples31,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples31 {
    no_such_flash: Option<AuthenticationFailedValue>,

    access_denied: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct FlashLike {
    post: FlashLikePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashLikePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody59,

    responses: HashMap<String, Response39>,
}

#[derive(Serialize, Deserialize)]
pub struct Response39 {
    description: ResponseDescription,

    content: Option<Content110>,
}

#[derive(Serialize, Deserialize)]
pub struct Content110 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson109,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson109 {
    schema: OneOf,

    examples: Examples32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples32 {
    no_such_flash: Option<AuthenticationFailedValue>,

    your_flash: Option<AuthenticationFailedValue>,

    already_liked: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Likes {
    post: FlashMyLikesPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashMyLikesPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody1,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct FlashShow {
    post: FlashShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashShowPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody59,

    responses: HashMap<String, Response40>,
}

#[derive(Serialize, Deserialize)]
pub struct Response40 {
    description: ResponseDescription,

    content: Content111,
}

#[derive(Serialize, Deserialize)]
pub struct Content111 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson110,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson110 {
    schema: Blockee,

    examples: Option<Examples33>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples33 {
    no_such_flash: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct FlashUnlike {
    post: FlashUnlikePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashUnlikePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody59,

    responses: HashMap<String, Response41>,
}

#[derive(Serialize, Deserialize)]
pub struct Response41 {
    description: ResponseDescription,

    content: Option<Content112>,
}

#[derive(Serialize, Deserialize)]
pub struct Content112 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson111,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson111 {
    schema: OneOf,

    examples: Examples34,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples34 {
    no_such_flash: Option<AuthenticationFailedValue>,

    not_liked: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct FlashUpdate {
    post: FlashUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody60,

    responses: HashMap<String, Response38>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody60 {
    required: bool,

    content: Content113,
}

#[derive(Serialize, Deserialize)]
pub struct Content113 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson112,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson112 {
    schema: Schema73,
}

#[derive(Serialize, Deserialize)]
pub struct Schema73 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties96,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties96 {
    flash_id: StartsAtClass,

    title: IdElement,

    summary: IdElement,

    script: IdElement,

    permissions: Users,

    visibility: Method,
}

#[derive(Serialize, Deserialize)]
pub struct FollowingCreate {
    post: FollowingCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowingCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody61,

    responses: HashMap<String, Response42>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody61 {
    required: bool,

    content: Content114,
}

#[derive(Serialize, Deserialize)]
pub struct Content114 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson113,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson113 {
    schema: Schema74,
}

#[derive(Serialize, Deserialize)]
pub struct Schema74 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties97,

    required: Vec<PurpleRequired>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties97 {
    user_id: StartsAtClass,

    with_replies: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response42 {
    description: ResponseDescription,

    content: Content115,
}

#[derive(Serialize, Deserialize)]
pub struct Content115 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson114,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson114 {
    schema: Blockee,

    examples: Option<HashMap<String, AuthenticationFailedValue>>,
}

#[derive(Serialize, Deserialize)]
pub struct FollowingDeleteClass {
    post: FollowingDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowingDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: IndecentRequestBody,

    responses: HashMap<String, Response43>,
}

#[derive(Serialize, Deserialize)]
pub struct Response43 {
    description: ResponseDescription,

    content: Content116,
}

#[derive(Serialize, Deserialize)]
pub struct Content116 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson115,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson115 {
    schema: Blockee,

    examples: Option<Examples35>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples35 {
    no_such_user: Option<AuthenticationFailedValue>,

    followee_is_yourself: Option<AuthenticationFailedValue>,

    not_following: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    follower_is_yourself: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct FollowingRequestsAccept {
    post: FollowingRequestsAcceptPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowingRequestsAcceptPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: IndecentRequestBody,

    responses: HashMap<String, Response44>,
}

#[derive(Serialize, Deserialize)]
pub struct Response44 {
    description: ResponseDescription,

    content: Option<Content117>,
}

#[derive(Serialize, Deserialize)]
pub struct Content117 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson116,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson116 {
    schema: OneOf,

    examples: Examples36,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples36 {
    no_such_user: Option<AuthenticationFailedValue>,

    no_follow_request: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct FollowingRequestsCancel {
    post: FollowingRequestsCancelPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowingRequestsCancelPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: IndecentRequestBody,

    responses: HashMap<String, Response45>,
}

#[derive(Serialize, Deserialize)]
pub struct Response45 {
    description: ResponseDescription,

    content: Content118,
}

#[derive(Serialize, Deserialize)]
pub struct Content118 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson117,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson117 {
    schema: Blockee,

    examples: Option<Examples37>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples37 {
    no_such_user: Option<AuthenticationFailedValue>,

    follow_request_not_found: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct FollowingRequestsReject {
    post: FollowingRequestsRejectPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowingRequestsRejectPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: IndecentRequestBody,

    responses: HashMap<String, Response46>,
}

#[derive(Serialize, Deserialize)]
pub struct Response46 {
    description: ResponseDescription,

    content: Option<Content119>,
}

#[derive(Serialize, Deserialize)]
pub struct Content119 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson118,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson118 {
    schema: Schema75,

    examples: Option<Examples38>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples38 {
    no_such_user: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema75 {
    #[serde(rename = "$ref")]
    schema_ref: Option<String>,

    #[serde(rename = "type")]
    schema_type: Option<UsersType>,

    items: Option<HilariousItems>,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: Properties98,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties98 {
    user: Blockee,

    weight: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct FollowingUpdate {
    post: FollowingUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowingUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody62,

    responses: HashMap<String, Response43>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody62 {
    required: bool,

    content: Content120,
}

#[derive(Serialize, Deserialize)]
pub struct Content120 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson119,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson119 {
    schema: Schema76,
}

#[derive(Serialize, Deserialize)]
pub struct Schema76 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties99,

    required: Vec<PurpleRequired>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties99 {
    user_id: StartsAtClass,

    notify: Method,

    with_replies: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct GalleryPostsAte {
    post: GalleryPostsCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GalleryPostsCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody63,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody63 {
    required: bool,

    content: Content121,
}

#[derive(Serialize, Deserialize)]
pub struct Content121 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson120,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson120 {
    schema: Schema77,
}

#[derive(Serialize, Deserialize)]
pub struct Schema77 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties100,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties100 {
    title: Password,

    description: ImageUrl,

    file_ids: Ids,

    is_sensitive: CaseSensitive,

    post_id: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ids {
    #[serde(rename = "type")]
    ids_type: UsersType,

    unique_items: bool,

    min_items: i64,

    max_items: i64,

    items: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct GalleryPostsDelete {
    post: GalleryPostsDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GalleryPostsDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody64,

    responses: HashMap<String, Response47>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody64 {
    required: bool,

    content: Content122,
}

#[derive(Serialize, Deserialize)]
pub struct Content122 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson121,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson121 {
    schema: Schema78,
}

#[derive(Serialize, Deserialize)]
pub struct Schema78 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties101,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties101 {
    post_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response47 {
    description: ResponseDescription,

    content: Option<Content123>,
}

#[derive(Serialize, Deserialize)]
pub struct Content123 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson122,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson122 {
    schema: OneOf,

    examples: Examples39,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples39 {
    no_such_post: Option<AuthenticationFailedValue>,

    access_denied: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct GalleryPostsLike {
    post: GalleryPostsLikePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GalleryPostsLikePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody64,

    responses: HashMap<String, Response48>,
}

#[derive(Serialize, Deserialize)]
pub struct Response48 {
    description: ResponseDescription,

    content: Option<Content124>,
}

#[derive(Serialize, Deserialize)]
pub struct Content124 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson123,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson123 {
    schema: OneOf,

    examples: Examples40,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples40 {
    no_such_post: Option<AuthenticationFailedValue>,

    your_post: Option<AuthenticationFailedValue>,

    already_liked: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct GalleryPostsShow {
    post: GalleryPostsShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GalleryPostsShowPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody64,

    responses: HashMap<String, Response49>,
}

#[derive(Serialize, Deserialize)]
pub struct Response49 {
    description: ResponseDescription,

    content: Content125,
}

#[derive(Serialize, Deserialize)]
pub struct Content125 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson124,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson124 {
    schema: Blockee,

    examples: Option<Examples41>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples41 {
    no_such_post: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct GalleryPostsUnlike {
    post: GalleryPostsUnlikePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GalleryPostsUnlikePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody64,

    responses: HashMap<String, Response50>,
}

#[derive(Serialize, Deserialize)]
pub struct Response50 {
    description: ResponseDescription,

    content: Option<Content126>,
}

#[derive(Serialize, Deserialize)]
pub struct Content126 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson125,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson125 {
    schema: OneOf,

    examples: Examples42,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples42 {
    no_such_post: Option<AuthenticationFailedValue>,

    not_liked: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct GetOnlineUsersCount {
    get: GetOnlineUsersCountGet,

    post: GetOnlineUsersCountGet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOnlineUsersCountGet {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    responses: HashMap<String, Response51>,
}

#[derive(Serialize, Deserialize)]
pub struct Response51 {
    description: ResponseDescription,

    content: Content127,
}

#[derive(Serialize, Deserialize)]
pub struct Content127 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson126,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson126 {
    schema: Schema79,

    examples: Option<FluffyExamples>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema79 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties102>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties102 {
    count: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct HashtagsSearch {
    post: HashtagsSearchPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashtagsSearchPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody65,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody65 {
    required: bool,

    content: Content128,
}

#[derive(Serialize, Deserialize)]
pub struct Content128 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson127,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson127 {
    schema: Schema80,
}

#[derive(Serialize, Deserialize)]
pub struct Schema80 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties103,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties103 {
    limit: Limit,

    query: IdElement,

    offset: NotesPerOneAd,
}

#[derive(Serialize, Deserialize)]
pub struct HashtagsShow {
    post: HashtagsShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashtagsShowPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody66,

    responses: HashMap<String, Response52>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody66 {
    required: bool,

    content: Content129,
}

#[derive(Serialize, Deserialize)]
pub struct Content129 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson128,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson128 {
    schema: Schema81,
}

#[derive(Serialize, Deserialize)]
pub struct Schema81 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties104,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties104 {
    tag: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response52 {
    description: ResponseDescription,

    content: Content130,
}

#[derive(Serialize, Deserialize)]
pub struct Content130 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson129,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson129 {
    schema: Blockee,

    examples: Option<Examples43>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples43 {
    no_such_hashtag: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct HashtagsTrend {
    get: HashtagsTrendGet,

    post: HashtagsTrendGet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashtagsTrendGet {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    responses: HashMap<String, Response53>,
}

#[derive(Serialize, Deserialize)]
pub struct Response53 {
    description: ResponseDescription,

    content: Content131,
}

#[derive(Serialize, Deserialize)]
pub struct Content131 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson130,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson130 {
    schema: Schema82,

    examples: Option<FluffyExamples>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema82 {
    #[serde(rename = "type")]
    schema_type: Option<UsersType>,

    items: Option<AmbitiousItems>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AmbitiousItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: Properties105,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties105 {
    tag: IdElement,

    chart: Users,

    users_count: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct I {
    post: IPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    responses: HashMap<String, Response54>,
}

#[derive(Serialize, Deserialize)]
pub struct Response54 {
    description: ResponseDescription,

    content: Content132,
}

#[derive(Serialize, Deserialize)]
pub struct Content132 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson131,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson131 {
    schema: Blockee,

    examples: Option<Examples44>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples44 {
    user_is_deleted: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct I2FaDone {
    post: I2FaDonePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I2FaDonePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody29,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct I2FaKeyDone {
    post: I2FaKeyDonePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I2FaKeyDonePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody67,

    responses: HashMap<String, Response55>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody67 {
    required: bool,

    content: Content133,
}

#[derive(Serialize, Deserialize)]
pub struct Content133 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson132,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson132 {
    schema: Schema83,
}

#[derive(Serialize, Deserialize)]
pub struct Schema83 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties106,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties106 {
    password: IdElement,

    token: ImageUrl,

    name: UrlClass,

    credential: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response55 {
    description: ResponseDescription,

    content: Content134,
}

#[derive(Serialize, Deserialize)]
pub struct Content134 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson133,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson133 {
    schema: Schema84,

    examples: Option<Examples45>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples45 {
    incorrect_password: Option<AuthenticationFailedValue>,

    two_factor_not_enabled: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    user_not_found: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema84 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties107>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties107 {
    id: IdElement,

    name: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct I2FaPasswordLess {
    post: I2FaPasswordLessPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I2FaPasswordLessPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody68,

    responses: HashMap<String, Response56>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody68 {
    required: bool,

    content: Content135,
}

#[derive(Serialize, Deserialize)]
pub struct Content135 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson134,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson134 {
    schema: Schema85,
}

#[derive(Serialize, Deserialize)]
pub struct Schema85 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties108,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties108 {
    value: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response56 {
    description: ResponseDescription,

    content: Option<Content136>,
}

#[derive(Serialize, Deserialize)]
pub struct Content136 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson135,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson135 {
    schema: OneOf,

    examples: Examples46,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples46 {
    no_security_key: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct I2_FaRegister {
    post: I2FaRegisterPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I2FaRegisterPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody69,

    responses: HashMap<String, Response57>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody69 {
    required: bool,

    content: Content137,
}

#[derive(Serialize, Deserialize)]
pub struct Content137 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson136,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson136 {
    schema: Schema86,
}

#[derive(Serialize, Deserialize)]
pub struct Schema86 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties109,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties109 {
    password: IdElement,

    token: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct Response57 {
    description: ResponseDescription,

    content: Option<Content138>,
}

#[derive(Serialize, Deserialize)]
pub struct Content138 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson137,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson137 {
    schema: Schema87,

    examples: Option<Examples47>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples47 {
    incorrect_password: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema87 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties110>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties110 {
    qr: IdElement,

    url: IdElement,

    secret: IdElement,

    label: IdElement,

    issuer: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct I2FaRegisterKey {
    post: I2FaRegisterKeyPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I2FaRegisterKeyPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody69,

    responses: HashMap<String, Response58>,
}

#[derive(Serialize, Deserialize)]
pub struct Response58 {
    description: ResponseDescription,

    content: Content139,
}

#[derive(Serialize, Deserialize)]
pub struct Content139 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson138,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson138 {
    schema: Schema88,

    examples: Option<Examples45>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema88 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties111>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties111 {
    rp: Rp,

    user: FluffyUser,

    challenge: IdElement,

    pub_key_cred_params: PubKeyCredParams,

    timeout: ImageUrl,

    exclude_credentials: ExcludeCredentials,

    authenticator_selection: AuthenticatorSelection,

    attestation: ReactionAcceptance,

    extensions: Extensions,
}

#[derive(Serialize, Deserialize)]
pub struct AuthenticatorSelection {
    #[serde(rename = "type")]
    authenticator_selection_type: Vec<TentacledType>,

    properties: AuthenticatorSelectionProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorSelectionProperties {
    authenticator_attachment: Method,

    require_resident_key: IdElement,

    user_verification: Method,
}

#[derive(Serialize, Deserialize)]
pub struct ExcludeCredentials {
    #[serde(rename = "type")]
    exclude_credentials_type: Vec<ExcludeCredentialsType>,

    items: ExcludeCredentialsItems,
}

#[derive(Serialize, Deserialize)]
pub struct ExcludeCredentialsItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: Properties112,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties112 {
    id: IdElement,

    #[serde(rename = "type")]
    properties_type: IdElement,

    transports: On,
}

#[derive(Serialize, Deserialize)]
pub struct Extensions {
    #[serde(rename = "type")]
    extensions_type: Vec<TentacledType>,

    properties: ExtensionsProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionsProperties {
    appid: ImageUrl,

    cred_props: ImageUrl,

    hmac_create_secret: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct PubKeyCredParams {
    #[serde(rename = "type")]
    pub_key_cred_params_type: UsersType,

    items: PubKeyCredParamsItems,
}

#[derive(Serialize, Deserialize)]
pub struct PubKeyCredParamsItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: Properties113,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties113 {
    #[serde(rename = "type")]
    properties_type: IdElement,

    alg: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Rp {
    #[serde(rename = "type")]
    rp_type: TentacledType,

    properties: RpProperties,
}

#[derive(Serialize, Deserialize)]
pub struct RpProperties {
    id: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyUser {
    #[serde(rename = "type")]
    user_type: TentacledType,

    properties: UserProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProperties {
    id: IdElement,

    name: IdElement,

    display_name: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct I2FaRemoveKey {
    post: I2FaRemoveKeyPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I2FaRemoveKeyPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody70,

    responses: HashMap<String, Response57>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody70 {
    required: bool,

    content: Content140,
}

#[derive(Serialize, Deserialize)]
pub struct Content140 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson139,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson139 {
    schema: Schema89,
}

#[derive(Serialize, Deserialize)]
pub struct Schema89 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties114,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties114 {
    password: IdElement,

    token: ImageUrl,

    credential_id: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct I2FaUpdateKey {
    post: I2FaUpdateKeyPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I2FaUpdateKeyPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody71,

    responses: HashMap<String, Response59>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody71 {
    required: bool,

    content: Content141,
}

#[derive(Serialize, Deserialize)]
pub struct Content141 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson140,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson140 {
    schema: Schema90,
}

#[derive(Serialize, Deserialize)]
pub struct Schema90 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties115,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties115 {
    name: UrlClass,

    credential_id: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response59 {
    description: ResponseDescription,

    content: Option<Content142>,
}

#[derive(Serialize, Deserialize)]
pub struct Content142 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson141,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson141 {
    schema: OneOf,

    examples: Examples48,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples48 {
    no_such_key: Option<AuthenticationFailedValue>,

    access_denied: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct IApps {
    post: IAppsPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IAppsPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody72,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody72 {
    required: bool,

    content: Content143,
}

#[derive(Serialize, Deserialize)]
pub struct Content143 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson142,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson142 {
    schema: Schema91,
}

#[derive(Serialize, Deserialize)]
pub struct Schema91 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties116,
}

#[derive(Serialize, Deserialize)]
pub struct Properties116 {
    sort: Method,
}

#[derive(Serialize, Deserialize)]
pub struct IAuthorizedApps {
    post: IAuthorizedAppsPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IAuthorizedAppsPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody73,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody73 {
    required: bool,

    content: Content144,
}

#[derive(Serialize, Deserialize)]
pub struct Content144 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson143,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson143 {
    schema: Schema92,
}

#[derive(Serialize, Deserialize)]
pub struct Schema92 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties117,
}

#[derive(Serialize, Deserialize)]
pub struct Properties117 {
    limit: Limit,

    offset: NotesPerOneAd,

    sort: NoteSearchableScope,
}

#[derive(Serialize, Deserialize)]
pub struct IImportAntennas {
    post: IImportAntennasPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IImportAntennasPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody45,

    responses: HashMap<String, Response60>,
}

#[derive(Serialize, Deserialize)]
pub struct Response60 {
    description: ResponseDescription,

    content: Option<Content145>,
}

#[derive(Serialize, Deserialize)]
pub struct Content145 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson144,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson144 {
    schema: OneOf,

    examples: Examples49,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples49 {
    no_such_file: Option<AuthenticationFailedValue>,

    no_such_user: Option<AuthenticationFailedValue>,

    empty_file: Option<AuthenticationFailedValue>,

    too_many_antennas: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct IImport {
    post: IImportBlockingPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IImportBlockingPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody45,

    responses: HashMap<String, Response61>,
}

#[derive(Serialize, Deserialize)]
pub struct Response61 {
    description: ResponseDescription,

    content: Option<Content146>,
}

#[derive(Serialize, Deserialize)]
pub struct Content146 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson145,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson145 {
    schema: OneOf,

    examples: Examples50,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples50 {
    no_such_file: Option<AuthenticationFailedValue>,

    unexpected_file_type: Option<AuthenticationFailedValue>,

    too_big_file: Option<AuthenticationFailedValue>,

    empty_file: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct IImportFollowing {
    post: IImportFollowingPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IImportFollowingPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody74,

    responses: HashMap<String, Response61>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody74 {
    required: bool,

    content: Content147,
}

#[derive(Serialize, Deserialize)]
pub struct Content147 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson146,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson146 {
    schema: Schema93,
}

#[derive(Serialize, Deserialize)]
pub struct Schema93 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties118,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties118 {
    file_id: StartsAtClass,

    with_replies: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct IMove {
    post: IMovePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IMovePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody75,

    responses: HashMap<String, Response42>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody75 {
    required: bool,

    content: Content148,
}

#[derive(Serialize, Deserialize)]
pub struct Content148 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson147,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson147 {
    schema: Schema94,
}

#[derive(Serialize, Deserialize)]
pub struct Schema94 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties119,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties119 {
    move_to_account: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct INotifications {
    post: INotificationsPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct INotificationsPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody76,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody76 {
    required: bool,

    content: Content149,
}

#[derive(Serialize, Deserialize)]
pub struct Content149 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson148,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson148 {
    schema: Schema95,
}

#[derive(Serialize, Deserialize)]
pub struct Schema95 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties120,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties120 {
    limit: Limit,

    since_id: StartsAtClass,

    until_id: StartsAtClass,

    mark_as_read: CaseSensitive,

    include_types: On,

    exclude_types: On,
}

#[derive(Serialize, Deserialize)]
pub struct IPin {
    post: IPinPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPinPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody77,

    responses: HashMap<String, Response62>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody77 {
    required: bool,

    content: Content150,
}

#[derive(Serialize, Deserialize)]
pub struct Content150 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson149,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson149 {
    schema: Schema96,
}

#[derive(Serialize, Deserialize)]
pub struct Schema96 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties121,

    required: Vec<FluffyRequired>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties121 {
    note_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FluffyRequired {
    #[serde(rename = "noteId")]
    NoteId,
}

#[derive(Serialize, Deserialize)]
pub struct Response62 {
    description: ResponseDescription,

    content: Content151,
}

#[derive(Serialize, Deserialize)]
pub struct Content151 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson150,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson150 {
    schema: Blockee,

    examples: Option<Examples51>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples51 {
    no_such_note: Option<AuthenticationFailedValue>,

    pin_limit_exceeded: Option<AuthenticationFailedValue>,

    already_pinned: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct IRegistryGetClass {
    post: IRegistryGetPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IRegistryGetPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody78,

    responses: HashMap<String, Response63>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody78 {
    required: bool,

    content: Content152,
}

#[derive(Serialize, Deserialize)]
pub struct Content152 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson151,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson151 {
    schema: Schema97,
}

#[derive(Serialize, Deserialize)]
pub struct Schema97 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties122,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties122 {
    key: IdElement,

    scope: Scope,

    domain: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct Response63 {
    description: ResponseDescription,

    content: Option<Content153>,
}

#[derive(Serialize, Deserialize)]
pub struct Content153 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson152,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson152 {
    schema: Blockee,

    examples: Option<Examples52>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples52 {
    no_such_key: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct IRegistryGetAllClass {
    post: IRegistryGetAllPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IRegistryGetAllPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody79,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody79 {
    required: bool,

    content: Content154,
}

#[derive(Serialize, Deserialize)]
pub struct Content154 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson153,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson153 {
    schema: Schema98,
}

#[derive(Serialize, Deserialize)]
pub struct Schema98 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties123,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties123 {
    scope: Scope,

    domain: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct IRegistryGetDetail {
    post: IRegistryGetDetailPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IRegistryGetDetailPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody78,

    responses: HashMap<String, Response64>,
}

#[derive(Serialize, Deserialize)]
pub struct Response64 {
    description: ResponseDescription,

    content: Content155,
}

#[derive(Serialize, Deserialize)]
pub struct Content155 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson154,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson154 {
    schema: Schema99,

    examples: Option<Examples52>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema99 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties124>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties124 {
    updated_at: IdElement,

    value: Var,
}

#[derive(Serialize, Deserialize)]
pub struct IRegistryScopesWithDomain {
    post: IRegistryScopesWithDomainPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IRegistryScopesWithDomainPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct IUnpin {
    post: IUnpinPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IUnpinPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Option<Vec<Security>>,

    request_body: RequestBody77,

    responses: HashMap<String, Response65>,
}

#[derive(Serialize, Deserialize)]
pub struct Response65 {
    description: ResponseDescription,

    content: Option<Content156>,
}

#[derive(Serialize, Deserialize)]
pub struct Content156 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson155,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson155 {
    schema: Blockee,

    examples: Option<Examples53>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples53 {
    no_such_note: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct IUpdate {
    post: IUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody80,

    responses: HashMap<String, Response42>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody80 {
    required: bool,

    content: Content157,
}

#[derive(Serialize, Deserialize)]
pub struct Content157 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson156,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson156 {
    schema: Schema100,
}

#[derive(Serialize, Deserialize)]
pub struct Schema100 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties125,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties125 {
    name: DescriptionClass,

    description: DescriptionClass,

    location: DescriptionClass,

    birthday: Birthday,

    lang: ReactionAcceptance,

    avatar_id: BannerUrlClass,

    avatar_decorations: FluffyAvatarDecorations,

    banner_id: BannerUrlClass,

    fields: Fields,

    is_locked: IdElement,

    is_explorable: IdElement,

    hide_online_status: IdElement,

    public_reactions: IdElement,

    careful_bot: IdElement,

    auto_accept_followed: IdElement,

    no_crawle: IdElement,

    prevent_ai_learning: IdElement,

    is_bot: IdElement,

    is_cat: IdElement,

    inject_featured_note: IdElement,

    receive_announcement_email: IdElement,

    always_mark_nsfw: IdElement,

    auto_sensitive: IdElement,

    following_visibility: Method,

    followers_visibility: Method,

    pinned_page_id: BannerUrlClass,

    muted_words: HardMutedWordsClass,

    hard_muted_words: HardMutedWordsClass,

    muted_instances: Users,

    notification_recieve_config: NotificationRecieveConfig,

    email_notification_types: Users,

    also_known_as: Permission,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyAvatarDecorations {
    #[serde(rename = "type")]
    avatar_decorations_type: UsersType,

    max_items: i64,

    items: CunningItems,
}

#[derive(Serialize, Deserialize)]
pub struct CunningItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: Properties126,

    required: Vec<Format>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties126 {
    id: StartsAtClass,

    angle: Angle,

    flip_h: ImageUrl,

    offset_x: Angle,

    offset_y: Angle,
}

#[derive(Serialize, Deserialize)]
pub struct Angle {
    #[serde(rename = "type")]
    angle_type: Vec<TentacledType>,

    maximum: f64,

    minimum: f64,
}

#[derive(Serialize, Deserialize)]
pub struct HardMutedWordsClass {
    #[serde(rename = "type")]
    muted_words_type: UsersType,

    items: HardMutedWordsItems,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HardMutedWordsItems {
    one_of: Vec<Users>,
}

#[derive(Serialize, Deserialize)]
pub struct IUpdateEmail {
    post: IUpdateEmailPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IUpdateEmailPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody81,

    responses: HashMap<String, Response66>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody81 {
    required: bool,

    content: Content158,
}

#[derive(Serialize, Deserialize)]
pub struct Content158 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson157,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson157 {
    schema: Schema101,
}

#[derive(Serialize, Deserialize)]
pub struct Schema101 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties127,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties127 {
    password: IdElement,

    email: ImageUrl,

    token: ImageUrl,
}

#[derive(Serialize, Deserialize)]
pub struct Response66 {
    description: ResponseDescription,

    content: Content159,
}

#[derive(Serialize, Deserialize)]
pub struct Content159 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson158,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson158 {
    schema: Blockee,

    examples: Option<Examples54>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples54 {
    incorrect_password: Option<AuthenticationFailedValue>,

    unavailable: Option<AuthenticationFailedValue>,

    email_required: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct IWebhooksCreate {
    post: IWebhooksCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IWebhooksCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody82,

    responses: HashMap<String, Response67>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody82 {
    required: bool,

    content: Content160,
}

#[derive(Serialize, Deserialize)]
pub struct Content160 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson159,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson159 {
    schema: Schema102,
}

#[derive(Serialize, Deserialize)]
pub struct Schema102 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties128,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties128 {
    name: UrlClass,

    url: UrlClass,

    secret: FluffyName,

    on: On,
}

#[derive(Serialize, Deserialize)]
pub struct Response67 {
    description: ResponseDescription,

    content: Content161,
}

#[derive(Serialize, Deserialize)]
pub struct Content161 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson160,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson160 {
    schema: Schema28,

    examples: Option<Examples55>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples55 {
    too_many_webhooks: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct IWebhooks {
    post: IWebhooksDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IWebhooksDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody83,

    responses: HashMap<String, Response8>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody83 {
    required: bool,

    content: Content162,
}

#[derive(Serialize, Deserialize)]
pub struct Content162 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson161,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson161 {
    schema: Schema103,
}

#[derive(Serialize, Deserialize)]
pub struct Schema103 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties129,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties129 {
    webhook_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct IWebhooksUpdate {
    post: IWebhooksUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IWebhooksUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody84,

    responses: HashMap<String, Response8>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody84 {
    required: bool,

    content: Content163,
}

#[derive(Serialize, Deserialize)]
pub struct Content163 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson162,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson162 {
    schema: Schema104,
}

#[derive(Serialize, Deserialize)]
pub struct Schema104 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties130,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties130 {
    webhook_id: StartsAtClass,

    name: UrlClass,

    url: UrlClass,

    secret: SecretClass,

    on: On,

    active: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct InviteCreate {
    post: InviteCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    responses: HashMap<String, Response68>,
}

#[derive(Serialize, Deserialize)]
pub struct Response68 {
    description: ResponseDescription,

    content: Content164,
}

#[derive(Serialize, Deserialize)]
pub struct Content164 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson163,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson163 {
    schema: Blockee,

    examples: Option<Examples56>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples56 {
    exceeded_limit_of_create_invite_code: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct InviteDelete {
    post: InviteDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody85,

    responses: HashMap<String, Response69>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody85 {
    required: bool,

    content: Content165,
}

#[derive(Serialize, Deserialize)]
pub struct Content165 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson164,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson164 {
    schema: Schema105,
}

#[derive(Serialize, Deserialize)]
pub struct Schema105 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties131,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties131 {
    invite_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response69 {
    description: ResponseDescription,

    content: Option<Content166>,
}

#[derive(Serialize, Deserialize)]
pub struct Content166 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson165,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson165 {
    schema: OneOf,

    examples: Examples57,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples57 {
    no_such_invite_code: Option<AuthenticationFailedValue>,

    can_not_delete_invite_code: Option<AuthenticationFailedValue>,

    access_denied: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct InviteLimit {
    post: InviteLimitPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteLimitPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    post: MetaPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaPost {
    operation_id: OperationId,

    summary: OperationId,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    request_body: RequestBody86,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody86 {
    required: bool,

    content: Content167,
}

#[derive(Serialize, Deserialize)]
pub struct Content167 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson166,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson166 {
    schema: Schema106,
}

#[derive(Serialize, Deserialize)]
pub struct Schema106 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties132,
}

#[derive(Serialize, Deserialize)]
pub struct Properties132 {
    detail: CaseSensitive,
}

#[derive(Serialize, Deserialize)]
pub struct MiauthGenToken {
    post: MiauthGenTokenPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiauthGenTokenPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody87,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody87 {
    required: bool,

    content: Content168,
}

#[derive(Serialize, Deserialize)]
pub struct Content168 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson167,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson167 {
    schema: Schema107,
}

#[derive(Serialize, Deserialize)]
pub struct Schema107 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties133,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties133 {
    session: ImageUrl,

    name: ImageUrl,

    description: ImageUrl,

    icon_url: ImageUrl,

    permission: Permission,
}

#[derive(Serialize, Deserialize)]
pub struct MuteCreate {
    post: MuteCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MuteCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody88,

    responses: HashMap<String, Response70>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody88 {
    required: bool,

    content: Content169,
}

#[derive(Serialize, Deserialize)]
pub struct Content169 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson168,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson168 {
    schema: Schema108,
}

#[derive(Serialize, Deserialize)]
pub struct Schema108 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties134,

    required: Vec<PurpleRequired>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties134 {
    user_id: StartsAtClass,

    expires_at: CategoryClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response70 {
    description: ResponseDescription,

    content: Option<Content170>,
}

#[derive(Serialize, Deserialize)]
pub struct Content170 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson169,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson169 {
    schema: OneOf,

    examples: Examples58,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples58 {
    no_such_user: Option<AuthenticationFailedValue>,

    mutee_is_yourself: Option<AuthenticationFailedValue>,

    already_muting: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    not_muting: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct MuteDelete {
    post: MuteDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MuteDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: IndecentRequestBody,

    responses: HashMap<String, Response71>,
}

#[derive(Serialize, Deserialize)]
pub struct Response71 {
    description: ResponseDescription,

    content: Option<Content171>,
}

#[derive(Serialize, Deserialize)]
pub struct Content171 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson170,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson170 {
    schema: OneOf,

    examples: Examples59,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples59 {
    no_such_user: Option<AuthenticationFailedValue>,

    mutee_is_yourself: Option<AuthenticationFailedValue>,

    already_muting: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    not_muting: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct MyApps {
    post: MyAppsPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyAppsPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody89,

    responses: HashMap<String, TentacledResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody89 {
    required: bool,

    content: Content172,
}

#[derive(Serialize, Deserialize)]
pub struct Content172 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson171,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson171 {
    schema: Schema109,
}

#[derive(Serialize, Deserialize)]
pub struct Schema109 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties135,
}

#[derive(Serialize, Deserialize)]
pub struct Properties135 {
    limit: Limit,

    offset: NotesPerOneAd,
}

#[derive(Serialize, Deserialize)]
pub struct NotesClips {
    post: NotesClipsPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesClipsPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody77,

    responses: HashMap<String, Response72>,
}

#[derive(Serialize, Deserialize)]
pub struct Response72 {
    description: ResponseDescription,

    content: Option<Content173>,
}

#[derive(Serialize, Deserialize)]
pub struct Content173 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson172,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson172 {
    schema: StickySchema,

    examples: Option<Examples60>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples60 {
    no_such_note: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesConversation {
    post: NotesConversationPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesConversationPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody90,

    responses: HashMap<String, Response72>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody90 {
    required: bool,

    content: Content174,
}

#[derive(Serialize, Deserialize)]
pub struct Content174 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson173,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson173 {
    schema: Schema110,
}

#[derive(Serialize, Deserialize)]
pub struct Schema110 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties136,

    required: Vec<FluffyRequired>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties136 {
    note_id: StartsAtClass,

    limit: Limit,

    offset: NotesPerOneAd,
}

#[derive(Serialize, Deserialize)]
pub struct NotesCreate {
    post: NotesCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody91,

    responses: HashMap<String, Response73>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody91 {
    required: bool,

    content: Content175,
}

#[derive(Serialize, Deserialize)]
pub struct Content175 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson174,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson174 {
    schema: Schema111,
}

#[derive(Serialize, Deserialize)]
pub struct Schema111 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties137,

    #[serde(rename = "if")]
    schema_if: If,

    then: Then,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties137 {
    visibility: NoteSearchableScope,

    visible_user_ids: UserIds,

    cw: DescriptionClass,

    local_only: CaseSensitive,

    reaction_acceptance: PurpleReactionAcceptance,

    no_extract_mentions: CaseSensitive,

    no_extract_hashtags: CaseSensitive,

    no_extract_emojis: CaseSensitive,

    reply_id: BannerUrlClass,

    renote_id: BannerUrlClass,

    channel_id: BannerUrlClass,

    text: DescriptionClass,

    file_ids: Ids,

    media_ids: Ids,

    poll: FluffyPoll,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyPoll {
    #[serde(rename = "type")]
    poll_type: Vec<TentacledType>,

    properties: Properties138,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties138 {
    choices: FluffyChoices,

    multiple: IdElement,

    expires_at: ImageUrl,

    expired_after: ExpiredAfter,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyChoices {
    #[serde(rename = "type")]
    choices_type: UsersType,

    unique_items: bool,

    min_items: i64,

    max_items: i64,

    items: UrlClass,
}

#[derive(Serialize, Deserialize)]
pub struct ExpiredAfter {
    #[serde(rename = "type")]
    expired_after_type: Vec<TentacledType>,

    minimum: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleReactionAcceptance {
    #[serde(rename = "type")]
    reaction_acceptance_type: Vec<TentacledType>,

    #[serde(rename = "enum")]
    reaction_acceptance_enum: Vec<Option<String>>,

    #[serde(rename = "default")]
    reaction_acceptance_default: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIds {
    #[serde(rename = "type")]
    user_ids_type: UsersType,

    unique_items: bool,

    items: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct If {
    properties: IfProperties,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfProperties {
    renote_id: IdElement,

    file_ids: IdElement,

    media_ids: IdElement,

    poll: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Then {
    properties: ThenProperties,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ThenProperties {
    text: UrlClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response73 {
    description: ResponseDescription,

    content: Content176,
}

#[derive(Serialize, Deserialize)]
pub struct Content176 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson175,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson175 {
    schema: Schema112,

    examples: Option<HashMap<String, AuthenticationFailedValue>>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema112 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties139>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties139 {
    created_note: Blockee,
}

#[derive(Serialize, Deserialize)]
pub struct NotesDelete {
    post: NotesDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody77,

    responses: HashMap<String, Response74>,
}

#[derive(Serialize, Deserialize)]
pub struct Response74 {
    description: ResponseDescription,

    content: Option<Content177>,
}

#[derive(Serialize, Deserialize)]
pub struct Content177 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson176,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson176 {
    schema: OneOf,

    examples: Examples61,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples61 {
    no_such_note: Option<AuthenticationFailedValue>,

    access_denied: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesFavoritesCreate {
    post: NotesFavoritesCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesFavoritesCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody77,

    responses: HashMap<String, Response75>,
}

#[derive(Serialize, Deserialize)]
pub struct Response75 {
    description: ResponseDescription,

    content: Option<Content178>,
}

#[derive(Serialize, Deserialize)]
pub struct Content178 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson177,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson177 {
    schema: OneOf,

    examples: Examples62,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples62 {
    no_such_note: Option<AuthenticationFailedValue>,

    already_favorited: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesFavoritesDelete {
    post: NotesFavoritesDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesFavoritesDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody77,

    responses: HashMap<String, Response76>,
}

#[derive(Serialize, Deserialize)]
pub struct Response76 {
    description: ResponseDescription,

    content: Option<Content179>,
}

#[derive(Serialize, Deserialize)]
pub struct Content179 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson178,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson178 {
    schema: OneOf,

    examples: Examples63,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples63 {
    no_such_note: Option<AuthenticationFailedValue>,

    not_favorited: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesFeatured {
    get: AdminAdListPost,

    post: AdminAdListPost,
}

#[derive(Serialize, Deserialize)]
pub struct NotesGlobalTimeline {
    post: NotesGlobalTimelinePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesGlobalTimelinePost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody92,

    responses: HashMap<String, Response77>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody92 {
    required: bool,

    content: Content180,
}

#[derive(Serialize, Deserialize)]
pub struct Content180 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson179,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson179 {
    schema: Schema113,
}

#[derive(Serialize, Deserialize)]
pub struct Schema113 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties140,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties140 {
    with_files: CaseSensitive,

    with_renotes: CaseSensitive,

    limit: Limit,

    since_id: StartsAtClass,

    until_id: StartsAtClass,

    since_date: IdElement,

    until_date: IdElement,

    with_replies: Option<CaseSensitive>,

    allow_partial: Option<CaseSensitive>,
}

#[derive(Serialize, Deserialize)]
pub struct Response77 {
    description: ResponseDescription,

    content: Content181,
}

#[derive(Serialize, Deserialize)]
pub struct Content181 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson180,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson180 {
    schema: StickySchema,

    examples: Option<Examples64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples64 {
    gtl_disabled: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesHybridTimeline {
    post: NotesHybridTimelinePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesHybridTimelinePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody93,

    responses: HashMap<String, Response78>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody93 {
    required: bool,

    content: Content182,
}

#[derive(Serialize, Deserialize)]
pub struct Content182 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson181,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson181 {
    schema: Schema114,
}

#[derive(Serialize, Deserialize)]
pub struct Schema114 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties141,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties141 {
    limit: Limit,

    since_id: StartsAtClass,

    until_id: StartsAtClass,

    since_date: IdElement,

    until_date: IdElement,

    allow_partial: CaseSensitive,

    include_my_renotes: Option<CaseSensitive>,

    include_renoted_my_notes: Option<CaseSensitive>,

    include_local_renotes: Option<CaseSensitive>,

    with_files: CaseSensitive,

    with_renotes: CaseSensitive,

    with_replies: Option<CaseSensitive>,

    user_id: Option<StartsAtClass>,

    with_channel_notes: Option<CaseSensitive>,
}

#[derive(Serialize, Deserialize)]
pub struct Response78 {
    description: ResponseDescription,

    content: Content183,
}

#[derive(Serialize, Deserialize)]
pub struct Content183 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson182,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson182 {
    schema: StickySchema,

    examples: Option<Examples65>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples65 {
    stl_disabled: Option<AuthenticationFailedValue>,

    both_with_replies_and_with_files: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesLocalTimeline {
    post: NotesLocalTimelinePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesLocalTimelinePost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody92,

    responses: HashMap<String, Response79>,
}

#[derive(Serialize, Deserialize)]
pub struct Response79 {
    description: ResponseDescription,

    content: Content184,
}

#[derive(Serialize, Deserialize)]
pub struct Content184 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson183,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson183 {
    schema: StickySchema,

    examples: Option<Examples66>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples66 {
    ltl_disabled: Option<AuthenticationFailedValue>,

    both_with_replies_and_with_files: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesPollsVote {
    post: NotesPollsVotePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesPollsVotePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody94,

    responses: HashMap<String, Response80>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody94 {
    required: bool,

    content: Content185,
}

#[derive(Serialize, Deserialize)]
pub struct Content185 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson184,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson184 {
    schema: Schema115,
}

#[derive(Serialize, Deserialize)]
pub struct Schema115 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties142,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties142 {
    note_id: StartsAtClass,

    choice: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response80 {
    description: ResponseDescription,

    content: Option<Content186>,
}

#[derive(Serialize, Deserialize)]
pub struct Content186 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson185,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson185 {
    schema: OneOf,

    examples: HashMap<String, AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesReactions {
    get: NotesReactionsGet,

    post: NotesReactionsGet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesReactionsGet {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody19,

    responses: HashMap<String, Response72>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesReactionsCreate {
    post: NotesReactionsCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesReactionsCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody95,

    responses: HashMap<String, Response81>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody95 {
    required: bool,

    content: Content187,
}

#[derive(Serialize, Deserialize)]
pub struct Content187 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson186,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson186 {
    schema: Schema116,
}

#[derive(Serialize, Deserialize)]
pub struct Schema116 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties143,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties143 {
    note_id: StartsAtClass,

    reaction: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response81 {
    description: ResponseDescription,

    content: Option<Content188>,
}

#[derive(Serialize, Deserialize)]
pub struct Content188 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson187,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson187 {
    schema: OneOf,

    examples: Examples67,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples67 {
    no_such_note: Option<AuthenticationFailedValue>,

    already_reacted: Option<AuthenticationFailedValue>,

    you_have_been_blocked: Option<AuthenticationFailedValue>,

    cannot_react_to_renote: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesReactionsDelete {
    post: NotesReactionsDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesReactionsDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody77,

    responses: HashMap<String, Response82>,
}

#[derive(Serialize, Deserialize)]
pub struct Response82 {
    description: ResponseDescription,

    content: Option<Content189>,
}

#[derive(Serialize, Deserialize)]
pub struct Content189 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson188,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson188 {
    schema: OneOf,

    examples: Examples68,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples68 {
    no_such_note: Option<AuthenticationFailedValue>,

    not_reacted: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesRenotes {
    post: NotesReactionsGet,
}

#[derive(Serialize, Deserialize)]
pub struct NotesSearch {
    post: NotesSearchPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesSearchPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody96,

    responses: HashMap<String, Response83>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody96 {
    required: bool,

    content: Content190,
}

#[derive(Serialize, Deserialize)]
pub struct Content190 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson189,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson189 {
    schema: Schema117,
}

#[derive(Serialize, Deserialize)]
pub struct Schema117 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties144,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties144 {
    query: IdElement,

    since_id: StartsAtClass,

    until_id: StartsAtClass,

    limit: Limit,

    offset: NotesPerOneAd,

    host: Code,

    user_id: PurpleId,

    channel_id: PurpleId,
}

#[derive(Serialize, Deserialize)]
pub struct Response83 {
    description: ResponseDescription,

    content: Content191,
}

#[derive(Serialize, Deserialize)]
pub struct Content191 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson190,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson190 {
    schema: StickySchema,

    examples: Option<Examples69>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples69 {
    unavailable: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesState {
    post: NotesStatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesStatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody77,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct Notes {
    post: NotesThreadMutingCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesThreadMutingCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody77,

    responses: HashMap<String, Response84>,
}

#[derive(Serialize, Deserialize)]
pub struct Response84 {
    description: ResponseDescription,

    content: Option<Content192>,
}

#[derive(Serialize, Deserialize)]
pub struct Content192 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson191,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson191 {
    schema: OneOf,

    examples: Examples53,
}

#[derive(Serialize, Deserialize)]
pub struct NotesThreadMutingDelete {
    post: NotesThreadMutingDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesThreadMutingDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody77,

    responses: HashMap<String, Response85>,
}

#[derive(Serialize, Deserialize)]
pub struct Response85 {
    description: ResponseDescription,

    content: Option<Content193>,
}

#[derive(Serialize, Deserialize)]
pub struct Content193 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson192,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson192 {
    schema: OneOf,

    examples: Examples60,
}

#[derive(Serialize, Deserialize)]
pub struct NotesTranslate {
    post: NotesTranslatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesTranslatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody97,

    responses: HashMap<String, Response86>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody97 {
    required: bool,

    content: Content194,
}

#[derive(Serialize, Deserialize)]
pub struct Content194 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson193,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson193 {
    schema: Schema118,
}

#[derive(Serialize, Deserialize)]
pub struct Schema118 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties145,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties145 {
    note_id: StartsAtClass,

    target_lang: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response86 {
    description: ResponseDescription,

    content: Option<Content195>,
}

#[derive(Serialize, Deserialize)]
pub struct Content195 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson194,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson194 {
    schema: Schema119,

    examples: Option<Examples70>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples70 {
    unavailable: Option<AuthenticationFailedValue>,

    no_such_note: Option<AuthenticationFailedValue>,

    cannot_translate_invisible_note: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema119 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties146>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties146 {
    source_lang: IdElement,

    text: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct NotesUserListTimeline {
    post: NotesUserListTimelinePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesUserListTimelinePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody98,

    responses: HashMap<String, Response87>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody98 {
    required: bool,

    content: Content196,
}

#[derive(Serialize, Deserialize)]
pub struct Content196 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson195,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson195 {
    schema: Schema120,
}

#[derive(Serialize, Deserialize)]
pub struct Schema120 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties147,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties147 {
    list_id: StartsAtClass,

    limit: Limit,

    since_id: StartsAtClass,

    until_id: StartsAtClass,

    since_date: IdElement,

    until_date: IdElement,

    allow_partial: CaseSensitive,

    include_my_renotes: CaseSensitive,

    include_renoted_my_notes: CaseSensitive,

    include_local_renotes: CaseSensitive,

    with_renotes: CaseSensitive,

    with_files: WithFiles,
}

#[derive(Serialize, Deserialize)]
pub struct Response87 {
    description: ResponseDescription,

    content: Option<Content197>,
}

#[derive(Serialize, Deserialize)]
pub struct Content197 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson196,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson196 {
    schema: Schema121,

    examples: Option<Examples71>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples71 {
    no_such_list: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema121 {
    #[serde(rename = "type")]
    schema_type: Option<UsersType>,

    items: Option<MagentaItems>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MagentaItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    #[serde(rename = "$ref")]
    items_ref: Option<String>,

    properties: Option<Properties148>,

    required: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties148 {
    id: StartsAtClass,

    created_at: StartsAtClass,

    user_id: StartsAtClass,

    user: Blockee,

    with_replies: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct PagePush {
    post: PagePushPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagePushPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody99,

    responses: HashMap<String, Response88>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody99 {
    required: bool,

    content: Content198,
}

#[derive(Serialize, Deserialize)]
pub struct Content198 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson197,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson197 {
    schema: Schema122,
}

#[derive(Serialize, Deserialize)]
pub struct Schema122 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties149,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties149 {
    page_id: StartsAtClass,

    event: IdElement,

    var: Var,
}

#[derive(Serialize, Deserialize)]
pub struct Response88 {
    description: ResponseDescription,

    content: Option<Content199>,
}

#[derive(Serialize, Deserialize)]
pub struct Content199 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson198,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson198 {
    schema: Blockee,

    examples: Option<Examples72>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples72 {
    no_such_page: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct PagesCreate {
    post: PagesCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagesCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody100,

    responses: HashMap<String, Response89>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody100 {
    required: bool,

    content: Content200,
}

#[derive(Serialize, Deserialize)]
pub struct Content200 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson199,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson199 {
    schema: Schema123,
}

#[derive(Serialize, Deserialize)]
pub struct Schema123 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties150,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties150 {
    title: IdElement,

    name: Password,

    summary: ImageUrl,

    content: VariablesClass,

    variables: VariablesClass,

    script: IdElement,

    eye_catching_image_id: BannerUrlClass,

    font: NoteSearchableScope,

    align_center: CaseSensitive,

    hide_title_when_pinned: CaseSensitive,
}

#[derive(Serialize, Deserialize)]
pub struct VariablesClass {
    #[serde(rename = "type")]
    content_type: UsersType,

    items: ContentItems,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    additional_properties: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Response89 {
    description: ResponseDescription,

    content: Content201,
}

#[derive(Serialize, Deserialize)]
pub struct Content201 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson200,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson200 {
    schema: Blockee,

    examples: Option<Examples73>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples73 {
    no_such_file: Option<AuthenticationFailedValue>,

    name_already_exists: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct PagesDelete {
    post: PagesDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagesDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody101,

    responses: HashMap<String, Response90>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody101 {
    required: bool,

    content: Content202,
}

#[derive(Serialize, Deserialize)]
pub struct Content202 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson201,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson201 {
    schema: Schema124,
}

#[derive(Serialize, Deserialize)]
pub struct Schema124 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties151,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties151 {
    page_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response90 {
    description: ResponseDescription,

    content: Option<Content203>,
}

#[derive(Serialize, Deserialize)]
pub struct Content203 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson202,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson202 {
    schema: OneOf,

    examples: Examples74,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples74 {
    no_such_page: Option<AuthenticationFailedValue>,

    access_denied: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct PagesLike {
    post: PagesLikePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagesLikePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody101,

    responses: HashMap<String, Response91>,
}

#[derive(Serialize, Deserialize)]
pub struct Response91 {
    description: ResponseDescription,

    content: Option<Content204>,
}

#[derive(Serialize, Deserialize)]
pub struct Content204 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson203,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson203 {
    schema: OneOf,

    examples: Examples75,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples75 {
    no_such_page: Option<AuthenticationFailedValue>,

    your_page: Option<AuthenticationFailedValue>,

    already_liked: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct PagesShow {
    post: PagesShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagesShowPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody102,

    responses: HashMap<String, Response88>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody102 {
    required: bool,

    content: Content205,
}

#[derive(Serialize, Deserialize)]
pub struct Content205 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson204,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson204 {
    schema: Schema125,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema125 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties152,

    any_of: Vec<SchemaAnyOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties152 {
    page_id: StartsAtClass,

    name: IdElement,

    username: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct PagesUnlike {
    post: PagesUnlikePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagesUnlikePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody101,

    responses: HashMap<String, Response92>,
}

#[derive(Serialize, Deserialize)]
pub struct Response92 {
    description: ResponseDescription,

    content: Option<Content206>,
}

#[derive(Serialize, Deserialize)]
pub struct Content206 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson205,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson205 {
    schema: OneOf,

    examples: Examples76,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples76 {
    no_such_page: Option<AuthenticationFailedValue>,

    not_liked: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct PagesUpdate {
    post: PagesUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagesUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody103,

    responses: HashMap<String, Response93>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody103 {
    required: bool,

    content: Content207,
}

#[derive(Serialize, Deserialize)]
pub struct Content207 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson206,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson206 {
    schema: Schema126,
}

#[derive(Serialize, Deserialize)]
pub struct Schema126 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties153,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties153 {
    page_id: StartsAtClass,

    title: IdElement,

    name: Password,

    summary: ImageUrl,

    content: VariablesClass,

    variables: VariablesClass,

    script: IdElement,

    eye_catching_image_id: BannerUrlClass,

    font: Method,

    align_center: IdElement,

    hide_title_when_pinned: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response93 {
    description: ResponseDescription,

    content: Option<Content208>,
}

#[derive(Serialize, Deserialize)]
pub struct Content208 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson207,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson207 {
    schema: OneOf,

    examples: Examples77,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples77 {
    no_such_page: Option<AuthenticationFailedValue>,

    access_denied: Option<AuthenticationFailedValue>,

    no_such_file: Option<AuthenticationFailedValue>,

    name_already_exists: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    rate_limit_exceeded: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Ping {
    post: PingPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RenoteMuteCreate {
    post: RenoteMuteCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenoteMuteCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: IndecentRequestBody,

    responses: HashMap<String, Response70>,
}

#[derive(Serialize, Deserialize)]
pub struct Retention {
    get: RetentionGet,

    post: RetentionGet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionGet {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    responses: HashMap<String, Response94>,
}

#[derive(Serialize, Deserialize)]
pub struct Response94 {
    description: ResponseDescription,

    content: Content209,
}

#[derive(Serialize, Deserialize)]
pub struct Content209 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson208,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson208 {
    schema: Schema127,

    examples: Option<FluffyExamples>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema127 {
    #[serde(rename = "type")]
    schema_type: Option<UsersType>,

    items: Option<FriskyItems>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FriskyItems {
    #[serde(rename = "type")]
    items_type: TentacledType,

    properties: Properties154,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties154 {
    created_at: StartsAtClass,

    users: IdElement,

    data: ReactionEmojisClass,
}

#[derive(Serialize, Deserialize)]
pub struct Reversi {
    post: ReversiGamesPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReversiGamesPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    request_body: Option<RequestBody1>,

    responses: HashMap<String, Response95>,

    security: Option<Vec<Security>>,
}

#[derive(Serialize, Deserialize)]
pub struct Response95 {
    description: ResponseDescription,

    content: Content210,
}

#[derive(Serialize, Deserialize)]
pub struct Content210 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson209,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson209 {
    schema: Schema128,

    examples: Option<FluffyExamples>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema128 {
    #[serde(rename = "type")]
    schema_type: Option<UsersType>,

    items: Option<OneOf>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ReversiMatch {
    post: ReversiMatchPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReversiMatchPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody104,

    responses: HashMap<String, Response96>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody104 {
    required: bool,

    content: Content211,
}

#[derive(Serialize, Deserialize)]
pub struct Content211 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson210,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson210 {
    schema: Schema129,
}

#[derive(Serialize, Deserialize)]
pub struct Schema129 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties155,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties155 {
    user_id: BannerUrlClass,

    no_irregular_rules: CaseSensitive,

    multiple: CaseSensitive,
}

#[derive(Serialize, Deserialize)]
pub struct Response96 {
    description: ResponseDescription,

    content: Option<Content212>,
}

#[derive(Serialize, Deserialize)]
pub struct Content212 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson211,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson211 {
    schema: Schema130,

    examples: Option<Examples78>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples78 {
    no_such_user: Option<AuthenticationFailedValue>,

    target_is_yourself: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema130 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    all_of: Option<Vec<OneOf>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ReversiShowGame {
    post: ReversiShowGamePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReversiShowGamePost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    request_body: RequestBody105,

    responses: HashMap<String, Response97>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody105 {
    required: bool,

    content: Content213,
}

#[derive(Serialize, Deserialize)]
pub struct Content213 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson212,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson212 {
    schema: Schema131,
}

#[derive(Serialize, Deserialize)]
pub struct Schema131 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties156,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties156 {
    game_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response97 {
    description: ResponseDescription,

    content: Content214,
}

#[derive(Serialize, Deserialize)]
pub struct Content214 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson213,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson213 {
    schema: Blockee,

    examples: Option<Examples79>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples79 {
    no_such_game: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct ReversiSurrender {
    post: ReversiSurrenderPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReversiSurrenderPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody105,

    responses: HashMap<String, Response98>,
}

#[derive(Serialize, Deserialize)]
pub struct Response98 {
    description: ResponseDescription,

    content: Option<Content215>,
}

#[derive(Serialize, Deserialize)]
pub struct Content215 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson214,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson214 {
    schema: OneOf,

    examples: Examples80,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples80 {
    no_such_game: Option<AuthenticationFailedValue>,

    already_ended: Option<AuthenticationFailedValue>,

    access_denied: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct ReversiVerify {
    post: ReversiVerifyPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReversiVerifyPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    request_body: RequestBody106,

    responses: HashMap<String, Response99>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody106 {
    required: bool,

    content: Content216,
}

#[derive(Serialize, Deserialize)]
pub struct Content216 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson215,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson215 {
    schema: Schema132,
}

#[derive(Serialize, Deserialize)]
pub struct Schema132 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties157,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties157 {
    game_id: StartsAtClass,

    crc32: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response99 {
    description: ResponseDescription,

    content: Content217,
}

#[derive(Serialize, Deserialize)]
pub struct Content217 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson216,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson216 {
    schema: Schema133,

    examples: Option<Examples79>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema133 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties158>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties158 {
    desynced: IdElement,

    game: Folder,
}

#[derive(Serialize, Deserialize)]
pub struct RolesNotes {
    post: RolesNotesPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RolesNotesPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody25,

    responses: HashMap<String, Response6>,
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    get: ServerInfoGet,

    post: ServerInfoGet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfoGet {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    responses: HashMap<String, Response100>,
}

#[derive(Serialize, Deserialize)]
pub struct Response100 {
    description: ResponseDescription,

    content: Content218,
}

#[derive(Serialize, Deserialize)]
pub struct Content218 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson217,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson217 {
    schema: Schema134,

    examples: Option<FluffyExamples>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema134 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties159>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties159 {
    machine: IdElement,

    cpu: Cpu,

    mem: FluffyMem,

    fs: FluffyFs,
}

#[derive(Serialize, Deserialize)]
pub struct SwRegister {
    post: SwRegisterPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwRegisterPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody107,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody107 {
    required: bool,

    content: Content219,
}

#[derive(Serialize, Deserialize)]
pub struct Content219 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson218,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson218 {
    schema: Schema135,
}

#[derive(Serialize, Deserialize)]
pub struct Schema135 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties160,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties160 {
    endpoint: IdElement,

    auth: IdElement,

    publickey: IdElement,

    send_read_message: CaseSensitive,
}

#[derive(Serialize, Deserialize)]
pub struct SwShowRegistration {
    post: SwShowRegistrationPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwShowRegistrationPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody53,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct SwUpdateRegistration {
    post: SwUpdateRegistrationPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwUpdateRegistrationPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody108,

    responses: HashMap<String, Response101>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody108 {
    required: bool,

    content: Content220,
}

#[derive(Serialize, Deserialize)]
pub struct Content220 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson219,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson219 {
    schema: Schema136,
}

#[derive(Serialize, Deserialize)]
pub struct Schema136 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties161,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties161 {
    endpoint: IdElement,

    send_read_message: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct Response101 {
    description: ResponseDescription,

    content: Content221,
}

#[derive(Serialize, Deserialize)]
pub struct Content221 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson220,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson220 {
    schema: Schema137,

    examples: Option<Examples81>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples81 {
    no_such_registration: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema137 {
    #[serde(rename = "type")]
    schema_type: Option<TentacledType>,

    properties: Option<Properties162>,

    required: Option<Vec<String>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties162 {
    user_id: IdElement,

    endpoint: IdElement,

    send_read_message: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct TestClass {
    post: TestPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody109,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody109 {
    required: bool,

    content: Content222,
}

#[derive(Serialize, Deserialize)]
pub struct Content222 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson221,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson221 {
    schema: Schema138,
}

#[derive(Serialize, Deserialize)]
pub struct Schema138 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties163,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties163 {
    required: IdElement,

    string: IdElement,

    #[serde(rename = "default")]
    properties_default: MascotImageUrl,

    nullable_default: FeedbackUrl,

    id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct UsernameAvailable {
    post: UsernameAvailablePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsernameAvailablePost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody110,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody110 {
    required: bool,

    content: Content223,
}

#[derive(Serialize, Deserialize)]
pub struct Content223 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson222,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson222 {
    schema: Schema139,
}

#[derive(Serialize, Deserialize)]
pub struct Schema139 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties164,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties164 {
    username: Username,
}

#[derive(Serialize, Deserialize)]
pub struct UsersAchievements {
    post: UsersAchievementsPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersAchievementsPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    request_body: IndecentRequestBody,

    responses: HashMap<String, Response11>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersClipsClass {
    post: UsersClipsPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersClipsPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody19,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersFeaturedNotes {
    get: AdminDriveFilesPost,

    post: AdminAdListPost,
}

#[derive(Serialize, Deserialize)]
pub struct UsersFollow {
    post: UsersFollowersPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersFollowersPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody111,

    responses: HashMap<String, Response102>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody111 {
    required: bool,

    content: Content224,
}

#[derive(Serialize, Deserialize)]
pub struct Content224 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson223,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson223 {
    schema: Schema140,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema140 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties165,

    any_of: Vec<SchemaAnyOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties165 {
    since_id: StartsAtClass,

    until_id: StartsAtClass,

    limit: Limit,

    user_id: StartsAtClass,

    username: IdElement,

    host: CategoryClass,

    birthday: Option<Birthday>,
}

#[derive(Serialize, Deserialize)]
pub struct Response102 {
    description: ResponseDescription,

    content: Content225,
}

#[derive(Serialize, Deserialize)]
pub struct Content225 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson224,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson224 {
    schema: StickySchema,

    examples: Option<Examples82>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples82 {
    no_such_user: Option<AuthenticationFailedValue>,

    forbidden: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    birthday_date_format_invalid: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersGetFrequentlyRepliedUsers {
    post: UsersGetFrequentlyRepliedUsersPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersGetFrequentlyRepliedUsersPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody112,

    responses: HashMap<String, Response46>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody112 {
    required: bool,

    content: Content226,
}

#[derive(Serialize, Deserialize)]
pub struct Content226 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson225,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson225 {
    schema: Schema141,
}

#[derive(Serialize, Deserialize)]
pub struct Schema141 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties166,

    required: Vec<PurpleRequired>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties166 {
    user_id: StartsAtClass,

    limit: Limit,
}

#[derive(Serialize, Deserialize)]
pub struct UsersListsCreate {
    post: UsersListsCreatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListsCreatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody113,

    responses: HashMap<String, Response103>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody113 {
    required: bool,

    content: Content227,
}

#[derive(Serialize, Deserialize)]
pub struct Content227 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson226,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson226 {
    schema: Schema142,
}

#[derive(Serialize, Deserialize)]
pub struct Schema142 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties167,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Properties167 {
    name: UrlClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response103 {
    description: ResponseDescription,

    content: Content228,
}

#[derive(Serialize, Deserialize)]
pub struct Content228 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson227,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson227 {
    schema: Blockee,

    examples: Option<Examples83>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples83 {
    too_many_userlists: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersListsCreateFromPublic {
    post: UsersListsCreateFromPublicPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListsCreateFromPublicPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody114,

    responses: HashMap<String, Response42>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody114 {
    required: bool,

    content: Content229,
}

#[derive(Serialize, Deserialize)]
pub struct Content229 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson228,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson228 {
    schema: Schema143,
}

#[derive(Serialize, Deserialize)]
pub struct Schema143 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties168,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties168 {
    name: UrlClass,

    list_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct UsersListsDelete {
    post: UsersListsDeletePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListsDeletePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody115,

    responses: HashMap<String, Response87>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody115 {
    required: bool,

    content: Content230,
}

#[derive(Serialize, Deserialize)]
pub struct Content230 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson229,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson229 {
    schema: Schema144,
}

#[derive(Serialize, Deserialize)]
pub struct Schema144 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties169,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties169 {
    list_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct UsersListsFavorite {
    post: UsersListsFavoritePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListsFavoritePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    security: Vec<Security>,

    request_body: RequestBody115,

    responses: HashMap<String, Response104>,
}

#[derive(Serialize, Deserialize)]
pub struct Response104 {
    description: ResponseDescription,

    content: Option<Content231>,
}

#[derive(Serialize, Deserialize)]
pub struct Content231 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson230,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson230 {
    schema: OneOf,

    examples: Examples84,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples84 {
    no_such_user_list: Option<AuthenticationFailedValue>,

    already_favorited: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersListsGetMemberships {
    post: UsersListsGetMembershipsPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListsGetMembershipsPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody116,

    responses: HashMap<String, Response87>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody116 {
    required: bool,

    content: Content232,
}

#[derive(Serialize, Deserialize)]
pub struct Content232 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson231,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson231 {
    schema: Schema145,
}

#[derive(Serialize, Deserialize)]
pub struct Schema145 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties170,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties170 {
    list_id: StartsAtClass,

    for_public: CaseSensitive,

    limit: Limit,

    since_id: StartsAtClass,

    until_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct UsersListsList {
    post: UsersListsListPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListsListPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody117,

    responses: HashMap<String, Response105>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody117 {
    required: bool,

    content: Content233,
}

#[derive(Serialize, Deserialize)]
pub struct Content233 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson232,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson232 {
    schema: Schema146,
}

#[derive(Serialize, Deserialize)]
pub struct Schema146 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties14,
}

#[derive(Serialize, Deserialize)]
pub struct Response105 {
    description: ResponseDescription,

    content: Content234,
}

#[derive(Serialize, Deserialize)]
pub struct Content234 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson233,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson233 {
    schema: StickySchema,

    examples: Option<Examples85>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples85 {
    no_such_user: Option<AuthenticationFailedValue>,

    remote_user_not_allowed: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersListsPull {
    post: UsersListsPullPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListsPullPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody118,

    responses: HashMap<String, Response106>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody118 {
    required: bool,

    content: Content235,
}

#[derive(Serialize, Deserialize)]
pub struct Content235 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson234,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson234 {
    schema: Schema147,
}

#[derive(Serialize, Deserialize)]
pub struct Schema147 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties171,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties171 {
    list_id: StartsAtClass,

    user_id: StartsAtClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response106 {
    description: ResponseDescription,

    content: Option<Content236>,
}

#[derive(Serialize, Deserialize)]
pub struct Content236 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson235,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson235 {
    schema: OneOf,

    examples: Examples86,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples86 {
    no_such_list: Option<AuthenticationFailedValue>,

    no_such_user: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersListsPush {
    post: UsersListsPushPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListsPushPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody118,

    responses: HashMap<String, Response80>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersListsShow {
    post: UsersListsShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListsShowPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody119,

    responses: HashMap<String, Response87>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody119 {
    required: bool,

    content: Content237,
}

#[derive(Serialize, Deserialize)]
pub struct Content237 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson236,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson236 {
    schema: Schema148,
}

#[derive(Serialize, Deserialize)]
pub struct Schema148 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties172,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties172 {
    list_id: StartsAtClass,

    for_public: CaseSensitive,
}

#[derive(Serialize, Deserialize)]
pub struct UsersListsUpdate {
    post: UsersListsUpdatePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListsUpdatePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody120,

    responses: HashMap<String, Response87>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody120 {
    required: bool,

    content: Content238,
}

#[derive(Serialize, Deserialize)]
pub struct Content238 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson237,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson237 {
    schema: Schema149,
}

#[derive(Serialize, Deserialize)]
pub struct Schema149 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties173,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties173 {
    list_id: StartsAtClass,

    name: UrlClass,

    is_public: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct UsersListsUpdateMembership {
    post: UsersListsUpdateMembershipPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListsUpdateMembershipPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody121,

    responses: HashMap<String, Response106>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody121 {
    required: bool,

    content: Content239,
}

#[derive(Serialize, Deserialize)]
pub struct Content239 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson238,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson238 {
    schema: Schema150,
}

#[derive(Serialize, Deserialize)]
pub struct Schema150 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties174,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties174 {
    list_id: StartsAtClass,

    user_id: StartsAtClass,

    with_replies: IdElement,
}

#[derive(Serialize, Deserialize)]
pub struct UsersNotes {
    post: UsersNotesPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersNotesPost {
    operation_id: String,

    summary: String,

    description: PostDescription,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody122,

    responses: HashMap<String, Response107>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody122 {
    required: bool,

    content: Content240,
}

#[derive(Serialize, Deserialize)]
pub struct Content240 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson239,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson239 {
    schema: Schema151,
}

#[derive(Serialize, Deserialize)]
pub struct Schema151 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties141,

    required: Vec<PurpleRequired>,
}

#[derive(Serialize, Deserialize)]
pub struct Response107 {
    description: ResponseDescription,

    content: Content241,
}

#[derive(Serialize, Deserialize)]
pub struct Content241 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson240,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson240 {
    schema: StickySchema,

    examples: Option<Examples87>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples87 {
    no_such_user: Option<AuthenticationFailedValue>,

    both_with_replies_and_with_files: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersReactions {
    post: UsersReactionsPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersReactionsPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody25,

    responses: HashMap<String, Response108>,
}

#[derive(Serialize, Deserialize)]
pub struct Response108 {
    description: ResponseDescription,

    content: Content242,
}

#[derive(Serialize, Deserialize)]
pub struct Content242 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson241,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson241 {
    schema: StickySchema,

    examples: Option<Examples88>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples88 {
    reactions_not_public: Option<AuthenticationFailedValue>,

    is_remote_user: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersRelation {
    post: UsersRelationPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersRelationPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody123,

    responses: HashMap<String, IndigoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody123 {
    required: bool,

    content: Content243,
}

#[derive(Serialize, Deserialize)]
pub struct Content243 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson242,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson242 {
    schema: Schema152,
}

#[derive(Serialize, Deserialize)]
pub struct Schema152 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties175,

    required: Vec<PurpleRequired>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties175 {
    user_id: FluffyUserId,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyUserId {
    any_of: Vec<UserIdAnyOf>,
}

#[derive(Serialize, Deserialize)]
pub struct UserIdAnyOf {
    #[serde(rename = "type")]
    any_of_type: UsersType,

    format: Option<Format>,

    items: Option<StartsAtClass>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersReportAbuse {
    post: UsersReportAbusePost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersReportAbusePost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    security: Vec<Security>,

    request_body: RequestBody124,

    responses: HashMap<String, Response109>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody124 {
    required: bool,

    content: Content244,
}

#[derive(Serialize, Deserialize)]
pub struct Content244 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson243,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson243 {
    schema: Schema153,
}

#[derive(Serialize, Deserialize)]
pub struct Schema153 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties176,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties176 {
    user_id: StartsAtClass,

    comment: UrlClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response109 {
    description: ResponseDescription,

    content: Option<Content245>,
}

#[derive(Serialize, Deserialize)]
pub struct Content245 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson244,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson244 {
    schema: OneOf,

    examples: Examples89,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples89 {
    no_such_user: Option<AuthenticationFailedValue>,

    cannot_report_yourself: Option<AuthenticationFailedValue>,

    cannot_report_the_admin: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersShow {
    post: UsersShowPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersShowPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<String>,

    request_body: RequestBody125,

    responses: HashMap<String, Response110>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody125 {
    required: bool,

    content: Content246,
}

#[derive(Serialize, Deserialize)]
pub struct Content246 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson245,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson245 {
    schema: Schema154,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema154 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties177,

    any_of: Vec<SchemaAnyOf>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties177 {
    user_id: StartsAtClass,

    user_ids: UserIds,

    username: IdElement,

    host: CategoryClass,
}

#[derive(Serialize, Deserialize)]
pub struct Response110 {
    description: ResponseDescription,

    content: Content247,
}

#[derive(Serialize, Deserialize)]
pub struct Content247 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson246,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson246 {
    schema: Schema155,

    examples: Option<Examples90>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Examples90 {
    failed_to_resolve_remote_user: Option<AuthenticationFailedValue>,

    no_such_user: Option<AuthenticationFailedValue>,

    invalid_param: Option<AuthenticationFailedValue>,

    credential_required: Option<AuthenticationFailedValue>,

    authentication_failed: Option<AuthenticationFailedValue>,

    i_am_ai: Option<AuthenticationFailedValue>,

    internal_error: Option<AuthenticationFailedValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema155 {
    one_of: Option<Vec<TentacledOneOf>>,

    #[serde(rename = "$ref")]
    schema_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledOneOf {
    #[serde(rename = "type")]
    one_of_type: UsersType,

    items: Option<Blockee>,

    #[serde(rename = "$ref")]
    one_of_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersUpdateMemo {
    post: UsersUpdateMemoPost,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersUpdateMemoPost {
    operation_id: String,

    summary: String,

    description: String,

    external_docs: ExternalDocs,

    tags: Vec<OperationId>,

    security: Vec<Security>,

    request_body: RequestBody126,

    responses: HashMap<String, Response46>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody126 {
    required: bool,

    content: Content248,
}

#[derive(Serialize, Deserialize)]
pub struct Content248 {
    #[serde(rename = "application/json")]
    application_json: ApplicationJson247,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationJson247 {
    schema: Schema156,
}

#[derive(Serialize, Deserialize)]
pub struct Schema156 {
    #[serde(rename = "type")]
    schema_type: TentacledType,

    properties: Properties178,

    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties178 {
    user_id: StartsAtClass,

    memo: CategoryClass,
}

#[derive(Serialize, Deserialize)]
pub struct Server {
    url: String,
}
