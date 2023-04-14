CLASSIFY_PROCON_HEADER_PROMPT = "Segment this paragraph into a premise and consequence: {{paragraph}} Return your answer as a JSON object. Do not return anything except for the JSON object. Use the below schema for your answer: \
    { \
        \"root_premise\": \"Premise 1\", \
        \"consequence\": \"Consequence 1\" \
    } \
"