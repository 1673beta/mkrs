/*
 * Misskey API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2024.9.0-alpha.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IUpdateRequest {
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "location", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<String>>,
    #[serde(rename = "birthday", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub birthday: Option<Option<String>>,
    #[serde(rename = "lang", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lang: Option<Option<Lang>>,
    #[serde(rename = "avatarId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<Option<String>>,
    #[serde(rename = "avatarDecorations", skip_serializing_if = "Option::is_none")]
    pub avatar_decorations: Option<Vec<models::IUpdateRequestAvatarDecorationsInner>>,
    #[serde(rename = "bannerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub banner_id: Option<Option<String>>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<models::IUpdateRequestFieldsInner>>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isExplorable", skip_serializing_if = "Option::is_none")]
    pub is_explorable: Option<bool>,
    #[serde(rename = "hideOnlineStatus", skip_serializing_if = "Option::is_none")]
    pub hide_online_status: Option<bool>,
    #[serde(rename = "publicReactions", skip_serializing_if = "Option::is_none")]
    pub public_reactions: Option<bool>,
    #[serde(rename = "carefulBot", skip_serializing_if = "Option::is_none")]
    pub careful_bot: Option<bool>,
    #[serde(rename = "autoAcceptFollowed", skip_serializing_if = "Option::is_none")]
    pub auto_accept_followed: Option<bool>,
    #[serde(rename = "noCrawle", skip_serializing_if = "Option::is_none")]
    pub no_crawle: Option<bool>,
    #[serde(rename = "preventAiLearning", skip_serializing_if = "Option::is_none")]
    pub prevent_ai_learning: Option<bool>,
    #[serde(rename = "isBot", skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<bool>,
    #[serde(rename = "isCat", skip_serializing_if = "Option::is_none")]
    pub is_cat: Option<bool>,
    #[serde(rename = "injectFeaturedNote", skip_serializing_if = "Option::is_none")]
    pub inject_featured_note: Option<bool>,
    #[serde(rename = "receiveAnnouncementEmail", skip_serializing_if = "Option::is_none")]
    pub receive_announcement_email: Option<bool>,
    #[serde(rename = "alwaysMarkNsfw", skip_serializing_if = "Option::is_none")]
    pub always_mark_nsfw: Option<bool>,
    #[serde(rename = "autoSensitive", skip_serializing_if = "Option::is_none")]
    pub auto_sensitive: Option<bool>,
    #[serde(rename = "followingVisibility", skip_serializing_if = "Option::is_none")]
    pub following_visibility: Option<FollowingVisibility>,
    #[serde(rename = "followersVisibility", skip_serializing_if = "Option::is_none")]
    pub followers_visibility: Option<FollowersVisibility>,
    #[serde(rename = "pinnedPageId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pinned_page_id: Option<Option<String>>,
    #[serde(rename = "mutedWords", skip_serializing_if = "Option::is_none")]
    pub muted_words: Option<Vec<models::IUpdateRequestMutedWordsInner>>,
    #[serde(rename = "hardMutedWords", skip_serializing_if = "Option::is_none")]
    pub hard_muted_words: Option<Vec<models::IUpdateRequestMutedWordsInner>>,
    #[serde(rename = "mutedInstances", skip_serializing_if = "Option::is_none")]
    pub muted_instances: Option<Vec<String>>,
    #[serde(rename = "notificationRecieveConfig", skip_serializing_if = "Option::is_none")]
    pub notification_recieve_config: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfig>>,
    #[serde(rename = "emailNotificationTypes", skip_serializing_if = "Option::is_none")]
    pub email_notification_types: Option<Vec<String>>,
    #[serde(rename = "alsoKnownAs", skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,
}

impl IUpdateRequest {
    pub fn new() -> IUpdateRequest {
        IUpdateRequest {
            name: None,
            description: None,
            location: None,
            birthday: None,
            lang: None,
            avatar_id: None,
            avatar_decorations: None,
            banner_id: None,
            fields: None,
            is_locked: None,
            is_explorable: None,
            hide_online_status: None,
            public_reactions: None,
            careful_bot: None,
            auto_accept_followed: None,
            no_crawle: None,
            prevent_ai_learning: None,
            is_bot: None,
            is_cat: None,
            inject_featured_note: None,
            receive_announcement_email: None,
            always_mark_nsfw: None,
            auto_sensitive: None,
            following_visibility: None,
            followers_visibility: None,
            pinned_page_id: None,
            muted_words: None,
            hard_muted_words: None,
            muted_instances: None,
            notification_recieve_config: None,
            email_notification_types: None,
            also_known_as: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Lang {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "ady")]
    Ady,
    #[serde(rename = "af")]
    Af,
    #[serde(rename = "af-NA")]
    AfNa,
    #[serde(rename = "af-ZA")]
    AfZa,
    #[serde(rename = "ak")]
    Ak,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "ar-AR")]
    ArAr,
    #[serde(rename = "ar-MA")]
    ArMa,
    #[serde(rename = "ar-SA")]
    ArSa,
    #[serde(rename = "ay-BO")]
    AyBo,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "az-AZ")]
    AzAz,
    #[serde(rename = "be-BY")]
    BeBy,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "bg-BG")]
    BgBg,
    #[serde(rename = "bn")]
    Bn,
    #[serde(rename = "bn-IN")]
    BnIn,
    #[serde(rename = "bn-BD")]
    BnBd,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "bs-BA")]
    BsBa,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "ca-ES")]
    CaEs,
    #[serde(rename = "cak")]
    Cak,
    #[serde(rename = "ck-US")]
    CkUs,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "cs-CZ")]
    CsCz,
    #[serde(rename = "cy")]
    Cy,
    #[serde(rename = "cy-GB")]
    CyGb,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "da-DK")]
    DaDk,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "de-AT")]
    DeAt,
    #[serde(rename = "de-DE")]
    DeDe,
    #[serde(rename = "de-CH")]
    DeCh,
    #[serde(rename = "dsb")]
    Dsb,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "el-GR")]
    ElGr,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "en-AU")]
    EnAu,
    #[serde(rename = "en-CA")]
    EnCa,
    #[serde(rename = "en-IE")]
    EnIe,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "en-PI")]
    EnPi,
    #[serde(rename = "en-SG")]
    EnSg,
    #[serde(rename = "en-UD")]
    EnUd,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "en-ZA")]
    EnZa,
    #[serde(rename = "en@pirate")]
    EnAtPirate,
    #[serde(rename = "eo")]
    Eo,
    #[serde(rename = "eo-EO")]
    EoEo,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "es-AR")]
    EsAr,
    #[serde(rename = "es-419")]
    Es419,
    #[serde(rename = "es-CL")]
    EsCl,
    #[serde(rename = "es-CO")]
    EsCo,
    #[serde(rename = "es-EC")]
    EsEc,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "es-LA")]
    EsLa,
    #[serde(rename = "es-NI")]
    EsNi,
    #[serde(rename = "es-MX")]
    EsMx,
    #[serde(rename = "es-US")]
    EsUs,
    #[serde(rename = "es-VE")]
    EsVe,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "et-EE")]
    EtEe,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "eu-ES")]
    EuEs,
    #[serde(rename = "fa")]
    Fa,
    #[serde(rename = "fa-IR")]
    FaIr,
    #[serde(rename = "fb-LT")]
    FbLt,
    #[serde(rename = "ff")]
    Ff,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fi-FI")]
    FiFi,
    #[serde(rename = "fo")]
    Fo,
    #[serde(rename = "fo-FO")]
    FoFo,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "fr-CA")]
    FrCa,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "fr-BE")]
    FrBe,
    #[serde(rename = "fr-CH")]
    FrCh,
    #[serde(rename = "fy-NL")]
    FyNl,
    #[serde(rename = "ga")]
    Ga,
    #[serde(rename = "ga-IE")]
    GaIe,
    #[serde(rename = "gd")]
    Gd,
    #[serde(rename = "gl")]
    Gl,
    #[serde(rename = "gl-ES")]
    GlEs,
    #[serde(rename = "gn-PY")]
    GnPy,
    #[serde(rename = "gu-IN")]
    GuIn,
    #[serde(rename = "gv")]
    Gv,
    #[serde(rename = "gx-GR")]
    GxGr,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "he-IL")]
    HeIl,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hi-IN")]
    HiIn,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "hr-HR")]
    HrHr,
    #[serde(rename = "hsb")]
    Hsb,
    #[serde(rename = "ht")]
    Ht,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "hu-HU")]
    HuHu,
    #[serde(rename = "hy")]
    Hy,
    #[serde(rename = "hy-AM")]
    HyAm,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "id-ID")]
    IdId,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "is-IS")]
    IsIs,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "it-IT")]
    ItIt,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ja-JP")]
    JaJp,
    #[serde(rename = "jv-ID")]
    JvId,
    #[serde(rename = "ka-GE")]
    KaGe,
    #[serde(rename = "kk-KZ")]
    KkKz,
    #[serde(rename = "km")]
    Km,
    #[serde(rename = "kl")]
    Kl,
    #[serde(rename = "km-KH")]
    KmKh,
    #[serde(rename = "kab")]
    Kab,
    #[serde(rename = "kn")]
    Kn,
    #[serde(rename = "kn-IN")]
    KnIn,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "ko-KR")]
    KoKr,
    #[serde(rename = "ku-TR")]
    KuTr,
    #[serde(rename = "kw")]
    Kw,
    #[serde(rename = "la")]
    La,
    #[serde(rename = "la-VA")]
    LaVa,
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "li-NL")]
    LiNl,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lt-LT")]
    LtLt,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "lv-LV")]
    LvLv,
    #[serde(rename = "mai")]
    Mai,
    #[serde(rename = "mg-MG")]
    MgMg,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "mk-MK")]
    MkMk,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "ml-IN")]
    MlIn,
    #[serde(rename = "mn-MN")]
    MnMn,
    #[serde(rename = "mr")]
    Mr,
    #[serde(rename = "mr-IN")]
    MrIn,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "ms-MY")]
    MsMy,
    #[serde(rename = "mt")]
    Mt,
    #[serde(rename = "mt-MT")]
    MtMt,
    #[serde(rename = "my")]
    My,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nb")]
    Nb,
    #[serde(rename = "nb-NO")]
    NbNo,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "ne-NP")]
    NeNp,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "nl-BE")]
    NlBe,
    #[serde(rename = "nl-NL")]
    NlNl,
    #[serde(rename = "nn-NO")]
    NnNo,
    #[serde(rename = "oc")]
    Oc,
    #[serde(rename = "or-IN")]
    OrIn,
    #[serde(rename = "pa")]
    Pa,
    #[serde(rename = "pa-IN")]
    PaIn,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pl-PL")]
    PlPl,
    #[serde(rename = "ps-AF")]
    PsAf,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "pt-PT")]
    PtPt,
    #[serde(rename = "qu-PE")]
    QuPe,
    #[serde(rename = "rm-CH")]
    RmCh,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ro-RO")]
    RoRo,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "ru-RU")]
    RuRu,
    #[serde(rename = "sa-IN")]
    SaIn,
    #[serde(rename = "se-NO")]
    SeNo,
    #[serde(rename = "sh")]
    Sh,
    #[serde(rename = "si-LK")]
    SiLk,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sk-SK")]
    SkSk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sl-SI")]
    SlSi,
    #[serde(rename = "so-SO")]
    SoSo,
    #[serde(rename = "sq")]
    Sq,
    #[serde(rename = "sq-AL")]
    SqAl,
    #[serde(rename = "sr")]
    Sr,
    #[serde(rename = "sr-RS")]
    SrRs,
    #[serde(rename = "su")]
    Su,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "sv-SE")]
    SvSe,
    #[serde(rename = "sw")]
    Sw,
    #[serde(rename = "sw-KE")]
    SwKe,
    #[serde(rename = "ta")]
    Ta,
    #[serde(rename = "ta-IN")]
    TaIn,
    #[serde(rename = "te")]
    Te,
    #[serde(rename = "te-IN")]
    TeIn,
    #[serde(rename = "tg")]
    Tg,
    #[serde(rename = "tg-TJ")]
    TgTj,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "th-TH")]
    ThTh,
    #[serde(rename = "fil")]
    Fil,
    #[serde(rename = "tlh")]
    Tlh,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "tr-TR")]
    TrTr,
    #[serde(rename = "tt-RU")]
    TtRu,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "uk-UA")]
    UkUa,
    #[serde(rename = "ur")]
    Ur,
    #[serde(rename = "ur-PK")]
    UrPk,
    #[serde(rename = "uz")]
    Uz,
    #[serde(rename = "uz-UZ")]
    UzUz,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "vi-VN")]
    ViVn,
    #[serde(rename = "xh-ZA")]
    XhZa,
    #[serde(rename = "yi")]
    Yi,
    #[serde(rename = "yi-DE")]
    YiDe,
    #[serde(rename = "zh")]
    Zh,
    #[serde(rename = "zh-Hans")]
    ZhHans,
    #[serde(rename = "zh-Hant")]
    ZhHant,
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "zh-HK")]
    ZhHk,
    #[serde(rename = "zh-SG")]
    ZhSg,
    #[serde(rename = "zh-TW")]
    ZhTw,
    #[serde(rename = "zu-ZA")]
    ZuZa,
}

impl Default for Lang {
    fn default() -> Lang {
        Self::Ach
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FollowingVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "followers")]
    Followers,
    #[serde(rename = "private")]
    Private,
}

impl Default for FollowingVisibility {
    fn default() -> FollowingVisibility {
        Self::Public
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FollowersVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "followers")]
    Followers,
    #[serde(rename = "private")]
    Private,
}

impl Default for FollowersVisibility {
    fn default() -> FollowersVisibility {
        Self::Public
    }
}
