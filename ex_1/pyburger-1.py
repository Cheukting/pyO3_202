import asyncio


async def burger():
    await asyncio.sleep(5)  # time it takes to make a burger
    print("burger made")


async def soda():
    await asyncio.sleep(1)  # time it takes to pour a soda
    print("soda pour")


async def fries():
    await asyncio.sleep(3)  # time it takes to serve fries
    print("fries served")


async def order(order=[]):
    actions = []
    for item in order:
        match item:
            case "burger":
                actions.append(burger())
            case "soda":
                actions.append(soda())
            case "fries":
                actions.append(fries())
            case _:
                print("invalid order")
    await asyncio.gather(*actions)


asyncio.run(order(["burger", "soda", "burger", "fries"]))
print("order complete")
