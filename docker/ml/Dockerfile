FROM python:3.9

WORKDIR /app

COPY ../machine_learning/pyproject.toml .

RUN pip install --upgrade pip setuptools wheel
RUN pip install .

COPY ../machine_learning/rest .

CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "9001"]