from fastapi import FastAPI

app = FastAPI()


@app.get("/health")
async def health():
    return


@app.get("/")
async def root():
    return "Hello Python!"
