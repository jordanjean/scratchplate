FROM python:3.13-slim AS builder

WORKDIR /app
ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    PIP_NO_CACHE_DIR=1

COPY requirements.txt ./
RUN python -m pip install --upgrade pip \
    && python -m pip install -r requirements.txt

COPY scratchplate_app ./scratchplate_app

FROM builder AS test
COPY requirements-dev.txt ./
RUN python -m pip install -r requirements-dev.txt
COPY tests ./tests
ENV PYTHONPATH=/app
RUN pytest -q

FROM python:3.13-slim AS final

ARG APP_VERSION=0.1.0
WORKDIR /app
ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    PYTHONPATH=/app \
    APP_VERSION=${APP_VERSION}

COPY --from=builder /usr/local /usr/local
COPY scratchplate_app ./scratchplate_app

USER 10001:10001
EXPOSE 8080 8081
ENTRYPOINT ["python", "-m", "scratchplate_app.infrastructure.main"]
