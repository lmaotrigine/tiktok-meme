//! https://cdn.awwni.me/o6gt.jpg
//! Copyright (c) 2022, VJ#5945, All rights reserved.
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions are met:
//!
//! 1. Redistributions of source code must retain the above copyright notice, this
//!    list of conditions and the following disclaimer.
//!
//! 2. Redistributions in binary form must reproduce the above copyright notice,
//!    this list of conditions and the following disclaimer in the documentation
//!    and/or other materials provided with the distribution.
//!
//! 3. Neither the name of the copyright holder nor the names of its
//!    contributors may be used to endorse or promote products derived from
//!    this software without specific prior written permission.
//!
//! 4. Each source code file must contain in its header or footer a comment containing a
//!    hyperlink to a publicly accessible resource of an image of a "catgirl"
//!    (https://en.wikipedia.org/wiki/Catgirl)
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
//! AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//! SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//! CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//! OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//! OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::io::{Read, Write};

use anyhow::Result;
use clap::StructOpt;
use reqwest::Client;

const TIKTOK_API_BASE: &str =
    "https://api16-normal-useast5.us.tiktokv.com/media/api/text/speech/invoke/?speaker_map_type=0";

const VOICES: [&str; 34] = [
    // DISNEY VOICES
    "en_us_ghostface",    // Ghost Face
    "en_us_chewbacca",    // Chewbacca
    "en_us_c3po",         // C3PO
    "en_us_stitch",       // Stitch
    "en_us_stormtrooper", // Stormtrooper
    "en_us_rocket",       // Rocket
    // ENGLISH VOICES
    "en_au_001", // English AU - Female
    "en_au_002", // English AU - Male
    "en_uk_001", // English UK - Male 1
    "en_uk_003", // English UK - Male 2
    "en_us_001", // English US - Female (Int. 1)
    "en_us_002", // English US - Female (Int. 2)
    "en_us_006", // English US - Male 1
    "en_us_007", // English US - Male 2
    "en_us_009", // English US - Male 3
    "en_us_010", // English US - Male 4
    // EUROPE VOICES
    "fr_001", // French - Male 1
    "fr_002", // French - Male 2
    "de_001", // German - Female
    "de_002", // German - Male
    "es_002", // Spanish - Male
    // AMERICA VOICES
    "es_mx_002", // Spanish MX - Male
    "br_001",    // Portuguese BR - Female 1
    "br_003",    // Portuguese BR - Female 2
    "br_004",    // Portuguese BR - Female 3
    "br_005",    // Portuguese BR - Male
    // ASIA VOICES
    "id_001", // Indonesian - Female
    "jp_001", // Japanese - Female 1
    "jp_003", // Japanese - Female 2
    "jp_005", // Japanese - Female 3
    "jp_006", // Japanese - Male
    "kr_002", // Korean - Male 1
    "kr_003", // Korean - Female
    "kr_004", // Korean - Male 2
];

#[derive(Debug)]
struct InvalidVoice {
    voice: String,
}

impl std::fmt::Display for InvalidVoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid voice: {}", self.voice)
    }
}

impl std::error::Error for InvalidVoice {}

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct CommandLine {
    #[clap(short = 'o', long = "--out")]
    pub output: Option<String>,
    #[clap(short = 't', long = "--text")]
    pub text: Option<String>,
    #[clap(short = 'v', long = "--voice")]
    pub voice: Option<String>,
}

#[derive(serde::Deserialize)]
struct ApiResp {
    pub data: ApiRespInner,
}

#[derive(serde::Deserialize)]
struct ApiRespInner {
    pub v_str: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CommandLine::parse();
    let client = Client::new();
    let rq = client.post(TIKTOK_API_BASE);
    let text = match cli.text {
        Some(t) => t,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };
    let voice = match cli.voice {
        Some(v) => v,
        None => "en_us_002".to_string(),
    };
    if !VOICES.iter().any(|&i| i == voice) {
        return Err(InvalidVoice { voice }.into());
    }
    let rq = rq.query(&[("text_speaker", &voice), ("req_text", &text)]);
    let res = rq.send().await?;
    let res_text = res.text().await?;
    let f = serde_json::from_str::<ApiResp>(&res_text)?;
    let bytes = data_encoding::BASE64.decode(f.data.v_str.as_bytes())?;
    match cli.output {
        Some(o) => {
            std::fs::write(o, &bytes)?;
        }
        None => {
            std::io::stdout().lock().write_all(&bytes)?;
        }
    }
    Ok(())
}
