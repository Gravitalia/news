import re
from transformers import AutoTokenizer, AutoModelForSeq2SeqLM

MODEL_NAME = "csebuetnlp/mT5_multilingual_XLSum"
MAX_INPUT_LENGTH = 4096 
MIN_SUMMARY_LENGTH = 100
MAX_SUMMARY_LENGTH = 1500

tokenizer = AutoTokenizer.from_pretrained(MODEL_NAME)
model = AutoModelForSeq2SeqLM.from_pretrained(MODEL_NAME)
model.eval()

def clean_text(text: str) -> str:
    """Replace line breaks by space."""
    cleaned_text = re.sub(r"\n+", " ", text.strip())
    cleaned_text = re.sub(r"\s+", " ", cleaned_text)
    return cleaned_text

def sum_text(text: str) -> str:
    """
    sum_text uses machine learning algorithm (mT5 model) to sum a large news text.

    :param text: large text to sum up
    :return: text entry summary
    """
    cleaned_text = clean_text(text)

    if len(cleaned_text) == 0:
        return "Text only contains line breaks."
    
    input_ids = tokenizer(
        [cleaned_text],
        return_tensors="pt",
        padding="max_length",
        truncation=True,
        max_length=MAX_INPUT_LENGTH,
    )["input_ids"]

    output_ids = model.generate(
        input_ids=input_ids,
        min_length=MIN_SUMMARY_LENGTH,
        max_length=MAX_SUMMARY_LENGTH,
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
