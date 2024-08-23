import re
from transformers import AutoTokenizer, AutoModelForSeq2SeqLM

tokenizer = AutoTokenizer.from_pretrained("csebuetnlp/mT5_multilingual_XLSum")
model = AutoModelForSeq2SeqLM.from_pretrained(
    "csebuetnlp/mT5_multilingual_XLSum"
)

WHITESPACE_HANDLER = lambda k: re.sub("\s+", " ", re.sub("\n+", " ", k.strip()))


def sum_text(text: str) -> str:
    """
    sum_text uses machine learning algorithm (mT5 model) to sum a large news text.

    :param text: large text to sum up
    :return: text entry summary
    """
    input_ids = tokenizer(
        [WHITESPACE_HANDLER(text)],
        return_tensors="pt",
        padding="max_length",
        truncation=True,
        max_length=4096,
    )["input_ids"]

    output_ids = model.generate(
        input_ids=input_ids,
        min_length=100,
        max_length=1500,
        no_repeat_ngram_size=4,
        num_beams=6,
        temperature=0.7,
        top_k=20,
        length_penalty=0.0,
        repetition_penalty=2.0,
        do_sample=True,
    )[0]

    return tokenizer.decode(
        output_ids,
        skip_special_tokens=True,
        clean_up_tokenization_spaces=True,
    )
