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

use std::io;

use clipass_rs::CliPass;

fn notification(block: u8) {
    println!();
    println!("[*] Executing Block: Example Nr. {}..", block);
}

fn main() -> io::Result<()> {
    {
        // ------------------------------- Example 1 ---------------------------------
        notification(1);

        const CUSTOM_LABEL: &str = "Please enter your password:";

        let mut session = CliPass::new();
        session.set_prompt_label(CUSTOM_LABEL); 

        let password = session.launch_prompt()?;

        /*
        * If not disabled with `session.set_no_visibility()`, the input can be unmasked with the arrow
        * down key and masked again with the arrow up key. Every time the user trying to type while the
        * prompt is in the unmasked state, the prompt will be masked again automatically. Same thing after 
        * hitting enter in the unmasked state.
        */

        if password.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Please provide a password!"));
        }

        println!("Cleartext Password: {}", password);
        println!("Sha256 Password Hash: {}", session.hash_sha256_internal());
    }

    {
        // ------------------------------- Example 2 ---------------------------------
        notification(2);

        const HASH_256: &str = "405f42005704da932ea8a4ad1f1e8c26e751af0316caa1e7e4bef4af4e2d93fe"; // correctpassword

        let mut session = CliPass::new();
        session.set_prompt_mask_token('#');
        session.set_no_label(); // We don't want to display any prompt label
        session.launch_prompt()?;
        
        // Compare the required hash against the hashed password provided by the user
        assert_eq!(session.hash_sha256_internal(), HASH_256);
    }

    /*
    * [*] Executing Block: Example Nr. 1..
    * Please enter your password: ******************* 
    * Cleartext Password: supersecretpassword
    * Sha256 Password Hash: 5ac152b6f8bdb8bb12959548d542cb237c4a730064bf88bbb8dd6e204912baad
    *
    * [*] Executing Block: Example Nr. 2..
    * ############### <-- "invalidpassword"
    *
    * thread 'main' panicked at src/bin/main.rs:56:9:
    * assertion `left == right` failed
    *  left: "5bd7f6cf3b61dd672341c7cc2baafd3e960b01ffef4509bb631b4b267e85b444"
    * right: "405f42005704da932ea8a4ad1f1e8c26e751af0316caa1e7e4bef4af4e2d93fe"
    */

    Ok(())
}