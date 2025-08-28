use regex::Regex;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct LocationResult {
    pub start_index: usize,
    pub end_index: usize,
    pub variable_name: Option<String>,
}

#[derive(Debug)]
pub struct ClaudeCodePatcher {
    file_content: String,
    file_path: String,
}

impl ClaudeCodePatcher {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = file_path.as_ref();
        let content = fs::read_to_string(path)?;

        Ok(Self {
            file_content: content,
            file_path: path.to_string_lossy().to_string(),
        })
    }

    /// Find the verbose property location in Claude Code's cli.js
    /// Based on the pattern from patching.ts getVerbosePropertyLocation function
    pub fn get_verbose_property_location(&self) -> Option<LocationResult> {
        // Step 1: Find createElement pattern with spinnerTip and overrideMessage
        let create_element_pattern =
            Regex::new(r"createElement\([$\w]+,\{[^}]+spinnerTip[^}]+overrideMessage[^}]+\}")
                .ok()?;

        let create_element_match = create_element_pattern.find(&self.file_content)?;
        let extracted_string =
            &self.file_content[create_element_match.start()..create_element_match.end()];

        println!(
            "Found createElement match at: {}-{}",
            create_element_match.start(),
            create_element_match.end()
        );
        println!(
            "Extracted string: {}",
            &extracted_string[..std::cmp::min(200, extracted_string.len())]
        );

        // Step 2: Find verbose property within the createElement match
        let verbose_pattern = Regex::new(r"verbose:[^,}]+").ok()?;
        let verbose_match = verbose_pattern.find(extracted_string)?;

        println!(
            "Found verbose match at: {}-{}",
            verbose_match.start(),
            verbose_match.end()
        );
        println!("Verbose string: {}", verbose_match.as_str());

        // Calculate absolute positions in the original file
        let absolute_verbose_start = create_element_match.start() + verbose_match.start();
        let absolute_verbose_end = absolute_verbose_start + verbose_match.len();

        Some(LocationResult {
            start_index: absolute_verbose_start,
            end_index: absolute_verbose_end,
            variable_name: None,
        })
    }

    /// Write the verbose property with new value
    pub fn write_verbose_property(
        &mut self,
        value: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let location = self
            .get_verbose_property_location()
            .ok_or("Failed to find verbose property location")?;

        let new_code = format!("verbose:{}", value);

        let new_content = format!(
            "{}{}{}",
            &self.file_content[..location.start_index],
            new_code,
            &self.file_content[location.end_index..]
        );

        self.show_diff(&new_code, location.start_index, location.end_index);
        self.file_content = new_content;

        Ok(())
    }

    /// Save the modified content back to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(&self.file_path, &self.file_content)?;
        Ok(())
    }

    /// Get a reference to the file content (for testing purposes)
    pub fn get_file_content(&self) -> &str {
        &self.file_content
    }

    /// Show a diff of the changes (for debugging)
    fn show_diff(&self, injected_text: &str, start_index: usize, end_index: usize) {
        let context_start = start_index.saturating_sub(50);
        let context_end_old = std::cmp::min(self.file_content.len(), end_index + 50);

        let old_before = &self.file_content[context_start..start_index];
        let old_changed = &self.file_content[start_index..end_index];
        let old_after = &self.file_content[end_index..context_end_old];

        println!("\n--- Verbose Property Diff ---");
        println!(
            "OLD: {}\x1b[31m{}\x1b[0m{}",
            old_before, old_changed, old_after
        );
        println!(
            "NEW: {}\x1b[32m{}\x1b[0m{}",
            old_before, injected_text, old_after
        );
        println!("--- End Diff ---\n");
    }

    /// Find the context low message location in Claude Code's cli.js
    /// Pattern: "Context low (",B,"% remaining) · Run /compact to compact & continue"
    /// where B is a variable name
    pub fn get_context_low_message_location(&self) -> Option<LocationResult> {
        // Pattern to match: "Context low (",{variable},"% remaining) · Run /compact to compact & continue"
        let context_low_pattern = Regex::new(
            r#""Context low \(",([^,]+),"% remaining\) · Run /compact to compact & continue""#,
        )
        .ok()?;

        let context_low_match = context_low_pattern.find(&self.file_content)?;

        println!(
            "Found context low match at: {}-{}",
            context_low_match.start(),
            context_low_match.end()
        );
        println!("Context low string: {}", context_low_match.as_str());

        // Extract the variable name from the capture group
        let captures = context_low_pattern.captures(&self.file_content)?;
        let variable_name = captures.get(1)?.as_str();

        println!("Variable name: {}", variable_name);

        Some(LocationResult {
            start_index: context_low_match.start(),
            end_index: context_low_match.end(),
            variable_name: Some(variable_name.to_string()),
        })
    }

    /// Core robust function locator using anchor-based expansion
    /// Uses stable text patterns to survive Claude Code version updates
    pub fn find_context_low_function_robust(&self) -> Option<LocationResult> {
        // Step 1: Locate stable anchor text that survives obfuscation
        let primary_anchor = "Context low (";
        let anchor_pos = self.file_content.find(primary_anchor)?;

        // Step 2: Search backward within reasonable range to find function declarations
        let search_range = 800; // Optimized range based on actual function size (~466 chars)
        let search_start = anchor_pos.saturating_sub(search_range);
        let backward_text = &self.file_content[search_start..anchor_pos];

        // Find the function declaration that contains our anchor
        let mut function_candidates = Vec::new();
        let mut start = 0;

        while let Some(func_pos) = backward_text[start..].find("function ") {
            let absolute_func_pos = search_start + start + func_pos;

            // Check if this function contains the expected stable patterns
            let func_to_anchor_text = &self.file_content[absolute_func_pos..anchor_pos + 100];

            if func_to_anchor_text.contains("tokenUsage:") {
                function_candidates.push(absolute_func_pos);
                println!("Found function candidate at: {}", absolute_func_pos);
            }

            start += func_pos + 9; // Move past "function "
        }

        // Use the closest function to anchor (last candidate found)
        if let Some(&func_start) = function_candidates.last() {
            println!("Selected function start at: {}", func_start);

            // We only need the function start for condition replacement
            // Return a minimal range that includes the condition
            let condition_search_end = anchor_pos + 100; // Small range after anchor

            Some(LocationResult {
                start_index: func_start,
                end_index: condition_search_end,
                variable_name: Some("context_function".to_string()),
            })
        } else {
            println!("❌ No suitable function candidate found");
            None
        }
    }

    /// Core robust condition locator that finds the if statement to patch
    /// Returns the exact location of 'if(!Q||D)return null' for replacement with 'if(true)return null'
    pub fn get_context_low_condition_location_robust(&self) -> Option<LocationResult> {
        // Find the function using stable patterns
        let function_location = self.find_context_low_function_robust()?;
        let function_content =
            &self.file_content[function_location.start_index..function_location.end_index];

        // Look for if condition pattern using regex - match any condition that returns null
        let if_pattern = Regex::new(r"if\([^)]+\)return null").ok()?;

        if let Some(if_match) = if_pattern.find(function_content) {
            let absolute_start = function_location.start_index + if_match.start();
            let absolute_end = function_location.start_index + if_match.end();

            println!("Found if condition: '{}'", if_match.as_str());

            Some(LocationResult {
                start_index: absolute_start,
                end_index: absolute_end,
                variable_name: Some(if_match.as_str().to_string()),
            })
        } else {
            println!("❌ Could not find if condition in context function");
            None
        }
    }

    /// Disable context low warnings by modifying the if condition to always return null
    /// Uses robust pattern matching based on stable identifiers
    pub fn disable_context_low_warnings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(location) = self.get_context_low_condition_location_robust() {
            let replacement_condition = "if(true)return null";

            let new_content = format!(
                "{}{}{}",
                &self.file_content[..location.start_index],
                replacement_condition,
                &self.file_content[location.end_index..]
            );

            self.show_diff(
                replacement_condition,
                location.start_index,
                location.end_index,
            );
            self.file_content = new_content;

            println!("✅ Context low warnings disabled successfully");
            Ok(())
        } else {
            Err("Could not locate context low condition using robust method".into())
        }
    }

    /// Write a replacement for the context low message
    pub fn write_context_low_message(
        &mut self,
        new_message: &str,
        variable_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let location = self
            .get_context_low_message_location()
            .ok_or("Failed to find context low message location")?;

        let new_code = format!(
            r#""{}","{}","{}""#,
            new_message.split(',').nth(0).unwrap_or(new_message),
            variable_name,
            new_message.split(',').nth(1).unwrap_or("")
        );

        let new_content = format!(
            "{}{}{}",
            &self.file_content[..location.start_index],
            new_code,
            &self.file_content[location.end_index..]
        );

        self.show_diff(&new_code, location.start_index, location.end_index);
        self.file_content = new_content;

        Ok(())
    }
}
