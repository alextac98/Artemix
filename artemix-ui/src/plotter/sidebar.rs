use egui::{ScrollArea, Ui};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub parent: String,
    pub channel: String,
}

#[derive(Debug, Deserialize)]
struct QueryResponse {
    query: String,
    columns: Vec<ColumnInfo>,
    #[serde(rename = "dataset")]
    data: Vec<Vec<serde_json::Value>>,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct ColumnInfo {
    name: String,
    #[serde(rename = "type")]
    column_type: String,
}

pub struct Sidebar {
    channels: Vec<ChannelInfo>,
    selected_channels: HashSet<String>,
    loading: bool,
    error_message: Option<String>,
    db_url: String,
    channels_loaded: bool,
    search_text: String,
    compiled_regex: Option<Regex>,
    regex_error: Option<String>,
    case_sensitive: bool,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl Sidebar {
    pub fn new() -> Self {
        let mut sidebar = Self {
            channels: Vec::new(),
            selected_channels: HashSet::new(),
            loading: false,
            error_message: None,
            db_url: "http://localhost:9000".to_string(),
            channels_loaded: false,
            search_text: String::new(),
            compiled_regex: None,
            regex_error: None,
            case_sensitive: false, // Default to case-insensitive
        };
        
        // Load channels on startup only once
        sidebar.load_mock_channels();
        sidebar.channels_loaded = true;
        sidebar
    }

    pub fn with_db_url(mut self, url: String) -> Self {
        self.db_url = url;
        self
    }

    pub fn selected_channels(&self) -> &HashSet<String> {
        &self.selected_channels
    }

    pub fn refresh_channels(&mut self) {
        if self.loading {
            return;
        }

        self.loading = true;
        self.error_message = None;

        // For now, load mock channels. 
        // TODO: Implement real database query when async context is available
        self.load_mock_channels();
        self.channels_loaded = true;
        self.loading = false;
    }

    // Future implementation for real database queries
    async fn query_channels_async(&self) -> Result<Vec<ChannelInfo>, String> {
        let query = "SELECT DISTINCT parent, channel FROM iss_001 WHERE channel IS NOT NULL ORDER BY parent, channel";
        let url = format!("{}/exec?query={}", self.db_url, urlencoding::encode(query));

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to query database: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Database query failed with status: {}", response.status()));
        }

        let query_response: QueryResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let mut channels = Vec::new();
        for row in query_response.data {
            if row.len() >= 2 {
                let parent = row[0].as_str().unwrap_or("Unknown").to_string();
                let channel = row[1].as_str().unwrap_or("Unknown").to_string();
                channels.push(ChannelInfo { parent, channel });
            }
        }

        Ok(channels)
    }

    // Mock function to simulate database response
    fn load_mock_channels(&mut self) {
        self.channels = vec![
            // Power System Channels
            ChannelInfo {
                parent: "Power".to_string(),
                channel: "SOLAR_ARRAY_1_VOLTAGE".to_string(),
            },
            ChannelInfo {
                parent: "Power".to_string(),
                channel: "SOLAR_ARRAY_2_VOLTAGE".to_string(),
            },
            ChannelInfo {
                parent: "Power".to_string(),
                channel: "SOLAR_ARRAY_3_VOLTAGE".to_string(),
            },
            ChannelInfo {
                parent: "Power".to_string(),
                channel: "SOLAR_ARRAY_4_VOLTAGE".to_string(),
            },
            ChannelInfo {
                parent: "Power".to_string(),
                channel: "BATTERY_1_VOLTAGE".to_string(),
            },
            ChannelInfo {
                parent: "Power".to_string(),
                channel: "BATTERY_2_VOLTAGE".to_string(),
            },
            ChannelInfo {
                parent: "Power".to_string(),
                channel: "MAIN_BUS_VOLTAGE".to_string(),
            },
            
            // Thermal System Channels
            ChannelInfo {
                parent: "Thermal".to_string(),
                channel: "NODE1_TEMP_1".to_string(),
            },
            ChannelInfo {
                parent: "Thermal".to_string(),
                channel: "NODE1_TEMP_2".to_string(),
            },
            ChannelInfo {
                parent: "Thermal".to_string(),
                channel: "NODE2_TEMP_1".to_string(),
            },
            ChannelInfo {
                parent: "Thermal".to_string(),
                channel: "RADIATOR_TEMP_1".to_string(),
            },
            ChannelInfo {
                parent: "Thermal".to_string(),
                channel: "RADIATOR_TEMP_2".to_string(),
            },
            
            // Attitude Control System
            ChannelInfo {
                parent: "Attitude".to_string(),
                channel: "GYRO_1_X_RATE".to_string(),
            },
            ChannelInfo {
                parent: "Attitude".to_string(),
                channel: "GYRO_1_Y_RATE".to_string(),
            },
            ChannelInfo {
                parent: "Attitude".to_string(),
                channel: "GYRO_1_Z_RATE".to_string(),
            },
            ChannelInfo {
                parent: "Attitude".to_string(),
                channel: "GYRO_2_X_RATE".to_string(),
            },
            ChannelInfo {
                parent: "Attitude".to_string(),
                channel: "GYRO_2_Y_RATE".to_string(),
            },
            ChannelInfo {
                parent: "Attitude".to_string(),
                channel: "GYRO_2_Z_RATE".to_string(),
            },
            
            // Life Support
            ChannelInfo {
                parent: "LifeSupport".to_string(),
                channel: "CABIN_PRESSURE".to_string(),
            },
            ChannelInfo {
                parent: "LifeSupport".to_string(),
                channel: "CABIN_TEMPERATURE".to_string(),
            },
            ChannelInfo {
                parent: "LifeSupport".to_string(),
                channel: "O2_CONCENTRATION".to_string(),
            },
            ChannelInfo {
                parent: "LifeSupport".to_string(),
                channel: "CO2_CONCENTRATION".to_string(),
            },
            
            // Communications
            ChannelInfo {
                parent: "Communications".to_string(),
                channel: "S_BAND_POWER".to_string(),
            },
            ChannelInfo {
                parent: "Communications".to_string(),
                channel: "KU_BAND_POWER".to_string(),
            },
            ChannelInfo {
                parent: "Communications".to_string(),
                channel: "ANTENNA_POSITION_AZ".to_string(),
            },
            ChannelInfo {
                parent: "Communications".to_string(),
                channel: "ANTENNA_POSITION_EL".to_string(),
            },
        ];
    }

    fn update_search_regex(&mut self) {
        if self.search_text.is_empty() {
            self.compiled_regex = None;
            self.regex_error = None;
            return;
        }

        // Build regex with case sensitivity setting
        let pattern = if self.case_sensitive {
            self.search_text.clone()
        } else {
            format!("(?i){}", self.search_text) // (?i) makes regex case-insensitive
        };

        match Regex::new(&pattern) {
            Ok(regex) => {
                self.compiled_regex = Some(regex);
                self.regex_error = None;
            }
            Err(e) => {
                self.compiled_regex = None;
                self.regex_error = Some(format!("Invalid regex: {}", e));
            }
        }
    }

    fn channel_matches_search(&self, channel: &ChannelInfo) -> bool {
        if self.search_text.is_empty() {
            return true;
        }

        if let Some(regex) = &self.compiled_regex {
            // Search in both channel name and parent name
            regex.is_match(&channel.channel) || regex.is_match(&channel.parent)
        } else {
            // Fallback to simple case-insensitive substring matching if regex is invalid
            let search_lower = self.search_text.to_lowercase();
            channel.channel.to_lowercase().contains(&search_lower) ||
            channel.parent.to_lowercase().contains(&search_lower)
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Channels");
        
        ui.horizontal(|ui| {
            if ui.button("🔄 Refresh").clicked() {
                self.refresh_channels();
            }
            
            if self.loading {
                ui.spinner();
                ui.label("Loading channels...");
            }
        });

        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
        }

        ui.separator();

        // Search box
        ui.horizontal(|ui| {
            ui.label("🔍");

            // We use a right-to-left layout to make the buttons appear on the right
            // and the text edit fill the remaining space.
            let response = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Add the clear button
                if ui.add(
                    egui::Button::new("🗑")
                        .min_size(egui::vec2(25.0, 18.0))
                ).on_hover_text("Clear search").clicked() {
                    self.search_text.clear();
                    self.compiled_regex = None;
                    self.regex_error = None;
                }

                // Add the case sensitivity button
                let case_button_text = if self.case_sensitive { "Aa" } else { "aa" };
                let case_button_color = if self.case_sensitive {
                    ui.visuals().selection.bg_fill
                } else {
                    ui.visuals().widgets.inactive.bg_fill
                };
                if ui.add(
                    egui::Button::new(case_button_text)
                        .fill(case_button_color)
                        .min_size(egui::vec2(25.0, 18.0))
                ).on_hover_text(
                    if self.case_sensitive {
                        "Case-sensitive search (click to make case-insensitive)"
                    } else {
                        "Case-insensitive search (click to make case-sensitive)"
                    }
                ).clicked() {
                    self.case_sensitive = !self.case_sensitive;
                    self.update_search_regex();
                }

                // Add the text edit field, making it fill the available width.
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_text)
                        .hint_text("Search channels...")
                        .desired_width(f32::INFINITY)
                )
            }).inner;

            // If the text edit field changed, update the regex.
            if response.changed() {
                self.update_search_regex();
            }
        });

        // Show regex error if any
        if let Some(error) = &self.regex_error {
            ui.colored_label(egui::Color32::RED, error);
        }

        ScrollArea::vertical().show(ui, |ui| {
            // Group channels by parent using BTreeMap to maintain consistent ordering
            let mut grouped_channels = std::collections::BTreeMap::new();
            for channel in &self.channels {
                if !self.channel_matches_search(channel) {
                    continue; // Skip channels that don't match the search
                }

                grouped_channels
                    .entry(channel.parent.clone())
                    .or_insert_with(Vec::new)
                    .push(channel.clone());
            }

            for (parent, channels) in grouped_channels {
                // Use a stable ID for each collapsing header to maintain state
                let header_id = format!("channel_group_{}", parent);
                let header_text = if channels.len() < self.channels.iter().filter(|ch| ch.parent == parent).count() {
                    // Show filtered count if search is active
                    format!("{} ({}/{})", parent, channels.len(), self.channels.iter().filter(|ch| ch.parent == parent).count())
                } else {
                    format!("{} ({})", parent, channels.len())
                };
                
                egui::CollapsingHeader::new(header_text)
                    .id_salt(header_id)
                    .default_open(false)
                    .show(ui, |ui| {
                        for channel in channels {
                            let channel_id = format!("{}::{}", channel.parent, channel.channel);
                            let mut selected = self.selected_channels.contains(&channel_id);
                            
                            if ui.checkbox(&mut selected, &channel.channel)
                                .on_hover_text(format!("Channel: {}\nParent: {}", channel.channel, channel.parent))
                                .changed() {
                                if selected {
                                    self.selected_channels.insert(channel_id);
                                } else {
                                    self.selected_channels.remove(&channel_id);
                                }
                            }
                        }
                    });
            }
        });

        ui.separator();
        
        // Action buttons
        ui.horizontal(|ui| {
            // Show selected channels count
            ui.label(format!("Selected: {}", self.selected_channels.len()));
            
            if !self.selected_channels.is_empty() {
                if ui.button("Clear All").clicked() {
                    self.selected_channels.clear();
                }
            }
            
            // Select/Deselect all visible channels
            if !self.search_text.is_empty() {
                let visible_channels: Vec<_> = self.channels.iter()
                    .filter(|ch| self.channel_matches_search(ch))
                    .collect();
                
                if !visible_channels.is_empty() {
                    let all_visible_selected = visible_channels.iter()
                        .all(|ch| {
                            let channel_id = format!("{}::{}", ch.parent, ch.channel);
                            self.selected_channels.contains(&channel_id)
                        });
                    
                    let button_text = if all_visible_selected {
                        "Deselect Visible"
                    } else {
                        "Select Visible"
                    };
                    
                    if ui.button(button_text).clicked() {
                        for channel in visible_channels {
                            let channel_id = format!("{}::{}", channel.parent, channel.channel);
                            if all_visible_selected {
                                self.selected_channels.remove(&channel_id);
                            } else {
                                self.selected_channels.insert(channel_id);
                            }
                        }
                    }
                }
            }
        });
    }
}
