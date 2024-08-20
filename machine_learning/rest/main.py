import logging
import unidecode
import re

from fastapi import FastAPI, Response, status
from fastapi.responses import JSONResponse
from fastapi.middleware.cors import CORSMiddleware

from transformers import AutoTokenizer, AutoModelForSeq2SeqLM

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

tokenizer = AutoTokenizer.from_pretrained("csebuetnlp/mT5_multilingual_XLSum")
model = AutoModelForSeq2SeqLM.from_pretrained("csebuetnlp/mT5_multilingual_XLSum")


# Handle prefetch request.
@app.options("/")
def read_root():
    return "OK"


WHITESPACE_HANDLER = lambda k: re.sub('\s+', ' ', re.sub('\n+', ' ', k.strip()))

@app.get("/summary/", status_code=200)
def read_sum(text: str, response: Response):
    """
    Summary model HTTP API.
    """
    text = unidecode.unidecode(text)

    input_ids = tokenizer(
        [WHITESPACE_HANDLER(text)],
        return_tensors="pt",
        padding="max_length",
        truncation=True,
        max_length=4096
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


@app.get("/qa/", status_code=200)
def read_sum(text: str, response: Response):
    """
    Question & answers model HTTP API.
    Generates one question and three answers (one true, two false) based on a text.
    """
    response.status_code = status.HTTP_201_CREATED
    return JSONResponse(
        status_code=200,
        content={
            "question": "...",
            "choices": [],
            "answer": 0,
        },
    )
