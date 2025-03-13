use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct EngravingData {
    pub id: u32,
    pub name: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ArkPassiveData {
    pub evolution: Option<Vec<ArkPassiveNode>>,
    pub enlightenment: Option<Vec<ArkPassiveNode>>,
    pub leap: Option<Vec<ArkPassiveNode>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ArkPassiveNode {
    pub id: u32,
    pub lv: u8,
}