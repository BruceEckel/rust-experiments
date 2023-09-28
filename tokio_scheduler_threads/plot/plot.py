import matplotlib.pyplot as plt

plt.figure(figsize=(10, 6))

# Yield percentages
yield_percents = [0, 5, 25, 50, 75, 100]

# Elapsed times for single-threaded tokio async
single_threaded_elapsed_times = [
    8.46,  # for yield_percent 0
    65.0268,  # for yield_percent 5
    277.8359,  # for yield_percent 25
    554.5785,  # for yield_percent 50
    830.8615,  # for yield_percent 75
    1.1144795 * 1000,  # for yield_percent 100 (converted from seconds to milliseconds)
]

# Elapsed times for two-threaded tokio async
two_threaded_elapsed_times = [
    4.4677,  # for yield_percent 0
    19.7723,  # for yield_percent 5
    67.1384,  # for yield_percent 25
    136.1394,  # for yield_percent 50
    173.5431,  # for yield_percent 75
    232.5077,  # for yield_percent 100
]


# Plotting data
plt.plot(
    yield_percents,
    single_threaded_elapsed_times,
    marker="o",
    label="Single-threaded",
    color="blue",
)
plt.plot(
    yield_percents,
    two_threaded_elapsed_times,
    marker="o",
    label="Two-threaded",
    color="red",
)

# Adding titles and labels
plt.title("Yield Percent vs. Elapsed Time")
plt.xlabel("Yield Percent")
plt.ylabel("Elapsed Time (ms)")
plt.legend()
plt.grid(True, which="both", linestyle="--", linewidth=0.5)
plt.tight_layout()

# Display the plot
plt.show()
