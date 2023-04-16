# notes

## Special Characters
- `<[clm]>`, `<[/clm]>` 
- `<[suparg]>`, `<[/suparg]>` 
- `<[rebarg]>`, `<[/rebarg]>` 
- `<[prm]>`, `<[/prm]>` 
- `<[csq]>`, `<[/csq]>` 

## TODO
 - https://immigration.procon.org/
 - https://marijuana.procon.org/
 - https://concealedguns.procon.org/
 - https://animal-testing.procon.org/
 - https://cellphones.procon.org/
 - https://alternativeenergy.procon.org/
 - https://climatechange.procon.org/
 - https://felonvoting.procon.org/
 - https://votingmachines.procon.org/
 - https://minimum-wage.procon.org/
 - https://gold-standard.procon.org/
 - https://corporatetax.procon.org/
 - https://churchesandtaxes.procon.org/
 - https://socialsecurity.procon.org/
 - https://medicalmarijuana.procon.org/
 - https://euthanasia.procon.org/
 - https://vaccines.procon.org/
 - https://milk.procon.org/
 - https://birth-control.procon.org/
 - https://convokit.cornell.edu/documentation/architecture.html
 - https://aclanthology.org/2020.emnlp-main.1.pdf
 - https://www.dataquest.io/blog/- tutorial-text-classification-in-python-using-spacy/

 # Resources
 - [UCSC Natural Language and Dialogue Systems](https://nlds.soe.ucsc.edu/software)
 - [Penn Treebank](https://catalog.ldc.upenn.edu/LDC99T42)

 # Architecture
 - **Classifier**
    - Classify arguments -> (supporting, rebuttal), (premise, consequence)
    - Binary classification?

Generate a counter-argument (response) for an argument (premise, consequence)
   - Classify whether or not it's an argument
      - Sentence structure (parts of speech)
      - Input: Argument (text)
      - Output: 0 or 1
   - Detect weak sentence by analyzing sentence structure
      - LTR parts of a sentence