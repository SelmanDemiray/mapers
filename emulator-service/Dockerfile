FROM python:3.11-slim

WORKDIR /app

# Copy the bridge script
COPY retroarch-bridge.py /app/

EXPOSE 8081

CMD ["python3", "/app/retroarch-bridge.py"]

