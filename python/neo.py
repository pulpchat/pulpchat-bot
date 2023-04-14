from neo4j import GraphDatabase

# "neo4j://localhost:7687"
# "neo4j"
# "123"
class Training4j:

    def __init__(self, uri, user, password):
        self.driver = GraphDatabase.driver(uri,
                                auth=(user, password))

    def add_training_data(tx, full_argument, premise, consequence, premise_speech_parts, consequence_speech_parts):
        """
        Adds a new argument to the neo4j - use this function to add training data

        tx -- the transaction
        full_argument -- the argument which contains the premise and consequence
        premise -- the premise
        consequence -- the consequence
        premise_speech_parts -- the premise speech parts
        consequence_speech_parts -- the consequence speech parts
        """
        tx.run("CREATE (ta:TrainingArgument {full_argument: $full_argument, premise: $premise, consequence: $consequence, premise_speech_parts: $premise_speech_parts, consequence_speech_parts: $consequence_speech_parts})"
            "RETURN (ta)",
            full_argument=full_argument, premise=premise, consequence=consequence, premise_speech_parts=premise_speech_parts, consequence_speech_parts=consequence_speech_parts)
        
    def add_validation_data(tx, argument, premise, consequence, premise_speech_parts, consequence_speech_parts):
        """
        Adds a new argument to the neo4j - use this function to add validation data

        tx -- the transaction
        argument -- the argument which contains the premise and consequence
        premise -- the premise
        consequence -- the consequence
        premise_speech_parts -- the premise speech parts
        consequence_speech_parts -- the consequence speech parts
        """
        tx.run("CREATE (va:ValidationArgument {argument: $argument, premise: $premise, consequence: $consequence, premise_speech_parts: $premise_speech_parts, consequence_speech_parts: $consequence_speech_parts})"
            "RETURN (va)",
            argument=argument, premise=premise, consequence=consequence, premise_speech_parts=premise_speech_parts, consequence_speech_parts=consequence_speech_parts)

    def get_training_data(tx):
        """
        Gets the training data from the neo4j database

        tx -- the transaction
        """
        query = ("MATCH (ta:TrainingArgument) "
                "RETURN ta.name ORDER BY ta.name")
        for record in tx.run(query):
            print(record["ta.name"])

    def get_validation_data(tx):
        """
        Gets the validation data from the neo4j database

        tx -- the transaction
        """
        query = ("MATCH (va:ValidationArgument) "
                "RETURN va.name ORDER BY va.name")
        for record in tx.run(query):
            print(record["va.name"])

    def execute_add_training_data(self, full_argument, premise, consequence, premise_speech_parts, consequence_speech_parts):
        """
        Executes the add_training_data function

        full_argument -- the argument which contains the premise and consequence
        premise -- the premise
        consequence -- the consequence
        premise_speech_parts -- the premise speech parts
        consequence_speech_parts -- the consequence speech parts
        """
        with self.driver.session() as session:
            session.execute_write(self.add_training_data, full_argument, premise, consequence, premise_speech_parts, consequence_speech_parts)

    def execute_add_validation_data(self, argument, premise, consequence, premise_speech_parts, consequence_speech_parts):
        """
        Executes the add_validation_data function

        argument -- the argument which contains the premise and consequence
        premise -- the premise
        consequence -- the consequence
        premise_speech_parts -- the premise speech parts
        consequence_speech_parts -- the consequence speech parts
        """
        with self.driver.session() as session:
            session.execute_write(self.add_validation_data, argument, premise, consequence, premise_speech_parts, consequence_speech_parts)

    def execute_get_training_data(self):
        """
        Executes the get_training_data function
        """
        with self.driver.session() as session:
            session.execute_read(self.get_training_data)

    def execute_get_validation_data(self):
        """
        Executes the get_validation_data function
        """
        with self.driver.session() as session:
            session.execute_read(self.get_validation_data)

    def close(self):
        """
        Closes the driver
        """
        self.driver.close()