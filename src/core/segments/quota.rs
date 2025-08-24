use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

// API 响应结构
#[derive(Debug, Deserialize)]
struct PackyCodeApiResponse {
    #[serde(rename = "daily_spent_usd")]
    daily_spent_usd: String,
    #[serde(rename = "opus_enabled")]
    opus_enabled: bool,
}

// 端点配置
#[derive(Debug, Clone)]
struct EndpointConfig {
    url: String,
    name: String,
}

// 端点缓存
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EndpointCache {
    api_key_hash: u64,
    successful_endpoint: String,
    last_success_time: SystemTime,
    success_count: u32,
}

// 智能端点检测器
struct SmartEndpointDetector {
    endpoints: Vec<EndpointConfig>,
    cache: Option<EndpointCache>,
    cache_file_path: PathBuf,
}

impl SmartEndpointDetector {
    fn new() -> Self {
        let endpoints = vec![
            EndpointConfig {
                url: "https://www.packycode.com/api/backend/users/info".to_string(),
                name: "main".to_string(),
            },
            EndpointConfig {
                url: "https://share.packycode.com/api/backend/users/info".to_string(),
                name: "share".to_string(),
            },
        ];

        let cache_file_path = Self::get_cache_file_path();
        let cache = Self::load_cache(&cache_file_path);

        Self {
            endpoints,
            cache,
            cache_file_path,
        }
    }

    fn get_cache_file_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".claude").join("ccline").join("endpoint_cache.json")
        } else {
            PathBuf::from("endpoint_cache.json")
        }
    }

    fn load_cache(cache_path: &PathBuf) -> Option<EndpointCache> {
        if let Ok(content) = fs::read_to_string(cache_path) {
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    fn save_cache(&self) {
        if let Some(ref cache) = self.cache {
            // 确保目录存在
            if let Some(parent) = self.cache_file_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            
            if let Ok(content) = serde_json::to_string_pretty(cache) {
                let _ = fs::write(&self.cache_file_path, content);
            }
        }
    }

    fn hash_api_key(api_key: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        api_key.hash(&mut hasher);
        hasher.finish()
    }

    fn is_cache_valid(&self, api_key: &str) -> bool {
        if let Some(ref cache) = self.cache {
            let current_hash = Self::hash_api_key(api_key);
            let cache_age = SystemTime::now()
                .duration_since(cache.last_success_time)
                .unwrap_or(Duration::from_secs(u64::MAX));
            
            // 缓存有效条件：API key 相同且时间不超过 24 小时
            current_hash == cache.api_key_hash && cache_age < Duration::from_secs(86400)
        } else {
            false
        }
    }

    fn try_endpoint(&self, endpoint: &EndpointConfig, api_key: &str) -> Option<PackyCodeApiResponse> {
        let debug = env::var("PACKYCODE_DEBUG").is_ok();
        
        if debug {
            eprintln!("[DEBUG] Trying endpoint: {}", endpoint.url);
        }

        let start_time = SystemTime::now();
        let result = ureq::get(&endpoint.url)
            .set("Authorization", &format!("Bearer {}", api_key))
            .set("accept", "*/*")
            .set("content-type", "application/json")
            .timeout(Duration::from_secs(5))
            .call();

        match result {
            Ok(response) => {
                if response.status() == 200 {
                    let elapsed = start_time.elapsed().unwrap_or(Duration::from_secs(0));
                    if debug {
                        eprintln!("[DEBUG] Success: {} in {}ms", endpoint.name, elapsed.as_millis());
                    }
                    
                    response.into_json::<PackyCodeApiResponse>().ok()
                } else {
                    if debug {
                        eprintln!("[DEBUG] Failed: {} status {}", endpoint.name, response.status());
                    }
                    None
                }
            }
            Err(e) => {
                if debug {
                    eprintln!("[DEBUG] Error: {} - {}", endpoint.name, e);
                }
                None
            }
        }
    }

    fn detect_endpoint(&mut self, api_key: &str) -> Option<(String, PackyCodeApiResponse)> {
        // 检查缓存是否有效
        if self.is_cache_valid(api_key) {
            if let Some(ref cache) = self.cache.clone() {
                let cached_endpoint = &cache.successful_endpoint;
                
                // 尝试使用缓存的端点
                if let Some(endpoint) = self.endpoints.iter().find(|e| e.url == *cached_endpoint) {
                    if let Some(response) = self.try_endpoint(endpoint, api_key) {
                        // 更新缓存统计
                        self.update_cache_stats(api_key, cached_endpoint);
                        return Some((cached_endpoint.clone(), response));
                    }
                }
            }
        }

        // 缓存失效或失败，尝试所有端点
        let endpoints_clone = self.endpoints.clone();
        for endpoint in &endpoints_clone {
            if let Some(response) = self.try_endpoint(endpoint, api_key) {
                // 更新缓存
                self.update_cache(api_key, &endpoint.url);
                return Some((endpoint.url.clone(), response));
            }
        }

        None
    }

    fn update_cache(&mut self, api_key: &str, successful_endpoint: &str) {
        let new_cache = EndpointCache {
            api_key_hash: Self::hash_api_key(api_key),
            successful_endpoint: successful_endpoint.to_string(),
            last_success_time: SystemTime::now(),
            success_count: 1,
        };

        self.cache = Some(new_cache);
        self.save_cache();
    }

    fn update_cache_stats(&mut self, _api_key: &str, _successful_endpoint: &str) {
        if let Some(ref mut cache) = self.cache {
            cache.last_success_time = SystemTime::now();
            cache.success_count += 1;
            self.save_cache();
        }
    }

    fn detect_endpoint_static(api_key: &str) -> Option<(String, PackyCodeApiResponse)> {
        let mut detector = SmartEndpointDetector::new();
        detector.detect_endpoint(api_key)
    }
}

#[derive(Default)]
pub struct QuotaSegment;

impl QuotaSegment {
    pub fn new() -> Self {
        Self
    }

    fn load_api_key(&self) -> Option<String> {
        // 优先级：环境变量 > Claude Code settings.json > api_key 文件
        
        // 1. 环境变量
        if let Ok(key) = env::var("PACKYCODE_API_KEY") {
            return Some(key);
        }
        
        if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
            return Some(key);
        }
        
        if let Ok(key) = env::var("ANTHROPIC_AUTH_TOKEN") {
            return Some(key);
        }

        // 2. Claude Code settings.json
        if let Some(key) = self.load_from_settings() {
            return Some(key);
        }

        // 3. api_key 文件
        if let Some(home) = dirs::home_dir() {
            let api_key_path = home.join(".claude").join("api_key");
            if let Ok(key) = fs::read_to_string(api_key_path) {
                return Some(key.trim().to_string());
            }
        }

        None
    }

    fn load_from_settings(&self) -> Option<String> {
        if let Some(home) = dirs::home_dir() {
            let settings_path = home.join(".claude").join("settings.json");
            if let Ok(content) = fs::read_to_string(settings_path) {
                if let Ok(settings) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(env) = settings.get("env") {
                        if let Some(token) = env.get("ANTHROPIC_AUTH_TOKEN") {
                            if let Some(token_str) = token.as_str() {
                                return Some(token_str.to_string());
                            }
                        }
                        if let Some(key) = env.get("ANTHROPIC_API_KEY") {
                            if let Some(key_str) = key.as_str() {
                                return Some(key_str.to_string());
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn format_daily_spent(&self, spent_str: &str) -> String {
        if let Ok(spent) = spent_str.parse::<f64>() {
            format!("${:.2}", spent)
        } else {
            format!("${}", spent_str)
        }
    }

    fn format_opus_status(&self, enabled: bool) -> String {
        if enabled {
            "Opus✓".to_string()
        } else {
            "Opus✗".to_string()
        }
    }
}

impl Segment for QuotaSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        #[cfg(not(feature = "quota"))]
        {
            return None;
        }

        #[cfg(feature = "quota")]
        {
            let api_key = self.load_api_key()?;
            
            // 使用静态方法进行端点检测
            if let Some((endpoint_url, response)) = SmartEndpointDetector::detect_endpoint_static(&api_key) {
                let daily_spent = self.format_daily_spent(&response.daily_spent_usd);
                let opus_status = self.format_opus_status(response.opus_enabled);
                
                let mut metadata = HashMap::new();
                metadata.insert("raw_spent".to_string(), response.daily_spent_usd);
                metadata.insert("opus_enabled".to_string(), response.opus_enabled.to_string());
                metadata.insert("endpoint_used".to_string(), endpoint_url);

                Some(SegmentData {
                    primary: daily_spent,
                    secondary: opus_status,
                    metadata,
                })
            } else {
                // 所有端点都失败
                let mut metadata = HashMap::new();
                metadata.insert("status".to_string(), "offline".to_string());
                
                Some(SegmentData {
                    primary: "Offline".to_string(),
                    secondary: "".to_string(),
                    metadata,
                })
            }
        }
    }

    fn id(&self) -> SegmentId {
        SegmentId::Quota
    }
}