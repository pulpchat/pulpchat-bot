import fs from 'fs';

async function preprocess() {
    let transcription = fs.readFileSync('../../asrOutput.json');
    transcription = JSON.parse(transcription);

    // rome-ignore lint/style/useConst: <explanation>
    let awsTranscript = {
            speaker0: {
                name: "piers morgan",
                transcription: []
            },
            speaker1: {
                name: "kanye west",
                transcription: []
            },
        };

    let lastSpeaker = "";
    let speaker0Line = "";
    let speaker1Line = "";
    transcription.results.items.map((item) => {
        switch (item.speaker_label) {
            case "spk_0":
                if (lastSpeaker === "spk_1") {
                    awsTranscript.speaker1.transcription.push({
                        startTime: item.start_time,
                        endTime: item.end_time,
                        content: speaker1Line
                    });

                    speaker1Line = "";
                    lastSpeaker = "spk_0";
                } else {
                    speaker0Line += `${item.alternatives[0].content} `;

                    lastSpeaker = "spk_0";
                }

                break;
            case "spk_1":
                if (lastSpeaker === "spk_0") {
                    awsTranscript.speaker0.transcription.push({
                        startTime: item.start_time,
                        endTime: item.end_time,
                        content: speaker0Line
                    });

                    speaker0Line = "";
                    lastSpeaker = "spk_1";
                } else {
                    speaker1Line += `${item.alternatives[0].content} `;

                    lastSpeaker = "spk_1";
                }

                break;
            default:
                throw new Error("Speaker not found");
        }
    });

    if (awsTranscript.speaker1.transcription.length > awsTranscript.speaker0.transcription.length) {
        for (let i = 0; i < awsTranscript.speaker0.transcription.length; i++) {
            const sentence0 = awsTranscript.speaker0.transcription[i];
            const sentence1 = awsTranscript.speaker1.transcription[i];

            console.log("piers morgan: ", sentence0.content);
            console.log("kanye west: ", sentence1.content);
            console.log();

            function writeToFile() {
                for (let i = 0; i < awsTranscript.speaker0.transcription.length; i++) {
                    fs.writeFileSync('../../forefrontData.jsonl', `{ prompt: ${awsTranscript.speaker0.transcription[i].content}, completion: ${awsTranscript.speaker1.transcription[i].content}<|endoftext|>}\n`, { flag: 'a+' });
                }
            }

            writeToFile();
        }
    } else {
        for (let i = 0; i < awsTranscript.speaker1.transcription.length; i++) {
            const sentence0 = awsTranscript.speaker0.transcription[i];
            const sentence1 = awsTranscript.speaker1.transcription[i];

            console.log("piers morgan: ", sentence0.content);
            console.log("kanye west: ", sentence1.content);
            console.log();

            function writeToFile() {
                for (let i = 0; i < awsTranscript.speaker1.transcription.length; i++) {
                    fs.writeFileSync('../../forefrontData.jsonl', `{ prompt: ${awsTranscript.speaker0.transcription[i].content}, completion: ${awsTranscript.speaker1.transcription[i].content}<|endoftext|>}\n`, { flag: 'a+' });
                }
            }

            writeToFile();
        }
    }
}

async function main() {
    await preprocess();
}

main();