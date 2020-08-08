#[derive(Clone, Debug)]
pub struct Activity {
    m_name: String,
    m_is_exported: bool,
}

impl Activity {
    pub fn new() -> Activity {
        Activity {
            m_name: "".to_string(),
            m_is_exported: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeepClass {
    name: String,
    process: String,
    typ: String,
}

#[derive(Clone, Debug)]
pub struct ManifestData {
    pub m_package: String,
    pub m_version_name: String,
    pub m_default_process: String,
    pub m_version_code: i32,
    pub m_activities: Vec<Activity>,
    pub m_keep_classes: Vec<KeepClass>,
    pub m_launcher_activity: Activity,
}

impl ManifestData {
    pub fn new() -> ManifestData {
        ManifestData {
            m_package: "".to_string(),
            m_version_name: "".to_string(),
            m_default_process: "".to_string(),
            m_version_code: 0,
            m_activities: vec![],
            m_keep_classes: vec![],
            m_launcher_activity: Activity::new(),
        }
    }
}
