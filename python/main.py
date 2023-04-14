import spacy
import json
import openai
import boto3
from boto3.dynamodb.conditions import Attr

from prompts import CLASSIFY_PROCON_HEADER_PROMPT
from neo import Training4j

def run_procon():
    nlp = spacy.load("en_core_web_sm")
    client = boto3.resource('dynamodb')
    table = client.Table('pulpscrape')

    columns = []
    titles = []
    data = table.scan(TableName='pulpscrape', FilterExpression=Attr('topic').exists())

    for i in range(len(data['Items'])):
        for column in data['Items']:
            arguments = []
            quotes = []
            i = 0

            while True:
                if "argument_"+str(i) not in column:
                    break

                if "header" not in column["argument_"+str(i)].keys():
                    column["argument_"+str(i)]["header"] = ""

                arguments.append(
                    {
                        "argument_type": column["argument_"+str(i)]["argument_type"],
                        "argument_header": column["argument_"+str(i)]["header"],
                        "argument_title": column["argument_"+str(i)]["argument_title"],
                        "argument_content": column["argument_"+str(i)]["argument_content"],
                    }
                )

                i += 1

            i = 0
            while True:
                quotes.append(
                    {
                        "quote": column["quote_"+str(i)]["quote"],
                        "quote_type": column["quote_"+str(i)]["quote_type"],
                    }
                )

                i += 1

                if "quote_"+str(i) not in column:
                    break

            columns.append(
                {
                    "topic": column["topic"],
                    "background": column["background"],
                    "history": column["history"],
                    "supporting_data": column["supporting_data"],
                    "arguments": arguments,
                    "quotes": quotes,
                    "tokens": {
                        "background": "",
                        "arguments": {},
                        "quotes": {},
                    },

                }
            )

    for column in columns:
        bg_nlp = nlp(column["background"])
        column["tokens"]["background"] = bg_nlp

        # print("Background Tokens: \n")
        # for token in bg_nlp:
        #     print(token.text, token.pos_, token.tag_, token.dep_, token.shape_, token.is_alpha, token.is_stop)

        for argument in column["arguments"]:
            arg_nlp = nlp(argument["argument_content"])
            column["tokens"]["arguments"] = {
                "argument_type": argument["argument_type"],
                "argument_title": argument["argument_title"],
                "argument_content": arg_nlp,
            }

            # print("Argument Tokens: \n")
            # for token in arg_nlp:
            #     print(token.text, token.pos_, token.tag_, token.dep_, token.shape_, token.is_alpha, token.is_stop)

        for quote in column["quotes"]:
            for q in quote["quote"]:
                q_nlp = nlp(q)
                column["tokens"]["quotes"] = {
                    "quote": q_nlp,
                    "quote_type": quote["quote_type"],
                }

                # print("Quote Tokens: \n")
                # for token in q_nlp:
                #     print(token.text, token.pos_, token.tag_, token.dep_, token.shape_, token.is_alpha, token.is_stop)

        titles.append(column["topic"])

    print()
    write_titles_to_file(titles)
    # generate_data(columns, "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")

def generate_data(columns, key):
    openai.api_key = key
    
    for column in columns:
        print(column, "\n\n\n")
        # for argument in column["arguments"]:

        #     if "argument_header" not in argument.keys():
        #         continue
        #     elif argument["argument_header"] == "":
        #         continue

        #     prompt = CLASSIFY_PROCON_HEADER_PROMPT.replace("{{paragraph}}", argument["argument_header"])
        #     response = openai.ChatCompletion.create(model="gpt-3.5-turbo",  messages=[{"role": "user", "content": prompt}], temperature=0, max_tokens=2000)

        #     if response["consequence"] == "":
        #         continue

        #     print(response)

def write_json_to_file(columns):
    with open('data.json', 'w') as outfile:
        json.dump(columns, outfile)

def write_titles_to_file(titles):
    with open('titles.csv', 'w') as outfile:
        outfile.write("title\n")
        for title in titles:
            outfile.write(title + "\n")

if __name__ == "__main__":
    run_procon()