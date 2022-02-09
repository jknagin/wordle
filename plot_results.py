import pandas as pd
import matplotlib.pyplot as plt

guesses = pd.read_csv('results_aloes_worst_sorted.txt', sep=' ', header=None)[1].values

plt.figure()
plt.hist(guesses, color='blue', label='aloes, worst case cost, sorted word bank', bins=50)
plt.ylabel("Guesses")
plt.legend()
plt.show()


