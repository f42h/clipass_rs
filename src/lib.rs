/* 
* MIT License
* 
* Copyright (c) 2025 f42h
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use std::io::{self, Write};
use crossterm::{
    terminal,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind}
};
use sha256;

struct PromptMask {
    token: char,
    content: String
}

pub struct CliPass {
    label: String, // Input prompt label
    no_label: bool, // Manage visibility of `label`
    no_visibility: bool, // Do not reveal input date via `KeyCode::Up`
    prompt_mask_token: char, // Default mask token --> '*'
    prompt_input: String
}

impl CliPass {
    pub fn new() -> Self {
        Self {
            label: String::from("Password:"), // Default prompt label
            no_label: false,
            no_visibility: false,
            prompt_mask_token: '*',
            prompt_input: String::new()
        }
    }

    fn token_set(&self) -> bool {
        // If this check is true, then we are allowed to display visible feedback
        // (The prompt label will still always be visible)
        self.prompt_mask_token != ' '
    }

    pub fn set_prompt_label(&mut self, custom_label: &str) {
        self.label = custom_label.to_string();
    }

    pub fn set_no_label(&mut self) {
        // Disable the visibility of the prompt label
        self.no_label = true;
    }

    pub fn set_no_visibility(&mut self) {
        // Disable input reveal on `KeyCode::Down` event
        self.no_visibility = true;
    }

    pub fn set_prompt_mask_token(&mut self, custom_token: char) {
        // If the custom token is empty, no visible feedback will be shown to the user
        self.prompt_mask_token = custom_token;
    }

    fn update_prompt_state(&self, mask: &str) {
        if self.token_set() {
            // On `KeyCode::Backspace` event the prompt needs to be updated to ensure the 
            // correct content state of `mask.content` is visible 
            print!("\r{} {}", self.label, " ".to_string().repeat(mask.len() + 1));
            print!("\r{} {}", self.label, mask);
        }
    }

    fn set_content_internal(&mut self, input: &str) {
        self.prompt_input = input.to_string();
    }

    fn get_content_internal(&self) -> String {
        self.prompt_input.clone()
    }

    #[must_use]
    pub fn launch_prompt(&mut self) -> io::Result<String> {
        terminal::enable_raw_mode()?;

        if !self.no_label {
            print!("{} ", self.label);
            std::io::stdout().flush()?; 
        }

        let mut input = String::new();
        let mut mask = PromptMask {
            token: self.prompt_mask_token,
            // `content` will only contain n of the specified `token`
            // --> Required for updating the visible feedback
            content: String::new() 
        };

        loop {
            std::io::stdout().flush()?; 

            if let Event::Key(KeyEvent {
                code, 
                kind: KeyEventKind::Press, 
                ..
            }) = event::read()? {
                match code {
                    KeyCode::Char(c) => {
                        input.push(c); 
                        mask.content.push(mask.token);

                        if self.token_set() { 
                            print!("{}", mask.token);
                        }
                    }
                    KeyCode::Enter => {
                        if !self.no_label {
                            self.update_prompt_state(&mask.content);
                        }

                        println!("\r");

                        break; 
                    }
                    KeyCode::Backspace => {
                        // Remove the last entry every time `KeyCode::Backspace` is pressed
                        input.pop(); 
                        mask.content.pop();

                        self.update_prompt_state(&mask.content);
                    }
                    KeyCode::Down => {
                        if !self.no_visibility {
                            // Allowed to unmask the input
                            print!("\r{} {}", self.label, input);
                        }
                    }
                    KeyCode::Up => {
                        // Restore mask after `KeyCode::Down` event
                        self.update_prompt_state(&mask.content);
                    }
                    _ => {}
                }
            }
        }

        terminal::disable_raw_mode()?;

        self.set_content_internal(&input);

        Ok(input)
    }

    pub fn hash_sha256_internal(&self) -> String {
        sha256::digest(self.get_content_internal())
    }

    pub fn hash_sha256_external(&self, input: &str) -> String {
        sha256::digest(input)
    }

    pub fn hash_md5_internal(&self) -> String {
        format!("{:x}", md5::compute(self.get_content_internal()))
    }

    pub fn hash_md5_external(&self, input: &str) -> String {
        format!("{:x}", md5::compute(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_compare_hashes() {
        let session = CliPass::new();
        assert_eq!(
            session.hash_sha256_external("supersecretpassword"), 
            "5ac152b6f8bdb8bb12959548d542cb237c4a730064bf88bbb8dd6e204912baad"
        );
    }

    #[test]
    fn md5_compare_hashes() {
        let session = CliPass::new();
        assert_eq!(
            session.hash_md5_external("supersecretpassword"), 
            "bbb2c5e63d2ef893106fdd0d797aa97a"
        )
    }
}