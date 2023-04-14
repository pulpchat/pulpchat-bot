pub struct Data {
    id: String,

    prompt: String,

    answer: String,
}

impl Data {
    pub fn new(id: String, prompt: String, answer: String) -> Data {
        Data { id, prompt, answer }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_prompt(&self) -> &String {
        &self.prompt
    }

    pub fn get_answer(&self) -> &String {
        &self.answer
    }
}
