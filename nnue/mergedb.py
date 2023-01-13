#!/usr/bin/env python

import sqlite3
import sys

files = list(sys.argv[1:])
print(files)

conn = sqlite3.connect('output.sqlite3')
cur = conn.cursor()
cur.execute("""CREATE TABLE IF NOT EXISTS positions
                (history TEXT,
                 p2mv INTEGER,
                 moves INTEGER,
                 p0 INTEGER,
                 p1 INTEGER,
                 eval INTEGER)""")

for f in files:
    conn2 = sqlite3.connect(f)
    cur2 = conn2.cursor()

    entries = cur2.execute('SELECT * from positions').fetchall()
    cur.executemany('INSERT INTO positions VALUES(?,?,?,?,?,?)', entries)

    conn2.close()

conn.commit()
conn.close()
