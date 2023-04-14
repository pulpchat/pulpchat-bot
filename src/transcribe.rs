use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transcript {
    pub transcript: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Segment {
    pub start_time: String,
    pub end_time: String,
    pub speaker_label: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SpeakerLabels {
    pub channel_label: String,
    pub speakers: i64,
    pub segments: Vec<Segment>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Alternative {
    pub confidence: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Item {
    pub start_time: String,
    pub end_time: String,

    pub alternatives: Vec<Alternative>,
    #[serde(rename = "type")]
    pub transcription_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TranscriptionResults {
    pub transcripts: Vec<Transcript>,
    pub speaker_labels: SpeakerLabels,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transcription {
    pub job_name: String,
    pub account_id: String,
    pub results: TranscriptionResults,
}

impl Transcription {
    pub fn new() -> Transcription {
        Transcription {
            job_name: String::new(),
            account_id: String::new(),
            results: TranscriptionResults {
                transcripts: Vec::new(),
                speaker_labels: SpeakerLabels {
                    channel_label: String::new(),
                    speakers: 0,
                    segments: Vec::new(),
                },
                items: Vec::new(),
            },
        }
    }

    pub fn read_test_transcription() -> Transcription {
        // let mut file = File::open("asrOutput.json").unwrap();
        // let mut contents = String::new();
        // file.read_to_string(&mut contents).unwrap();
        // let transcription: Transcription = serde_json::from_str(&contents).unwrap();

        let file = File::open("asrOutput.json").unwrap();
        let reader = std::io::BufReader::new(file);
        let transcription: Transcription = serde_json::from_reader(reader).unwrap();

        println!("{:#?}", transcription);

        transcription
    }
}
