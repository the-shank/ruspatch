import pandas as pd
import matplotlib.pyplot as plt
import math
import matplotlib

matplotlib.rc("font", family="Noto Sans CJK JP", weight="bold")

df = pd.read_csv("./e3.csv")

x = df["op_num"]
y1 = df["before"]
y2 = df["after"]
fig, ax = plt.subplots()

ax.plot(x, y1, label="before", linewidth=3)
ax.plot(x, y2, label="after", linewidth=3)
ax.legend(loc="upper left", fontsize=20)
plt.xticks(fontsize=20)
plt.yticks(fontsize=20)
ax.set_xlabel(xlabel="次数", fontsize=20)
ax.set_ylabel(ylabel="时间(us)", fontsize=20)
ax.set_title(label="copy_nonoverlapping()", fontsize=20)
plt.show()
