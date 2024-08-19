FROM tiangolo/uvicorn-gunicorn-fastapi:python3.6-2022-11-25

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc \
    libpython3-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

#COPY ../machine_learning/pyproject.toml .
COPY ../machine_learning/requirements.txt ./requirements.txt
RUN pip install --upgrade pip setuptools wheel
RUN pip install -r requirements.txt

COPY ../machine_learning/rest .

EXPOSE 8000