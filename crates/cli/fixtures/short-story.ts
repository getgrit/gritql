import { Configuration, OpenAIApi } from 'openai';

const configuration = new Configuration({
  apiKey: process.env.OPENAI_API_KEY,
});
const openai = new OpenAIApi(configuration);

export default async function (req, res) {
  const completion = await openai.createCompletion({
    model: 'text-davinci-003',
    prompt: reviewPrompt(req.body.product),
    max_tokens: 150,
    temperature: 0.8,
    top_p: 1.0,
    frequency_penalty: 0.5,
    presence_penalty: 0.0,
  });
  res.status(200).json({ result: completion.data.choices[0].text });
}

function reviewPrompt(productName) {
  return `Topic: Breakfast
  Two-Sentence Horror Story: He always stops crying when I pour the milk on his cereal. I just have to remember not to let him see his face on the carton.
      
  Topic: ${productName}
  Two-Sentence Horror Story:`;
}
