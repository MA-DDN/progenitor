#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///`Alert`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "labels"
    ///  ],
    ///  "properties": {
    ///    "generatorURL": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    },
    ///    "labels": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Alert {
        #[serde(
            rename = "generatorURL",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub generator_url: ::std::option::Option<::std::string::String>,
        pub labels: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
    }
    impl ::std::convert::From<&Alert> for Alert {
        fn from(value: &Alert) -> Self {
            value.clone()
        }
    }
    ///`AlertGroup`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "alerts",
    ///    "labels",
    ///    "receiver"
    ///  ],
    ///  "properties": {
    ///    "alerts": {
    ///      "type": "array",
    ///      "items": {
    ///        "allOf": [
    ///          {
    ///            "type": "object",
    ///            "required": [
    ///              "annotations",
    ///              "endsAt",
    ///              "fingerprint",
    ///              "receivers",
    ///              "startsAt",
    ///              "status",
    ///              "updatedAt"
    ///            ],
    ///            "properties": {
    ///              "annotations": {
    ///                "$ref": "#/components/schemas/labelSet"
    ///              },
    ///              "endsAt": {
    ///                "type": "string",
    ///                "format": "date-time"
    ///              },
    ///              "fingerprint": {
    ///                "type": "string"
    ///              },
    ///              "receivers": {
    ///                "type": "array",
    ///                "items": {
    ///                  "$ref": "#/components/schemas/receiver"
    ///                }
    ///              },
    ///              "startsAt": {
    ///                "type": "string",
    ///                "format": "date-time"
    ///              },
    ///              "status": {
    ///                "$ref": "#/components/schemas/alertStatus"
    ///              },
    ///              "updatedAt": {
    ///                "type": "string",
    ///                "format": "date-time"
    ///              }
    ///            }
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/alert"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "labels": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "receiver": {
    ///      "type": "object",
    ///      "required": [
    ///        "name"
    ///      ],
    ///      "properties": {
    ///        "name": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlertGroup {
        pub alerts: ::std::vec::Vec<AlertGroupAlertsItem>,
        pub labels: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        pub receiver: AlertGroupReceiver,
    }
    impl ::std::convert::From<&AlertGroup> for AlertGroup {
        fn from(value: &AlertGroup) -> Self {
            value.clone()
        }
    }
    ///`AlertGroupAlertsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "annotations",
    ///        "endsAt",
    ///        "fingerprint",
    ///        "receivers",
    ///        "startsAt",
    ///        "status",
    ///        "updatedAt"
    ///      ],
    ///      "properties": {
    ///        "annotations": {
    ///          "$ref": "#/components/schemas/labelSet"
    ///        },
    ///        "endsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "fingerprint": {
    ///          "type": "string"
    ///        },
    ///        "receivers": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/receiver"
    ///          }
    ///        },
    ///        "startsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "status": {
    ///          "$ref": "#/components/schemas/alertStatus"
    ///        },
    ///        "updatedAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/alert"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlertGroupAlertsItem {
        pub annotations: LabelSet,
        #[serde(rename = "endsAt")]
        pub ends_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub fingerprint: ::std::string::String,
        #[serde(
            rename = "generatorURL",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub generator_url: ::std::option::Option<::std::string::String>,
        pub labels: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        pub receivers: ::std::vec::Vec<Receiver>,
        #[serde(rename = "startsAt")]
        pub starts_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub status: AlertStatus,
        #[serde(rename = "updatedAt")]
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&AlertGroupAlertsItem> for AlertGroupAlertsItem {
        fn from(value: &AlertGroupAlertsItem) -> Self {
            value.clone()
        }
    }
    ///`AlertGroupReceiver`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlertGroupReceiver {
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&AlertGroupReceiver> for AlertGroupReceiver {
        fn from(value: &AlertGroupReceiver) -> Self {
            value.clone()
        }
    }
    ///`AlertGroups`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "type": "object",
    ///    "required": [
    ///      "alerts",
    ///      "labels",
    ///      "receiver"
    ///    ],
    ///    "properties": {
    ///      "alerts": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/gettableAlert"
    ///        }
    ///      },
    ///      "labels": {
    ///        "$ref": "#/components/schemas/labelSet"
    ///      },
    ///      "receiver": {
    ///        "$ref": "#/components/schemas/receiver"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct AlertGroups(pub ::std::vec::Vec<AlertGroupsItem>);
    impl ::std::ops::Deref for AlertGroups {
        type Target = ::std::vec::Vec<AlertGroupsItem>;
        fn deref(&self) -> &::std::vec::Vec<AlertGroupsItem> {
            &self.0
        }
    }
    impl ::std::convert::From<AlertGroups> for ::std::vec::Vec<AlertGroupsItem> {
        fn from(value: AlertGroups) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&AlertGroups> for AlertGroups {
        fn from(value: &AlertGroups) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::std::vec::Vec<AlertGroupsItem>> for AlertGroups {
        fn from(value: ::std::vec::Vec<AlertGroupsItem>) -> Self {
            Self(value)
        }
    }
    ///`AlertGroupsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "alerts",
    ///    "labels",
    ///    "receiver"
    ///  ],
    ///  "properties": {
    ///    "alerts": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/gettableAlert"
    ///      }
    ///    },
    ///    "labels": {
    ///      "$ref": "#/components/schemas/labelSet"
    ///    },
    ///    "receiver": {
    ///      "$ref": "#/components/schemas/receiver"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlertGroupsItem {
        pub alerts: ::std::vec::Vec<GettableAlert>,
        pub labels: LabelSet,
        pub receiver: Receiver,
    }
    impl ::std::convert::From<&AlertGroupsItem> for AlertGroupsItem {
        fn from(value: &AlertGroupsItem) -> Self {
            value.clone()
        }
    }
    ///`AlertStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "inhibitedBy",
    ///    "silencedBy",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "inhibitedBy": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "silencedBy": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "state": {
    ///      "type": "string",
    ///      "enum": [
    ///        "unprocessed",
    ///        "active",
    ///        "suppressed"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlertStatus {
        #[serde(rename = "inhibitedBy")]
        pub inhibited_by: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "silencedBy")]
        pub silenced_by: ::std::vec::Vec<::std::string::String>,
        pub state: AlertStatusState,
    }
    impl ::std::convert::From<&AlertStatus> for AlertStatus {
        fn from(value: &AlertStatus) -> Self {
            value.clone()
        }
    }
    ///`AlertStatusState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "unprocessed",
    ///    "active",
    ///    "suppressed"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum AlertStatusState {
        #[serde(rename = "unprocessed")]
        Unprocessed,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "suppressed")]
        Suppressed,
    }
    impl ::std::convert::From<&Self> for AlertStatusState {
        fn from(value: &AlertStatusState) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for AlertStatusState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Unprocessed => write!(f, "unprocessed"),
                Self::Active => write!(f, "active"),
                Self::Suppressed => write!(f, "suppressed"),
            }
        }
    }
    impl ::std::str::FromStr for AlertStatusState {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "unprocessed" => Ok(Self::Unprocessed),
                "active" => Ok(Self::Active),
                "suppressed" => Ok(Self::Suppressed),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for AlertStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for AlertStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for AlertStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`AlertmanagerConfig`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "original"
    ///  ],
    ///  "properties": {
    ///    "original": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlertmanagerConfig {
        pub original: ::std::string::String,
    }
    impl ::std::convert::From<&AlertmanagerConfig> for AlertmanagerConfig {
        fn from(value: &AlertmanagerConfig) -> Self {
            value.clone()
        }
    }
    ///`AlertmanagerStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "cluster",
    ///    "config",
    ///    "uptime",
    ///    "versionInfo"
    ///  ],
    ///  "properties": {
    ///    "cluster": {
    ///      "type": "object",
    ///      "required": [
    ///        "status"
    ///      ],
    ///      "properties": {
    ///        "name": {
    ///          "type": "string"
    ///        },
    ///        "peers": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/peerStatus"
    ///          }
    ///        },
    ///        "status": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ready",
    ///            "settling",
    ///            "disabled"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "config": {
    ///      "type": "object",
    ///      "required": [
    ///        "original"
    ///      ],
    ///      "properties": {
    ///        "original": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "uptime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "versionInfo": {
    ///      "type": "object",
    ///      "required": [
    ///        "branch",
    ///        "buildDate",
    ///        "buildUser",
    ///        "goVersion",
    ///        "revision",
    ///        "version"
    ///      ],
    ///      "properties": {
    ///        "branch": {
    ///          "type": "string"
    ///        },
    ///        "buildDate": {
    ///          "type": "string"
    ///        },
    ///        "buildUser": {
    ///          "type": "string"
    ///        },
    ///        "goVersion": {
    ///          "type": "string"
    ///        },
    ///        "revision": {
    ///          "type": "string"
    ///        },
    ///        "version": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlertmanagerStatus {
        pub cluster: AlertmanagerStatusCluster,
        pub config: AlertmanagerStatusConfig,
        pub uptime: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(rename = "versionInfo")]
        pub version_info: AlertmanagerStatusVersionInfo,
    }
    impl ::std::convert::From<&AlertmanagerStatus> for AlertmanagerStatus {
        fn from(value: &AlertmanagerStatus) -> Self {
            value.clone()
        }
    }
    ///`AlertmanagerStatusCluster`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "peers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/peerStatus"
    ///      }
    ///    },
    ///    "status": {
    ///      "type": "string",
    ///      "enum": [
    ///        "ready",
    ///        "settling",
    ///        "disabled"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlertmanagerStatusCluster {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub peers: ::std::vec::Vec<PeerStatus>,
        pub status: AlertmanagerStatusClusterStatus,
    }
    impl ::std::convert::From<&AlertmanagerStatusCluster> for AlertmanagerStatusCluster {
        fn from(value: &AlertmanagerStatusCluster) -> Self {
            value.clone()
        }
    }
    ///`AlertmanagerStatusClusterStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ready",
    ///    "settling",
    ///    "disabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum AlertmanagerStatusClusterStatus {
        #[serde(rename = "ready")]
        Ready,
        #[serde(rename = "settling")]
        Settling,
        #[serde(rename = "disabled")]
        Disabled,
    }
    impl ::std::convert::From<&Self> for AlertmanagerStatusClusterStatus {
        fn from(value: &AlertmanagerStatusClusterStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for AlertmanagerStatusClusterStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ready => write!(f, "ready"),
                Self::Settling => write!(f, "settling"),
                Self::Disabled => write!(f, "disabled"),
            }
        }
    }
    impl ::std::str::FromStr for AlertmanagerStatusClusterStatus {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ready" => Ok(Self::Ready),
                "settling" => Ok(Self::Settling),
                "disabled" => Ok(Self::Disabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for AlertmanagerStatusClusterStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for AlertmanagerStatusClusterStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for AlertmanagerStatusClusterStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`AlertmanagerStatusConfig`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "original"
    ///  ],
    ///  "properties": {
    ///    "original": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlertmanagerStatusConfig {
        pub original: ::std::string::String,
    }
    impl ::std::convert::From<&AlertmanagerStatusConfig> for AlertmanagerStatusConfig {
        fn from(value: &AlertmanagerStatusConfig) -> Self {
            value.clone()
        }
    }
    ///`AlertmanagerStatusVersionInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "branch",
    ///    "buildDate",
    ///    "buildUser",
    ///    "goVersion",
    ///    "revision",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "branch": {
    ///      "type": "string"
    ///    },
    ///    "buildDate": {
    ///      "type": "string"
    ///    },
    ///    "buildUser": {
    ///      "type": "string"
    ///    },
    ///    "goVersion": {
    ///      "type": "string"
    ///    },
    ///    "revision": {
    ///      "type": "string"
    ///    },
    ///    "version": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AlertmanagerStatusVersionInfo {
        pub branch: ::std::string::String,
        #[serde(rename = "buildDate")]
        pub build_date: ::std::string::String,
        #[serde(rename = "buildUser")]
        pub build_user: ::std::string::String,
        #[serde(rename = "goVersion")]
        pub go_version: ::std::string::String,
        pub revision: ::std::string::String,
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&AlertmanagerStatusVersionInfo>
    for AlertmanagerStatusVersionInfo {
        fn from(value: &AlertmanagerStatusVersionInfo) -> Self {
            value.clone()
        }
    }
    ///`ClusterStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "peers": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "address",
    ///          "name"
    ///        ],
    ///        "properties": {
    ///          "address": {
    ///            "type": "string"
    ///          },
    ///          "name": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "status": {
    ///      "type": "string",
    ///      "enum": [
    ///        "ready",
    ///        "settling",
    ///        "disabled"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ClusterStatus {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub peers: ::std::vec::Vec<ClusterStatusPeersItem>,
        pub status: ClusterStatusStatus,
    }
    impl ::std::convert::From<&ClusterStatus> for ClusterStatus {
        fn from(value: &ClusterStatus) -> Self {
            value.clone()
        }
    }
    ///`ClusterStatusPeersItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "address",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "address": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ClusterStatusPeersItem {
        pub address: ::std::string::String,
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&ClusterStatusPeersItem> for ClusterStatusPeersItem {
        fn from(value: &ClusterStatusPeersItem) -> Self {
            value.clone()
        }
    }
    ///`ClusterStatusStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ready",
    ///    "settling",
    ///    "disabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum ClusterStatusStatus {
        #[serde(rename = "ready")]
        Ready,
        #[serde(rename = "settling")]
        Settling,
        #[serde(rename = "disabled")]
        Disabled,
    }
    impl ::std::convert::From<&Self> for ClusterStatusStatus {
        fn from(value: &ClusterStatusStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for ClusterStatusStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ready => write!(f, "ready"),
                Self::Settling => write!(f, "settling"),
                Self::Disabled => write!(f, "disabled"),
            }
        }
    }
    impl ::std::str::FromStr for ClusterStatusStatus {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ready" => Ok(Self::Ready),
                "settling" => Ok(Self::Settling),
                "disabled" => Ok(Self::Disabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ClusterStatusStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for ClusterStatusStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ClusterStatusStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetReceiversResponseItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReceiversResponseItem {
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&GetReceiversResponseItem> for GetReceiversResponseItem {
        fn from(value: &GetReceiversResponseItem) -> Self {
            value.clone()
        }
    }
    ///`GetSilenceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "status",
    ///        "updatedAt"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "status": {
    ///          "$ref": "#/components/schemas/silenceStatus"
    ///        },
    ///        "updatedAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/silence"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSilenceResponse {
        pub comment: ::std::string::String,
        #[serde(rename = "createdBy")]
        pub created_by: ::std::string::String,
        #[serde(rename = "endsAt")]
        pub ends_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: ::std::string::String,
        pub matchers: ::std::vec::Vec<Matcher>,
        #[serde(rename = "startsAt")]
        pub starts_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub status: SilenceStatus,
        #[serde(rename = "updatedAt")]
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&GetSilenceResponse> for GetSilenceResponse {
        fn from(value: &GetSilenceResponse) -> Self {
            value.clone()
        }
    }
    ///`GetStatusResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "cluster",
    ///    "config",
    ///    "uptime",
    ///    "versionInfo"
    ///  ],
    ///  "properties": {
    ///    "cluster": {
    ///      "$ref": "#/components/schemas/clusterStatus"
    ///    },
    ///    "config": {
    ///      "$ref": "#/components/schemas/alertmanagerConfig"
    ///    },
    ///    "uptime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "versionInfo": {
    ///      "$ref": "#/components/schemas/versionInfo"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetStatusResponse {
        pub cluster: ClusterStatus,
        pub config: AlertmanagerConfig,
        pub uptime: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(rename = "versionInfo")]
        pub version_info: VersionInfo,
    }
    impl ::std::convert::From<&GetStatusResponse> for GetStatusResponse {
        fn from(value: &GetStatusResponse) -> Self {
            value.clone()
        }
    }
    ///`GettableAlert`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "annotations",
    ///        "endsAt",
    ///        "fingerprint",
    ///        "receivers",
    ///        "startsAt",
    ///        "status",
    ///        "updatedAt"
    ///      ],
    ///      "properties": {
    ///        "annotations": {
    ///          "type": "object",
    ///          "additionalProperties": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "endsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "fingerprint": {
    ///          "type": "string"
    ///        },
    ///        "receivers": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "object",
    ///            "required": [
    ///              "name"
    ///            ],
    ///            "properties": {
    ///              "name": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "startsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "status": {
    ///          "type": "object",
    ///          "required": [
    ///            "inhibitedBy",
    ///            "silencedBy",
    ///            "state"
    ///          ],
    ///          "properties": {
    ///            "inhibitedBy": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "string"
    ///              }
    ///            },
    ///            "silencedBy": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "string"
    ///              }
    ///            },
    ///            "state": {
    ///              "type": "string",
    ///              "enum": [
    ///                "unprocessed",
    ///                "active",
    ///                "suppressed"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        "updatedAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "labels"
    ///      ],
    ///      "properties": {
    ///        "generatorURL": {
    ///          "type": "string",
    ///          "format": "uri"
    ///        },
    ///        "labels": {
    ///          "$ref": "#/components/schemas/labelSet"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GettableAlert {
        pub annotations: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        #[serde(rename = "endsAt")]
        pub ends_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub fingerprint: ::std::string::String,
        #[serde(
            rename = "generatorURL",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub generator_url: ::std::option::Option<::std::string::String>,
        pub labels: LabelSet,
        pub receivers: ::std::vec::Vec<GettableAlertReceiversItem>,
        #[serde(rename = "startsAt")]
        pub starts_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub status: GettableAlertStatus,
        #[serde(rename = "updatedAt")]
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&GettableAlert> for GettableAlert {
        fn from(value: &GettableAlert) -> Self {
            value.clone()
        }
    }
    ///`GettableAlertReceiversItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GettableAlertReceiversItem {
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&GettableAlertReceiversItem>
    for GettableAlertReceiversItem {
        fn from(value: &GettableAlertReceiversItem) -> Self {
            value.clone()
        }
    }
    ///`GettableAlertStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "inhibitedBy",
    ///    "silencedBy",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "inhibitedBy": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "silencedBy": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "state": {
    ///      "type": "string",
    ///      "enum": [
    ///        "unprocessed",
    ///        "active",
    ///        "suppressed"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GettableAlertStatus {
        #[serde(rename = "inhibitedBy")]
        pub inhibited_by: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "silencedBy")]
        pub silenced_by: ::std::vec::Vec<::std::string::String>,
        pub state: GettableAlertStatusState,
    }
    impl ::std::convert::From<&GettableAlertStatus> for GettableAlertStatus {
        fn from(value: &GettableAlertStatus) -> Self {
            value.clone()
        }
    }
    ///`GettableAlertStatusState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "unprocessed",
    ///    "active",
    ///    "suppressed"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum GettableAlertStatusState {
        #[serde(rename = "unprocessed")]
        Unprocessed,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "suppressed")]
        Suppressed,
    }
    impl ::std::convert::From<&Self> for GettableAlertStatusState {
        fn from(value: &GettableAlertStatusState) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GettableAlertStatusState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Unprocessed => write!(f, "unprocessed"),
                Self::Active => write!(f, "active"),
                Self::Suppressed => write!(f, "suppressed"),
            }
        }
    }
    impl ::std::str::FromStr for GettableAlertStatusState {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "unprocessed" => Ok(Self::Unprocessed),
                "active" => Ok(Self::Active),
                "suppressed" => Ok(Self::Suppressed),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GettableAlertStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GettableAlertStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GettableAlertStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GettableAlerts`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "allOf": [
    ///      {
    ///        "type": "object",
    ///        "required": [
    ///          "annotations",
    ///          "endsAt",
    ///          "fingerprint",
    ///          "receivers",
    ///          "startsAt",
    ///          "status",
    ///          "updatedAt"
    ///        ],
    ///        "properties": {
    ///          "annotations": {
    ///            "$ref": "#/components/schemas/labelSet"
    ///          },
    ///          "endsAt": {
    ///            "type": "string",
    ///            "format": "date-time"
    ///          },
    ///          "fingerprint": {
    ///            "type": "string"
    ///          },
    ///          "receivers": {
    ///            "type": "array",
    ///            "items": {
    ///              "$ref": "#/components/schemas/receiver"
    ///            }
    ///          },
    ///          "startsAt": {
    ///            "type": "string",
    ///            "format": "date-time"
    ///          },
    ///          "status": {
    ///            "$ref": "#/components/schemas/alertStatus"
    ///          },
    ///          "updatedAt": {
    ///            "type": "string",
    ///            "format": "date-time"
    ///          }
    ///        }
    ///      },
    ///      {
    ///        "$ref": "#/components/schemas/alert"
    ///      }
    ///    ]
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct GettableAlerts(pub ::std::vec::Vec<GettableAlertsItem>);
    impl ::std::ops::Deref for GettableAlerts {
        type Target = ::std::vec::Vec<GettableAlertsItem>;
        fn deref(&self) -> &::std::vec::Vec<GettableAlertsItem> {
            &self.0
        }
    }
    impl ::std::convert::From<GettableAlerts> for ::std::vec::Vec<GettableAlertsItem> {
        fn from(value: GettableAlerts) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&GettableAlerts> for GettableAlerts {
        fn from(value: &GettableAlerts) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::std::vec::Vec<GettableAlertsItem>> for GettableAlerts {
        fn from(value: ::std::vec::Vec<GettableAlertsItem>) -> Self {
            Self(value)
        }
    }
    ///`GettableAlertsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "annotations",
    ///        "endsAt",
    ///        "fingerprint",
    ///        "receivers",
    ///        "startsAt",
    ///        "status",
    ///        "updatedAt"
    ///      ],
    ///      "properties": {
    ///        "annotations": {
    ///          "$ref": "#/components/schemas/labelSet"
    ///        },
    ///        "endsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "fingerprint": {
    ///          "type": "string"
    ///        },
    ///        "receivers": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/receiver"
    ///          }
    ///        },
    ///        "startsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "status": {
    ///          "$ref": "#/components/schemas/alertStatus"
    ///        },
    ///        "updatedAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/alert"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GettableAlertsItem {
        pub annotations: LabelSet,
        #[serde(rename = "endsAt")]
        pub ends_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub fingerprint: ::std::string::String,
        #[serde(
            rename = "generatorURL",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub generator_url: ::std::option::Option<::std::string::String>,
        pub labels: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        pub receivers: ::std::vec::Vec<Receiver>,
        #[serde(rename = "startsAt")]
        pub starts_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub status: AlertStatus,
        #[serde(rename = "updatedAt")]
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&GettableAlertsItem> for GettableAlertsItem {
        fn from(value: &GettableAlertsItem) -> Self {
            value.clone()
        }
    }
    ///`GettableSilence`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "status",
    ///        "updatedAt"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "status": {
    ///          "type": "object",
    ///          "required": [
    ///            "state"
    ///          ],
    ///          "properties": {
    ///            "state": {
    ///              "type": "string",
    ///              "enum": [
    ///                "expired",
    ///                "active",
    ///                "pending"
    ///              ]
    ///            }
    ///          }
    ///        },
    ///        "updatedAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "comment",
    ///        "createdBy",
    ///        "endsAt",
    ///        "matchers",
    ///        "startsAt"
    ///      ],
    ///      "properties": {
    ///        "comment": {
    ///          "type": "string"
    ///        },
    ///        "createdBy": {
    ///          "type": "string"
    ///        },
    ///        "endsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "matchers": {
    ///          "$ref": "#/components/schemas/matchers"
    ///        },
    ///        "startsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GettableSilence {
        pub comment: ::std::string::String,
        #[serde(rename = "createdBy")]
        pub created_by: ::std::string::String,
        #[serde(rename = "endsAt")]
        pub ends_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: ::std::string::String,
        pub matchers: Matchers,
        #[serde(rename = "startsAt")]
        pub starts_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub status: GettableSilenceStatus,
        #[serde(rename = "updatedAt")]
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&GettableSilence> for GettableSilence {
        fn from(value: &GettableSilence) -> Self {
            value.clone()
        }
    }
    ///`GettableSilenceStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "state": {
    ///      "type": "string",
    ///      "enum": [
    ///        "expired",
    ///        "active",
    ///        "pending"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GettableSilenceStatus {
        pub state: GettableSilenceStatusState,
    }
    impl ::std::convert::From<&GettableSilenceStatus> for GettableSilenceStatus {
        fn from(value: &GettableSilenceStatus) -> Self {
            value.clone()
        }
    }
    ///`GettableSilenceStatusState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "expired",
    ///    "active",
    ///    "pending"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum GettableSilenceStatusState {
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "pending")]
        Pending,
    }
    impl ::std::convert::From<&Self> for GettableSilenceStatusState {
        fn from(value: &GettableSilenceStatusState) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GettableSilenceStatusState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Expired => write!(f, "expired"),
                Self::Active => write!(f, "active"),
                Self::Pending => write!(f, "pending"),
            }
        }
    }
    impl ::std::str::FromStr for GettableSilenceStatusState {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "expired" => Ok(Self::Expired),
                "active" => Ok(Self::Active),
                "pending" => Ok(Self::Pending),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GettableSilenceStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GettableSilenceStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GettableSilenceStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GettableSilences`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "allOf": [
    ///      {
    ///        "type": "object",
    ///        "required": [
    ///          "id",
    ///          "status",
    ///          "updatedAt"
    ///        ],
    ///        "properties": {
    ///          "id": {
    ///            "type": "string"
    ///          },
    ///          "status": {
    ///            "$ref": "#/components/schemas/silenceStatus"
    ///          },
    ///          "updatedAt": {
    ///            "type": "string",
    ///            "format": "date-time"
    ///          }
    ///        }
    ///      },
    ///      {
    ///        "$ref": "#/components/schemas/silence"
    ///      }
    ///    ]
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct GettableSilences(pub ::std::vec::Vec<GettableSilencesItem>);
    impl ::std::ops::Deref for GettableSilences {
        type Target = ::std::vec::Vec<GettableSilencesItem>;
        fn deref(&self) -> &::std::vec::Vec<GettableSilencesItem> {
            &self.0
        }
    }
    impl ::std::convert::From<GettableSilences>
    for ::std::vec::Vec<GettableSilencesItem> {
        fn from(value: GettableSilences) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&GettableSilences> for GettableSilences {
        fn from(value: &GettableSilences) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::std::vec::Vec<GettableSilencesItem>>
    for GettableSilences {
        fn from(value: ::std::vec::Vec<GettableSilencesItem>) -> Self {
            Self(value)
        }
    }
    ///`GettableSilencesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "id",
    ///        "status",
    ///        "updatedAt"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        },
    ///        "status": {
    ///          "$ref": "#/components/schemas/silenceStatus"
    ///        },
    ///        "updatedAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/silence"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GettableSilencesItem {
        pub comment: ::std::string::String,
        #[serde(rename = "createdBy")]
        pub created_by: ::std::string::String,
        #[serde(rename = "endsAt")]
        pub ends_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: ::std::string::String,
        pub matchers: ::std::vec::Vec<Matcher>,
        #[serde(rename = "startsAt")]
        pub starts_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub status: SilenceStatus,
        #[serde(rename = "updatedAt")]
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&GettableSilencesItem> for GettableSilencesItem {
        fn from(value: &GettableSilencesItem) -> Self {
            value.clone()
        }
    }
    ///`LabelSet`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "type": "string"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct LabelSet(
        pub ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    );
    impl ::std::ops::Deref for LabelSet {
        type Target = ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >;
        fn deref(
            &self,
        ) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
            &self.0
        }
    }
    impl ::std::convert::From<LabelSet>
    for ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        fn from(value: LabelSet) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&LabelSet> for LabelSet {
        fn from(value: &LabelSet) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > for LabelSet {
        fn from(
            value: ::std::collections::HashMap<
                ::std::string::String,
                ::std::string::String,
            >,
        ) -> Self {
            Self(value)
        }
    }
    ///`Matcher`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "isRegex",
    ///    "name",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "isEqual": {
    ///      "default": true,
    ///      "type": "boolean"
    ///    },
    ///    "isRegex": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Matcher {
        #[serde(rename = "isEqual", default = "defaults::default_bool::<true>")]
        pub is_equal: bool,
        #[serde(rename = "isRegex")]
        pub is_regex: bool,
        pub name: ::std::string::String,
        pub value: ::std::string::String,
    }
    impl ::std::convert::From<&Matcher> for Matcher {
        fn from(value: &Matcher) -> Self {
            value.clone()
        }
    }
    ///`Matchers`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "type": "object",
    ///    "required": [
    ///      "isRegex",
    ///      "name",
    ///      "value"
    ///    ],
    ///    "properties": {
    ///      "isEqual": {
    ///        "default": true,
    ///        "type": "boolean"
    ///      },
    ///      "isRegex": {
    ///        "type": "boolean"
    ///      },
    ///      "name": {
    ///        "type": "string"
    ///      },
    ///      "value": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  },
    ///  "minItems": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct Matchers(pub ::std::vec::Vec<MatchersItem>);
    impl ::std::ops::Deref for Matchers {
        type Target = ::std::vec::Vec<MatchersItem>;
        fn deref(&self) -> &::std::vec::Vec<MatchersItem> {
            &self.0
        }
    }
    impl ::std::convert::From<Matchers> for ::std::vec::Vec<MatchersItem> {
        fn from(value: Matchers) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&Matchers> for Matchers {
        fn from(value: &Matchers) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::std::vec::Vec<MatchersItem>> for Matchers {
        fn from(value: ::std::vec::Vec<MatchersItem>) -> Self {
            Self(value)
        }
    }
    ///`MatchersItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "isRegex",
    ///    "name",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "isEqual": {
    ///      "default": true,
    ///      "type": "boolean"
    ///    },
    ///    "isRegex": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MatchersItem {
        #[serde(rename = "isEqual", default = "defaults::default_bool::<true>")]
        pub is_equal: bool,
        #[serde(rename = "isRegex")]
        pub is_regex: bool,
        pub name: ::std::string::String,
        pub value: ::std::string::String,
    }
    impl ::std::convert::From<&MatchersItem> for MatchersItem {
        fn from(value: &MatchersItem) -> Self {
            value.clone()
        }
    }
    ///`PeerStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "address",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "address": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PeerStatus {
        pub address: ::std::string::String,
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&PeerStatus> for PeerStatus {
        fn from(value: &PeerStatus) -> Self {
            value.clone()
        }
    }
    ///`PostAlertsBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "alerts"
    ///  ],
    ///  "properties": {
    ///    "alerts": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/postableAlert"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostAlertsBody {
        pub alerts: ::std::vec::Vec<PostableAlert>,
    }
    impl ::std::convert::From<&PostAlertsBody> for PostAlertsBody {
        fn from(value: &PostAlertsBody) -> Self {
            value.clone()
        }
    }
    ///`PostSilencesBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "silence"
    ///  ],
    ///  "properties": {
    ///    "silence": {
    ///      "allOf": [
    ///        {
    ///          "type": "object",
    ///          "properties": {
    ///            "id": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/silence"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostSilencesBody {
        pub silence: PostSilencesBodySilence,
    }
    impl ::std::convert::From<&PostSilencesBody> for PostSilencesBody {
        fn from(value: &PostSilencesBody) -> Self {
            value.clone()
        }
    }
    ///`PostSilencesBodySilence`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/silence"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostSilencesBodySilence {
        pub comment: ::std::string::String,
        #[serde(rename = "createdBy")]
        pub created_by: ::std::string::String,
        #[serde(rename = "endsAt")]
        pub ends_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        pub matchers: ::std::vec::Vec<Matcher>,
        #[serde(rename = "startsAt")]
        pub starts_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&PostSilencesBodySilence> for PostSilencesBodySilence {
        fn from(value: &PostSilencesBodySilence) -> Self {
            value.clone()
        }
    }
    ///`PostSilencesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "silenceID": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostSilencesResponse {
        #[serde(
            rename = "silenceID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub silence_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&PostSilencesResponse> for PostSilencesResponse {
        fn from(value: &PostSilencesResponse) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PostSilencesResponse {
        fn default() -> Self {
            Self {
                silence_id: Default::default(),
            }
        }
    }
    ///`PostableAlert`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "annotations": {
    ///          "type": "object",
    ///          "additionalProperties": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "endsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "startsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "labels"
    ///      ],
    ///      "properties": {
    ///        "generatorURL": {
    ///          "type": "string",
    ///          "format": "uri"
    ///        },
    ///        "labels": {
    ///          "$ref": "#/components/schemas/labelSet"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostableAlert {
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub annotations: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        #[serde(
            rename = "endsAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ends_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "generatorURL",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub generator_url: ::std::option::Option<::std::string::String>,
        pub labels: LabelSet,
        #[serde(
            rename = "startsAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub starts_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    }
    impl ::std::convert::From<&PostableAlert> for PostableAlert {
        fn from(value: &PostableAlert) -> Self {
            value.clone()
        }
    }
    ///`PostableAlerts`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "allOf": [
    ///      {
    ///        "type": "object",
    ///        "properties": {
    ///          "annotations": {
    ///            "$ref": "#/components/schemas/labelSet"
    ///          },
    ///          "endsAt": {
    ///            "type": "string",
    ///            "format": "date-time"
    ///          },
    ///          "startsAt": {
    ///            "type": "string",
    ///            "format": "date-time"
    ///          }
    ///        }
    ///      },
    ///      {
    ///        "$ref": "#/components/schemas/alert"
    ///      }
    ///    ]
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct PostableAlerts(pub ::std::vec::Vec<PostableAlertsItem>);
    impl ::std::ops::Deref for PostableAlerts {
        type Target = ::std::vec::Vec<PostableAlertsItem>;
        fn deref(&self) -> &::std::vec::Vec<PostableAlertsItem> {
            &self.0
        }
    }
    impl ::std::convert::From<PostableAlerts> for ::std::vec::Vec<PostableAlertsItem> {
        fn from(value: PostableAlerts) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&PostableAlerts> for PostableAlerts {
        fn from(value: &PostableAlerts) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::std::vec::Vec<PostableAlertsItem>> for PostableAlerts {
        fn from(value: ::std::vec::Vec<PostableAlertsItem>) -> Self {
            Self(value)
        }
    }
    ///`PostableAlertsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "annotations": {
    ///          "$ref": "#/components/schemas/labelSet"
    ///        },
    ///        "endsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "startsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/alert"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostableAlertsItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub annotations: ::std::option::Option<LabelSet>,
        #[serde(
            rename = "endsAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ends_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "generatorURL",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub generator_url: ::std::option::Option<::std::string::String>,
        pub labels: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        #[serde(
            rename = "startsAt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub starts_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    }
    impl ::std::convert::From<&PostableAlertsItem> for PostableAlertsItem {
        fn from(value: &PostableAlertsItem) -> Self {
            value.clone()
        }
    }
    ///`PostableSilence`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "id": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "comment",
    ///        "createdBy",
    ///        "endsAt",
    ///        "matchers",
    ///        "startsAt"
    ///      ],
    ///      "properties": {
    ///        "comment": {
    ///          "type": "string"
    ///        },
    ///        "createdBy": {
    ///          "type": "string"
    ///        },
    ///        "endsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "matchers": {
    ///          "$ref": "#/components/schemas/matchers"
    ///        },
    ///        "startsAt": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostableSilence {
        pub comment: ::std::string::String,
        #[serde(rename = "createdBy")]
        pub created_by: ::std::string::String,
        #[serde(rename = "endsAt")]
        pub ends_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        pub matchers: Matchers,
        #[serde(rename = "startsAt")]
        pub starts_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&PostableSilence> for PostableSilence {
        fn from(value: &PostableSilence) -> Self {
            value.clone()
        }
    }
    ///`Receiver`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Receiver {
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&Receiver> for Receiver {
        fn from(value: &Receiver) -> Self {
            value.clone()
        }
    }
    ///`Silence`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "comment",
    ///    "createdBy",
    ///    "endsAt",
    ///    "matchers",
    ///    "startsAt"
    ///  ],
    ///  "properties": {
    ///    "comment": {
    ///      "type": "string"
    ///    },
    ///    "createdBy": {
    ///      "type": "string"
    ///    },
    ///    "endsAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "matchers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/matcher"
    ///      },
    ///      "minItems": 1
    ///    },
    ///    "startsAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Silence {
        pub comment: ::std::string::String,
        #[serde(rename = "createdBy")]
        pub created_by: ::std::string::String,
        #[serde(rename = "endsAt")]
        pub ends_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub matchers: ::std::vec::Vec<Matcher>,
        #[serde(rename = "startsAt")]
        pub starts_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&Silence> for Silence {
        fn from(value: &Silence) -> Self {
            value.clone()
        }
    }
    ///`SilenceStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "state": {
    ///      "type": "string",
    ///      "enum": [
    ///        "expired",
    ///        "active",
    ///        "pending"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SilenceStatus {
        pub state: SilenceStatusState,
    }
    impl ::std::convert::From<&SilenceStatus> for SilenceStatus {
        fn from(value: &SilenceStatus) -> Self {
            value.clone()
        }
    }
    ///`SilenceStatusState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "expired",
    ///    "active",
    ///    "pending"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum SilenceStatusState {
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "pending")]
        Pending,
    }
    impl ::std::convert::From<&Self> for SilenceStatusState {
        fn from(value: &SilenceStatusState) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for SilenceStatusState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Expired => write!(f, "expired"),
                Self::Active => write!(f, "active"),
                Self::Pending => write!(f, "pending"),
            }
        }
    }
    impl ::std::str::FromStr for SilenceStatusState {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "expired" => Ok(Self::Expired),
                "active" => Ok(Self::Active),
                "pending" => Ok(Self::Pending),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SilenceStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SilenceStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SilenceStatusState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`VersionInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "branch",
    ///    "buildDate",
    ///    "buildUser",
    ///    "goVersion",
    ///    "revision",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "branch": {
    ///      "type": "string"
    ///    },
    ///    "buildDate": {
    ///      "type": "string"
    ///    },
    ///    "buildUser": {
    ///      "type": "string"
    ///    },
    ///    "goVersion": {
    ///      "type": "string"
    ///    },
    ///    "revision": {
    ///      "type": "string"
    ///    },
    ///    "version": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct VersionInfo {
        pub branch: ::std::string::String,
        #[serde(rename = "buildDate")]
        pub build_date: ::std::string::String,
        #[serde(rename = "buildUser")]
        pub build_user: ::std::string::String,
        #[serde(rename = "goVersion")]
        pub go_version: ::std::string::String,
        pub revision: ::std::string::String,
        pub version: ::std::string::String,
    }
    impl ::std::convert::From<&VersionInfo> for VersionInfo {
        fn from(value: &VersionInfo) -> Self {
            value.clone()
        }
    }
    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn default_bool<const V: bool>() -> bool {
            V
        }
    }
    ///Error enum for the `get_silences` operation
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum GetSilencesError {
        #[doc = concat!("Error response for status code ", "400")]
        Status400(::std::string::String),
        #[doc = concat!("Error response for status code ", "500")]
        Status500(::std::string::String),
        /// Error response for an unknown status code
        UnknownValue(serde_json::Value),
    }
    impl std::str::FromStr for GetSilencesError {
        type Err = crate::error::ConversionError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (status_code, value) = s
                .split_once(':')
                .ok_or_else(|| {
                    crate::error::ConversionError(
                        "Invalid format for error enum, expected 'status_code:value'"
                            .into(),
                    )
                })?;
            let status_code: u16 = status_code
                .parse()
                .map_err(|_| {
                    crate::error::ConversionError(
                        "Invalid status code in error enum".into(),
                    )
                })?;
            match status_code {
                400u16 => Ok(Self::Status400(value)),
                500u16 => Ok(Self::Status500(value)),
                _ => {
                    match serde_json::from_str(value) {
                        Ok(json_value) => Ok(Self::UnknownValue(json_value)),
                        Err(_) => {
                            Err(
                                crate::error::ConversionError(
                                    format!(
                                        "Failed to parse unknown error response for status code: {}",
                                        status_code
                                    )
                                        .into(),
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
    ///Error enum for the `post_silences` operation
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum PostSilencesError {
        #[doc = concat!("Error response for status code ", "400")]
        Status400(::std::string::String),
        #[doc = concat!("Error response for status code ", "404")]
        Status404(::std::string::String),
        /// Error response for an unknown status code
        UnknownValue(serde_json::Value),
    }
    impl std::str::FromStr for PostSilencesError {
        type Err = crate::error::ConversionError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (status_code, value) = s
                .split_once(':')
                .ok_or_else(|| {
                    crate::error::ConversionError(
                        "Invalid format for error enum, expected 'status_code:value'"
                            .into(),
                    )
                })?;
            let status_code: u16 = status_code
                .parse()
                .map_err(|_| {
                    crate::error::ConversionError(
                        "Invalid status code in error enum".into(),
                    )
                })?;
            match status_code {
                400u16 => Ok(Self::Status400(value)),
                404u16 => Ok(Self::Status404(value)),
                _ => {
                    match serde_json::from_str(value) {
                        Ok(json_value) => Ok(Self::UnknownValue(json_value)),
                        Err(_) => {
                            Err(
                                crate::error::ConversionError(
                                    format!(
                                        "Failed to parse unknown error response for status code: {}",
                                        status_code
                                    )
                                        .into(),
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
    ///Error enum for the `get_silence` operation
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum GetSilenceError {
        #[doc = concat!("Error response for status code ", "404")]
        Status404(()),
        #[doc = concat!("Error response for status code ", "500")]
        Status500(::std::string::String),
        /// Error response for an unknown status code
        UnknownValue(serde_json::Value),
    }
    impl std::str::FromStr for GetSilenceError {
        type Err = crate::error::ConversionError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (status_code, value) = s
                .split_once(':')
                .ok_or_else(|| {
                    crate::error::ConversionError(
                        "Invalid format for error enum, expected 'status_code:value'"
                            .into(),
                    )
                })?;
            let status_code: u16 = status_code
                .parse()
                .map_err(|_| {
                    crate::error::ConversionError(
                        "Invalid status code in error enum".into(),
                    )
                })?;
            match status_code {
                404u16 => Ok(Self::Status404(value)),
                500u16 => Ok(Self::Status500(value)),
                _ => {
                    match serde_json::from_str(value) {
                        Ok(json_value) => Ok(Self::UnknownValue(json_value)),
                        Err(_) => {
                            Err(
                                crate::error::ConversionError(
                                    format!(
                                        "Failed to parse unknown error response for status code: {}",
                                        status_code
                                    )
                                        .into(),
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
    ///Error enum for the `delete_silence` operation
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum DeleteSilenceError {
        #[doc = concat!("Error response for status code ", "404")]
        Status404(()),
        #[doc = concat!("Error response for status code ", "500")]
        Status500(::std::string::String),
        /// Error response for an unknown status code
        UnknownValue(serde_json::Value),
    }
    impl std::str::FromStr for DeleteSilenceError {
        type Err = crate::error::ConversionError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (status_code, value) = s
                .split_once(':')
                .ok_or_else(|| {
                    crate::error::ConversionError(
                        "Invalid format for error enum, expected 'status_code:value'"
                            .into(),
                    )
                })?;
            let status_code: u16 = status_code
                .parse()
                .map_err(|_| {
                    crate::error::ConversionError(
                        "Invalid status code in error enum".into(),
                    )
                })?;
            match status_code {
                404u16 => Ok(Self::Status404(value)),
                500u16 => Ok(Self::Status500(value)),
                _ => {
                    match serde_json::from_str(value) {
                        Ok(json_value) => Ok(Self::UnknownValue(json_value)),
                        Err(_) => {
                            Err(
                                crate::error::ConversionError(
                                    format!(
                                        "Failed to parse unknown error response for status code: {}",
                                        status_code
                                    )
                                        .into(),
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
    ///Error enum for the `get_alerts` operation
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum GetAlertsError {
        #[doc = concat!("Error response for status code ", "400")]
        Status400(::std::string::String),
        #[doc = concat!("Error response for status code ", "500")]
        Status500(::std::string::String),
        /// Error response for an unknown status code
        UnknownValue(serde_json::Value),
    }
    impl std::str::FromStr for GetAlertsError {
        type Err = crate::error::ConversionError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (status_code, value) = s
                .split_once(':')
                .ok_or_else(|| {
                    crate::error::ConversionError(
                        "Invalid format for error enum, expected 'status_code:value'"
                            .into(),
                    )
                })?;
            let status_code: u16 = status_code
                .parse()
                .map_err(|_| {
                    crate::error::ConversionError(
                        "Invalid status code in error enum".into(),
                    )
                })?;
            match status_code {
                400u16 => Ok(Self::Status400(value)),
                500u16 => Ok(Self::Status500(value)),
                _ => {
                    match serde_json::from_str(value) {
                        Ok(json_value) => Ok(Self::UnknownValue(json_value)),
                        Err(_) => {
                            Err(
                                crate::error::ConversionError(
                                    format!(
                                        "Failed to parse unknown error response for status code: {}",
                                        status_code
                                    )
                                        .into(),
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
    ///Error enum for the `post_alerts` operation
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum PostAlertsError {
        #[doc = concat!("Error response for status code ", "400")]
        Status400(::std::string::String),
        #[doc = concat!("Error response for status code ", "500")]
        Status500(::std::string::String),
        /// Error response for an unknown status code
        UnknownValue(serde_json::Value),
    }
    impl std::str::FromStr for PostAlertsError {
        type Err = crate::error::ConversionError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (status_code, value) = s
                .split_once(':')
                .ok_or_else(|| {
                    crate::error::ConversionError(
                        "Invalid format for error enum, expected 'status_code:value'"
                            .into(),
                    )
                })?;
            let status_code: u16 = status_code
                .parse()
                .map_err(|_| {
                    crate::error::ConversionError(
                        "Invalid status code in error enum".into(),
                    )
                })?;
            match status_code {
                400u16 => Ok(Self::Status400(value)),
                500u16 => Ok(Self::Status500(value)),
                _ => {
                    match serde_json::from_str(value) {
                        Ok(json_value) => Ok(Self::UnknownValue(json_value)),
                        Err(_) => {
                            Err(
                                crate::error::ConversionError(
                                    format!(
                                        "Failed to parse unknown error response for status code: {}",
                                        status_code
                                    )
                                        .into(),
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
    ///Error enum for the `get_alert_groups` operation
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum GetAlertGroupsError {
        #[doc = concat!("Error response for status code ", "400")]
        Status400(::std::string::String),
        #[doc = concat!("Error response for status code ", "500")]
        Status500(::std::string::String),
        /// Error response for an unknown status code
        UnknownValue(serde_json::Value),
    }
    impl std::str::FromStr for GetAlertGroupsError {
        type Err = crate::error::ConversionError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (status_code, value) = s
                .split_once(':')
                .ok_or_else(|| {
                    crate::error::ConversionError(
                        "Invalid format for error enum, expected 'status_code:value'"
                            .into(),
                    )
                })?;
            let status_code: u16 = status_code
                .parse()
                .map_err(|_| {
                    crate::error::ConversionError(
                        "Invalid status code in error enum".into(),
                    )
                })?;
            match status_code {
                400u16 => Ok(Self::Status400(value)),
                500u16 => Ok(Self::Status500(value)),
                _ => {
                    match serde_json::from_str(value) {
                        Ok(json_value) => Ok(Self::UnknownValue(json_value)),
                        Err(_) => {
                            Err(
                                crate::error::ConversionError(
                                    format!(
                                        "Failed to parse unknown error response for status code: {}",
                                        status_code
                                    )
                                        .into(),
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Alertmanager API

API of the Prometheus Alertmanager (https://github.com/prometheus/alertmanager)

Version: 0.0.1*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "0.0.1"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
#[allow(elided_named_lifetimes)]
impl Client {
    /**Get current status of an Alertmanager instance and its cluster

Sends a `GET` request to `/status`

*/
    pub async fn get_status<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::GetStatusResponse>, Error<()>> {
        let url = format!("{}/status", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_status",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::empty(response))),
        }
    }
    /**Get list of all receivers (name of notification integrations)

Sends a `GET` request to `/receivers`

*/
    pub async fn get_receivers<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::GetReceiversResponseItem>>,
        Error<()>,
    > {
        let url = format!("{}/receivers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_receivers",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::empty(response))),
        }
    }
    /**Get a list of silences

Sends a `GET` request to `/silences`

Arguments:
- `filter`: A list of matchers to filter silences by
- `body`
*/
    pub async fn get_silences<'a>(
        &'a self,
        filter: Option<&'a ::std::vec::Vec<::std::string::String>>,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::GettableSilence>>,
        Error<types::GetSilencesError>,
    > {
        let url = format!("{}/silences", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("filter", &filter))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_silences",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::GetSilencesError::Status400(v)),
                    ),
                )
            }
            500u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::GetSilencesError::Status500(v)),
                    ),
                )
            }
            _ => {
                let status = response.status().as_u16();
                match response.json::<serde_json::Value>().await {
                    Ok(json_value) => {
                        Err(
                            Error::ErrorResponse(
                                ResponseValue::new(
                                    response,
                                    types::GetSilencesError::UnknownValue(json_value),
                                ),
                            ),
                        )
                    }
                    Err(_) => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                }
            }
        }
    }
    /**Post a new silence or update an existing one

Sends a `POST` request to `/silences`

*/
    pub async fn post_silences<'a>(
        &'a self,
        body: &'a types::PostSilencesBody,
    ) -> Result<
        ResponseValue<types::PostSilencesResponse>,
        Error<types::PostSilencesError>,
    > {
        let url = format!("{}/silences", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_silences",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::PostSilencesError::Status400(v)),
                    ),
                )
            }
            404u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::PostSilencesError::Status404(v)),
                    ),
                )
            }
            _ => {
                let status = response.status().as_u16();
                match response.json::<serde_json::Value>().await {
                    Ok(json_value) => {
                        Err(
                            Error::ErrorResponse(
                                ResponseValue::new(
                                    response,
                                    types::PostSilencesError::UnknownValue(json_value),
                                ),
                            ),
                        )
                    }
                    Err(_) => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                }
            }
        }
    }
    /**Get a silence by its ID

Sends a `GET` request to `/silence/{silenceID}`

Arguments:
- `silence_id`: ID of the silence to get
- `body`
*/
    pub async fn get_silence<'a>(
        &'a self,
        silence_id: &'a ::uuid::Uuid,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<
        ResponseValue<types::GetSilenceResponse>,
        Error<types::GetSilenceError>,
    > {
        let url = format!(
            "{}/silence/{}", self.baseurl, encode_path(& silence_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_silence",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::empty(response)
                            .map(|_| types::GetSilenceError::Status404(()))?,
                    ),
                )
            }
            500u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::GetSilenceError::Status500(v)),
                    ),
                )
            }
            _ => {
                let status = response.status().as_u16();
                match response.json::<serde_json::Value>().await {
                    Ok(json_value) => {
                        Err(
                            Error::ErrorResponse(
                                ResponseValue::new(
                                    response,
                                    types::GetSilenceError::UnknownValue(json_value),
                                ),
                            ),
                        )
                    }
                    Err(_) => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                }
            }
        }
    }
    /**Delete a silence by its ID

Sends a `DELETE` request to `/silence/{silenceID}`

Arguments:
- `silence_id`: ID of the silence to get
- `body`
*/
    pub async fn delete_silence<'a>(
        &'a self,
        silence_id: &'a ::uuid::Uuid,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<()>, Error<types::DeleteSilenceError>> {
        let url = format!(
            "{}/silence/{}", self.baseurl, encode_path(& silence_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_silence",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            404u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::empty(response)
                            .map(|_| types::DeleteSilenceError::Status404(()))?,
                    ),
                )
            }
            500u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::DeleteSilenceError::Status500(v)),
                    ),
                )
            }
            _ => {
                let status = response.status().as_u16();
                match response.json::<serde_json::Value>().await {
                    Ok(json_value) => {
                        Err(
                            Error::ErrorResponse(
                                ResponseValue::new(
                                    response,
                                    types::DeleteSilenceError::UnknownValue(json_value),
                                ),
                            ),
                        )
                    }
                    Err(_) => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                }
            }
        }
    }
    /**Get a list of alerts

Sends a `GET` request to `/alerts`

Arguments:
- `active`: Show active alerts
- `filter`: A list of matchers to filter alerts by
- `inhibited`: Show inhibited alerts
- `receiver`: A regex matching receivers to filter alerts by
- `silenced`: Show silenced alerts
- `unprocessed`: Show unprocessed alerts
- `body`
*/
    pub async fn get_alerts<'a>(
        &'a self,
        active: Option<bool>,
        filter: Option<&'a ::std::vec::Vec<::std::string::String>>,
        inhibited: Option<bool>,
        receiver: Option<&'a str>,
        silenced: Option<bool>,
        unprocessed: Option<bool>,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::GettableAlert>>,
        Error<types::GetAlertsError>,
    > {
        let url = format!("{}/alerts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("active", &active))
            .query(&progenitor_client::QueryParam::new("filter", &filter))
            .query(&progenitor_client::QueryParam::new("inhibited", &inhibited))
            .query(&progenitor_client::QueryParam::new("receiver", &receiver))
            .query(&progenitor_client::QueryParam::new("silenced", &silenced))
            .query(&progenitor_client::QueryParam::new("unprocessed", &unprocessed))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_alerts",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::GetAlertsError::Status400(v)),
                    ),
                )
            }
            500u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::GetAlertsError::Status500(v)),
                    ),
                )
            }
            _ => {
                let status = response.status().as_u16();
                match response.json::<serde_json::Value>().await {
                    Ok(json_value) => {
                        Err(
                            Error::ErrorResponse(
                                ResponseValue::new(
                                    response,
                                    types::GetAlertsError::UnknownValue(json_value),
                                ),
                            ),
                        )
                    }
                    Err(_) => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                }
            }
        }
    }
    /**Create new Alerts

Sends a `POST` request to `/alerts`

*/
    pub async fn post_alerts<'a>(
        &'a self,
        body: &'a types::PostAlertsBody,
    ) -> Result<ResponseValue<()>, Error<types::PostAlertsError>> {
        let url = format!("{}/alerts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_alerts",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::PostAlertsError::Status400(v)),
                    ),
                )
            }
            500u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::PostAlertsError::Status500(v)),
                    ),
                )
            }
            _ => {
                let status = response.status().as_u16();
                match response.json::<serde_json::Value>().await {
                    Ok(json_value) => {
                        Err(
                            Error::ErrorResponse(
                                ResponseValue::new(
                                    response,
                                    types::PostAlertsError::UnknownValue(json_value),
                                ),
                            ),
                        )
                    }
                    Err(_) => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                }
            }
        }
    }
    /**Get a list of alert groups

Sends a `GET` request to `/alerts/groups`

Arguments:
- `active`: Show active alerts
- `filter`: A list of matchers to filter alerts by
- `inhibited`: Show inhibited alerts
- `receiver`: A regex matching receivers to filter alerts by
- `silenced`: Show silenced alerts
- `body`
*/
    pub async fn get_alert_groups<'a>(
        &'a self,
        active: Option<bool>,
        filter: Option<&'a ::std::vec::Vec<::std::string::String>>,
        inhibited: Option<bool>,
        receiver: Option<&'a str>,
        silenced: Option<bool>,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::AlertGroup>>,
        Error<types::GetAlertGroupsError>,
    > {
        let url = format!("{}/alerts/groups", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("active", &active))
            .query(&progenitor_client::QueryParam::new("filter", &filter))
            .query(&progenitor_client::QueryParam::new("inhibited", &inhibited))
            .query(&progenitor_client::QueryParam::new("receiver", &receiver))
            .query(&progenitor_client::QueryParam::new("silenced", &silenced))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_alert_groups",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::GetAlertGroupsError::Status400(v)),
                    ),
                )
            }
            500u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::from_response::<::std::string::String>(response)
                            .await?
                            .map(|v| types::GetAlertGroupsError::Status500(v)),
                    ),
                )
            }
            _ => {
                let status = response.status().as_u16();
                match response.json::<serde_json::Value>().await {
                    Ok(json_value) => {
                        Err(
                            Error::ErrorResponse(
                                ResponseValue::new(
                                    response,
                                    types::GetAlertGroupsError::UnknownValue(json_value),
                                ),
                            ),
                        )
                    }
                    Err(_) => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                }
            }
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
