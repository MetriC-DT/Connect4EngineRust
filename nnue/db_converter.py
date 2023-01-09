#!/usr/bin/env python

import os, sys, sqlite3
from sqlite3 import Connection, Cursor

"""
Formats the database to the form that the neural network can use.
Specifically:
    1. Instead of curr player / opponent, we now store player0, player1 positions.
    2. Instead of moves played, we not store next_to_play (either 0 or 1 for player)
"""

def create_table(cursor, connection):
    cursor.execute("""CREATE TABLE IF NOT EXISTS positions 
                (history TEXT,
                 p2mv INTEGER,
                 p0 INTEGER,
                 p1 INTEGER,
                 eval INTEGER)""")
    connection.commit()


def insert_into_new(conn: Connection, cur: Cursor, src: str):
    s_con = sqlite3.connect(src)
    s_cur = s_con.cursor()
    s_cur.execute("SELECT * FROM positions")

    def new_entry(old_entry):
        """Converts the old entry format to the new one."""
        (hist, moves, player, opp, eval) = old_entry
        p2mv = moves % 2
        (p0, p1) = (player, opp) if p2mv == 0 else (opp, player)
        return (hist, p2mv, p0, p1, eval)

    new_entries = map(new_entry, s_cur.fetchall())
    cur.executemany('INSERT INTO positions VALUES(?,?,?,?,?)', new_entries)

    conn.commit()
    return


if __name__ == '__main__':
    db_to_convert = sys.argv[1]

    if not os.path.exists(db_to_convert):
        print("File doesn't exist.")
        exit(-1)

    name = "formatted_" + os.path.basename(db_to_convert)

    folder = os.path.dirname(db_to_convert)
    loc = os.path.join(folder, name)
    conn = sqlite3.connect(loc)
    cur = conn.cursor()

    create_table(cur, conn)
    insert_into_new(conn, cur, db_to_convert)
