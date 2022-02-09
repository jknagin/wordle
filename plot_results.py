import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

guesses = pd.read_csv('results_aesir.txt', sep=' ', header=None)[1].values
print(f"Average number of guesses: {round(np.mean(guesses), 1)}")

plt.figure()
plt.hist(guesses, color='blue', label='aesir', bins=50)
plt.ylabel("Guesses")
plt.legend()
plt.show()


