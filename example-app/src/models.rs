use serde::{Deserialize, Serialize};

/// User profile information stored as JSON in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub preferences: UserPreferences,
    pub social_links: Vec<SocialLink>,
}

/// User preferences within the profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
}

/// Social media links (for nested use in UserProfile)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLink {
    pub platform: String,
    pub url: String,
}

/// Social media link for top-level social_links column
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSocialLink {
    pub name: String,
    pub url: String,
}

/// Tag for top-level tags jsonb[] column
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserTag {
    pub label: String,
    pub value: String,
}

/// Article metadata stored as a single JSONB object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArticleMetadata {
    pub category: String,
    pub published: bool,
}

/// Article contributor stored in a JSONB array
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArticleContributor {
    pub name: String,
    pub role: String,
}
