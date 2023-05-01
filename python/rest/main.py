from fastapi import FastAPI
import uvicorn
import otel

def get_app() -> FastAPI:
    app = FastAPI()

    otel.enable_opentelemetry(
        app,
        "rest",
        "0.1.0",
        "http://localhost:4317",
        "local",
    )

    return app

app = get_app()

@app.get("/foo")
async def foo():
    return { "message": "foo" }

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8001)
    