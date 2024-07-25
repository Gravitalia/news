import logging
import tensorflow as tf

from fastapi import FastAPI, Response, status
from fastapi.responses import JSONResponse
from fastapi.middleware.cors import CORSMiddleware

#summary_model = tf.keras.models.load_model("")

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Handle prefetch request.
@app.options("/")
def read_root():
    return "OK"

# Summary model HTTP API.
@app.get("/summary/", status_code=200)
def read_sum(text: str, response: Response):
    response.status_code = status.HTTP_201_CREATED
    return "SUM"

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
