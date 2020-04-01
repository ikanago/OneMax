import pandas as pd

df = pd.read_csv('./result.csv')
print(df["fitness"].std(axis=0))