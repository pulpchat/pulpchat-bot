import csv
import json
import openai

openai.api_key = "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"
claim_prompt = """
For each item in the list below, turn the questions into 2 "claims", positive and negative:\  
List: {{list}}\

Only return a JSON object. Use the following JSON schema:\

{\
"questions_and_claims":[{\
"original_question":,\
"claim_positive":,\
"claim_negative":,\
}]\
}\
"""

def ask(content):
        response = openai.Completion.create(
                model="text-davinci-003",
                prompt=content,
                temperature=0,
                max_tokens=2000,
                top_p=1,
                frequency_penalty=0,
                presence_penalty=0
                )
        return(response.choices[0].text)

def replace_right(source, target, replacement, replacements=None):
    """
        This function replaces a substring in a string starting from the right side.
    """
    return replacement.join(source.rsplit(target, replacements))

def writeout(content, file):
    with open(file, 'w') as clean_titles:
        clean_titles.write(content)

def writeout_append(content, file):
    with open(file, 'a+') as clean_titles:
        clean_titles.write(content)

def clean():
    list_content = ""
    responses = []
    questions_and_claims = { "questions_and_claims": [] }

    # Read titles from 'titles.csv' and find "questions and claims" for each title
    with open("../../data/procon/titles.csv") as file:
        lines = file.readlines()
        
        for line in lines[1:len(lines)-1]:
            list_content += (line + "\n")
        
        prompt = claim_prompt.replace("{{list}}", list_content)
        data = ask(prompt)
        
        writeout(data, "../../data/procon/cleaned/clean_titles.json")

    # Preprocess the titles and questions from 'a_to_z_debates_dirty.csv'
    with open("../../data/procon/a_to_z_debates_dirty.csv") as file:
        dirty_debate_lines = file.readlines()
        titles = []
        questions = []
        
        for i in range(0, len(dirty_debate_lines)-1):           
            if len(dirty_debate_lines[i]) < 4:
                continue

            title_and_question_array = dirty_debate_lines[i].split(',', 1)

            writeout_append(title_and_question_array[1].replace("\"", "").lstrip(), "../../data/procon/cleaned/clean_dirty_debates.csv")

    # Read the preprocessed titles and questions from 'clean_dirty_debates.csv' and find "questions and claims" for each title
    with open("../../data/procon/cleaned/clean_dirty_debates.csv") as file:
        lines = file.readlines()
        
        # This loop increments by 10, sending a request to OpenAI for every 10 lines
        for i in range(0, len(lines)-1, 10):
            line_content = ""
            if i > len(lines)-15:
                for l in lines[i:len(lines)-1]:
                    if l is None:
                        continue
                    line_content += (lines[i].lstrip() + "\n")

                prompt = claim_prompt.replace("{{list}}", line_content)

                data = ask(prompt)
                responses.append(data)

                break

                
            
            for j in range(i, (i+10)):
                if j is None:
                    continue

                print("line_content for row (", str(i), "), line (", str(j), "): ", line_content)
                line_content += (lines[j].lstrip() + "\n") 
            
            prompt = claim_prompt.replace("{{list}}", line_content)

            data = ask(prompt)
            responses.append(data)

    for i in range(0, len(responses)-1):
        res = responses[i]
        data = json.loads(res)
        questions_and_claims["questions_and_claims"].append(data)

    # Write the dictionary to a file as a JSON object
    with open("../../data/procon/cleaned/clean_debates_debug_full_v3.json", "w") as outfile:
        json.dump(questions_and_claims, outfile)     

if __name__ == "__main__":
    openai.api_key = "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"

    clean()