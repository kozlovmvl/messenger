import asyncio
import websockets


async def handler():
    async with websockets.connect("ws://localhost:88/ws/") as ws:
        await ws.send("test message")
        answer = await ws.recv()
        print(answer)

asyncio.run(handler())

