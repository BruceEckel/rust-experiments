import asyncio

counter = 0


async def incrementer(name):
    global counter
    while not done:
        # print(f"{name} incrementing...")
        current = counter
        await asyncio.sleep(0)  # Yield control to the event loop
        counter = current + 1
        # print(f"{name} done incrementing. Counter: {counter}")


async def main():
    global done
    done = False
    # Schedule both incrementers to run "concurrently"
    task1 = asyncio.create_task(incrementer("A"))
    task2 = asyncio.create_task(incrementer("B"))

    await asyncio.sleep(10)  # Let it run for 10 seconds

    done = True  # Signal the incrementers to stop
    await task1  # Wait for task1 to finish
    await task2  # Wait for task2 to finish

    expected = counter // 2 * 2  # Because two tasks are incrementing the counter
    if counter != expected:
        print(f"Data race detected! Counter: {counter}, Expected: {expected}")
    else:
        print(f"No data race detected after 10 seconds. Counter: {counter}")


asyncio.run(main())
