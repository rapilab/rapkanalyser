pub struct Activity {
    m_name: String,
    m_is_exported: bool
}

pub struct KeepClass {
    name: String,
    process: String,
    typ: String,
}

pub struct ManifestData {
    m_package: String,
    m_version_name: String,
    m_default_process: String,
    m_version_code: i32,
    m_activities: Vec<Activity>,
    m_keep_classes: Vec<KeepClass>,
    m_launcher_activity: Activity
}
