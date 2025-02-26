import asyncio
from py202 import order


async def main():
    await order("burger soda burger")


asyncio.run(main())
