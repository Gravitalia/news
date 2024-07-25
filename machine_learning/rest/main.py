import logging
import unidecode

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

tokenizer = AutoTokenizer.from_pretrained("google/pegasus-large")
model = AutoModelForSeq2SeqLM.from_pretrained("google/pegasus-large")

# Handle prefetch request.
@app.options("/")
def read_root():
    return "OK"

# Summary model HTTP API.
@app.get("/summary/", status_code=200)
def read_sum(text: str, response: Response):
    text = unidecode.unidecode(text)
    inputs = tokenizer(text, return_tensors="pt", truncation=True, padding="longest")
    summary_ids = model.generate(inputs["input_ids"], max_length=2000, min_length=200, length_penalty=0.0, num_beams=4, early_stopping=True)
    return tokenizer.decode(summary_ids[0], skip_special_tokens=True)

"""
Question & answers model HTTP API.
Generates one question and three answers (one true, two false) based on a text.
"""
@app.get("/qa/", status_code=200)
def read_sum(text: str, response: Response):
    response.status_code = status.HTTP_201_CREATED
    return JSONResponse(
        status_code=200,
        content={
            "question": "...",
            "choices": [],
            "answer": 0,
        }
    )
